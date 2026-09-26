// 16.1, 16.2 — the rotor's class key and normal form: two tapes of one
// vertex have one key, and the canonical form is projective either sign.

use crate::transducer::alphabet::*;

/// Rebase is conjugation then the normal form. Contact is not a letter
/// and is not in the M1 key.
#[test]
fn rebase_is_normal_form_and_contact_in_key() {
    let q = LetterId::turn_90((0, 1)).expect("sensor 90° turn");
    let w = Word::from_letters(&[(q, false)]);
    let by = Word::from_letters(&[(q, false), (q, false)]);
    let r = w.rebase(&by);
    assert_eq!(r, r.normal_form(), "rebase stores the normal form");
    let want = by.compose(&w.compose(&by.inverse())).normal_form();
    assert!(r.rotor().projective_eq(&want.rotor()), "rebase is conjugation");
    assert_eq!(w.key(), w.rotor().key(), "M1 key is the rotor key");
    assert_eq!(r.key(), r.normal_form().key());
}

/// An unspellable rotor does not invent the identity word, and a rotor
/// that is no versor has no order. A letter's spelling carries the same
/// A1 key as the rotor itself, and the same order.
#[test]
fn spelled_word_is_none_when_the_rotor_does_not_spell() {
    assert!(Rotor::zero().spelled_word().is_none());
    assert!(Rotor::zero().order().is_none());
    let q = LetterId::turn_90((0, 1)).expect("sensor 90° turn");
    let r = letter(q);
    let w = r.spelled_word().expect("the 90° turn spells");
    assert_eq!(w.key(), r.key());
    assert_eq!(r.order().expect("the 90° turn has order"), w.order());
}

#[test]
fn rotor_class_key_invariant() {
    let q = LetterId::turn_90((0, 1)).unwrap();
    let w = Word::from_letters(&[(q, false)]);
    let r = w.rotor();
    let s = Word::from_letters(&[(LetterId::turn_90((0, 2)).unwrap(), false)]).rotor();
    let conj = s.geo(&r).geo(&s.reverse());
    assert_eq!(r.class_key(), conj.class_key());
    let scaled = r.scale(&qn(3, 2));
    assert_eq!(r.class_key(), scaled.class_key());
    let f = w.form().unwrap();
    assert_eq!(
        w.class_key(),
        r.class_key(),
        "Word::class_key is the rotor conjugacy (15.3), not the plane section"
    );
    assert_eq!(f.conjugacy_key(), f.conjugacy_rep().conjugacy_key());
}

/// THE CLASS IS THE RAY'S, NOT THE LIFT'S. `projective_eq` says `−R` and
/// `R` are one vertex because the action divides the scale out, so the
/// class key of one must be the class key of the other.
///
/// The test above scales by `3/2` and stops, so the property was asked of
/// every positive multiple and of no negative one — and the sign bytes
/// flip under exactly those.
///
/// `q⁴ = −1` in Spin(4), so `q·q·q` is `−q⁻¹`: the other lift of the same
/// rotation, which a composition reaches without anyone choosing it.
#[test]
fn a_rotor_and_its_negative_are_one_class() {
    let q = LetterId::turn_90((0, 1)).unwrap();
    let r = Word::from_letters(&[(q, false)]).rotor();
    let neg = r.scale(&-qn(1, 1));
    assert!(r.projective_eq(&neg), "the action divides the scale out");
    assert_eq!(r.class_key(), neg.class_key(), "one ray, one class");
    assert!(r.class_eq(&neg));
    // And the composition that produces the other lift on its own: three
    // 90° turns are `−q⁻¹`, whose class is `q⁻¹`'s.
    let cubed = Word::from_letters(&[(q, false), (q, false), (q, false)]).rotor();
    let inv = r.reverse();
    assert!(cubed.projective_eq(&inv), "q³ is the other lift of q⁻¹");
    assert_eq!(cubed.class_key(), inv.class_key(), "and carries its class");
}

/// Two tapes of one vertex have one key; a tape and its geodesic
/// replay to the same affine.
#[test]
fn two_tapes_of_one_vertex_have_one_key() {
    let q = LetterId::turn_90((0, 1)).unwrap();
    let a = Word::from_letters(&[(q, false), (q, false), (q, false), (q, false)]);
    let b = Word::identity();
    assert!(a.rotor().projective_eq(&b.rotor()), "q^4 and 1 are one rotor under ℚ×");
    assert_eq!(a, b, "vertex equality is A1, not a digest of either tape");
    assert_eq!(a.key(), b.key(), "q^4 and 1 are one A1 key");
    let long = Word::raw(vec![(q, false), (q, false), (q, false), (q, false), (q, false)]);
    let geo = long.geodesic();
    assert_eq!(geo.len(), 1, "five 90° turns geodesic to one");
    let ma = long.matrix().expect("affine");
    let mb = geo.matrix().expect("affine");
    assert_eq!(ma, mb, "tape and geodesic replay the same affine");
    let g = relators();
    assert_eq!(g.len(), RELATOR_N);
    assert!(g.iter().any(|r| r.kind == RelatorKind::Turn90OrderFour));
    assert!(g.iter().any(|r| r.kind == RelatorKind::PlaneCommute));
    assert!(g.iter().any(|r| r.kind == RelatorKind::SheetConjugate));
    assert!(g.iter().any(|r| r.kind == RelatorKind::PrimeInfinite));
    assert!(g.iter().any(|r| r.kind == RelatorKind::Turn90OrderFour && r.word.geodesic().is_empty()));
}

#[test]
fn a1_canonical_is_projective_either_sign() {
    let q = LetterId::turn_90((0, 1)).unwrap();
    let r = Word::from_letters(&[(q, false)]).rotor();
    let q4 = Word::from_letters(&[(q, false), (q, false), (q, false), (q, false)]);
    assert_eq!(q4.key(), Word::identity().key(), "q^4 and 1 share an A1 key");
    assert_eq!(r.normalised().key(), r.scale(&qn(-5, 2)).normalised().key());
    assert!(r.projective_eq(&r.scale(&qn(-1, 1))));
    let next = Word::from_letters(&[(q, false), (q, false)]).rotor();
    assert_ne!(r.key(), next.key(), "R and R·q are distinct vertices");
    let f = Word::from_letters(&[(q, false), (q, false), (q, false)]).form().unwrap();
    assert_eq!(f.rot[q.raw() as usize], -1, "A3 writes three 90° turns as −1");
}
