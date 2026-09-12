// 37.7 P2–P3 — THE COURT OPENS A LETTER. A prime outside the seventeen
// arrives as a witness on a demand; the factor is read off it by Euclid
// over ℤ[i], docketed as a provisional generator, and installed when a
// second organ reaches the same address. The adjoined count leaves zero.

use crate::cognition::*;
use crate::transducer::address::{Address, TAG_FORM};
use crate::transducer::alphabet::Derivation;
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::{as_element_form, Demand, Form, FormFace, Vertex};
use num_rational::BigRational;

fn q(n: i64) -> BigRational {
    BigRational::from_integer(num_bigint::BigInt::from(n))
}

/// The witness block `[[s, −t], [t, s]]` of the arc `s + t·i`.
fn witness(s: i64, t: i64) -> Box<[[BigRational; 2]; 2]> {
    Box::new([[q(s), -q(t)], [q(t), q(s)]])
}

/// A sentence that ACTS and carries a `RotorPrime` demand beside its word
/// vertex: a demand rides a sentence (50.5) and never travels alone.
fn demanding_sentence(w: crate::transducer::alphabet::Word, tag: u8, kind: Demand) -> Element {
    let one = q(1);
    let zero = q(0);
    let mut f = Form::default();
    f.vertices.push(Vertex::Word {
        address: Address::from_bytes([TAG_FORM, tag]),
        derivation: Derivation::spelled(w),
        carriage: Box::new(element::carriage(&one, &(zero.clone(), zero))),
        gauge: Vec::new(),
        class: crate::transducer::stalk::Invariant::default(),
        contact: None,
    });
    f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("star"), edge: None });
    f.vertices.push(Vertex::Demand {
        kind,
        chart: crate::transducer::stalk::Invariant(vec![17]),
        // The cells the demand stands for, and they MOVE: a demand
        // whose bodies measure nothing cost the language nothing.
        bodies: vec![(0, 0, 0, 1), (0, 1, 0, 1)],
    });
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f.reindex();
    as_element_form(&f, [1, 0, 0, 0]).expect("a tape")
}

fn quarter(n: usize) -> crate::transducer::alphabet::Word {
    let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("the quarter turn");
    crate::transducer::alphabet::Word::from_letters(&vec![(q, false); n])
}

/// 17 = 4² + 1², a prime the seventeen letters do not spell.
fn seventeen() -> Demand {
    Demand::RotorPrime { plane: (0, 1), prime: 17, witness: witness(4, 1) }
}

/// 37.7 P2–P3, THE FIRST PROOF, IN PROCESS AND WITH NO CORPUS. Two
/// voices demand the same prime on witnesses of it; two courts of
/// distinct identity each read the factor off the witness by
/// `core::gaussian` and docket it. Both reach ONE address, because the
/// generator is a form and its address is `form_vertex_key` over the
/// factor's canonical rotor. Arity 2 installs it, and the count of
/// generators the ENGINE opened leaves zero on each court.
#[test]
fn two_courts_open_one_letter_from_two_witnesses_and_the_count_leaves_zero() {
    let (voice_a, voice_b) = ([1u8; 32], [2u8; 32]);
    let said_a = demanding_sentence(quarter(1), 1, seventeen());
    let said_b = demanding_sentence(quarter(3), 3, seventeen());

    // The premise, asserted rather than assumed: the factor's rotor is a
    // versor the seventeen cannot spell. If A22 could spell it there
    // would be no letter to open, and the whole row would be moot.
    let pi = crate::transducer::alphabet::Rotor::scalar(q(4))
        .add(&crate::transducer::alphabet::Rotor::blade(0b0011, q(1)));
    assert!(
        !matches!(crate::transducer::alphabet::factor_rotor(&pi), Some(Ok(_))),
        "17 is outside the seventeen's span: {:?}",
        crate::transducer::alphabet::factor_rotor(&pi)
    );

    let mut opened = Vec::new();
    for (court, voice, said) in [([21u8; 32], voice_a, &said_a), ([22u8; 32], voice_b, &said_b)] {
        let mut k = Kernel::new();
        k.set_court_voice(court);
        assert_eq!(k.adjoined(), 0, "a court that has heard nothing has opened nothing");
        assert_eq!(
            k.perceive_speech(voice, &[0; 32], std::slice::from_ref(said)),
            vec![crate::anastome::dialogue::SPEECH_STOOD],
            "the sentence stands and its demand is listed"
        );
        // The letter is on the docket, derived from the witness.
        let provisional: Vec<Vec<u8>> = k
            .provisional
            .fiber(crate::network::nerve::ROOT)
            .map(|(a, _)| a)
            .collect();
        let letter = provisional
            .iter()
            .find(|a| {
                k.provisional.get(a).is_some_and(|e| {
                    crate::transducer::sentence::from_element_form(e).is_some_and(|f| {
                        f.vertices.iter().any(|v| match v {
                            Vertex::Word { derivation, .. } => {
                                derivation.generators.is_empty()
                                    && !matches!(
                                        crate::transducer::alphabet::factor_rotor(&derivation.rotor()),
                                        Some(Ok(_))
                                    )
                            }
                            _ => false,
                        })
                    })
                })
            })
            .cloned()
            .expect("the court opened a letter on the witness");

        // Arity 2: a second organ reaches the same address and the home
        // installs it under the ruling path that already exists.
        k.rule_all_provisionals();
        let cover = k.genesis_cover().expect("measured");
        k.enroll([9; 32], cover);
        k.confirm_peer_holds_addrs(&[Address::from(letter.clone())], [9; 32]);
        assert!(k.ruled_contains(&letter), "arity 2 installs the letter on the ruled cover");

        assert_eq!(
            k.adjoined(),
            1,
            "and the count of generators the engine opened leaves zero"
        );
        assert_eq!(
            k.discoverer_of(&letter),
            Some(voice),
            "the demanding voice is its discoverer; a court is never one"
        );
        opened.push(letter);
    }

    // ONE ADDRESS on both courts: the factor is canonical (A1 at the
    // altitude of ℤ[i]), so two organs that each ran Euclid reach one
    // form and P3's consilience is on content.
    assert_eq!(opened[0], opened[1], "two witnesses, two courts, one letter");
}

