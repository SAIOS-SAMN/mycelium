// 17.1, 17.2 — M1 and M2: when two elements are one vertex, and what the
// address of a vertex is derived from.

use crate::core::Q;
use crate::transducer::element::*;
use num_bigint::BigInt;
use crate::transducer::germinate::germinate;
use super::common::*;

/// A COMPOSITE CARRIES ITS ROTOR, AND A TRANSITION ANSWERS ONE.
///
/// `letters()` gates on the description's tag and `cech::rotor_of` reads
/// the tape and nothing else, so a composite that left its description
/// empty would answer no rotor, the court would have none to put on a
/// simplex, and `Complex::fill` would never reach `classify` — the only
/// producer of `Fill::Class` and `Fill::Unspellable`.
///
/// The rotor composes in `Cl(4,0)`, six planes. This is the case that says
/// so: a 90° turn on the sensor and a 90° turn on `(2, 3)` have the SAME
/// sensor action — the second is the identity there — so the `Cl(3,1)`
/// product cannot tell them apart, and the composed spelling can.
#[test]
fn compose_carries_rotor_and_transition_answers_one() {
    use crate::transducer::alphabet::{LetterId, Word};
    let q01 = Word::from_letters(&[(LetterId::turn_90((0, 1)).expect("the sensor plane"), false)]);
    let q13 = Word::from_letters(&[(LetterId::turn_90((1, 3)).expect("the (1,3) base plane"), false)]);
    let q23 = Word::from_letters(&[(LetterId::turn_90((2, 3)).expect("the far plane"), false)]);

    // A word turning the base off itself has no base action and cannot be a
    // connection on the base manifold (51.2, A4).
    assert!(
        residual::from_connection(&q23, &identity_versor(), &[], &Vec::new(), [0; 4]).is_none(),
        "a (2,3) rotation turns e₂ into the base and has no base connection"
    );

    let conn = |w: &Word| {
        residual::from_connection(w, &identity_versor(), &[], &Vec::new(), [0; 4])
            .expect("a connection")
    };
    let (a, b) = (conn(&q01), conn(&q13));

    // The (1,3) 90° turn's action on the base rotates e₂ ∧ e₃: non-identity in Cl(4,1).
    assert!(
        !same_versor(&b.versor().expect("a versor"), &identity_versor()),
        "a (1,3) rotation is active in the conformal algebra of the base"
    );
    assert_eq!(b.axes(), vec![FIBRE_SPATIAL], "the turn occupies S");
    assert!(!b.is_identity(), "a 90° turn is not the identity word");

    let c = a.compose(&b).expect("the composite forms");
    let r = c.carried_rotor().expect("the composite carries its rotor");
    assert!(
        r.projective_eq(&q01.rotor().geo(&q13.rotor())),
        "and it is the Cl(4,0) product, both planes"
    );
    assert!(
        !r.projective_eq(&q01.rotor()),
        "which the first factor alone does not equal"
    );

    // THE READING THE COURT TAKES. `transition(x, y)` is `y ∘ x⁻¹`, and it
    // answered `None` for every input before this.
    let g = crate::transducer::cech::transition(&a, &b).expect("a transition");
    let got = crate::transducer::cech::rotor_of(&g).expect("a transition has a rotor");
    let want = q13.rotor().geo(&q01.rotor().reverse());
    assert!(got.projective_eq(&want), "the transition's rotor is g_y ∘ g_x⁻¹");
    assert!(!got.is_identity(), "and it is not the identity the court was writing");
}

/// M1 (17.1) and M2 (17.2): equality is the projective class, and the
/// address is derived from it. A versor and its negation are ONE element
/// of the conformal group and TWO coefficient lists; `form_key_bytes`
/// wrote the list, so one element had two addresses and `same_vertex`
/// had to read the class behind the bytes. It now writes the A4 output,
/// so the two are one address — the codec no longer writes the sign, and
/// a differing address is a differing element.
#[test]
fn address_identifies_versor_with_its_negation() {
    let q = |n: i64| num_rational::BigRational::from_integer(BigInt::from(n));
    let v = translator_versor(2, 3);
    let neg = v.scale(&q(-1));
    assert!(same_versor(&v, &neg), "one projective element");

    let a = of_versor(v);
    let b = of_versor(neg);
    assert_eq!(a.address(), b.address(), "and one address: the codec writes A4, not the sign");
    assert!(a.same_vertex(&b), "M1 reads the class");
    let scaled = of_versor(translator_versor(2, 3).scale(&q(7)));
    assert_eq!(a.address(), scaled.address(), "any λ·V is the same address");

    // A genuinely different action is still a different vertex.
    let c = of_versor(translator_versor(2, 4));
    assert!(!a.same_vertex(&c));
}

/// Two elements have the same action exactly when the elements' versors are
/// projectively equal.
#[test]
fn same_action_is_projective_versor_equality() {
    let t = translator_versor(1, 0);
    let a = elem_from_versor(t.clone());
    let scaled = elem_from_versor(t.scale(&q(3)));
    let other = elem_from_versor(translator_versor(0, 1));
    assert!(a.same_action(&scaled), "projective equality of V");
    assert!(!a.same_action(&other), "distinct versors are distinct actions");
}

