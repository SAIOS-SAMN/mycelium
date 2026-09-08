// Law 5, 12.6 — THE FOUR LANES: what a word measures, how one measurement
// reads against another, and what the surface prices it at.
//
//! Lanes are never summed and never ordered by a scalar. `lane_moves` is
//! the READING — what each of the four lanes did, and nothing else. There
//! is no dominance here and nothing that picks: the consolidation of
//! 2026-09-08 (12.6, 16.16 step 5, T13, T22) holds every certified
//! discharge as a vortex and lets no reading of μ⃗ choose among them.
//! `price` is per lane.

use super::*;

/// A germ's polytonic magnitude — per-lane support size (‖Δ‖'s |supp| component),
/// never summed, computed EXACTLY from the germ's description. 𝒮 = motion support
/// (shift/rotation/scale), 𝒢 = relabeled-hue count, 𝒯 = temporal extent,
/// 𝒽 = holonomic volume. A settled demand's residual magnitude is the zero vector; a real
/// germ's is nonzero — hence the descent.
pub fn magnitude(g: &Element) -> [u64; 4] {
    g.magnitude()
}

/// WHAT ONE LANE DID between two measurements. Three states, and the
/// amount travels inside the two that have one: a lane that fell by three
/// and a lane that fell by one are not the same reading, and neither is a
/// lane that held.
///
/// This is a READING and never a verdict (law 5, 12.6). Nothing in the
/// engine certifies by a lane falling and nothing selects among certified
/// discharges by any function of these four values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaneMove {
    /// The lane fell by this much: `before − after`.
    Fell(u64),
    /// The lane did not move.
    Held,
    /// The lane rose by this much: `after − before`.
    Rose(u64),
}

impl LaneMove {
    /// What this lane contributes to a deflation: what it fell by, and
    /// nothing at all when it held or rose.
    pub fn deflation(&self) -> u64 {
        match self {
            LaneMove::Fell(d) => *d,
            LaneMove::Held | LaneMove::Rose(_) => 0,
        }
    }
}

/// THE READING of two lane vectors: what each of the four lanes did, in
/// lane order. Never summed, never ordered against each other, and never
/// reduced to an answer — the caller reads the lanes it needs.
pub fn lane_moves(after: &[u64; 4], before: &[u64; 4]) -> [LaneMove; 4] {
    std::array::from_fn(|l| match after[l].cmp(&before[l]) {
        std::cmp::Ordering::Less => LaneMove::Fell(before[l] - after[l]),
        std::cmp::Ordering::Equal => LaneMove::Held,
        std::cmp::Ordering::Greater => LaneMove::Rose(after[l] - before[l]),
    })
}

/// The lanes that ROSE in a reading, by index. Not a verdict: the sites
/// 12.6 sanctions refuse on this and name the lane, and everyone else
/// reports it.
pub fn lanes_risen(reading: &[LaneMove; 4]) -> Vec<usize> {
    (0..4).filter(|&l| matches!(reading[l], LaneMove::Rose(_))).collect()
}

/// The lanes that FELL, by index.
pub fn lanes_fallen(reading: &[LaneMove; 4]) -> Vec<usize> {
    (0..4).filter(|&l| matches!(reading[l], LaneMove::Fell(_))).collect()
}

/// The deflation a reading measured, per lane.
pub fn deflation_of(reading: &[LaneMove; 4]) -> [u64; 4] {
    std::array::from_fn(|l| reading[l].deflation())
}

/// A refinement is measured as deflation on the split strata. No new price.
pub fn split_deflation(before: &[u64; 4], after: &[u64; 4]) -> Option<[u64; 4]> {
    let reading = lane_moves(after, before);
    // A lane that rose is a trade, not a split; a reading where every lane
    // held is not a refinement at all. Both are refused, and the economy is
    // one of the sites 12.6 sanctions to take that refusal.
    if !lanes_risen(&reading).is_empty() || lanes_fallen(&reading).is_empty() {
        return None;
    }
    Some(deflation_of(&reading))
}

/// THE YIELD SURFACE — the treasury's dynamic per-lane quote, read from GENUINE
/// metabolic tension: open pressure (unmet demand backlog) + reserve deficit
/// (the depletion premium) + entropic excess (torsion outrunning syntropy).
/// Strained lanes quote high; settled lanes quote low — never a static faucet,
/// and never a summed scalar (polytonic, #34).
pub fn yield_surface(p: &crate::physics::Physics) -> [u64; 4] {
    let flux = p.order_flux();
    let deficit = p.reserve_deficit();
    std::array::from_fn(|l| {
        let entropic = (-flux[l]).max(0) as u64;
        p.pressure[l] + deficit[l] + entropic
    })
}

/// PRICE IS THE MAGNITUDE ON THE LANE: a verified word is worth what it
/// deflated. The surface paces pressure, it does not scale the mint.
pub fn price(mag: &[u64; 4], _surface: &[u64; 4]) -> [u64; 4] {
    *mag
}

/// THE FLOW ORIENTATION of a ledger entry relative to its identity — a signed
/// coefficient in the flow algebra (the chain-complex orientation), never a
/// boolean gate: +1 credits the identity, −1 debits it, 0 orients nothing
/// (markers whose μ⃗ moved the stake bond, not a balance). An exhaustive total
/// function — an unknown kind orients nothing rather than silently crediting.
/// Transfer credits are the second of a paired pair (same μ⃗, K, receipt);
/// `recompute_balances` applies those, not this sign.
pub fn orientation(kind: &str) -> i8 {
    match kind {
        "mint" | "genesis" => 1,
        "transfer" | "slash" | "burn" => -1,
        _ => 0,
    }
}
