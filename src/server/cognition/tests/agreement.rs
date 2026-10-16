// MU-CLIENT §5.5, 78.161(5) — AGREED LANGUAGE ATTESTED: the court seals the
// attestation of a second mind whose signed HOLD names the very word a voice
// spoke, where the word is true on the scene it is bound to, and nothing else;
// and seals a CONVERGENCE where that mind's own word on the scene is another
// vertex with the same section.

use crate::anastome::dialogue::{self, Spoken};
use crate::client::sdk::chatter::{AckWord, Chatter, Said};
use crate::cognition::book::agreement::Attested;
use crate::cognition::*;

fn key(b: u8) -> ed25519_dalek::SigningKey {
    ed25519_dalek::SigningKey::from_bytes(&[b; 32])
}

fn grid(rows: &[&str]) -> crate::arc::Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// The scene: one row's two bodies carried two columns on.
const SCENE: [u8; 32] = [5; 32];
fn family() -> Vec<crate::transducer::family::TrainPair> {
    vec![(grid(&["1100", "0000"]), grid(&["0011", "0000"]))]
}

/// The slide by `(r, c)`, spelled as a connection.
fn slide(r: i64, c: i64) -> Element {
    let w = crate::transducer::alphabet::Word::identity();
    crate::transducer::element::residual::from_connection(&w, &crate::transducer::element::translator_versor(r, c), &[], &Vec::new(), [1, 0, 0, 0]).expect("a slide spells")
}

/// The 180° turn of the sensor plane carried by `(r, c)`: the unit square at
/// `(i, j)` lands at `(r − 1 − i, c − 1 − j)`.
fn turn_180(r: i64, c: i64) -> Element {
    let q = crate::transducer::alphabet::LetterId::turn_90((0, 1)).expect("the sensor plane");
    let w = crate::transducer::alphabet::Word::from_letters(&[(q, false), (q, false)]);
    crate::transducer::element::residual::from_connection(&w, &crate::transducer::element::translator_versor(r, c), &[], &Vec::new(), [1, 0, 0, 0]).expect("a 180° turn spells")
}

/// A court holding the scene's family, with `words` on its docket bound to
/// the scene, each discovered by the voice beside it.
fn court_with(words: &[(&Element, [u8; 32])]) -> Kernel {
    let mut court = Kernel::new();
    court.bind_task_family(SCENE, family());
    for (w, who) in words {
        let addr = w.address();
        court.provisional.insert(addr.as_bytes(), (*w).clone());
        court.bind_listing_task(addr.as_bytes(), SCENE);
        court.note_discovered(addr.as_bytes(), *who);
    }
    court
}

/// The HOLD a mind signs on `said`.
fn hold(by: &ed25519_dalek::SigningKey, said: &[u8]) -> Vec<u8> {
    let words = vec![AckWord { vertex: said.to_vec(), motion: None, hue: None }];
    let deviation = [1, 0, 0, 2].map(|n: i64| num_rational::BigRational::new(n.into(), 5.into()));
    Chatter { speaker: by.verifying_key().to_bytes(), point: None, said: Said::Hold { said: said.to_vec(), words, deviation } }.sign(by).expect("signed")
}

/// One word spoken by `voice` carrying `agreement`.
fn speech(voice: [u8; 32], e: &Element, agreement: Vec<u8>) -> dialogue::SpeechAct {
    let frame = dialogue::encode_speech_spoken(
        &voice,
        SCENE,
        0,
        &[Spoken { tape: e.to_bytes().expect("the record writes"), standing: 0, contraction: None, agreement: Some(agreement) }],
    )
    .expect("the speech frames");
    dialogue::decode_speech(&frame).expect("the frame decodes")
}

fn attests(k: &Kernel) -> usize {
    k.chain().nodes().filter(|r| r.kind == crate::chronometry::ActKind::Attest).count()
}

/// A SECOND MIND'S SIGNED HOLD ON A WORD TRUE ON ITS SCENE IS SEALED AS ITS
/// ATTESTATION, ONCE. The court holds the slide bound to the scene; the
/// speaker carries the hearer's HOLD naming it: one `Attest` credited to the
/// hearer. Said again, nothing more is sealed. A HOLD naming another word,
/// one the speaker signed itself, a forged frame, and a word the court does
/// not hold seal nothing.
#[test]
fn a_second_minds_hold_on_the_word_is_sealed_as_its_attestation() {
    let (speaker, hearer) = (key(1), key(2));
    let me = speaker.verifying_key().to_bytes();
    let them = hearer.verifying_key().to_bytes();
    let e = slide(0, 2);
    let addr = e.address();
    let mut court = court_with(&[(&e, me)]);
    let before = attests(&court);

    let sealed = court.attest_agreements(&speech(me, &e, hold(&hearer, addr.as_bytes())));
    let deviation = [1, 0, 0, 2].map(|n: i64| num_rational::BigRational::new(n.into(), 5.into()));
    assert_eq!(
        sealed,
        vec![Attested { word: addr.as_bytes().to_vec(), by: them, deviation, converged: Vec::new() }],
        "the hearer's attestation is sealed on the word, its signed deviation read"
    );
    assert_eq!(attests(&court), before + 1, "one Attest on the line");
    assert!(court.attest_agreements(&speech(me, &e, hold(&hearer, addr.as_bytes()))).is_empty(), "sealed once");

    let other = slide(0, 3);
    assert!(court.attest_agreements(&speech(me, &e, hold(&hearer, other.address().as_bytes()))).is_empty(), "a HOLD on another word");
    assert!(court.attest_agreements(&speech(me, &e, hold(&speaker, addr.as_bytes()))).is_empty(), "the speaker's own HOLD");
    let mut forged = hold(&key(3), addr.as_bytes());
    let mid = forged.len() / 2;
    forged[mid] ^= 1;
    assert!(court.attest_agreements(&speech(me, &e, forged)).is_empty(), "a forged frame");
    assert!(court.attest_agreements(&speech(me, &other, hold(&hearer, other.address().as_bytes()))).is_empty(), "a word the court does not hold");
    assert_eq!(attests(&court), before + 1, "nothing else sealed");
}

