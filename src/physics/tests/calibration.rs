use super::super::*;

/// PH1 — THE RATIO STANDS EXACT ON THIS HOST AND THE FLOOR IS THE
/// HOST'S: the reserve is a whole number of spends, the aperture is the
/// two-stroke ratio itself, a spend halves exactly, and the stroke floor
/// is a positive measurement every park is at least one of.
#[test]
fn spend_unit_ratio_exact_and_stroke_floor_positive() {
    assert_eq!(spend_unit() * STROKES_PER_RESERVE, reserve_max(), "a whole number of spends");
    assert_eq!(u64::from(aperture_cycle()), STROKES_PER_RESERVE, "the aperture is the ratio");
    assert!(spend_unit() >= 2, "the reserve's floor leaves a spend the regeneration can halve");
    let floor = stroke_floor();
    assert!(!floor.is_zero(), "the floor is a measurement, never zero");
    assert_eq!(idle_park(None), floor, "an unmeasured wire parks one stroke");
    assert_eq!(idle_park(Some(0)), floor, "a same-stroke reply still parks the stroke it parks");
    assert_eq!(idle_park(Some(3)), floor * 3, "a measured wire parks its round trip");
}
