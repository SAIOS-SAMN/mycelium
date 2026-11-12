// D3.2 §D — ACT: the incidence record of two frames, and its projection.

use super::{changes, delta, ground, record, sample, sample_by, stroke};
use crate::transducer::family::transduce::*;
use crate::transducer::sample::Sample;

/// An enclosure of the given periods.
fn periods(p: Vec<usize>) -> Enclosure {
    Enclosure::new(p).expect("periods stand")
}

/// THE MERGE OF ONE PHASE HAS `a + b − gcd(a, b)` ARCS: `18` and `7` are
/// coprime (24), `19` and `9` too (27), `29` and `30` too (58); `2` and `4`
/// share two boundaries besides the seam (4). The need is the arcs' product over
/// the phases.
#[test]
fn arcs_equal_sum_less_gcd() {
    for (a, b, r) in [(18, 7, 24), (19, 9, 27), (29, 30, 58), (2, 4, 4), (5, 5, 5)] {
        assert_eq!(arcs(a, b), Some(r), "R({a}, {b})");
    }
    assert_eq!(need(&periods(vec![18, 19]), &periods(vec![7, 9])), Some(24 * 27));
    assert_eq!(need(&periods(vec![1]), &periods(vec![1, 1])), None, "no phase to merge with, nothing to need");
}

/// The reported pair, `0b148d64` pair 0 (`18 × 19 → 7 × 9`), as the corpus
/// holds it.
fn reported_pair() -> (Sample, Sample) {
    let x = sample(&[
        "2022220000202222002",
        "2222022000022222000",
        "0022020000222022222",
        "2020220000022222200",
        "0202222000000220222",
        "2220202000202222020",
        "0000000000000000000",
        "0000000000000000000",
        "0000000000000000000",
        "0000000000000000000",
        "0000000000000000000",
        "2020002000033333033",
        "0220022000333000330",
        "0220020000333030300",
        "2220022000330003333",
        "2002220000300030303",
        "2020002000033033303",
        "0220220000033003030",
    ]);
    let y = sample(&["033333033", "333000330", "333030300", "330003333", "300030303", "033033303", "033003030"]);
    (x, y)
}

/// THE REPORTED PAIR held by its bodies' boundaries: 111 bands in the input
/// and 32 in the output, and 226 incidences between them, 157 of them a
/// change of symbol, against 21 546 cells of `K_lcm`, and below the bound
/// `R_0 · R_1 = 24 · 27 = 648`. The counts were read off the two samples by
/// hand-checkable run merging (a band continues while its slice's run keeps
/// its two turns and its symbol), independently of this organ.
#[test]
fn incidences_of_0b148d64_pair_count() {
    let (x, y) = reported_pair();
    let need = need_of(&x, &y).expect("both frames stand");
    assert_eq!(need, 648);
    let mut budget = stroke(need);
    let t = Transduction::of(&x, &y, &mut budget).expect("the need is the stroke");
    assert_eq!((t.from.bands.len(), t.to.bands.len()), (111, 32));
    assert_eq!(t.incidences.len(), 226);
    assert_eq!(t.changed().count(), 157);
    assert!(need - budget.remaining() <= need, "the spend stays inside the need");
    // A shrink whose boundaries fall inside output compartments projects
    // onto no sample: the turn `1/18` lands on no boundary of period 7.
    assert_eq!(t.projected(), Err(TransductionRefusal::OffSite { turn: Turn::of(1, 18), period: 7 }));
}

/// Equal-extent fixtures of every shape the bands distinguish: one body,
/// nested bodies, a body splitting, ground appearing and vanishing, a
/// symbol moving, a T and its stem.
fn fixtures() -> Vec<(Sample, Sample)> {
    vec![
        (sample(&["0"]), sample(&["3"])),
        (sample(&["110", "110", "000"]), sample(&["000", "011", "011"])),
        (sample(&["1111", "1221", "1221", "1111"]), sample(&["1111", "1331", "1301", "1111"])),
        (sample(&["111", "010"]), sample(&["222", "020"])),
        (sample(&["10", "01"]), sample(&["01", "10"])),
        (sample(&["12", "34"]), sample(&["12", "34"])),
    ]
}

