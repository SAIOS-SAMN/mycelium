// 50.9 — the contraction protocol at the court: a cited contraction is
// verified before the sentence is replayed, and two courts of distinct
// identity reach one standing through the codec with no shared memory.

use super::common::*;
use crate::anastome::dialogue::{self, Adjudication, Spoken};
use crate::cognition::*;
use crate::transducer::family::{mint_sentence, one_pair_form, sentence_measure, NoCrystal, TrainPair};
use crate::transducer::sentence::{
    as_element_form, form_vertex_key, rule_of_form, Contracted, Contraction, RefusalKind,
};

/// A bar and a lone painted site of another hue carried together (50.3's
/// rigid pair): two word vertices and their edge — the voice's family, and
/// the sentence it mints over an empty genome.
fn voice_family() -> Vec<TrainPair> {
    vec![(digits(&["11200"]), digits(&["00112"]))]
}

/// The court's copy of the task: the lone site lands painted another hue,
/// so the voice's word for it does not carry its body there.
fn court_family() -> Vec<TrainPair> {
    vec![(digits(&["11200"]), digits(&["00113"]))]
}

/// The voice's pair form for a one-pair family, measured over it and
/// spoken as a sentence (F, 50.2).
fn sentence_over(family: &[TrainPair]) -> Element {
    let [(x, y)] = family else {
        panic!("one pair");
    };
    let f = one_pair_form(x, y);
    let mag = sentence_measure(&f, family, &NoCrystal, 0).expect("every body is spoken");
    as_element_form(&f, mag).expect("a tape")
}

fn task_of(family: &[TrainPair]) -> Address {
    crate::registry::task_id(&crate::arc::Task { train: family.to_vec(), test: Vec::new() })
        .expect("a grid pair's chain is A5's, so the task has an identity")
}

fn court(id: u8, task: &Address, family: Vec<TrainPair>) -> Kernel {
    let mut k = Kernel::new();
    k.bind_identity([id; 32]);
    k.bind_task_family(task, family);
    k
}

/// The judge's round trip, through the codec: a speech frame in, a reply
/// frame out.
fn hear(k: &mut Kernel, frame: &[u8]) -> Vec<u8> {
    let act = dialogue::decode_speech(frame).expect("the frame decodes");
    match k.hear_frame(&act) {
        Ok(adjudicated) => {
            dialogue::encode_speech_reply(&adjudicated).expect("the reply fits its count")
        }
        Err(lanes) => dialogue::encode_deferral(outcome_code(&Outcome::GermPending), &lanes),
    }
}

/// The court refuses the voice's sentence at the vertex that does not
/// carry its body on the court's family; the honest contraction, citing
/// the receipt, is verified and replayed as a sentence; a sentence citing
/// that receipt which is not the contraction is refused as a malformed
/// complex at its tape, and the court's record of the refusal stands.
#[test]
fn the_court_verifies_a_contraction_before_it_replays_the_sentence() {
    let f_v = voice_family();
    let f_1 = court_family();
    let s = sentence_over(&f_v);
    let task = task_of(&f_1);
    let mut k = court(1, &task, f_1);
    let voice = [5u8; 32];
    let first = k.hear_speech(voice, task.as_bytes(), &[(s.clone(), None)]).expect("heard");
    let [Adjudication::Refused { kind, at, receipt }] = first.as_slice() else {
        panic!("the second word does not carry its body on the court's family, got {first:?}");
    };
    assert_eq!(*kind, RefusalKind::VertexBody);
    // THE VOICE'S MOVE, on its own copy of the form: the demand's bodies
    // are the remainder the contracted form leaves on the voice's family.
    let mut f =
        crate::transducer::sentence::from_element_form(&s).expect("the tape reads back");
    let mut probe = f.clone();
    assert_eq!(probe.contract(*kind, at, &[]), Some(Contracted::Moved));
    let remainder: Vec<(i64, i64, u8, u8)> =
        crate::transducer::family::form_deflation(&probe, &f_v, &NoCrystal).1.into_iter().flatten().collect();
    assert!(!remainder.is_empty(), "the refused word's body is what the contraction leaves");
    assert_eq!(f.contract(*kind, at, &remainder), Some(Contracted::Moved));
    let contracted = mint_sentence(&rule_of_form(f.clone()), &f_v, &NoCrystal, 0).expect("the contraction is a sentence");
    let certificate = Contraction { kind: *kind, at: at.clone(), before: s.address(), after: form_vertex_key(&f), receipt: *receipt };
    // A FORGED CITATION: another sentence citing the receipt is not the
    // contraction at the coordinate.
    let other = sentence_over(&[(digits(&["11"]), digits(&["22"]))]);
    let forged = k
        .hear_speech(voice, task.as_bytes(), &[(other.clone(), Some(certificate.clone()))])
        .expect("heard");
    assert!(
        matches!(forged.as_slice(), [Adjudication::Refused { kind: RefusalKind::MalformedComplex, .. }]),
        "a sentence that is not the contraction is refused at its tape, got {forged:?}"
    );
    // THE HONEST ONE is verified and then replayed as a sentence: what the
    // court finds next is the sentence's own business, never the
    // contraction's.
    let second = k
        .hear_speech(voice, task.as_bytes(), &[(contracted, Some(certificate))])
        .expect("heard");
    assert!(
        !matches!(second.as_slice(), [Adjudication::Refused { kind: RefusalKind::MalformedComplex, .. }]),
        "the verified contraction is replayed as a sentence, got {second:?}"
    );
}

