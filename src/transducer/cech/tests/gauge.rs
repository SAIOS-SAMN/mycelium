// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 13, 14, 12.2 — WHAT A TREE WALK PRODUCES: the 0-cochain that places the
//! vertices, and the 1-cocycle the chords carry that no placement removes.
//!
//! `solve_gauge` formed `Ω` inside its disagreement test and dropped it,
//! returning `None` for the whole complex when any one chord disagreed —
//! one orbit's class erasing every vertex the tree had placed. These read
//! both halves of what it now returns.

use crate::transducer::alphabet::{Order, Rotor, Word};
use crate::transducer::cech::{solve_gauge, Gauge};
use crate::transducer::element::{identity_versor, same_versor, Versor};

/// The 90° turn on a named plane, as a rotor of order four. Two turns
/// on DIFFERENT planes, never one scaled against another: a rotor told
/// apart by a scalar carries no dimension the next reader can use.
fn turn_90_on(plane: (u8, u8)) -> Rotor {
    let id = crate::transducer::alphabet::LetterId::turn_90(plane).expect("a plane of the chart");
    Word::from_letters(&[(id, false)]).rotor()
}

fn edge(i: usize, j: usize, g: Rotor) -> (usize, usize, Rotor, Versor) {
    (i, j, g, identity_versor())
}

