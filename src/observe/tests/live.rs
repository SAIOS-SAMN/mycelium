use super::super::*;

/// THE OBSERVER'S CALL (ruled 2026-10-15): a call travels whole, is
/// left for the winding carrier, and is answered at the next law boundary
/// the carrier passes — the asker reads the carrier's standing beside the
/// answer; its lines travel whole, and lines past the envelope are said by
/// count and where to ask from.
#[test]
fn live_call_is_answered_at_next_boundary() {
    let line = "observe-test: thoughts";
    let q = live::encode_call(line).expect("the call writes");
    assert_eq!(live::decode_call(&q).as_deref(), Some(line), "the call reads back");
    assert_eq!(decode_act(&q), None, "a call is no query: a court's door reads no act in it");
    // An observer this test holds alone: no other carrier of the process
    // enters it, takes its calls or answers them.
    let observer = live::Live::new();
    // Before any boundary answers it, the call is said to be waiting.
    let asked = observer.ask(line).expect("the call is left");
    let waiting = observer.read(line, asked);
    assert!(matches!(waiting.as_slice(), [l, ..] if l.starts_with("winding carrier: in")), "the carrier's standing first: {waiting:?}");
    assert!(waiting.iter().any(|l| l.contains("NOT ANSWERED")), "a call no boundary reached is said so: {waiting:?}");
    // The carrier passes a boundary and answers what waits there.
    observer.enter("observe-test: a law");
    let wanted = observer.wanted();
    assert_eq!(wanted, vec![line.to_string()], "the call waits at the boundary");
    observer.answer(line, vec!["#0 thought · ℓ 4 · hours 5 · m 1".to_string()]);
    let out = observer.read(line, asked);
    assert!(out.iter().any(|l| l.starts_with(line) && l.contains("answered at boundary")), "answered fresh: {out:?}");
    let lines: Vec<String> = (0..50).map(|i| format!("line {i}")).collect();
    let all = live::encode_lines(&lines, usize::MAX).expect("the lines write");
    assert_eq!(live::decode_lines(&all), Some(lines.clone()), "the lines read back whole");
    let cut = live::decode_lines(&live::encode_lines(&lines, 200).expect("the lines write")).expect("they read");
    assert!(cut.len() < lines.len(), "an envelope of 200 bytes holds fewer");
    assert!(matches!(cut.as_slice(), [.., l] if l.contains("lines past the envelope") && l.contains("from")), "and says what it left: {cut:?}");
}

/// THE ASKER WAITS WHILE THE CARRIER WORKS (ruled 2026-10-24). A call asked
/// of a carrier working inside one law — not parked, no boundary passing —
/// is still waiting several parks later, and is answered fresh at the
/// carrier's next boundary. The call is left before the asker waits on
/// it, and the observer is the test's own: the process's
/// minds, winding in other tests, park, enter and take calls on the
/// process's observer, never on this one (#78).
#[test]
fn live_call_waits_while_carrier_works_inside_law() {
    let line = "observe-test: a long law";
    let observer = live::Live::new();
    observer.park(false);
    let asked = observer.ask(line).expect("the call is left");
    let out = std::thread::scope(|s| {
        let asker = s.spawn(|| observer.wait(line, asked));
        std::thread::sleep(crate::physics::idle_park(None) * 8);
        assert!(!asker.is_finished(), "eight parks inside one law, the carrier working: the asker waits");
        observer.enter("observe-test: the law after");
        assert_eq!(observer.wanted(), vec![line.to_string()], "the call waits at the boundary");
        observer.answer(line, vec!["answered after the long law".to_string()]);
        asker.join().expect("the asker returns")
    });
    assert!(out.iter().any(|l| l.starts_with(line) && l.contains("answered at boundary")), "answered fresh at the next boundary: {out:?}");
}
