// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ARC-AGI INGESTION ARM — read a task's grids and yield input→output train pairs
//! as A→B cell demands for the mesh. Ported from the old mycelium ingest (uplink /
//! sdk::ocular). The grid→cells convention is the PERCEPTUAL eye's: background hue
//! 0 is not a cell (the eye sees novelty — old `ocular::observe`'s `if v != 0`),
//! which is also forced by the coboundary germination (hue 0 = absent). This arm
//! does NO cognition — it only presents the scene; the mesh perceives and judges.

use serde::Deserialize;

/// A dense ARC grid of hues (0 = background).
pub type Grid = Vec<Vec<u8>>;

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

/// Parse a task's JSON (`{"train":[{"input","output"}],"test":[...]}`).
pub fn parse_task(json: &str) -> Option<Task> {
    let raw: RawTask = serde_json::from_str(json).ok()?;
    Some(Task {
        train: raw.train.into_iter().map(|p| (p.input, p.output)).collect(),
        test: raw.test.into_iter().map(|p| (p.input, p.output)).collect(),
    })
}

/// A grid → cells, the PERCEPTUAL convention: background hue 0 is omitted (the eye
/// sees only novelty), consistent with the coboundary germination's hue 0 = absent.
/// The walk is the grid's own cells (a scene boundary), not a product of carriers.
pub fn grid_to_cells(g: &Grid) -> Vec<(i64, i64, u8)> {
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
    let ca = grid_to_cells(a);
    let cb = grid_to_cells(b);
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
        let chain = crate::transducer::germinate::germinate(&grid_to_cells(a), &grid_to_cells(b))?;
        cover[3] = cover[3].checked_add(chain.len() as u64)?;
    }
    any.then_some(cover)
}

// ── #97: THE CODOMAIN APERTURE — cross-complex chains ────────────────────────
//
// A pair of complexes K_in (h_in × w_in) and K_out (h_out × w_out) is related
// by the exact frame map ρ = (h_out/h_in, w_out/w_in) ∈ ℚ². The cross-frame
// coboundary is supported on the codomain complex: every K_out cell reads the
// input through ρ⁻¹ (the input cell containing the output cell's min corner —
// the substrate's framing convention, the same min-corner quotient every chain
// is centered by) and differs, or not, from the output. No common refinement
// is materialized: the support is bounded by |K_out|, and ρ rides as the
// word's frame fiber. Uniform for every ratio including (1,1), which is the
// planar delta bit-for-bit. Banned surface: no branch anywhere on ratio
// VALUES — the map is total.

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a.max(1) } else { gcd(b, a % b) }
}

/// THE PULLBACK along ρ⁻¹: the grid read on a complex of `h × w` cells,
/// each output cell taking the input cell that contains its min corner.
/// Exact integer arithmetic; the identity when the frames agree.
pub fn embed(g: &Grid, h: usize, w: usize) -> Grid {
    let (gh, gw) = (g.len().max(1), g.first().map_or(1, |r| r.len().max(1)));
    (0..h).map(|r| (0..w).map(|c| g[r * gh / h][c * gw / w]).collect()).collect()
}


/// The MEASURED frame of a pair — (h_in, w_in, h_out, w_out) exact counts —
/// and its reduced ratio ((h_num, h_den), (w_num, w_den)).
pub type Frame = (usize, usize, usize, usize);
pub fn frame_ratio(f: &Frame) -> ((usize, usize), (usize, usize)) {
    let (hi, wi, ho, wo) = *f;
    let (gh, gw) = (gcd(ho, hi), gcd(wo, wi));
    ((ho / gh.max(1), hi / gh.max(1)), (wo / gw.max(1), wi / gw.max(1)))
}


/// WHAT REFINING A PAIR YIELDS: the delta between the two grids as changed
/// cells, the refined output grid, and the frame it was read in.
pub type Refined = (Vec<(i64, i64, u8, u8)>, Grid, Frame);

/// THE CROSS-COMPLEX COBOUNDARY on the codomain: the input pulled back along
/// ρ⁻¹ onto K_out, then the ordinary per-cell difference. Returns (chain on
/// K_out, the pulled-back input — the scene π evaluates over, the measured
/// frame). Total for all dims ≥ 1; equal frames yield exactly the planar
/// delta. Support ≤ |K_out|.
pub fn refine_delta(a: &Grid, b: &Grid) -> Option<Refined> {
    let (hi, wi) = (a.len(), a.first().map(|r| r.len()).unwrap_or(0));
    let (ho, wo) = (b.len(), b.first().map(|r| r.len()).unwrap_or(0));
    if hi == 0 || wi == 0 || ho == 0 || wo == 0 {
        return None;
    }
    if a.iter().any(|r| r.len() != wi) || b.iter().any(|r| r.len() != wo) {
        return None; // ragged grids carry no complex
    }
    let scene = embed(a, ho, wo);
    let mut chain = Vec::new();
    for r in 0..ho {
        for c in 0..wo {
            if scene[r][c] != b[r][c] {
                chain.push((r as i64, c as i64, scene[r][c], b[r][c]));
            }
        }
    }
    Some((chain, scene, (hi, wi, ho, wo)))
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
    fn grid_to_cells_omits_background() {
        // hue 0 cells are dropped; non-zero cells keep (row, col, hue).
        let cells = grid_to_cells(&vec![vec![0, 1], vec![2, 0]]);
        assert_eq!(cells, vec![(0, 1, 1), (1, 0, 2)], "only novelty (non-background)");
        assert!(grid_to_cells(&vec![vec![0, 0], vec![0, 0]]).is_empty(), "all-background → no cells");
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
            crate::transducer::germinate::germinate(&grid_to_cells(&p.0), &grid_to_cells(&p.1))
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
}
