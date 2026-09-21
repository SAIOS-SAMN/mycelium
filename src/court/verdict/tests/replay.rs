// 23, 42 — replay: a clean word replays, a false speech is named, and a
// form whose rank or certificate does not re-derive is refused.

use crate::transducer::element::Element;
use crate::transducer::sentence::ToralPeriod;
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
    let q = LetterId::quarter((0, 1)).expect("quarter");
    let w = Word::from_letters(&[(q, false)]);
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    f.vertices.push(Vertex::word_on(
        crate::transducer::alphabet::Derivation::spelled(w),
        stalk::Invariant::counts(&[7]),
        Box::new(element::carriage(&one, &(zero.clone(), zero))),
        vec![(1, 2)],
        None,
    ));
    f.faces.push(f.face_of(0, 1).expect("vertices"));
    f.loops = Form::loops_of(&f.faces, &f.vertices);
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
    bad.faces[0] = {
        // The tamper keeps the face's ENDS and takes the inner form's word and
        // placements, which is what made this face fail to re-derive before the
        // face was a fibre.
        let taken = inner.face_of(0, 1).expect("vertices");
        crate::transducer::sentence::FormFace {
            i: bad.faces[0].i,
            j: bad.faces[0].j,
            word: taken.word,
            at: taken.at,
            pairs: bad.faces[0].pairs.clone(),
        }
    };
    let bad_e = as_element_form(&bad, [1, 0, 0, 0]).expect("tampered tape");
    assert_eq!(blind_replay_with(&bad_e, &resolve), Some(REFUSE_FACE));
    // 50.9 (R1): the face that does not re-derive is refused at its two
    // addresses and travels as `FaceDerivation`. The tampered face is the
    // Frame's star face, and the Frame has no address, so this one is at
    // the tape; a face between two addressed vertices names both.
    let face_refusal = replay_form_at(&bad_e, &resolve, None, &crate::transducer::family::NoCrystal)
        .expect("the tampered face refuses");
    assert_eq!(face_refusal.kind(), crate::transducer::sentence::RefusalKind::FaceDerivation);
    assert!(matches!(face_refusal.at, crate::transducer::sentence::At::Tape(_)), "a star face has one address to name");
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
    // A TAMPERED DEPTH NO LONGER SURVIVES THE COORDINATE (54.5, as ruled
    // 2026-09-20). This moved a citation's stored depth to 1 and expected
    // `REFUSE_GLUED_DEPTH`, which fired because `f.depth` came off the tape
    // and disagreed with `f.depth_of()`.
    //
    // `from_address` DERIVES `Form.depth` now — one more than the deepest
    // citation, read off the `Glued` vertices the key writes — so once the
    // address is the only decoder this tamper will be ERASED rather than
    // refused, which is the stronger property: a derived coordinate cannot
    // be forged, so there is nothing left to catch.
    //
    // Both readings are asserted, and the first is decoder-independent: a
    // citation's own `depth` is in the key, so moving it names a different
    // word whatever reads it.
    let mut cited_deeper = outer.clone();
    if let Vertex::Glued { depth, .. } = &mut cited_deeper.vertices[gi as usize] {
        *depth = 1;
    }
    // 54.7 B5a — AND THE EDGE MOVES WITH IT, because the boundary and the
    // incidence are two coordinates of one citation. `depth` is part of the
    // vertex's SITE, so tampering the vertex alone leaves it standing where
    // no edge of the cover stands, and the tape writer refuses to write a
    // citation whose target the form does not carry (rule 11) rather than
    // writing a zero address. A forger has to move both, which is what this
    // does — and the key still names a different word for it.
    for e in &mut cited_deeper.cover {
        e.depth = 1;
    }
    let deeper_e = as_element_form(&cited_deeper, [1, 0, 0, 0]).expect("tape");
    assert_ne!(
        deeper_e.address(),
        outer_e.address(),
        "a citation's depth is in the key: moving it names a different word"
    );
    // AND THE TAMPER IS NOW ERASED RATHER THAN REFUSED (54.5 B4, landed
    // 2026-09-20) — which is what the note above predicted, and it is the
    // stronger property.
    //
    // `f.depth` used to ride the tape, so a forged value survived the
    // decode and disagreed with `f.depth_of()`, and the court caught the
    // disagreement. `from_address` DERIVES `Form.depth` from the `Glued`
    // vertices the key writes, so there is no longer a carrier to forge:
    // the decoded form's depth is one more than its deepest citation by
    // construction, the disagreement cannot exist, and the check has
    // nothing left to catch.
    //
    // A derived coordinate cannot be lied about. The word this tamper
    // names is a DIFFERENT word — asserted above, decoder-independently,
    // because the citation's own depth is in the key — and that word
    // replays honestly as itself.
    assert_eq!(
        blind_replay_with(&deeper_e, &resolve),
        None,
        "the forged depth is erased by derivation, not caught by a check"
    );
    let rebuilt = crate::transducer::sentence::from_element_form(&deeper_e).expect("decodes");
    assert_eq!(
        rebuilt.depth,
        rebuilt.depth_of(),
        "a derived depth agrees with its own citations by construction"
    );
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
    // THE NON-VERSOR HOLONOMY GOES ON THE FACE, which is where a holonomy
    // comes from (54.5, as ruled 2026-09-20). It was written onto
    // `Loop.holonomy` alone — a composite cached on the projected vertex
    // path — and `holonomy_words_on` reads the composite THIS PATH walked
    // now, member by member, so a value parked on that field is not what
    // the class is read from. The face carries it and the walk composes it.
    if let Some(face) = f.faces.first_mut() {
        face.word = Word::formed(Rotor::zero());
    }
    f.loops.push(crate::transducer::sentence::Loop {
        // The cycle names its members; `vertices` is π of them. This one is
        // hand-built to carry a holonomy that is not a versor, so its nodes
        // are the two vertices at no named chart.
        members: vec![
            crate::transducer::family::member_key(0, &[]),
            crate::transducer::family::member_key(1, &[]),
            crate::transducer::family::member_key(0, &[]),
        ],
        vertices: vec![0, 1, 0],
        faces: vec![0, 0],
        chord: 0,
        holonomy: Word::formed(Rotor::zero()),
        carriages: crate::network::nerve::Keyed::new(),
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
    use crate::transducer::sentence::as_element_form;
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let sheet = Word::from_letters(&[(q, false)]);
    let mut f = Form::default();
    for tag in 1..=2u8 {
        f.vertices.push(Vertex::word_on(
            crate::transducer::alphabet::Derivation::spelled(Word::identity()),
            // Each vertex stands on its OWN chart: with one chart they would be one
            // vertex, since an address is derived from its coordinates (54.2, §1.5).
            stalk::Invariant::counts(&[u64::from(tag)]),
            Box::new(element::carriage(&one.clone(), &(zero.clone(), zero.clone()))),
            vec![(1, 2)],
            None,
        ));
    }
    f.faces.push(f.face_of(0, 1).expect("star 1"));
    f.faces.push(f.face_of(0, 2).expect("star 2"));
    f.faces.push(f.face_carrying(1, 2, sheet).expect("a chord between two placed bodies"));
    f.loops = Form::loops_of(&f.faces, &f.vertices);
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
    // S9.2'S FORGERY CANNOT BE BUILT ANY MORE (54.5, as ruled 2026-09-20).
    //
    // This went on to strip the tape's loops, then to add a loop of a class
    // the walk does not derive, then to duplicate one — three tapes whose
    // STORED forest disagreed with the one the faces prove, each checked
    // against `REFUSE_RANK_LOST`. The tape no longer carries a forest. The
    // decoder runs `loops_of` on the faces it parsed, so `f.loops` IS the
    // walk and `Rank::moved(taped, derived)` compared a call with itself.
    //
    // The three attacks are not unguarded; they are unconstructible. A
    // forger can forge FACES, and `face_rederives` reads those. There is
    // nothing left to claim a rank with.
    let mut gone = crate::transducer::sentence::from_element_form(&e).expect("v2");
    gone.loops.clear();
    // RE-MINTED BY THE SAME SPEAKER (54.5 Road (d)). `as_element_form` takes
    // the attestation off the form, and a form that came out of an ADDRESS
    // carries none — so this minted a word that filed nothing and refused on
    // the filing rather than on the loops, which is not the claim. The
    // forgery being tested is about the FOREST, so everything else is held
    // fixed and `remint_form` is what holds it.
    let bad = crate::transducer::sentence::remint_form(&gone, [1, 0, 0, 0], &e)
        .expect("a tape from a form with no loops");
    assert_eq!(
        crate::transducer::sentence::from_element_form(&bad)
            .expect("it decodes")
            .loops
            .len(),
        f.loops.len(),
        "clearing the loops changes no byte of the tape: they were never on it"
    );
    assert_eq!(replay_form(&bad, &|_| None), None, "so the same form replays");

    // WHAT `REFUSE_RANK_LOST` STILL MEANS, and it is the reading that was
    // always underneath: the rank is UNMEASURABLE. A loop whose holonomy is
    // not a versor has no class (15.3), `rank()` answers `None`, and the
    // court refuses rather than counting it under a key of the rotor's
    // bytes — which is
    // `a_loop_whose_holonomy_is_not_a_versor_is_refused_not_keyed`.
    assert!(f.rank().is_some(), "this form's rank is measurable");
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
        crate::transducer::alphabet::Derivation::spelled(Word::identity()),
        stalk::Invariant::default(),
        Box::new(element::carriage(&one.clone(), &(zero.clone(), zero.clone()))),
        Vec::new(),
        None,
    ));
    re.vertices.push(Vertex::rebased_on(
        1,
        2,
        stalk::Invariant::default(),
        Box::new(element::carriage(&one.clone(), &(zero.clone(), zero.clone()))),
        Vec::new(),
    ));
    re.faces.push(re.face_of(0, 1).expect("vertices"));
    re.loops = Form::loops_of(&re.faces, &re.vertices);
    let re_e = as_element_form(&re, [1, 0, 0, 0]).expect("rebased tape");
    assert_eq!(
        replay_form(&re_e, &|_| None),
        Some(REFUSE_REBASE_TRIVIAL),
        "rebasing onto the identity equals the base"
    );

    let mut un = Form::default();
    un.unfold(Measure::Remainder);
    un.faces.push(FormFace::new(0, 0, Word::identity(),));
    un.loops = Form::loops_of(&un.faces, &un.vertices);
    let un_e = as_element_form(&un, [1, 0, 0, 0]).expect("unfold tape");
    assert_eq!(
        replay_form(&un_e, &|_| None),
        Some(REFUSE_UNFOLD_STILL),
        "an empty remainder does not unfold"
    );

    let mut orb = Form::default();
    orb.vertices.push(Vertex::word_on(
        crate::transducer::alphabet::Derivation::spelled(Word::identity()),
        stalk::Invariant::default(),
        Box::new(element::carriage(&one, &(zero.clone(), zero))),
        Vec::new(),
        Some(crate::transducer::alphabet::Contact { anchor: 0, ring: 1 }),
    ));
    orb.faces.push(orb.face_of(0, 1).expect("vertices"));
    orb.loops = Form::loops_of(&orb.faces, &orb.vertices);
    let orb_e = as_element_form(&orb, [1, 0, 0, 0]).expect("orbit tape");
    assert_eq!(
        replay_form(&orb_e, &|_| None),
        Some(REFUSE_ORBIT_STILL),
        "an identity word does not orbit"
    );
    // 50.9 (R1): the orbit's refusal names the contact vertex with the
    // chart it acts on, and travels as `OrbitStill`, never as a malformed
    // complex at the tape.
    let still = replay_form_at(&orb_e, &|_| None, None, &crate::transducer::family::NoCrystal)
        .expect("the still orbit refuses");
    assert_eq!(still.kind(), crate::transducer::sentence::RefusalKind::OrbitStill);
    // THE CONTACT VERTEX IS NAMED BY ITS OWN ADDRESS (54.2). It was
    // `[TAG_FORM, 3]`, a coordinate the fixture invented and handed to
    // `word_on`; a vertex now derives its address from its coordinates.
    let contact_at = crate::transducer::sentence::from_element_form(&orb_e)
        .expect("the orbit form decodes")
        .vertices
        .iter()
        .find_map(|v| match v {
            crate::transducer::sentence::Vertex::Word { address, contact: Some(_), .. } => {
                Some(address.clone())
            }
            _ => None,
        })
        .expect("the orbit form has a contact vertex");
    assert!(
        matches!(&still.at, crate::transducer::sentence::At::Vertex { address, .. } if *address == contact_at),
        "at the contact vertex: {:?}",
        still.at
    );

    // 50.5: a demand vertex names a kind that carries its coordinates. A
    // source no chart carries onto is a creation, not a source of
    // multiplicity zero; the court refuses the vertex by name.
    let mut dem = closed_turn_form();
    dem.vertices.push(Vertex::Demand {
        kind: crate::transducer::sentence::Demand::Source { multiplicity: 0 },
        chart: stalk::Invariant::counts(&[1, 2, 2]),
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
        chart: stalk::Invariant::counts(&[1, 2, 2]),
        bodies: vec![(3, 3, 0, 2)],
    });
    let typed_e = as_element_form(&typed, [1, 0, 0, 0]).expect("typed demand tape");
    assert_eq!(replay_form(&typed_e, &|_| None), None, "a typed demand vertex replays beside its words");
}

