// 78.102 — AN OPEN SENTENCE IS HEARD: every sentence a voice speaks has its
// rings re-derived by the court, whatever the adjudication, and the words they
// teach read — spelled, or offered as letters.

use super::common::*;
use crate::anastome::dialogue::{self, Spoken};
use crate::cognition::*;
use crate::transducer::sentence::as_element_form;

/// The slide family's form, spoken as a sentence: its rings of seen rays are
/// nilpotent theorems, each teaching a slide word.
fn slide_sentence() -> Element {
    let slides = vec![
        (digits(&["1100000", "0000000", "0002000", "0002200"]), digits(&["1100000", "0000000", "0000020", "0000022"])),
        (digits(&["0000110", "0000000", "0200000", "0220000"]), digits(&["0000110", "0000000", "0002000", "0002200"])),
    ];
    let rule = crate::transducer::family::extract_rule(&slides, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("a rule");
    as_element_form(&rule.form, [1, 0, 0, 0]).expect("a tape")
}

/// A SENTENCE THE COURT DOES NOT STAND IS STILL HEARD. Spoken to a court that
/// holds no family for its task, the slide sentence is not stood — and its
/// rings are re-derived all the same, each taught word read: the slides
/// spelled by the alphabet, nothing offered that the alphabet says.
#[test]
fn every_sentence_heard_has_its_rings_read_whatever_its_adjudication() {
    let e = slide_sentence();
    let mut court = Kernel::new();
    court.bind_identity([0x5a; 32]);
    let frame = dialogue::encode_speech_spoken(
        &[0x5b; 32],
        [9u8; 4],
        0,
        &[Spoken { tape: e.to_bytes().expect("the record writes"), standing: 0, contraction: None }],
    )
    .expect("the task frames");
    let act = dialogue::decode_speech(&frame).expect("the frame decodes");
    let before = court.winding_tally();
    let heard = court.hear_frame(&act);
    let after = court.winding_tally();
    let read = |t: &crate::server::cognition::court::winding::WindingTally| t.heard_spelled + t.heard_offered + t.heard_peel_unread + t.heard_rotor_unread;
    assert!(read(&after) > read(&before), "the sentence's taught words were read ({heard:?}): {after:?}");
    assert!(after.heard_spelled > before.heard_spelled, "the slide words are spelled by the alphabet");
    assert_eq!(after.heard_offered, before.heard_offered, "and nothing the alphabet says is offered as a letter");
}
