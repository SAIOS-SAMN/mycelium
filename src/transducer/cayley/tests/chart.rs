// 52.1, 52.2 — THE CHART: read in either direction, adding on one plane and
// not across two, refused at its boundary.

use crate::transducer::blade::{shape_of, versor_between};
use crate::transducer::cayley::{bivector_of, versor_of_bivector};
use crate::transducer::cga::{self, Mv, E1, E2, E3, EP};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

/// `E = e_o ∧ e_∞`, the dilation plane, written as `cga::dilator` writes it.
fn dilation_plane() -> Mv {
    cga::e_o().geo(&cga::e_inf()).add(&Mv::scalar(q(1, 1)))
}

/// The coefficient of a bivector on the dilation plane, when it lies on
/// that plane alone.
fn rapidity_of(b: &Mv) -> Option<BigRational> {
    cga::projective_ratio(b, &dilation_plane())
}

/// Every versor the tree writes — a dilation, a translation with a frame
/// component, the 90° turn, the (3,4,5) rotor, a turn onto the frame
/// axis, and the composite of all of these, which carries a grade-4 part — reads its
/// bivector by grade and is read back from it projectively; the bivector
/// read back off the derived versor is coefficient-exact.
#[test]
fn bivector_of_round_trips_through_versor_of_bivector() {
    let (z, o) = (q(0, 1), q(1, 1));
    let d = cga::dilator(&q(2, 1));
    let t = cga::at_three::translator(&o, &q(-2, 1), &q(3, 1));
    let turn_90 =cga::rotator(&o, &o);
    let pythagorean = cga::rotator(&q(2, 1), &o);
    let onto_frame = cga::rotator_on_blade(&o, &o, E1 | E3);
    let composite = d.geo(&t).geo(&turn_90);
    assert!(!composite.grade(4).is_zero(), "the composite carries a grade-4 part the chart must account for");
    for v in [&d, &t, &turn_90, &pythagorean, &onto_frame, &composite] {
        let b = bivector_of(v).expect("an even versor with a scalar part is in the chart");
        assert!(b.grade(2).sub(&b).is_zero(), "the derivation is a bivector");
        let back = versor_of_bivector(&b).expect("a bivector derives a versor");
        assert!(back.projective_eq(v), "the versor re-derives from its bivector: {v:?}");
        let again = bivector_of(&back).expect("the derived versor is in the chart");
        assert!(again.sub(&b).is_zero(), "and reads the same bivector, coefficient for coefficient");
        assert!(bivector_of(&v.scale(&q(-7, 3))).expect("projective").sub(&b).is_zero(), "λV reads one bivector");
    }
    let b = bivector_of(&turn_90).expect("the 90° turn");
    assert!(b.sub(&Mv::blade(E1 | E2, o.clone())).is_zero(), "the 90° turn reads 1 on its plane");
    let b = bivector_of(&pythagorean).expect("the (3,4,5) rotor");
    assert!(b.sub(&Mv::blade(E1 | E2, q(1, 2))).is_zero(), "the (3,4,5) rotor reads 1/2 on its plane");
    let b = bivector_of(&t).expect("a translator");
    assert!(outer_is_zero(&b), "a translator's bivector is nilpotent");
    let _ = z;
}

fn outer_is_zero(b: &Mv) -> bool {
    b.geo(b).grade(4).is_zero()
}

/// The dilator by ρ reads β = (ρ − 1)/(ρ + 1) on the dilation plane — one
/// third for the dilation by two — and two dilations compose by the
/// rational addition law of that plane, read off the geometric product.
#[test]
fn bivector_of_dilators_adds_by_rational_law() {
    let two = cga::dilator(&q(2, 1));
    let three = cga::dilator(&q(3, 1));
    let six = cga::dilator(&q(6, 1));
    assert!(two.geo(&three).projective_eq(&six), "the dilations compose");
    let b2 = rapidity_of(&bivector_of(&two).expect("in the chart")).expect("on the dilation plane");
    let b3 = rapidity_of(&bivector_of(&three).expect("in the chart")).expect("on the dilation plane");
    let b6 = rapidity_of(&bivector_of(&six).expect("in the chart")).expect("on the dilation plane");
    assert_eq!(&b2 * &b2, q(1, 9), "the dilation by two is the rapidity one third");
    assert_eq!(&b3 * &b3, q(1, 4), "the dilation by three is one half");
    assert_eq!(b6, (&b2 + &b3) / (q(1, 1) + &b2 * &b3), "β₃ = (β₁ + β₂)/(1 + β₁β₂)");
    assert_eq!(&b6 * &b6, q(25, 49), "the dilation by six is five sevenths");
}

