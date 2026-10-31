use super::super::*;

/// 45.7 — a carrier's circuits are read in circuits and the circuits' period in
/// strokes, off the ring: three strokes to the first circuit, one to the
/// next.
#[test]
fn circuits_stroke_reads_circuits_and_period() {
    let mut o = Circuits::default();
    assert_eq!(o.stroke(0), None, "no circuit, no reading");
    assert_eq!(o.stroke(0), None);
    assert_eq!(o.stroke(1), Some((1, 3)), "the first circuit took three strokes");
    assert_eq!(o.stroke(1), Some((2, 1)), "a budget that covers the carrier circles it every stroke");
    assert_eq!(o.stroke(2), Some((4, 1)), "two circuits in one stroke are two");
    assert_eq!((o.circuits(), o.period()), (4, 1));
}
