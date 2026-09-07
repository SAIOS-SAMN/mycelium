// A13, A21 — THE KEYED RELATION: a radix trie over variable-length A21
// keys, and the walks that read it.
//
//! Every index in this tree is one of these. A key is the object's own
//! coordinates written out, so a fiber, a link, a cursor and a shell are
//! reads of structure rather than searches over a list. `meet_nodes` is the
//! intersection of two relations, computed by descending both at once.


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
