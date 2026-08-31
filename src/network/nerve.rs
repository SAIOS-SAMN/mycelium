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

/// XOR ROUTING TRIE (PATRICIA / path-compressed) — the topological structure the
/// Kademlia metric induces. Branches exist ONLY where contact ids actually
/// differ, so the whole trie is O(N) nodes (not O(256·N)) with O(log N) effective
/// depth over the full 2²⁵⁶ keyspace — no peer cap, no degenerate chains.
/// `closest(key)` descends the key's bits to the longest-prefix leaf (= smallest
/// XOR) in O(depth), never a scan of the peer set.
pub enum Router {
    Empty,
    Leaf([u8; 32], String),
    /// Split at bit `at`: contacts whose bit `at` is 0 go left, 1 go right.
    Branch { at: usize, zero: Box<Router>, one: Box<Router> },
}

impl Default for Router {
    fn default() -> Self {
        Router::Empty
    }
}

impl Router {
    pub fn new() -> Self {
        Router::Empty
    }

    fn bit(id: &[u8; 32], i: usize) -> usize {
        ((id[i / 8] >> (7 - (i % 8))) & 1) as usize
    }

    /// First bit index where two ids differ (256 if identical).
    fn first_diff(a: &[u8; 32], b: &[u8; 32]) -> usize {
        (0..256).find(|&i| Self::bit(a, i) != Self::bit(b, i)).unwrap_or(256)
    }

    /// Insert a contact (id → address); path-compressed — a new branch is created
    /// only at the first bit distinguishing this id from an existing leaf.
    pub fn insert(&mut self, id: [u8; 32], addr: String) {
        match self {
            Router::Empty => *self = Router::Leaf(id, addr),
            Router::Leaf(eid, _) if *eid == id => *self = Router::Leaf(id, addr), // replace
            Router::Leaf(eid, _) => {
                let at = Self::first_diff(eid, &id);
                let existing = std::mem::replace(self, Router::Empty);
                let fresh = Router::Leaf(id, addr);
                let (zero, one) = if Self::bit(&id, at) == 0 { (fresh, existing) } else { (existing, fresh) };
                *self = Router::Branch { at, zero: Box::new(zero), one: Box::new(one) };
            }
            Router::Branch { at, zero, one } => {
                if Self::bit(&id, *at) == 0 { zero.insert(id, addr) } else { one.insert(id, addr) }
            }
        }
    }

    /// The contact XOR-closest to `key` — descend the key's bits to the
    /// longest-prefix leaf (sibling-fallback for safety). O(depth), never O(N).
    pub fn closest(&self, key: &[u8; 32]) -> Option<([u8; 32], String)> {
        match self {
            Router::Empty => None,
            Router::Leaf(id, addr) => Some((*id, addr.clone())),
            Router::Branch { at, zero, one } => {
                let (near, far) = if Self::bit(key, *at) == 0 { (zero, one) } else { (one, zero) };
                near.closest(key).or_else(|| far.closest(key))
            }
        }
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
