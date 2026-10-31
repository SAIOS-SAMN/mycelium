use super::super::*;
use super::common::*;

/// THE DISPATCH OBSERVATION (2026-10-22): a task that leaves a
/// court stands under its whole citation with the voice it last went to and
/// the court's height then; dispatched again it is counted, the later voice
/// and height standing; and the reading travels whole.
#[test]
fn note_dispatch_keeps_latest_voice_by_citation() {
    let cite = [0xf2; 32];
    note_dispatch(cite, party(1), 40);
    note_dispatch(cite, party(2), 41);
    let d = dispatched();
    let s = d.sent.iter().find(|s| s.citation == cite).copied().expect("kept by its citation");
    assert_eq!((s.voice, s.height, s.times), (party(2), 41, 2), "the later voice and height stand; dispatched twice");
    reads_back(&[Reading::Dispatched(Box::new(d.clone()))]);
    assert!(d.lines().iter().any(|l| l.contains(&"f2".repeat(32)) && l.contains("at height 41")), "named whole");
}
