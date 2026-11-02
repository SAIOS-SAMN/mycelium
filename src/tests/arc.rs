use crate::arc::*;

/// 78.56 — THE ENVELOPES' DILATION: two grids of one extent correspond
/// site for site; 3×3 onto 9×9 is s = 3, carrying each cell's centre to
/// the centre of its 3×3 block, never to a corner; an extent whose
/// envelope ratio is no rational square corresponds nowhere (78.45); and
/// a point that lands between sites carries nothing.
#[test]
fn envelope_dilation_carries_sites_between_extents() {
    let grid = |h: usize, w: usize| -> Grid { vec![vec![1; w]; h] };
    assert_eq!(envelope_dilation(&grid(3, 4), &grid(3, 4)), Some((1, 1)));
    assert_eq!(envelope_dilation(&grid(3, 3), &grid(9, 9)), Some((3, 1)));
    assert_eq!(carried_site(&grid(3, 3), &grid(9, 9), (3, 1), [1, 1]), Some([4, 4]), "centre to centre");
    assert_eq!(carried_site(&grid(3, 3), &grid(9, 9), (3, 1), [0, 0]), Some([1, 1]), "a cell to its block's centre");
    assert_eq!(envelope_dilation(&grid(1, 1), &grid(1, 2)), None, "√(5/2) is no rational");
    assert_eq!(carried_site(&grid(2, 2), &grid(4, 4), (2, 1), [0, 0]), None, "between sites");
    // The momentum: a recolour in place carries none; a drift one way does.
    let still = vec![(vec![vec![1, 1, 0]], vec![vec![2, 2, 0]])];
    let drift = vec![(vec![vec![1, 1, 0]], vec![vec![0, 1, 1]])];
    assert!(momentum_vanishes(&still));
    assert!(!momentum_vanishes(&drift));
}


/// A task's JSON parses into its train pairs and test inputs, and a
/// withheld test output is empty.
#[test]
fn parse_task_reads_train_and_withheld_pairs() {
    let json = r#"{"train":[{"input":[[0,1],[2,0]],"output":[[1,0],[0,2]]}],"test":[{"input":[[3]]}]}"#;
    let t = parse_task(json).expect("parses");
    assert_eq!(t.train.len(), 1);
    assert_eq!(t.test.len(), 1);
    assert_eq!(t.train[0].0, vec![vec![0, 1], vec![2, 0]]);
    assert!(t.test[0].1.is_empty(), "a withheld test output is empty");
}

/// A grid's painted sites omit the background, each kept as row,
/// column and hue.
#[test]
fn painted_of_grid_omits_background() {
    // hue-0 sites are dropped; painted sites keep (row, col, hue).
    let painted = painted_of_grid(&vec![vec![0, 1], vec![2, 0]]);
    assert_eq!(painted, vec![(0, 1, 1), (1, 0, 2)], "only novelty (non-background)");
    assert!(painted_of_grid(&vec![vec![0, 0], vec![0, 0]]).is_empty(), "all-background → no painted site");
}

/// 12.6: a bundle's pressure is the lane-wise SUM over its pairs, which
/// are disjoint scenes — never a maximum. A maximum would report this
/// two-pair family as no larger than its wider pair, so a bond measured
/// from it could not cover the family it was measured from.
#[test]
fn family_cover_sums_pairs_lane_wise() {
    let small = (vec![vec![1, 1]], vec![vec![2, 2]]);
    let wide = (vec![vec![1, 1, 1]], vec![vec![2, 2, 2]]);
    let cover = family_cover(&[small.clone(), wide.clone()]).expect("measured");
    let a = pair_magnitude(&small.0, &small.1).expect("small");
    let b = pair_magnitude(&wide.0, &wide.1).expect("wide");
    for l in 0..3 {
        assert_eq!(cover[l], a[l] + b[l], "lane {l} sums the disjoint scenes");
        assert!(cover[l] >= a[l].max(b[l]), "and therefore covers each pair");
    }
    let h = |p: &(Grid, Grid)| {
        crate::transducer::germinate::germinate(&painted_of_grid(&p.0), &painted_of_grid(&p.1))
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

/// The Euclidean gcd is exact and the frame ratio is total, an empty
/// frame reading `((0, 0), (0, 0))`.
#[test]
fn frame_ratio_totals_over_exact_gcd() {
    assert_eq!(gcd(0, 0), 0);
    assert_eq!(gcd(12, 8), 4);
    assert_eq!(gcd(7, 0), 7);
    assert_eq!(gcd(0, 5), 5);
    assert_eq!(frame_ratio(&Frame::new(0, 0, 0, 0)), ((0, 0), (0, 0)));
    assert_eq!(frame_ratio(&Frame::new(4, 6, 2, 3)), ((1, 2), (1, 2)));
}

/// TRACK 26, S4 — THE SAMPLE BRIDGE: an ARC grid reaches the transduction
/// as a sample of rank 2 and width 1 in row-major order, and a spoken
/// sample returns to the same grid; a ragged grid and an empty one are
/// refused at intake, never padded.
#[test]
fn sample_bridge_reads_grid_as_rank_two_sample() {
    use crate::transducer::sample::{AxisReading, Place, SampleRefusal};
    let g: Grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let s = sample_of(&g, Place::new(vec![0])).expect("the grid stands");
    assert_eq!((s.rank(), s.width(), s.extent(0), s.extent(1)), (2, 1, AxisReading::Read(2), AxisReading::Read(3)));
    assert_eq!(grid_of(&s), Some(g));
    assert!(matches!(sample_unplaced(&vec![vec![1, 2], vec![3]]), Err(SampleRefusal::Length { .. })));
    assert_eq!(sample_unplaced(&Vec::new()), Err(SampleRefusal::EmptyAxis { axis: 0 }));
}

/// THE BODY-BOUNDARY RECORD READ BACK THROUGH PROJECTION IS TODAY'S δ on
/// every equal-extent pair of the corpus, read through the bridge: on one
/// enclosure every turn lands, and the projected record, written back as
/// planar links, is the planar coboundary link for link.
#[test]
fn corpus_pairs_through_bridge_project_to_delta() {
    use crate::transducer::family::transduce::{need_of, Transduction};
    let dir = corpus("training");
    let entries = std::fs::read_dir(&dir).expect("the corpus stands");
    let mut read = 0usize;
    for e in entries {
        let path = e.expect("an entry reads").path();
        let Some(task) = std::fs::read_to_string(&path).ok().and_then(|s| parse_task(&s)) else { continue };
        for (x, y) in task.train.iter().chain(task.test.iter().filter(|(_, y)| !y.is_empty())) {
            if extents(x) != extents(y) {
                continue;
            }
            let (a, b) = (sample_unplaced(x).expect("stands"), sample_unplaced(y).expect("stands"));
            let need = need_of(&a, &b).expect("one rank");
            let t = Transduction::of(&a, &b, &mut crate::physics::Physics::default().stroke_n(need)).expect("the need is the stroke");
            let projected = links_of(&t.projected().expect("one enclosure projects"));
            assert_eq!(projected, crate::transducer::family::speak::residual_of(Some(x), y), "{}", path.display());
            read += 1;
        }
    }
    assert!(read > 0, "the corpus held equal-extent pairs");
}
