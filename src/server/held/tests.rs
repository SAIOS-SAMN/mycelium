// 78.153b — THE HOLD'S ONE STROKE: a hold is taken with the court's stroke,
// the only way in hands it beside the kernel, a metered read spends it on
// the holding thread's count, and a spent stroke ends metered work unread.

use super::*;
use std::sync::Mutex;

/// A hold is taken with one stroke of the court's own metabolism; a read
/// through `run` spends it and is counted on the holding thread; let go,
/// the stroke goes on whole with what it still grants.
#[test]
fn a_metered_path_spends_the_holds_one_stroke() {
    let kernel = Mutex::new(Kernel::new());
    let stroke = Kernel::new().stroke().remaining();
    let mut h = held!(kernel).expect("the kernel");
    assert_eq!(h.remaining(), stroke, "the hold is taken with one stroke of the court");
    let before = crate::physics::spent_here();
    assert!(h.run(|_, reads| reads.spend()), "a read granted");
    assert_eq!(crate::physics::spent_here(), before + 1, "and counted on the holding thread");
    assert_eq!(h.remaining(), stroke - 1);
    let carried = h.into_stroke();
    assert_eq!(carried.remaining(), stroke - 1, "let go, the stroke goes on whole");
    assert!(holding().is_none(), "and the kernel stands free");
}

/// Every take through the guard is numbered in the book, the library's own
/// as the binary's.
#[test]
fn every_take_is_counted_in_the_book() {
    let kernel = Mutex::new(Kernel::new());
    let before = holds();
    drop(held!(kernel).expect("the kernel"));
    drop(held!(kernel).expect("the kernel"));
    assert!(holds() >= before + 2, "two takes, two holds numbered");
}

/// A SPENT STROKE ENDS METERED WORK UNREAD, never with a wrong answer: the
/// helicity is `None`, the nerve census reads no complex, and the digest
/// confirmation reaches no stratum and leaves no cursor.
#[test]
fn a_spent_stroke_ends_metered_work_unread() {
    let kernel = Mutex::new(Kernel::new());
    let mut h = held!(kernel).expect("the kernel");
    h.run(|k, reads| {
        while reads.spend() {}
        // No tube is held: no row to read, the helicity exactly zero.
        let (tubes, _, _, _, helicity) = k.tube_reading_on(reads);
        assert_eq!((tubes, helicity), (0, Some(num_rational::BigRational::from_integer(0.into()))));
        let (census, read) = k.nerve_census(reads);
        assert_eq!((census.len(), read), (0, 0), "no complex read on a spent stroke");
        let remote: Vec<crate::transducer::sheaf::StratumDigest> = (1..4u64)
            .map(|n| crate::transducer::sheaf::StratumDigest { inv: crate::transducer::stalk::Invariant::counts(&[n]), count: 1, root: vec![n as u8] })
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
fn the_digest_confirmation_resumes_past_the_stratum_it_reached() {
    let mut k = Kernel::new();
    let mut remote: Vec<crate::transducer::sheaf::StratumDigest> = (1..4u64)
        .map(|n| crate::transducer::sheaf::StratumDigest { inv: crate::transducer::stalk::Invariant::counts(&[n]), count: 1, root: vec![n as u8] })
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