/// 37.7 P2's refusal. A witness whose gcd with the prime is not of norm
/// `p` names a prime it does not carry: refused AT THE DEMAND VERTEX,
/// before anything is docketed, so no letter is opened on it.
#[test]
fn a_witness_that_does_not_carry_its_prime_is_refused_at_the_vertex() {
    let mut k = Kernel::new();
    // 2 + i has norm 5 and is coprime to 17.
    let bad = Demand::RotorPrime { plane: (0, 1), prime: 17, witness: witness(2, 1) };
    let said = demanding_sentence(quarter(1), 1, bad);
    assert_eq!(
        k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&said)),
        vec![crate::anastome::dialogue::SPEECH_REFUSED],
        "the demand claims a prime its witness does not carry"
    );
    assert_eq!(k.adjoined(), 0, "and no letter is opened on it");
    let refused = k.refusals.get(said.address().as_bytes()).expect("a receipt");
    assert!(
        matches!(refused.at, crate::transducer::sentence::At::Vertex { .. }),
        "refused at the demand vertex, with its coordinate (50.9)"
    );
}

/// A one-word sentence whose vertex carries `d`.
///
/// NO STAR FACE, and that is the vertex's own doing: the face to the Frame
/// is this vertex's action, and a vertex derived through a letter acts by
/// a rotor the seventeen cannot spell, so `face_of` has no transport word
/// to give it. `letter_of` in `family/tests/derivation.rs` says the same
/// at its own line (L11 2/2). The claim here is about the derivation, and
/// a face invented for it would be a claim about a spelling that does not
/// exist.
fn derived_sentence(d: Derivation, tag: u8) -> Element {
    let (one, zero) = (q(1), q(0));
    let mut f = Form::default();
    f.vertices.push(Vertex::Word {
        address: Address::from_bytes([TAG_FORM, tag]),
        derivation: d,
        carriage: Box::new(element::carriage(&one, &(zero.clone(), zero))),
        gauge: Vec::new(),
        class: crate::transducer::stalk::Invariant::default(),
        contact: None,
    });
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f.reindex();
    as_element_form(&f, [1, 0, 0, 0]).expect("a tape")
}

/// The versor whose sensor block is the arc `s + t·i`. A rotor
/// `x + y·e01` has block arc `(x² − y²) + 2xy·i`, so the arc is reached
/// from its square root in ℤ[i].
fn versor_of_arc(s: i64, t: i64) -> element::Versor {
    use crate::transducer::alphabet::{Rotor, Word};
    let r = Rotor::scalar(q(s)).add(&Rotor::blade(0b0011, q(t)));
    element::versor_of_word(&Word::formed(r)).expect("a plane versor")
}

