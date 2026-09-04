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
#[derive(Clone, Debug, Default)]
pub struct Cover {
    lanes: [CoverNode; 4],
}

#[derive(Clone, Debug, Default)]
struct CoverNode {
    here: Vec<[u8; 32]>,
    next: Keyed<CoverNode>,
}

impl CoverNode {
    fn is_empty(&self) -> bool {
        self.here.is_empty() && self.next.entries().is_empty()
    }

    fn admit(&mut self, axes: &[u64], hypha: [u8; 32]) {
        match axes.split_first() {
            None => {
                if !self.here.contains(&hypha) {
                    self.here.push(hypha);
                }
            }
            Some((&a, rest)) => {
                let k = axis_key_le(a);
                if self.next.get(&k).is_none() {
                    self.next.insert(k, CoverNode::default());
                }
                self.next.get_mut(&k).unwrap().admit(rest, hypha);
            }
        }
    }

    fn collect(&self) -> Vec<[u8; 32]> {
        let mut out = self.here.clone();
        for (_, child) in self.next.entries() {
            for id in child.collect() {
                if !out.contains(&id) {
                    out.push(id);
                }
            }
        }
        out
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

    fn retract(&mut self, hypha: &[u8; 32]) {
        self.here.retain(|h| h != hypha);
        let keys: Vec<[u8; 32]> = self.next.entries().into_iter().map(|(k, _)| *k).collect();
        for k in keys {
            let empty = {
                let Some(ch) = self.next.get_mut(&k) else { continue };
                ch.retract(hypha);
                ch.is_empty()
            };
            if empty {
                self.next.remove(&k);
            }
        }
    }

    fn walk(&self, path: &mut Vec<u64>, out: &mut Vec<crate::transducer::stalk::Invariant>) {
        if !self.here.is_empty() {
            out.push(crate::transducer::stalk::Invariant(path.clone()));
        }
        for (k, ch) in self.next.entries() {
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
        self.lanes[(lane as usize) & 3].admit(&inv.0, hypha);
    }

    /// Drop every chart of this hypha. Used to replace a peer's cover from
    /// a fresh digest rather than union forever (the leak of last deploy).
    pub fn retract_hypha(&mut self, hypha: &[u8; 32]) {
        for lane in &mut self.lanes {
            lane.retract(hypha);
        }
    }

    /// Fiber at the deepest matching prefix on `lane`. Empty at every
    /// prefix means the chart is novel to the mesh.
    pub fn fiber(&self, lane: u8, inv: &crate::transducer::stalk::Invariant) -> Vec<[u8; 32]> {
        self.lanes[(lane as usize) & 3].lcp(&inv.0, false)
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
        let mut h = blake3::Hasher::new();
        for inv in self.charts(lane) {
            for a in &inv.0 {
                h.update(&a.to_le_bytes());
            }
            h.update(&[0xff]);
        }
        *h.finalize().as_bytes()
    }

    pub fn roots(&self) -> [[u8; 32]; 4] {
        std::array::from_fn(|l| self.root(l as u8))
    }
}

/// The lane a word moves on: the first occupied lane, else 0.
pub fn occupied_lane(mag: &[u64; 4]) -> u8 {
    mag.iter().position(|&x| x > 0).unwrap_or(0) as u8
}

/// Home by incidence. Fiber at the deepest prefix of the word's lane; if
/// that fiber is empty at every prefix the chart is novel and the home is
/// the hypha with room on every lane the word moves. Identity order
/// (`[u8;32]` min) breaks a remaining tie and nothing else does.
/// `self_id` is this hypha's verifying key when the caller is a hypha;
/// a client passes `None` and never homes to itself.
pub fn home(
    cover: &Cover,
    inv: &crate::transducer::stalk::Invariant,
    mag: &[u64; 4],
    rooms: &Keyed<[u64; 4]>,
    addrs: &Keyed<String>,
    self_id: Option<(&[u8; 32], &[u64; 4], &str)>,
) -> ([u8; 32], String) {
    let lane = occupied_lane(mag);
    let fiber = cover.fiber(lane, inv);
    let mut pick: Option<[u8; 32]> = None;
    for id in &fiber {
        if let Some((sid, _, _)) = self_id {
            if id == sid {
                pick = match pick {
                    Some(p) if p < *id => Some(p),
                    _ => Some(*id),
                };
                continue;
            }
        }
        if addrs.get(id).is_some() {
            pick = match pick {
                Some(p) if p < *id => Some(p),
                _ => Some(*id),
            };
        }
    }
    if let Some(id) = pick {
        if let Some((sid, _, saddr)) = self_id {
            if id == *sid {
                return (id, saddr.to_string());
            }
        }
        if let Some(a) = addrs.get(&id) {
            return (id, a.clone());
        }
    }
    // Novel chart: room on every occupied lane, then identity order.
    let fits = |room: &[u64; 4]| (0..4).all(|l| mag[l] == 0 || room[l] > 0);
    let mut best: Option<([u8; 32], String)> = None;
    let consider = |best: &mut Option<([u8; 32], String)>, id: [u8; 32], addr: String| {
        match best {
            Some((b, _)) if *b < id => {}
            _ => *best = Some((id, addr)),
        }
    };
    if let Some((sid, sroom, saddr)) = self_id {
        if fits(sroom) {
            consider(&mut best, *sid, saddr.to_string());
        }
    }
    for (id, room) in rooms.entries() {
        if fits(room) {
            if let Some(a) = addrs.get(id) {
                consider(&mut best, *id, a.clone());
            }
        }
    }
    best.unwrap_or_else(|| {
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
        assert!(c.fiber(0, &inv(&[7, 8])).is_empty(), "a first-axis miss is novel");
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
        assert_eq!(id, lo, "identity order, not XOR");
        assert_eq!(addr, "127.0.0.1:7500");
        // Novel chart: both have room; identity still wins.
        let (id, _) = home(&c, &inv(&[99]), &[1, 0, 0, 0], &rooms, &addrs, None);
        assert_eq!(id, lo);
        // Room: hi has room on the residual lane, lo does not.
        rooms.insert(lo, [8, 0, 8, 8]);
        rooms.insert(hi, [0, 3, 0, 0]);
        let (id, _) = home(&c, &inv(&[99]), &[0, 1, 0, 0], &rooms, &addrs, None);
        assert_eq!(id, hi, "novel home is the hypha with room on every moved lane");
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
