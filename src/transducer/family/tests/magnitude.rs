// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 12.6 — THE MAGNITUDE A FAMILY PRESENTS: the genesis bond is the lane-wise
// sum over the family's disjoint pairs. Moved from the former `arc`
// module's tests (ROADMAP track 26, S1).

use crate::transducer::family::magnitude::{family_cover, pair_magnitude};
use crate::transducer::raster::painted_of;

/// 12.6: a bundle's pressure is the lane-wise SUM over its pairs, which
/// are disjoint scenes — never a maximum. A maximum would report this
/// two-pair family as no larger than its wider pair, so a bond measured
/// from it could not cover the family it was measured from.
#[test]
fn family_cover_sums_pairs_lane_wise() {
    type Pair = (crate::transducer::spelled::SectionSample, crate::transducer::spelled::SectionSample);
    let small: Pair = (vec![vec![1, 1]].into(), vec![vec![2, 2]].into());
    let wide: Pair = (vec![vec![1, 1, 1]].into(), vec![vec![2, 2, 2]].into());
    let cover = family_cover(&[small.clone(), wide.clone()]).expect("measured");
    let a = pair_magnitude(&small.0, &small.1).expect("small");
    let b = pair_magnitude(&wide.0, &wide.1).expect("wide");
    for l in 0..3 {
        assert_eq!(cover[l], a[l] + b[l], "lane {l} sums the disjoint scenes");
        assert!(cover[l] >= a[l].max(b[l]), "and therefore covers each pair");
    }
    let h = |p: &Pair| {
        crate::transducer::germinate::germinate(&painted_of(&p.0), &painted_of(&p.1))
            .map(|c| c.len() as u64)
            .expect("measured")
    };
    assert_eq!(
        cover[3],
        a[3] + b[3] + h(&small) + h(&wide),
        "H accumulates the coboundary extent with the rest"
    );
    assert!(family_cover(&[]).is_none(), "an empty family is unmeasured, not a zero bond");
}
