// 78.62 — the register and the verifier, tested by the algebra: a turn that
// carries its bodies is admitted under every law, and each law refuses the
// transition that breaks it, by name, with every other breach beside it.

use super::*;
use crate::physics::tensegrity::{metric_defect, similarity};
use crate::physics::tensegrity::carried_body;
use crate::transducer::cga::{point_at, rotator, sphere_at};

fn q(n: i64) -> Q {
    Q::from_integer(n.into())
}

fn budget() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// A body at `centre` with a second moment on its first axis.
fn body(centre: [i64; 3]) -> Body {
    Body {
        centre: centre.iter().map(|&c| q(c)).collect(),
        mass: q(1),
        moments: vec![vec![q(1), q(0), q(0)], vec![q(0), q(0), q(0)], vec![q(0), q(0), q(0)]],
    }
}

/// The (3,4,5) turn in (e₁, e₂) — `2 + e₁₂`, a rational rotor, no angle.
fn turn() -> Mv {
    rotator(&q(2), &q(1))
}

/// Two bodies carried by `step` exactly: nucleus (a unit sphere — a point
/// is null and its inversive distance unread) by the sandwich, body by the
/// rigid carriage.
fn carried(step: &Mv) -> Vec<(BodyState, BodyState)> {
    [[1, 0, 0], [0, 2, 0]]
        .iter()
        .map(|&c| {
            let b = body(c);
            let nucleus = sphere_at(&b.centre, &q(1));
            let after = BodyState { nucleus: step.sandwich(&nucleus), body: carried_body(&b, step).expect("a rigid carriage") };
            (BodyState { nucleus, body: b }, after)
        })
        .collect()
}

#[test]
fn the_register_holds_five_laws_each_at_its_own_coordinate() {
    let addresses: Vec<_> = LAWS.iter().map(|l| l.address()).collect();
    for (law, a) in LAWS.iter().zip(&addresses) {
        assert_eq!(a.as_bytes(), [crate::transducer::address::TAG_LAW, law.index(), LAW_VERSION].as_slice());
        assert!(!law.statement().is_empty());
    }
    for (i, a) in addresses.iter().enumerate() {
        assert!(addresses.iter().skip(i + 1).all(|b| b != a), "every law at its own coordinate");
    }
}

#[test]
fn a_turn_that_carries_its_bodies_is_admitted_under_every_law() {
    let t = Transition { step: turn(), heights: (7, 8), bodies: carried(&turn()), hues: Vec::new() };
    let v = verify(&t, &mut budget());
    assert!(v.admitted(), "{:?}", v.breaches);
    assert!(v.modes >= 1, "the turn split into its modes");
    assert!(v.exchanged.is_empty(), "an unforced turn exchanges no torque");
}

#[test]
fn each_law_refuses_what_breaks_it_and_every_breach_is_named() {
    let laws_of = |v: &Verdict| v.breaches.iter().map(|(l, _)| *l).collect::<Vec<_>>();
    // TIME: the height stands still.
    let still = Transition { step: turn(), heights: (8, 8), bodies: carried(&turn()), hues: Vec::new() };
    assert_eq!(laws_of(&verify(&still, &mut budget())), vec![Law::Time]);
    // CONFORMAL: a body's nucleus is not where the step sends it.
    let mut moved = carried(&turn());
    if let [(_, after), ..] = moved.as_mut_slice() {
        after.nucleus = sphere_at(&[q(5), q(5), q(0)], &q(1));
    }
    let v = verify(&Transition { step: turn(), heights: (1, 2), bodies: moved, hues: Vec::new() }, &mut budget());
    assert!(v.breaches.contains(&(Law::Conformal, Breach::NotCarried { body: 0 })), "{:?}", v.breaches);
    assert!(laws_of(&v).contains(&Law::Rigidity), "and the pair's distance moved: {:?}", v.breaches);
    // MOMENTUM: a body's inertia changed with no body to answer for it.
    let mut spun = carried(&turn());
    if let [(_, after), ..] = spun.as_mut_slice() {
        after.body.moments[1][1] = q(3);
    }
    let v = verify(&Transition { step: turn(), heights: (1, 2), bodies: spun, hues: Vec::new() }, &mut budget());
    assert!(matches!(v.breaches.as_slice(), [(Law::Momentum, Breach::MomentumNotConserved { .. })]), "{:?}", v.breaches);
    // RESERVOIR, beside TIME: a merge whose moved preimage is not recorded.
    let lossy = Transition { step: turn(), heights: (3, 3), bodies: carried(&turn()), hues: vec![(1, 2), (2, 2)] };
    assert_eq!(laws_of(&verify(&lossy, &mut budget())), vec![Law::Time, Law::Reservoir], "every breach named, not the first");
}

/// 78.59 — A POINT IS READ, AT UNIT WEIGHT: two point nuclei carried by the
/// turn keep their product (no squared ratio was ever needed). A uniform
/// dilation with no step moves every pair — rigidity is strict — and reads
/// as one ratio, the dilation lane; a drift of one body moves the pairs it
/// touches, by no one ratio.
#[test]
fn points_are_read_at_unit_weight_and_a_dilation_is_a_similarity() {
    let mut bodies = carried(&turn());
    for (before, after) in bodies.iter_mut() {
        before.nucleus = point_at(&before.body.centre);
        after.nucleus = turn().sandwich(&before.nucleus);
    }
    let v = verify(&Transition { step: turn(), heights: (1, 2), bodies: bodies.clone(), hues: Vec::new() }, &mut budget());
    assert!(v.admitted(), "{:?}", v.breaches);
    assert_eq!(v.similarity, Some(q(1)), "an isometry");
    let pts = |cs: &[[i64; 3]]| cs.iter().map(|c| point_at(&c.iter().map(|&x| q(x)).collect::<Vec<_>>())).collect::<Vec<_>>();
    let before = pts(&[[1, 0, 0], [0, 2, 0], [0, 0, 3]]);
    let doubled = pts(&[[2, 0, 0], [0, 4, 0], [0, 0, 6]]);
    let pairs: Vec<(Mv, Mv)> = before.iter().cloned().zip(doubled).collect();
    assert_eq!(metric_defect(&pairs).len(), 3, "rigidity is strict: a dilation with no step to answer for it moves every pair");
    assert_eq!(similarity(&pairs), Some(q(4)), "and reads as one ratio: every pair's product × 4, lengths × 2");
    let drifted = pts(&[[1, 0, 0], [0, 2, 0], [0, 0, 5]]);
    let pairs: Vec<(Mv, Mv)> = before.into_iter().zip(drifted).collect();
    assert_eq!(metric_defect(&pairs).len(), 2, "a drift moves the two pairs it touches, each named");
    assert_eq!(similarity(&pairs), None, "and no one ratio");
}