/// L11, THE WHOLE CHAIN AS ONE CLAIM — the row's three facts joined end to
/// end, each one's output the next one's input, with no fixture between
/// them:
///
///   1. A COURT OPENS a letter the seventeen cannot spell, from the witness
///      on a voice's demand, and rules it (37.7 P2–P3).
///   2. THE SYLLABUS CARRIES IT and a mind learns it from the page's own
///      bytes — not by insertion, so what is proved is that it travels.
///   3. THE MIND DERIVES THROUGH IT: a versor whose arc is π₁₇², which A22
///      refuses, factors against that mind's presentation and CITES the
///      letter at multiplicity two (50.2 step 2).
///   4. A COURT COMPOSES THE CITATION: the derived sentence is spoken to
///      the court that ruled the letter, and `blind_replay`'s
///      `derivations_compose` resolves the citation through its own ruled
///      cover and agrees. The sentence STANDS.
///
/// EACH HALF WAS ALREADY PROVED AND THE JOIN WAS NOT. The three tests this
/// sits beside prove the opening, the teaching, and — in
/// `family/tests/derivation.rs` — a derivation through a letter against a
/// hand-built `Holds` crystal verified by a hand-built cover closure. What
/// none of them proved is that the SAME letter survives every hop: the
/// court's address, the page's bytes, the mind's index, the derivation's
/// citation, and the court's resolution of that citation. A break at any
/// hop leaves every existing test green.
#[test]
fn a_mind_taught_by_the_page_derives_through_the_letter_and_a_court_composes_it() {
    use crate::client::sdk::Epigenome;
    let voice = [1u8; 32];
    let said = demanding_sentence(quarter(1), 1, seventeen());

    // 1. THE COURT OPENS IT, from the witness and nothing else.
    let mut k = Kernel::new();
    assert_eq!(
        k.perceive_speech(voice, &[0; 32], std::slice::from_ref(&said)),
        vec![crate::anastome::dialogue::SPEECH_STOOD]
    );
    k.rule_all_provisionals();
    let cover = k.genesis_cover().expect("measured");
    k.enroll([9; 32], cover);
    let ruled: Vec<Vec<u8>> =
        k.ruled_addrs.fiber(crate::network::nerve::ROOT).map(|(a, _)| a).collect();
    for a in &ruled {
        k.confirm_peer_holds_addrs(&[Address::from(a.clone())], [9; 32]);
    }
    assert_eq!(k.adjoined(), 1, "the court opened exactly one letter");

    // 2. THE PAGE CARRIES IT and a mind learns it from the bytes.
    let page = k.serve_ruled(&crate::transducer::stalk::Invariant(Vec::new()), &[], 1 << 20);
    let mut epi = Epigenome::new();
    for bytes in &page {
        let _ = epi.learn(bytes);
    }

    // 3. THE MIND DERIVES THROUGH IT. (4 + i)² = 15 + 8i, so the target's
    //    arc is π₁₇² and the peel divides it out twice. The premise is
    //    asserted first: a mind that holds NOTHING cannot spell this, so
    //    what the taught mind does is the letter's doing and not the
    //    alphabet's.
    let target = versor_of_arc(15, 8);
    let bare = crate::transducer::family::factor_versor(
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(1 << 20),
        &target,
        &[],
    )
    .expect("a linear block");
    assert!(bare.is_err(), "the seventeen cannot spell π₁₇²: {bare:?}");

    let derived = crate::transducer::family::factor_versor(
        &epi,
        &mut crate::physics::Physics::default().stroke_n(1 << 20),
        &target,
        &[],
    )
    .expect("a linear block")
    .expect("the taught mind reaches what the alphabet cannot");
    assert!(!derived.is_spelled(), "it cites the letter it was taught");
    // ONE CITATION, so the derivation carries one generator.
    let [(cited_form, _, exponent)] = derived.generators.as_slice() else {
        panic!("one citation")
    };
    assert_eq!(*exponent, 2, "at the multiplicity the arc carries");
    let cited_form = cited_form.clone();
    assert!(
        ruled.iter().any(|a| a.as_slice() == cited_form.as_bytes()),
        "and the form it cites is the one THIS court ruled — the address survived the page"
    );
    assert!(derived.residue.is_identity(), "nothing is left for the alphabet");

    // 4. THE COURT COMPOSES IT. Spoken to the court that ruled the letter:
    //    `blind_replay` resolves the citation through its own ruled cover,
    //    re-derives the rotor, and agrees. A court that had not ruled the
    //    letter would refuse at this vertex, which is what makes standing
    //    here a fact about the LANGUAGE and not about the arithmetic.
    let speaks = derived_sentence(derived, 5);
    // A court with NO cover cannot resolve the citation, and says so. This
    // is the control: it is what standing below is measured against, and
    // it is why standing is a fact about this court's ruled language.
    assert_eq!(
        crate::court::verdict::blind_replay(&speaks),
        Some(crate::court::verdict::REFUSE_DERIVATION),
        "a court that resolves nothing cannot compose the citation"
    );
    assert_eq!(
        crate::court::verdict::blind_replay_on(
            &speaks,
            &|a| k.resolve_form(a),
            None,
            &crate::transducer::family::NoCrystal,
        ),
        None,
        "the court that ruled the letter re-derives the citation through it and agrees"
    );
    assert_eq!(
        k.perceive_speech([3u8; 32], &[0; 32], std::slice::from_ref(&speaks)),
        vec![crate::anastome::dialogue::SPEECH_STOOD],
        "and the sentence that derives through the letter stands on that court"
    );
}

