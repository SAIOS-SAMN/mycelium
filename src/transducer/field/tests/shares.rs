// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-ANASTOME §9.4 (ruled 2026-11-06; issue #149), LAW-PASSAGE v3 (78.172)
// — THE COHESION OF A HELD WHOLE, PER PLANE: the held share `H/(H + R)`
// of the frames a well stands on, the viscosity, beside the open share,
// the fluidity; a keyed reading, never stored and never summed.

use crate::core::Q;
use crate::transducer::field::carrier::*;
use crate::transducer::field::*;
use crate::transducer::sentence::tests::fixtures::link;
use forms::coordinate::{Axis, Key, Square};
use forms::passage::PassageState;
use num_traits::One;

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

/// One thought held from a remainder of two links.
fn held() -> Well {
    let mut c = Contemplation::new();
    c.hold(vec![link(0, 0, 1, 2), link(0, 1, 1, 2)]);
    c.wells.scoped().next().map(|(_, w)| w.clone()).expect("one well held")
}

/// The continuum's turning `e₊ₖe₊ₗ`.
fn turning(k: u32, l: u32) -> Key {
    let unit = |i| Axis::unit(Square::Positive, i).expect("a unit");
    Key::generator(unit(k), unit(l)).expect("a turning plane")
}

/// A HELD THOUGHT'S COHESION IS THE HELD SHARE OF ITS INFLATION, per the
/// plane it stands on: one component, keyed by its membrane's frame plane
/// (`e_+k-k`), equal to `H/(H + R)` of its gauge; a thought whose cover
/// closed every chart reads 1, the closed pole, and its fluidity 0.
#[test]
fn well_cohesion_is_the_held_share_of_its_inflation() {
    let well = held();
    let cohesion = well.cohesion().expect("a reading");
    let plane = pole_plane(well.body().expect("stands").membrane().plane()).expect("the membrane's plane");
    let gauge = well.inflation().expect("a gauge").gauge;
    assert_eq!(cohesion.planes().len(), 1, "one frame, one plane");
    assert_eq!(cohesion.on(&plane), gauge.share().ok().as_ref());
    assert_eq!(cohesion.on(&plane), Some(&Q::one()), "nothing open: the closed pole");
    assert_eq!(cohesion.fluidity(), vec![(plane, q(0, 1))]);
}

/// AT THE THROAT BOTH SHARES ARE ½ on the well's body (`H = R`, its
/// membrane); off it they differ, and the passage's pole swap on the
/// membrane's poles exchanges the well's cohesion and fluidity.
#[test]
fn well_body_cohesion_half_at_the_throat_and_exchanged_by_the_passage() {
    let body = held().body().expect("stands");
    let plane = pole_plane(body.membrane().plane()).expect("the membrane's plane");
    let at_throat = body.cohesion(q(3, 1), q(3, 1)).expect("a reading");
    assert_eq!(at_throat.on(&plane), Some(&q(1, 2)));
    assert_eq!(at_throat.fluidity(), vec![(plane.clone(), q(1, 2))]);
    let poles = body.membrane().poles();
    for (h, r) in [(q(2, 1), q(1, 1)), (q(1, 3), q(5, 3)), (q(4, 1), q(0, 1))] {
        let before = body.cohesion(h.clone(), r.clone()).expect("a reading");
        let state = poles.state(&PassageState { hold: h, reach: r }).expect("a state");
        let passed = poles.read(&poles.swap(&state).expect("the swap")).expect("a frame");
        let after = body.cohesion(passed.hold, passed.reach).expect("a reading");
        let fluidity_on = |c: &Cohesion| c.fluidity().into_iter().find(|(k, _)| *k == plane).map(|(_, nu)| nu);
        assert_eq!(after.on(&plane).cloned(), fluidity_on(&before), "the cohesion past is the fluidity before");
        assert_eq!(before.on(&plane).cloned(), fluidity_on(&after), "and the reverse");
    }
}

/// COHESION IS READ PER PLANE, NEVER SUMMED: frames on three planes read
/// three components, each its own held share, never their total or mean;
/// a plane named twice (in either orientation) is refused, and a key that
/// holds no frame (one axis, or a mixed pair) is refused.
#[test]
fn well_cohesion_read_per_plane_never_summed() {
    let frame = |h, r| PassageState { hold: q(h, 1), reach: q(r, 1) };
    let planes = [(turning(1, 2), frame(2, 1)), (turning(2, 3), frame(1, 1)), (pole_plane(1).expect("e_+1-1"), frame(1, 3))];
    let cohesion = Cohesion::of(&planes).expect("a reading");
    assert_eq!(cohesion.planes().len(), 3);
    assert_eq!(cohesion.on(&turning(1, 2)), Some(&q(2, 3)));
    assert_eq!(cohesion.on(&turning(2, 3)), Some(&q(1, 2)));
    assert_eq!(cohesion.on(&pole_plane(1).expect("e_+1-1")), Some(&q(1, 4)));
    let twice = [(turning(1, 2), frame(2, 1)), (turning(2, 1), frame(1, 1))];
    assert!(matches!(Cohesion::of(&twice), Err(FieldRefusal::NotTurningPlane(_))));
    let one_axis = Key::axis(Axis::unit(Square::Positive, 1).expect("a unit"));
    assert!(matches!(Cohesion::of(&[(one_axis, frame(1, 1))]), Err(FieldRefusal::NotTurningPlane(_))));
    let mixed = Key::generator(Axis::unit(Square::Positive, 1).expect("+1"), Axis::unit(Square::Negative, 2).expect("-2")).expect("a key");
    assert!(matches!(Cohesion::of(&[(mixed, frame(1, 1))]), Err(FieldRefusal::NotTurningPlane(_))));
}

