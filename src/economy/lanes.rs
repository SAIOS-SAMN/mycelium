// Law 5, 12.6 — THE FOUR LANES: what a word measures, how one measurement
// reads against another, and what the surface prices it at.
//
//! Lanes are never summed and never ordered by a scalar. `lane_moves` is
//! the READING — what each of the four lanes did, and nothing else: no
//! dominance, nothing that picks among discharges, and `price` per lane.

use super::*;

/// A germ's polytonic magnitude — per-lane support size (‖Δ‖'s |supp| component),
/// never summed, computed EXACTLY from the germ's description. 𝒮 = motion support
/// (shift/rotation/scale), 𝒢 = relabeled-hue count, 𝒯 = temporal extent,
/// 𝒽 = holonomic volume. A settled demand's residual magnitude is the zero
/// vector and a real germ's is nonzero; the reading between two
/// measurements is `lane_moves`, per lane, never a descent (12.6).
pub fn magnitude(g: &Element) -> [u64; 4] {
    g.magnitude()
}

/// A lane vector (Law 5, §2.3, §4.4, X5).
/// μ⃗ is a type implementing neither `PartialEq` nor `PartialOrd`.
/// The one comparison is `lane_moves(after, before) -> [LaneMove; 4]`,
/// and arithmetic is lane-wise `add` and `shortfall`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Mu(pub [u64; 4]);

impl Mu {
    pub const ZERO: Self = Mu([0, 0, 0, 0]);

    pub fn new(lanes: [u64; 4]) -> Self {
        Mu(lanes)
    }

    pub fn lanes(&self) -> [u64; 4] {
        self.0
    }

    pub fn get(&self, lane: usize) -> u64 {
        self.0[lane]
    }

    pub fn add(&self, other: &Mu) -> Mu {
        Mu(std::array::from_fn(|l| self.0[l].saturating_add(other.0[l])))
    }

    pub fn shortfall(&self, room: &Mu) -> Mu {
        Mu(shortfall(&self.0, &room.0))
    }

    pub fn lane_moves(&self, before: &Mu) -> [LaneMove; 4] {
        lane_moves(&self.0, &before.0)
    }
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

/// THE SHORTFALL of a draw against a room, per lane: what the draw needs
/// beyond the room, and zero where the lane fits (S9.6, 23.8). A reading,
/// never a verdict — the caller acts on the lanes that are short, and the
/// refusal it writes names them. The floor at zero is the accounting
/// floor of a deficit, not a saturation of a measurement.
#[expect(
    clippy::manual_unwrap_or_default,
    reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; a lane that fits is short by nothing, the empty difference"
)]
pub fn shortfall(draw: &[u64; 4], room: &[u64; 4]) -> [u64; 4] {
    std::array::from_fn(|l| match draw[l].checked_sub(room[l]) {
        Some(short) => short,
        None => 0,
    })
}

/// The lanes that are SHORT in a shortfall reading, by index.
pub fn lanes_short(short: &[u64; 4]) -> Vec<usize> {
    (0..4).filter(|&l| short[l] > 0).collect()
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

/// THE YIELD SURFACE — the metabolism's per-lane reading a reply carries
/// and a voice dwells on, in strokes (24.3, R3): the reserve deficit (the
/// depletion premium, bounded by the calibrated reserve) plus the entropic
/// excess (torsion outrunning syntropy). Strained lanes read high; settled
/// lanes read low — never a static faucet, never a summed scalar
/// (polytonic, #34). The open-demand backlog is NOT on it: pressure is a
/// magnitude the metabolism relaxes and `price` reads the word's own
/// measure, so it priced nothing and paced everything — read as strokes it
/// held every voice on one hypha for 48,000 strokes (2026-09-09).
pub fn yield_surface(p: &crate::physics::Physics) -> [u64; 4] {
    let flux = p.order_flux();
    let deficit = p.reserve_deficit();
    std::array::from_fn(|l| {
        let entropic = (-flux[l]).max(0) as u64;
        deficit[l].saturating_add(entropic)
    })
}

/// PRICE IS THE MAGNITUDE ON THE LANE: a verified word is worth what it
/// deflated. The surface paces pressure, it does not scale the mint.
pub fn price(mag: &[u64; 4], _surface: &[u64; 4]) -> [u64; 4] {
    *mag
}
