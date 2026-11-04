// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-SENSING §2; ROADMAP track 26 S1 — the rank-2 raster at intake: its
// painted sites, the frame's momentum and the envelopes' dilation it reads,
// and the bridge to the sample. Moved from the former `arc` module's tests;
// the corpus's own claims are the decoder's (`sources/arc-agi/tests`).

use crate::transducer::family::{carried_site, envelope_dilation};
use crate::transducer::raster::*;
use crate::transducer::spelled::{pairs_of, SectionSample, SectionSite};

/// 78.56 — THE ENVELOPES' DILATION: two rasters of one extent correspond
/// site for site; 3×3 onto 9×9 is s = 3, carrying each site's centre to
/// the centre of its 3×3 block, never to a corner; an extent whose
/// envelope ratio is no rational square corresponds nowhere (78.45); and
/// a point that lands between sites carries nothing.
#[test]
fn envelope_dilation_carries_sites_between_extents() {
    let raster = |h: usize, w: usize| SectionSample::from(vec![vec![1; w]; h]);
    let dilation = |a: SectionSample, b: SectionSample| envelope_dilation(a.sample(), b.sample());
    assert_eq!(dilation(raster(3, 4), raster(3, 4)), Some((1, 1)));
    assert_eq!(dilation(raster(3, 3), raster(9, 9)), Some((3, 1)));
    let site = SectionSite::on_plane;
    assert_eq!(carried_site(&raster(3, 3), &raster(9, 9), (3, 1), site([1, 1])), Some(site([4, 4])), "centre to centre");
    assert_eq!(carried_site(&raster(3, 3), &raster(9, 9), (3, 1), site([0, 0])), Some(site([1, 1])), "a site to its block's centre");
    assert_eq!(dilation(raster(1, 1), raster(1, 2)), None, "√(5/2) is no rational");
    assert_eq!(carried_site(&raster(2, 2), &raster(4, 4), (2, 1), site([0, 0])), None, "between sites");
}

/// 78.56 — THE MOMENTUM: a change of symbol in place carries none; a drift
/// one way does. The fixtures stand on 0.
#[test]
fn momentum_vanishes_only_without_drift() {
    stand_on(0);
    let still = pairs_of(vec![(vec![vec![1, 1, 0]], vec![vec![2, 2, 0]])]);
    let drift = pairs_of(vec![(vec![vec![1, 1, 0]], vec![vec![0, 1, 1]])]);
    assert!(momentum_vanishes(&still));
    assert!(!momentum_vanishes(&drift));
}

/// A raster's painted sites omit the ground the scope reads (here the
/// fixture's declared 0), each kept as its two coordinates and its symbol.
#[test]
fn painted_of_omits_ground() {
    stand_on(0);
    let painted = painted_of(&SectionSample::from(vec![vec![0, 1], vec![2, 0]]));
    assert_eq!(painted, vec![(0, 1, 1), (1, 0, 2)], "only novelty (off the ground)");
    assert!(painted_of(&SectionSample::from(vec![vec![0, 0], vec![0, 0]])).is_empty(), "all ground → no painted site");
}

/// TRACK 26, S4, S4d — THE SAMPLE BRIDGE: a raster reaches the sensor as a
/// sample of rank 2 and width 1 in canonical address order, held on the
/// alphabet's section as the same sample, and spoken back as the same
/// raster; a ragged raster and an empty one are refused at intake, never
/// padded.
#[test]
fn sample_bridge_reads_raster_as_rank_two_sample() {
    use crate::transducer::sample::{AxisReading, Place, SampleRefusal};
    let g: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let s = sample_of(&g, Place::new(vec![0])).expect("the raster stands");
    assert_eq!((s.rank(), s.width(), s.extent(0), s.extent(1)), (2, 1, AxisReading::Read(2), AxisReading::Read(3)));
    let held = SectionSample::of(s.clone()).expect("rank 2 at width 1 is on the section");
    assert_eq!((held.sample(), held.extents(), held.raster()), (&s, [2, 3], g));
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
    use crate::transducer::family::{task_of_lines, Task};
    let task = task_of_lines(
        vec![(vec![vec![0, 1], vec![2, 0]], vec![vec![1, 0], vec![0, 2]]), (vec![vec![7]], vec![vec![7, 7]])],
        vec![(vec![vec![3, 4, 5]], Vec::new())],
    );
    let bytes = encode_posted_task(&task).expect("the task posts");
    let back = decode_posted_task(&bytes).expect("the posted task reads");
    assert_eq!(back, task, "the same task");
    let samples = |t: &Task| -> Vec<SectionSample> {
        t.train.iter().flat_map(|(x, y)| [x.clone(), y.clone()]).chain(t.test.iter().flat_map(|(x, y)| std::iter::once(x.clone()).chain(y.clone()))).collect()
    };
    assert_eq!(samples(&back), samples(&task), "every raster the same sample");
    assert!(back.test[0].1.is_none(), "a withheld output stays withheld");
    assert_eq!(decode_posted_task(&bytes[..bytes.len() - 1]), None, "a cut file is no task");
    let mut longer = bytes.clone();
    longer.push(0);
    assert_eq!(decode_posted_task(&longer), None, "nothing after the last pair");
}

/// TRACK 26 S4d, stage 2 — A SITE OF THE SECTION IS THE SAMPLE'S
/// COORDINATE VECTOR RESTRICTED BY ITS PERSPECTIVE: the section is read
/// through the sample's default perspective, whose plane spans every axis;
/// each site the walk yields is the sample's own site restricted to that
/// plane, in canonical address order, with the sample's symbol there; a
/// site off the sample carries no symbol; a write lands in place, keeping
/// the extents and the place; and the site on the family's base carries the
/// plane's coordinates and the place on the perspective's place axis.
#[test]
fn section_site_is_the_sample_coordinate_restricted_by_the_perspective() {
    use crate::transducer::perspective::Perspective;
    use crate::transducer::sample::Place;
    let s = sample_of(&[vec![1, 2, 3], vec![4, 5, 6]], Place::new(vec![3])).expect("a raster at intake");
    let mut held = SectionSample::of(s.clone()).expect("on the section");
    assert_eq!(held.perspective(), &Perspective::of_sample(2), "the default perspective of the sample");
    let walked: Vec<(Vec<i64>, u8)> = held.sites().map(|(site, h)| (site.coordinates().to_vec(), h)).collect();
    let read: Vec<(Vec<i64>, u8)> = s.sites().zip(s.symbols()).map(|(site, h)| (site.axes().to_vec(), h.bytes()[0])).collect();
    assert_eq!(walked, read, "each site the sample's own, restricted to the plane, with its symbol");
    assert_eq!(held.at(SectionSite::on_plane([1, 2])), Some(6));
    assert_eq!(held.at(SectionSite::on_plane([2, 0])), None, "off the sample: no symbol, never the ground");
    assert_eq!(held.at(SectionSite::on_plane([-1, 0])), None);
    assert_eq!(held.paint(SectionSite::on_plane([0, 3]), 9), None, "nothing written off the sample");
    held.paint(SectionSite::on_plane([0, 1]), 9).expect("a site of the sample");
    assert_eq!((held.raster(), held.sample().place()), (vec![vec![1, 9, 3], vec![4, 5, 6]], &Place::new(vec![3])), "written in place");
    let on_base = SectionSite::on_plane([1, 2]).on_family(4).expect("the alphabet's base");
    assert_eq!(on_base.axes(), &[1, 2, 4], "the plane's coordinates, then the place");
    assert_eq!(SectionSite::of_family(&on_base), Some(SectionSite::on_plane([1, 2])), "and read back off the base");
}
