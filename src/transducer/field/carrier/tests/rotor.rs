// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A rotor held by its rate, composed by the law of LAW-COORDINATE fixture
//! 11b (78.173 §5, `forms::turning`), and the composition of turnings on
//! planes that share a unit wherever both motions are rational.

use super::super::*;
use super::fixtures::{frame, plane, three_charts, turning};
use crate::core::Q;
use crate::transducer::cga::Clifford;
use forms::coordinate::{Key, Square};
use forms::turning::{same, TurningRefusal};
use num_bigint::BigInt;

fn q(n: i64, d: i64) -> Q {
    Q::new(BigInt::from(n), BigInt::from(d))
}

/// `c` times the plane's bivector.
fn rate(frame: &FramePlanes, plane: &TurningPlane, c: Q) -> Clifford {
    plane.bivector().product(&Clifford::blade(frame.negatives(), 0, c).expect("a scalar")).expect("one signature")
}

/// The planes a rotor's rate reads, as keys and components.
fn read(r: &Rotor) -> Vec<(Key, Q)> {
    r.planes().expect("every part on a plane")
}

/// 11b-i THROUGH THE CARRIER: `½e₊₁e₊₂` then `⅓e₊₂e₊₃` composes as `R₁R₂`
/// and reads back the rate `(16 e₊₁e₊₂ + 12 e₊₁e₊₃ + 9 e₊₂e₊₃)/37`, its
/// three planes read; the other order turns the other way on `e₊₁e₊₃`.
#[test]
fn rotor_composes_planes_sharing_a_unit_onto_the_plane_between() {
    let f = frame(3);
    let (p12, p13, p23) = (plane(&f, 1, 2), plane(&f, 1, 3), plane(&f, 2, 3));
    let r1 = Rotor::of(rate(&f, &p12, q(1, 2))).expect("a rate");
    let r2 = Rotor::of(rate(&f, &p23, q(1, 3))).expect("a rate");
    let one_way = r1.then(&r2).expect("a composite with a rate");
    assert_eq!(read(&one_way), vec![(p12.key().clone(), q(16, 37)), (p13.key().clone(), q(12, 37)), (p23.key().clone(), q(9, 37))]);
    let other_way = r2.then(&r1).expect("a composite with a rate");
    assert_eq!(read(&other_way), vec![(p12.key().clone(), q(16, 37)), (p13.key().clone(), q(-12, 37)), (p23.key().clone(), q(9, 37))]);
    let r1_motion = r1.motion().expect("a motion");
    assert_eq!(Rotor::of_motion(&r1_motion).expect("its rate"), r1, "the rate read back is the rate held");
}

/// 11b-ii THROUGH THE CARRIER: `½e₊₁e₊₂` then `⅓e₋₁e₋₂` on disjoint planes
/// reads back `4/7` and `3/7`, never `½` and `⅓`: the rotor keeps the
/// composite whole.
#[test]
fn rotor_on_disjoint_planes_reads_back_the_composite_rate() {
    let f = frame(2);
    let p12 = plane(&f, 1, 2);
    let n12 = TurningPlane::of(&f, Square::Negative, 1, 2).expect("the vacuum's plane");
    let r1 = Rotor::of(rate(&f, &p12, q(1, 2))).expect("a rate");
    let r2 = Rotor::of(rate(&f, &n12, q(1, 3))).expect("a rate");
    let composite = r1.then(&r2).expect("a composite with a rate");
    assert_eq!(read(&composite), vec![(p12.key().clone(), q(4, 7)), (n12.key().clone(), q(3, 7))]);
    let product = r1.motion().expect("a motion").product(&r2.motion().expect("a motion")).expect("one signature");
    assert!(same(&composite.motion().expect("a motion"), &product), "R read from the rate read back is R₁R₂");
}

/// A motion with no rate (`R = −1`) is refused by the law's own name.
#[test]
fn rotor_refuses_motion_with_no_rate() {
    let f = frame(2);
    let minus_one = Clifford::blade(f.negatives(), 0, q(-1, 1)).expect("a scalar");
    assert_eq!(Rotor::of_motion(&minus_one), Err(FieldRefusal::Turning(TurningRefusal::NoGenerator)));
}

