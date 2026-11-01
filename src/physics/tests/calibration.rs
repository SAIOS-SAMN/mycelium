use super::super::*;

/// PH1 — THE RATIO STANDS EXACT ON THIS HOST AND THE FLOOR IS THE
/// HOST'S: the reserve is a whole number of spends, the aperture is the
/// two-stroke ratio itself, a spend halves exactly, and the stroke floor
/// is a positive measurement every park is at least one of.
#[test]
fn spend_unit_ratio_is_exact_and_stroke_floor_positive() {
    assert_eq!(spend_unit() * STROKES_PER_RESERVE, reserve_max(), "a whole number of spends");
    assert_eq!(u64::from(aperture_cycle()), STROKES_PER_RESERVE, "the aperture is the ratio");
    assert!(spend_unit() >= 2, "the reserve's floor leaves a spend the regeneration can halve");
    let floor = stroke_floor();
    assert!(!floor.is_zero(), "the floor is a measurement, never zero");
    assert_eq!(idle_park(None), floor, "an unmeasured wire parks one stroke, the unit");
    assert_eq!(idle_park(std::num::NonZeroU64::new(3)), floor * 3, "a measured wire parks its round trip");
}

/// D2.6 — A HOST THAT DOES NOT SAY HOW MANY CORES IT HAS IS REFUSED BY NAME,
/// never run on one core: the read's refusal is `Uncalibrated::Cores`.
#[test]
fn calibration_refuses_unread_cores() {
    let unread = Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "not said"));
    assert_eq!(cores_of(unread), Err(Uncalibrated::Cores));
    assert_eq!(cores_of(Ok(std::num::NonZeroUsize::MIN)).map(std::num::NonZeroU64::get), Ok(1));
    assert_eq!(calibrate(), Ok(()), "this host says");
}

/// D2.6 — THE STROKE FLOOR READS THE TIMER SLACK WHERE THE TICK IS
/// UNPUBLISHED: one tick where `CONFIG_HZ` is published; else the thread's
/// slack, the grant every shorter ask is rounded to; else refused by name —
/// never a nanosecond asked in its place.
#[test]
fn stroke_floor_reads_slack_where_hz_unpublished() {
    let ns = |hz, slack| stroke_floor_of(hz, slack).map(|d| d.as_nanos());
    assert_eq!(ns(Some(250), Some(50_000)), Ok(4_000_000), "the tick, where published");
    assert_eq!(ns(None, Some(50_000)), Ok(50_000), "the slack, where it is not");
    assert_eq!(stroke_floor_of(None, None), Err(Uncalibrated::StrokeFloor));
    assert_eq!(stroke_floor_of(None, Some(0)), Err(Uncalibrated::StrokeFloor), "a zero slack is no floor");
}
