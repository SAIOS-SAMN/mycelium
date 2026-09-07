// 17.1, 17.2 — M1 and M2: when two elements are one vertex, and what the
// address of a vertex is derived from.

use crate::core::Q;
use crate::transducer::element::*;
use num_bigint::BigInt;
use crate::transducer::germinate::germinate;
use super::common::*;

/// M1 (17.1): equality is the projective class, and `same_vertex` says
/// so. A versor and its negation are ONE element of the conformal group
/// and TWO byte strings, and `form_key_bytes` writes the bytes, so the
/// two have different M2 addresses. `same_vertex` was
/// `address() == address()`, which called them two vertices; two organs
/// that derived opposite signs would then have been REFUSE_MV_INEXACT
/// for agreeing.
#[test]
fn a_versor_and_its_negation_are_one_vertex_and_two_addresses() {
    let q = |n: i64| num_rational::BigRational::from_integer(BigInt::from(n));
    let v = translator_versor(2, 3);
    let neg = v.scale(&q(-1));
    assert!(same_versor(&v, &neg), "one projective element");

    let a = of_versor(v);
    let b = of_versor(neg);
    assert_ne!(a.address(), b.address(), "and two addresses: the codec writes the sign");
    assert!(a.same_vertex(&b), "M1 reads the class, not the bytes");

    // A genuinely different action is still a different vertex.
    let c = of_versor(translator_versor(2, 4));
    assert!(!a.same_vertex(&c));
}

#[test]
fn same_action_is_projective_versor_equality() {
    let t = translator_versor(1, 0);
    let a = elem_from_versor(t.clone());
    let scaled = elem_from_versor(t.scale(&q(3)));
    let other = elem_from_versor(translator_versor(0, 1));
    assert!(a.same_action(&scaled), "projective equality of V");
    assert!(!a.same_action(&other), "distinct versors are distinct actions");
}

#[test]
fn substance_is_the_centered_chain() {
    let a = [(0, 0, 1u8), (0, 1, 2), (1, 0, 3)];
    let b = [(0, 0, 1u8), (0, 1, 9), (1, 0, 3)]; // (0,1): 2→9
    // The same deformation transposed +7 rows, +5 cols.
    let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 7, c + 5, *h)).collect();
    let b_far: Vec<_> = b.iter().map(|(r, c, h)| (r + 7, c + 5, *h)).collect();
    let here = from_residual(&germinate(&a, &b).unwrap());
    let far = from_residual(&germinate(&a_far, &b_far).unwrap());
    assert_eq!(here.address(), far.address(), "same deformation, any position — ONE word");
    assert!(here.carriage(&far).is_some(), "one thought: a versor carries one onto the other");
    // Equal tally, different geometry — the cured conflation: {1→0 here, 0→4
    // there} at two different relative offsets must stay DISTINCT.
    let g1 = from_residual(&vec![(0, 0, 1, 0), (1, 1, 0, 4)]);
    let g2 = from_residual(&vec![(1, 0, 1, 0), (0, 2, 0, 4)]);
    assert!(g1.carriage(&g2).is_none(), "equal tally, different geometry — no versor relates them");
    assert_ne!(g1.address(), g2.address(), "and distinct identity");
}

#[test]
fn same_vertex_is_address_rho_sigma_and_shift() {
    let id = crate::transducer::alphabet::Word::identity();
    let swap = from_connection(&id, &identity_versor(), &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0]).expect("swap");
    let cycle = from_connection(&id, &identity_versor(), &[(1, 2), (2, 3), (3, 1)], &Vec::new(), [1, 0, 0, 0]).expect("cycle");
    let place = from_connection(&id, &translator_versor(5, 0), &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0]).expect("shift");
    let mut dilated = swap.clone();
    if let Some(letters) = dilated.letters() {
        let two = Q::from(BigInt::from(2));
        let car = crate::transducer::cga::dilator(&two).geo(&letters.carriage);
        let rem = dilated.sub().to_vec();
        dilated.word = connection_word(&letters.spelling, &car, &letters.sigma, &rem);
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

#[test]
fn m2_two_spellings_one_address() {
    let w = from_connection(&crate::transducer::alphabet::Word::identity(), &identity_versor(), &[(1, 2), (2, 1)], &Vec::new(), [5, 2, 0, 0]).expect("spells");
    let qid = crate::transducer::alphabet::LetterId::quarter((0, 1)).unwrap();
    let long = crate::transducer::alphabet::Word::from_letters(&[(qid, false), (qid, false), (qid, false), (qid, false)]);
    let mut alt = w.clone();
    if let Some(letters) = alt.letters() {
        let spelling = crate::transducer::alphabet::Spelling { word: long };
        let rem = alt.sub().to_vec();
        alt.word = connection_word(&spelling, &letters.carriage, &letters.sigma, &rem);
        alt.sync_action_from_tape();
        alt.address.take();
        assert_eq!(alt.address(), w.address());
        assert!(alt.same_vertex(&w));
    }
}

#[test]
fn connection_address_keys_the_carriage_not_a_pair() {
    let car = translator_versor(3, 5);
    let w = from_connection(&crate::transducer::alphabet::Word::identity(), &car, &[], &Vec::new(), [1, 0, 0, 0]).expect("spells");
    let letters = w.letters().expect("letters");
    let mut bytes = Vec::new();
    letters.carriage.to_bytes(&mut bytes);
    assert!(
        w.address().as_bytes().windows(bytes.len()).any(|s| s == bytes.as_slice()),
        "M2 keys the A4 carriage, not a coordinate pair"
    );
    let w2 = from_connection(&crate::transducer::alphabet::Word::identity(), &translator_versor(7, 0), &[], &Vec::new(), [1, 0, 0, 0]).expect("spells");
    assert_ne!(w.address(), w2.address());
}