/// QUARTER TURNS ON PLANES SHARING A UNIT COMPOSE by the law: `90°` on
/// `e_+1+2` then `90°` on `e_+2+3` is `90°` on `e_+1+3`, and the other
/// order `−90°` there; a half turn then a quarter turn is the quarter
/// turn's opposite on its own plane; two half turns are the identity.
#[test]
fn motion_composes_rational_turnings_sharing_a_unit() {
    let f = frame(3);
    let (p12, p13, p23) = (plane(&f, 1, 2), plane(&f, 1, 3), plane(&f, 2, 3));
    let composed = |a: Turning, b: Turning| Motion::of(vec![a, b]).expect("rational motions compose");
    assert_eq!(composed(turning(&p12, 90, 1), turning(&p23, 90, 1)).turnings(), [turning(&p13, 90, 1)]);
    assert_eq!(composed(turning(&p23, 90, 1), turning(&p12, 90, 1)).turnings(), [turning(&p13, -90, 1)]);
    assert_eq!(composed(turning(&p12, 180, 1), turning(&p23, 90, 1)).turnings(), [turning(&p23, -90, 1)]);
    assert_eq!(composed(turning(&p12, -90, 1), turning(&p23, 270, 1)).turnings(), [turning(&p13, 90, 1)], "(−J₁₂)(−J₂₃) = J₁₃");
    assert!(composed(turning(&p12, 180, 1), turning(&p23, 180, 1)).turnings().is_empty(), "(−1)(−1) = 1");
    let reversed = plane(&f, 3, 2);
    assert_eq!(composed(turning(&p12, 90, 1), turning(&reversed, 90, 1)).turnings(), [turning(&p13, -90, 1)], "e₊₃e₊₂ = −e₊₂e₊₃");
}

/// The composite sits beside the held turnings disjoint from both planes:
/// `45°` on `e_+4+5` is kept while `90°` on `e_+1+2` and on `e_+2+3`
/// compose onto `e_+1+3`.
#[test]
fn motion_keeps_disjoint_turnings_beside_the_composite() {
    let f = frame(5);
    let (p12, p13, p23, p45) = (plane(&f, 1, 2), plane(&f, 1, 3), plane(&f, 2, 3), plane(&f, 4, 5));
    let m = Motion::of(vec![turning(&p12, 90, 1), turning(&p45, 45, 1), turning(&p23, 90, 1)]).expect("one pair shares a unit");
    let expected = Motion::of(vec![turning(&p13, 90, 1), turning(&p45, 45, 1)]).expect("disjoint planes");
    assert_eq!(m, expected);
}

/// What the law does not carry here stays refused `Uncommuted` with both
/// planes: a winding on either turning (how `K` is held beside `R` is
/// open, 78.173 §8.3), and a plane sharing a unit with two held planes.
#[test]
fn motion_refuses_wound_turnings_and_three_planes_as_uncommuted() {
    let f = frame(4);
    let (p12, p23, p34) = (plane(&f, 1, 2), plane(&f, 2, 3), plane(&f, 3, 4));
    assert_eq!(
        Motion::of(vec![turning(&p12, 450, 1), turning(&p23, 90, 1)]),
        Err(FieldRefusal::NotExact { residue: Residue::Uncommuted { held: p12.key().clone(), arriving: p23.key().clone() } })
    );
    let disjoint = Motion::of(vec![turning(&p12, 90, 1), turning(&p34, 90, 1)]).expect("disjoint planes");
    let arriving = Motion::of(vec![turning(&p23, 90, 1)]).expect("one turning");
    assert_eq!(
        disjoint.then(&arriving),
        Err(FieldRefusal::NotExact { residue: Residue::Uncommuted { held: p34.key().clone(), arriving: p23.key().clone() } })
    );
}

