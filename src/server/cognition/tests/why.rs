// 78.92 (2) — WHY AN ACT STANDS (`Kernel::why`, `dialogue::why`): the court
// answers an act on its ledger with every word it names, each with the
// reasons read off the word it holds — the reasons the speaker read.

use super::common::*;
use crate::anastome::dialogue::why::{decode_why, decode_why_reply, encode_why, encode_why_reply, head_of, WhyAsk};
use crate::anastome::dialogue::Adjudication;
use crate::cognition::*;
use crate::transducer::family::{cipher_form, sentence_measure, NoCrystal, TrainPair};
use crate::transducer::sentence::{as_element_form, Reasons};

/// Two separated bodies both translating by `+2`: one family the voice and
/// the court hold alike, so the sentence stands.
fn family() -> Vec<TrainPair> {
    vec![(digits(&["1100200"]), digits(&["0011002"]))]
}

/// A sentence the court stands is admitted by an act on its line; asked why
/// that act stands — by its stamp, or by its place on the court's own line —
/// the court answers the act, the word with the task it was spoken for, and
/// EXACTLY the reasons the speaker read off the sentence it sent. The answer
/// crosses the wire whole; an act the court does not hold is answered as
/// none, never as an empty one.
#[test]
fn the_court_says_why_an_act_stands_with_the_speakers_reasons() {
    let fam = family();
    let [(x, y)] = fam.as_slice() else { panic!("one pair") };
    let f = cipher_form(x, y);
    let mag = sentence_measure(&f, &fam, &NoCrystal, 0).expect("every body is spoken");
    let s = as_element_form(&f, mag).expect("a sentence");
    let spoken = Reasons::of(&s).expect("the speaker's reasons");
    let task = crate::registry::task_id(&crate::arc::Task { train: fam.clone(), test: Vec::new() }).expect("a task");
    let mut k = Kernel::new();
    k.bind_identity([1; 32]);
    k.bind_task_family(&task, fam);
    let mut frame = k.stroke();
    let heard = k.hear_speech([5; 32], task.as_bytes(), &[(s.clone(), None)], &mut frame).expect("heard");
    assert!(matches!(heard.as_slice(), [Adjudication::Stood { .. } | Adjudication::Ratified { .. }]), "{heard:?}");
    let at = k.admitted_at(s.address().as_bytes()).expect("an act admitted the sentence");
    // 78.93 B0 — the verdict itself says where the word stands.
    let [verdict] = heard.as_slice() else { panic!("one verdict") };
    assert_eq!(verdict.at(), Some(at), "the adjudication answers the admitting stamp");
    let why = k.why(WhyAsk::Of(at)).expect("the act is on the court's ledger");
    assert_eq!(why.at, at);
    let [named] = why.named.as_slice() else { panic!("the act names the one sentence: {why:?}") };
    assert_eq!(named.head, head_of(s.address().as_bytes()));
    assert_eq!(named.task, Some(head_of(task.as_bytes())), "spoken for its task");
    assert_eq!(named.reasons.as_ref(), Some(&spoken), "the court reads the speaker's reasons");
    assert_eq!(k.why(WhyAsk::Own(at.place())).as_ref(), Some(&why), "its own line, by place");
    // Through the codec, whole.
    let ask = WhyAsk::Of(at);
    assert_eq!(decode_why(&encode_why(ask)), Some(ask));
    assert_eq!(decode_why(&encode_why(WhyAsk::Own(at.place()))), Some(WhyAsk::Own(at.place())));
    let reply = encode_why_reply(Some(&why));
    assert_eq!(decode_why_reply(&reply), Some(Some(why)));
    assert!(decode_why_reply(&reply[..reply.len() - 1]).is_none(), "a truncated reply is no reply");
    let far = crate::chronometry::Place { k: u64::MAX, i: 0 };
    assert_eq!(k.why(WhyAsk::Own(far)), None);
    assert_eq!(decode_why_reply(&encode_why_reply(None)), Some(None));
}
