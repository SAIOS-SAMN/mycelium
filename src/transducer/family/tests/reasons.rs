// 78.92 — SPEECH THAT CARRIES ITS REASONS (`sentence::reasons`): the reasons
// are read off the sentence itself, so the speaker and the court that holds
// the bytes it was sent read the same reasons.

use super::common::grid;
use crate::transducer::element::Element;
use crate::transducer::sentence::{as_element_form, Reasons, Vertex};

/// A sentence spoken from an extracted rule: its reasons name one entry per
/// word vertex and per demand vertex of its form, and its unsaid cells are
/// its remainder's. Sent as bytes and decoded as a court decodes it, the
/// sentence yields EXACTLY the reasons the speaker read — the reasons are
/// the sentence's, not the speaker's story about it — and the reasons
/// themselves travel whole.
#[test]
fn the_court_reads_the_reasons_the_speaker_read() {
    let family = vec![
        (grid(&["11100", "00000", "00001"]), grid(&["22200", "00000", "00005"])),
        (grid(&["11110", "00000", "00011"]), grid(&["22220", "00000", "00055"])),
    ];
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let rule = crate::transducer::family::extract_rule(&family, &mut budget).expect("a rule");
    let e = as_element_form(&rule.form, [1, 0, 0, 1]).expect("a sentence");
    let r = Reasons::of(&e).expect("a sentence has reasons");
    let form = crate::transducer::sentence::from_element_form(&e).expect("its form");
    let words = form.vertices.entries().filter(|(_, v)| matches!(v, Vertex::Word { .. })).count();
    let demands = form.vertices.entries().filter(|(_, v)| matches!(v, Vertex::Demand { .. })).count();
    assert_eq!((r.words.len(), r.open.len()), (words, demands), "one reason per word and per demand");
    assert_eq!(r.unsaid, e.receipts().remainder.len() as u64);
    assert_eq!(r.measure, [1, 0, 0, 1]);
    // The court's copy: the bytes the voice puts on the wire, decoded.
    let bytes = e.to_bytes().expect("the sentence writes");
    let mut p = 0usize;
    let heard = Element::from_bytes(&bytes, &mut p).expect("the court decodes it");
    assert_eq!(Reasons::of(&heard).as_ref(), Some(&r), "speaker and court read one set of reasons");
    let mut out = Vec::new();
    r.put(&mut out);
    let mut q = 0usize;
    assert_eq!(Reasons::take(&out, &mut q), Some(r.clone()));
    assert_eq!(q, out.len(), "read whole");
    assert!(Reasons::take(&out[..out.len() - 1], &mut 0).is_none(), "a truncated reading is no reading");
    let says = r.says();
    assert_eq!(says.len(), 5, "what, where, meaning, unexplained, unsaid: {says:?}");}

/// A word that is not a sentence carries no reasons: none are invented.
#[test]
fn a_word_that_is_not_a_sentence_has_no_reasons() {
    assert_eq!(Reasons::of(&Element::identity()), None);
}