/// 50.3, 52.3 — AN EDGE TRANSPORT OF ANY ORDER IS A FACE. A (3,4,5) rotor
/// on an edge has no period and is not refused for that: with no family to
/// re-derive its transport against, the tape replays exactly as it does
/// with a quarter turn there. What an edge carries is judged on the family
/// (`edges_rederive_frames`), never by its order.
#[test]
fn a_face_whose_edge_word_has_infinite_order_is_not_refused_for_its_order() {
    use crate::transducer::alphabet::{Order, Word};
    use crate::transducer::sentence::as_element_form;
    let mut edged = closed_turn_form();
    let p5 = crate::transducer::alphabet::LetterId::pythagorean_5((0, 1)).expect("(3,4,5)");
    let fi = edged.faces.iter().position(|f| f.i == 0 && f.j == 1).expect("the star face");
    // The edge goes on every member pair of the face — one here.
    let set_edge = |f: &mut crate::transducer::sentence::Form, fi: usize, w: Word| {
        let pairs: Vec<Vec<u8>> = f.faces[fi].at.scoped().map(|(k, _)| k).collect();
        for k in pairs {
            if let Some(p) = f.faces[fi].at.get_mut(&k) {
                p.edge = Some(crate::transducer::sentence::Transport::from_word(w.clone()));
            }
        }
    };
    let tilt = Word::from_letters(&[(p5, false)]);
    assert_eq!(tilt.order(), Order::Infinite, "the (3,4,5) rotor has no period");
    set_edge(&mut edged, fi, tilt);
    let edged_e = as_element_form(&edged, [1, 0, 0, 0]).expect("edged tape");
    assert_eq!(replay_form(&edged_e, &|_| None), None, "an edge of infinite order is not refused for its order");
    let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
    set_edge(&mut edged, fi, Word::from_letters(&[(q, false)]));
    let fine_e = as_element_form(&edged, [1, 0, 0, 0]).expect("edged tape");
    assert_eq!(replay_form(&fine_e, &|_| None), None, "a quarter turn on an edge is a stabilizer element and replays");
}

