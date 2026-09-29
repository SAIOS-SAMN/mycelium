// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Law 11 — THE SENSOR'S GRIDS: a task's grids read as input→output train
//! pairs of A→B demands on painted sites, and the measured frame of a
//! pair. Hue 0 is not a painted site; this arm presents the scene and does
//! no cognition.

use serde::Deserialize;

/// A dense ARC grid of hues (0 = background).
pub type Grid = Vec<Vec<u8>>;

/// THE WIDTH OF A GRID. A grid is rectangular — every reader that takes
/// one refuses a ragged one — so row zero is a COORDINATE of the grid and
/// its length is the grid's width, not the length of whichever row arrived
/// first (rule 15). A grid with no row has no sites and measures zero.
///
/// One carrier for it: the same `first().map_or(0, …)` was written out at
/// seven sites, three of them with a different floor.
pub fn grid_width(g: &[Vec<u8>]) -> usize {
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 15 — canonical key order: row zero is the grid's own coordinate, and a rectangular grid's width is its length"
    )]
    g.first().map_or(0, Vec::len)
}

#[derive(Deserialize)]
struct RawPair {
    input: Grid,
    #[serde(default)]
    output: Grid,
}

#[derive(Deserialize)]
struct RawTask {
    #[serde(default)]
    train: Vec<RawPair>,
    #[serde(default)]
    test: Vec<RawPair>,
}

/// A parsed ARC task: the train family (input→output demonstrations) and the test
/// inputs (output may be withheld in evaluation tasks — empty then).
#[derive(Clone)]
pub struct Task {
    pub train: Vec<(Grid, Grid)>,
    pub test: Vec<(Grid, Grid)>,
}

/// Parse a task's JSON (`{"train":[{"input","output"}],"test":[...]}`),
/// its palette read at the sensor and nowhere after (`gauge_fixed`, 78.53).
pub fn parse_task(json: &str) -> Option<Task> {
    let raw: RawTask = serde_json::from_str(json).ok()?;
    Some(gauge_fixed(Task {
        train: raw.train.into_iter().map(|p| (p.input, p.output)).collect(),
        test: raw.test.into_iter().map(|p| (p.input, p.output)).collect(),
    }))
}

/// 78.53 — THE PALETTE IS READ ONLY AT THE SENSOR, and the gauge is fixed
/// by the base. The integer a grid gives a hue is a gauge choice on the
/// colour fibre (61.1): relabel every hue of a task by any permutation and
/// it is the same task. So the task is handed on in labels the GEOMETRY
/// fixes — each hue labelled by where it first stands, in the lattice's
/// own order across the task's scenes (train inputs and outputs, then the
/// test inputs) — and every word, key and verdict downstream reads those
/// labels, never the palette's integers.
///
/// Exact and with nothing to break a tie: a relabelling moves no cell, and
/// two hues never stand on one cell, so two hues never share a first
/// standing. The labels are read from what a voice is shown — the train
/// pairs and the test inputs — and a hue standing only in a withheld test
/// output is labelled after them, by where it first stands there, so no
/// withheld answer moves a label a voice sees. Hue 0 is no colour — the
/// sensor reads it as the void (`painted_of_grid`) — and stays 0.
pub fn gauge_fixed(task: Task) -> Task {
    let mut order: Vec<u8> = Vec::new();
    let stand = |g: &Grid, order: &mut Vec<u8>| {
        for row in g {
            for &h in row {
                if h != 0 && !order.contains(&h) {
                    order.push(h);
                }
            }
        }
    };
    for (a, b) in &task.train {
        stand(a, &mut order);
        stand(b, &mut order);
    }
    for (a, _) in &task.test {
        stand(a, &mut order);
    }
    for (_, b) in &task.test {
        stand(b, &mut order);
    }
    // The label of a hue is one past its place in the order: at most the
    // palette's own count of colours, so it fits the byte the hue came in.
    let label = |h: u8| -> u8 {
        match order.iter().position(|&o| o == h) {
            #[expect(clippy::cast_possible_truncation, reason = "STD1 C1 — the sensor's: a place in the order of a byte's distinct values is below 256")]
            Some(i) => (i + 1) as u8,
            None => 0,
        }
    };
    let relabel = |g: Grid| -> Grid { g.into_iter().map(|row| row.into_iter().map(label).collect()).collect() };
    Task {
        train: task.train.into_iter().map(|(a, b)| (relabel(a), relabel(b))).collect(),
        test: task.test.into_iter().map(|(a, b)| (relabel(a), relabel(b))).collect(),
    }
}

