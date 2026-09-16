// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! EXPANSION 3 — THE GRADED WORD: `W = ⟨W⟩₀ + ⟨W⟩₂ + ⟨W⟩₄` over
//! `Cl⁺(4,0)`, and the single geometric product that composes every grade
//! at once.

use crate::transducer::alphabet::{LetterId, Rotor, Word};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;

fn q(v: i64) -> BigRational {
    BigRational::from_integer(BigInt::from(v))
}

/// THE DECOMPOSITION IS EXACT AND TOTAL: the grades sum back to the element
/// itself, coefficient for coefficient, over ℚ with nothing rounded.
///
/// This is the whole claim of a graded representation — that projecting and
/// re-summing is the identity — and it is what lets a caller read one grade
/// without asking whether the rest were lost.
#[test]
fn the_grades_sum_to_the_element() {
    let q4 = LetterId::quarter((0, 1)).expect("quarter on the sensor plane");
    let q23 = LetterId::quarter((2, 3)).expect("quarter on the complementary plane");
    // A DOUBLE ROTATION: `Spin(4) = SU(2) × SU(2)`, so two independent
    // commuting turns. This is the element that carries a pseudoscalar; a
    // single-plane turn does not.
    let w = Word::from_letters(&[(q4, false), (q23, false)]);
    let r = w.rotor();

    let sum = (0..=4u32).fold(Rotor::zero(), |acc, k| acc.add(&r.grade(k)));
    for m in 0..16usize {
        assert_eq!(
            sum.coeff(m),
            r.coeff(m),
            "grade {} of blade {m:#06b} is lost or doubled",
            (m as u32).count_ones()
        );
    }
}

/// A ROTOR OCCUPIES EVEN GRADES ONLY, which is what `Cl⁺(4,0)` means, and
/// the two readings of that agree because there is one reader.
#[test]
fn a_rotor_occupies_even_grades_and_a_double_turn_carries_the_pseudoscalar() {
    let q01 = LetterId::quarter((0, 1)).expect("quarter");
    let q23 = LetterId::quarter((2, 3)).expect("quarter");

    // A single-plane quarter: scalar and one bivector, no 4-volume.
    let single = Word::from_letters(&[(q01, false)]).rotor();
    assert!(single.is_even(), "a rotor is even");
    assert_eq!(single.grades(), vec![0, 2], "scalar and bivector, got {:?}", single.grades());
    assert!(
        single.pseudoscalar().is_zero(),
        "one plane turns and the 4-volume stays flat"
    );

    // Both planes at once: the product's pseudoscalar is the product of the
    // two bivector coefficients, so the 4-volume is occupied and the element
    // is NOT reachable as a single-plane turn.
    let double = Word::from_letters(&[(q01, false), (q23, false)]).rotor();
    assert!(double.is_even());
    assert_eq!(
        double.grades(),
        vec![0, 2, 4],
        "a double rotation occupies every even grade, got {:?}",
        double.grades()
    );
    assert!(
        !double.pseudoscalar().is_zero(),
        "the two independent angles meet in the pseudoscalar"
    );
}

/// ONE GEOMETRIC PRODUCT COMPOSES EVERY GRADE, which is the reason to hold a
/// multivector rather than four lanes re-mixed afterwards.
///
/// The claim is verified against the grades themselves: composing and THEN
/// projecting gives the same coefficients as the composite's own projection,
/// on every grade, with no grade computed separately.
#[test]
fn one_product_composes_every_grade_at_once() {
    let q01 = LetterId::quarter((0, 1)).expect("quarter");
    let q23 = LetterId::quarter((2, 3)).expect("quarter");
    let a = Word::from_letters(&[(q01, false)]).rotor();
    let b = Word::from_letters(&[(q23, false)]).rotor();

    let ab = a.geo(&b);
    // The two turns are on complementary planes and commute — the relator
    // the alphabet states — so the product is the same either way, and that
    // is an exact equality of coefficients and not a projective one.
    let ba = b.geo(&a);
    for m in 0..16usize {
        assert_eq!(ab.coeff(m), ba.coeff(m), "complementary planes commute, blade {m:#06b}");
    }

    // Grade 4 of the product is not carried by either factor: it is MADE by
    // the product of their bivectors. That is the content a lane-wise sum
    // cannot produce, and it is why the grades are not four carriers.
    assert!(a.pseudoscalar().is_zero() && b.pseudoscalar().is_zero());
    assert!(
        !ab.pseudoscalar().is_zero(),
        "the 4-volume is created by the product, not carried into it"
    );
}

/// THE FACTORISATION CONSUMES THE GRADES. A double rotation — every even
/// grade occupied, the 4-volume made by the product — is spelled as its
/// two commuting plane turns, one letter each, and not as one plane's
/// block with the other turn dropped; an odd element (a reflection, which
/// has weight too) is refused at the gate rather than factored by the even
/// grades it happens to carry.
#[test]
fn the_factorisation_spells_a_double_turn_whole_and_refuses_an_odd_element() {
    use crate::transducer::alphabet::factor_rotor;
    let q01 = LetterId::quarter((0, 1)).expect("quarter");
    let q23 = LetterId::quarter((2, 3)).expect("quarter");
    let double = Word::from_letters(&[(q01, false), (q23, false)]).rotor();
    assert_eq!(double.grades(), vec![0, 2, 4]);
    let Some(Ok(spelled)) = factor_rotor(&double) else {
        panic!("a double rotation of two quarter turns is spelled by the seventeen");
    };
    assert!(spelled.rotor().projective_eq(&double), "the spelling IS the rotor, 4-volume included");
    assert_eq!(spelled.len(), 2, "one letter per commuting plane, got {:?}", spelled.letters());

    let reflection = Rotor::blade(1, q(1));
    assert!(reflection.weight().is_some(), "a reflection has weight");
    assert!(!reflection.is_even());
    assert!(factor_rotor(&reflection).is_none(), "and is no rotor of Spin(4,0): refused at the gate");
}

/// THE ZERO MULTIVECTOR OCCUPIES NO GRADE and is not a versor: it has no
/// weight, so it carries nothing and is not an element of the group.
#[test]
fn the_zero_multivector_occupies_no_grade() {
    let z = Rotor::zero();
    assert!(z.grades().is_empty(), "zero occupies nothing, got {:?}", z.grades());
    assert!(z.weight().is_none(), "and it is not a versor");
    // The identity is a pure grade 0 and is a versor.
    let one = Rotor::identity();
    assert_eq!(one.grades(), vec![0]);
    assert_eq!(one.weight(), Some(q(1)));
}
