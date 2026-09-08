// 23, 42 — replay: a clean word replays, a false speech is named, and a
// form whose rank or certificate does not re-derive is refused.

use crate::transducer::element::Element;
use crate::verdict::*;
use super::common::*;

/// A clean word replays; an unparseable body is a named Slashed state.
#[test]
fn replay_accepts_clean_and_names_false_speech() {
    let w = clean_word();
    assert!(blind_replay(&w).is_none(), "a clean derivation replays");
    let ruling = rule(&[], &Keyed::new(), &Stalks::default(),&w.address(), None, None);
    assert!(
        matches!(rule(&[], &Keyed::new(), &Stalks::default(),&w.address(), Some(&w), Some(&[0; 4])), Ruling::Slashed(_)),
        "a word that deflated nothing on the book is slashed"
    );
    assert_eq!(ruling, Ruling::Slashed("unparseable word"));
    assert_eq!(ruling.code(), dialogue::RULING_SLASHED);
}

/// A ruled word deepened by a section (`attach`/`merge_fibre` on a
/// hypha) still replays on every peer: the chain lanes are compared
/// against the word's base magnitude, and the section is counted on the
/// word, never re-derived from the chain.
#[test]
fn deepened_word_replays_on_a_peer() {
    let mut w = clean_word();
    assert!(blind_replay(&w).is_none());
    let base = w.base_magnitude();
    assert!(w.puncture(vec![(1, 3), (1, 3)]), "a fresh section punctures");
    assert_eq!(w.base_magnitude(), base, "the chain's lanes do not move");
    assert_eq!(w.magnitude()[3], base[3] + 1, "the section is counted on the word");
    assert!(blind_replay(&w).is_none(), "the deepened word replays at home");
    let bytes = w.to_bytes();
    let mut pos = 0usize;
    let served = Element::from_bytes(&bytes, &mut pos).expect("wire round trip");
    assert_eq!(pos, bytes.len());
    assert!(blind_replay(&served).is_none(), "the served copy replays on the peer");
}

/// A closed depth-0 form: Frame, one quarter-turn word, the star face.
fn closed_turn_form() -> Form {
    use crate::transducer::alphabet::{LetterId, Word};
    use crate::transducer::sentence::FormFace;
    let q = LetterId::quarter((0, 1)).expect("quarter");
    let w = Word::from_letters(&[(q, false)]);
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    f.vertices.push(Vertex::Word {
        address: crate::transducer::a21::Address::from_bytes([crate::transducer::a21::TAG_FORM, 1]),
        word: w,
        carriage: element::carriage(&one, &(zero.clone(), zero)),
        gauge: vec![(1, 2)],
        class: stalk::Invariant(vec![7]),
        contact: None,
    });
    f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("vertices"), });
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    assert!(f.is_closed(), "the star form coheres");
    f
}

/// Chapter 23.1: a citer replays only when its cited form resolves on
/// the cover; without a cover the citation refuses; a tampered face and
/// a zero descent refuse by name; a v2 tape is never accepted unparsed.
#[test]
fn replay_form_refuses_unruled_glue_and_replays_resolved() {
    use crate::transducer::alphabet::Word;
    use crate::transducer::sentence::as_element_form;
    let inner = closed_turn_form();
    let inner_e = as_element_form(&inner, [1, 0, 0, 0]).expect("inner tape");
    let inner_addr = inner_e.address();
    let mut outer = Form::default();
    let gi = outer
        .glue(&inner, inner_addr.clone(), Word::identity(), [1, 0, 0, 0])
        .expect("a depth-0 host glues a closed form");
    let outer_e = as_element_form(&outer, [1, 0, 0, 0]).expect("outer tape");
    assert_eq!(
        blind_replay(&outer_e),
        Some(REFUSE_GLUED_UNRULED),
        "with no cover behind it the citation refuses"
    );
    let resolve = |a: &[u8]| (a == inner_addr.as_bytes()).then(|| inner.clone());
    assert_eq!(blind_replay_with(&outer_e, &resolve), None, "a resolved citation replays");
    let mut bad = outer.clone();
    bad.faces[0].transport = inner.face_of(0, 1).expect("vertices");
    let bad_e = as_element_form(&bad, [1, 0, 0, 0]).expect("tampered tape");
    assert_eq!(blind_replay_with(&bad_e, &resolve), Some(REFUSE_FACE));
    let mut flat = as_element_form(&outer, [1, 0, 0, 0]).expect("tape");
    flat.set_magnitude([0; 4]);
    assert_eq!(blind_replay_with(&flat, &resolve), Some(REFUSE_NO_DESCENT));
    let mut cited_deeper = outer.clone();
    if let Vertex::Glued { depth, .. } = &mut cited_deeper.vertices[gi as usize] {
        *depth = 1;
    }
    let deeper_e = as_element_form(&cited_deeper, [1, 0, 0, 0]).expect("tape");
    assert_eq!(blind_replay_with(&deeper_e, &resolve), Some(REFUSE_GLUED_DEPTH));
}

