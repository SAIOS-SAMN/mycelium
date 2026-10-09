// 78.71 — A TEST PER FLUID AXIOM: 𝒯 as null flux on the conformal sphere,
// the lanes read by a motion's modes, never by a chord.

use crate::transducer::cech::flux::{null_fluxes, TubeRegistry, Tubes};
use num_traits::Zero;
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

/// 78.71 (1), (2) — A TUBE HELD OVER TIME OPENS, THEN CLOSES: the first flux
/// on a ray opens its tube by its mass, standing open as 𝒯's pressure; a
/// second flux on the ray closes on it, relieving what stood open exactly;
/// a finer flux refines the quantum and rescales what the tube held.
#[test]
fn a_held_tube_opens_then_closes_and_relieves_exactly() {
    let f = |x: i64| null_fluxes(&translator(x, 0), &mut stroke()).expect("reads")[0].clone();
    let g = f(1);
    let mut tubes = TubeRegistry::new();
    let first = tubes.admit(&f(1), &g, 4).expect("a ray");
    assert_eq!((first.opened, first.closed, first.relieved, first.transport), (4, 0, 0, 4), "the first flux opens its tube");
    let second = tubes.admit(&f(2), &g, 4).expect("the same ray");
    assert_eq!((second.opened, second.closed, second.relieved, second.transport), (0, 8, 4, 8), "two quanta of four close on it, relieving the four open");
    assert_eq!(tubes.len(), 1, "one ray, one tube");
    let (_, transport, open) = tubes.totals();
    assert_eq!((transport, open), (12.into(), 0.into()), "transported twelve, nothing left open");
    let half = cga::translator_at(&[q(1, 2), q(0, 1), q(0, 1)]);
    let third = tubes.admit(&null_fluxes(&half, &mut stroke()).expect("reads")[0], &g, 1).expect("the same ray");
    assert_eq!(third.closed, 1, "one half-quantum");
    assert_eq!(tubes.totals().1, 25.into(), "the held transport rescaled by the finer quantum: 24 + 1");
}

/// 78.71 (5) — NET IS NOT TRANSPORT: flux there and back on one ray nets to
/// zero and transports both ways. Sign cancellation hides nothing.
#[test]
fn flux_there_and_back_nets_zero_and_transports_both() {
    let f = |x: i64| null_fluxes(&translator(x, 0), &mut stroke()).expect("reads")[0].clone();
    let mut tubes = TubeRegistry::new();
    let g = f(1);
    tubes.admit(&f(1), &g, 3).expect("a ray");
    tubes.admit(&f(-1), &g, 3).expect("the same ray, the other way");
    let (net, transport, open) = tubes.totals();
    assert_eq!((net, transport, open), (0.into(), 6.into(), 0.into()), "net zero, transport six, closed");
}

/// 78.71 (1), (4) — FROZEN IN, HELD: the registry carried by a unit flow
/// keeps its tubes, their windings, and their helicity. Translations do not
/// link (helicity zero); a screw — a turn coupled to a slide along its axis —
/// does.
#[test]
fn a_carried_registry_keeps_its_windings_and_helicity() {
    let mut b = stroke();
    let mut tubes = TubeRegistry::new();
    for x in [1, 2, 3] {
        let line = translator(x, 0);
        let g = crate::transducer::cayley::bivector_of(&line).expect("on the chart");
        for flux in null_fluxes(&line, &mut b).expect("reads") {
            tubes.admit(&flux, &g, 1).expect("a ray");
        }
    }
    assert!(tubes.helicity().is_zero(), "parallel field lines do not link");
    let carried = tubes.carried(&translator(5, 2));
    assert_eq!((carried.len(), carried.totals(), carried.helicity()), (tubes.len(), tubes.totals(), tubes.helicity()), "the flow's invariants");
    let screw = turn().geo(&cga::translator_at(&[q(0, 1), q(0, 1), q(1, 1)]));
    let g = crate::transducer::cayley::bivector_of(&screw).expect("on the chart");
    let mut screwed = TubeRegistry::new();
    for flux in null_fluxes(&screw, &mut b).expect("reads") {
        screwed.admit(&flux, &g, 1).expect("a ray");
    }
    assert_eq!(screwed.len(), 1, "a screw carries one null flux, along its axis");
    assert!(!screwed.helicity().is_zero(), "a screw's field line links itself");
    assert_eq!((tubes.links(), screwed.links()), (0, 0), "self-linking is no reconnection: no two tubes link");
}

/// 78.153b — THE HELICITY ON A STROKE: one read per tube's row; a stroke
/// that reads every row answers the helicity exactly, one short answers
/// `None` — unread, never the partial sum.
#[test]
fn the_helicity_on_a_stroke_is_whole_or_unread() {
    let mut b = stroke();
    let mut tubes = TubeRegistry::new();
    for x in [1, 2, 3] {
        let line = translator(x, 0);
        let g = crate::transducer::cayley::bivector_of(&line).expect("on the chart");
        for flux in null_fluxes(&line, &mut b).expect("reads") {
            tubes.admit(&flux, &g, 1).expect("a ray");
        }
    }
    let rows = tubes.len();
    assert!(rows > 0, "a tube to pair with itself");
    let mut whole = crate::physics::Physics::default().stroke_n(rows);
    assert_eq!(tubes.helicity_on(&mut whole), Some(tubes.helicity()), "every row read: the helicity exactly");
    let mut short = crate::physics::Physics::default().stroke_n(rows - 1);
    assert_eq!(tubes.helicity_on(&mut short), None, "a row unread: the helicity unread");
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
