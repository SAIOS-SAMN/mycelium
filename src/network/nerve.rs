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
/// A KEYED RELATION over 32-byte identities: a value reached by its key's
/// bits, O(bits) per read and write, never a scan of the relation. The
/// carrier every address-keyed fact in the engine is reached through —
/// listings spoken on, deflations, roots, a language's words — so an
/// endpoint is always read from the key, whatever the relation's size.
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
    Leaf([u8; 32], V),
    /// Both children are always bound; the option is the slot a split or
    /// a collapse works through.
    Branch { bit: usize, children: [Option<Box<Node<V>>>; 2] },
}

impl<V> Default for Keyed<V> {
    fn default() -> Self {
        Keyed { root: None, n: 0 }
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
    pub fn insert(&mut self, key: [u8; 32], value: V) -> Option<V> {
        #[cfg(test)]
        work::count_insert();
        let Some(root) = self.root.as_deref() else {
            self.root = Some(Box::new(Node::Leaf(key, value)));
            self.n = 1;
            return None;
        };
        let held = root.held_key(&key);
        match first_difference(&key, &held) {
            None => self.root.as_deref_mut()?.get_mut(&key).map(|v| std::mem::replace(v, value)),
            Some(d) => {
                insert_at(&mut self.root, key, value, d);
                self.n += 1;
                None
            }
        }
    }

    pub fn get(&self, key: &[u8; 32]) -> Option<&V> {
        #[cfg(test)]
        work::count_get();
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

    /// Subtree under `prefix`. The empty prefix [`ROOT`] is this carrier's
    /// own fiber — valid when the Keyed is already scoped (one bubble's
    /// charts, one vertex's star, one chart's incidence).
    pub fn fiber(&self, prefix: &[u8; 32]) -> Fiber<'_, V> {
        let mut stack = Vec::new();
        if let Some(root) = self.root.as_deref() {
            stack.push(root);
        }
        Fiber { stack, prefix: *prefix }
    }

    /// Parent shell of `key`: the branch that split this leaf. None when
    /// the key is unbound or is the sole leaf.
    pub fn peel(&self, key: &[u8; 32]) -> Option<Shell<'_, V>> {
        let mut node = self.root.as_deref()?;
        let mut parent: Option<&Node<V>> = None;
        let mut parent_bit = 0usize;
        loop {
            match node {
                Node::Leaf(k, _) => {
                    if k != key {
                        return None;
                    }
                    let p = parent?;
                    return Some(Shell {
                        fiber: Fiber { stack: vec![p], prefix: ROOT },
                        prefix: clear_after_bit(key, parent_bit),
                    });
                }
                Node::Branch { bit, children } => {
                    parent = Some(node);
                    parent_bit = *bit;
                    node = children[key_bit(key, *bit)].as_deref()?;
                }
            }
        }
    }

    /// At most `budget` entries in key order after `from`. [`ROOT`] starts
    /// at the first key. The last yielded key is the resume cursor.
    pub fn cursor<'a>(&'a self, from: [u8; 32], budget: &'a mut crate::physics::Budget) -> Cursor<'a, V> {
        Cursor {
            fiber: self.fiber(&ROOT),
            budget,
            from,
            last: if from == ROOT { None } else { Some(from) },
        }
    }
}

/// The empty prefix: the root fiber of a keyed that is already a fiber.
pub const ROOT: [u8; 32] = [0u8; 32];

fn prefix_len(prefix: &[u8; 32]) -> usize {
    prefix.iter().rposition(|&b| b != 0).map(|i| i + 1).unwrap_or(0)
}

fn key_in_fiber(key: &[u8; 32], prefix: &[u8; 32]) -> bool {
    let n = prefix_len(prefix);
    n == 0 || key[..n] == prefix[..n]
}