/// THE INTELLIGIBILITY ASSERTION (50.9). Voice A utters S; Court 1 replays
/// on its copy of the family and answers a typed adjudication through the
/// codec; Voice A receives it, contracts, and utters S' citing the
/// receipt; the loop is well founded and closes on a standing within a few
/// rounds. Court 2 — a second kernel with its own identity and no shared
/// runtime memory — replays the same serialized frame and reaches the same
/// address and the same standing.
#[test]
fn two_courts_of_distinct_identity_reach_one_standing_through_the_codec() {
    let f_v = voice_family();
    let f_1 = court_family();
    let task = task_of(&f_1);
    let mut voice = crate::client::sdk::Entity::open("127.0.0.1:1", crate::client::sdk::Kernel::named("voice-a"));
    let vk = voice.kernel().voice();
    let mut court1 = court(1, &task, f_1.clone());
    let mut court2 = court(2, &task, f_1);
    let s = sentence_over(&f_v);
    let mut utter = s.clone();
    let mut cited: Option<Contraction> = None;
    let mut contractions = 0usize;
    let mut standing: Option<u8> = None;
    for _round in 0..4 {
        let frame = dialogue::encode_speech_spoken(
            &vk,
            &task,
            0,
            &[Spoken { tape: utter.to_bytes(), standing: 0, contraction: cited.clone() }],
        );
        let reply = hear(&mut court1, &frame);
        let adjudicated = dialogue::decode_speech_reply(&reply).expect("one adjudication per sentence");
        let [first] = adjudicated.as_slice() else {
            panic!("one sentence, one adjudication: {adjudicated:?}");
        };
        match first {
            Adjudication::Refused { kind, at, receipt } => {
                assert_eq!(
                    voice.receive_adjudication(&utter, *kind, at.clone(), *receipt, &f_v, task.as_bytes()),
                    Contracted::Moved,
                    "the refusal contracts: {kind:?} at {}",
                    at.label()
                );
                let mut pending = voice.take_contracted();
                let Some(next) = pending.pop() else {
                    panic!("the contraction is a sentence to utter");
                };
                assert!(pending.is_empty(), "one contraction, one sentence");
                utter = next.sentence;
                cited = Some(next.certificate);
                contractions += 1;
            }
            _ => {
                standing = Some(first.code());
                break;
            }
        }
    }
    assert!(contractions >= 1, "the first utterance was refused and contracted");
    let code = standing.expect("court 1 stood the contracted sentence within four rounds");
    assert!(voice.sealed_at(s.address().as_bytes()).is_some(), "the refused address is sealed on the voice");
    // COURT 2: the same serialized frame, its own identity, nothing shared.
    let frame = dialogue::encode_speech_spoken(
        &vk,
        &task,
        0,
        &[Spoken { tape: utter.to_bytes(), standing: 0, contraction: cited.clone() }],
    );
    let reply = hear(&mut court2, &frame);
    let adjudicated = dialogue::decode_speech_reply(&reply).expect("one adjudication per sentence");
    assert_eq!(adjudicated.iter().map(|a| a.code()).collect::<Vec<_>>(), vec![code], "the same standing on the second court");
    let addr = utter.address();
    assert!(court1.provisional.contains(&addr), "court 1 dockets the sentence");
    assert!(court2.provisional.contains(&addr), "court 2 dockets the same address");
    assert_ne!(court1.identity, court2.identity, "two courts, two identities");
    assert!(
        court1.chain().receipts().iter().all(|r| r.vk == [1; 32])
            && court2.chain().receipts().iter().all(|r| r.vk == [2; 32]),
        "each court's receipts are sealed under its own key"
    );
}