/// A grid → its painted sites, the PERCEPTUAL convention: background hue 0
/// is omitted (the eye sees only novelty), consistent with the coboundary
/// germination's hue 0 = absent. The walk is the grid's own sites (a scene
/// boundary), not a product of carriers.
pub fn painted_of_grid(g: &Grid) -> Vec<(i64, i64, u8)> {
    let mut out = Vec::new();
    for (r, row) in g.iter().enumerate() {
        for (c, &h) in row.iter().enumerate() {
            if h != 0 {
                out.push((r as i64, c as i64, h));
            }
        }
    }
    out
}

/// The μ⃗ of one train pair: the coboundary's lane measure. T is not on the chain.
/// None when the coboundary is unmeasured.
pub fn pair_magnitude(a: &Grid, b: &Grid) -> Option<[u64; 4]> {
    let ca = painted_of_grid(a);
    let cb = painted_of_grid(b);
    crate::transducer::germinate::germinate(&ca, &cb)
        .map(|g| crate::transducer::element::measure_lanes(&g))
}

/// Lane-wise accumulation of one pair's μ⃗ into a bundle's. The pairs of a
/// bundle are DISJOINT SCENES, so a bundle's measure on a lane is the sum of
/// its pairs' — chapter 12.6: "the lane-wise sum of the per-pair pressures,
/// each pair measured by `measure_lanes` on its own residual, the pairs
/// being disjoint scenes; never a maximum over pairs". A maximum reports a
/// bundle as no larger than its largest pair, which is how a genesis bond
/// measured from a corpus came out too small to cover a single pair of it.
/// Lanes never meet: each accumulates alone. `None` on overflow — a lane
/// that cannot hold its own sum is unmeasured, never capped (rule 11).
pub fn cover_join(cover: &mut [u64; 4], mag: &[u64; 4]) -> Option<()> {
    for l in 0..4 {
        cover[l] = cover[l].checked_add(mag[l])?;
    }
    Some(())
}

/// Genesis bond of one family: the lane-wise measure of its train pairs,
/// which is the work the family presents. The train is the family's
/// boundary. `None` when no pair is measured or a lane overflows.
pub fn family_cover(train: &[(Grid, Grid)]) -> Option<[u64; 4]> {
    let mut cover = [0u64; 4];
    let mut any = false;
    for (a, b) in train {
        let Some(mag) = pair_magnitude(a, b) else {
            continue;
        };
        if mag.iter().all(|&x| x == 0) {
            continue;
        }
        any = true;
        cover_join(&mut cover, &mag)?;
        // H carries the pair's own coboundary extent, accumulated with the
        // rest: the same disjoint-scene rule as every other lane.
        let chain = crate::transducer::germinate::germinate(&painted_of_grid(a), &painted_of_grid(b))?;
        cover[3] = cover[3].checked_add(chain.len() as u64)?;
    }
    any.then_some(cover)
}

// ── #97: THE CODOMAIN APERTURE — cross-complex chains ────────────────────────
//
// A pair of complexes K_in (h_in × w_in) and K_out (h_out × w_out) is related
// by the exact frame map ρ = (h_out/h_in, w_out/w_in) ∈ ℚ². The cross-frame
// coboundary is supported on the codomain complex: every site of K_out reads
// the input through ρ⁻¹ (the input site containing the output site's min
// corner — the substrate's framing convention, the same min-corner quotient
// every chain is centered by) and differs, or not, from the output. No common refinement
// is materialized: the support is bounded by |K_out|, and ρ rides as the
// word's frame fiber. Uniform for every ratio including (1,1), which is the
// planar delta bit-for-bit. Banned surface: no branch anywhere on ratio
// VALUES — the map is total.

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// THE MEASURED FRAME of a pair: four exact counts, never a ratio and never
/// a guess. The ratio is derived from them by `frame_ratio`, and a word at a
/// frame that is not (1,1) is a DIFFERENT word from its planar shadow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Frame {
    pub h_in: usize,
    pub w_in: usize,
    pub h_out: usize,
    pub w_out: usize,
}

