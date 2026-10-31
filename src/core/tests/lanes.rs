// STD1 (rule 11) — A MEASURE PAST ITS CARRIER IS REFUSED, WHOLE.

use crate::core::lanes::{lanes_step, lanes_sum};

/// A step one lane of which would pass its carrier moves no lane; one that
/// fits moves all four.
#[test]
fn lanes_step_refuses_overflow_whole() {
    let mut acc = [1, u64::MAX, 0, 0];
    assert!(!lanes_step(&mut acc, &[1, 1, 1, 1]), "one lane would pass its carrier");
    assert_eq!(acc, [1, u64::MAX, 0, 0], "no lane moved");
    assert!(lanes_step(&mut acc, &[1, 0, 2, 3]));
    assert_eq!(acc, [2, u64::MAX, 2, 3]);
    assert_eq!(lanes_sum([[1, 2, 3, 4], [4, 3, 2, 1]]), Some([5; 4]));
    assert_eq!(lanes_sum([[u64::MAX, 0, 0, 0], [1, 0, 0, 0]]), None, "a sum past the carrier reads nothing");
}
