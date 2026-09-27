// 78.49 — A VOICE ONLY SPEAKS: an identity and the acts its speech carries,
// no ledger and no clock.

use crate::anastome::dialogue;
use crate::voice::Voice;

/// A VOICE IS AN IDENTITY THAT SPEAKS. Seated by its key, it names itself,
/// and each act it speaks is framed under that identity; it holds nothing
/// else — no receipt, no K — so speaking moves nothing in it.
#[test]
fn a_voice_is_an_identity_that_speaks_and_holds_nothing_else() {
    let v = Voice::named("worker-a");
    assert_eq!(v.identity(), Voice::named("worker-a").identity());
    assert_ne!(v.identity(), Voice::named("worker-b").identity());
    let ask = v.quote().expect("the ask fits its lengths");
    assert_eq!(dialogue::act_tag(&ask), Some(dialogue::ACT_QUOTE));
    assert_eq!(dialogue::act_tag(&v.task()), Some(dialogue::ACT_TASK));
    assert_eq!(dialogue::act_tag(&v.syllabus()), Some(dialogue::ACT_SYLLABUS));
}
