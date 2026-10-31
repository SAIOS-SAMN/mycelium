use super::super::*;
use super::common::*;

/// PH5 — a court's phase travels whole, and the later window stands.
#[test]
fn phase_round_trips_and_later_window_stands() {
    let phase = Phase { windows: 3, h0: 986, h1: 989, held: 1, sections: 1, strata: 2, installed: 1, ..Phase::default() };
    reads_back(&[Reading::Phase(phase.clone())]);
    let mut earlier = Reading::Phase(Phase::default());
    assert!(earlier.absorb(&Reading::Phase(phase.clone())), "one kind folds");
    assert_eq!(earlier, Reading::Phase(phase), "the later window stands");
}
