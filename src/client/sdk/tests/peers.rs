// MU-CLIENT §5.3(5) (#145, ruled 2026-11-06) — A PEER REMEMBERED BY ITS
// RECEIPTS: its verified chatter frames under a question's coordinates,
// folded per peer key by the residue's conjugacy invariants, held while the
// question is and lapsing with it.

use crate::anastome::dialogue::chatter::{Chatter, Said, Spoken};
use crate::client::sdk::*;
use crate::transducer::cga::{Conjugacy, Mv};

fn q(n: i64) -> num_rational::BigRational {
    num_rational::BigRational::from_integer(n.into())
}

fn peer(b: u8) -> ed25519_dalek::SigningKey {
    ed25519_dalek::SigningKey::from_bytes(&[b; 32])
}

/// `by`'s signed Say under the coordinates of the question on `said`,
/// `vertex`, its factors `factors` in the order said.
fn answer(by: &ed25519_dalek::SigningKey, said: &[u8], vertex: &[u8], factors: &[Mv]) -> (Vec<u8>, Vec<u8>) {
    let coords = crate::client::sdk::mind::chatter::question_key(said, vertex);
    let words = factors.iter().map(|m| Spoken { vertex: b"factor".to_vec(), motion: m.clone(), gauge: Vec::new(), terms: Vec::new(), vantage: vec![m.clone()] }).collect();
    let frame = Chatter { speaker: by.verifying_key().to_bytes(), point: None, said: Said::Say { said: coords.clone(), words, evidence: None } }.sign(by).expect("signed");
    let mut key = by.verifying_key().to_bytes().to_vec();
    key.extend_from_slice(&coords);
    (key, frame)
}

/// The gain asked about, and its inverse: one class (the generator's square
/// and the weight are the same), another motion.
fn gain_and_conjugate() -> (Mv, Mv) {
    let gain = crate::transducer::cga::dilator(&q(2));
    let inverse = crate::transducer::cga::dilator(&num_rational::BigRational::new(1.into(), 2.into()));
    (gain, inverse)
}

/// THE RECORD IS FOLDED FROM THE PEER'S FRAMES, FILED BY THE RESIDUE'S
/// CONJUGACY INVARIANTS. A question asked of a peer stands with the frame
/// it answered under its coordinates: an answer whose factor re-derives the
/// residue reads TAKEN, filed by the residue's class read off the motion,
/// the vantage it was said with kept; another peer's frame is not this
/// peer's record.
#[test]
fn peer_record_folds_answer_frames_by_the_residue_class() {
    let (p, other) = (peer(1), peer(2));
    let pk = p.verifying_key().to_bytes();
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("asking"));
    let mut field = std::mem::take(&mut mind.field);
    let record = field.think(|| {
        let (gain, _) = gain_and_conjugate();
        assert!(mind.question(&pk, b"said", b"vertex", &gain).is_some(), "the gain is asked of the peer");
        let (key, frame) = answer(&p, b"said", b"vertex", std::slice::from_ref(&gain));
        mind.hold_answer_frame(&key, frame);
        let (_, stray) = answer(&other, b"said", b"vertex", std::slice::from_ref(&gain));
        mind.hold_answer_frame(&key, stray);
        mind.peer_record(&pk)
    });
    mind.field = field;
    let (gain, _) = gain_and_conjugate();
    assert_eq!(record.len(), 1, "both frames held under the question; the fold reads the speaker off each, and one is this peer's");
    assert!(record.iter().all(|r| r.answer == crate::client::sdk::mind::peers::Answer::Taken), "its factor re-derives the residue");
    assert!(record.iter().all(|r| Some(r.class.clone()) == gain.class_key()), "filed by the residue's conjugacy invariants");
    assert!(record.iter().all(|r| r.vantage.len() == 1), "the vantage said with it kept");
}

/// A PEER INEXACT ON A CLASS IS NOT ASKED THAT CLASS AGAIN. The peer's
/// answer to the gain does not re-derive it; the gain's inverse (one class,
/// another motion) is then not asked of that peer, and its residue is bound
/// by construction; the same inverse is asked of a peer whose record holds
/// nothing against it.
#[test]
fn peer_inexact_on_a_class_is_not_asked_it_again() {
    let (p, fresh) = (peer(3), peer(4));
    let (pk, fk) = (p.verifying_key().to_bytes(), fresh.verifying_key().to_bytes());
    use crate::client::sdk::mind::peers::Addressed;
    // Two minds with one history: the peer asked the gain and answered it
    // inexactly. Then one asks the peer the inverse, the other a fresh peer.
    let ask_after_inexact = |name: &str, of: [u8; 32]| {
        let mut mind = Mind::open("127.0.0.1:1", Voice::named(name));
        let mut field = std::mem::take(&mut mind.field);
        let out = field.think(|| {
            let (gain, inverse) = gain_and_conjugate();
            assert!(mind.question(&pk, b"said", b"vertex", &gain).is_some());
            let wrong = crate::transducer::cga::dilator(&q(3));
            let (key, frame) = answer(&p, b"said", b"vertex", &[wrong]);
            mind.hold_answer_frame(&key, frame);
            let addressed = mind.addressed(&of, &inverse);
            let asked = mind.question(&of, b"again", b"vertex", &inverse).is_some();
            (addressed, asked, mind.questions.len())
        });
        mind.field = field;
        out
    };
    assert_eq!(ask_after_inexact("addressing", pk), (Addressed::Inexact, false, 1), "the class it answered inexactly is not asked of it again");
    assert_eq!(ask_after_inexact("addressing fresh", fk), (Addressed::Ask, true, 2), "a peer whose record holds nothing against it is asked");
}

/// A PEER'S FRAMES LAPSE WITH THE QUESTION THEY ANSWERED: the question's
/// line let go, its frames go with it and the record holds nothing.
#[test]
fn peer_frames_lapse_with_the_question() {
    let p = peer(5);
    let pk = p.verifying_key().to_bytes();
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("lapsing"));
    let mut field = std::mem::take(&mut mind.field);
    let (held, after) = field.think(|| {
        let (gain, _) = gain_and_conjugate();
        assert!(mind.question(&pk, b"said", b"vertex", &gain).is_some());
        let (key, frame) = answer(&p, b"said", b"vertex", &[crate::transducer::cga::dilator(&q(3))]);
        mind.hold_answer_frame(&key, frame);
        let held = mind.peer_record(&pk).len();
        let mut line = crate::client::sdk::mind::chatter::QUESTION_LINE.to_vec();
        line.extend_from_slice(&key);
        let mut budget = mind.stroke();
        assert!(mind.lapse_question(&line, &mut budget), "the line was the question's");
        (held, mind.peer_record(&pk).len())
    });
    mind.field = field;
    assert_eq!((held, after), (1, 0), "held while the question was, gone with it");
}
