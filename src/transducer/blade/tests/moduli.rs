// 16, A6 — the moduli are exact coordinates of a chart: its prefix is the
// topological stratum, and a ratio is one coordinate whatever its width.

use super::common::*;
use crate::transducer::blade::*;

/// 24.4, A21 §2, Stage 12 S12.1a — A CHART'S PREFIX IS THE TOPOLOGICAL
/// STRATUM. Two rectangles of different proportions have the same
/// base, Betti numbers and turns and different moment ratios: their charts
/// differ, share the coarsest axes, and the atlas read at that depth holds
/// both — the stratum a peel reaches. The shared stratum is the
/// base's `n`, its `n` Betti numbers and the turns (BQ4), and it is read by
/// peel, never by the whole chart.
#[test]
fn a_chart_prefix_is_the_topological_stratum() {
    use crate::transducer::stalk::{Invariant, Stalks};
    let bar_support = rect(0, 0, 2, 3);
    let plank_support = rect(0, 0, 2, 5);
    let heard = |s: &[crate::transducer::base::Site]| moduli(&shape_of(s).blade).expect("heard at n = 3");
    let (a, b) = (heard(&bar_support), heard(&plank_support));
    assert_ne!(a, b, "different proportions, different charts");
    // THE TOPOLOGICAL STRATUM at the corpus's n = 3: the base's n, its n
    // Betti numbers, and the turns (BQ4).
    let n = 3usize;
    let depth = 1 + n + 1;
    assert_eq!(a[..depth], b[..depth], "one topology: n, the Betti numbers and turns lead the chart");
    assert_eq!(
        a[..depth],
        [3u64, 1, 0, 0, 4].map(crate::transducer::stalk::ChartAxis::Count),
        "the base of three, one component, no hole, no cavity, four turns"
    );
    let word_of = |support: &[crate::transducer::base::Site]| {
        let raw: Vec<(i64, i64, u8, u8)> =
            support.iter().map(|s| (s.axes()[0], s.axes()[1], 1u8, 2u8)).collect();
        let chain = crate::transducer::element::canonical_chain(&raw)
            .expect("one entry per site is A5's chain");
        crate::transducer::element::from_canonical(&chain).address()
    };
    let (wa, wb) = (word_of(&bar_support), word_of(&plank_support));
    let (ia, ib) = (Invariant(a.clone()), Invariant(b.clone()));
    let mut atlas = Stalks::default();
    atlas.admit(&ia, wa.clone());
    atlas.admit(&ib, wb.clone());
    assert_eq!(atlas.fiber(&ia).len(), 1, "the exact chart holds one word");
    // The stratum is the fibre UNDER the prefix (A21 §2): `fiber` reads one
    // node's own members and holds nothing at a prefix no word sits on.
    let stratum = atlas.fiber_under(&Invariant(a[..depth].to_vec()));
    assert!(
        stratum.contains(&wa.clone().into_vec()) && stratum.contains(&wb.clone().into_vec()),
        "the topological stratum holds both: {} members",
        stratum.len()
    );
    let mut at = ia.clone();
    while at.0.len() > depth {
        at = at.peel();
    }
    assert_eq!(at.0, a[..depth], "peel reaches the topological stratum");
    assert_eq!(atlas.fiber_under(&at).len(), 2, "and the stratum's fibre is read under that prefix");
}

/// N7 as corrected by A21 (ruled 2026-09-18) — A RATIO IS ONE COORDINATE,
/// WHATEVER ITS WIDTH. `axes_of` packed a rational into two `u64` limbs and
/// refused when either overflowed; `moduli` read the refusal as an empty
/// axis list and `stalk::invariant` as the empty chart, the root of the
/// trie, so bodies whose only fault was the width of their own moduli were
/// filed together there and with every word that has no support at all.
///
/// Widening the limbs alone would have moved the defect rather than closed
/// it: a two-limb axis is two positions, `peel` pops one, and the "coarser
/// stratum" left behind is half a rational. The ratio is a whole
/// `ChartAxis` instead, so its width is inside one coordinate.
#[test]
fn a_ratio_is_one_coordinate_whatever_its_width() {
    use crate::transducer::stalk::ChartAxis;
    let axis = |n: i64, d: i64| {
        ChartAxis::Ratio(BigRational::new(BigInt::from(n), BigInt::from(d)))
    };
    let neg = axis(-3, 7);
    assert_ne!(neg, axis(3, 7), "the sign is a coordinate, not dropped");
    // WIDER THAN ℤ₆₄ IS STILL ONE COORDINATE, and two distinct wide ratios
    // are two — which is the collapse this retires.
    let huge = ChartAxis::Ratio(BigRational::new(BigInt::from(1) << 80, BigInt::from(1)));
    let wider = ChartAxis::Ratio(BigRational::new(BigInt::from(1) << 81, BigInt::from(1)));
    assert_ne!(huge, wider, "two wide ratios are two coordinates, not one root bucket");
    // AND EACH ROUND-TRIPS THROUGH ITS OWN BYTES, which is the unit the
    // chart key and the stratum trie both branch on (law 2).
    for a in [neg, huge, wider] {
        assert_eq!(
            ChartAxis::from_key_bytes(&a.key_bytes()),
            Some(a.clone()),
            "a coordinate is recovered from its key exactly"
        );
    }
}

/// The same reading one level up: a body whose harmonics exceed ℤ₆₄ has a
/// chart of its own, where it was charted at the root before N7.
#[test]
fn a_body_is_never_charted_at_the_root_for_the_width_of_its_moduli() {
    use crate::transducer::stalk::invariant;
    let l: Vec<(i64, i64, u8, u8)> = vec![(0, 0, 1, 2), (1, 0, 1, 2), (2, 0, 1, 2), (2, 1, 1, 2)];
    let inv = invariant(&l);
    assert!(!inv.0.is_empty(), "a body with support is charted by its moduli");
    // The root belongs to genuine absence alone (`invariant_of`'s no-support
    // arm), which is above this and untouched.
    assert!(invariant(&[]).0.is_empty(), "and no support is still the root");
}
