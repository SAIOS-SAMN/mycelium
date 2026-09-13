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
    f.vertices.push(Vertex::word_on(
        crate::transducer::address::Address::from_bytes([crate::transducer::address::TAG_FORM, 1]),
        crate::transducer::alphabet::Derivation::spelled(w),
        stalk::Invariant(vec![7]),
        Box::new(element::carriage(&one, &(zero.clone(), zero))),
        vec![(1, 2)],
        None,
    ));
    f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("vertices"), edge: None });
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    // A form off the tape is indexed by the decoder; one built by hand
    // must be indexed too, or its `by_address` answers nothing and a
    // citation into it reads as unheld (50.2 step 2).
    f.reindex();
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
    // S9.4 — A ZERO MAGNITUDE IS NOT A REFUSAL. This tape replays: its
    // loops return, its faces re-derive, its citation resolves. It was
    // refused for measuring nothing, which made the court an economist —
    // μ⃗ is a cost, not a score, and a word that costs nothing to say has
    // still said it. What closes the family is checked where the family is
    // (REFUSE_NOT_CLOSED); pricing is read afterwards, by the economy.
    let mut flat = as_element_form(&outer, [1, 0, 0, 0]).expect("tape");
    flat.set_magnitude([0; 4]);
    assert_eq!(
        blind_replay_with(&flat, &resolve),
        None,
        "closure certifies; a zero magnitude prices at zero and refuses nothing"
    );
    let mut cited_deeper = outer.clone();
    if let Vertex::Glued { depth, .. } = &mut cited_deeper.vertices[gi as usize] {
        *depth = 1;
    }
    let deeper_e = as_element_form(&cited_deeper, [1, 0, 0, 0]).expect("tape");
    assert_eq!(blind_replay_with(&deeper_e, &resolve), Some(REFUSE_GLUED_DEPTH));
}

/// 34.1: a sentence whose tape does not parse whole is refused by name,
/// never partly read. This is `REFUSE_FORM_TAPE`'s producing test (3.5).
#[test]
fn a_form_whose_tape_does_not_parse_is_refused_by_name() {
    use crate::transducer::sentence::as_element_form;
    let e = as_element_form(&closed_turn_form(), [1, 0, 0, 0]).expect("tape");
    assert_eq!(replay_form(&e, &|_| None), None, "the whole tape replays");
    let mut short = e.description().to_vec();
    short.pop();
    let mut cut = e.clone();
    cut.set_description(short);
    assert_eq!(
        replay_form(&cut, &|_| None),
        Some(REFUSE_FORM_TAPE),
        "a tape one byte short is refused, not read up to the cut"
    );
}

/// 15.3, 16.17, rule 11 (I5): a loop whose holonomy is not a versor has
/// no class. It is never keyed by its rotor's bytes in place of the class
/// it lacks: the rank audit is unmeasured, the form is not cauterized, and
/// the loop does not replay. The fallback this refuses survived 0.12b under
/// the name `class_or_rotor_key`.
#[test]
fn a_loop_whose_holonomy_is_not_a_versor_is_refused_not_keyed() {
    use crate::transducer::alphabet::{Rotor, Word};
    let mut f = closed_turn_form();
    assert!(f.loops.is_empty() && f.rank().is_some(), "a tree has a rank, and it is zero");
    f.loops.push(crate::transducer::sentence::Loop {
        vertices: vec![0, 1, 0],
        faces: vec![0, 0],
        chord: 0,
        holonomy: Word::formed(Rotor::zero()),
        carriage: element::identity_versor(),
    });
    assert!(f.loops[0].holonomy.rotor().class_key().is_none(), "a zero rotor is not a class");
    assert!(f.rank().is_none(), "the audit is unmeasured, not counted under a rotor key");
    assert!(!f.is_cauterized(), "nothing files what has no class");
    assert!(!form_loops_replay(&f), "and the loop does not replay");
}

