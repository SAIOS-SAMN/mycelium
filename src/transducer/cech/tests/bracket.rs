// 78.26(2), CB1 — the cross-chamber bracket: two chambers meeting at a body
// whose generators fail to commute carry a bracket neither holds; an
// enclosure wrapping both holds it, and without one it is lost.

use crate::transducer::cech::*;
use crate::transducer::cga::{self, Mv, E2, E3};
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// A chamber's orbit algebra over the circuits whose holonomies are given,
/// each its own ring.
fn chamber(holonomies: &[Mv]) -> ChamberAlgebra {
    use crate::network::nerve::coord::{ChartAt, Overlap, VertexAt};
    let circuits: Vec<ChamberCircuit> = holonomies
        .iter()
        .zip(0u64..)
        .map(|(h, i)| {
            let o = Overlap { from: ChartAt::Frame, to: ChartAt::at(VertexAt::Enclosure, crate::transducer::stalk::Invariant::counts(&[i + 1]), 0) };
            ChamberCircuit { ring: ChamberCircuit::ring_of(&[(o, true)]), holonomy: Ok(h.clone()), hue: Some(Vec::new()) }
        })
        .collect();
    read_chamber(1, None, None, &circuits, &[], &mut stroke())
}

/// The 90° turn on the plane (1,2), and on the plane (2,3), at one width.
fn turns() -> (Mv, Mv) {
    let across = cga::rotator_on_blade(&q(1, 1), &q(1, 1), E2 | E3);
    (cga::rotator(&q(1, 1), &q(1, 1)).widened(across.n()), across)
}

fn bodies(names: &[u8]) -> Vec<Vec<u8>> {
    names.iter().map(|n| vec![*n]).collect()
}

/// TWO CHAMBERS MEETING AT A BODY, TURNING ON PLANES THAT SHARE AN AXIS: the
/// two turns fail to commute, and their bracket is read once per pair of
/// actions, exact. Neither chamber's algebra holds it; with no enclosure the
/// bracket is LOST, and a chamber wrapping both bodies whose circuits carry
/// both turns HOLDS it.
#[test]
fn cross_brackets_hold_in_enclosure_and_lose_without() {
    let (a, b) = turns();
    let (ca, cb, wrap) = (chamber(std::slice::from_ref(&a)), chamber(std::slice::from_ref(&b)), chamber(&[a, b]));
    let (xa, xb, xw) = (bodies(&[1, 2]), bodies(&[2, 3]), bodies(&[1, 2, 3]));
    let lone = cross_brackets(&[(&xa, &ca), (&xb, &cb)], &mut stroke());
    let [one] = lone.brackets.as_slice() else { panic!("one bracket: {lone:?}") };
    assert!(!one.bracket.is_zero(), "the turns fail to commute");
    assert_eq!((one.chambers, one.held), ((0, 1), Bracketed::Lost), "no chamber holds it");
    assert_eq!(lone.held_lost(), (0, 1));
    let wrapped = cross_brackets(&[(&xa, &ca), (&xb, &cb), (&xw, &wrap)], &mut stroke());
    let [one] = wrapped.brackets.as_slice() else { panic!("one cross pair: {wrapped:?}") };
    assert_eq!(one.held, Bracketed::Enclosure(2), "the enclosure wrapping both holds it");
    assert_eq!(wrapped.held_lost(), (1, 0));
}

/// GENERATORS THAT COMMUTE CARRY NO BRACKET, AND CHAMBERS SHARING NO BODY ARE
/// NOT PAIRED: two turns on one plane commute, and two chambers on disjoint
/// bodies are not read against each other (MU-CLIENT §4.5).
#[test]
fn cross_brackets_read_nothing_where_generators_commute_or_chambers_part() {
    let (a, b) = turns();
    let half = a.geo(&a);
    let (ca, ch, cb) = (chamber(&[a]), chamber(&[half]), chamber(&[b]));
    let same = cross_brackets(&[(&bodies(&[1, 2]), &ca), (&bodies(&[2, 3]), &ch)], &mut stroke());
    assert!(same.brackets.is_empty() && same.unread.is_empty(), "turns on one plane commute: {same:?}");
    let apart = cross_brackets(&[(&bodies(&[1, 2]), &ca), (&bodies(&[3, 4]), &cb)], &mut stroke());
    assert!(apart.brackets.is_empty() && apart.unread.is_empty(), "chambers sharing no body are not paired");
}

/// A CHAMBER WHOSE ACTIONS DID NOT READ IS NAMED, never read as commuting.
#[test]
fn cross_brackets_name_a_chamber_with_no_actions() {
    let (a, _) = turns();
    let ca = chamber(&[a]);
    let none = read_chamber(1, None, None, &[], &[], &mut stroke());
    let read = cross_brackets(&[(&bodies(&[1, 2]), &ca), (&bodies(&[2, 3]), &none)], &mut stroke());
    assert_eq!(read.unread, vec![CrossUnread::Actions { chamber: 1 }]);
    assert!(read.brackets.is_empty());
}
