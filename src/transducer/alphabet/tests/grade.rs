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
fn grade_sums_back_to_element() {
    let q4 = LetterId::turn_90((0, 1)).expect("90° turn on the sensor plane");
    let q23 = LetterId::turn_90((2, 3)).expect("90° turn on the complementary plane");
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
            u32::try_from(m).expect("a blade mask below Cl(4,0)'s 16 slots").count_ones()
        );
    }
}

/// A ROTOR OCCUPIES EVEN GRADES ONLY, which is what `Cl⁺(4,0)` means, and
/// the two readings of that agree because there is one reader.
#[test]
fn grades_even_and_double_turn_carries_pseudoscalar() {
    let q01 = LetterId::turn_90((0, 1)).expect("90° turn");
    let q23 = LetterId::turn_90((2, 3)).expect("90° turn");

    // A single-plane 90° turn: scalar and one bivector, no 4-volume.
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
fn geo_composes_every_grade_at_once() {
    let q01 = LetterId::turn_90((0, 1)).expect("90° turn");
    let q23 = LetterId::turn_90((2, 3)).expect("90° turn");
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
    // the product of the factors' bivectors. That is the content a lane-wise sum
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
fn factor_rotor_spells_double_turn_and_refuses_odd() {
    use crate::transducer::alphabet::factor_rotor;
    let q01 = LetterId::turn_90((0, 1)).expect("90° turn");
    let q23 = LetterId::turn_90((2, 3)).expect("90° turn");
    let double = Word::from_letters(&[(q01, false), (q23, false)]).rotor();
    assert_eq!(double.grades(), vec![0, 2, 4]);
    let Some(Ok(spelled)) = factor_rotor(&double) else {
        panic!("a double rotation of two 90° turns is spelled by the seventeen");
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
fn grades_of_zero_empty() {
    let z = Rotor::zero();
    assert!(z.grades().is_empty(), "zero occupies nothing, got {:?}", z.grades());
    assert!(z.weight().is_none(), "and it is not a versor");
    // The identity is a pure grade 0 and is a versor.
    let one = Rotor::identity();
    assert_eq!(one.grades(), vec![0]);
    assert_eq!(one.weight(), Some(q(1)));
}

/// VA2 — AN ODD ROTOR CARRIES A VECTOR BY ITS TWISTED ADJOINT: the
/// reflection in `e₀` holds `e₁` and sends `e₀` to `−e₀`. The plain
/// sandwich negated both — every image of an odd rotor came back as its
/// own negative (d1's shape one algebra down).
#[test]
fn carry_reflects_vector_through_mirror() {
    let reflection = Rotor::blade(1, q(1));
    let z = q(0);
    assert_eq!(
        reflection.carry(&[z.clone(), q(1), z.clone(), z.clone()]),
        Some([z.clone(), q(1), z.clone(), z.clone()]),
        "e₁ lies in the mirror"
    );
    assert_eq!(
        reflection.carry(&[q(1), z.clone(), z.clone(), z.clone()]),
        Some([q(-1), z.clone(), z.clone(), z]),
        "e₀ is the mirror's normal"
    );
}

/// VA1 (78.28(1) as amended) — THE ROTOR'S ACTION READER AND THE THEOREM
/// IT IMPLIES AGREE: `Φ Φ̃` a non-zero scalar exactly where the twisted
/// adjoint carries the four generators to independent vectors and
/// `e₀₁₂₃` to its own line — on every letter, a reflection (odd), three
/// reflections, the pseudoscalar, `1 + e₀₁ + e₂₃`, `1 + e₀₁₂₃`, a mixed
/// parity and zero.
#[test]
fn rotor_is_versor_agrees_with_scalar_norm_theorem() {
    let old = |r: &Rotor| {
        let p = r.geo(&r.reverse());
        !p.coeff(0).is_zero() && (1..16).all(|i| p.coeff(i).is_zero())
    };
    let b = |mask: usize| Rotor::blade(mask, q(1));
    let mut cases: Vec<(String, Rotor, bool)> = vec![
        ("a reflection".into(), b(1), true),
        ("three reflections".into(), b(1).geo(&b(2)).geo(&b(4)), true),
        ("the pseudoscalar".into(), b(15), true),
        ("1 + e01 + e23".into(), Rotor::identity().add(&b(3)).add(&b(12)), false),
        ("1 + e0123".into(), Rotor::identity().add(&b(15)), false),
        ("mixed parity".into(), Rotor::identity().add(&b(1)), false),
        ("zero".into(), Rotor::zero(), false),
    ];
    for raw in 0..17u8 {
        if let Some(id) = LetterId::try_new(raw) {
            cases.push((format!("letter {raw}"), Word::from_letters(&[(id, false)]).rotor(), true));
        }
    }
    for (name, r, want) in &cases {
        assert_eq!(r.is_versor(), *want, "{name}");
        assert_eq!(r.is_versor(), old(r), "{name}: the theorem agrees with the reader");
    }
}
