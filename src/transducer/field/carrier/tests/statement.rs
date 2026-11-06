// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! The field stated as an object a court reconstructs (MU-SENSING §0;
//! ROADMAP track 10): the statement writes no reading, and the court derives each.

use super::super::*;
use super::fixtures::{atlas_holding, plane, revolution_turning, three_charts};

/// A stated atlas reconstructs to the same charts and, derived afresh by
/// the court, the same curvature around each attractor: the 90° holonomy
/// of a revolution of three 30° steps.
#[test]
fn put_statement_round_trips() {
    let (lifted, charts) = three_charts();
    let p = plane(&lifted.frame, 1, 2);
    let atlas = atlas_holding(&lifted, revolution_turning(&charts, &p, 30, 1));
    let mut stated = Vec::new();
    atlas.put_statement(&mut stated);
    let rebuilt = reconstruct(&stated).expect("reconstructs");
    assert_eq!(rebuilt.charts(), atlas.charts());
    assert_eq!(rebuilt.curvature(), atlas.curvature());
}

/// The court admits a mind's claimed curvature where the curvature it
/// derives from the stated field agrees, and refuses `Differs` with the
/// pair where the claim is the curvature of another field (45° steps
/// claimed over a field stated with 30° steps).
#[test]
fn admit_refuses_claimed_curvature_court_does_not_derive() {
    let (lifted, charts) = three_charts();
    let p = plane(&lifted.frame, 1, 2);
    let atlas = atlas_holding(&lifted, revolution_turning(&charts, &p, 30, 1));
    let mut stated = Vec::new();
    atlas.put_statement(&mut stated);
    assert_eq!(admit(&stated, &atlas.curvature()), Ok(atlas.curvature()));
    let other = atlas_holding(&lifted, revolution_turning(&charts, &p, 45, 1)).curvature();
    let expected = vec![(Some(other[0].clone()), Some(atlas.curvature()[0].clone()))];
    assert_eq!(admit(&stated, &other), Err(StatementRefusal::Differs { pairs: expected }));
}

/// A well's body is stated as its own atlas and its stage, and a court
/// reconstructs it and derives its curvature itself: the 90° of three 30°
/// steps per revolution admitted, a claim of 135° refused `Differs` with
/// the pair, and a flat well's `Unbound` derived on both sides.
#[test]
fn well_body_stated_and_admitted_by_its_curvature() {
    let (lifted, charts) = three_charts();
    let p = plane(&lifted.frame, 1, 2);
    let body = WellBody::of(lifted.frame, charts[0].clone(), Some(revolution_turning(&charts, &p, 30, 1)), WellStage::perceived()).expect("the well stands");
    let mut stated = Vec::new();
    body.put_statement(&mut stated).expect("stated");
    let claimed = Curvature { centre: charts[0].clone(), holonomy: body.holonomy() };
    let admitted = admit_well(&stated, &claimed).expect("admitted");
    assert_eq!(admitted.stage(), WellStage::perceived(), "the stage is stated and read back");
    let other = WellBody::of(lifted.frame, charts[0].clone(), Some(revolution_turning(&charts, &p, 45, 1)), WellStage::perceived()).expect("stands");
    let wrong = Curvature { centre: charts[0].clone(), holonomy: other.holonomy() };
    assert_eq!(admit_well(&stated, &wrong).map(|_| ()), Err(StatementRefusal::Differs { pairs: vec![(Some(wrong.clone()), Some(claimed))] }));
    let flat = WellBody::of(lifted.frame, charts[1].clone(), None, WellStage::default()).expect("stands");
    let mut stated = Vec::new();
    flat.put_statement(&mut stated).expect("stated");
    let unbound = Curvature { centre: charts[1].clone(), holonomy: Err(FieldRefusal::Unbound { charts: vec![charts[1].clone()] }) };
    assert!(admit_well(&stated, &unbound).is_ok(), "a flat well's Unbound is derived, never a stored reading");
}

/// A stated atlas holding two attractors is no well: refused `Malformed`.
#[test]
fn reconstruct_well_refuses_an_atlas_of_two_attractors() {
    let (lifted, charts) = three_charts();
    let p = plane(&lifted.frame, 1, 2);
    let mut atlas = atlas_holding(&lifted, revolution_turning(&charts, &p, 30, 1));
    atlas.hold(Attractor { centre: charts[1].clone(), orbit: Some(revolution_turning(&charts, &p, 30, 1)), membrane: lifted.membrane.clone() }).expect("held");
    let mut stated = Vec::new();
    atlas.put_statement(&mut stated);
    stated.extend([0, 0]);
    assert!(matches!(reconstruct_well(&stated), Err(FieldRefusal::Malformed { .. })));
}

/// A chart is an invariant's exact coordinates, a ratio among them: an
/// atlas on the chart of `(1, 3/7)` is stated and reconstructed to that
/// chart, the ratio read back whole.
#[test]
fn chart_of_ratio_invariant_round_trips() {
    use crate::transducer::stalk::{ChartAxis, Invariant};
    let ratio = num_rational::BigRational::new(3.into(), 7.into());
    let chart = Chart::of_invariant(&Invariant(vec![ChartAxis::Count(1), ChartAxis::Ratio(ratio)]));
    let atlas = Atlas::of(super::fixtures::frame(2), vec![chart.clone()]);
    let mut stated = Vec::new();
    atlas.put_statement(&mut stated);
    assert_eq!(reconstruct(&stated).expect("reconstructs").charts(), &[chart]);
}

/// A statement with bytes past its writing is refused `Malformed` at the
/// first byte past it, never read as a field.
#[test]
fn reconstruct_refuses_trailing_bytes() {
    let (lifted, _) = three_charts();
    let mut stated = Vec::new();
    lifted.atlas().put_statement(&mut stated);
    let end = stated.len();
    stated.push(0);
    assert!(matches!(reconstruct(&stated), Err(FieldRefusal::Malformed { at }) if at == end));
}
