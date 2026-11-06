// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! The sample lifted into the field (MU-SENSING §3.6–§3.7; ROADMAP track 26
//! S4a): phases in degrees on turning planes, windings kept, charts invariant.

use super::super::*;
use super::fixtures::{atlas_holding, degrees, plane, revolution_turning, sample_of, three_charts};
use crate::transducer::family::transduce::{Enclosed, Sense, SignedPermutation};
use forms::coordinate::Plane;
use forms::passage::Place;
use num_bigint::BigInt;

/// A sample of rank 3 lifts onto four frame planes; every boundary point
/// holds one phase per axis, axis `i`'s on `e₊ᵢe₊₄`, a plane whose two
/// units square alike, each phase exact in degrees within `[0°, 360°]`. No
/// norm, radius or reference surface is held.
#[test]
fn lift_holds_one_phase_per_axis_on_turning_planes() {
    let lifted = Lifted::of(&sample_of(&[2, 3, 2], "110110220220")).expect("lifts");
    assert_eq!(lifted.frame.count(), 4);
    let written: Vec<String> = lifted.planes.iter().map(|p| p.key().written()).collect();
    assert_eq!(written, ["e_+1+4", "e_+2+4", "e_+3+4"]);
    assert!(lifted.planes.iter().all(|p| matches!(p.key().plane(), Some(Plane::Turning(_)))));
    for body in &lifted.bodies {
        for (lo, hi) in &body.boundary {
            for point in [lo, hi] {
                assert_eq!(point.phases.len(), 3);
                for (phase, plane) in point.phases.iter().zip(&lifted.planes) {
                    assert_eq!(phase.plane(), plane);
                    assert!(phase.degrees() >= &degrees(0, 1) && phase.degrees() <= &degrees(360, 1));
                }
            }
        }
    }
}

/// A rank-1 sample's axis turns on `e₊₁e₊₂`, as LAW-FIELD's fixtures name
/// it, and a band reaching the far side of the seam holds 360° with its
/// winding `K = 1`, never read back as 0°.
#[test]
fn lift_keeps_far_seam_as_whole_turn_with_winding_one() {
    let lifted = Lifted::of(&sample_of(&[4], "1122")).expect("lifts");
    assert_eq!(lifted.planes[0].key().written(), "e_+1+2");
    let far: Vec<&Turning> = lifted.bodies.iter().flat_map(|b| &b.boundary).map(|(_, hi)| &hi.phases[0]).collect();
    assert!(far.iter().any(|t| t.degrees() == &degrees(360, 1) && t.winding() == BigInt::from(1)));
    assert!(far.iter().any(|t| t.degrees() == &degrees(180, 1) && t.winding() == BigInt::from(0)));
}

/// The charts are the bodies' invariant coordinates: a sample and its image
/// under a signed permutation of its phases (two axes exchanged, one
/// reversed) lift onto the same charts.
#[test]
fn lift_charts_are_invariant_under_signed_permutation() {
    let x = sample_of(&[2, 3], "112133");
    let sigma = SignedPermutation::new(vec![(1, Sense::Reversed), (0, Sense::Kept)]).expect("a signed permutation");
    let e = Enclosed::of(&x);
    let y = sigma.spoken_onto(&e, &sigma.enclosure(&e.enclosure).expect("rank 2")).expect("lands");
    let charts = |s| {
        let mut c: Vec<Chart> = Lifted::of(s).expect("lifts").bodies.into_iter().map(|b| b.chart).collect();
        c.sort();
        c
    };
    assert_eq!(charts(&x), charts(&y));
}

/// The field curves only where an orbit is held: a path through the lift's
/// three charts binds no orbit on the flat atlas (refused `Unbound`), and
/// once an attractor binds its orbit of three 30° steps on `e₊₁e₊₂`, the
/// same revolution read from the connection precesses by 90°.
#[test]
fn atlas_revolution_reads_holonomy_only_where_orbit_held() {
    let (lifted, charts) = three_charts();
    let flat = lifted.atlas();
    assert_eq!(flat.revolution(&charts), Err(FieldRefusal::Unbound { charts: charts.clone() }));
    let p = plane(&lifted.frame, 1, 2);
    let curved = atlas_holding(&lifted, revolution_turning(&charts, &p, 30, 1));
    let h = curved.revolution(&charts).and_then(|o| o.holonomy()).expect("the held orbit curves the revolution");
    assert_eq!(h.angle().map(|t| t.degrees().clone()), Ok(degrees(90, 1)));
    assert_eq!(curved.curvature().len(), 1, "one attractor, one well");
}

/// The lift's membrane is the throat of its own plane, the frame state on
/// `e₊` alone, never null; and the lift's atlas holds no attractor, so it
/// is flat and a path through its charts binds no orbit (`Unbound`).
#[test]
fn lift_membrane_is_throat_and_atlas_is_flat() {
    let lifted = Lifted::of(&sample_of(&[3, 3], "111121111")).expect("lifts");
    assert_eq!(lifted.membrane.place(), Ok(Place::Throat));
    assert_eq!(lifted.membrane.plane(), 3, "the lift's own plane, n + 1");
    let atlas = lifted.atlas();
    assert!(atlas.curvature().is_empty(), "nothing held, no curvature");
    let charts = atlas.charts().to_vec();
    assert_eq!(atlas.revolution(&charts), Err(FieldRefusal::Unbound { charts }));
}