/// The chart's boundary: a 180° turn has no scalar part, a reflection is
/// odd and is its own derivation, and the zero element is nothing.
#[test]
fn bivector_of_refuses_turn_180_and_reflection() {
    let o = q(1, 1);
    let turn_90 =cga::rotator(&o, &o);
    let turn_180 = turn_90.geo(&turn_90);
    assert!(turn_180.scalar_part().is_zero(), "a 180° turn is a pure bivector");
    assert_eq!(bivector_of(&turn_180), None, "and has no bivector in the chart");
    assert_eq!(bivector_of(&cga::reflector()), None, "a reflection is its mirror, not a bivector");
    assert_eq!(bivector_of(&Mv::zero()), None);
}

/// An even element whose grade-4 part is not `⟨V⟩₂∧⟨V⟩₂ / (2⟨V⟩₀)` is not a
/// versor and the chart refuses it rather than reading a bivector off it;
/// a bivector on the light cone derives an element with no weight, which is
/// refused the same way; a non-bivector is refused as a derivation.
#[test]
fn bivector_of_refuses_even_non_versor() {
    let o = q(1, 1);
    let b = Mv::blade(E1 | E2, o.clone()).add(&Mv::blade(E3 | EP, o.clone()));
    let quad = b.geo(&b).grade(4);
    assert!(!quad.is_zero(), "two orthogonal planes have an outer square");
    let versor = Mv::scalar(o.clone()).add(&b).add(&quad.scale(&q(1, 2)));
    assert!(bivector_of(&versor).is_some(), "with half the outer square it is a versor");
    let not_versor = Mv::scalar(o.clone()).add(&b).add(&quad.scale(&q(2, 1)));
    assert!(!cga::is_versor(&not_versor), "with twice it, the action leaves the vectors");
    assert_eq!(bivector_of(&not_versor), None, "and the chart refuses it");
    let cone = dilation_plane();
    assert_eq!(versor_of_bivector(&cone), None, "β = 1 is the light cone: no weight, no versor");
    assert_eq!(versor_of_bivector(&Mv::blade(E1, o)), None, "a vector is not a derivation in the chart");
}

/// On one plane the parameters add by the rational law — the (3,4,5) and
/// (5,12,13) rotors compose to the 90° turn, `(1/2 + 1/3)/(1 − 1/6) =
/// 1` — and across two planes that share an axis they do not: the product
/// is the reading and the sum of the bivectors derives another versor.
#[test]
fn bivector_of_adds_product_on_one_plane_only() {
    let o = q(1, 1);
    let a = cga::rotator(&q(2, 1), &o);
    let b = cga::rotator(&q(3, 1), &o);
    let product = a.geo(&b);
    let expected = Mv::blade(E1 | E2, o.clone());
    assert!(bivector_of(&product).expect("one plane").sub(&expected).is_zero(), "on one plane the parameters add to 1");
    assert!(product.projective_eq(&cga::rotator(&o, &o)), "which is the 90° turn");
    let p = cga::rotator_on_blade(&o, &o, E1 | E2);
    let r = cga::rotator_on_blade(&o, &o, E1 | E3);
    let across = p.geo(&r);
    let sum = bivector_of(&p).expect("in the chart").add(&bivector_of(&r).expect("in the chart"));
    let read = bivector_of(&across).expect("the product is in the chart");
    assert!(!read.sub(&sum).is_zero(), "across two planes the bivectors do not add");
    let derived = versor_of_bivector(&sum).expect("the sum derives a versor of its own");
    assert!(!derived.projective_eq(&across), "and it is not the product");
}

/// The quotient between two shapes is in the chart: a translated square
/// reads a nilpotent bivector and a body carried onto its dilated block
/// reads one with a dilation component; each re-derives its versor.
#[test]
fn bivector_of_reads_versor_between_shapes() {
    let square = shape_of(&[(0, 0, 0), (0, 1, 0), (1, 0, 0), (1, 1, 0)]);
    let moved = shape_of(&[(2, 3, 0), (2, 4, 0), (3, 3, 0), (3, 4, 0)]);
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let v = versor_between(&square, &moved, &mut budget).expect("a translation carries the square");
    let b = bivector_of(&v).expect("in the chart");
    assert!(outer_is_zero(&b), "a translation's bivector is nilpotent");
    assert!(versor_of_bivector(&b).expect("derives").projective_eq(&v));
    let one = shape_of(&[(1, 2, 0)]);
    let block = shape_of(&[(2, 4, 0), (2, 5, 0), (3, 4, 0), (3, 5, 0)]);
    let w = versor_between(&block, &one, &mut budget).expect("a dilation carries the block onto one compartment");
    let c = bivector_of(&w).expect("in the chart");
    assert!(!c.geo(&dilation_plane()).scalar_part().is_zero(), "the bivector has a dilation component");
    assert!(versor_of_bivector(&c).expect("derives").projective_eq(&w));
}
