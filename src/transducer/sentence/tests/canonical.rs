// 17.2, A4 — THE KEY IS THE A4 OUTPUT. A carriage is a ray of the conformal
// group and not a list of sixteen coefficients: `c` and every `λ·c` are one
// element. Every key the sentence writes over a carriage — the class key, the
// vertex address, the certificate key — is the same bytes for every λ, so a
// differing key is a differing object and never a differing derivation.

use super::common::*;
use crate::transducer::alphabet::{self};
use crate::transducer::element::{self, same_versor};
use crate::transducer::sentence::*;
use crate::transducer::stalk;

fn by(v: &element::Versor, n: i64, d: i64) -> element::Versor {
    v.scale(&num_rational::BigRational::new(
        num_bigint::BigInt::from(n),
        num_bigint::BigInt::from(d),
    ))
}

/// One class key for one carriage ray, and the key still reads back to
/// that ray (law 2: reversible, content).
#[test]
fn a_class_key_is_one_under_every_scaling_of_the_carriage() {
    let c = element::translator_versor(0, 3);
    let neg = by(&c, -1, 1);
    assert!(same_versor(&c, &neg), "one projective element");

    let a = ClassVector::read(&order4(), c.clone(), CycleType::Identity).expect("a placed turn");
    let b = ClassVector::read(&order4(), neg, CycleType::Identity).expect("the same turn, carriage negated");
    let s = ClassVector::read(&order4(), by(&c, 7, 3), CycleType::Identity).expect("the same turn, carriage scaled");
    assert_eq!(a.class_key(), b.class_key(), "one class key: the carriage is written A4, not by sign");
    assert_eq!(a.class_key(), s.class_key(), "and not by scale");

    let back = ClassVector::from_key(a.class_key().as_bytes()).expect("a class key reads back");
    assert!(same_versor(&back.carriage, &c), "the key names the ray");

    let other = ClassVector::read(&order4(), element::translator_versor(0, 4), CycleType::Identity)
        .expect("a different placement");
    assert_ne!(a.class_key(), other.class_key(), "a different carriage is a different class");
}

/// One vertex address for one member carriage ray.
#[test]
fn a_form_address_is_one_under_every_scaling_of_a_members_carriage() {
    let form_with = |car: element::Versor| -> Form {
        let mut f = Form::default();
        stand_with_star(
            &mut f,
            &Vertex::word_on(
                alphabet::Derivation::spelled(order4()),
                stalk::Invariant::counts(&[3, 5]),
                Box::new(car),
                vec![(1, 2)],
                [FRAME],
                None,
            ),
        );
        f.remainder = crate::transducer::element::PlacedChain::default();
        f.depth = f.depth_of().expect("a fixture's depth reads");
        f.reindex();
        f
    };
    let c = element::carriage(&q1(), &(q0(), q1()));
    let a = form_vertex_key(&form_with(c.clone()));
    assert_eq!(a, form_vertex_key(&form_with(by(&c, -1, 1))), "one address (17.2): A4, not the sign");
    assert_eq!(a, form_vertex_key(&form_with(by(&c, 5, 2))), "and not the scale");
    let other = form_vertex_key(&form_with(element::carriage(&q1(), &(q1(), q1()))));
    assert_ne!(a, other, "a different carriage is a different vertex");
}

/// One certificate key for one move: a pivot by `T` and by `−T` is one pivot
/// (A20), and an axis opened on a seed ray is one opening.
#[test]
fn a_certificate_key_is_one_under_every_scaling_of_its_versor() {
    let t = element::translator_versor(2, 3);
    let placed = |translator: element::Versor| PivotCertificate {
        by: PivotBy::Placement { face: some_overlap(), translator },
        ..cert(Axis::Placement, [0; 4])
    };
    assert_eq!(placed(t.clone()).key(), placed(by(&t, -1, 1)).key(), "one certificate: A4, not the sign");
    assert_eq!(placed(t.clone()).key(), placed(by(&t, 3, 4)).key(), "and not the scale");
    assert_ne!(placed(t.clone()).key(), placed(element::translator_versor(2, 4)).key());

    let seeded = |seed: element::Versor| PivotCertificate {
        by: PivotBy::OpenAxis { face: some_overlap(), seed },
        ..cert(Axis::Open, [0; 4])
    };
    assert_eq!(seeded(t.clone()).key(), seeded(by(&t, -1, 1)).key(), "one opening: the seed is a ray");
}

/// LD0 step 5 — A CERTIFICATE IS WRITTEN AS ITS RAYS. The translator and
/// the seed go on the tape as their canonical rays, so `T` and `λT` write
/// the same bytes; and a FORMED word — an action no tape spells — keeps its
/// action across the tape, where the letters alone wrote it as the
/// identity.
#[test]
fn a_certificate_is_written_as_its_rays() {
    let t = element::translator_versor(2, 3);
    let bytes = |c: &PivotCertificate| {
        let mut out = Vec::new();
        c.to_bytes(&mut out).expect("the certificate writes");
        out
    };
    let placed = |translator: element::Versor| PivotCertificate {
        by: PivotBy::Placement { face: some_overlap(), translator },
        ..cert(Axis::Placement, [0; 4])
    };
    assert_eq!(bytes(&placed(t.clone())), bytes(&placed(by(&t, -3, 4))), "one ray, one writing");

    let formed = alphabet::Word::formed(order4().rotor());
    assert!(formed.letters().is_empty() && !formed.is_identity(), "an action no tape spells");
    let worded = PivotCertificate {
        by: PivotBy::Word { face: some_overlap(), word: Box::new(formed.clone()), side: Side::Right },
        ..cert(Axis::Placement, [0; 4])
    };
    let tape = bytes(&worded);
    let mut p = 0usize;
    let back = PivotCertificate::from_bytes(&tape, &mut p).expect("the certificate reads back");
    assert_eq!(p, tape.len(), "whole");
    let PivotBy::Word { word, .. } = &back.by else { panic!("a word move") };
    assert_eq!(**word, formed, "the formed word's action crossed the tape");
    assert_eq!(back.key(), worded.key(), "and the certificate is the one written");
}
