// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Fixture constructors for the field carrier's claims.

use super::super::*;
use crate::core::Q;
use crate::transducer::sample::{Place, Sample};
use forms::coordinate::Square;
use num_bigint::BigInt;

/// The sample of `extents` whose sites hold the digits of `digits` in
/// canonical address order, at symbol width 1.
pub(super) fn sample_of(extents: &[usize], digits: &str) -> Sample {
    let symbols = digits.bytes().map(|b| b - b'0').collect();
    Sample::new(extents.to_vec(), 1, symbols, Place::new(vec![0])).expect("the fixture stands")
}

/// An exact number of degrees, `n/d`.
pub(super) fn degrees(n: i64, d: i64) -> Q {
    Q::new(BigInt::from(n), BigInt::from(d))
}

/// The field's frame of `count` planes.
pub(super) fn frame(count: u32) -> FramePlanes {
    FramePlanes::of(count).expect("the frame stands")
}

/// The turning plane `e₊ₖe₊ₗ` of `frame`.
pub(super) fn plane(frame: &FramePlanes, k: u32, l: u32) -> TurningPlane {
    TurningPlane::of(frame, Square::Positive, k, l).expect("the plane stands")
}

/// The turning by `n/d` degrees on `plane`.
pub(super) fn turning(plane: &TurningPlane, n: i64, d: i64) -> Turning {
    Turning::of(plane.clone(), degrees(n, d)).expect("the turning stands")
}

/// The lift of a rank-1 sample of three bodies of measures 2, 3 and 1, and
/// its three charts in the atlas's order.
pub(super) fn three_charts() -> (Lifted, Vec<Chart>) {
    let lifted = Lifted::of(&sample_of(&[6], "112223")).expect("the sample lifts");
    let charts = lifted.atlas().charts().to_vec();
    (lifted, charts)
}

/// The lap through `charts` in order, each step a turning by `n/d` degrees
/// on `plane`.
pub(super) fn lap_turning(charts: &[Chart], plane: &TurningPlane, n: i64, d: i64) -> Orbit {
    let steps = charts.len();
    let lap = (0..steps)
        .map(|i| Transition {
            from: charts[i].clone(),
            to: charts[(i + 1) % steps].clone(),
            motion: Motion::of(vec![turning(plane, n, d)]).expect("one turning is a motion"),
        })
        .collect();
    Orbit::of(lap).expect("the lap closes")
}

/// The atlas of `lifted`'s charts holding one attractor at its first chart,
/// its local orbit `orbit` and its membrane on the lift's own plane.
pub(super) fn atlas_holding(lifted: &Lifted, orbit: Orbit) -> Atlas {
    let mut atlas = lifted.atlas();
    let centre = atlas.charts()[0].clone();
    atlas.hold(Attractor { centre, orbit, membrane: lifted.membrane.clone() }).expect("the attractor is charted");
    atlas
}
