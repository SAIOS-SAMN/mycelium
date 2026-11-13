// 78.183 §7.5 — the metabolic rate per lane over a bell span.

use crate::physics::rate::{open_draw, Rate};
use crate::physics::Physics;

/// THE THROAT RELEASES HALF: a lane whose span booked as much entropy as
/// syntropy (`t = 1`) may draw half its holding; one running entropic draws
/// more and one running syntropic less; one that booked nothing draws
/// nothing (an idle court spends nothing).
#[test]
fn open_draw_is_the_open_share_of_the_holding() {
    assert_eq!(open_draw(5, 5, 1000), Some(500), "the throat releases half");
    assert_eq!(open_draw(3, 1, 1000), Some(750), "entropic draws more");
    assert_eq!(open_draw(1, 3, 1000), Some(250), "syntropic draws less");
    assert_eq!(open_draw(0, 0, 1000), Some(0), "an idle lane draws nothing");
    assert_eq!(open_draw(0, 4, 1000), Some(0), "pure consolidation draws nothing");
    assert_eq!(open_draw(4, 0, 1000), Some(1000), "pure expansion may draw all it holds");
}

/// A SPAN CLOSES ONCE PER BELL, on the lane's own events since the last
/// close (the reserve's ledger, in reads), per lane and never summed: the entropic lane's draw follows its
/// own events, the lane that booked nothing draws nothing, and a second
/// close at the same bell reads nothing.
#[test]
fn rate_closes_per_bell_on_each_lane_own_events() {
    let mut p = Physics::default();
    let mut r = Rate::default();
    let first = r.close(&p, 1).expect("the first span closes");
    assert!(first.iter().all(|l| l.drawn == 0), "nothing booked, nothing drawn");
    let give = p.reserve.lanes()[0] / 2;
    assert!(give > 0, "the reserve holds something to give");
    assert!(p.reserve.draw([give, 0, 0, 0]).is_ok(), "S gives to expansion");
    p.reserve.restore([give, 3, 0, 0]);
    assert_eq!(r.close(&p, 1), None, "the span stays open until the bell moves");
    let span = r.close(&p, 2).expect("the bell moved");
    let held = p.reserve.lanes();
    assert_eq!((span[0].entropic, span[0].syntropic), (give, give));
    assert_eq!(span[0].drawn, held[0] / 2, "S at the throat releases half");
    assert_eq!((span[1].entropic, span[1].syntropic, span[1].drawn), (0, 3, 0), "G consolidating draws nothing");
    assert_eq!(span[2].drawn, 0, "T booked nothing");
    let next = r.close(&p, 3).expect("the next bell");
    assert_eq!((next[0].entropic, next[0].syntropic), (0, 0), "events are read once, per span");
}
