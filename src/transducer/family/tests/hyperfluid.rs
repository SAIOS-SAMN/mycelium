// 78.83 — TIME'S OWN CONGRUENCE (`family/hyperfluid.rs`, HR5): a ray is a
// point on the null flow of its plane `d ∧ e∞`, its length its time; two
// transports are one flow at two times where they differ only there; and a
// free ray's time is read from what every observation of it agreed on.

use super::common::grid;
use crate::transducer::family::hyperfluid::{congruence, flows, reconstruct_in_time, Congruence};
use crate::transducer::family::reconstruct::reconstruct;

/// ONE FLOW AT TWO TIMES. A dot emits hue 4 rightward for two sites on one
/// pair and three on another, ending in open space both times: the rays lie
/// on one null plane with one hue and one kind of end — congruent in time,
/// not equal. Emitting another hue lifts the light; recolouring the dot
/// lifts the gauge.
#[test]
fn congruence_reads_two_lengths_as_one_flow_in_time() {
    let short = flows(&grid(&["300000"]), &grid(&["344000"])).expect("one frame");
    let long = flows(&grid(&["300000"]), &grid(&["344400"])).expect("one frame");
    assert_eq!(congruence(&short[0], &long[0]), Congruence::InTime);
    let other = flows(&grid(&["300000"]), &grid(&["355000"])).expect("one frame");
    assert_eq!(congruence(&short[0], &other[0]), Congruence::Light, "another hue: the light lifts");
    let recoloured = flows(&grid(&["300000"]), &grid(&["744000"])).expect("one frame");
    assert_eq!(congruence(&short[0], &recoloured[0]), Congruence::Gauge, "the dot recoloured: the gauge lifts");
    assert_eq!(short[1].to, 0, "a site the light reaches moves nothing of its own: the light carries its colour");
}

/// THE TIME, READ. Each pair's dot emits hue 4 rightward for exactly two
/// sites, ending in open space, at a different place each time. Frozen in
/// time (HR4) a ray counts only where it fills its whole reach, so nothing is
/// carried; in time (HR5) the free ray's time is the one every other pair
/// agreed on, and every pair left out is rebuilt.
#[test]
fn reconstruct_in_time_runs_free_ray_agreed_time() {
    let family = vec![
        (grid(&["0300000", "0000000"]), grid(&["0344000", "0000000"])),
        (grid(&["0000000", "3000000"]), grid(&["0000000", "3440000"])),
        (grid(&["0003000", "0000000"]), grid(&["0003440", "0000000"])),
    ];
    let frozen = reconstruct(&family);
    assert!(!frozen.whole, "frozen in time, a free ray is no transport");
    let it = reconstruct_in_time(&family);
    assert_eq!((it.framed, it.reproduced), (3, 3));
    assert!(it.whole);
}
