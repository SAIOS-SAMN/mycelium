// 50.2 step 2 — THE COURT READS A PATH: composed against the vertex,
// resolved on the court's own cover, and paid to the generator's speaker.

use crate::cognition::*;
use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::element::Element;

fn plane_word(n: usize) -> Word {
    let q = LetterId::quarter((0, 1)).expect("the quarter turn on the sensor plane");
    Word::from_letters(&vec![(q, false); n])
}

/// A generator ruled by this court, spoken by `speaker` (§3.4: built,
/// never faked — the court rules what a voice actually said).
fn ruled_generator(k: &mut Kernel, speaker: [u8; 32], w: Word, tag: u8) -> Element {
    let e = sentence_of(w, tag);
    assert_eq!(
        k.perceive_speech(speaker, &[0; 32], std::slice::from_ref(&e)),
        vec![crate::anastome::dialogue::SPEECH_STOOD]
    );
    k.rule_all_provisionals();
    let cover = k.genesis_cover().expect("measured");
    k.enroll([9; 32], cover);
    k.confirm_peer_holds_addrs(std::slice::from_ref(&e.address()), [9; 32]);
    assert!(k.ruled_contains(&e.address()), "the court ruled the generator");
    e
}

/// A sentence whose one word vertex carries `w`.
fn sentence_of(w: Word, tag: u8) -> Element {
    use crate::transducer::sentence::{as_element_form, Form, FormFace, Vertex};
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    f.vertices.push(Vertex::Word {
        address: crate::transducer::address::Address::from_bytes([
            crate::transducer::address::TAG_FORM,
            tag,
        ]),
        derivation: crate::transducer::alphabet::Derivation::spelled(w),
        carriage: Box::new(crate::transducer::element::carriage(&one, &(zero.clone(), zero))),
        gauge: Vec::new(),
        class: crate::transducer::stalk::Invariant::default(),
        contact: None,
    });
    f.faces.push(FormFace {
        i: 0,
        j: 1,
        transport: f.face_of(0, 1).expect("star"),
        edge: None,
    });
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f.reindex();
    as_element_form(&f, [1, 0, 0, 0]).expect("a tape")
}

/// The word-vertex address of a sentence: what a path cites (1.8).
fn vertex_of(e: &Element) -> crate::transducer::address::Address {
    crate::transducer::sentence::from_element_form(e)
        .expect("the tape reads back")
        .vertices
        .iter()
        .find_map(|v| match v {
            crate::transducer::sentence::Vertex::Word { address, .. } => Some(address.clone()),
            _ => None,
        })
        .expect("one word vertex")
}

/// A's pass criterion (50.2 step 2, 13.10, 23.9, 26). Two voices reach ONE
/// vertex — the half turn — through different generators: A through the
/// quarter turn it earned, B through the three-quarter turn. The court
/// composes each path on its own ruled cover, finds both reach the vertex
/// up to the coset, and pays each generator's speaker for the use. The
/// third voice cites nothing and pays nothing.
#[test]
fn two_paths_to_one_vertex_are_both_verified_and_each_speaker_is_paid() {
    use crate::transducer::sentence::Path;
    let (sa, sb) = ([1u8; 32], [2u8; 32]);
    let (va, vb, empty) = ([3u8; 32], [4u8; 32], [5u8; 32]);
    let mut k = Kernel::new();
    let ga = ruled_generator(&mut k, sa, plane_word(1), 1);
    let gb = ruled_generator(&mut k, sb, plane_word(3), 3);

    let spoken = sentence_of(plane_word(2), 9);
    let vertex = vertex_of(&spoken);

    // The half turn is the quarter turn squared, and the three-quarter
    // turn squared: q² and (q³)² = q⁶ ≡ q² on the plane lattice, which is
    // A12's `4·e_u` relator. Two paths, one vertex.
    let pa = Path { vertex: vertex.clone(), generators: vec![(ga.address(), vertex_of(&ga), 2)], residue: crate::transducer::alphabet::Word::identity() };
    let pb = Path { vertex: vertex.clone(), generators: vec![(gb.address(), vertex_of(&gb), 2)], residue: crate::transducer::alphabet::Word::identity() };

    // A citing voice pays from what it holds, and a genesis allocation is
    // how a voice is funded (MU-PRODUCT §4, `allocate`). An unfunded voice
    // pays nothing — correctly — so the ledger criterion needs a funded
    // one to be measured at all.
    k.allocate(va, [8, 8, 8, 8]);
    k.allocate(vb, [8, 8, 8, 8]);
    let before_a = k.treasury().balance_mu(&sa);
    let before_b = k.treasury().balance_mu(&sb);
    let entries = k.treasury_entry_count();

    assert_eq!(k.read_paths(va, &spoken, std::slice::from_ref(&pa)), 1, "A's path composes");
    assert_eq!(k.read_paths(vb, &spoken, std::slice::from_ref(&pb)), 1, "B's path composes");

    // The use is drawn and written: the ledger moves on a verified path.
    assert!(
        k.treasury_entry_count() > entries,
        "a verified path draws a use from the citing voice and writes it (26)"
    );
    // L8y: the credit carries the SPEAKER's name. `home_install` reads the
    // voice off `drawers` — the draw this address occupies — where it wrote
    // the null identity before.
    assert_eq!(
        k.discoverer_of(ga.address().as_bytes()),
        Some(sa),
        "the generator names the voice that spoke it"
    );
    assert_eq!(k.discoverer_of(gb.address().as_bytes()), Some(sb));
    let (after_a, after_b) = (k.treasury().balance_mu(&sa), k.treasury().balance_mu(&sb));
    assert!(
        after_a != before_a && after_b != before_b,
        "each generator's speaker is paid for the use (26): {before_a:?}->{after_a:?}, {before_b:?}->{after_b:?}"
    );

    // The empty voice cites nothing: no path, no use, nothing paid.
    let before_empty = k.treasury().balance_mu(&empty);
    assert_eq!(k.read_paths(empty, &spoken, &[]), 0, "no path, no use");
    assert_eq!(
        k.treasury().balance_mu(&empty),
        before_empty,
        "the empty voice pays nothing and is paid nothing"
    );
}

