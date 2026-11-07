// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-SENSING §2; ROADMAP track 26 S1 — the rank-2 raster at intake: its
// painted sites, the frame's momentum and the re-period of each phase it
// reads, and the bridge to the sample. Moved from the former `arc` module's
// tests; the corpus's own claims are the decoder's (`sources/arc-agi/tests`).

use crate::transducer::element::Reperiod;
use crate::transducer::family::carried_site;
use crate::transducer::raster::*;
use crate::transducer::spelled::{pairs_of, PaintedSite, SectionSample, SectionSite};

/// 78.56; MU-SENSING §3.1 (track 26 S4) — THE RE-PERIOD OF EACH PHASE
/// carries a site: two rasters of one extent correspond site for site; 3×3
/// onto 9×9 re-periods each phase by 3, carrying each site's centre to the
/// centre of its 3×3 block, never to a corner; an anisotropic change
/// re-periods each phase by its own ratio (1×1 onto 1×3: the one site onto
/// the centre of three), where the envelopes' dilation read the irrational
/// `√(5/2)` and carried nothing; and a point that lands between sites
/// carries nothing. Two ranks have no re-period (`Reperiod::of_extents`).
#[test]
fn reperiod_carries_sites_between_extents_phase_by_phase() {
    let raster = |h: usize, w: usize| SectionSample::from(vec![vec![1; w]; h]);
    let reperiod = |a: &SectionSample, b: &SectionSample| Reperiod::of_extents(a.sample().extents(), b.sample().extents());
    let (same, other) = (raster(3, 4), raster(3, 4));
    let r = reperiod(&same, &other).expect("one rank");
    assert_eq!(carried_site(&same, &other, &r, SectionSite::on_plane([2, 3])), Some(SectionSite::on_plane([2, 3])), "one extent, site for site");
    let (small, large) = (raster(3, 3), raster(9, 9));
    let r = reperiod(&small, &large).expect("one rank");
    let site = SectionSite::on_plane;
    assert_eq!(carried_site(&small, &large, &r, site([1, 1])), Some(site([4, 4])), "centre to centre");
    assert_eq!(carried_site(&small, &large, &r, site([0, 0])), Some(site([1, 1])), "a site to its block's centre");
    let (dot, line) = (raster(1, 1), raster(1, 3));
    let r = reperiod(&dot, &line).expect("one rank");
    assert_eq!(carried_site(&dot, &line, &r, site([0, 0])), Some(site([0, 1])), "each phase by its own ratio");
    let (two, four) = (raster(2, 2), raster(4, 4));
    let r = reperiod(&two, &four).expect("one rank");
    assert_eq!(carried_site(&two, &four, &r, site([0, 0])), None, "between sites");
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

/// MU-SENSING §3.1 (track 26 S4) — THE MOMENTUM IS READ PHASE BY PHASE
/// across an anisotropic re-period: one site spread over the three of a
/// 1×3 carries none (each axis weighed by its own `p′ᵢ/pᵢ`, where the
/// envelopes' dilation read `√(5/2)` and left the momentum unread), and a
/// site moved to the far end of a 1×4 carries some. The fixtures stand on 0.
#[test]
fn momentum_read_per_phase_across_anisotropic_reperiod() {
    stand_on(0);
    let spread = pairs_of(vec![(vec![vec![1]], vec![vec![1, 1, 1]])]);
    let moved = pairs_of(vec![(vec![vec![1, 0]], vec![vec![0, 0, 1, 1]])]);
    assert!(momentum_vanishes(&spread));
    assert!(!momentum_vanishes(&moved));
}

/// A raster's painted sites omit the ground the scope reads (here the
/// fixture's declared 0), each kept as its site of the section and its
/// symbol.
#[test]
fn painted_sites_omits_ground() {
    stand_on(0);
    let painted = painted_sites(&SectionSample::from(vec![vec![0, 1], vec![2, 0]]));
    assert_eq!(
        painted,
        vec![PaintedSite::on_plane([0, 1], 1), PaintedSite::on_plane([1, 0], 2)],
        "only novelty (off the ground)"
    );
    assert!(painted_sites(&SectionSample::from(vec![vec![0, 0], vec![0, 0]])).is_empty(), "all ground → no painted site");
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

/// TRACK 26 S4d, stage 2b — A PAINTED SITE IS A SITE OF THE SECTION AND ITS
/// SYMBOL: a sample's painted sites, laid back on their span
/// (`SectionSite::span_of`, `sample_of_painted`) at the scene's ground, are the
/// sample's painted part read from the span's least corner; sites order by
/// the site, then the symbol; and no site has no span.
#[test]
fn painted_sites_round_trip_through_their_span() {
    stand_on(0);
    let g = SectionSample::from(vec![vec![0, 0, 0], vec![0, 7, 8], vec![0, 9, 0]]);
    let painted = painted_sites(&g);
    assert!(painted.windows(2).all(|w| w[0] < w[1]), "canonical address order is the sites' order");
    let span = SectionSite::span_of(painted.iter().map(|p| p.site));
    assert_eq!(span, Some((SectionSite::on_plane([1, 1]), [2, 2])), "the least corner and the extents from it");
    let laid = crate::transducer::family::sample_of_painted(&painted, 0).expect("a scene on the ground");
    assert_eq!(laid, vec![vec![7, 8], vec![9, 0]], "the painted part, read from the corner");
    assert_eq!(SectionSite::span_of(std::iter::empty()), None, "no site, no span");
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

/// TRACK 26 S4d, stage 2c — A SECTION SITE ROUND-TRIPS THROUGH ITS CANONICAL
/// BYTES: what `put_canonical` writes `take_canonical` reads back, the cursor
/// at the end, for coordinates of either sign; bytes that end inside a
/// coordinate read no site.
#[test]
fn section_site_round_trips() {
    for site in [SectionSite::on_plane([0, 0]), SectionSite::on_plane([-3, 7]), SectionSite::on_plane([i64::MAX, i64::MIN])] {
        let mut buf = Vec::new();
        site.put_canonical(&mut buf);
        let mut p = 0;
        assert_eq!((SectionSite::take_canonical(&buf, &mut p), p), (Some(site), buf.len()));
        let short = buf.get(..buf.len() - 1).expect("bytes");
        assert_eq!(SectionSite::take_canonical(short, &mut 0), None, "a coordinate cut short is no site");
    }
}
