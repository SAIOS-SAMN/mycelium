use super::super::*;
use super::common::*;

/// The shelf advances without the court and wakes a parked reader.
#[test]
fn tshelf_raise_final_wakes_parked_reader() {
    let shelf = std::sync::Arc::new(TShelf::new());
    let p = TPoint { origin: ORIGIN, h: 10, lanes: [1, 0, 0, 0], k: 1 };
    assert!(shelf.raise_final(p));
    assert!(!shelf.raise_final(p), "the same point is no advance");
    assert!(!shelf.raise_final(TPoint { h: 9, ..p }), "a lower one neither");
    assert_eq!((shelf.final_point(), shelf.advances()), (Some(p), 1));
    // A speaker's point is heard, never final.
    shelf.heard_at_door(Some(TPoint { h: 99, ..p }));
    assert_eq!((shelf.final_point(), shelf.heard().map(|q| q.h)), (Some(p), Some(99)));
    // A reader parked on what it saw wakes on the next advance.
    let seen = shelf.seen();
    let s2 = std::sync::Arc::clone(&shelf);
    let raiser = std::thread::spawn(move || s2.raise_final(TPoint { h: 11, ..p }));
    let woke = loop {
        let now = shelf.park(seen, crate::physics::stroke_floor());
        if now.advances > seen.advances {
            break now;
        }
    };
    assert!(raiser.join().expect("joins"));
    assert_eq!(woke.advances, 2);
}
