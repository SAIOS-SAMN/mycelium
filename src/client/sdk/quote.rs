// 24, 26 — the quote a client reads: the yield surface and the deferral.

/// The quoted yield: the per-lane surface and the open-demand market's top
/// clusters (address, priced μ⃗, and the chain itself — real, attemptable work).
/// Each `[u64; 4]` here is a reading per base lane S, G, T, H, in that
/// order, never a holding: a holding is a coordinate of the
/// multi-dimensional ledger, read and never stored (MU-ECONOMY §2.2).
#[derive(Clone, Debug)]
pub struct YieldQuote {
    /// Book generation at this quote.
    pub height: u64,
    /// The yield surface read per lane S, G, T, H.
    pub surface: [u64; 4],
    /// The judge's k-step at the quote — the temporal coordinate of the
    /// book this quote reads.
    pub k: u64,
    pub clusters: Vec<Cluster>,
    /// MU-CLIENT §4.13 — WHERE THE COURT DEFERRED: its door's deficit read
    /// per lane S, G, T, H, the gravity it exports; `None` for a quote it
    /// answered.
    pub deficit: Option<[u64; 4]>,
}

#[derive(Clone, Debug)]
pub struct Cluster {
    pub address: crate::transducer::address::Address,
    /// The price read per lane S, G, T, H.
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

/// THE DEFERRAL a judge answered with (23.8, S13.4): the door's reserve
/// deficit read per lane, carried on the `WouldBlock` refusal so the arm
/// dwells on the lanes the judge named.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Deferral {
    /// The door's reserve deficit read on each lane S, G, T, H, in that
    /// order, as it stated it: a reading per lane, never a holding.
    pub deficit: [u64; 4],
}

impl std::fmt::Display for Deferral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [s, g, t, h] = self.deficit;
        write!(f, "deferred by the judge: deficit S {s}, G {g}, T {t}, H {h}")
    }
}

impl std::error::Error for Deferral {}

/// THE DEFERRAL READ OFF A REPLY'S BYTES: the pending code, no height, the
/// deficit read on each of the four base lanes, nothing taught, nothing after — or `None`
/// where the reply is any other answer.
pub fn read_deferral(reply: &[u8]) -> Option<Deferral> {
    crate::anastome::dialogue::deferral_lanes(reply, crate::anastome::dialogue::PENDING_CODE).map(|deficit| Deferral { deficit })
}

/// THE DEFERRAL A SESSION'S REFUSAL CARRIES: `Session::speak` answers a
/// deferral as a `WouldBlock` refusal holding it; any other refusal is
/// `None`.
pub fn deferral_of(refusal: &std::io::Error) -> Option<Deferral> {
    match refusal.kind() {
        std::io::ErrorKind::WouldBlock => refusal.get_ref().and_then(|inner| inner.downcast_ref::<Deferral>()).copied(),
        _ => None,
    }
}
