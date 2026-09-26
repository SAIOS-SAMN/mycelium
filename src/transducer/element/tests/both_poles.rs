// A.13 — A CONFORMAL VERSOR READ AT BOTH POLES: `V = T(a)·K(b)·D(ρ)·R·Iᵉ`,
// the transversion `K(b) = I·T(b)·I` read through the chart's inversion.
//
//! Proved by the algebra (T1, fixed loci, refusal by name): each factor is
//! read back exactly from a versor built of known factors, the product
//! composes back to the versor as one ray, the inversion exchanges the two
//! poles, and a motion that moves the point at infinity — which the `∞`-pole
//! readers never read — now yields its rotation.

use crate::core::Q;
use crate::transducer::cga::{self, dilator, rotator, translator_at};
use crate::transducer::element::*;
use num_bigint::BigInt;

fn q(n: i64, d: i64) -> Q {
    Q::new(BigInt::from(n), BigInt::from(d))
}

/// `K(b) = I · T(b) · I`.
fn transversor(b: &[Q]) -> Versor {
    let i = inversion();
    i.geo(&translator_at(b)).geo(&i)
}

/// The inversion is its own inverse and exchanges the origin with infinity:
/// the two poles the two halves of the conformal pair fix.
#[test]
fn the_inversion_exchanges_the_two_poles_and_is_its_own_inverse() {
    let i = inversion();
    assert_eq!(cga::projective_ratio(&i.geo(&i), &identity_versor()), Some(Q::from_integer(BigInt::from(1))), "I² = 1 exactly");
    assert!(i.sandwich(&cga::point_at(&[])).projective_eq(&cga::e_inf()), "I(o) = ∞");
    assert!(i.sandwich(&cga::e_inf()).projective_eq(&cga::point_at(&[])), "I(∞) = o");
    // The transversion fixes the origin and moves infinity — the translator's
    // mirror image across the inversion.
    let k = transversor(&[q(1, 3), q(-2, 1), q(0, 1)]);
    assert!(k.sandwich(&cga::point_at(&[])).projective_eq(&cga::point_at(&[])), "K(b) fixes the origin");
    assert!(!k.sandwich(&cga::e_inf()).projective_eq(&cga::e_inf()), "and moves infinity");
}

/// EVERY FACTOR READ EXACTLY, AND THE PRODUCT COMPOSES BACK (T1). A versor
/// built as `T(a)·K(b)·D(ρ)·R` is read into exactly those coordinates; the
/// `∞`-pole reader has no reading of it at all, and the rotor it carries is
/// the rotation's own.
#[test]
fn every_factor_is_read_exactly_and_composes_back() {
    let a = vec![q(3, 1), q(-2, 1), q(1, 2)];
    let b = vec![q(1, 5), q(0, 1), q(-1, 3)];
    let rho = q(3, 2);
    let r = rotator(&q(3, 1), &q(4, 1));
    let v = translator_at(&a).geo(&transversor(&b)).geo(&dilator(&rho)).geo(&r);
    let f = conformal_factors(&v).expect("a conformal versor reads at both poles");
    assert_eq!(f.translation, a, "a = V(o)");
    assert_eq!(f.transversion, b, "b read through the inversion");
    assert_eq!(f.rho, rho, "ρ read once both poles are held");
    assert!(!f.inverted, "the origin stays finite");
    assert!(f.transverses());
    assert_eq!(f.rotation, r, "R as a ray");
    assert_eq!(f.composed(), v, "T1: the factors compose back to the versor");
    assert_eq!(rho_of(&v), None, "the ∞-pole reader has no reading of a motion that moves infinity");
    assert_eq!(base_rotor_of(&v), base_rotor_of(&r), "and the rotor is the rotation's own");
}

/// WHERE THE VERSOR FIXES INFINITY NOTHING MOVES: a similarity `D(ρ)·T(t)·R`
/// reads `b = 0`, `a = ρt`, and the same rotor ray `(D·T)⁻¹ V` always read.
#[test]
fn a_similarity_reads_as_it_always_did() {
    let t = vec![q(2, 1), q(5, 1), q(-1, 1)];
    let rho = q(2, 1);
    let r = rotator(&q(1, 1), &q(1, 1));
    let v = carriage_at(&rho, &t).geo(&r);
    let f = conformal_factors(&v).expect("a similarity reads");
    assert!(!f.transverses() && !f.inverted, "no origin-pole half, no inversion");
    assert_eq!(f.translation, t.iter().map(|x| x * &rho).collect::<Vec<_>>(), "a = ρt: T(ρt)·D(ρ) = D(ρ)·T(t)");
    let old = dilator_of(&v).expect("ρ").geo(&translator_of(&v).expect("t")).reverse().geo(&v);
    assert_eq!(cga::projective_ratio(&f.rotation, &old), Some(Q::from_integer(BigInt::from(1))), "the rotation is the same ray, coefficient for coefficient");
}

/// A HOLONOMY OF TRANSVECTIONS MOVES INFINITY — the connection's own motion
/// (54.7 R3), `eᵢ ∧ e₋ = ½ eᵢ ∧ e_∞ + eᵢ ∧ e_o`, both poles at once — and
/// the peel read nothing from it, so every such refusal was unclassified. It
/// now reads its rotation and its transversion.
#[test]
fn a_transvection_holonomy_reads_its_rotation() {
    let placed = translator_at(&[q(1, 3), q(1, 4), q(0, 1)]);
    let boost = transvection_of(&placed).expect("a finite transvection");
    let h = boost.geo(&rotator(&q(3, 1), &q(4, 1)));
    assert_eq!(rho_of(&h), None, "it moves infinity");
    let f = conformal_factors(&h).expect("read at both poles");
    assert!(f.transverses(), "a transvection carries the origin pole's half");
    assert_eq!(f.composed(), h, "T1");
    assert_eq!(base_rotor_of(&h), base_rotor_of(&rotator(&q(3, 1), &q(4, 1))), "the turn it carries");
    let kind = crate::transducer::sentence::read_obstruction(crate::transducer::sentence::Content::Holonomy {
        versor: Some(&h),
        chart: None,
        proof: "a transvection holonomy",
    });
    assert_ne!(kind.kind(), crate::transducer::sentence::ObstructionKind::Unclassified, "its kind is read, never unclassified");
}

/// THE INVERSION'S CELL: a versor carrying the origin to infinity reads
/// `ε = 1` with the rest of it factored, composes back, has no base rotor,
/// and its kind is D — a versor no rotor spells.
#[test]
fn the_inversions_cell_reads_and_its_kind_is_the_letter_demand() {
    let v = translator_at(&[q(2, 1), q(0, 1), q(1, 1)]).geo(&inversion());
    let f = conformal_factors(&v).expect("read in the inversion's cell");
    assert!(f.inverted, "V(o) = ∞");
    assert_eq!(f.composed(), v, "T1");
    assert_eq!(base_rotor_of(&v), Err(RotorUnread::Inverted), "the inversion is no rotation of the base");
    let kind = crate::transducer::sentence::read_obstruction(crate::transducer::sentence::Content::Holonomy {
        versor: Some(&v),
        chart: None,
        proof: "an inverting holonomy",
    });
    assert_eq!(kind.kind(), crate::transducer::sentence::ObstructionKind::D, "the letter the inversion is");
}

/// What is no conformal motion is refused by name.
#[test]
fn a_non_versor_is_refused_by_name() {
    let not = identity_versor().add(&cga::e_inf());
    assert_eq!(conformal_factors(&not), Err(Unfactored::NotVersor));
}
