// 78.152 TU-6 — WHAT A FINAL POINT LETS ONE DO. PINPOINT, UNPACK and UNROLL
// are court reads (`Court::pinpoint`, `unpack`, `unroll`); RESET, BRANCH
// and SUPERPOSE are typed here with the origin citation each needs — the
// recut flow that would act on them is not built (78.152, owed).

use super::point::{Origin, TPoint};
use crate::core::Q;

/// A POINT'S CITATION: the citation hash of exactly its wire coordinates
/// (`TPoint::put`) — what a new origin names it by, never its identity.
pub fn citation(p: &TPoint) -> [u8; 32] {
    crate::chronometry::payload_hash(&p.to_bytes())
}

/// UNPACK (TU-6): a point's coordinates by name, each lane exact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unpacked {
    pub origin: Origin,
    pub h: u64,
    pub s: Q,
    pub g: Q,
    pub t: Q,
    pub hh: Q,
    pub k: u64,
}

impl TPoint {
    /// The point's coordinates by name.
    pub fn unpack(&self) -> Unpacked {
        let [s, g, t, hh] = self.lanes.lanes().clone();
        Unpacked { origin: self.origin, h: self.h, s, g, t, hh, k: self.k }
    }
}

/// RESET (TU-6): a new epoch re-originated at a final point. The new
/// universe's genesis is its first receipt, written at the recut; what this
/// carries is the citation that genesis's mesh record names (`cites`) and
/// the epoch it takes — one past the point's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reset {
    pub at: TPoint,
    pub cites: [u8; 32],
    pub epoch: u32,
}

impl Reset {
    /// The reset at `at`; `None` where the epoch passes its carrier.
    pub fn at(at: TPoint) -> Option<Reset> {
        let (cites, epoch) = (citation(&at), at.origin.epoch.checked_add(1)?);
        Some(Reset { at, cites, epoch })
    }
}

/// BRANCH (TU-6): a sub-universe whose origin's parent is the point. Its
/// genesis is the citation of the point and the branch's own name, its epoch
/// zero: two branches of one point are two universes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Branch {
    pub parent: TPoint,
    pub name: [u8; 32],
    pub origin: Origin,
}

impl Branch {
    pub fn of(parent: TPoint, name: [u8; 32]) -> Branch {
        let mut b = b"mycelium branch v1".to_vec();
        b.extend_from_slice(&citation(&parent));
        b.extend_from_slice(&name);
        Branch { parent, name, origin: Origin { genesis: crate::chronometry::payload_hash(&b), epoch: 0 } }
    }
}

/// SUPERPOSE (TU-6): two branches' points held side by side with the
/// residue between them — `b − a` per lane, an exact signed rational, and
/// on `K` — never merged by a reader.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Superposed {
    pub a: TPoint,
    pub b: TPoint,
    pub lanes: [Q; 4],
    pub k: i128,
}

impl Superposed {
    pub fn of(a: TPoint, b: TPoint) -> Superposed {
        let lanes = std::array::from_fn(|l| b.lanes.lane(l) - a.lanes.lane(l));
        let k = i128::from(b.k) - i128::from(a.k);
        Superposed { a, b, lanes, k }
    }

    /// MERGED ONLY BY A QUORUM-FINAL POINT CITING BOTH: `by` is final (the
    /// caller read it off `Keepers::final_point`), it stands past both ticks,
    /// and `cites` names exactly the two points' citations. `None` otherwise:
    /// the superposition stands.
    pub fn merged_by(&self, by: TPoint, cites: [[u8; 32]; 2]) -> Option<TPoint> {
        let both = [citation(&self.a), citation(&self.b)];
        let named = cites == both || cites == [both[1], both[0]];
        (named && by.h > self.a.h && by.h > self.b.h).then_some(by)
    }
}
