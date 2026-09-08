// 45.4 — the voice's clock is its own admitted acts, and no reply moves it.

use crate::anastome::dialogue;
use crate::voice::Voice;

/// 45.4 — A VOICE'S CLOCK IS ITS OWN ACTS. It seats at zero and steps
/// once per admitted act; nothing sets it, so a reply carrying another
/// kernel's k-step cannot move it.
#[test]
fn a_voice_counts_its_own_admitted_acts() {
    let mut v = Voice::named("worker-a");
    assert_eq!(v.identity(), Voice::named("worker-a").identity());
    assert_ne!(v.identity(), Voice::named("worker-b").identity());
    assert_eq!(v.k(), 0, "a voice that has spoken nothing accrues nothing");
    assert_eq!(v.advance(), 1, "one admitted act is one step");
    assert_eq!(v.advance(), 2);
    assert_eq!(v.k(), 2);
    assert_eq!(v.quote()[0], dialogue::ACT_QUOTE);
}
