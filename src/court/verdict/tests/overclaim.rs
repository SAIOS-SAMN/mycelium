// 23.1 step 3 against 21 step 1 — A MEMBER CLAIMS ONLY THE FRAMES ITS GAUGE
// REPRODUCES: the mind's extraction, replayed by the court's own body check,
// raises no vertex-body fault where the frames' hue maps differ (the
// over-claim answered at the pivot, the operator 2026-10-18).

use crate::transducer::family::{extract_rule, Grid};
use crate::verdict::body::vertices_carry_frames;

fn digits(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// THREE FRAMES, ONE SLIDE, THREE HUE MAPS. One cell slides one column right
/// in every frame, recoloured 1 → 2, then 3 → 4, then 3 → 5. The second and
/// third frames each agree with the first's map (no hue sent two ways) and
/// disagree with each other, so no one gauge reproduces all three. The
/// member formerly kept the first frame's gauge and claimed every frame, and
/// the court's replay landed the second frame's compartment on hue 3 where 4
/// stood: "a lamina lands on the wrong hue". Now each claimed frame is one
/// its gauge reproduces, and the court's body check finds nothing to refuse.
#[test]
fn a_member_claims_only_the_frames_its_gauge_reproduces() {
    let family = vec![
        (digits(&["000", "010", "000"]), digits(&["000", "002", "000"])),
        (digits(&["000", "030", "000"]), digits(&["000", "004", "000"])),
        (digits(&["000", "030", "000"]), digits(&["000", "005", "000"])),
    ];
    let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the slide extracts");
    assert_eq!(vertices_carry_frames(&rule.form, &family).map(|(_, why)| why), None, "no member claims a frame its gauge does not reproduce");
}