/// 52.3 — A LOOP OF INFINITE ORDER REPLAYS ON ITS FILING. The transports
/// are coboundaries and compose to the identity while the body face carries
/// a (3,4,5) rotor as its edge: unfiled, the loop does not replay; filed
/// under the edge holonomy's class at period 0, A19's spelling of infinite
/// order, it replays — through the tape, where the court reads the zero —
/// and at a period the loop does not have it is refused by name.
#[test]
fn a_filed_loop_of_infinite_order_replays_and_a_wrong_period_does_not() {
    use crate::transducer::alphabet::{LetterId, Order, Word};
    use crate::transducer::sentence::{as_element_form, FiledLoop, Form, FormFace, Transport};
    let p5 = LetterId::pythagorean_5((0, 1)).expect("(3,4,5)");
    let tilt = Word::from_letters(&[(p5, false)]);
    let edge_of = |w: &Word| Transport {
        word: w.clone(),
        carriage: element::identity_versor(),
        gauge: Vec::new(),
    };
    let mut f = Form::default();
    for tag in [1u8, 2u8] {
        f.vertices.push(Vertex::word_on(
            crate::transducer::alphabet::Derivation::spelled(Word::identity()),
            // Its own chart: an address is derived from its coordinates (54.2).
            stalk::Invariant::counts(&[u64::from(tag)]),
            Box::new(element::identity_versor()),
            Vec::new(),
            None,
        ));
    }
    let with_edge = |mut face: FormFace, edge: Transport| -> FormFace {
        let pairs: Vec<Vec<u8>> = face.at.scoped().map(|(k, _)| k).collect();
        for k in pairs {
            if let Some(p) = face.at.get_mut(&k) {
                p.edge = Some(edge.clone());
            }
        }
        face
    };
    for (i, j) in [(0u16, 1u16), (0, 2)] {
        let face = f.face_of(i, j).expect("a coboundary");
        f.faces.push(with_edge(face, edge_of(&Word::identity())));
    }
    let face = f.face_of(1, 2).expect("a coboundary");
    f.faces.push(with_edge(face, edge_of(&tilt)));
    f.loops = Form::loops_of(&f.faces, &f.vertices);
    f.reindex();
    let [lp, ..] = f.loops.as_slice() else { panic!("the triangle closes") };
    assert!(lp.holonomy.is_identity(), "the transport composite is flat");
    let es = f.edge_holonomies(lp).expect("the loop names live faces");
    let [(_, e)] = es.scoped().collect::<Vec<_>>()[..] else { panic!("one member path") };
    assert_eq!(e.word.order(), Order::Infinite, "and its edge holonomy has no period");
    // A FREE PART DOES NOT REPLAY, AND A FILING DOES NOT CHANGE THAT
    // (54.5, as ruled 2026-09-20). The edge holonomy has NO period: `g^ℓ`
    // is never the identity, so the circuit walks. It read that filing the
    // class at 0 made the loop replay, which is a permit deciding a
    // geometric question — every field of a `FiledLoop` is derivable
    // (`loop_classes`), so by law 12 the annotation carries no fact the
    // complex does not already determine.
    assert!(!form_loops_replay(&f), "a free part does not replay");
    let classes: Vec<_> = f.edge_classes(lp).scoped().map(|(_, c)| c.clone()).collect();
    let [class] = classes.as_slice() else { panic!("one member path, so one class") };
    let mut filed = f.clone();
    filed.torsion.put(
        class,
        FiledLoop { sigma: Vec::new(), periods: ToralPeriod::OPEN, chord: lp.chord },
    );
    assert!(!form_loops_replay(&filed), "and filing it as open does not close a circuit that walks");
    let filed_e = as_element_form(&filed, [1, 0, 0, 0]).expect("the tape carries the filed zero");
    assert_eq!(
        replay_form(&filed_e, &|_| None),
        Some(REFUSE_LOOP),
        "the court refuses it on the geometry, by name"
    );
    // AND THE RECEIPT IS PRICED SEPARATELY. The complex derives 0 for this
    // class; a receipt claiming 4 is a claim ABOUT THE TERRAIN, and the
    // terrain contradicts it. The geometric refusal comes first because it
    // is asked first — the labour is only priced on terrain that stands.
    let mut wrong = f.clone();
    wrong.torsion.put(
        &class,
        FiledLoop { sigma: Vec::new(), periods: ToralPeriod::from_pair(4, 4), chord: lp.chord },
    );
    assert_eq!(
        receipts_price_the_terrain(&wrong, &wrong.torsion),
        Some(REFUSE_INFLATED_YIELD),
        "a period the complex does not derive is perjury, not pruning"
    );
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
    let word = mint_sentence(&rule, &one_pair, &NoCrystal, 0).expect("it closes its own pair");
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
        derivation,
        stalk::Invariant::counts(&[7]),
        Box::new(element::carriage(&one, &(zero.clone(), zero))),
        vec![(1, 2)],
        None,
    ));
    f.faces.push(f.face_of(0, 1).expect("vertices"));
    f.loops = Form::loops_of(&f.faces, &f.vertices);
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
            crate::transducer::alphabet::Derivation::spelled(Word::identity()),
            // Its own chart: an address is derived from its coordinates (54.2).
            stalk::Invariant::counts(&[u64::from(tag)]),
            Box::new(element::identity_versor()),
            Vec::new(),
            None,
        ));
    }
    // The two star faces and the body-to-body face, every transport the
    // coboundary of its vertices; the body face carries the half turn as
    // the transition between the two pieces' own frames.
    // THE EDGE IS SET AT EVERY MEMBER PAIR the face carries — one here, since
    // each of these vertices stands at one chart.
    let with_edge = |mut face: FormFace, edge: Transport| -> FormFace {
        let pairs: Vec<Vec<u8>> = face.at.scoped().map(|(k, _)| k).collect();
        for k in pairs {
            if let Some(p) = face.at.get_mut(&k) {
                p.edge = Some(edge.clone());
            }
        }
        face
    };
    for (i, j) in [(0u16, 1u16), (0, 2)] {
        let face = f.face_of(i, j).expect("a coboundary");
        f.faces.push(with_edge(face, edge_of(&Word::identity())));
    }
    let face = f.face_of(1, 2).expect("a coboundary");
    f.faces.push(with_edge(face, edge_of(&half)));
    f.loops = Form::loops_of(&f.faces, &f.vertices);
    f.reindex();
    let [lp, ..] = f.loops.as_slice() else { panic!("the triangle closes") };
    assert!(
        lp.holonomy.is_identity(),
        "the transport composite is flat: every face is a coboundary"
    );
    let es = f.edge_holonomies(lp).expect("the loop names live faces");
    let [(_, e)] = es.scoped().collect::<Vec<_>>()[..] else {
        panic!("one member path, since each vertex stands at one chart")
    };
    assert!(!e.word.is_identity(), "and its edge holonomy is the half turn");

    // A COMPACT EDGE HOLONOMY REPLAYS ON ITS OWN ALGEBRA (54.5, as ruled
    // 2026-09-20). The half turn returns after two circuits about a fixed
    // point: `ℤ/2ℤ`, which is torsion, and torsion is content (law 7).
    //
    // This asserted the loop did NOT replay until a class was filed, which
    // made an annotation's presence a geometric property. What the
    // transport reading still cannot say is the interesting half, and it
    // stands: on the transport composite this loop is FLAT, and only the
    // edge holonomy sees the half turn (50.3, `edge_class_vector`).
    assert!(
        form_loops_replay(&f),
        "a compact edge holonomy replays unfiled: ℤ/2ℤ is a resonance, not a debt"
    );
    // One member path here, so one class — said by the slice pattern.
    let classes: Vec<_> = f.edge_classes(lp).scoped().map(|(_, c)| c.clone()).collect();
    let [class] = classes.as_slice() else {
        panic!("one member path, so one class; got {}", classes.len())
    };
    let class = class.clone();
    let mut filed = f.clone();
    filed.torsion.put(
        &class,
        FiledLoop {
            // THE SPECTRUM THE COMPLEX ITSELF DERIVES (54.7). Not the
            // loop's holonomy rotor: `loop_classes` reads the period off
            // each MEMBER PATH's word, and a receipt is honest when it
            // matches what the terrain derives — which is the whole point
            // of the terrain/labour split. Taking it from the loop's
            // composite instead was filing one rotor's spectrum against
            // another's, and `REFUSE_INFLATED_YIELD` caught it, correctly.
            sigma: Vec::new(),
            periods: match f.loop_classes(lp).as_slice() {
                [row, ..] => row.periods,
                [] => panic!("the loop carries a class"),
            },
            chord: lp.chord,
        },
    );
    assert!(form_loops_replay(&filed), "and filing it says nothing the geometry did not");
    assert_eq!(
        receipts_price_the_terrain(&filed, &filed.torsion),
        None,
        "a receipt whose period the complex derives is honest labour"
    );
    // THE TWO HALVES SEPARATE HERE, and this is the whole point of the
    // split. At the wrong period the GEOMETRY is unchanged — the loop is
    // still a compact ℤ/2ℤ and still replays — while the RECEIPT is now a
    // claim about the terrain that the terrain contradicts. The period is
    // the loop's order, not the filer's opinion of it (A17), and it is the
    // one lie in an envelope the court can catch.
    let mut wrong = f.clone();
    wrong.torsion.put(
        &class,
        FiledLoop { sigma: Vec::new(), periods: ToralPeriod::from_pair(3, 3), chord: lp.chord },
    );
    assert!(form_loops_replay(&wrong), "the terrain is what it is, whatever is claimed of it");
    assert_eq!(
        receipts_price_the_terrain(&wrong, &wrong.torsion),
        Some(REFUSE_INFLATED_YIELD),
        "and the claim is refused as perjury about a period"
    );

    // TWO CLAIMS WITH ONE ENVELOPE AND TWO YIELDS (54.7). This is the
    // reading that only exists because the carrier is a spectrum.
    //
    //     [12, 1]  →  (12−1) + (1−1)  =  11 units
    //     [ 3, 4]  →  ( 3−1) + (4−1)  =   5 units
    //
    // `lcm(12, 1) = lcm(3, 4) = 12`, so under the old scalar these two
    // claims were the SAME number and the court could not tell them apart —
    // a speaker claiming `[12, 1]` on terrain that derives `[3, 4]` was
    // claiming twice the labour and passing. The eigenspaces differ: one is
    // a planar rotation with an idle orthogonal plane, the other a genuine
    // double rotation doing work on both.
    assert_eq!(ToralPeriod::from_pair(12, 1).yield_units(), Some(11));
    assert_eq!(ToralPeriod::from_pair(3, 4).yield_units(), Some(5));
    assert_ne!(
        ToralPeriod::from_pair(12, 1),
        ToralPeriod::from_pair(3, 4),
        "one lcm, two spectra, two yields — and the court compares the spectrum"
    );
    let mut colliding = f.clone();
    colliding.torsion.put(
        &class,
        FiledLoop { sigma: Vec::new(), periods: ToralPeriod::from_pair(12, 1), chord: lp.chord },
    );
    assert_eq!(
        receipts_price_the_terrain(&colliding, &colliding.torsion),
        Some(REFUSE_INFLATED_YIELD),
        "a claim that agrees on the envelope and differs on the planes is still perjury"
    );

    // AND A PLANE THAT DOES NOT CLOSE EARNS NOTHING. That is torsion, and
    // it is charged as shear — never estimated through an invented finite
    // order (rule 11).
    assert_eq!(ToralPeriod::from_pair(5, 0).yield_units(), None);
    assert!(!ToralPeriod::from_pair(5, 0).closes());
    assert_eq!(ToralPeriod::OPEN.yield_units(), None);
}