/// K1 — AN AGREEMENT THE SCENE REFUTES IS NO ATTESTATION. The slide by one
/// row, bound to the scene, paints the row below where the answer holds
/// nothing: its section is empty, and the hearer's signed HOLD seals nothing.
/// A word bound to no family is evidenced by nothing either.
#[test]
fn an_agreement_the_scene_refutes_is_not_sealed() {
    let (speaker, hearer) = (key(1), key(2));
    let me = speaker.verifying_key().to_bytes();
    let false_word = slide(1, 0);
    let mut court = court_with(&[(&false_word, me)]);
    let before = attests(&court);
    assert!(court.attest_agreements(&speech(me, &false_word, hold(&hearer, false_word.address().as_bytes()))).is_empty(), "false on its scene");
    let unbound = slide(0, 2);
    court.provisional.insert(unbound.address().as_bytes(), unbound.clone());
    assert!(court.attest_agreements(&speech(me, &unbound, hold(&hearer, unbound.address().as_bytes()))).is_empty(), "bound to no family");
    assert_eq!(attests(&court), before, "nothing sealed");
}

/// K2 — TWO CONSTRUCTIONS, ONE CONCLUSION, VERIFIED. The speaker's slide and
/// the hearer's own 180° turn are bound to one scene and change its same four
/// sites: the hearer's attestation of the slide carries the convergence, and
/// the speaker's `Attest` is sealed on the turn, once. A word of the
/// scene the hearer did not discover, and a hearer's word with another
/// section, converge with nothing.
#[test]
fn two_words_with_one_section_on_the_scene_converge() {
    let (speaker, hearer, third) = (key(1), key(2), key(3));
    let me = speaker.verifying_key().to_bytes();
    let them = hearer.verifying_key().to_bytes();
    // `wrong` is true on a smaller section (the two sites it changed): its
    // own, not the slide's.
    let (mine, theirs, wrong) = (slide(0, 2), turn_180(1, 4), slide(0, 1));
    let mut court = court_with(&[(&mine, me), (&theirs, them), (&wrong, them)]);
    let before = attests(&court);
    let sealed = court.attest_agreements(&speech(me, &mine, hold(&hearer, mine.address().as_bytes())));
    let [a] = sealed.as_slice() else { panic!("one attestation, got {sealed:?}") };
    assert_eq!(a.converged, vec![theirs.address().as_bytes().to_vec()], "the 180° turn converged; the one-column slide, another section, did not");
    assert_eq!(attests(&court), before + 2, "the hearer's Attest on the slide, the speaker's on the turn");
    let _ = court.attest_agreements(&speech(me, &mine, hold(&hearer, mine.address().as_bytes())));
    assert_eq!(attests(&court), before + 2, "sealed once");

    // The same 180° turn discovered by a third mind is not the hearer's own.
    let mut other = court_with(&[(&mine, me), (&theirs, third.verifying_key().to_bytes())]);
    let sealed = other.attest_agreements(&speech(me, &mine, hold(&hearer, mine.address().as_bytes())));
    let [a] = sealed.as_slice() else { panic!("one attestation") };
    assert!(a.converged.is_empty(), "a word the attesting mind did not construct converges with nothing");
}

/// MU-CLIENT §5.6 (E3) — THE UNION'S ATTESTATION PAYS. A word the court holds
/// on its docket with a claim in escrow for its speaker: the hearer's HOLD
/// joins the hearer to the claim beside the speaker, so the install pays both;
/// the claim's measure is the speaker's, untouched. A word with no claim
/// standing (installed, or its claim dropped) gains no party.
#[test]
fn an_attestation_joins_the_hearer_to_the_words_claim() {
    let (speaker, hearer) = (key(1), key(2));
    let me = speaker.verifying_key().to_bytes();
    let them = hearer.verifying_key().to_bytes();
    let e = slide(0, 2);
    let addr = e.address();
    let bare = turn_180(1, 4);
    let mut court = court_with(&[(&e, me), (&bare, me)]);
    let anchor = court.anchor();
    court.treasury.escrow_award(&addr, vec![me], [3, 0, 0, 1], anchor);
    let _ = court.attest_agreements(&speech(me, &e, hold(&hearer, addr.as_bytes())));
    let claim = court.treasury.escrow_of(addr.as_bytes()).expect("the claim stands");
    assert_eq!(claim.voices, vec![me, them], "the speaker and the mind that agreed, both parties");
    assert_eq!(claim.mu, [3, 0, 0, 1], "the measure is the speaker's, untouched");

    let _ = court.attest_agreements(&speech(me, &bare, hold(&hearer, bare.address().as_bytes())));
    assert!(court.treasury.escrow_of(bare.address().as_bytes()).is_none(), "no claim standing, no party joined, none invented");
}
