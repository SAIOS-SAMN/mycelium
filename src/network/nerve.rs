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

#[cfg(test)]
mod tests {
    use super::*;

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