/// A20: a certificate that does not un-pivot to the tape's before/after
/// refuses the form whole; a form with no certificates still replays.
#[test]
fn pivot_certificate_replays_or_refuses() {
    use crate::transducer::alphabet::Word;
    use crate::transducer::sentence::{
        as_element_form, ClassVector, PivotBy, PivotCertificate, Rank, Side,
    };
    let f = closed_turn_form();
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("tape");
    assert_eq!(
        replay_form(&e, &|_| None),
        None,
        "a closed form with no certificates replays"
    );
    let mut bad = f.clone();
    let bogus = PivotCertificate {
        chord: 0,
        axis: crate::transducer::sentence::Axis::Unit(0),
        before: crate::transducer::a21::Address::from_bytes([1u8]),
        after: ClassVector::of_word(&Word::identity()).expect("identity is a versor").class_key(),
        rank_before: Rank {
            torsion: 1,
            free: 0,
            offplane: 0,
            axes: 0,
        },
        rank_after: Rank::default(),
        mu: crate::transducer::sentence::Mu::Unmeasured,
        by: PivotBy::Word {
            face: 0,
            word: Word::identity(),
            side: Side::Left,
        },
    };
    bad.certificates.insert(bogus.key(), bogus);
    let bad_e = as_element_form(&bad, [1, 0, 0, 0]).expect("tampered tape");
    assert_eq!(
        replay_form(&bad_e, &|_| None),
        Some(REFUSE_PIVOT_INEXACT),
        "a certificate that does not re-derive refuses the form"
    );
}

