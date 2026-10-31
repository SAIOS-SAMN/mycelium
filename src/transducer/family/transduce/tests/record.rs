// D3.2 §D — ACT: the incidence record of two frames, and its projection.

use super::{grid, stroke};
use crate::transducer::family::perceive::Grid;
use crate::transducer::family::transduce::*;

/// The record of a pair under a stroke its need sizes.
fn record(x: &Grid, y: &Grid) -> Transduction {
    let need = need_of(x, y).expect("both frames stand");
    Transduction::of(x, y, &mut stroke(need)).expect("the need is the stroke")
}

/// THE MERGE OF ONE CIRCLE HAS `a + b − gcd(a, b)` ARCS: `18` and `7` are
/// coprime (24), `19` and `9` too (27), `29` and `30` too (58); `2` and `4`
/// share two boundaries besides the seam (4).
#[test]
fn arcs_equal_sum_less_gcd() {
    for (a, b, r) in [(18, 7, 24), (19, 9, 27), (29, 30, 58), (2, 4, 4), (5, 5, 5)] {
        assert_eq!(arcs(a, b), Some(r), "R({a}, {b})");
    }
    assert_eq!(need(18, 19, 7, 9), Some(24 * 27));
    assert_eq!(need(0, 1, 1, 1), None, "no enclosure, nothing to need");
}

