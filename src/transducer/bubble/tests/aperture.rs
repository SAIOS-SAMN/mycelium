// 24.1 — the aperture over held thoughts: T room is measured in receipts,
// and the oldest bubble is the one that moves.

use crate::transducer::bubble::*;

#[test]
fn t_room_moves_the_oldest_bubble_to_cavity() {
    let mut c = Contemplation::new();
    c.set_now(10);
    c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
    c.set_now(50);
    c.hold(vec![(5, 5, 3, 4), (5, 6, 3, 4)]);
    assert_eq!(c.len(), 2);
    c.retire_on_t(1);
    assert_eq!(c.len(), 1, "T-room of one keeps the younger bubble");
    assert_eq!(c.cavity_len(), 1, "the older remainder is a cavity chart");
}
