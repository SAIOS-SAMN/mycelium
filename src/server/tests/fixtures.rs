// MU-SENSING §6, §3.9; ROADMAP track 26 S4c — A POSTED TASK OF RANK 3, exact
// samples at their positions `[split, pair, member]`: the fixture the
// hypha's rank-n tests and the end-to-end factorisation test share.

use crate::anastome::dialogue::samples::{position, Member, Split};
use crate::anastome::dialogue::PostedTask;
use crate::transducer::sample::Sample;

/// One sample of extents `2 × 2 × 3` at width 1, its symbols written as
/// digits in canonical address order, at `[split, pair, member]`.
pub(crate) fn sample_at(digits: &str, split: Split, pair: usize, member: Member) -> Sample {
    let place = position(split, pair, member).expect("a position");
    Sample::new(vec![2, 2, 3], 1, digits.bytes().map(|b| b - b'0').collect(), place).expect("the fixture stands")
}

/// THE RANK-3 POSTED TASK: two train pairs, each a body of `1` at one site
/// and the far corner painted `2` in the output, and one test pair whose
/// output the court withholds. The alphabet does not spell rank 3.
pub(crate) fn rank_three_task() -> PostedTask {
    let pair = |i: usize, x: &str, y: &str| (sample_at(x, Split::Train, i, Member::Input), sample_at(y, Split::Train, i, Member::Output));
    PostedTask {
        train: vec![pair(0, "100000000000", "100000000002"), pair(1, "010000000000", "010000000002")],
        test: vec![(
            sample_at("001000000000", Split::Test, 0, Member::Input),
            Some(sample_at("001000000002", Split::Test, 0, Member::Output)),
        )],
    }
}