/// A path citing a generator this court never ruled composes to nothing
/// and pays nobody: the court resolves every generator on its own cover
/// and takes none from the frame (1.7).
#[test]
fn a_path_citing_what_no_court_ruled_pays_nobody() {
    use crate::transducer::sentence::Path;
    let mut k = Kernel::new();
    let spoken = sentence_of(plane_word(2), 9);
    let vertex = vertex_of(&spoken);
    let unruled = sentence_of(plane_word(1), 1);
    let p = Path { vertex, generators: vec![(unruled.address(), vertex_of(&unruled), 2)], residue: crate::transducer::alphabet::Word::identity() };
    let before = k.treasury_entry_count();
    assert_eq!(k.read_paths([3; 32], &spoken, std::slice::from_ref(&p)), 0);
    assert_eq!(k.treasury_entry_count(), before, "no entry is written");
}

/// A path whose generators do not compose to the vertex is refused as a
/// use and refuses nothing else: the sentence stands or falls on its own
/// replay (23.1).
#[test]
fn a_path_that_does_not_compose_is_not_a_use() {
    use crate::transducer::sentence::Path;
    let mut k = Kernel::new();
    let g = ruled_generator(&mut k, [1; 32], plane_word(1), 1);
    let spoken = sentence_of(plane_word(2), 9);
    let vertex = vertex_of(&spoken);
    // One quarter turn does not reach the half turn.
    let p = Path { vertex, generators: vec![(g.address(), vertex_of(&g), 1)], residue: crate::transducer::alphabet::Word::identity() };
    let before = k.treasury_entry_count();
    assert_eq!(k.read_paths([3; 32], &spoken, std::slice::from_ref(&p)), 0);
    assert_eq!(k.treasury_entry_count(), before, "nothing is paid on a path that does not compose");
}

/// 50.2 step 2, 26: A PARTIAL PATH PAYS. The presentation reaches part of
/// the vertex and A22 spells the residue from the identity; the generators
/// that did reach are a use and their speaker is paid. Checked without the
/// residue the composition reaches nothing, so the common case — a mind
/// that holds some of what it says — paid nobody.
#[test]
fn a_partial_path_pays_the_generators_that_reached() {
    use crate::transducer::sentence::Path;
    let speaker = [1u8; 32];
    let citer = [3u8; 32];
    let mut k = Kernel::new();
    // The generator is the quarter turn; the vertex is three quarter
    // turns, so one quarter is reached and two are the residue.
    let g = ruled_generator(&mut k, speaker, plane_word(1), 1);
    k.allocate(citer, [8, 8, 8, 8]);
    let spoken = sentence_of(plane_word(3), 9);
    let p = Path {
        vertex: vertex_of(&spoken),
        generators: vec![(g.address(), vertex_of(&g), 1)],
        residue: plane_word(2),
    };
    assert!(!p.whole(), "the presentation did not reach the whole vertex");
    let before = k.treasury().balance_mu(&speaker);
    assert_eq!(
        k.read_paths(citer, &spoken, std::slice::from_ref(&p)),
        1,
        "generators then residue reach the vertex"
    );
    assert_ne!(
        k.treasury().balance_mu(&speaker),
        before,
        "the generator that reached is a use and its speaker is paid"
    );
}

/// 26: A USE RESTS ON A SENTENCE THE COURT ADMITTED. Read before the
/// adjudication, a refused sentence's generators were paid for a form the
/// court had just refused. `hear_frame` reads paths only for a sentence
/// that Stood or was Ratified.
#[test]
fn a_refused_sentences_path_pays_nobody() {
    use crate::anastome::dialogue::{Adjudication, SpeechAct, Spoken};
    use crate::transducer::sentence::Path;
    let speaker = [1u8; 32];
    let citer = [3u8; 32];
    let mut k = Kernel::new();
    let g = ruled_generator(&mut k, speaker, plane_word(1), 1);
    k.allocate(citer, [8, 8, 8, 8]);
    // A tape that is not a sentence: the court refuses it whole (50.6).
    let refused = vec![0xffu8; 8];
    let p = Path {
        vertex: crate::transducer::address::Address::from_bytes([
            crate::transducer::address::TAG_FORM,
            9,
        ]),
        generators: vec![(g.address(), vertex_of(&g), 2)],
        residue: crate::transducer::alphabet::Word::identity(),
    };
    let act = SpeechAct {
        voice: citer,
        task: crate::transducer::address::Address::from_bytes([0u8; 32]),
        k: 1,
        sentences: vec![refused.clone()],
        standing: vec![0],
        contractions: vec![None],
        paths: vec![vec![p]],
    };
    let before = k.treasury().balance_mu(&speaker);
    let out = k.hear_frame(&act).unwrap_or_default();
    assert!(
        matches!(out.first(), Some(Adjudication::Refused { .. })),
        "the court refuses a tape that is not a sentence"
    );
    assert_eq!(
        k.treasury().balance_mu(&speaker),
        before,
        "and nothing on its frame is a use"
    );
    let _ = Spoken {
        tape: refused,
        standing: 0,
        contraction: None,
        paths: Vec::new(),
    };
}
