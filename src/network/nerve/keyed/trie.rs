// A13, A21 — THE TRIE: the radix nodes under a keyed relation, and the
// three moves over them — insert, remove, and the meet of two.
//
//! Descent is by the first bit at which two keys differ, so a key is its
//! own path: nothing is searched and nothing is hashed.
use super::relation::Keyed;


/// A path-compressed binary trie (PATRICIA): a branch only where two keys
/// first differ, a leaf holding its whole key. One node per key, so the
/// relation costs what it holds, and every operation is O(bits).
#[derive(Clone, Debug)]
pub(crate) enum Node<V> {
    Leaf(Vec<u8>, V),
    Branch {
        bit: usize,
        children: [Option<Box<Node<V>>>; 2],
        /// A key whose bit-length equals `bit`. Discriminates a key from
        /// a proper extension whose next bit is 0 (`key_bit` past the end
        /// would otherwise walk into the same child).
        end: Option<Box<(Vec<u8>, V)>>,
    },
}

pub(crate) fn key_bit(key: &[u8], i: usize) -> usize {
    let byte = i / 8;
    if byte >= key.len() {
        return 0;
    }
    ((key[byte] >> (7 - (i % 8))) & 1) as usize
}

/// The first bit at which two keys differ, if they differ.
pub(crate) fn first_difference(a: &[u8], b: &[u8]) -> Option<usize> {
    let n = a.len().min(b.len());
    for i in 0..n {
        let x = a[i] ^ b[i];
        if x != 0 {
            return Some(i * 8 + x.leading_zeros() as usize);
        }
    }
    if a.len() != b.len() {
        return Some(n * 8);
    }
    None
}

impl<V> Node<V> {
    pub(crate) fn bits(key: &[u8]) -> usize {
        key.len().saturating_mul(8)
    }

    /// The key of the leaf this key's path reaches.
    pub(crate) fn held_key(&self, key: &[u8]) -> &[u8] {
        let mut node = self;
        loop {
            match node {
                Node::Leaf(k, _) => return k,
                Node::Branch { bit, children, end } => {
                    if Self::bits(key) <= *bit {
                        if let Some((k, _)) = end.as_deref() {
                            return k;
                        }
                        return node.representative();
                    }
                    match children[key_bit(key, *bit)].as_deref() {
                        Some(n) => node = n,
                        None => return node.representative(),
                    }
                }
            }
        }
    }

    pub(crate) fn get(&self, key: &[u8]) -> Option<&V> {
        let mut node = self;
        loop {
            match node {
                Node::Leaf(k, v) => return (k.as_slice() == key).then_some(v),
                Node::Branch { bit, children, end } => {
                    if Self::bits(key) == *bit {
                        return end.as_deref().and_then(|(k, v)| (k.as_slice() == key).then_some(v));
                    }
                    if Self::bits(key) < *bit {
                        return None;
                    }
                    node = children[key_bit(key, *bit)].as_deref()?;
                }
            }
        }
    }

    pub(crate) fn get_mut(&mut self, key: &[u8]) -> Option<&mut V> {
        match self {
            Node::Leaf(k, v) => (k.as_slice() == key).then_some(v),
            Node::Branch { bit, children, end } => {
                if Node::<V>::bits(key) == *bit {
                    return end.as_mut().and_then(|e| (e.0.as_slice() == key).then_some(&mut e.1));
                }
                if Node::<V>::bits(key) < *bit {
                    return None;
                }
                children[key_bit(key, *bit)].as_deref_mut()?.get_mut(key)
            }
        }
    }

    pub(crate) fn representative(&self) -> &[u8] {
        match self {
            Node::Leaf(k, _) => k,
            Node::Branch { children, end, .. } => children
                .iter()
                .find_map(|c| c.as_deref().map(|n| n.representative()))
                .or_else(|| end.as_deref().map(|(k, _)| k.as_slice()))
                .unwrap_or(&[]),
        }
    }
}

pub(crate) fn meet_end<V, W>(end: Option<&(Vec<u8>, V)>, other: Option<&Node<W>>, out: &mut Keyed<()>) {
    let Some((k, _)) = end else { return };
    if other.is_some_and(|n| n.get(k).is_some()) {
        out.insert(k.as_slice(), ());
    }
}

pub(crate) fn meet_nodes<V, W>(a: Option<&Node<V>>, b: Option<&Node<W>>, out: &mut Keyed<()>) {
    match (a, b) {
        (None, _) | (_, None) => {}
        (Some(Node::Leaf(k, _)), Some(Node::Leaf(k2, _))) => {
            if k == k2 {
                out.insert(k.as_slice(), ());
            }
        }
        (Some(leaf @ Node::Leaf(k, _)), Some(Node::Branch { bit, children, end })) => {
            if Node::<V>::bits(k) == *bit {
                meet_end(end.as_deref(), Some(leaf), out);
            } else if Node::<V>::bits(k) > *bit {
                meet_nodes(Some(leaf), children[key_bit(k, *bit)].as_deref(), out);
            }
        }
        (Some(Node::Branch { bit, children, end }), Some(leaf @ Node::Leaf(k, _))) => {
            if Node::<W>::bits(k) == *bit {
                meet_end(end.as_deref(), Some(leaf), out);
            } else if Node::<W>::bits(k) > *bit {
                meet_nodes(children[key_bit(k, *bit)].as_deref(), Some(leaf), out);
            }
        }
        (
            Some(na @ Node::Branch { bit: ba, children: ca, end: ea }),
            Some(nb @ Node::Branch { bit: bb, children: cb, end: eb }),
        ) => {
            let ra = na.representative();
            let rb = nb.representative();
            if let Some(d) = first_difference(ra, rb) {
                if d < (*ba).min(*bb) {
                    return;
                }
            }
            meet_end(ea.as_deref(), Some(nb), out);
            meet_end(eb.as_deref(), Some(na), out);
            if ba == bb {
                meet_nodes(ca[0].as_deref(), cb[0].as_deref(), out);
                meet_nodes(ca[1].as_deref(), cb[1].as_deref(), out);
            } else if ba < bb {
                meet_nodes(ca[key_bit(rb, *ba)].as_deref(), Some(nb), out);
            } else {
                meet_nodes(Some(na), cb[key_bit(ra, *bb)].as_deref(), out);
            }
        }
    }
}

