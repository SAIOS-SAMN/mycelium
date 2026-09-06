// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE NERVE — keyed relations over 32-byte identities and the Cover trie
//! that homes a word by incidence: longest-common-prefix descent on the
//! chart's lane, never XOR. Join peers seed the first digest; after that
//! the trie is the topology.

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

/// A path-compressed binary trie (PATRICIA): a branch only where two keys
/// first differ, a leaf holding its whole key. One node per key, so the
/// relation costs what it holds, and every operation is O(bits).
#[derive(Clone, Debug)]
enum Node<V> {
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

impl<V> Default for Keyed<V> {
    fn default() -> Self {
        Keyed { root: None, n: 0 }
    }
}

fn key_bit(key: &[u8], i: usize) -> usize {
    let byte = i / 8;
    if byte >= key.len() {
        return 0;
    }
    ((key[byte] >> (7 - (i % 8))) & 1) as usize
}

/// The first bit at which two keys differ, if they differ.
fn first_difference(a: &[u8], b: &[u8]) -> Option<usize> {
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
    fn bits(key: &[u8]) -> usize {
        key.len().saturating_mul(8)
    }

    /// The key of the leaf this key's path reaches.
    fn held_key(&self, key: &[u8]) -> &[u8] {
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

    fn get(&self, key: &[u8]) -> Option<&V> {
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

    fn get_mut(&mut self, key: &[u8]) -> Option<&mut V> {
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

    fn representative(&self) -> &[u8] {
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

fn meet_end<V, W>(end: Option<&(Vec<u8>, V)>, other: Option<&Node<W>>, out: &mut Keyed<()>) {
    let Some((k, _)) = end else { return };
    if other.is_some_and(|n| n.get(k).is_some()) {
        out.insert(k.as_slice(), ());
    }
}

fn meet_nodes<V, W>(a: Option<&Node<V>>, b: Option<&Node<W>>, out: &mut Keyed<()>) {
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

fn branch<V>(
    bit: usize,
    children: [Option<Box<Node<V>>>; 2],
    end: Option<Box<(Vec<u8>, V)>>,
) -> Box<Node<V>> {
    Box::new(Node::Branch { bit, children, end })
}

fn collapse<V>(slot: &mut Option<Box<Node<V>>>) {
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
fn insert_at<V>(slot: &mut Option<Box<Node<V>>>, key: Vec<u8>, value: V, d: usize) {
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
fn remove_at<V>(slot: &mut Option<Box<Node<V>>>, key: &[u8]) -> Option<V> {
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

/// Work counters for the enumeration gate. Incremented in test builds on
/// Keyed get/insert, Rotor::geo, and cech::extend.
#[cfg(test)]
pub mod work {
    use std::cell::Cell;
    thread_local! {
        static GETS: Cell<u64> = const { Cell::new(0) };
        static INSERTS: Cell<u64> = const { Cell::new(0) };
        static GEOS: Cell<u64> = const { Cell::new(0) };
        static EXTENDS: Cell<u64> = const { Cell::new(0) };
    }
    pub fn reset() {
        GETS.with(|c| c.set(0));
        INSERTS.with(|c| c.set(0));
        GEOS.with(|c| c.set(0));
        EXTENDS.with(|c| c.set(0));
    }
    pub fn snapshot() -> (u64, u64, u64, u64) {
        (
            GETS.with(|c| c.get()),
            INSERTS.with(|c| c.get()),
            GEOS.with(|c| c.get()),
            EXTENDS.with(|c| c.get()),
        )
    }
    pub fn total() -> u64 {
        let (a, b, c, d) = snapshot();
        a.saturating_add(b).saturating_add(c).saturating_add(d)
    }
    pub fn count_get() {
        GETS.with(|c| c.set(c.get().saturating_add(1)));
    }
    pub fn count_insert() {
        INSERTS.with(|c| c.set(c.get().saturating_add(1)));
    }
    pub fn count_geo() {
        GEOS.with(|c| c.set(c.get().saturating_add(1)));
    }
    pub fn count_extend() {
        EXTENDS.with(|c| c.set(c.get().saturating_add(1)));
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
                        let Some((k, _)) = end.as_deref() else { return None };
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

fn key_in_fiber(key: &[u8], prefix: &[u8]) -> bool {
    prefix.is_empty() || key.starts_with(prefix)
}

fn clear_after_bit(key: &[u8], bit: usize) -> Vec<u8> {
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

/// Subtree under a key prefix. The only read of more than one entry that
/// is not a link, a peel, or a budgeted cursor.
pub struct Fiber<'a, V> {
    stack: Vec<&'a Node<V>>,
    prefix: Vec<u8>,
}

impl<'a, V> Fiber<'a, V> {
    fn empty() -> Self {
        Fiber { stack: Vec::new(), prefix: ROOT.to_vec() }
    }
}

impl<'a, V> Iterator for Fiber<'a, V> {
    type Item = (Vec<u8>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            match node {
                Node::Leaf(k, v) => {
                    if key_in_fiber(k, &self.prefix) {
                        return Some((k.clone(), v));
                    }
                }
                Node::Branch { children, end, .. } => {
                    if let Some(c) = children[1].as_deref() {
                        self.stack.push(c);
                    }
                    if let Some(c) = children[0].as_deref() {
                        self.stack.push(c);
                    }
                    if let Some((k, v)) = end.as_deref() {
                        if key_in_fiber(k, &self.prefix) {
                            return Some((k.clone(), v));
                        }
                    }
                }
            }
        }
        None
    }
}

/// Entries incident to one key: the inner trie at that key.
pub struct Link<'a, V> {
    fiber: Fiber<'a, V>,
}

impl<'a, V> Iterator for Link<'a, V> {
    type Item = (Vec<u8>, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        self.fiber.next()
    }
}

impl<T> Keyed<Keyed<T>> {
    /// The inner trie at `key` — cells on a vertex, bubbles on a chart.
    pub fn link(&self, key: &[u8]) -> Link<'_, T> {
        Link {
            fiber: match self.get(key) {
                Some(inner) => inner.fiber(ROOT),
                None => Fiber::empty(),
            },
        }
    }
}

/// Budgeted walk in key order from `from`, consuming a [`crate::physics::Budget`].
pub struct Cursor<'a, V> {
    fiber: Fiber<'a, V>,
    budget: &'a mut crate::physics::Budget,
    from: Vec<u8>,
    last: Option<Vec<u8>>,
}

impl<'a, V> Cursor<'a, V> {
    /// Last key yielded, so the next stroke resumes after it.
    pub fn last_key(&self) -> Option<&[u8]> {
        self.last.as_deref()
    }
}

impl<'a, V> Iterator for Cursor<'a, V> {
    type Item = (Vec<u8>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.budget.remaining() == 0 {
                return None;
            }
            let (k, v) = self.fiber.next()?;
            if !self.from.is_empty() && k.as_slice() <= self.from.as_slice() {
                continue;
            }
            if !self.budget.spend() {
                return None;
            }
            self.last = Some(k.clone());
            return Some((k, v));
        }
    }
}

/// One shell: the parent prefix of a peel, and the entries at that branch.
pub struct Shell<'a, V> {
    fiber: Fiber<'a, V>,
    prefix: Vec<u8>,
}

impl<'a, V> Shell<'a, V> {
    pub fn key(&self) -> &[u8] {
        &self.prefix
    }
}

impl<'a, V> Iterator for Shell<'a, V> {
    type Item = (Vec<u8>, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        self.fiber.next()
    }
}

/// Sealed persist walk: only [`crate::snapshot`] names this.
pub(crate) fn keyed_persist_walk<V>(k: &Keyed<V>, f: &mut impl FnMut(&[u8], &V)) {
    for (key, v) in k.fiber(ROOT) {
        f(&key, v);
    }
}

#[cfg(test)]
mod keyed_tests {
    use super::{Keyed, ROOT};

    /// Keys bind, rebind, read, and unbind exactly; a branch left with one
    /// child collapses; the walk lists everything. Prefix reads a stratum.
    #[test]
    fn the_keyed_relation_is_exact() {
        let mut k: Keyed<u32> = Keyed::new();
        let a = b"a".as_slice();
        let b = b"b".as_slice();
        let c = b"c".as_slice();
        assert_eq!(k.insert(a, 1), None);
        assert_eq!(k.insert(b, 2), None);
        assert_eq!(k.insert(c, 3), None);
        assert_eq!(k.get(a), Some(&1));
        assert_eq!(k.get(b), Some(&2));
        assert_eq!(k.get(c), Some(&3));
        assert_eq!(k.insert(b, 20), Some(2));
        assert_eq!(k.get(b), Some(&20));
        assert_eq!(k.len(), 3);
        assert_eq!(k.fiber(ROOT).count(), 3);
        assert_eq!(k.remove(b), Some(20));
        assert_eq!(k.get(b), None);
        assert_eq!(k.get(a), Some(&1));
        assert_eq!(k.get(c), Some(&3));
        assert_eq!(k.remove(a), Some(1));
        assert_eq!(k.remove(a), None);
        assert_eq!(k.len(), 1);
        assert_eq!(k.fiber(ROOT).count(), 1);
        assert_eq!(k.get(c), Some(&3));
    }

    #[test]
    fn a_prefix_of_a_key_is_its_stratum() {
        let mut k: Keyed<u8> = Keyed::new();
        k.insert([1u8, 10, 0], 1);
        k.insert([1u8, 10, 1], 2);
        k.insert([1u8, 11, 0], 3);
        k.insert([2u8, 0, 0], 4);
        let coarse: Vec<_> = k.fiber(&[1]).map(|(key, v)| (key.to_vec(), *v)).collect();
        assert_eq!(coarse.len(), 3, "prefix [1] is the chart's coarse stratum");
        assert!(coarse.iter().all(|(key, _)| key.starts_with(&[1])));
        let mid: Vec<_> = k.fiber(&[1, 10]).map(|(key, _)| key.to_vec()).collect();
        assert_eq!(mid.len(), 2);
        assert!(mid.iter().all(|key| key.starts_with(&[1, 10])));
        assert_eq!(k.fiber(&[1, 10, 0]).count(), 1);
        assert_eq!(k.get(&[2, 0, 0]), Some(&4));
    }

    #[test]
    fn a_key_and_its_proper_prefix_coexist() {
        let mut k: Keyed<u8> = Keyed::new();
        k.insert([1u8, 0], 1);
        k.insert([1u8], 2);
        assert_eq!(k.get(&[1]), Some(&2), "the prefix is bound");
        assert_eq!(k.get(&[1, 0]), Some(&1), "the extension whose next bit is 0 is bound");
        assert_eq!(k.len(), 2);
        let mut k2: Keyed<u8> = Keyed::new();
        k2.insert([1u8], 2);
        k2.insert([1u8, 0], 1);
        assert_eq!(k2.get(&[1]), Some(&2));
        assert_eq!(k2.get(&[1, 0]), Some(&1));
        assert_eq!(k2.remove(&[1]), Some(2));
        assert_eq!(k2.get(&[1]), None);
        assert_eq!(k2.get(&[1, 0]), Some(&1));
        assert_eq!(k2.len(), 1);
    }

    #[test]
    fn meet_is_intersection_and_costs_shared_prefixes() {
        use super::work;
        let key = |n: u8| vec![n];
        let mut a: Keyed<u8> = Keyed::new();
        let mut b: Keyed<u8> = Keyed::new();
        a.insert(key(1), 1);
        a.insert(key(2), 2);
        a.insert(key(3), 3);
        b.insert(key(2), 20);
        b.insert(key(3), 30);
        b.insert(key(4), 40);
        let m = a.meet(&b);
        assert!(m.contains(&key(2)) && m.contains(&key(3)));
        assert!(!m.contains(&key(1)) && !m.contains(&key(4)));
        let run = |n: usize| {
            work::reset();
            let mut big: Keyed<u8> = Keyed::new();
            let mut small: Keyed<u8> = Keyed::new();
            small.insert(key(1), 1);
            small.insert(key(2), 2);
            for i in 0..n {
                big.insert([10u8, i as u8], 1);
            }
            big.insert(key(1), 1);
            let _ = big.meet(&small);
            work::total()
        };
        let w = run(8);
        let w4 = run(32);
        assert!(w > 0 && w4 <= w.saturating_mul(8), "meet cost follows shared prefixes, not |A|×|B|: {w} then {w4}");
    }
}

/// An invariant axis as a Keyed key: the coordinate little-endian, so a
/// prefix of axes is a prefix of the plane.
pub fn axis_key_le(a: u64) -> [u8; 8] {
    a.to_le_bytes()
}

fn axis_of_le(key: &[u8]) -> u64 {
    let mut b = [0u8; 8];
    let n = key.len().min(8);
    b[..n].copy_from_slice(&key[..n]);
    u64::from_le_bytes(b)
}

/// Cover trie: per lane, nested Keyed by invariant axes. The fiber at a
/// node is the hyphas covering that plane (this prefix and every finer
/// chart beneath it). Own charts and peer charts share the trie.
#[derive(Clone, Debug)]
pub struct Cover {
    lanes: [CoverNode; 4],
    /// Cover root per lane, hashed in trie order on admit/retract.
    roots: [[u8; 32]; 4],
    /// Charts each hypha holds, so retract walks those paths only.
    held: Keyed<Vec<(u8, crate::transducer::stalk::Invariant)>>,
}

fn empty_cover_root() -> [u8; 32] {
    *blake3::Hasher::new().finalize().as_bytes()
}

impl Default for Cover {
    fn default() -> Self {
        Cover {
            lanes: Default::default(),
            roots: [empty_cover_root(); 4],
            held: Keyed::new(),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct CoverNode {
    here: Keyed<()>,
    next: Keyed<CoverNode>,
}

impl CoverNode {
    fn is_empty(&self) -> bool {
        self.here.is_empty() && self.next.is_empty()
    }

    fn admit(&mut self, axes: &[u64], hypha: &[u8]) {
        match axes.split_first() {
            None => {
                self.here.insert(hypha, ());
            }
            Some((&a, rest)) => {
                let k = axis_key_le(a);
                match self.next.get_mut(&k) {
                    Some(ch) => ch.admit(rest, hypha),
                    None => {
                        let mut ch = CoverNode::default();
                        ch.admit(rest, hypha);
                        self.next.insert(k, ch);
                    }
                }
            }
        }
    }

    fn collect_into(&self, out: &mut Keyed<()>) {
        for (id, _) in self.here.fiber(ROOT) {
            out.insert(id, ());
        }
        for (_, child) in self.next.fiber(ROOT) {
            child.collect_into(out);
        }
    }

    fn collect(&self) -> Vec<Vec<u8>> {
        let mut out = Keyed::new();
        self.collect_into(&mut out);
        out.fiber(ROOT).map(|(k, _)| k).collect()
    }

    /// Longest common prefix: descend while axes match. A miss after some
    /// match returns the fiber at that plane. A miss at the first axis is
    /// novel — empty. The empty chart (no axes) is the root fiber.
    fn lcp(&self, axes: &[u64], matched: bool) -> Vec<Vec<u8>> {
        match axes.split_first() {
            None => self.collect(),
            Some((&a, rest)) => match self.next.get(&axis_key_le(a)) {
                Some(child) => child.lcp(rest, true),
                None if matched => self.collect(),
                None => Vec::new(),
            },
        }
    }

    /// Retract one hypha along one chart path (the hypha's own axes).
    fn retract_path(&mut self, axes: &[u64], hypha: &[u8]) {
        match axes.split_first() {
            None => {
                self.here.remove(hypha);
            }
            Some((&a, rest)) => {
                let k = axis_key_le(a);
                let empty = {
                    let Some(ch) = self.next.get_mut(&k) else { return };
                    ch.retract_path(rest, hypha);
                    ch.is_empty()
                };
                if empty {
                    self.next.remove(&k);
                }
            }
        }
    }

    fn hash_charts(&self, hasher: &mut blake3::Hasher, path: &mut Vec<u64>) {
        if !self.here.is_empty() {
            for a in path.iter() {
                hasher.update(&a.to_le_bytes());
            }
            hasher.update(&[0xff]);
        }
        // Trie order is key order: hash without materialising a Vec of charts.
        for (k, ch) in self.next.fiber(ROOT) {
            path.push(axis_of_le(k.as_slice()));
            ch.hash_charts(hasher, path);
            path.pop();
        }
    }

    fn walk(&self, path: &mut Vec<u64>, out: &mut Vec<crate::transducer::stalk::Invariant>) {
        if !self.here.is_empty() {
            out.push(crate::transducer::stalk::Invariant(path.clone()));
        }
        for (k, ch) in self.next.fiber(ROOT) {
            path.push(axis_of_le(k.as_slice()));
            ch.walk(path, out);
            path.pop();
        }
    }
}

impl Cover {
    pub fn new() -> Self {
        Cover::default()
    }

    pub fn admit(&mut self, lane: u8, inv: &crate::transducer::stalk::Invariant, hypha: impl AsRef<[u8]>) {
        let hypha = hypha.as_ref();
        let lane = lane & 3;
        self.lanes[lane as usize].admit(&inv.0, hypha);
        match self.held.get_mut(hypha) {
            Some(charts) => {
                if !charts.iter().any(|(l, i)| *l == lane && i == inv) {
                    charts.push((lane, inv.clone()));
                }
            }
            None => {
                self.held.insert(hypha, vec![(lane, inv.clone())]);
            }
        }
        self.refresh_root(lane);
    }

    /// Drop every chart of this hypha. Retract along the hypha's own chart
    /// paths rather than visiting the whole trie.
    pub fn retract_hypha(&mut self, hypha: &[u8]) {
        let Some(charts) = self.held.remove(hypha) else {
            return;
        };
        let mut touched = [false; 4];
        for (lane, inv) in charts {
            self.lanes[lane as usize].retract_path(&inv.0, hypha);
            touched[lane as usize] = true;
        }
        for (l, t) in touched.iter().enumerate() {
            if *t {
                self.refresh_root(l as u8);
            }
        }
    }

    fn refresh_root(&mut self, lane: u8) {
        let mut h = blake3::Hasher::new();
        self.lanes[(lane as usize) & 3].hash_charts(&mut h, &mut Vec::new());
        self.roots[(lane as usize) & 3] = *h.finalize().as_bytes();
    }

    /// Fiber at the deepest matching prefix on `lane`. Empty at every
    /// prefix means the chart is novel to the mesh. Ordered by the trie's
    /// walk: these identities are the vertices simplex keys are built from.
    pub fn fiber(&self, lane: u8, inv: &crate::transducer::stalk::Invariant) -> Vec<Vec<u8>> {
        self.lanes[(lane as usize) & 3].lcp(&inv.0, false)
    }

    /// Cover multiplicity of a chart: the vertex count of the Čech nerve,
    /// not the word count. Read, never chosen; the room vector is untouched.
    pub fn multiplicity(&self, lane: u8, inv: &crate::transducer::stalk::Invariant) -> usize {
        self.fiber(lane, inv).len()
    }

    /// Retract one hypha from one chart path.
    pub fn retract(
        &mut self,
        lane: u8,
        inv: &crate::transducer::stalk::Invariant,
        hypha: &[u8],
    ) {
        let lane = lane & 3;
        self.lanes[lane as usize].retract_path(&inv.0, hypha);
        self.refresh_root(lane);
    }

    /// Prefix split: one hypha leaves `old` and is admitted under `new`.
    /// Only the refined path is touched.
    pub fn rehome(
        &mut self,
        lane: u8,
        old: &crate::transducer::stalk::Invariant,
        new: &crate::transducer::stalk::Invariant,
        hypha: impl AsRef<[u8]>,
    ) {
        let hypha = hypha.as_ref();
        let lane = lane & 3;
        self.lanes[lane as usize].retract_path(&old.0, hypha);
        self.lanes[lane as usize].admit(&new.0, hypha);
        if let Some(charts) = self.held.get_mut(hypha) {
            for slot in charts.iter_mut() {
                if slot.0 == lane && slot.1 == *old {
                    slot.1 = new.clone();
                }
            }
        }
        self.refresh_root(lane);
    }

    pub fn charts(&self, lane: u8) -> Vec<crate::transducer::stalk::Invariant> {
        let mut out = Vec::new();
        self.lanes[(lane as usize) & 3].walk(&mut Vec::new(), &mut out);
        out
    }

    pub fn all_charts(&self) -> Vec<(u8, crate::transducer::stalk::Invariant)> {
        let mut out = Vec::new();
        for lane in 0..4u8 {
            for inv in self.charts(lane) {
                out.push((lane, inv));
            }
        }
        out
    }

    pub fn root(&self, lane: u8) -> [u8; 32] {
        self.roots[(lane as usize) & 3]
    }

    pub fn roots(&self) -> [[u8; 32]; 4] {
        self.roots
    }
}

/// Room on one lane: that lane's bonded μ times the host's remaining
/// strokes per unit bond on that lane, minus occupancy. Other lanes do
/// not enter. A zero bond yields zero room.
pub fn lane_cover_room(bond: u64, reserve: u64, occupancy: u64, spend: u64) -> u64 {
    let per = (reserve / spend.max(1)).max(1);
    bond.saturating_mul(per).saturating_sub(occupancy)
}

/// Occupied lanes of a magnitude — every lane the word moves, not the first.
pub fn occupied_lanes(mag: &[u64; 4]) -> Vec<u8> {
    let v: Vec<u8> = (0..4u8).filter(|&l| mag[l as usize] > 0).collect();
    if v.is_empty() {
        vec![0]
    } else {
        v
    }
}

fn addr_of(
    id: &[u8; 32],
    addrs: &Keyed<String>,
    self_id: Option<(&[u8; 32], &[u64; 4], &str)>,
) -> Option<String> {
    if let Some((sid, _, saddr)) = self_id {
        if id == sid {
            return Some(saddr.to_string());
        }
    }
    addrs.get(id).cloned()
}

/// Dershowitz–Manna dominance on occupied lanes: `a` wins only when it is
/// at least as large on every occupied lane and larger on one. Inverse of
/// `economy::descends` (more remaining room, never a lane ordering).
fn room_dominates(a: &[u64; 4], b: &[u64; 4], lanes: &[u8]) -> bool {
    lanes.iter().all(|&l| a[l as usize] >= b[l as usize])
        && lanes.iter().any(|&l| a[l as usize] > b[l as usize])
}

/// Home by incidence: intersection of cover fibers on every occupied lane,
/// then remaining-room dominance on those lanes (a hypha wins only when its
/// room is ≥ on every occupied lane and > on one), then identity when rooms
/// are equal or incomparable.
pub fn home(
    cover: &Cover,
    inv: &crate::transducer::stalk::Invariant,
    mag: &[u64; 4],
    rooms: &Keyed<[u64; 4]>,
    addrs: &Keyed<String>,
    self_id: Option<(&[u8; 32], &[u64; 4], &str)>,
) -> ([u8; 32], String) {
    let lanes = occupied_lanes(mag);
    let fits = |room: &[u64; 4]| lanes.iter().all(|&l| mag[l as usize] == 0 || room[l as usize] > 0);
    let room_of = |id: &[u8; 32]| -> Option<[u64; 4]> {
        if let Some((sid, sroom, _)) = self_id {
            if id == sid {
                return Some(*sroom);
            }
        }
        rooms.get(id).copied()
    };
    // Intersection of fibers across occupied lanes, as a trie union/meet.
    let mut fiber: Option<Keyed<()>> = None;
    for l in &lanes {
        let f = cover.fiber(*l, inv);
        let mut next = Keyed::new();
        for id in f {
            if fiber.as_ref().map_or(true, |acc| acc.contains(&id)) {
                next.insert(id, ());
            }
        }
        fiber = Some(next);
    }
    let fiber = fiber.unwrap_or_default();
    let mut best: Option<([u8; 32], String, [u64; 4])> = None;
    let consider = |best: &mut Option<([u8; 32], String, [u64; 4])>, id: [u8; 32], addr: String, room: [u64; 4]| {
        match best {
            None => *best = Some((id, addr, room)),
            Some((bid, _, broom)) => {
                if room_dominates(&room, broom, &lanes) {
                    *best = Some((id, addr, room));
                } else if !room_dominates(broom, &room, &lanes) && id < *bid {
                    // Equal or incomparable: identity is the last tiebreak.
                    *best = Some((id, addr, room));
                }
            }
        }
    };
    for (id, _) in fiber.fiber(ROOT) {
        let Some(vk) = <[u8; 32]>::try_from(id.as_slice()).ok() else { continue };
        if let Some(room) = room_of(&vk) {
            if fits(&room) {
                if let Some(a) = addr_of(&vk, addrs, self_id) {
                    consider(&mut best, vk, a, room);
                }
            }
        }
    }
    if let Some((id, addr, _)) = best {
        return (id, addr);
    }
    // No covering hypha with room: novel (or the fiber is full), room then identity.
    if let Some((sid, sroom, saddr)) = self_id {
        if fits(sroom) {
            consider(&mut best, *sid, saddr.to_string(), *sroom);
        }
    }
    for (id, room) in rooms.fiber(ROOT) {
        if fits(room) {
            let Some(vk) = <[u8; 32]>::try_from(id.as_slice()).ok() else { continue };
            if let Some(a) = addrs.get(&vk) {
                consider(&mut best, vk, a.clone(), *room);
            }
        }
    }
    best.map(|(id, addr, _)| (id, addr)).unwrap_or_else(|| {
        self_id
            .map(|(id, _, a)| (*id, a.to_string()))
            .unwrap_or(([0u8; 32], String::new()))
    })
}

#[cfg(test)]
mod cover_tests {
    use super::*;
    use crate::transducer::stalk::Invariant;

    fn inv(axes: &[u64]) -> Invariant {
        Invariant(axes.to_vec())
    }

    #[test]
    fn cover_lcp_is_the_plane_and_a_miss_at_the_first_axis_is_novel() {
        let mut c = Cover::new();
        let a = [1u8; 32];
        let b = [2u8; 32];
        c.admit(0, &inv(&[1, 2, 3]), a);
        c.admit(0, &inv(&[1, 2, 9]), b);
        let plane = c.fiber(0, &inv(&[1, 2]));
        assert!(plane.iter().any(|x| x.as_slice() == a.as_slice()) && plane.iter().any(|x| x.as_slice() == b.as_slice()), "prefix 1,2 is the plane both cover");
        assert_eq!(c.fiber(0, &inv(&[1, 2, 3])), vec![a.to_vec()], "exact chart is one hypha");
        assert_eq!(c.multiplicity(0, &inv(&[1, 2])), 2, "multiplicity is the vertex count");
        assert!(c.fiber(0, &inv(&[7, 8])).is_empty(), "a first-axis miss is novel");
        let refined = inv(&[1, 2, 3, 7]);
        c.rehome(0, &inv(&[1, 2, 3]), &refined, a);
        assert_eq!(c.fiber(0, &refined), vec![a.to_vec()], "the hypha re-homes under the refined prefix");
        assert_eq!(c.multiplicity(0, &inv(&[1, 2])), 2, "the plane still has both covering vertices");
    }

    #[test]
    fn home_is_identity_order_on_the_fiber_and_room_when_novel() {
        let mut c = Cover::new();
        let mut rooms = Keyed::new();
        let mut addrs = Keyed::new();
        let lo = [1u8; 32];
        let hi = [9u8; 32];
        c.admit(0, &inv(&[4]), lo);
        c.admit(0, &inv(&[4]), hi);
        addrs.insert(lo, "127.0.0.1:7500".into());
        addrs.insert(hi, "127.0.0.1:7501".into());
        rooms.insert(lo, [8, 8, 8, 8]);
        rooms.insert(hi, [8, 8, 8, 8]);
        let (id, addr) = home(&c, &inv(&[4]), &[1, 0, 0, 0], &rooms, &addrs, None);
        assert_eq!(id, lo, "equal rooms: identity is the last tiebreak");
        assert_eq!(addr, "127.0.0.1:7500");
        // Unequal rooms on the occupied lane: more remaining room wins.
        rooms.insert(lo, [2, 8, 8, 8]);
        rooms.insert(hi, [8, 8, 8, 8]);
        let (id, _) = home(&c, &inv(&[4]), &[1, 0, 0, 0], &rooms, &addrs, None);
        assert_eq!(id, hi, "more remaining room on the occupied lane homes there");
        // Incomparable rooms on two occupied lanes: no lane is ranked above
        // another, so identity is the tiebreak (lexicographic room would pick hi).
        rooms.insert(lo, [2, 8, 8, 8]);
        rooms.insert(hi, [8, 2, 8, 8]);
        let (id, _) = home(&c, &inv(&[4]), &[1, 1, 0, 0], &rooms, &addrs, None);
        assert_eq!(id, lo, "incomparable rooms fall to identity, never a lane order");
        rooms.insert(lo, [8, 8, 8, 8]);
        rooms.insert(hi, [8, 8, 8, 8]);
        // Novel chart: both have room; identity still wins.
        let (id, _) = home(&c, &inv(&[99]), &[1, 0, 0, 0], &rooms, &addrs, None);
        assert_eq!(id, lo);
        // Room: hi has room on the residual lane, lo does not.
        rooms.insert(lo, [8, 0, 8, 8]);
        rooms.insert(hi, [0, 3, 0, 0]);
        let (id, _) = home(&c, &inv(&[99]), &[0, 1, 0, 0], &rooms, &addrs, None);
        assert_eq!(id, hi, "novel home is the hypha with room on every moved lane");
        // Both cover the chart; lo has no room on the occupied lane, hi does.
        rooms.insert(lo, [8, 0, 8, 8]);
        rooms.insert(hi, [0, 3, 0, 0]);
        let (id, _) = home(&c, &inv(&[4]), &[0, 1, 0, 0], &rooms, &addrs, None);
        assert_eq!(id, hi, "fiber ∩ room: covering without room loses to covering with room");
    }

    #[test]
    fn retract_hypha_does_not_leave_stale_charts() {
        let mut c = Cover::new();
        let a = [1u8; 32];
        c.admit(0, &inv(&[1]), a);
        c.retract_hypha(&a);
        assert!(c.fiber(0, &inv(&[1])).is_empty());
    }

    /// Three covers over one corpus: each listing homes at one hypha, and
    /// the home counts sum to the corpus.
    #[test]
    fn three_covers_home_counts_sum_to_the_corpus() {
        let mut c = Cover::new();
        let mut rooms = Keyed::new();
        let mut addrs = Keyed::new();
        let ha = [1u8; 32];
        let hb = [2u8; 32];
        let hc = [3u8; 32];
        c.admit(0, &inv(&[10]), ha);
        c.admit(0, &inv(&[20]), hb);
        c.admit(0, &inv(&[30]), hc);
        addrs.insert(ha, "127.0.0.1:7500".into());
        addrs.insert(hb, "127.0.0.1:7501".into());
        addrs.insert(hc, "127.0.0.1:7502".into());
        rooms.insert(ha, [8, 8, 8, 8]);
        rooms.insert(hb, [8, 8, 8, 8]);
        rooms.insert(hc, [8, 8, 8, 8]);
        let corpus: Vec<Invariant> = (0..9).map(|i| inv(&[10 + 10 * (i % 3)])).collect();
        let mut counts = [0usize; 3];
        for chart in &corpus {
            let (id, _) = home(&c, chart, &[1, 0, 0, 0], &rooms, &addrs, None);
            match id {
                x if x == ha => counts[0] += 1,
                x if x == hb => counts[1] += 1,
                x if x == hc => counts[2] += 1,
                _ => panic!("a corpus listing must home on a covering hypha, got {id:?}"),
            }
        }
        assert_eq!(counts[0] + counts[1] + counts[2], corpus.len(), "home counts sum to the corpus");
        assert!(counts.iter().all(|&n| n > 0), "each cover homes some of the corpus");
    }
}

#[derive(Default)]
pub struct Router {
    children: [Option<Box<Router>>; 2],
    leaf: Option<(Vec<u8>, String)>,
}

const XOR_BITS: usize = 256;

fn xor_bit(id: &[u8], i: usize) -> usize {
    let byte = i / 8;
    if byte >= id.len() {
        return 0;
    }
    ((id[byte] >> (7 - (i % 8))) & 1) as usize
}

impl Router {
    pub fn new() -> Self {
        Router::default()
    }

    pub fn insert(&mut self, id: impl AsRef<[u8]>, addr: String) {
        let id = id.as_ref().to_vec();
        let mut node = self;
        for depth in 0..XOR_BITS {
            let b = xor_bit(&id, depth);
            node = node.children[b].get_or_insert_with(|| Box::new(Router::new()));
        }
        node.leaf = Some((id, addr));
    }

    /// The contact XOR-closest to `key` — O(bits), never O(N). A chart
    /// Address walks the same trie as a verifying key; bits past the key
    /// are 0, never a digest of the key.
    pub fn closest(&self, key: &[u8]) -> Option<(Vec<u8>, String)> {
        self.near(key, 0, &[])
    }

    /// The contact XOR-closest to `key` among those not excluded — the next
    /// hop on the nerve when the closest is the speaker itself or the one
    /// it heard from. Deterministic: the same key walks the same path from
    /// every node.
    pub fn closest_excluding(&self, key: &[u8], exclude: &[[u8; 32]]) -> Option<(Vec<u8>, String)> {
        self.near(key, 0, exclude)
    }

    /// Does the router hold this exact identity — a peer it was configured
    /// with, the only kind whose speech the nerve accepts?
    pub fn knows(&self, id: &[u8]) -> bool {
        self.closest(id).is_some_and(|(held, _)| held == id)
    }

    fn near(&self, key: &[u8], depth: usize, exclude: &[[u8; 32]]) -> Option<(Vec<u8>, String)> {
        if depth == XOR_BITS {
            return self.leaf.clone().filter(|(id, _)| {
                exclude.iter().all(|e| e.as_slice() != id.as_slice())
            });
        }
        let b = xor_bit(key, depth);
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
            assert_eq!(r.closest(&key).unwrap().0.as_slice(), brute.as_slice(), "trie descent finds the XOR-closest");
        }
    }

}