/// A20: a certificate that does not un-pivot to the tape's before/after
/// refuses the form whole; a form with no certificates still replays.
#[test]
fn pivot_certificate_replays_or_refuses() {
    use crate::transducer::alphabet::Word;
    use crate::transducer::sentence::{
        as_element_form, ClassVector, PivotBy, PivotCertificate, Side,
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
        before: crate::transducer::address::Address::from_bytes([1u8]),
        after: ClassVector::of_word(&Word::identity()).expect("identity is a versor").class_key(),
        rank_move: crate::transducer::sentence::RankMove::Discharged {
            from: crate::transducer::sentence::RankAxis::Torsion,
        },
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
    for tag in 1..=2u8 {
        f.vertices.push(Vertex::word_on(
            crate::transducer::address::Address::from_bytes([crate::transducer::address::TAG_FORM, tag]),
            crate::transducer::alphabet::Derivation::spelled(Word::identity()),
            stalk::Invariant::default(),
            Box::new(element::carriage(&one.clone(), &(zero.clone(), zero.clone()))),
            vec![(1, 2)],
            None,
        ));
    }
    f.faces.push(FormFace {
        i: 0,
        j: 1,
        transport: f.face_of(0, 1).expect("star 1"),
        edge: None,
    });
    f.faces.push(FormFace {
        i: 0,
        j: 2,
        transport: f.face_of(0, 2).expect("star 2"),
        edge: None,
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

    // S9.2 — AND A TAPE CLAIMING MORE THAN THE WALK IS REFUSED TOO. The
    // clause used to test `taped < derived` alone, so a form whose stored
    // loops carry rank the re-walk does not produce passed: structure the
    // court could not derive, admitted because it pointed the convenient
    // way. Duplicating a stored loop raises the tape's audit above the
    // forest's without touching a face.
    let mut extra = crate::transducer::sentence::from_element_form(&e).expect("v2");
    let held = extra.loops[0].clone();
    extra.loops.push(held);
    let over = as_element_form(&extra, [1, 0, 0, 0]).expect("loops duplicated");
    assert_eq!(
        replay_form(&over, &|_| None),
        Some(REFUSE_RANK_LOST),
        "a rank the walk does not re-derive refuses in either direction"
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
    if let Vertex::Word { derivation, .. } = &mut rho0.vertices[1] {
        let q02 = crate::transducer::alphabet::LetterId::quarter((0, 2)).expect("off-plane");
        *derivation =
            crate::transducer::alphabet::Derivation::spelled(Word::from_letters(&[(q02, false)]));
    }
    let bad_e = as_element_form(&rho0, [1, 0, 0, 0]).expect("off-plane tape");
    assert_eq!(
        replay_form(&bad_e, &|_| None),
        Some(REFUSE_VERTEX_TAPE),
        "a word that does not keep the sensor plane refuses the tape"
    );

    let mut re = Form::default();
    re.vertices.push(Vertex::word_on(
        crate::transducer::address::Address::from_bytes([crate::transducer::address::TAG_FORM, 2]),
        crate::transducer::alphabet::Derivation::spelled(Word::identity()),
        stalk::Invariant::default(),
        Box::new(element::carriage(&one.clone(), &(zero.clone(), zero.clone()))),
        Vec::new(),
        None,
    ));
    re.vertices.push(Vertex::Rebased {
        by: 1,
        of: 2,
        carriage: Box::new(element::carriage(&one.clone(), &(zero.clone(), zero.clone()))),
        gauge: Vec::new(),
        class: stalk::Invariant::default(),
    });
    re.faces.push(FormFace { i: 0, j: 1, transport: re.face_of(0, 1).expect("vertices"), edge: None });
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
    orb.vertices.push(Vertex::word_on(
        crate::transducer::address::Address::from_bytes([crate::transducer::address::TAG_FORM, 3]),
        crate::transducer::alphabet::Derivation::spelled(Word::identity()),
        stalk::Invariant::default(),
        Box::new(element::carriage(&one, &(zero.clone(), zero))),
        Vec::new(),
        Some(crate::transducer::alphabet::Contact { anchor: 0, ring: 1 }),
    ));
    orb.faces.push(FormFace { i: 0, j: 1, transport: orb.face_of(0, 1).expect("vertices"), edge: None });
    orb.loops = Form::loops_of(&orb.faces, orb.vertices.len());
    let orb_e = as_element_form(&orb, [1, 0, 0, 0]).expect("orbit tape");
    assert_eq!(
        replay_form(&orb_e, &|_| None),
        Some(REFUSE_ORBIT_STILL),
        "an identity word does not orbit"
    );

    // 50.5: a demand vertex names a kind that carries its coordinates. A
    // source no chart carries onto is a creation, not a source of
    // multiplicity zero; the court refuses the vertex by name.
    let mut dem = closed_turn_form();
    dem.vertices.push(Vertex::Demand {
        kind: crate::transducer::sentence::Demand::Source { multiplicity: 0 },
        chart: stalk::Invariant(vec![1, 2, 2]),
        bodies: vec![(3, 3, 0, 2)],
    });
    let dem_e = as_element_form(&dem, [1, 0, 0, 0]).expect("demand tape");
    assert_eq!(
        replay_form(&dem_e, &|_| None),
        Some(REFUSE_DEMAND_UNTYPED),
        "a demand vertex carrying nothing is refused by name"
    );
    let mut typed = closed_turn_form();
    typed.vertices.push(Vertex::Demand {
        kind: crate::transducer::sentence::Demand::Creation,
        chart: stalk::Invariant(vec![1, 2, 2]),
        bodies: vec![(3, 3, 0, 2)],
    });
    let typed_e = as_element_form(&typed, [1, 0, 0, 0]).expect("typed demand tape");
    assert_eq!(replay_form(&typed_e, &|_| None), None, "a typed demand vertex replays beside its words");

    // 50.3: an edge transport is a stabilizer element or an exchange, both
    // of finite order on the word; a (3,4,5) rotor on an edge is neither.
    let mut edged = closed_turn_form();
    let p5 = crate::transducer::alphabet::LetterId::pythagorean_5((0, 1)).expect("(3,4,5)");
    let fi = edged.faces.iter().position(|f| f.i == 0 && f.j == 1).expect("the star face");
    edged.faces[fi].edge = Some(crate::transducer::sentence::Transport::from_word(Word::from_letters(&[(p5, false)])));
    let edged_e = as_element_form(&edged, [1, 0, 0, 0]).expect("edged tape");
    assert_eq!(replay_form(&edged_e, &|_| None), Some(REFUSE_EDGE), "an edge of infinite order is refused by name");
    let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
    edged.faces[fi].edge = Some(crate::transducer::sentence::Transport::from_word(Word::from_letters(&[(q, false)])));
    let fine_e = as_element_form(&edged, [1, 0, 0, 0]).expect("edged tape");
    assert_eq!(replay_form(&fine_e, &|_| None), None, "a quarter turn on an edge is a stabilizer element and replays");
}

/// S9.4 — CLOSURE IS WHAT THE COURT CHECKS. A form minted over one train
/// pair replays clean against that pair, and refuses against a family whose
/// second pair it does not close. The refusal names the remainder, not the
/// magnitude: a form that closes every pair while measuring nothing has
/// still closed them, and what it measured is the economy's reading.
///
/// This is `REFUSE_NOT_CLOSED`'s producing test (3.5). The reason it
/// replaces, `REFUSE_NO_DESCENT`, fired on a zero magnitude — a fact about
/// price, ruled on by a court.
#[test]
fn a_form_that_leaves_a_remainder_is_refused_for_closure() {
    use crate::transducer::family::{extract_rule, mint_sentence, NoCrystal};
    let grid = |rows: &[&str]| -> crate::arc::Grid {
        rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
    };
    let (x, y) = (grid(&["1"]), grid(&["2"]));
    let one_pair = vec![(x.clone(), y.clone())];
    let rule = extract_rule(&one_pair, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("a recolour is a rule");
    let word = mint_sentence(&rule, &one_pair, &NoCrystal).expect("it closes its own pair");
    assert_eq!(
        blind_replay_on(&word, &|_| None, Some(&one_pair), &NoCrystal),
        None,
        "the form closes the family it was minted over"
    );

    // A second pair the same rule does not speak: the remainder is not empty.
    let wider = vec![(x, y), (grid(&["10"]), grid(&["23"]))];
    assert_eq!(
        blind_replay_on(&word, &|_| None, Some(&wider), &NoCrystal),
        Some(REFUSE_NOT_CLOSED),
        "a remainder on any pair is the refusal"
    );
}

/// A form whose one word vertex cites `inner`'s word vertex, with the
/// exponent it was composed at and the residue that completes it. Built
/// through `Derivation::compose`, so the product is what the citations
/// actually reach and never a number written beside them (§3.4).
fn citing_form(
    inner_addr: &crate::transducer::address::Address,
    inner: &Form,
    exponent: i64,
    residue: crate::transducer::alphabet::Word,
) -> Form {
    use crate::transducer::sentence::FormFace;
    let vertex = match inner.vertices.get(1) {
        Some(Vertex::Word { address, derivation, .. }) => {
            (address.clone(), derivation.word_local().expect("the inner vertex spells").clone())
        }
        _ => panic!("the inner form's word vertex"),
    };
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let derivation = crate::transducer::alphabet::Derivation::compose(
        &[(inner_addr.clone(), vertex.0, exponent, vertex.1)],
        residue,
    );
    let mut f = Form::default();
    f.vertices.push(Vertex::word_on(
        crate::transducer::address::Address::from_bytes([
            crate::transducer::address::TAG_FORM,
            9,
        ]),
        derivation,
        stalk::Invariant(vec![7]),
        Box::new(element::carriage(&one, &(zero.clone(), zero))),
        vec![(1, 2)],
        None,
    ));
    f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("vertices"), edge: None });
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f.reindex();
    f
}

/// 50.2 step 2 at the court — G COMPOSES THE DERIVATION. A cited
/// generator the cover resolves composes and the sentence replays.
///
/// A cited FORM the court has not ruled never reaches here: the door
/// decides that before any replay runs and the sentence WAITS
/// (`hear_sentence`, `unruled_citation`), because a refusal would be
/// sealed by the voice and the sentence lost for being one syllabus step
/// early. What reaches here is the other case — the form resolved and
/// the vertex named is not in it — which is a claim about the tape and a
/// refusal at the vertex that made it.
#[test]
fn a_court_composes_a_resolved_citation_and_refuses_one_naming_no_vertex() {
    use crate::transducer::family::NoCrystal;
    use crate::transducer::sentence::as_element_form;
    let inner = closed_turn_form();
    let inner_e = as_element_form(&inner, [1, 0, 0, 0]).expect("inner tape");
    let inner_addr = inner_e.address();
    let citer = citing_form(&inner_addr, &inner, 1, crate::transducer::alphabet::Word::identity());
    let citer_e = as_element_form(&citer, [1, 0, 0, 0]).expect("citer tape");

    let cover = |a: &[u8]| -> Option<Form> {
        match a == inner_addr.as_bytes() {
            true => Some(inner.clone()),
            false => None,
        }
    };
    assert_eq!(
        replay_form_at(&citer_e, &cover, None, &NoCrystal).map(|r| r.reason),
        None,
        "the court resolved the citation, composed it, and agreed"
    );

    // The form resolves, and carries no vertex at the address cited: an
    // empty form stands for one the citation names wrongly.
    let hollow = |a: &[u8]| -> Option<Form> {
        match a == inner_addr.as_bytes() {
            true => Some(Form::default()),
            false => None,
        }
    };
    let refusal = replay_form_at(&citer_e, &hollow, None, &NoCrystal)
        .expect("the cited form carries no such vertex");
    assert_eq!(refusal.reason, REFUSE_DERIVATION);
    assert!(
        matches!(refusal.at, crate::transducer::sentence::At::Vertex { .. }),
        "and the refusal names the vertex that made the claim"
    );
}

/// The staleness answer of 50.2 step 2. The product is on the tape beside
/// the citations, so a voice could carry citations that do not reach it;
/// the court's own re-derivation is what refuses that, at the vertex.
#[test]
fn a_derivation_that_does_not_compose_to_its_word_is_refused_at_the_vertex() {
    use crate::transducer::family::NoCrystal;
    use crate::transducer::sentence::as_element_form;
    let inner = closed_turn_form();
    let inner_e = as_element_form(&inner, [1, 0, 0, 0]).expect("inner tape");
    let inner_addr = inner_e.address();
    // The citation says the quarter turn once; the tape says it three
    // times. One of them is wrong and the court says which vertex.
    let honest = citing_form(&inner_addr, &inner, 1, crate::transducer::alphabet::Word::identity());
    let lying = citing_form(&inner_addr, &inner, 3, crate::transducer::alphabet::Word::identity());
    let cover = |a: &[u8]| -> Option<Form> {
        match a == inner_addr.as_bytes() {
            true => Some(inner.clone()),
            false => None,
        }
    };
    let mut tampered = honest.clone();
    match (&mut tampered.vertices[1], lying.vertices.get(1)) {
        (Vertex::Word { derivation, .. }, Some(Vertex::Word { derivation: other, .. })) => {
            *derivation = crate::transducer::alphabet::Derivation::factored(
                other.generators.clone(),
                other.residue.clone(),
                derivation.rotor(),
            );
        }
        _ => panic!("both forms carry a word vertex"),
    }
    let refusal = replay_form_at(
        &as_element_form(&tampered, [1, 0, 0, 0]).expect("tampered tape"),
        &cover,
        None,
        &NoCrystal,
    )
    .expect("the citations reach a different vertex than the word carried");
    assert_eq!(refusal.reason, REFUSE_DERIVATION);
    assert_eq!(refusal.kind(), crate::transducer::sentence::RefusalKind::MalformedComplex);
    assert!(
        matches!(refusal.at, crate::transducer::sentence::At::Vertex { .. }),
        "the coordinate is the vertex, not the tape (50.9)"
    );
}

/// 50.3, 23.1 step 4 — THE COURT READS THE LOOP THE VOICE EVALUATED. A
/// loop has two obstructions: the TRANSPORT composite the pivot discharges
/// and files, and the EDGE HOLONOMY the cover classifies and files (50.3).
/// The court read the first alone, so the class a cover actually filed was
/// never looked for — and the transport composite telescopes over the
/// sensor-plane words a pair form composes, so it reads flat however curved
/// the scene is.
///
/// Here the transports are pure coboundaries and compose to the identity,
/// while the faces carry a half-turn edge: the loop is flat on the reading
/// the court had and curved on the one the voice made.
#[test]
fn a_cover_filed_class_is_what_the_court_reads_on_a_flat_transport() {
    use crate::transducer::alphabet::{LetterId, Word};
    use crate::transducer::sentence::{FiledLoop, Form, FormFace, Transport};
    let q = LetterId::quarter((0, 1)).expect("the sensor plane");
    let half = Word::from_letters(&[(q, false), (q, false)]);
    let edge_of = |w: &Word| Transport {
        word: w.clone(),
        carriage: element::identity_versor(),
        gauge: Vec::new(),
    };
    let mut f = Form::default();
    for tag in [1u8, 2u8] {
        f.vertices.push(Vertex::word_on(
            crate::transducer::address::Address::from_bytes([tag; 32]),
            crate::transducer::alphabet::Derivation::spelled(Word::identity()),
            stalk::Invariant::default(),
            Box::new(element::identity_versor()),
            Vec::new(),
            None,
        ));
    }
    // The two star faces and the body-to-body face, every transport the
    // coboundary of its vertices; the body face carries the half turn as
    // the transition between the two pieces' own frames.
    for (i, j) in [(0u16, 1u16), (0, 2)] {
        let transport = f.face_of(i, j).expect("a coboundary");
        f.faces.push(FormFace { i, j, transport, edge: Some(edge_of(&Word::identity())) });
    }
    let transport = f.face_of(1, 2).expect("a coboundary");
    f.faces.push(FormFace { i: 1, j: 2, transport, edge: Some(edge_of(&half)) });
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f.reindex();
    let [lp, ..] = f.loops.as_slice() else { panic!("the triangle closes") };
    assert!(
        lp.holonomy.is_identity(),
        "the transport composite is flat: every face is a coboundary"
    );
    let e = f.edge_holonomy(lp).expect("the loop is evaluated: every face carries an edge");
    assert!(!e.word.is_identity(), "and its edge holonomy is the half turn");

    // UNFILED, the loop does not replay — which the transport reading could
    // not say, because on it the loop is flat.
    assert!(
        !form_loops_replay(&f),
        "an evaluated loop with an unfiled class does not replay"
    );
    // FILED under the edge holonomy's own class at its own period, it does.
    let class = f.edge_class(lp).expect("the half turn has a class");
    let mut filed = f.clone();
    filed.torsion.insert(
        class,
        FiledLoop { sigma: Vec::new(), period: 2, chord: lp.chord },
    );
    assert!(form_loops_replay(&filed), "filed at its period, the loop replays");
    // And at any other period it does not: the period is the loop's order,
    // not the filer's opinion of it (A17).
    let mut wrong = f.clone();
    wrong.torsion.insert(
        f.edge_class(lp).expect("class"),
        FiledLoop { sigma: Vec::new(), period: 3, chord: lp.chord },
    );
    assert!(!form_loops_replay(&wrong), "a period the loop does not close at is not a filing");
}
