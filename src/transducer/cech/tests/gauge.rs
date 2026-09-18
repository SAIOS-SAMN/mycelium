// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 13, 14, 12.2 — WHAT A TREE WALK PRODUCES: the 0-cochain that places the
//! vertices, and the 1-cocycle the chords carry that no placement removes.
//!
//! `solve_gauge` formed `Ω` inside its disagreement test and dropped it,
//! returning `None` for the whole complex when any one chord disagreed —
//! one loop's class erasing every vertex the tree had placed. These read
//! both halves of what it now returns.

use crate::transducer::alphabet::{Order, Rotor, Word};
use crate::transducer::cech::{solve_gauge, Gauge};
use crate::transducer::element::{identity_versor, same_versor, Versor};

/// The quarter turn on a named plane, as a rotor of order four. Two turns
/// on DIFFERENT planes, never one scaled against another: a rotor told
/// apart by a scalar carries no dimension the next reader can use.
fn quarter_on(plane: (u8, u8)) -> Rotor {
    let id = crate::transducer::alphabet::LetterId::quarter(plane).expect("a plane of the chart");
    Word::from_letters(&[(id, false)]).rotor()
}

fn edge(i: usize, j: usize, g: Rotor) -> (usize, usize, Rotor, Versor) {
    (i, j, g, identity_versor())
}

/// A TREE THAT ITS CHORD AGREES WITH IS EXACT: the sections are placed and
/// there is no class. The triangle `0–1–2` with the chord `0–2` carrying
/// exactly what the two tree edges compose to.
#[test]
fn a_chord_that_agrees_leaves_no_class() {
    let r = quarter_on((0, 1));
    let edges = [edge(0, 1, r.clone()), edge(1, 2, Rotor::identity()), edge(0, 2, r.clone())];
    let Some(Gauge { sections, holonomy }) = solve_gauge(3, &edges) else {
        panic!("every reading here is invertible");
    };
    assert!(holonomy.is_empty(), "the chord agrees with the tree path: δγ = ω exactly");
    assert_eq!(sections.len(), 3, "and every vertex is placed");
    assert!(sections[0].0.is_identity(), "the root sits at the identity");
    assert!(sections[1].0.projective_eq(&r), "the next carries the tree edge");
    assert!(
        same_versor(&sections[2].1, &identity_versor()),
        "and the carriage half is placed with it"
    );
}

/// A CHORD THAT DISAGREES CARRIES THE CLASS, AND THE TREE STILL PLACED ITS
/// VERTICES. `Ω = (tree path) · (chord)⁻¹` is the Čech 1-cocycle: the
/// obstruction to the sections gluing, and the one object in the walk a
/// spanning forest does not already determine.
///
/// It was computed here and discarded. The whole gauge came back `None`,
/// so the three placements the tree had earned were thrown away with the
/// one loop's class, and the caller fell through to a reading that never
/// sees the loop.
#[test]
fn a_chord_that_disagrees_returns_the_cocycle_it_measures() {
    let tree = quarter_on((0, 1));
    let edges =
        [edge(0, 1, tree.clone()), edge(1, 2, Rotor::identity()), edge(0, 2, Rotor::identity())];
    let Some(Gauge { sections, holonomy }) = solve_gauge(3, &edges) else {
        panic!("a disagreement is a class, not an unreadable gauge");
    };
    assert_eq!(sections.len(), 3, "the tree placed its vertices whatever the chord says");
    assert_eq!(holonomy.len(), 1, "one chord, one loop, one class");
    let (i, j, class, carriage) = &holonomy[0];
    assert_eq!((*i, *j), (0, 2), "and it names the chord it closed");
    assert!(
        class.projective_eq(&tree),
        "the loop carries the quarter turn the chord did not: Ω = γ_2 γ_0⁻¹ g⁻¹"
    );
    assert_eq!(class.order(), Some(Order::Finite(4)), "H¹ here is torsion of order four");
    assert!(
        same_versor(carriage, &identity_versor()),
        "and the carriage half of the loop closes, which the rotor half did not"
    );
}

/// BOTH HALVES, SEPARATELY. A chord whose ROTOR agrees and whose CARRIAGE
/// does not is still a class: the loop returns to the same orientation at a
/// different place. A reader of the rotor alone called this exact.
#[test]
fn a_loop_that_returns_turned_right_and_placed_wrong_is_a_class() {
    use crate::transducer::cga;
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let walk = cga::translator(&q(3), &q(0), &q(0));
    let edges = [
        (0usize, 1usize, Rotor::identity(), walk.clone()),
        (1, 2, Rotor::identity(), identity_versor()),
        (0, 2, Rotor::identity(), identity_versor()),
    ];
    let Some(Gauge { holonomy, .. }) = solve_gauge(3, &edges) else {
        panic!("a translator is invertible");
    };
    assert_eq!(holonomy.len(), 1, "the placements disagree round the loop");
    let (_, _, class, carriage) = &holonomy[0];
    assert!(class.is_identity(), "nothing turned");
    assert!(!same_versor(carriage, &identity_versor()), "and the loop closes three sites over");
}
