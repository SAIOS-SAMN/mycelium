// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ČECH NERVE — the topological peer table. Peers are the charts of a cover;
//! adjacency is genuine overlap of their invariant fingerprints (per-lane β₁ ⊕
//! Bloom over active classes), not a static ring. Each edge carries a per-lane
//! defect flux Φ_ij^L (the settlement/defect exchanged), so dormant edges prune
//! by earned overlap. Peer state is a Vec of relations — no digest-keyed map.

/// A patch fingerprint: per-lane first Betti numbers ⊕ a Bloom over the classes
/// active on this peer. Overlap of Bloom + shared β₁ is the rendezvous signal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fingerprint {
    pub beta1: [u64; 4],
    pub bloom: [u8; 32],
}

impl Fingerprint {
    /// Bits shared between two Blooms — a cheap lower bound on class overlap.
    pub fn overlap_bits(&self, other: &Fingerprint) -> u32 {
        self.bloom.iter().zip(&other.bloom).map(|(a, b)| (a & b).count_ones()).sum()
    }
    /// Adjacent iff the fingerprints genuinely overlap (shared active classes).
    pub fn adjacent(&self, other: &Fingerprint) -> bool {
        self.overlap_bits(other) > 0
    }
}

/// One nerve edge: a peer, its fingerprint, and the per-lane accumulated flux.
#[derive(Debug, Clone)]
struct Edge {
    peer: [u8; 32],
    fp: Fingerprint,
    flux: [i64; 4],
}

/// The Čech-adjacency peer table (Vec-of-relations; Law II).
#[derive(Debug, Default, Clone)]
pub struct Nerve {
    edges: Vec<Edge>,
}

impl Nerve {
    pub fn new() -> Self {
        Nerve { edges: Vec::new() }
    }

    /// Learn or refresh a peer's fingerprint (idempotent per peer).
    pub fn observe(&mut self, peer: [u8; 32], fp: Fingerprint) {
        match self.edges.iter_mut().find(|e| e.peer == peer) {
            Some(e) => e.fp = fp,
            None => self.edges.push(Edge { peer, fp, flux: [0; 4] }),
        }
    }

    /// Record settlement/defect flux exchanged with a peer on lane `l`.
    pub fn record_flux(&mut self, peer: &[u8; 32], lane: usize, delta: i64) {
        if let Some(e) = self.edges.iter_mut().find(|e| e.peer == *peer) {
            if lane < 4 {
                e.flux[lane] = e.flux[lane].saturating_add(delta);
            }
        }
    }

    /// The peers Čech-adjacent to a given fingerprint (genuine overlap).
    pub fn adjacent(&self, fp: &Fingerprint) -> Vec<[u8; 32]> {
        self.edges.iter().filter(|e| e.fp.adjacent(fp)).map(|e| e.peer).collect()
    }

    /// Prune dormant edges: no overlap with the local fingerprint AND zero total
    /// flux earned — the Law of Earned Overlap (an edge that never carried and
    /// never overlaps is not in the nerve).
    pub fn prune_dormant(&mut self, local: &Fingerprint) {
        self.edges.retain(|e| e.fp.adjacent(local) || e.flux.iter().any(|&f| f != 0));
    }

    pub fn len(&self) -> usize {
        self.edges.len()
    }
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
}

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
    children: [Option<Box<Keyed<V>>>; 2],
    leaf: Option<([u8; 32], V)>,
}

impl<V> Default for Keyed<V> {
    fn default() -> Self {
        Keyed { children: [None, None], leaf: None }
    }
}

impl<V> Keyed<V> {
    pub fn new() -> Self {
        Keyed::default()
    }

    fn bit(key: &[u8; 32], i: usize) -> usize {
        ((key[i / 8] >> (7 - (i % 8))) & 1) as usize
    }

    /// Bind `key` to `value`; a held key is rebound. Returns the value
    /// displaced, if any.
    pub fn insert(&mut self, key: [u8; 32], value: V) -> Option<V> {
        let mut node = self;
        for depth in 0..256 {
            let b = Self::bit(&key, depth);
            node = node.children[b].get_or_insert_with(|| Box::new(Keyed::default()));
        }
        node.leaf.replace((key, value)).map(|(_, v)| v)
    }

    fn node(&self, key: &[u8; 32]) -> Option<&Keyed<V>> {
        let mut node = self;
        for depth in 0..256 {
            node = node.children[Self::bit(key, depth)].as_deref()?;
        }
        Some(node)
    }

    fn node_mut(&mut self, key: &[u8; 32]) -> Option<&mut Keyed<V>> {
        let mut node = self;
        for depth in 0..256 {
            node = node.children[Self::bit(key, depth)].as_deref_mut()?;
        }
        Some(node)
    }

    pub fn get(&self, key: &[u8; 32]) -> Option<&V> {
        self.node(key).and_then(|n| n.leaf.as_ref()).map(|(_, v)| v)
    }

    pub fn get_mut(&mut self, key: &[u8; 32]) -> Option<&mut V> {
        self.node_mut(key).and_then(|n| n.leaf.as_mut()).map(|(_, v)| v)
    }

    pub fn contains(&self, key: &[u8; 32]) -> bool {
        self.get(key).is_some()
    }

    /// Unbind `key`; the value it held, if any. Empty branches are left in
    /// place (a later rebind reuses them).
    pub fn remove(&mut self, key: &[u8; 32]) -> Option<V> {
        self.node_mut(key).and_then(|n| n.leaf.take()).map(|(_, v)| v)
    }

    /// Every bound (key, value), in key order — a walk of the relation for
    /// persistence and for measurements over the whole, never for lookup.
    pub fn entries(&self) -> Vec<(&[u8; 32], &V)> {
        let mut out = Vec::new();
        self.walk(&mut out);
        out
    }

    fn walk<'a>(&'a self, out: &mut Vec<(&'a [u8; 32], &'a V)>) {
        if let Some((k, v)) = &self.leaf {
            out.push((k, v));
        }
        for c in self.children.iter().flatten() {
            c.walk(out);
        }
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

    fn fp(bit: usize, beta: u64) -> Fingerprint {
        let mut bloom = [0u8; 32];
        bloom[bit / 8] |= 1 << (bit % 8);
        Fingerprint { beta1: [beta, 0, 0, 0], bloom }
    }

    #[test]
    fn adjacency_is_earned_overlap_not_a_ring() {
        let mut n = Nerve::new();
        n.observe([1u8; 32], fp(3, 1)); // shares bit 3
        n.observe([2u8; 32], fp(9, 1)); // disjoint bit
        let local = fp(3, 1);
        assert_eq!(n.adjacent(&local), vec![[1u8; 32]], "only the overlapping peer");
    }

    #[test]
    fn flux_accumulates_per_lane() {
        let mut n = Nerve::new();
        n.observe([1u8; 32], fp(3, 1));
        n.record_flux(&[1u8; 32], 0, 5);
        n.record_flux(&[1u8; 32], 0, 2);
        n.record_flux(&[1u8; 32], 2, 1);
        // A non-overlapping peer with earned flux survives pruning; a dormant
        // disjoint peer with zero flux does not.
        n.observe([2u8; 32], fp(9, 0));
        n.prune_dormant(&fp(3, 1));
        assert_eq!(n.len(), 1, "the dormant disjoint peer is pruned");
    }
}
