// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-SENSING §2; ROADMAP track 26 S1 — the rank-2 raster at intake: its
// painted sites, the frame's momentum and the envelopes' dilation it reads,
// and the bridge to the sample. Moved from the former `arc` module's tests;
// the corpus's own claims are the decoder's (`sources/arc-agi/tests`).

use crate::transducer::family::{carried_site, envelope_dilation};
use crate::transducer::raster::*;

/// 78.56 — THE ENVELOPES' DILATION: two rasters of one extent correspond
/// site for site; 3×3 onto 9×9 is s = 3, carrying each site's centre to
/// the centre of its 3×3 block, never to a corner; an extent whose
/// envelope ratio is no rational square corresponds nowhere (78.45); and
/// a point that lands between sites carries nothing.
#[test]
fn envelope_dilation_carries_sites_between_extents() {
    let raster = |h: usize, w: usize| -> Vec<Vec<u8>> { vec![vec![1; w]; h] };
    assert_eq!(envelope_dilation(&raster(3, 4), &raster(3, 4)), Some((1, 1)));
    assert_eq!(envelope_dilation(&raster(3, 3), &raster(9, 9)), Some((3, 1)));
    assert_eq!(carried_site(&raster(3, 3), &raster(9, 9), (3, 1), [1, 1]), Some([4, 4]), "centre to centre");
    assert_eq!(carried_site(&raster(3, 3), &raster(9, 9), (3, 1), [0, 0]), Some([1, 1]), "a site to its block's centre");
    assert_eq!(envelope_dilation(&raster(1, 1), &raster(1, 2)), None, "√(5/2) is no rational");
    assert_eq!(carried_site(&raster(2, 2), &raster(4, 4), (2, 1), [0, 0]), None, "between sites");
}

/// 78.56 — THE MOMENTUM: a change of symbol in place carries none; a drift
/// one way does.
#[test]
fn momentum_vanishes_only_without_drift() {
    let still = vec![(vec![vec![1, 1, 0]], vec![vec![2, 2, 0]])];
    let drift = vec![(vec![vec![1, 1, 0]], vec![vec![0, 1, 1]])];
    assert!(momentum_vanishes(&still));
    assert!(!momentum_vanishes(&drift));
}

/// A raster's painted sites omit the ground, each kept as its two
/// coordinates and its symbol.
#[test]
fn painted_of_omits_ground() {
    let painted = painted_of(&[vec![0, 1], vec![2, 0]]);
    assert_eq!(painted, vec![(0, 1, 1), (1, 0, 2)], "only novelty (off the ground)");
    assert!(painted_of(&[vec![0, 0], vec![0, 0]]).is_empty(), "all ground → no painted site");
}

/// TRACK 26, S4 — THE SAMPLE BRIDGE: a raster reaches the transduction as a
/// sample of rank 2 and width 1 in canonical address order, and a spoken
/// sample returns to the same raster; a ragged raster and an empty one are
/// refused at intake, never padded.
#[test]
fn sample_bridge_reads_raster_as_rank_two_sample() {
    use crate::transducer::sample::{AxisReading, Place, SampleRefusal};
    let g: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let s = sample_of(&g, Place::new(vec![0])).expect("the raster stands");
    assert_eq!((s.rank(), s.width(), s.extent(0), s.extent(1)), (2, 1, AxisReading::Read(2), AxisReading::Read(3)));
    assert_eq!(raster_of(&s), Some(g));
    assert!(matches!(sample_unplaced(&[vec![1, 2], vec![3]]), Err(SampleRefusal::Length { .. })));
    assert_eq!(sample_unplaced(&[]), Err(SampleRefusal::EmptyAxis { axis: 0 }));
}

/// THE POSTED TASK ROUND-TRIPS EXACT SAMPLES (ROADMAP track 26, S1): a task
/// written as a decoder posts it reads back as the same task, every raster
/// the same sample, and a withheld (empty) output stays empty, so the
/// hypha, the soak and a client's feed read the task without the decoder.
#[test]
fn posted_task_round_trips_exact_samples() {
    use crate::anastome::dialogue::{decode_posted_task, encode_posted_task};
    use crate::transducer::family::Task;
    let task = Task {
        train: vec![(vec![vec![0, 1], vec![2, 0]], vec![vec![1, 0], vec![0, 2]]), (vec![vec![7]], vec![vec![7, 7]])],
        test: vec![(vec![vec![3, 4, 5]], Vec::new())],
    };
    let bytes = encode_posted_task(&task).expect("the task posts");
    let back = decode_posted_task(&bytes).expect("the posted task reads");
    assert_eq!(back, task, "the same task");
    let samples = |t: &Task| -> Vec<_> {
        t.train.iter().chain(&t.test).flat_map(|(x, y)| [x, y]).filter(|g| !g.is_empty()).map(|g| sample_unplaced(g)).collect()
    };
    assert_eq!(samples(&back), samples(&task), "every raster the same sample");
    assert!(back.test[0].1.is_empty(), "a withheld output stays withheld");
    assert_eq!(decode_posted_task(&bytes[..bytes.len() - 1]), None, "a cut file is no task");
    let mut longer = bytes.clone();
    longer.push(0);
    assert_eq!(decode_posted_task(&longer), None, "nothing after the last pair");
}