/// A rate-held [`Rotor`] counts signed revolutions `K` on its invariant plane
/// when `1 + ⟨B₁ B₂⟩₀ ≤ 0` is reached or crossed. At the 180° pole
/// (`1 + ⟨B₁ B₂⟩₀ = 0`), the residual rate is `B = 0` and `K = ±1`; across the
/// pole (`1 + ⟨B₁ B₂⟩₀ < 0`) and back to `R = 1` (`B₁ = 2, B₂ = 3, B₃ = 1`),
/// `LocalOrbit::precession` reads `RotorPrecession::Resonant` with `K = ±1`
/// kept, whereas a flat path (`B = 0, K = 0`) is refused `Unbound`.
#[test]
fn rotor_counts_signed_revolutions_and_local_orbit_reads_resonance() {
    let f = frame(4);
    let (p12, p34) = (plane(&f, 1, 2), plane(&f, 3, 4));
    let canon12 = p12.bivector().multivector().canonical().expect("canonical plane");

    // Two quarter turns (`t = 1` then `t = 1`) land on the pole `1 - 1·1 = 0`:
    // residual B = 0 and K = +1; reversed (`t = -1` then `t = -1`), K = -1.
    let q_pos = Rotor::of(rate(&f, &p12, q(1, 1))).expect("rate +1");
    let q_neg = Rotor::of(rate(&f, &p12, q(-1, 1))).expect("rate -1");
    let pole_pos = q_pos.then(&q_pos).expect("pole reached");
    assert!(pole_pos.is_resonant(), "residual rate B = 0 at the pole");
    assert_eq!(pole_pos.winding_on(&canon12), BigInt::from(1));
    let pole_neg = q_neg.then(&q_neg).expect("negative pole reached");
    assert!(pole_neg.is_resonant(), "residual rate B = 0 at the pole");
    assert_eq!(pole_neg.winding_on(&canon12), BigInt::from(-1));

    // Three rational steps `t = 2, 3, 1` cross `1 - 2·3 = -5 < 0` and close at
    // `R = 1` (`(-1 + 1)/2 = 0`) with signed `K = +1` (and `-1` when negated).
    let (_, charts) = three_charts();
    let [c1, c2, c3] = charts.as_slice() else { panic!("three charts") };
    let r2 = Rotor::of(rate(&f, &p12, q(2, 1))).expect("rate 2");
    let r3 = Rotor::of(rate(&f, &p12, q(3, 1))).expect("rate 3");
    let r1 = Rotor::of(rate(&f, &p12, q(1, 1))).expect("rate 1");
    let orbit = LocalOrbit::of_steps(vec![(c1.clone(), r1), (c2.clone(), r2), (c3.clone(), r3)]).expect("bound revolution");
    match orbit.precession().expect("revolution closes") {
        RotorPrecession::Resonant(h) => assert_eq!(h.winding_on(&canon12), BigInt::from(1)),
        other => panic!("expected Resonant(K = 1), got {other:?}"),
    }

    // A single-line orbit stands attached and refuses revolution readings with
    // `NotRevolution { at: 1 }`; a flat path (`B = 0, K = 0`) refuses `Unbound`.
    let single = LocalOrbit::of_steps(vec![(c1.clone(), q_pos.clone())]).expect("single-line orbit kept");
    assert_eq!(single.steps().len(), 1);
    assert_eq!(single.precession(), Err(FieldRefusal::NotRevolution { at: 1 }));
    let id = Rotor::identity(f.negatives()).expect("identity");
    assert_eq!(
        LocalOrbit::of_steps(vec![(c1.clone(), id.clone()), (c2.clone(), id)]),
        Err(FieldRefusal::Unbound { charts: vec![c1.clone(), c2.clone()] })
    );

    // Rate shear between local orbits on orthogonal planes (`p12` and `p34`)
    // returns the neighbour's rate drag; on the same plane it returns `None`.
    let r34 = Rotor::of(rate(&f, &p34, q(3, 5))).expect("rate 3/5 on p34");
    let ortho = LocalOrbit::of_steps(vec![(c1.clone(), r34.clone()), (c2.clone(), q_pos)]).expect("bound");
    let single_34 = LocalOrbit::of_steps(vec![(c3.clone(), r34.clone())]).expect("single on p34");
    assert_eq!(single.rate_shear(&single_34).expect("shear read"), Some(r34.rate().multivector().grade(2)));
    assert_eq!(single.rate_shear(&orbit).expect("same plane"), None);
    let _ = ortho;
}
