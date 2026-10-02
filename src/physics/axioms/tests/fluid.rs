// 78.71 — A TEST PER FLUID AXIOM: 𝒯 as null flux on the conformal sphere,
// the lanes read by a motion's modes, never by a chord.

use crate::transducer::cech::flux::{null_fluxes, Tubes};
use crate::transducer::cga::{self, Mv, E1, E2};
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

fn translator(x: i64, y: i64) -> Mv {
    cga::translator_at(&[q(x, 1), q(y, 1), q(0, 1)])
}

fn turn() -> Mv {
    Mv::scalar(q(1, 1)).add(&Mv::blade(E1 | E2, q(1, 1)))
}

/// F1, F2 — A TRANSLATION'S HOLONOMY IS NULL FLUX; A TURN'S IS NONE. A cell
/// whose boundary composes to a translation carries one null flux, its
/// generator squaring to zero; one whose boundary composes to a turn carries
/// none on 𝒯 (its curvature is 𝒽's).
#[test]
fn a_translation_is_null_flux_and_a_turn_is_none() {
    let flux = null_fluxes(&translator(1, 0), &mut stroke()).expect("the modes read");
    assert_eq!(flux.len(), 1, "one null mode");
    assert!(flux[0].geo(&flux[0]).is_zero(), "its generator squares to zero: no length");
    assert!(null_fluxes(&turn(), &mut stroke()).expect("the modes read").is_empty(), "a turn is not 𝒯");
}

/// F4 — THE QUANTUM IS READ OFF THE TUBE: fluxes two and three units along
/// one ray share the quantum of one unit and wind it twice and three times;
/// a half-unit flux refines the quantum to a half and the windings double.
#[test]
fn a_tubes_quantum_is_the_generator_of_its_fluxes() {
    let f = |x: i64| null_fluxes(&translator(x, 0), &mut stroke()).expect("reads")[0].clone();
    let mut tubes = Tubes::new();
    assert!(tubes.admit(&f(2), 1) && tubes.admit(&f(3), 1));
    let [(qnt, w)] = tubes.windings().try_into().expect("one tube: one ray");
    let ms: Vec<i64> = w.iter().map(|(m, _)| i64::try_from(m.clone()).expect("small")).collect();
    assert_eq!(ms.iter().map(|m| m.abs()).collect::<Vec<_>>(), vec![2, 3], "quantum {qnt}: two and three windings");
    let half = cga::translator_at(&[q(1, 2), q(0, 1), q(0, 1)]);
    assert!(tubes.admit(&null_fluxes(&half, &mut stroke()).expect("reads")[0], 1));
    let [(_, w)] = tubes.windings().try_into().expect("still one tube");
    let ms: Vec<i64> = w.iter().map(|(m, _)| i64::try_from(m.clone()).expect("small").abs()).collect();
    assert_eq!(ms, vec![4, 6, 1], "the finer flux refines the quantum; nothing is refused");
    assert_eq!(tubes.measure(), Some(11), "mass flux in quanta: 4 + 6 + 1");
}

/// F3 — FROZEN IN: carrying a tube by a turn keeps it one tube with the same
/// windings.
#[test]
fn a_tube_carried_by_the_flow_keeps_its_windings() {
    let f = |x: i64| null_fluxes(&translator(x, 0), &mut stroke()).expect("reads")[0].clone();
    let g = turn();
    let carry = |v: &Mv| g.geo(v).geo(&g.reverse());
    let (mut here, mut there) = (Tubes::new(), Tubes::new());
    for x in [2, 3] {
        here.admit(&f(x), 1);
        there.admit(&carry(&f(x)), 1);
    }
    assert_eq!((here.len(), there.len()), (1, 1), "one tube before and after the flow");
    assert_eq!(here.measure(), there.measure(), "the windings are the flow's invariants");
}

/// F5 — NO LATTICE ON THE SPHERE: a turn about a centre away from the origin
/// moves the origin along a chord, and is still a turn — 𝒽, never 𝒯. A
/// pure translation is 𝒯.
#[test]
fn a_turn_about_another_centre_is_not_a_translation() {
    use crate::transducer::family::pairing::carriage_lanes;
    let t = translator(3, 1);
    let off_centre = t.geo(&turn()).geo(&t.reverse());
    let mut b = stroke();
    assert_eq!(carriage_lanes(&off_centre, &mut b), [false, false, false, true], "a turn, wherever it is centred");
    assert_eq!(carriage_lanes(&t, &mut b), [false, false, true, false], "a translation is null: 𝒯");
}