fn clear_after_bit(key: &[u8; 32], bit: usize) -> [u8; 32] {
    let mut p = *key;
    let start = bit.saturating_add(1);
    let byte = start / 8;
    let off = start % 8;
    if byte >= 32 {
        return p;
    }
    if off == 0 {
        p[byte..].fill(0);
    } else {
        p[byte] &= 0xffu8 << (8 - off);
        if byte + 1 < 32 {
            p[byte + 1..].fill(0);
        }
    }
    p
}

/// Subtree under a key prefix. The only read of more than one entry that
/// is not a link, a peel, or a budgeted cursor.
pub struct Fiber<'a, V> {
    stack: Vec<&'a Node<V>>,
    prefix: [u8; 32],
}

impl<'a, V> Fiber<'a, V> {
    fn empty() -> Self {
        Fiber { stack: Vec::new(), prefix: ROOT }
    }
}

impl<'a, V> Iterator for Fiber<'a, V> {
    type Item = (&'a [u8; 32], &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            match node {
                Node::Leaf(k, v) => {
                    if key_in_fiber(k, &self.prefix) {
                        return Some((k, v));
                    }
                }
                Node::Branch { children, .. } => {
                    // index: the two children of a PATRICIA branch (fixed arity)
                    if let Some(c) = children[1].as_deref() {
                        self.stack.push(c);
                    }
                    if let Some(c) = children[0].as_deref() {
                        self.stack.push(c);
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
    type Item = (&'a [u8; 32], &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        self.fiber.next()
    }
}

impl<T> Keyed<Keyed<T>> {
    /// The inner trie at `key` — cells on a vertex, bubbles on a chart.
    pub fn link(&self, key: &[u8; 32]) -> Link<'_, T> {
        Link {
            fiber: match self.get(key) {
                Some(inner) => inner.fiber(&ROOT),
                None => Fiber::empty(),
            },
        }
    }
}

/// Budgeted walk in key order from `from`, consuming a [`crate::physics::Budget`].
pub struct Cursor<'a, V> {
    fiber: Fiber<'a, V>,
    budget: &'a mut crate::physics::Budget,
    from: [u8; 32],
    last: Option<[u8; 32]>,
}

impl<'a, V> Cursor<'a, V> {
    /// Last key yielded, so the next stroke resumes after it.
    pub fn last_key(&self) -> Option<[u8; 32]> {
        self.last
    }
}

impl<'a, V> Iterator for Cursor<'a, V> {
    type Item = (&'a [u8; 32], &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.budget.remaining() == 0 {
                return None;
            }
            let (k, v) = self.fiber.next()?;
            if self.from != ROOT && k <= &self.from {
                continue;
            }
            if !self.budget.spend() {
                return None;
            }
            self.last = Some(*k);
            return Some((k, v));
        }
    }
}

/// One shell: the parent prefix of a peel, and the entries at that branch.
pub struct Shell<'a, V> {
    fiber: Fiber<'a, V>,
    prefix: [u8; 32],
}

impl<'a, V> Shell<'a, V> {
    pub fn key(&self) -> [u8; 32] {
        self.prefix
    }
}

impl<'a, V> Iterator for Shell<'a, V> {
    type Item = (&'a [u8; 32], &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        self.fiber.next()
    }
}

/// Sealed persist walk: only [`crate::snapshot`] names this.
pub(crate) fn keyed_persist_walk<V>(k: &Keyed<V>, f: &mut impl FnMut(&[u8; 32], &V)) {
    for (key, v) in k.fiber(&ROOT) {
        f(key, v);
    }
}

#[cfg(test)]
mod keyed_tests {
    use super::{Keyed, ROOT};

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
        assert_eq!(k.len(), 3);
        assert_eq!(k.fiber(&ROOT).count(), 3);
        assert_eq!(k.remove(&b), Some(20));
        assert_eq!(k.get(&b), None);
        assert_eq!(k.get(&a), Some(&1));
        assert_eq!(k.get(&c), Some(&3));
        assert_eq!(k.remove(&a), Some(1));
        assert_eq!(k.remove(&a), None);
        assert_eq!(k.len(), 1);
        assert_eq!(k.fiber(&ROOT).count(), 1);
        assert_eq!(k.get(&c), Some(&3));
    }
}