/// A TREE THAT ITS CHORD AGREES WITH IS EXACT: the sections are placed and
/// there is no class. The triangle `0–1–2` with the chord `0–2` carrying
/// exactly what the two tree edges compose to.
#[test]
fn solve_gauge_agreeing_chord_leaves_no_class() {
    let r = turn_90_on((0, 1));
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
/// one orbit's class, and the caller fell through to a reading that never
/// sees the orbit.
#[test]
fn solve_gauge_disagreeing_chord_returns_cocycle() {
    let tree = turn_90_on((0, 1));
    let edges =
        [edge(0, 1, tree.clone()), edge(1, 2, Rotor::identity()), edge(0, 2, Rotor::identity())];
    let Some(Gauge { sections, holonomy }) = solve_gauge(3, &edges) else {
        panic!("a disagreement is a class, not an unreadable gauge");
    };
    assert_eq!(sections.len(), 3, "the tree placed its vertices whatever the chord says");
    assert_eq!(holonomy.len(), 1, "one chord, one orbit, one class");
    let (i, j, class, carriage) = &holonomy[0];
    assert_eq!((*i, *j), (0, 2), "and it names the chord it closed");
    assert!(
        class.projective_eq(&tree),
        "the orbit carries the 90° turn the chord did not: Ω = γ_2 γ_0⁻¹ g⁻¹"
    );
    assert_eq!(class.order(), Some(Order::Finite(4)), "H¹ here is torsion of order four");
    assert!(
        same_versor(carriage, &identity_versor()),
        "and the carriage half of the orbit closes, which the rotor half did not"
    );
}

/// TWO CHORDS ARE TWO ORBITS, AND THE COMPOSITE IS NEITHER. Two chords
/// carrying opposite classes have the identity as their product, so a
/// reader that composed them read "nothing curved" off two real classes
/// and enumerated no occupied axis — 3.9 would have had no candidate move
/// to instantiate. Each `Ω` is its own cocycle.
#[test]
fn solve_gauge_two_chords_return_two_cocycles() {
    let r = turn_90_on((0, 1));
    // A path 0–1–2–3 carrying one 90° turn, closed by two chords. The
    // tree puts `r` on 1, 2 and 3, so the chord 0–2 (carrying nothing)
    // reads `Ω = r` and the chord 1–3 (carrying `r`) reads `Ω = r⁻¹`:
    // two orbits of opposite class on one simplex.
    let edges = [
        edge(0, 1, r.clone()),
        edge(1, 2, Rotor::identity()),
        edge(2, 3, Rotor::identity()),
        edge(0, 2, Rotor::identity()),
        edge(1, 3, r.clone()),
    ];
    let Some(Gauge { holonomy, .. }) = solve_gauge(4, &edges) else {
        panic!("every reading here is invertible");
    };
    assert_eq!(holonomy.len(), 2, "two chords close two independent orbits");
    let composite = holonomy
        .iter()
        .fold(Rotor::identity(), |acc, (_, _, class, _)| acc.geo(class));
    assert!(
        composite.is_identity(),
        "their product is the identity — which is why composing them erased both"
    );
    for (_, _, class, _) in &holonomy {
        assert!(!class.is_identity(), "and neither orbit is flat on its own");
        assert!(
            matches!(class.order(), Some(Order::Finite(_))),
            "each closes at its own period"
        );
    }
}

/// THE ORBITS REACH THE COMPLEX, AT THEIR OWN COORDINATE. A degree-2
/// simplex whose chord disagrees files that chord's `Ω` under
/// `simplex_key ++ vertex_i ++ vertex_j`, so `orbits_of` is a fiber read
/// on the simplex and `classes()` carries what it holds. The simplex's
/// own fill is untouched: the orbits are filed BESIDE it.
#[test]
fn fill_files_disagreeing_chord_orbit_on_complex() {
    use super::common::{turn_90, v, vol};
    use crate::transducer::cech::{Complex, Fill};
    use crate::transducer::element::identity_versor as ident;
    let (a, b, c) = (v(1), v(2), v(3));
    let mut cx = Complex::new();
    for x in [&a, &b, &c] {
        cx.set_section(
            x,
            crate::transducer::cech::Section::at(x.clone(), Rotor::identity()),
            vol(),
        );
    }
    // Two 90°-turn legs and an identity chord: the tree says a 180°
    // turn round the triangle, the chord says nothing, so the orbit stands.
    cx.set_edge(&a, &b, turn_90(), ident(), vol());
    cx.set_edge(&b, &c, turn_90(), ident(), vol());
    cx.set_edge(&a, &c, Rotor::identity(), ident(), vol());
    cx.coboundary_on_link(&a, |_| vol());
    let tri = cx.link(&a, 2);
    let [key] = tri.as_slice() else { panic!("one triangle stands on this link, got {tri:?}") };
    let key = key.clone();
    let filled = cx.fill(&key);
    let orbits = cx.orbits_of(&key);
    assert_eq!(orbits.len(), 1, "one chord, one orbit filed at its own coordinate");
    let [Fill::Congruent(kappa, class, ell)] = orbits.as_slice() else {
        panic!("the orbit closes at a finite period, got {orbits:?}")
    };
    assert_eq!(*ell, 2, "two 90° turns against an identity chord close at two");
    assert!(
        class.projective_eq(&turn_90().geo(&turn_90())),
        "and the filed class is the orbit's own holonomy: {class:?}"
    );
    assert!(
        !kappa.coordinates().is_empty(),
        "held WITH its class vector (11.3), so 3.9 has axes to read candidates off"
    );
    assert!(
        matches!(filled.attached(), Some(Fill::Congruent(..))),
        "and the simplex's own fill still stands beside the orbit, not replaced by it"
    );
}

/// BOTH HALVES, SEPARATELY. A chord whose ROTOR agrees and whose CARRIAGE
/// does not is still a class: the orbit returns to the same orientation at a
/// different place. A reader of the rotor alone called this exact.
#[test]
fn solve_gauge_misplaced_return_is_class() {
    use crate::transducer::cga;
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let walk = cga::at_three::translator(&q(3), &q(0), &q(0));
    let edges = [
        (0usize, 1usize, Rotor::identity(), walk.clone()),
        (1, 2, Rotor::identity(), identity_versor()),
        (0, 2, Rotor::identity(), identity_versor()),
    ];
    let Some(Gauge { holonomy, .. }) = solve_gauge(3, &edges) else {
        panic!("a translator is invertible");
    };
    assert_eq!(holonomy.len(), 1, "the placements disagree round the orbit");
    let (_, _, class, carriage) = &holonomy[0];
    assert!(class.is_identity(), "nothing turned");
    assert!(!same_versor(carriage, &identity_versor()), "and the orbit closes three sites over");
}