/// 37.7 — THE SYLLABUS TEACHES THE LETTER. A court that ruled a letter
/// serves it as any ruled form; the mind learns it from the page's own
/// bytes, files it under the versor chart of its rotor, and answers it as
/// a generator to a versor whose norm the prime divides.
///
/// Taught through the page, not by direct insertion: what is proved is
/// that the letter travels, which is the whole of what a syllabus is for.
#[test]
fn the_syllabus_teaches_a_letter_and_the_mind_then_holds_it_as_a_generator() {
    use crate::client::sdk::Epigenome;
    use crate::transducer::alphabet::Rotor;
    use crate::transducer::element::Element as El;
    let voice = [1u8; 32];
    let said = demanding_sentence(quarter(1), 1, seventeen());

    let mut k = Kernel::new();
    assert_eq!(
        k.perceive_speech(voice, &[0; 32], std::slice::from_ref(&said)),
        vec![crate::anastome::dialogue::SPEECH_STOOD]
    );
    k.rule_all_provisionals();
    let cover = k.genesis_cover().expect("measured");
    k.enroll([9; 32], cover);
    let ruled: Vec<Vec<u8>> = k.ruled_addrs.fiber(crate::network::nerve::ROOT).map(|(a, _)| a).collect();
    for a in &ruled {
        k.confirm_peer_holds_addrs(&[Address::from(a.clone())], [9; 32]);
    }
    assert_eq!(k.adjoined(), 1, "the court holds one letter");

    // THE PAGE. Served from the root as an empty mind is seeded, and
    // learned from its bytes.
    let page = k.serve_ruled(&crate::transducer::stalk::Invariant(Vec::new()), &[], 1 << 20);
    assert!(!page.is_empty(), "the syllabus serves what the court ruled");
    let mut epi = Epigenome::new();
    for bytes in &page {
        let _ = epi.learn(bytes);
    }

    // The mind now holds the letter as a GENERATOR, at the chart a versor
    // carrying that prime reads.
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let target = Rotor::scalar(q(15)).add(&Rotor::blade(0b0011, q(8)));
    let chart = crate::transducer::sentence::versor_chart_of(&target).expect("a chart");
    let held = crate::transducer::family::Crystal::generators_under(
        &epi,
        &chart,
        &mut crate::physics::Physics::default().stroke(),
    );
    assert!(
        held.iter().any(|(_, _, g)| {
            g.letters().is_empty()
                && !matches!(crate::transducer::alphabet::factor_rotor(&g.rotor()), Some(Ok(_)))
        }),
        "the taught letter is offered as a generator the alphabet cannot spell"
    );

    // And it is the one the court ruled, by address: the page carried the
    // letter itself and not a re-derivation of it.
    let taught: Vec<Vec<u8>> = page
        .iter()
        .filter_map(|b| {
            let mut p = 0usize;
            El::from_bytes(b, &mut p).map(|e| e.address().into_vec())
        })
        .collect();
    assert!(
        ruled.iter().any(|a| taught.contains(a)),
        "the page carries an address this court ruled"
    );
}
