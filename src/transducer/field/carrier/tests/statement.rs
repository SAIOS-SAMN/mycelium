// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! The field stated as an object a court reconstructs (MU-SENSING §0;
//! ROADMAP track 10): the statement writes no reading, and the court derives each.

use super::super::*;
use super::fixtures::{atlas_holding, lap_turning, plane, three_charts};

/// A stated atlas reconstructs to the same charts and, derived afresh by
/// the court, the same curvature around each attractor: the 90° precession
/// of a lap of three 30° steps.
#[test]
fn put_statement_round_trips() {
    let (lifted, charts) = three_charts();
    let p = plane(&lifted.frame, 1, 2);
    let atlas = atlas_holding(&lifted, lap_turning(&charts, &p, 30, 1));
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
    let atlas = atlas_holding(&lifted, lap_turning(&charts, &p, 30, 1));
    let mut stated = Vec::new();
    atlas.put_statement(&mut stated);
    assert_eq!(admit(&stated, &atlas.curvature()), Ok(atlas.curvature()));
    let other = atlas_holding(&lifted, lap_turning(&charts, &p, 45, 1)).curvature();
    let expected = vec![(Some(other[0].clone()), Some(atlas.curvature()[0].clone()))];
    assert_eq!(admit(&stated, &other), Err(StatementRefusal::Differs { pairs: expected }));
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