pub(crate) fn branch<V>(
    bit: usize,
    children: [Option<Box<Node<V>>>; 2],
    end: Option<Box<(Vec<u8>, V)>>,
) -> Box<Node<V>> {
    Box::new(Node::Branch { bit, children, end })
}

pub(crate) fn collapse<V>(slot: &mut Option<Box<Node<V>>>) {
    let Some(Node::Branch { children, end, .. }) = slot.as_deref_mut() else {
        return;
    };
    let n0 = children[0].is_some();
    let n1 = children[1].is_some();
    let has_end = end.is_some();
    match (n0, n1, has_end) {
        (false, false, true) => {
            let (k, v) = *end.take().unwrap();
            *slot = Some(Box::new(Node::Leaf(k, v)));
        }
        (true, false, false) => *slot = children[0].take(),
        (false, true, false) => *slot = children[1].take(),
        _ => {}
    }
}

/// Insert into the subtree at `slot`, splitting at bit `d` (the first bit
/// at which `key` differs from the leaf its path reaches).
pub(crate) fn insert_at<V>(slot: &mut Option<Box<Node<V>>>, key: Vec<u8>, value: V, d: usize) {
    let key_ends = Node::<V>::bits(&key) == d;
    let descend = match slot.as_deref() {
        Some(Node::Branch { bit, .. }) if *bit < d => Some(key_bit(&key, *bit)),
        Some(Node::Branch { bit, .. }) if *bit == d && !key_ends => Some(key_bit(&key, *bit)),
        _ => None,
    };
    if let Some(b) = descend {
        if let Some(Node::Branch { children, .. }) = slot.as_deref_mut() {
            insert_at(&mut children[b], key, value, d);
        }
        return;
    }
    if let Some(Node::Branch { bit, end, .. }) = slot.as_deref_mut() {
        if *bit == d && key_ends {
            *end = Some(Box::new((key, value)));
            return;
        }
    }
    let old = slot.take();
    if key_ends {
        let side = old.as_ref().map(|n| key_bit(n.representative(), d)).unwrap_or(0);
        let mut children: [Option<Box<Node<V>>>; 2] = [None, None];
        children[side] = old;
        *slot = Some(branch(d, children, Some(Box::new((key, value)))));
        return;
    }
    let old_ends = match old.as_deref() {
        Some(Node::Leaf(k, _)) if Node::<V>::bits(k) == d => true,
        Some(Node::Branch { bit, end, .. }) if *bit == d && end.is_some() => {
            // old already splits at d; should have descended. Fall through.
            false
        }
        _ => false,
    };
    if old_ends {
        let Some(Node::Leaf(ok, ov)) = old.map(|b| *b) else {
            return;
        };
        let side = key_bit(&key, d);
        let mut children: [Option<Box<Node<V>>>; 2] = [None, None];
        children[side] = Some(Box::new(Node::Leaf(key, value)));
        *slot = Some(branch(d, children, Some(Box::new((ok, ov)))));
        return;
    }
    let bit_d = key_bit(&key, d);
    let leaf = Some(Box::new(Node::Leaf(key, value)));
    let children = match bit_d {
        0 => [leaf, old],
        _ => [old, leaf],
    };
    *slot = Some(branch(d, children, None));
}

/// Remove `key` from the subtree at `slot`; a branch left with one child
/// and no `end` collapses into it.
pub(crate) fn remove_at<V>(slot: &mut Option<Box<Node<V>>>, key: &[u8]) -> Option<V> {
    let is_leaf = matches!(slot.as_deref(), Some(Node::Leaf(k, _)) if k.as_slice() == key);
    if is_leaf {
        return match slot.take().map(|b| *b) {
            Some(Node::Leaf(_, v)) => Some(v),
            _ => None,
        };
    }
    let Some(Node::Branch { bit, children, end }) = slot.as_deref_mut() else {
        return None;
    };
    if Node::<V>::bits(key) == *bit {
        let taken = end.take().and_then(|e| {
            let (k, v) = *e;
            (k.as_slice() == key).then_some(v)
        });
        if taken.is_some() {
            collapse(slot);
        }
        return taken;
    }
    if Node::<V>::bits(key) < *bit {
        return None;
    }
    let b = key_bit(key, *bit);
    let removed = remove_at(&mut children[b], key);
    if removed.is_some() {
        collapse(slot);
    }
    removed
}

pub(crate) fn key_in_fiber(key: &[u8], prefix: &[u8]) -> bool {
    prefix.is_empty() || key.starts_with(prefix)
}

pub(crate) fn clear_after_bit(key: &[u8], bit: usize) -> Vec<u8> {
    let start = bit.saturating_add(1);
    let byte = start / 8;
    let off = start % 8;
    if byte >= key.len() {
        return key.to_vec();
    }
    let mut p = key[..=byte].to_vec();
    if off == 0 {
        p.truncate(byte);
    } else if let Some(last) = p.last_mut() {
        *last &= 0xffu8 << (8 - off);
    }
    p
}
