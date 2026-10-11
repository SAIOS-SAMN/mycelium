// AT1 Q4 — THE FRAME'S SPELLING, the coordinate a chart at frame depth is
// written by.

/// AT1 Q4 — THE FRAME'S SPELLING is unbounded and its byte order is the
/// numeric order: 255 before 256 before 65 536, where u16 LE put 256 first
/// and LEB128 orders by the low bits.
#[test]
fn the_frame_spelling_orders_as_the_frames_do() {
    use crate::transducer::sentence::form::atlas::frame_spelling;
    let frames = [0u64, 1, 2, 255, 256, 65_535, 65_536, u64::MAX];
    let spelled: Vec<Vec<u8>> = frames.iter().map(|&k| frame_spelling(k)).collect();
    assert!(spelled.windows(2).all(|w| w[0] < w[1]), "byte order is numeric order");
    assert_eq!(frame_spelling(0), vec![0], "zero is the empty body");
    assert_eq!(frame_spelling(256), vec![2, 1, 0]);
    assert_eq!(frame_spelling(u64::MAX).len(), 9, "no wall below the value's own width");
}