/// The substance of a germinated element is its centred chain: one
/// deformation at any position is one address, while an equal tally with
/// different geometry stays distinct.
#[test]
fn address_ignores_position_keeps_geometry() {
    let a = [(0, 0, 1u8), (0, 1, 2), (1, 0, 3)];
    let b = [(0, 0, 1u8), (0, 1, 9), (1, 0, 3)]; // (0,1): 2→9
    // The same deformation transposed +7 rows, +5 cols.
    let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 7, c + 5, *h)).collect();
    let b_far: Vec<_> = b.iter().map(|(r, c, h)| (r + 7, c + 5, *h)).collect();
    let here = from_canonical(&germinate(&a, &b).unwrap());
    let far = from_canonical(&germinate(&a_far, &b_far).unwrap());
    assert_eq!(here.address(), far.address(), "same deformation, any position — ONE word");
    assert!(here.carriage(&far, &mut super::common::budget()).is_some(), "one thought: a versor carries one onto the other");
    // Equal tally, different geometry — the cured conflation: {1→0 here, 0→4
    // there} at two different relative offsets must stay DISTINCT.
    let chain = |links: &[(i64, i64, u8, u8)]| {
        from_canonical(
            &crate::transducer::element::canonical_chain(links)
                .expect("one entry per position is A5's chain"),
        )
    };
    let g1 = chain(&[(0, 0, 1, 0), (1, 1, 0, 4)]);
    let g2 = chain(&[(1, 0, 1, 0), (0, 2, 0, 4)]);
    assert!(g1.carriage(&g2, &mut super::common::budget()).is_none(), "equal tally, different geometry — no versor relates them");
    assert_ne!(g1.address(), g2.address(), "and distinct identity");
}

/// Vertex identity is the address together with ρ, σ and the shift: an
/// element differing in any one of them is a different vertex.
#[test]
fn same_vertex_distinguishes_rho_sigma_and_shift() {
    let id = crate::transducer::alphabet::Word::identity();
    let swap = from_connection(&id, &identity_versor(), &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0]).expect("swap");
    let cycle = from_connection(&id, &identity_versor(), &[(1, 2), (2, 3), (3, 1)], &Vec::new(), [1, 0, 0, 0]).expect("cycle");
    let place = from_connection(&id, &translator_versor(5, 0), &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0]).expect("shift");
    let mut dilated = swap.clone();
    if let Some(letters) = dilated.letters() {
        let two = Q::from(BigInt::from(2));
        let car = crate::transducer::cga::dilator(&two).geo(&letters.carriage);
        let rem = dilated.sub().to_vec();
        dilated.description =
            connection_description(&letters.spelling, &car, &letters.sigma, &rem);
        dilated.sync_action_from_tape();
        dilated.address.take();
    }
    assert!(!swap.same_vertex(&dilated), "ρ is identity");
    assert!(!swap.same_vertex(&cycle), "σ is identity");
    assert!(!swap.same_vertex(&place), "shift is identity");
    assert_ne!(swap.address(), dilated.address());
    assert_ne!(swap.address(), cycle.address());
    assert_ne!(swap.address(), place.address());
}

/// Two spellings of one connection (M2) give one address and one vertex.
#[test]
fn address_identifies_two_spellings() {
    let w = from_connection(&crate::transducer::alphabet::Word::identity(), &identity_versor(), &[(1, 2), (2, 1)], &Vec::new(), [5, 2, 0, 0]).expect("spells");
    let qid = crate::transducer::alphabet::LetterId::turn_90((0, 1)).unwrap();
    let long = crate::transducer::alphabet::Word::from_letters(&[(qid, false), (qid, false), (qid, false), (qid, false)]);
    let mut alt = w.clone();
    if let Some(letters) = alt.letters() {
        let spelling = crate::transducer::alphabet::Spelling { word: long };
        let rem = alt.sub().to_vec();
        alt.description =
            connection_description(&spelling, &letters.carriage, &letters.sigma, &rem);
        alt.sync_action_from_tape();
        alt.address.take();
        assert_eq!(alt.address(), w.address());
        assert!(alt.same_vertex(&w));
    }
}

/// A connection's address keys the A4 carriage, not a coordinate pair.
#[test]
fn from_connection_address_keys_carriage_not_pair() {
    let car = translator_versor(3, 5);
    let w = from_connection(&crate::transducer::alphabet::Word::identity(), &car, &[], &Vec::new(), [1, 0, 0, 0]).expect("spells");
    let letters = w.letters().expect("letters");
    let mut bytes = Vec::new();
    letters.carriage.put_canonical(&mut bytes);
    assert!(
        w.address().as_bytes().windows(bytes.len()).any(|s| s == bytes.as_slice()),
        "M2 keys the A4 carriage, not a coordinate pair"
    );
    let w2 = from_connection(&crate::transducer::alphabet::Word::identity(), &translator_versor(7, 0), &[], &Vec::new(), [1, 0, 0, 0]).expect("spells");
    assert_ne!(w.address(), w2.address());
}
