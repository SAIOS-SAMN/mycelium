// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! LAW-FIELD v1 §3.2 (proposal 78.171), MU-SENSING §3.8, ROADMAP track 26 S9c
//! (#134) — CANDIDATE MONODROMY BETWEEN CHARTS: reading candidate monodromy
//! (flat, torsion, rupture) over the field's atlas, motion inversion, and
//! court reconstruction of stated monodromy, precession and holonomy.

use super::super::*;
use super::fixtures::{atlas_holding, frame, plane, revolution_turning, three_charts, turning};

/// Track 26 S9c (MU-SENSING §3.8; 78.171 §3.2) — THE MONODROMY OF A CANDIDATE
/// BETWEEN CHARTS: read over the atlas's connection, a closed geodesic of three
/// 120° steps is resonant and reads `Monodromy::Flat` (order 1); three 30°
/// steps advance by 90° and read `Monodromy::Torsion { order: 4 }`; and a
/// path in a flat region binds no orbit, refused `Unbound`.
#[test]
fn candidate_monodromy_reads_flat_torsion_and_unbound_between_charts() {
    let (lifted, charts) = three_charts();
    let p = plane(&lifted.frame, 1, 2);

    // Three 30° steps compose to 90°, order 4 torsion on e₊₁e₊₂.
    let atlas_torsion = atlas_holding(&lifted, revolution_turning(&charts, &p, 30, 1));
    assert_eq!(
        atlas_torsion.candidate_monodromy(&charts),
        Ok(Monodromy::Torsion { order: 4 }),
        "three 30° steps close at order 4"
    );

    // Three 120° steps compose to 360° with K = 1: resonant, flat monodromy (order 1).
    let atlas_resonant = atlas_holding(&lifted, revolution_turning(&charts, &p, 120, 1));
    assert_eq!(
        atlas_resonant.candidate_monodromy(&charts),
        Ok(Monodromy::Flat),
        "three 120° steps compose to 360°, K = 1: resonant, flat monodromy"
    );

    // A flat atlas holds no orbit: the path binds no orbit and is refused Unbound.
    let flat_atlas = lifted.atlas();
    assert_eq!(
        flat_atlas.candidate_monodromy(&charts),
        Err(FieldRefusal::Unbound { charts: charts.to_vec() }),
        "flat region binds no orbit"
    );
}

/// A MOTION AND ITS INVERSE COMPOSE TO THE IDENTITY: each turning reversed on
/// its plane cancels the forward motion, exact over disjoint planes.
#[test]
fn motion_and_its_inverse_compose_to_the_identity() {
    let f = frame(2);
    let p1 = plane(&f, 1, 2);
    let t1 = turning(&p1, 30, 1);
    let m = Motion::of(vec![t1]).expect("motion of one turning");
    let inv = m.inverse().expect("inverse motion");
    let composed = m.then(&inv).expect("composed with inverse");
    assert_eq!(composed, Motion::identity(), "m ∘ m⁻¹ is the identity");
}