/// CONTEMPLATION READS COHESION OVER HELD THOUGHTS (MU-CLIENT §4.9, §7.1):
/// an empty contemplation has `None`; holding a thought reads its cohesion
/// on its membrane's plane, its viscosity and fluidity summing to 1.
#[test]
fn contemplation_cohesion_reads_per_plane_over_held_wells() {
    let mut c = Contemplation::new();
    assert_eq!(c.cohesion().expect("reading"), None, "empty contemplation has no cohesion");

    c.hold(vec![link(0, 0, 1, 2), link(0, 1, 1, 2)]);
    let (key, well) = c.wells.scoped().next().map(|(k, w)| (k.clone(), w.clone())).expect("held well");

    let cohesion = c.cohesion().expect("reading").expect("cohesion stands");
    let plane = pole_plane(well.body().expect("stands").membrane().plane()).expect("pole plane");
    assert_eq!(cohesion.planes().len(), 1);
    assert_eq!(cohesion.on(&plane), Some(&Q::one()));

    let of_thought = c.cohesion_of(&key).expect("thought stands").expect("cohesion of thought");
    assert_eq!(of_thought, cohesion);

    let fluidity = cohesion.fluidity();
    assert_eq!(fluidity, vec![(plane, q(0, 1))]);
}

/// FIRST-DEGREE PLANAR ORBITAL SHEAR AND COHESION MULTI-PLANE UNITY:
/// on every plane k, c_k + d_k = 1 holds identically; planar orbital shear
/// Δθ_k = 360° · c_k = 360° / (1 + t_k) is strictly first-degree without UV
/// singularity at r -> 0 (bounded at 360°) and without IR drop-off requiring
/// dark matter patches at r -> ∞.
#[test]
fn first_degree_planar_orbital_shear_and_throat_traversal() {
    let frame = |h, r| PassageState { hold: q(h, 1), reach: q(r, 1) };
    let p1 = turning(1, 2);
    let p2 = turning(2, 3);
    let p3 = pole_plane(1).expect("pole plane 1");

    // p1: throat (t = 1, h = 5, r = 5) => c = 1/2, d = 1/2
    // p2: deep hold (t = 1/3, h = 3, r = 1) => c = 3/4, d = 1/4
    // p3: far reach (t = 3, h = 1, r = 3) => c = 1/4, d = 3/4
    let planes = [
        (p1.clone(), frame(5, 5)),
        (p2.clone(), frame(3, 1)),
        (p3.clone(), frame(1, 3)),
    ];
    let cohesion = Cohesion::of(&planes).expect("valid cohesion");

    // 1. Unity on every plane: c_k + d_k = 1
    for (k, d_k) in cohesion.fluidity() {
        let c_k = cohesion.on(&k).expect("cohesion present");
        assert_eq!(c_k + &d_k, Q::one(), "cohesion + fluidity = 1 on plane {}", k.written());
    }

    // 2. Throat traversal state
    assert_eq!(cohesion.traversal_state_on(&p1), Some(TraversalState::Balanced));
    assert_eq!(cohesion.traversal_state_on(&p2), Some(TraversalState::TooCohesive));
    assert_eq!(cohesion.traversal_state_on(&p3), Some(TraversalState::TooFluid));

    // 3. Planar orbital shear Δθ_k = 360° · c_k
    assert_eq!(cohesion.planar_shear_on(&p1), Some(q(180, 1))); // 360 * 1/2 = 180°
    assert_eq!(cohesion.planar_shear_on(&p2), Some(q(270, 1))); // 360 * 3/4 = 270°
    assert_eq!(cohesion.planar_shear_on(&p3), Some(q(90, 1)));  // 360 * 1/4 = 90°

    // 4. UV zero-singularity limit (r -> 0, t -> 0, c -> 1):
    let uv_cohesion = Cohesion::of(&[(p1.clone(), frame(100, 0))]).expect("uv frame");
    assert_eq!(uv_cohesion.on(&p1), Some(&Q::one()));
    assert_eq!(uv_cohesion.planar_shear_on(&p1), Some(q(360, 1)));
    assert_eq!(uv_cohesion.traversal_state_on(&p1), Some(TraversalState::TooCohesive));

    // 5. IR large reach (r >> M, e.g. t = 999, c = 1/1000):
    let ir_cohesion = Cohesion::of(&[(p1.clone(), frame(1, 999))]).expect("ir frame");
    assert_eq!(ir_cohesion.on(&p1), Some(&q(1, 1000)));
    assert_eq!(ir_cohesion.planar_shear_on(&p1), Some(q(360, 1000))); // 0.36°, first-degree decay, not 1/r^2
    assert_eq!(ir_cohesion.traversal_state_on(&p1), Some(TraversalState::TooFluid));
}
