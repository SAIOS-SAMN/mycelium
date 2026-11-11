use super::super::*;
use super::fixtures::*;

/// The shelf advances without the court and wakes a parked reader.
#[test]
fn tshelf_raise_final_wakes_parked_reader() {
    let shelf = std::sync::Arc::new(TShelf::new());
    let p = TPoint { origin: ORIGIN, h: 10, lanes: [1, 0, 0, 0].into(), k: 1 };
    assert!(shelf.raise_final(p.clone()));
    assert!(!shelf.raise_final(p.clone()), "the same point is no advance");
    assert!(!shelf.raise_final(TPoint { h: 9, ..p.clone() }), "a lower one neither");
    assert_eq!((shelf.final_point(), shelf.advances()), (Some(p.clone()), 1));
    // A speaker's point is heard, never final.
    shelf.heard_at_door(Some(TPoint { h: 99, ..p.clone() }));
    assert_eq!((shelf.final_point(), shelf.heard().map(|q| q.h)), (Some(p.clone()), Some(99)));
    // A reader parked on what it saw wakes on the next advance.
    let seen = shelf.seen();
    let s2 = std::sync::Arc::clone(&shelf);
    let raiser = std::thread::spawn(move || s2.raise_final(TPoint { h: 11, ..p }));
    let woke = shelf.park_until_moved(seen);
    assert!(raiser.join().expect("joins"));
    assert_eq!(woke.advances, 2, "woken by the advance, with no bound");
}

/// #234 (ruled 2026-11-11, option (a)) — AN IDLE READER PARKS UNTIL THE NEXT
/// MOTION, with no bound: an act arriving at the door wakes it, and so does
/// an act judged (the court may have moved); a reader that saw nothing new
/// is never woken by the passing of strokes.
#[test]
fn tshelf_park_until_moved_wakes_on_act_and_judgement() {
    let shelf = std::sync::Arc::new(TShelf::new());
    for wake in [|s: &TShelf| s.heard_at_door(None), |s: &TShelf| s.judged()] {
        let seen = shelf.seen();
        let s2 = std::sync::Arc::clone(&shelf);
        let waker = std::thread::spawn(move || wake(&s2));
        let woke = shelf.park_until_moved(seen);
        waker.join().expect("joins");
        assert_eq!(woke.acts, seen.acts + 1, "woken by the one motion");
    }
}
