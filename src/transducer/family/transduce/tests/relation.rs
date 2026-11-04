// O10 — the relation of a pair's two frames, typed.

use super::{ground, sample};
use crate::transducer::family::transduce::*;

/// EACH CLASS ON ITS WITNESS: a doubling carries every body by the
/// re-period alone; a `3 × 5` frame turned to `5 × 3` by the quarter turn
/// and no element before it in the group's order; a frame repeated `2 × 2`
/// is a cover; a crop holds its body in the body's own enclosure; a body
/// appearing in a shape no input body has is the residue; frames of two
/// ranks are refused.
#[test]
fn relation_of_reads_each_class() {
    let g = &ground();
    assert_eq!(
        relation_of(&sample(&["10", "02"]), &sample(&["1100", "1100", "0022", "0022"]), g),
        Ok(PairRelation::Carried(SignedPermutation::identity(2)))
    );
    let quarter = SignedPermutation::new(vec![(1, Sense::Reversed), (0, Sense::Kept)]).expect("a signed permutation");
    assert_eq!(
        relation_of(&sample(&["12000", "10300", "00004"]), &sample(&["004", "000", "030", "200", "110"]), g),
        Ok(PairRelation::Carried(quarter))
    );
    assert_eq!(relation_of(&sample(&["12"]), &sample(&["1212", "1212"]), g), Ok(PairRelation::Cover { copies: vec![2, 2] }));
    assert_eq!(
        relation_of(&sample(&["0000", "0110", "0110", "0000"]), &sample(&["11", "11"]), g),
        Ok(PairRelation::Reenclosed(SignedPermutation::identity(2)))
    );
    assert_eq!(relation_of(&sample(&["1"]), &sample(&["10", "11"]), g), Ok(PairRelation::Unenclosed));
    assert_eq!(
        relation_of(&super::sample_n(&[1], "1"), &sample(&["1"]), g),
        Err(TransductionRefusal::Rank { from: 1, to: 2 })
    );
}
