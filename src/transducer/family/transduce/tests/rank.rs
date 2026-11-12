// ROADMAP track 26, S4 (MU-SENSING §3.1, §3.6) — the transduction at rank
// `n`: one path for every rank, no phase privileged, the frame's measure
// the product of its extents.

use super::{changes, delta, dilated, extents, ground, moved_sitewise, record, sample_by, sample_n, stroke};
use crate::transducer::family::transduce::*;
use crate::transducer::sample::Sample;

/// The frame with one site's symbol replaced.
fn changed_at(s: &Sample, site: &[i64], symbol: u8) -> Sample {
    sample_by(&extents(s), |t| if t == site { symbol } else { super::at(s, t) })
}

/// The signed permutations that carry every body of `x` onto `y`: the
/// readings of the pair under the whole group, each one generated.
fn carrying(x: &Sample, y: &Sample) -> Vec<SignedPermutation> {
    let from = Enclosed::of(x);
    SignedPermutation::all(x.rank())
        .filter(|g| {
            let moved = g.spoken_onto(&from, &g.enclosure(&from.enclosure).expect("one rank")).expect("own periods");
            let need = need_of(&moved, y).expect("one rank");
            let t = Transduction::of(&moved, y, &mut stroke(need)).expect("the need is the stroke");
            t.relations().iter().all(Result::is_ok)
        })
        .collect()
}

/// ONE CODE PATH RELATES PAIRS OF RANK 1, 2 AND 3: for each rank, a frame
/// with ground and two bodies against itself with one site repainted
/// projects to the site-by-site difference and changes `1/measure` of the
/// frame; and against its dilation by 2 on every phase carries every body,
/// the relation reading `Carried` by the identity of its own rank. Nothing
/// in the path names the rank.
#[test]
fn rank_one_and_rank_three_samples_perceived_by_one_path() {
    let frames = [
        sample_n(&[5], "01102"),
        sample_n(&[2, 3], "011002"),
        sample_n(&[2, 2, 3], "011000000002"),
    ];
    for x in &frames {
        let n = x.rank();
        let last: Vec<i64> = extents(x).iter().map(|&e| e as i64 - 1).collect();
        let y = changed_at(x, &last, 3);
        let t = record(x, &y);
        assert!(t.keeps_periods());
        assert_eq!(changes(&t.projected().expect("one enclosure")), delta(x, &y), "rank {n}");
        let measure = Enclosure::of(x).measure().expect("a measure") as u64;
        assert_eq!(t.share(), crate::core::Q::new(1.into(), measure.into()), "rank {n}");
        let doubled = dilated(x, &vec![2; n]);
        assert!(record(x, &doubled).relations().iter().all(Result::is_ok), "rank {n}");
        assert_eq!(
            relation_of(x, &doubled, &ground()),
            Ok(PairRelation::Carried(SignedPermutation::identity(n))),
            "rank {n}"
        );
    }
}

/// NO PHASE IS PRIVILEGED: relabelling the axes of both samples of a rank-3
/// pair by each permutation `π` of the three phases leaves the need, the
/// shares by lane, the bodies and the carriage of every body as they were;
/// it permutes the projected record's sites exactly; and the set of signed
/// permutations that carry the one frame onto the other becomes its
/// conjugate `π g π⁻¹`, element for element.
#[test]
fn no_axis_is_privileged() {
    let x = sample_n(&[2, 3, 2], "011020000330");
    // `g` moves the frame's phases `(0, 1, 2)` to `(−2, 0, −1)`; the output
    // is the moved frame dilated by `(2, 1, 3)`.
    let g = SignedPermutation::new(vec![(2, Sense::Reversed), (0, Sense::Kept), (1, Sense::Reversed)]).expect("a signed permutation");
    let y = dilated(&moved_sitewise(&x, &g), &[2, 1, 3]);
    let z = changed_at(&x, &[1, 2, 0], 4);
    let held = carrying(&x, &y);
    assert!(held.contains(&g), "the motion that made the pair carries it");
    for pi in SignedPermutation::all(3).filter(|p| p.images().iter().all(|&(_, s)| s == Sense::Kept)) {
        let (xr, yr, zr) = (moved_sitewise(&x, &pi), moved_sitewise(&y, &pi), moved_sitewise(&z, &pi));
        assert_eq!(need_of(&xr, &yr), need_of(&x, &y), "{pi:?}");
        let (t, tr) = (record(&x, &y), record(&xr, &yr));
        assert_eq!(tr.lane_shares(&ground()), t.lane_shares(&ground()), "{pi:?}");
        assert_eq!((tr.from.bodies, tr.to.bodies), (t.from.bodies, t.to.bodies), "{pi:?}");
        let ok = |t: &Transduction| t.relations().iter().filter(|r| r.is_ok()).count();
        assert_eq!(ok(&tr), ok(&t), "{pi:?}");
        // The projected record of one enclosure: its sites relabelled.
        let mut expected: Vec<(Vec<i64>, Vec<u8>, Vec<u8>)> = changes(&record(&x, &z).projected().expect("one enclosure"))
            .into_iter()
            .map(|(s, a, b)| (pi.images().iter().map(|&(k, _)| s[k]).collect(), a, b))
            .collect();
        expected.sort();
        assert_eq!(changes(&record(&xr, &zr).projected().expect("one enclosure")), expected, "{pi:?}");
        // The carrying set conjugated: `π⁻¹`, then `h`, then `π`.
        // Compared as the sets of the elements' matrices.
        let mut conjugated: Vec<Vec<Vec<i64>>> =
            held.iter().map(|h| pi.inverse().then(h).and_then(|k| k.then(&pi)).expect("one rank").matrix()).collect();
        let mut relabelled: Vec<Vec<Vec<i64>>> = carrying(&xr, &yr).iter().map(SignedPermutation::matrix).collect();
        conjugated.sort_unstable();
        relabelled.sort_unstable();
        assert_eq!(relabelled, conjugated, "{pi:?}");
    }
}

/// THE FRAME'S MEASURE IS THE PRODUCT OF ITS EXTENTS (78.170 O1 at rank
/// `n`): 5 at rank 1, 6 at rank 2, 24 at rank 3; S and G of a chain are
/// shares of it, so `[3, 2, 0, 1]` on a `2 × 3 × 4` frame is
/// `[3/24, 2/24, 0, 1]`; and one changed site of that frame is `1/24` of it.
#[test]
fn frame_measure_is_product_of_extents() {
    for (e, m) in [(vec![5usize], 5usize), (vec![2, 3], 6), (vec![2, 3, 4], 24)] {
        assert_eq!(Enclosure::of(&sample_by(&e, |_| 0)).measure(), Some(m));
    }
    let frame = sample_by(&[2, 3, 4], |_| 0);
    let mu = share_of_enclosure([3, 2, 0, 1], Some(&frame));
    let q = |n: i64, d: i64| crate::core::Q::new(n.into(), d.into());
    assert_eq!([mu.lane(0), mu.lane(1), mu.lane(2), mu.lane(3)], [q(3, 24), q(2, 24), q(0, 1), q(1, 1)]);
    let y = changed_at(&frame, &[1, 2, 3], 7);
    assert_eq!(record(&frame, &y).share(), q(1, 24));
}