/// THE BODY-BOUNDARY RECORD READ BACK THROUGH PROJECTION IS THE COBOUNDARY
/// on every equal-extent pair: on one enclosure every turn lands, and each
/// changed incidence projects onto exactly the sites its two bands share,
/// so the projected record is the site-by-site difference link for link.
/// The corpus's equal-extent pairs are read through the ARC bridge
/// (`tests::arc`).
#[test]
fn incidences_project_to_delta_on_equal_extents() {
    for (x, y) in fixtures() {
        let t = record(&x, &y);
        assert!(t.keeps_periods());
        assert_eq!(changes(&t.projected().expect("one enclosure projects")), delta(&x, &y), "{x:?} → {y:?}");
    }
}

/// O1 — THE SHARE IS THE CHANGED FRACTION OF THE FRAME, exactly: on one
/// enclosure the changed sites over the frame's measure; across `29 × 29 →
/// 30 × 30` one changed output site is `1/900` of the frame and one changed
/// incidence, where `K_lcm` wrote a `29 × 29` block of 841 cells.
#[test]
fn share_of_one_changed_site_is_lcm_free() {
    for (x, y) in fixtures() {
        let t = record(&x, &y);
        let sites = x.sites().count() as u64;
        let changed = delta(&x, &y).len() as u64;
        assert_eq!(t.share(), crate::core::Q::new(changed.into(), sites.into()));
    }
    let x = sample_by(&[29, 29], |_| 1);
    let y = sample_by(&[30, 30], |t| if t == [1, 2] { 5 } else { 1 });
    let t = record(&x, &y);
    assert_eq!(t.changed().count(), 1);
    assert_eq!(t.share(), crate::core::Q::new(1.into(), 900.into()));
    // And every input turn is a seam, so the record projects: the one
    // changed site, and no block.
    assert_eq!(changes(&t.projected().expect("seams land")), vec![(vec![1, 2], vec![1], vec![5])]);
}

/// A rank-2, width-1 difference as the section's chain the lane measure
/// reads.
fn chain(d: &[(Vec<i64>, Vec<u8>, Vec<u8>)]) -> Vec<crate::transducer::sentence::SectionLink> {
    d.iter().map(|(s, a, b)| crate::transducer::sentence::tests::fixtures::link(s[0], s[1], a[0], b[0])).collect()
}

/// O1 (ruled 2026-11-01) — THE SHARE BY LANE: on one enclosure S is the
/// sites changed between ground and painted over the frame's measure and G
/// the sites changed from one painted symbol to another over it: the
/// coboundary's spatial and gauge counts over the frame, which is the share
/// `share_of_enclosure` reads off a chain, and the two sum to the share. Across
/// `29 × 29 → 30 × 30` the one site repainted `1 → 5` is `1/900` on G.
#[test]
fn lane_shares_equal_chain_counts_over_frame() {
    crate::transducer::raster::stand_on(0);
    for (x, y) in fixtures() {
        let t = record(&x, &y);
        let counts = crate::transducer::element::measure_lanes(&chain(&delta(&x, &y)));
        let mu = share_of_enclosure(counts, Some(&y));
        assert_eq!(t.lane_shares(&ground()), [mu.lane(0).clone(), mu.lane(1).clone()], "{x:?} → {y:?}");
        assert_eq!(t.lane_shares(&ground())[0].clone() + t.lane_shares(&ground())[1].clone(), t.share());
        assert_eq!(mu.lane(3), crate::core::Q::from_integer(counts[3].into()), "H, the shell count, stands as it is");
    }
    let x = sample_by(&[29, 29], |_| 1);
    let y = sample_by(&[30, 30], |t| if t == [1, 2] { 5 } else { 1 });
    let zero = crate::core::Q::from_integer(0.into());
    assert_eq!(record(&x, &y).lane_shares(&ground()), [zero, crate::core::Q::new(1.into(), 900.into())]);
    // A discharge with no frame keeps its measure as n/1 (ruling 3).
    assert_eq!(share_of_enclosure([3, 2, 0, 1], None), [3, 2, 0, 1]);
}