/// A18: the tape's stored loops may not claim a rank below the forest
/// the faces re-derive.
#[test]
fn rank_lost_refused() {
    use crate::transducer::alphabet::Word;
    use crate::transducer::sentence::{as_element_form, FormFace};
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let sheet = Word::from_letters(&[(q, false)]);
    let mut f = Form::default();
    f.vertices.push(Vertex::Word {
        address: crate::transducer::a21::Address::from_bytes([crate::transducer::a21::TAG_FORM, 1]),
        word: Word::identity(),
        carriage: element::carriage(&one.clone(), &(zero.clone(), zero.clone())),
        gauge: vec![(1, 2)],
        class: stalk::Invariant::default(),
        contact: None,
    });
    f.vertices.push(Vertex::Word {
        address: crate::transducer::a21::Address::from_bytes([crate::transducer::a21::TAG_FORM, 2]),
        word: Word::identity(),
        carriage: element::carriage(&one, &(zero.clone(), zero)),
        gauge: vec![(1, 2)],
        class: stalk::Invariant::default(),
        contact: None,
    });
    f.faces.push(FormFace {
        i: 0,
        j: 1,
        transport: f.face_of(0, 1).expect("star 1"),
    });
    f.faces.push(FormFace {
        i: 0,
        j: 2,
        transport: f.face_of(0, 2).expect("star 2"),
    });
    f.faces.push(FormFace::new(1, 2, sheet));
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    let _ = f.pivot_loop(
        [0x18; 32],
        &crate::network::nerve::Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    assert!(
        !f.torsion.is_empty(),
        "finite holonomy files when no word discharges"
    );
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("filed tape");
    assert_eq!(replay_form(&e, &|_| None), None, "filed torsion replays");
    let mut gone = crate::transducer::sentence::from_element_form(&e).expect("v2");
    gone.loops.clear();
    let bad = as_element_form(&gone, [1, 0, 0, 0]).expect("loops stripped");
    assert_eq!(
        replay_form(&bad, &|_| None),
        Some(REFUSE_RANK_LOST),
        "stored loops below the re-derived forest refuse"
    );
}

/// Chapter 23.1 steps 3 and 6: a Word whose affine does not spell
/// refuses the tape; a Rebased whose word is identity equals its base;
/// an Unfolded Remainder with nothing left does not move; an identity
/// orbit does not move.
#[test]
fn vertex_tape_and_moves_replay_or_refuse() {
    use crate::transducer::alphabet::Word;
    use crate::transducer::sentence::{as_element_form, FormFace, Measure};
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let f = closed_turn_form();
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("tape");
    assert_eq!(replay_form(&e, &|_| None), None, "a closed word vertex re-derives");

    let mut rho0 = f.clone();
    if let Vertex::Word { word, .. } = &mut rho0.vertices[1] {
        let q02 = crate::transducer::alphabet::LetterId::quarter((0, 2)).expect("off-plane");
        *word = Word::from_letters(&[(q02, false)]);
    }
    let bad_e = as_element_form(&rho0, [1, 0, 0, 0]).expect("off-plane tape");
    assert_eq!(
        replay_form(&bad_e, &|_| None),
        Some(REFUSE_VERTEX_TAPE),
        "a word that does not keep the sensor plane refuses the tape"
    );

    let mut re = Form::default();
    re.vertices.push(Vertex::Word {
        address: crate::transducer::a21::Address::from_bytes([crate::transducer::a21::TAG_FORM, 2]),
        word: Word::identity(),
        carriage: element::carriage(&one.clone(), &(zero.clone(), zero.clone())),
        gauge: Vec::new(),
        class: stalk::Invariant::default(),
        contact: None,
    });
    re.vertices.push(Vertex::Rebased {
        by: 1,
        of: 2,
        carriage: element::carriage(&one.clone(), &(zero.clone(), zero.clone())),
        gauge: Vec::new(),
        class: stalk::Invariant::default(),
    });
    re.faces.push(FormFace { i: 0, j: 1, transport: re.face_of(0, 1).expect("vertices"), });
    re.loops = Form::loops_of(&re.faces, re.vertices.len());
    let re_e = as_element_form(&re, [1, 0, 0, 0]).expect("rebased tape");
    assert_eq!(
        replay_form(&re_e, &|_| None),
        Some(REFUSE_REBASE_TRIVIAL),
        "rebasing onto the identity equals the base"
    );

    let mut un = Form::default();
    un.unfold(Measure::Remainder);
    un.faces.push(FormFace::new(0, 0, Word::identity(),));
    un.loops = Form::loops_of(&un.faces, un.vertices.len());
    let un_e = as_element_form(&un, [1, 0, 0, 0]).expect("unfold tape");
    assert_eq!(
        replay_form(&un_e, &|_| None),
        Some(REFUSE_UNFOLD_STILL),
        "an empty remainder does not unfold"
    );

    let mut orb = Form::default();
    orb.vertices.push(Vertex::Word {
        address: crate::transducer::a21::Address::from_bytes([crate::transducer::a21::TAG_FORM, 3]),
        word: Word::identity(),
        carriage: element::carriage(&one, &(zero.clone(), zero)),
        gauge: Vec::new(),
        class: stalk::Invariant::default(),
        contact: Some(crate::transducer::alphabet::Contact { anchor: 0, ring: 1 }),
    });
    orb.faces.push(FormFace { i: 0, j: 1, transport: orb.face_of(0, 1).expect("vertices"), });
    orb.loops = Form::loops_of(&orb.faces, orb.vertices.len());
    let orb_e = as_element_form(&orb, [1, 0, 0, 0]).expect("orbit tape");
    assert_eq!(
        replay_form(&orb_e, &|_| None),
        Some(REFUSE_ORBIT_STILL),
        "an identity word does not orbit"
    );
}
