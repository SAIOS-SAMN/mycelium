// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE NERVE — keyed relations over 32-byte identities and the XOR router
//! that addresses a peer deterministically by a task's key. The Bloom
//! adjacency table that once stood here was write-only on the live path
//! and is excised (2026-09-03); the router is the peer topology.

/// XOR ROUTING TRIE — the topological structure the Kademlia metric induces.
/// `closest(key)` descends the key's bits greedily (matching child when present,
/// sibling otherwise): because XOR is dominated by the highest differing bit,
/// agreeing with the key at the earliest bit minimizes XOR — so the greedy leaf
/// IS the XOR-closest, in O(bits), INDEPENDENT of N and never a scan of the peer
/// set. (Correct + simple; path-compression to O(N) nodes is tracked as R5's
/// follow-up — correctness first.)
/// A KEYED RELATION over 32-byte identities: a value reached by its key's
/// bits, O(bits) per read and write, never a scan of the relation. The
/// carrier every address-keyed fact in the engine is reached through —
/// listings spoken on, deflations, roots, a language's words — so an
/// endpoint is always read from the key, whatever the relation's size.
#[derive(Clone, Debug)]
pub struct Keyed<V> {
    root: Option<Box<Node<V>>>,
}

/// A path-compressed binary trie (PATRICIA): a branch only where two keys
/// first differ, a leaf holding its whole key. One node per key, so the
/// relation costs what it holds, and every operation is O(bits).
#[derive(Clone, Debug)]
enum Node<V> {
    Leaf([u8; 32], V),
    /// Both children are always bound; the option is the slot a split or
    /// a collapse works through.
    Branch { bit: usize, children: [Option<Box<Node<V>>>; 2] },
}

impl<V> Default for Keyed<V> {
    fn default() -> Self {
        Keyed { root: None }
    }
}

fn key_bit(key: &[u8; 32], i: usize) -> usize {
    ((key[i / 8] >> (7 - (i % 8))) & 1) as usize
}

/// The first bit at which two keys differ, if they differ.
fn first_difference(a: &[u8; 32], b: &[u8; 32]) -> Option<usize> {
    for i in 0..32 {
        let x = a[i] ^ b[i];
        if x != 0 {
            return Some(i * 8 + x.leading_zeros() as usize);
        }
    }
    None
}

impl<V> Node<V> {
    /// The key of the leaf this key's path reaches.
    fn held_key(&self, key: &[u8; 32]) -> [u8; 32] {
        let mut node = self;
        loop {
            match node {
                Node::Leaf(k, _) => return *k,
                Node::Branch { bit, children } => {
                    node = children[key_bit(key, *bit)].as_deref().expect("a branch holds two children");
                }
            }
        }
    }

    fn get(&self, key: &[u8; 32]) -> Option<&V> {
        let mut node = self;
        loop {
            match node {
                Node::Leaf(k, v) => return (k == key).then_some(v),
                Node::Branch { bit, children } => node = children[key_bit(key, *bit)].as_deref()?,
            }
        }
    }

    fn get_mut(&mut self, key: &[u8; 32]) -> Option<&mut V> {
        let mut node = self;
        loop {
            match node {
                Node::Leaf(k, v) => return (k == key).then_some(v),
                Node::Branch { bit, children } => {
                    let b = key_bit(key, *bit);
                    node = children[b].as_deref_mut()?;
                }
            }
        }
    }

    fn walk<'a>(&'a self, out: &mut Vec<(&'a [u8; 32], &'a V)>) {
        match self {
            Node::Leaf(k, v) => out.push((k, v)),
            Node::Branch { children, .. } => {
                for c in children.iter().flatten() {
                    c.walk(out);
                }
            }
        }
    }
}

/// Insert into the subtree at `slot`, splitting at bit `d` (the first bit
/// at which `key` differs from the leaf its path reaches).
fn insert_at<V>(slot: &mut Option<Box<Node<V>>>, key: [u8; 32], value: V, d: usize) {
    let descend = match slot.as_deref() {
        Some(Node::Branch { bit, .. }) if *bit < d => Some(key_bit(&key, *bit)),
        _ => None,
    };
    match descend {
        Some(b) => {
            if let Some(Node::Branch { children, .. }) = slot.as_deref_mut() {
                insert_at(&mut children[b], key, value, d);
            }
        }
        None => {
            // Split here: the old subtree and the new leaf under a branch
            // at bit d, ordered by the key's bit.
            let old = slot.take();
            let leaf = Some(Box::new(Node::Leaf(key, value)));
            let children = match key_bit(&key, d) {
                0 => [leaf, old],
                _ => [old, leaf],
            };
            *slot = Some(Box::new(Node::Branch { bit: d, children }));
        }
    }
}

/// Remove `key` from the subtree at `slot`; a branch left with one child
/// collapses into it.
fn remove_at<V>(slot: &mut Option<Box<Node<V>>>, key: &[u8; 32]) -> Option<V> {
    let is_leaf = matches!(slot.as_deref(), Some(Node::Leaf(k, _)) if k == key);
    if is_leaf {
        return match slot.take().map(|b| *b) {
            Some(Node::Leaf(_, v)) => Some(v),
            _ => None,
        };
    }
    let Some(Node::Branch { bit, children }) = slot.as_deref_mut() else { return None };
    let b = key_bit(key, *bit);
    let removed = remove_at(&mut children[b], key);
    if children[b].is_none() {
        let other = children[1 - b].take();
        *slot = other;
    }
    removed
}

impl<V> Keyed<V> {
    pub fn new() -> Self {
        Keyed::default()
    }