/// The reported pair, `0b148d64` pair 0 (`18 × 19 → 7 × 9`), as the corpus
/// holds it.
fn reported_pair() -> (Grid, Grid) {
    let x = grid(&[
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
    let y = grid(&["033333033", "333000330", "333030300", "330003333", "300030303", "033033303", "033003030"]);
    (x, y)
}

/// THE REPORTED PAIR held by its bodies' boundaries: 111 bands in the input
/// and 32 in the output, and 226 incidences between them, 157 of them a
/// change of hue — against 21 546 cells of `K_lcm`, and below the bound
/// `R_h · R_w = 24 · 27 = 648`. The counts were read off the two grids by
/// hand-checkable run merging (a band continues while its row's run keeps
/// its two column turns and its hue), independently of this organ.
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
    // onto no grid: the turn `1/18` lands on no boundary of period 7.
    assert_eq!(t.projected(), Err(TransductionRefusal::OffSite { turn: Turn::new(1, 18), period: 7 }));
}

/// The planar coboundary of two frames of one enclosure, site for site:
/// the speech-against-observation reader every planar chain is.
fn planar_delta(x: &Grid, y: &Grid) -> Vec<(i64, i64, u8, u8)> {
    crate::transducer::family::speak::residual_of(Some(x), y)
}

/// Equal-extent fixtures of every shape the bands distinguish: one body,
/// nested bodies, a body splitting, ground appearing and vanishing, a hue
/// moving, a T and its stem.
fn fixtures() -> Vec<(Grid, Grid)> {
    vec![
        (grid(&["0"]), grid(&["3"])),
        (grid(&["110", "110", "000"]), grid(&["000", "011", "011"])),
        (grid(&["1111", "1221", "1221", "1111"]), grid(&["1111", "1331", "1301", "1111"])),
        (grid(&["111", "010"]), grid(&["222", "020"])),
        (grid(&["10", "01"]), grid(&["01", "10"])),
        (grid(&["12", "34"]), grid(&["12", "34"])),
    ]
}

/// THE BODY-BOUNDARY RECORD READ BACK THROUGH PROJECTION IS TODAY'S δ on
/// every equal-extent pair: on one enclosure every turn lands, and each
/// changed incidence projects onto exactly the sites its two bands share,
/// so the projected record is the planar coboundary link for link — on the
/// fixtures above, and on every equal-extent train pair of the corpus.
#[test]
fn incidences_project_to_delta_on_equal_extents() {
    for (x, y) in fixtures() {
        let t = record(&x, &y);
        assert!(t.is_planar());
        assert_eq!(t.projected().expect("one enclosure projects"), planar_delta(&x, &y), "{x:?} → {y:?}");
    }
    let dir = crate::arc::corpus("training");
    let entries = std::fs::read_dir(&dir).expect("the corpus stands");
    let mut read = 0usize;
    for e in entries {
        let path = e.expect("an entry reads").path();
        let Some(task) = std::fs::read_to_string(&path).ok().and_then(|s| crate::arc::parse_task(&s)) else { continue };
        for (x, y) in task.train.iter().chain(task.test.iter().filter(|(_, y)| !y.is_empty())) {
            if (x.len(), x.iter().next().map(Vec::len)) != (y.len(), y.iter().next().map(Vec::len)) {
                continue;
            }
            let t = record(x, y);
            assert_eq!(t.projected().expect("one enclosure projects"), planar_delta(x, y), "{}", path.display());
            read += 1;
        }
    }
    assert!(read > 0, "the corpus held equal-extent pairs");
}

/// O1 — THE SHARE IS THE CHANGED FRACTION OF THE FRAME, exactly: on one
/// enclosure the changed sites over `h·w`; across `29 × 29 → 30 × 30` one
/// changed output site is `1/900` of the frame and one changed incidence,
/// where `K_lcm` wrote a `29 × 29` block of 841 cells.
#[test]
fn share_of_one_changed_site_is_lcm_free() {
    for (x, y) in fixtures() {
        let t = record(&x, &y);
        let sites = (x.len() * x[0].len()) as u64;
        let changed = planar_delta(&x, &y).len() as u64;
        assert_eq!(t.share(), crate::core::Q::new(changed.into(), sites.into()));
    }
    let x: Grid = vec![vec![1; 29]; 29];
    let mut y: Grid = vec![vec![1; 30]; 30];
    y[1][2] = 5;
    let t = record(&x, &y);
    assert_eq!(t.changed().count(), 1);
    assert_eq!(t.share(), crate::core::Q::new(1.into(), 900.into()));
    // And every input turn is a seam, so the record projects: the one
    // changed site, and no block.
    assert_eq!(t.projected(), Ok(vec![(1, 2, 1, 5)]));
}

/// THE RECORD PROJECTED IS THE IDENTITY'S SPEECH AGAINST THE OUTPUT: where
/// the input's turns land, the record read back on the output frame equals
/// the input's bands spoken onto the output enclosure (`spoken_onto`)
/// compared site for site with the output — two readers, one residue.
#[test]
fn projection_agrees_with_spoken_frame() {
    let pairs = [
        (grid(&["10", "02"]), grid(&["1100", "1100", "0022", "0002"])),
        (grid(&["12"]), grid(&["111222", "112222"])),
        (grid(&["1", "2", "3"]), grid(&["1", "1", "2", "2", "3", "0"])),
        (grid(&["11", "11"]), grid(&["111", "121", "111"])),
    ];
    for (x, y) in pairs {
        let t = record(&x, &y);
        let spoken = Enclosed::of(&x).expect("stands").spoken_onto(t.to.enclosure).expect("the turns land");
        assert_eq!(t.projected(), Ok(planar_delta(&spoken, &y)), "{x:?} → {y:?}");
    }
}

/// A STROKE ONE SHORT OF THE NEED REFUSES BY NAME and returns no record.
/// `2 × 2 → 3 × 3` with every site its own hue: each band is one site, so
/// every overlapping pair of sites is one visit, and those are the `R(2, 3)²
/// = 16` arcs of the merged circles — the need, spent whole.
#[test]
fn transduction_refuses_spent_stroke() {
    let x = grid(&["12", "34"]);
    let y = grid(&["123", "456", "789"]);
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
    let t = record(&grid(&["10", "02"]), &grid(&["1100", "1100", "0022", "0022"]));
    assert!(t.relations().iter().all(Result::is_ok), "{:?}", t.relations());
    let split = record(&grid(&["11"]), &grid(&["12"]));
    assert_eq!(split.carried(0), Err(TransductionRefusal::Mixed { body: 0 }));
    let merged = record(&grid(&["12"]), &grid(&["1111"]));
    assert_eq!(merged.carried(0), Err(TransductionRefusal::Mixed { body: 0 }));
    assert_eq!(merged.carried(1), Err(TransductionRefusal::Mixed { body: 1 }));
}

/// PROJECT REFUSES A TURN THAT LANDS ON NO BOUNDARY, naming it: `[[1], [2]]
/// → 3 × 1` puts the boundary `1/2` at `3/2` of the output column; a
/// monochrome input has only seams and lands on any period.
#[test]
fn projection_refuses_off_site() {
    let t = record(&grid(&["1", "2"]), &grid(&["1", "1", "2"]));
    assert_eq!(t.projected(), Err(TransductionRefusal::OffSite { turn: Turn::new(1, 2), period: 3 }));
    let mono = record(&grid(&["11", "11"]), &grid(&["111", "121", "111"]));
    assert_eq!(mono.projected(), Ok(vec![(1, 1, 1, 2)]));
}
