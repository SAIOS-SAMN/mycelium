// MU-CLIENT §5.5, 78.161(5) — AGREED LANGUAGE ATTESTED: the court seals the
// attestation of a second mind whose signed HOLD names the very sentence a
// voice spoke, and nothing else.

use crate::anastome::dialogue::{self, Spoken};
use crate::client::sdk::chatter::{AckWord, Chatter, Said};
use crate::cognition::*;

fn key(b: u8) -> ed25519_dalek::SigningKey {
    ed25519_dalek::SigningKey::from_bytes(&[b; 32])
}

/// A connection word: the quarter turn on the sensor plane, said `n` times.
fn word(n: usize) -> Element {
    let q = crate::transducer::alphabet::LetterId::turn_90((0, 1)).expect("the sensor plane");
    let w = crate::transducer::alphabet::Word::from_letters(&vec![(q, false); n]);
    crate::transducer::element::residual::from_connection(&w, &crate::transducer::element::identity_versor(), &[], &Vec::new(), [1, 0, 0, 0]).expect("a connection")
}

/// The HOLD a mind signs on `said`.
fn hold(by: &ed25519_dalek::SigningKey, said: &[u8]) -> Vec<u8> {
    let words = vec![AckWord { vertex: said.to_vec(), motion: None, hue: None }];
    Chatter { speaker: by.verifying_key().to_bytes(), point: None, said: Said::Hold { said: said.to_vec(), words } }.sign(by).expect("signed")
}

/// One sentence spoken by `voice` carrying `agreement`.
fn speech(voice: [u8; 32], e: &Element, agreement: Vec<u8>) -> dialogue::SpeechAct {
    let frame = dialogue::encode_speech_spoken(
        &voice,
        [],
        0,
        &[Spoken { tape: e.to_bytes().expect("the record writes"), standing: 0, contraction: None, agreement: Some(agreement) }],
    )
    .expect("the speech frames");
    dialogue::decode_speech(&frame).expect("the frame decodes")
}

/// A SECOND MIND'S SIGNED HOLD ON THE WORD IS SEALED AS ITS ATTESTATION, ONCE.
/// The court holds the word; the speaker carries the hearer's HOLD naming it:
/// one `Attest` credited to the hearer. Said again, nothing more is sealed.
/// A HOLD naming another sentence, one the speaker signed itself, a forged
/// frame, and a word the court does not hold seal nothing.
#[test]
fn a_second_minds_hold_on_the_word_is_sealed_as_its_attestation() {
    let mut court = Kernel::new();
    let (speaker, hearer) = (key(1), key(2));
    let me = speaker.verifying_key().to_bytes();
    let them = hearer.verifying_key().to_bytes();
    let e = word(2);
    let addr = e.address();
    court.provisional.insert(addr.as_bytes(), e.clone());
    let attests = |k: &Kernel| k.chain().nodes().filter(|r| r.kind == crate::chronometry::ActKind::Attest).count();
    let before = attests(&court);

    let sealed = court.attest_agreements(&speech(me, &e, hold(&hearer, addr.as_bytes())));
    assert_eq!(sealed, vec![(addr.as_bytes().to_vec(), them)], "the hearer's attestation is sealed on the word");
    assert_eq!(attests(&court), before + 1, "one Attest on the line");
    assert!(court.attest_agreements(&speech(me, &e, hold(&hearer, addr.as_bytes()))).is_empty(), "sealed once");

    let other = word(3);
    assert!(court.attest_agreements(&speech(me, &e, hold(&hearer, other.address().as_bytes()))).is_empty(), "a HOLD on another sentence");
    assert!(court.attest_agreements(&speech(me, &e, hold(&speaker, addr.as_bytes()))).is_empty(), "the speaker's own HOLD");
    let mut forged = hold(&key(3), addr.as_bytes());
    let mid = forged.len() / 2;
    forged[mid] ^= 1;
    assert!(court.attest_agreements(&speech(me, &e, forged)).is_empty(), "a forged frame");
    assert!(court.attest_agreements(&speech(me, &other, hold(&hearer, other.address().as_bytes()))).is_empty(), "a word the court does not hold");
    assert_eq!(attests(&court), before + 1, "nothing else sealed");
}
