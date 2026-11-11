use super::super::*;
use std::sync::Mutex;

/// A stratum root for the `n`th fixture stratum: a locator whose first
/// eight bytes write `n`.
fn root_of(n: u64) -> crate::court::admission::Locator {
    let mut r = [0u8; 32];
    r[..8].copy_from_slice(&n.to_be_bytes());
    crate::court::admission::Locator::of_bytes(r)
}

/// A hold is taken with one stroke of the court's own metabolism; a read
/// through `run` spends it and is counted on the holding thread; let go,
/// the stroke goes on whole with what it still grants.
#[test]
fn run_spends_from_one_stroke_hold() {
    let court = Mutex::new(Court::new());
    let stroke = Court::new().stroke().remaining();
    let mut h = held!(court).expect("the court");
    assert_eq!(h.remaining(), stroke, "the hold is taken with one stroke of the court");
    let before = crate::physics::spent_here();
    assert!(h.run(|_, reads| reads.spend()), "a read granted");
    assert_eq!(crate::physics::spent_here(), before + 1, "and counted on the holding thread");
    assert_eq!(h.remaining(), stroke - 1);
    let carried = h.into_stroke();
    assert_eq!(carried.remaining(), stroke - 1, "let go, the stroke goes on whole");
    assert!(holding().is_none(), "and the court stands free");
}

/// Every take through the guard is numbered in the book, the library's own
/// as the binary's.
#[test]
fn holds_counts_every_take() {
    let court = Mutex::new(Court::new());
    let before = holds();
    drop(held!(court).expect("the court"));
    drop(held!(court).expect("the court"));
    assert!(holds() >= before + 2, "two takes, two holds numbered");
}

/// A SPENT STROKE ENDS METERED WORK UNREAD, never with a wrong answer: the
/// helicity is `None`, the nerve census reads no complex, and the digest
/// confirmation reaches no stratum and leaves no cursor.
#[test]
fn run_on_ends_work_unread_on_spent_stroke() {
    let court = Mutex::new(Court::new());
    let mut h = held!(court).expect("the court");
    h.run(|k, reads| {
        while reads.spend() {}
        // No tube is held: no row to read, the helicity exactly zero.
        let (tubes, _, _, _, helicity) = k.tube_reading_on(reads);
        assert_eq!((tubes, helicity), (0, Some(num_rational::BigRational::from_integer(0.into()))));
        let (census, read) = k.nerve_census(reads);
        assert_eq!((census.len(), read), (0, 0), "no complex read on a spent stroke");
        let remote: Vec<crate::transducer::sheaf::StratumDigest> = (1..4u64)
            .map(|n| crate::transducer::sheaf::StratumDigest { inv: crate::transducer::stalk::Invariant::counts(&[n]), count: 1, root: root_of(n) })
            .collect();
        k.confirm_peer_holds_on(&remote, [5; 32], reads);
        assert!(k.confirm_cursor.get(&[5u8; 32]).is_none(), "nothing reached, nothing to resume past");
    });
}

/// THE CONFIRMATION RESUMES PAST THE STRATUM IT REACHED: on a stroke of one
/// read the first stratum is reached and the cursor stands on it; the next
/// digest takes the second, and a stroke that rounds the whole digest
/// leaves no cursor.
#[test]
fn confirm_peer_holds_on_resumes_past_reached_stratum() {
    let mut k = Court::new();
    let mut remote: Vec<crate::transducer::sheaf::StratumDigest> = (1..4u64)
        .map(|n| crate::transducer::sheaf::StratumDigest { inv: crate::transducer::stalk::Invariant::counts(&[n]), count: 1, root: root_of(n) })
        .collect();
    #[expect(clippy::disallowed_methods, reason = "rule 4 — a sort by the canonical key, the order digests stand in")]
    remote.sort_by(|a, b| a.inv.key().as_bytes().cmp(b.inv.key().as_bytes()));
    let peer = [6u8; 32];
    let key = |i: usize| remote[i].inv.key().as_bytes().to_vec();
    k.confirm_peer_holds_on(&remote, peer, &mut crate::physics::Physics::default().stroke_n(1));
    assert_eq!(k.confirm_cursor.get(&peer), Some(&key(0)), "one read: the first stratum");
    k.confirm_peer_holds_on(&remote, peer, &mut crate::physics::Physics::default().stroke_n(1));
    assert_eq!(k.confirm_cursor.get(&peer), Some(&key(1)), "the next digest takes the second");
    k.confirm_peer_holds_on(&remote, peer, &mut crate::physics::Physics::default().stroke_n(3));
    assert!(k.confirm_cursor.get(&peer).is_none(), "round once: the next starts from the first");
}

/// 78.182 item 5 — A WAITER READS THE HOLD IT WAITS BEHIND: a hold that has
/// spent past its stroke answers the waiter `Deferred` at once, naming the
/// holder's site, its spent reads and its stroke; the holder is flagged and
/// its next metered read is refused; let go, the court is taken again.
#[test]
fn waiter_behind_overrun_hold_is_deferred_and_holder_preempted() {
    let court: &'static Mutex<Court> = Box::leak(Box::new(Mutex::new(Court::new())));
    let (held_tx, held_rx) = std::sync::mpsc::channel();
    let (go_tx, go_rx) = std::sync::mpsc::channel::<()>();
    let holder = std::thread::spawn(move || {
        let mut h = held!(court).expect("the court");
        let stroke = u64::try_from(h.remaining()).expect("a stroke counts in u64");
        h.run(|_, reads| while reads.spend() {});
        assert!(crate::physics::Physics::default().stroke_n(1).spend(), "a read past the stroke, on a token of its own");
        held_tx.send(stroke).expect("the test waits");
        go_rx.recv().expect("the test answers");
        let after = crate::physics::Physics::default().stroke_n(1).spend();
        drop(h);
        let free = crate::physics::Physics::default().stroke_n(1).spend();
        (after, free)
    });
    let stroke = held_rx.recv().expect("the holder holds");
    let before = deferred_here();
    match held!(court) {
        Err(Refused::Deferred { holder, spent, stroke: s, .. }) => {
            assert_eq!((spent, s), (stroke + 1, stroke), "the holder's reads and its stroke, as the meter read them");
            assert!(holder.is_some_and(|h| h.contains("tests/meter.rs")), "the holding site named");
        }
        Err(Refused::Poisoned) => panic!("the lock is not poisoned"),
        Ok(_) => panic!("a hold past its stroke is never waited behind"),
    }
    assert_eq!(deferred_here(), before + 1, "the deferral counted on the waiting thread");
    go_tx.send(()).expect("the holder waits");
    let (after, free) = holder.join().expect("the holder ends");
    assert!(!after, "the flagged holder is refused its next metered read");
    assert!(free, "let go, its thread reads again");
    assert!(held!(court).is_ok(), "the court is taken again once let go");
}

/// A WAITER BEHIND A HOLD WITHIN ITS STROKE WAITS, and takes the court once
/// it is let go: a release is never missed between the try and the park.
#[test]
fn waiter_behind_hold_within_stroke_takes_court_on_release() {
    let court: &'static Mutex<Court> = Box::leak(Box::new(Mutex::new(Court::new())));
    let h = held!(court).expect("the court");
    let waiter = std::thread::spawn(move || held!(court).map(|g| g.remaining()).is_ok());
    // The waiter is parked or about to; the hold read nothing.
    std::thread::yield_now();
    drop(h);
    assert!(waiter.join().expect("the waiter ends"), "the waiter took the court once it was let go");
}
