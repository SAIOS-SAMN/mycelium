// O10 — the relation of a pair's two frames, typed.

use super::grid;
use crate::transducer::family::transduce::*;

/// EACH CLASS ON ITS WITNESS: a doubling carries every body by the
/// re-period alone; a `3 × 5` frame turned to `5 × 3` by the 90° turn and
/// no other swap; a frame repeated `2 × 2` is a cover; a crop holds its body
/// in the body's own enclosure; a body appearing in a shape no input body
/// has is the residue.
#[test]
fn relation_of_reads_each_class() {
    assert_eq!(
        relation_of(&grid(&["10", "02"]), &grid(&["1100", "1100", "0022", "0022"])),
        Ok(PairRelation::Carried(SignedSwap::IDENTITY))
    );
    assert_eq!(
        relation_of(&grid(&["12000", "10300", "00004"]), &grid(&["004", "000", "030", "200", "110"])),
        Ok(PairRelation::Carried(SignedSwap::TURN_90))
    );
    assert_eq!(relation_of(&grid(&["12"]), &grid(&["1212", "1212"])), Ok(PairRelation::Cover { rows: 2, cols: 2 }));
    assert_eq!(
        relation_of(&grid(&["0000", "0110", "0110", "0000"]), &grid(&["11", "11"])),
        Ok(PairRelation::Reenclosed(SignedSwap::IDENTITY))
    );
    assert_eq!(relation_of(&grid(&["1"]), &grid(&["10", "11"])), Ok(PairRelation::Unenclosed));
    assert_eq!(relation_of(&grid(&["1"]), &Vec::new()), Err(TransductionRefusal::Empty));
}