impl Frame {
    /// Identity: no scale. Unframed speech uses this so its key matches a
    /// planar intake.
    pub const IDENTITY: Frame = Frame { h_in: 1, w_in: 1, h_out: 1, w_out: 1 };

    /// The frame of a pair whose extents are known.
    pub const fn new(h_in: usize, w_in: usize, h_out: usize, w_out: usize) -> Frame {
        Frame { h_in, w_in, h_out, w_out }
    }
}

pub fn frame_ratio(f: &Frame) -> ((usize, usize), (usize, usize)) {
    let Frame { h_in: hi, w_in: wi, h_out: ho, w_out: wo } = *f;
    let (gh, gw) = (gcd(ho, hi), gcd(wo, wi));
    let hr = match (ho.checked_div(gh), hi.checked_div(gh)) {
        (Some(o), Some(i)) => (o, i),
        _ => (0, 0),
    };
    let wr = match (wo.checked_div(gw), wi.checked_div(gw)) {
        (Some(o), Some(i)) => (o, i),
        _ => (0, 0),
    };
    (hr, wr)
}


/// WHAT REFINING A PAIR YIELDS.
pub struct Refined {
    /// The coboundary: the sites that changed, each with the hue it came
    /// from and the hue it went to.
    pub delta: Vec<(i64, i64, u8, u8)>,
    /// The input pulled back onto the output complex — the scene π is
    /// evaluated over, not the input as given.
    pub pulled_back: Grid,
    /// The frame it was read in.
    pub frame: Frame,
}

/// THE CROSS-COMPLEX COBOUNDARY on the codomain: the input pulled back along
/// ρ⁻¹ onto K_out, then the ordinary per-site difference. Returns (chain on
/// K_out, the pulled-back input — the scene π evaluates over, the measured
/// frame). Total for all dims ≥ 1; equal frames yield exactly the planar
/// delta. Support ≤ |K_out|.
pub fn refine_delta(a: &Grid, b: &Grid) -> Option<Refined> {
    let (hi, wi) = (a.len(), grid_width(a));
    let (ho, wo) = (b.len(), grid_width(b));
    if hi == 0 || wi == 0 || ho == 0 || wo == 0 {
        return None;
    }
    if a.iter().any(|r| r.len() != wi) || b.iter().any(|r| r.len() != wo) {
        return None; // ragged grids carry no complex
    }
    let refn = crate::transducer::family::refine::CommonRefinement::of(hi, wi, ho, wo)?;
    let (pulled_back, _) = refn.refined(a, b)?;
    let chain = refn.bipartite_cochain(a, b)?;
    Some(Refined { delta: chain, pulled_back, frame: Frame::new(hi, wi, ho, wo) })
}




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn a_task_parses_into_train_pairs() {
        let json = r#"{"train":[{"input":[[0,1],[2,0]],"output":[[1,0],[0,2]]}],"test":[{"input":[[3]]}]}"#;
        let t = parse_task(json).expect("parses");
        assert_eq!(t.train.len(), 1);
        assert_eq!(t.test.len(), 1);
        assert_eq!(t.train[0].0, vec![vec![0, 1], vec![2, 0]]);
        assert!(t.test[0].1.is_empty(), "a withheld test output is empty");
    }

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
    fn family_cover_is_the_lane_wise_sum_of_its_pairs() {
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

    #[test]
    fn exact_euclidean_gcd_and_frame_ratio_total() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 0), 7);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(frame_ratio(&Frame::new(0, 0, 0, 0)), ((0, 0), (0, 0)));
        assert_eq!(frame_ratio(&Frame::new(4, 6, 2, 3)), ((1, 2), (1, 2)));
    }
}

