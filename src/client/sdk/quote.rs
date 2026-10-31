// 24, 26 — the quote a client reads: the yield surface and the deferral.

/// The quoted yield: the per-lane surface and the open-demand market's top
/// clusters (address, priced μ⃗, and the chain itself — real, attemptable work).
#[derive(Clone, Debug)]
pub struct YieldQuote {
    /// Book generation at this quote.
    pub height: u64,
    pub surface: [u64; 4],
    /// The judge's k-step at the quote — the temporal coordinate of the
    /// book this quote reads.
    pub k: u64,
    pub clusters: Vec<Cluster>,
    /// MU-CLIENT §4.13 — WHERE THE COURT DEFERRED: its door's per-lane
    /// deficit, the gravity it exports; `None` for a quote it answered.
    pub deficit: Option<[u64; 4]>,
}

#[derive(Clone, Debug)]
pub struct Cluster {
    pub address: crate::transducer::address::Address,
    pub mu: [u64; 4],
    /// The listing WHOLE, as the judge served it (L8q). Served as the
    /// coboundary links alone, a sentence would reach the voice with its
    /// demand vertices stripped and the only move left would be to subtract
    /// links; `Held::of` rebuilds the form and `demand_vertices` names what
    /// it could not spell.
    pub element: Vec<u8>,
    /// Open (non-flat) charts of the listing's coherence field. The mind
    /// reads these against its own atlas; several may close, none is ranked.
    pub open: Vec<crate::transducer::stalk::Invariant>,
}

/// THE DEFERRAL a judge answered with (23.8, S13.4): the door's per-lane
/// reserve deficit, carried on the `WouldBlock` refusal so the arm dwells
/// on the lanes the judge named.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Deferral {
    pub lanes: [u64; 4],
}

impl std::fmt::Display for Deferral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "deferred by the judge: deficit {:?}", self.lanes)
    }
}

impl std::error::Error for Deferral {}