    /// Bind `key` to `value`; a held key is rebound. Returns the value
    /// displaced, if any.
    pub fn insert(&mut self, key: [u8; 32], value: V) -> Option<V> {
        let Some(root) = self.root.as_deref() else {
            self.root = Some(Box::new(Node::Leaf(key, value)));
            return None;
        };
        let held = root.held_key(&key);
        match first_difference(&key, &held) {
            None => self.root.as_deref_mut()?.get_mut(&key).map(|v| std::mem::replace(v, value)),
            Some(d) => {
                insert_at(&mut self.root, key, value, d);
                None
            }
        }
    }

    pub fn get(&self, key: &[u8; 32]) -> Option<&V> {
        self.root.as_deref()?.get(key)
    }

    pub fn get_mut(&mut self, key: &[u8; 32]) -> Option<&mut V> {
        self.root.as_deref_mut()?.get_mut(key)
    }

    pub fn contains(&self, key: &[u8; 32]) -> bool {
        self.get(key).is_some()
    }

    /// Unbind `key`; the value it held, if any.
    pub fn remove(&mut self, key: &[u8; 32]) -> Option<V> {
        remove_at(&mut self.root, key)
    }

    /// Every bound (key, value), in key order — a walk of the relation for
    /// persistence and for measurements over the whole, never for lookup.
    pub fn entries(&self) -> Vec<(&[u8; 32], &V)> {
        let mut out = Vec::new();
        if let Some(root) = self.root.as_deref() {
            root.walk(&mut out);
        }
        out
    }
}

#[cfg(test)]
mod keyed_tests {
    use super::Keyed;

    /// Keys bind, rebind, read, and unbind exactly; a branch left with one
    /// child collapses; the walk lists everything.
    #[test]
    fn the_keyed_relation_is_exact() {
        let mut k: Keyed<u32> = Keyed::new();
        let a = *blake3::hash(b"a").as_bytes();
        let b = *blake3::hash(b"b").as_bytes();
        let c = *blake3::hash(b"c").as_bytes();
        assert_eq!(k.insert(a, 1), None);
        assert_eq!(k.insert(b, 2), None);
        assert_eq!(k.insert(c, 3), None);
        assert_eq!(k.get(&a), Some(&1));
        assert_eq!(k.get(&b), Some(&2));
        assert_eq!(k.get(&c), Some(&3));
        assert_eq!(k.insert(b, 20), Some(2));
        assert_eq!(k.get(&b), Some(&20));
        assert_eq!(k.entries().len(), 3);
        assert_eq!(k.remove(&b), Some(20));
        assert_eq!(k.get(&b), None);
        assert_eq!(k.get(&a), Some(&1));
        assert_eq!(k.get(&c), Some(&3));
        assert_eq!(k.remove(&a), Some(1));
        assert_eq!(k.remove(&a), None);
        assert_eq!(k.entries().len(), 1);
        assert_eq!(k.get(&c), Some(&3));
    }
}

#[derive(Default)]
pub struct Router {
    children: [Option<Box<Router>>; 2],
    leaf: Option<([u8; 32], String)>,
}

impl Router {
    pub fn new() -> Self {
        Router::default()
    }

    fn bit(id: &[u8; 32], i: usize) -> usize {
        ((id[i / 8] >> (7 - (i % 8))) & 1) as usize
    }

    pub fn insert(&mut self, id: [u8; 32], addr: String) {
        let mut node = self;
        for depth in 0..256 {
            let b = Self::bit(&id, depth);
            node = node.children[b].get_or_insert_with(|| Box::new(Router::new()));
        }
        node.leaf = Some((id, addr));
    }

    /// The contact XOR-closest to `key` — O(bits), never O(N).
    pub fn closest(&self, key: &[u8; 32]) -> Option<([u8; 32], String)> {
        self.near(key, 0, &[])
    }

    /// The contact XOR-closest to `key` among those not excluded — the next
    /// hop on the nerve when the closest is the speaker itself or the one
    /// it heard from. Deterministic: the same key walks the same path from
    /// every node.
    pub fn closest_excluding(&self, key: &[u8; 32], exclude: &[[u8; 32]]) -> Option<([u8; 32], String)> {
        self.near(key, 0, exclude)
    }

    /// Does the router hold this exact identity — a peer it was configured
    /// with, the only kind whose speech the nerve accepts?
    pub fn knows(&self, id: &[u8; 32]) -> bool {
        self.closest(id).is_some_and(|(held, _)| held == *id)
    }

    fn near(&self, key: &[u8; 32], depth: usize, exclude: &[[u8; 32]]) -> Option<([u8; 32], String)> {
        if depth == 256 {
            return self.leaf.clone().filter(|(id, _)| !exclude.contains(id));
        }
        let b = Self::bit(key, depth);
        if let Some(c) = &self.children[b] {
            if let Some(hit) = c.near(key, depth + 1, exclude) {
                return Some(hit);
            }
        }
        self.children[1 - b].as_ref().and_then(|c| c.near(key, depth + 1, exclude))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn xor(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
        std::array::from_fn(|i| a[i] ^ b[i])
    }

    /// The trie's closest matches a brute-force XOR argmin — but by descent, not
    /// a scan. (Correctness anchor; the trie is O(bits), the brute-force O(N).)
    #[test]
    fn trie_closest_matches_brute_force_without_scanning() {
        let ids: Vec<[u8; 32]> = (0u8..40).map(|k| *blake3::hash(&[k]).as_bytes()).collect();
        let mut r = Router::new();
        for (i, id) in ids.iter().enumerate() {
            r.insert(*id, format!("peer{i}"));
        }
        for k in [7u8, 19, 33, 200] {
            let key = *blake3::hash(&[k, 0xAA]).as_bytes();
            let brute = ids.iter().min_by_key(|id| xor(id, &key)).copied().unwrap();
            assert_eq!(r.closest(&key).unwrap().0, brute, "trie descent finds the XOR-closest");
        }
    }

}