/// An invariant axis as a Keyed key: the coordinate little-endian in the
/// first eight bytes, so a prefix of axes is a prefix of the plane.
pub fn axis_key_le(a: u64) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[..8].copy_from_slice(&a.to_le_bytes());
    k
}

fn axis_of_le(key: &[u8; 32]) -> u64 {
    u64::from_le_bytes(key[..8].try_into().unwrap())
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

    fn admit(&mut self, axes: &[u64], hypha: [u8; 32]) {
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
        for (id, _) in self.here.fiber(&ROOT) {
            out.insert(*id, ());
        }
        for (_, child) in self.next.fiber(&ROOT) {
            child.collect_into(out);
        }
    }

    fn collect(&self) -> Vec<[u8; 32]> {
        let mut out = Keyed::new();
        self.collect_into(&mut out);
        out.fiber(&ROOT).map(|(k, _)| *k).collect()
    }

    /// Longest common prefix: descend while axes match. A miss after some
    /// match returns the fiber at that plane. A miss at the first axis is
    /// novel — empty. The empty chart (no axes) is the root fiber.
    fn lcp(&self, axes: &[u64], matched: bool) -> Vec<[u8; 32]> {
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
    fn retract_path(&mut self, axes: &[u64], hypha: &[u8; 32]) {
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
        for (k, ch) in self.next.fiber(&ROOT) {
            path.push(axis_of_le(k));
            ch.hash_charts(hasher, path);
            path.pop();
        }
    }

    fn walk(&self, path: &mut Vec<u64>, out: &mut Vec<crate::transducer::stalk::Invariant>) {
        if !self.here.is_empty() {
            out.push(crate::transducer::stalk::Invariant(path.clone()));
        }
        for (k, ch) in self.next.fiber(&ROOT) {
            path.push(axis_of_le(k));
            ch.walk(path, out);
            path.pop();
        }
    }
}

impl Cover {
    pub fn new() -> Self {
        Cover::default()
    }

    pub fn admit(&mut self, lane: u8, inv: &crate::transducer::stalk::Invariant, hypha: [u8; 32]) {
        let lane = lane & 3;
        self.lanes[lane as usize].admit(&inv.0, hypha);
        match self.held.get_mut(&hypha) {
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
    pub fn retract_hypha(&mut self, hypha: &[u8; 32]) {
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
    pub fn fiber(&self, lane: u8, inv: &crate::transducer::stalk::Invariant) -> Vec<[u8; 32]> {
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
        hypha: &[u8; 32],
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
        hypha: [u8; 32],
    ) {
        let lane = lane & 3;
        self.lanes[lane as usize].retract_path(&old.0, &hypha);
        self.lanes[lane as usize].admit(&new.0, hypha);
        if let Some(charts) = self.held.get_mut(&hypha) {
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
    for (id, _) in fiber.fiber(&ROOT) {
        if let Some(room) = room_of(id) {
            if fits(&room) {
                if let Some(a) = addr_of(id, addrs, self_id) {
                    consider(&mut best, *id, a, room);
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
    for (id, room) in rooms.fiber(&ROOT) {
        if fits(room) {
            if let Some(a) = addrs.get(id) {
                consider(&mut best, *id, a.clone(), *room);
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
        assert!(plane.contains(&a) && plane.contains(&b), "prefix 1,2 is the plane both cover");
        assert_eq!(c.fiber(0, &inv(&[1, 2, 3])), vec![a], "exact chart is one hypha");
        assert_eq!(c.multiplicity(0, &inv(&[1, 2])), 2, "multiplicity is the vertex count");
        assert!(c.fiber(0, &inv(&[7, 8])).is_empty(), "a first-axis miss is novel");
        let refined = inv(&[1, 2, 3, 7]);
        c.rehome(0, &inv(&[1, 2, 3]), &refined, a);
        assert_eq!(c.fiber(0, &refined), vec![a], "the hypha re-homes under the refined prefix");
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