/// THE RECORD PROJECTED IS THE IDENTITY'S SPEECH AGAINST THE OUTPUT: where
/// the input's turns land, the record read back on the output frame equals
/// the input's bands spoken onto the output enclosure (`spoken_onto`)
/// compared site for site with the output: two readers, one residue.
#[test]
fn projection_agrees_with_spoken_frame() {
    let pairs = [
        (sample(&["10", "02"]), sample(&["1100", "1100", "0022", "0002"])),
        (sample(&["12"]), sample(&["111222", "112222"])),
        (sample(&["1", "2", "3"]), sample(&["1", "1", "2", "2", "3", "0"])),
        (sample(&["11", "11"]), sample(&["111", "121", "111"])),
    ];
    for (x, y) in pairs {
        let t = record(&x, &y);
        let spoken = Enclosed::of(&x).spoken_onto(&t.to.enclosure).expect("the turns land");
        assert_eq!(t.projected().map(|c| changes(&c)), Ok(delta(&spoken, &y)), "{x:?} → {y:?}");
    }
}

/// A STROKE ONE SHORT OF THE NEED REFUSES BY NAME and returns no record.
/// `2 × 2 → 3 × 3` with every site its own symbol: each band is one site,
/// so every overlapping pair of sites is one visit, and those are the
/// `R(2, 3)² = 16` arcs of the merged phases: the need, spent whole.
#[test]
fn transduction_refuses_spent_stroke() {
    let x = sample(&["12", "34"]);
    let y = sample(&["123", "456", "789"]);
    assert_eq!(need_of(&x, &y), Some(16));
    assert!(matches!(Transduction::of(&x, &y, &mut stroke(15)), Err(TransductionRefusal::Spent { need: Some(16) })));
    let mut whole = stroke(16);
    assert!(Transduction::of(&x, &y, &mut whole).is_ok());
    assert_eq!(whole.remaining(), 0, "the need is spent whole");
}

/// A BODY IS CARRIED WHOLE OR NAMED `Mixed`: doubling `2 × 2 → 4 × 4`
/// carries each body onto the one body that is its image, boundary turn for
/// boundary turn; a body whose image is split across two output bodies is
/// `Mixed`, and so is one merged with another.
#[test]
fn transduction_carries_bodies_or_names_mixed() {
    let t = record(&sample(&["10", "02"]), &sample(&["1100", "1100", "0022", "0022"]));
    assert!(t.relations().iter().all(Result::is_ok), "{:?}", t.relations());
    let split = record(&sample(&["11"]), &sample(&["12"]));
    assert_eq!(split.carried(0), Err(TransductionRefusal::Mixed { body: 0 }));
    let merged = record(&sample(&["12"]), &sample(&["1111"]));
    assert_eq!(merged.carried(0), Err(TransductionRefusal::Mixed { body: 0 }));
    assert_eq!(merged.carried(1), Err(TransductionRefusal::Mixed { body: 1 }));
}

/// PROJECT REFUSES A TURN THAT LANDS ON NO BOUNDARY, naming it: `2 × 1 →
/// 3 × 1` puts the boundary `1/2` at `3/2` of the output's first phase; a
/// frame of one symbol has only seams and lands on any period.
#[test]
fn projection_refuses_off_site() {
    let t = record(&sample(&["1", "2"]), &sample(&["1", "1", "2"]));
    assert_eq!(t.projected(), Err(TransductionRefusal::OffSite { turn: Turn::of(1, 2), period: 3 }));
    let mono = record(&sample(&["11", "11"]), &sample(&["111", "121", "111"]));
    assert_eq!(changes(&mono.projected().expect("seams land")), vec![(vec![1, 1], vec![1], vec![2])]);
}

/// TWO DELIMITED OF DIFFERENT RANK ARE NOT RELATED: a phase of one has no phase
/// of the other, and nothing is padded or dropped.
#[test]
fn transduction_refuses_rank_mismatch() {
    let x = super::sample_n(&[2], "12");
    let y = sample(&["12"]);
    assert!(matches!(Transduction::of(&x, &y, &mut stroke(4)), Err(TransductionRefusal::Rank { from: 1, to: 2 })));
    assert_eq!(need_of(&x, &y), None);
}
