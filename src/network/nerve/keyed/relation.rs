// A13 — THE KEYED RELATION: what a mind reads by key, and the reads it
// exposes.

use super::*;

/// XOR ROUTING TRIE — the topological structure the Kademlia metric induces.
/// `closest(key)` descends the key's bits greedily (matching child when present,
/// sibling otherwise): because XOR is dominated by the highest differing bit,
/// agreeing with the key at the earliest bit minimizes XOR — so the greedy leaf
/// IS the XOR-closest, in O(bits), INDEPENDENT of N and never a scan of the peer
/// set. (Correct + simple; path-compression to O(N) nodes is tracked as R5's
/// follow-up — correctness first.)
/// A KEYED RELATION over A21 coordinate bytes: a value reached by its
/// key's bits, O(bits) per read and write, never a scan. A prefix of a
/// key is a coarser stratum (`fiber`). Digests are not keys.
#[derive(Clone, Debug)]
pub struct Keyed<V> {
    root: Option<Box<Node<V>>>,
    n: usize,
}

impl<V> Default for Keyed<V> {
    fn default() -> Self {
        Keyed { root: None, n: 0 }
    }
}

impl<V> Keyed<V> {
    pub fn new() -> Self {
        Keyed::default()
    }

    /// Bind `key` to `value`; a held key is rebound. Returns the value
    /// displaced, if any.
    pub fn insert(&mut self, key: impl AsRef<[u8]>, value: V) -> Option<V> {
        #[cfg(test)]
        work::count_insert();
        let key = key.as_ref().to_vec();
        let Some(root) = self.root.as_deref() else {
            self.root = Some(Box::new(Node::Leaf(key, value)));
            self.n = 1;
            return None;
        };
        let held = root.held_key(&key).to_vec();
        match first_difference(&key, &held) {
            None => self.root.as_deref_mut()?.get_mut(&key).map(|v| std::mem::replace(v, value)),
            Some(d) => {
                insert_at(&mut self.root, key, value, d);
                self.n += 1;
                None
            }
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<&V> {
        #[cfg(test)]
        work::count_get();
        self.root.as_deref()?.get(key)
    }

    pub fn get_mut(&mut self, key: &[u8]) -> Option<&mut V> {
        self.root.as_deref_mut()?.get_mut(key)
    }

    pub fn contains(&self, key: &[u8]) -> bool {
        self.get(key).is_some()
    }

    /// Unbind `key`; the value it held, if any.
    pub fn remove(&mut self, key: &[u8]) -> Option<V> {
        let v = remove_at(&mut self.root, key);
        if v.is_some() {
            self.n = self.n.saturating_sub(1);
        }
        v
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Keys present in both carriers. Simultaneous PATRICIA descent (A13):
    /// subtrees that share no prefix are pruned; cost is the shared
    /// structure, never a product of the two fibers.
    pub fn meet<W>(&self, other: &Keyed<W>) -> Keyed<()> {
        let mut out = Keyed::new();
        meet_nodes(self.root.as_deref(), other.root.as_deref(), &mut out);
        out
    }

    /// Subtree under `prefix`. The empty prefix [`ROOT`] is this carrier's
    /// own fiber — valid when the Keyed is already scoped (one bubble's
    /// charts, one vertex's star, one chart's incidence).
    pub fn fiber(&self, prefix: &[u8]) -> Fiber<'_, V> {
        let mut stack = Vec::new();
        if let Some(root) = self.root.as_deref() {
            stack.push(root);
        }
        Fiber { stack, prefix: prefix.to_vec() }
    }

    /// Parent shell of `key`: the branch that split this leaf. None when
    /// the key is unbound or is the sole leaf.
    pub fn peel(&self, key: &[u8]) -> Option<Shell<'_, V>> {
        let mut node = self.root.as_deref()?;
        let mut parent: Option<&Node<V>> = None;
        let mut parent_bit = 0usize;
        loop {
            match node {
                Node::Leaf(k, _) => {
                    if k.as_slice() != key {
                        return None;
                    }
                    let p = parent?;
                    return Some(Shell {
                        fiber: Fiber { stack: vec![p], prefix: ROOT.to_vec() },
                        prefix: clear_after_bit(key, parent_bit),
                    });
                }
                Node::Branch { bit, children, end } => {
                    if Node::<V>::bits(key) == *bit {
                        let (k, _) = end.as_deref()?;
                        if k.as_slice() != key {
                            return None;
                        }
                        return Some(Shell {
                            fiber: Fiber { stack: vec![node], prefix: ROOT.to_vec() },
                            prefix: clear_after_bit(key, *bit),
                        });
                    }
                    parent = Some(node);
                    parent_bit = *bit;
                    node = children[key_bit(key, *bit)].as_deref()?;
                }
            }
        }
    }

    /// At most `budget` entries in key order after `from`. [`ROOT`] starts
    /// at the first key. The last yielded key is the resume cursor.
    pub fn cursor<'a>(&'a self, from: &[u8], budget: &'a mut crate::physics::Budget) -> Cursor<'a, V> {
        Cursor {
            fiber: self.fiber(ROOT),
            budget,
            from: from.to_vec(),
            last: if from.is_empty() { None } else { Some(from.to_vec()) },
        }
    }
}

impl<V> Keyed<Keyed<V>> {
    /// Insert at `(a, b)` without rebuilding the inner trie when `a` is held.
    pub fn nested_insert(&mut self, a: impl AsRef<[u8]>, b: impl AsRef<[u8]>, value: V) {
        let a = a.as_ref();
        if let Some(inner) = self.get_mut(a) {
            inner.insert(b, value);
            return;
        }
        let mut inner = Keyed::new();
        inner.insert(b, value);
        self.insert(a, inner);
    }
}

/// The empty prefix: the root fiber of a keyed that is already a fiber.
pub const ROOT: &[u8] = &[];

/// Sealed persist walk: only [`crate::snapshot`] names this.
pub(crate) fn keyed_persist_walk<V>(k: &Keyed<V>, f: &mut impl FnMut(&[u8], &V)) {
    for (key, v) in k.fiber(ROOT) {
        f(&key, v);
    }
}
