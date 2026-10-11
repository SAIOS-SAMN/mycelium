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
/// measurements is `lane_moves`, per lane, never a verdict (12.6).
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

    /// Lane-wise, all lanes or none: `None` where a lane would pass its
    /// carrier (STD1) — money never tops out silently.
    pub fn add(&self, other: &Mu) -> Option<Mu> {
        crate::core::lanes::lanes_add(self.0, other.0).map(Mu)
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

/// WHAT ONE LANE OF A PAYOUT READ: no data at all — a lane its measure
/// cannot carry, never a zero standing for one (rule 11) — or a reading,
/// whose `Held` is an open lane that moved by nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaneRead {
    Unread,
    Read(LaneMove),
}

/// A lane nothing has read yet is unread — the reading of absence.
impl Default for LaneRead {
    fn default() -> Self {
        LaneRead::Unread
    }
}

impl LaneRead {
    /// The lane as a person reads it: `unread`, `held`, `fell n`, `rose n`.
    pub fn print(&self) -> String {
        match self {
            LaneRead::Unread => "unread".to_string(),
            LaneRead::Read(LaneMove::Held) => "held".to_string(),
            LaneRead::Read(LaneMove::Fell(n)) => format!("fell {n}"),
            LaneRead::Read(LaneMove::Rose(n)) => format!("rose {n}"),
        }
    }

    /// The wire's writing: `[state u8][amount u64 LE]`, the state 0 unread,
    /// 1 held, 2 fell, 3 rose.
    pub fn put(&self, out: &mut Vec<u8>) {
        let (s, n) = match self {
            LaneRead::Unread => (0u8, 0u64),
            LaneRead::Read(LaneMove::Held) => (1, 0),
            LaneRead::Read(LaneMove::Fell(n)) => (2, *n),
            LaneRead::Read(LaneMove::Rose(n)) => (3, *n),
        };
        out.push(s);
        out.extend_from_slice(&n.to_le_bytes());
    }

    /// The writing read back, whole or refused.
    pub fn take(b: &[u8], p: &mut usize) -> Option<LaneRead> {
        let s = *b.get(*p)?;
        let n = u64::from_le_bytes(b.get(*p + 1..*p + 9)?.try_into().ok()?);
        *p += 9;
        Some(match s {
            0 => LaneRead::Unread,
            1 => LaneRead::Read(LaneMove::Held),
            2 => LaneRead::Read(LaneMove::Fell(n)),
            3 => LaneRead::Read(LaneMove::Rose(n)),
            _ => return None,
        })
    }

    /// TWO READINGS OF ONE LANE TOGETHER, as one payout's pairs or one mind's
    /// solves add up: what fell adds to what fell, what rose to what rose,
    /// and a lane read anywhere is read. Exact or refused past the carrier.
    pub fn and(&self, other: &LaneRead) -> Option<LaneRead> {
        let parts = |r: &LaneRead| match r {
            LaneRead::Unread => None,
            LaneRead::Read(LaneMove::Held) => Some((0u64, 0u64)),
            LaneRead::Read(LaneMove::Fell(n)) => Some((*n, 0)),
            LaneRead::Read(LaneMove::Rose(n)) => Some((0, *n)),
        };
        Some(match (parts(self), parts(other)) {
            (None, None) => LaneRead::Unread,
            (Some(x), None) | (None, Some(x)) => read_of(x),
            (Some((f1, r1)), Some((f2, r2))) => read_of((f1.checked_add(f2)?, r1.checked_add(r2)?)),
        })
    }
}

/// A lane's fall and rise together as one reading: the fall where it fell, the
/// rise where it only rose, held where it did neither.
fn read_of((fell, rose): (u64, u64)) -> LaneRead {
    match (fell, rose) {
        (0, 0) => LaneRead::Read(LaneMove::Held),
        (0, r) => LaneRead::Read(LaneMove::Rose(r)),
        (f, _) => LaneRead::Read(LaneMove::Fell(f)),
    }
}

/// A READING OF FOUR LANES, each read only where `read` says the measure
/// carries it: `lane_moves` on the lanes that have data, `Unread` on the rest.
pub fn lane_reads(after: &[u64; 4], before: &[u64; 4], read: &[bool; 4]) -> [LaneRead; 4] {
    let moves = lane_moves(after, before);
    std::array::from_fn(|l| match read[l] {
        true => LaneRead::Read(moves[l]),
        false => LaneRead::Unread,
    })
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
///
/// `None` where a lane's deficit plus its excess would pass its carrier
/// (STD1): the surface is refused by name, never read pinned.
pub fn yield_surface(p: &crate::physics::Physics) -> Option<[u64; 4]> {
    let flux = p.order_flux();
    let deficit = p.reserve_deficit();
    let mut out = [0u64; 4];
    for (l, o) in out.iter_mut().enumerate() {
        // The excess is torsion past syntropy: exact in `i128`, and a
        // difference of two `u64` below zero is at most one `u64` whole.
        let entropic = match flux[l] < 0 {
            true => u64::try_from(-flux[l]).ok()?,
            false => 0,
        };
        *o = deficit[l].checked_add(entropic)?;
    }
    Some(out)
}

/// PRICE IS THE MAGNITUDE ON THE LANE: a verified word is worth what it
/// deflated. The surface paces pressure, it does not scale the mint.
pub fn price(mag: &[u64; 4], _surface: &[u64; 4]) -> [u64; 4] {
    *mag
}
