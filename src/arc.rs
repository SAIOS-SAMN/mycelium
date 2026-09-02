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

// ── #97 Phase A: THE CODOMAIN APERTURE — cross-complex chains ────────────────
//
// Both frames embed EXACTLY into their common refinement K_lcm by integer
// block subdivision (pure arithmetic — no interpolation, no direction bias,
// no rounding policy; uniform for every ratio including (1,1), which
// reproduces the planar path bit-for-bit). The cross-frame demand is then an
// ORDINARY chain on the refinement. Banned surface: no branch anywhere on
// ratio VALUES — the mapper is total.

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a.max(1) } else { gcd(b, a % b) }
}
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

/// EMBED a grid into a refinement lattice: each source cell becomes an exact
/// (lh/h) × (lw/w) block. Total whenever lh, lw are multiples of the dims.
pub fn embed(g: &Grid, lh: usize, lw: usize) -> Grid {
    let (h, w) = (g.len().max(1), g.first().map_or(1, |r| r.len().max(1)));
    (0..lh).map(|r| (0..lw).map(|c| g[r * h / lh][c * w / lw]).collect()).collect()
}

/// READ BACK from a refinement lattice to target dims — every target cell's
/// block must be CONSTANT (an ill-formed prediction is a REFUSAL, never a
/// rounding: None).
pub fn readback(refined: &Grid, h: usize, w: usize) -> Option<Grid> {
    let (lh, lw) = (refined.len().max(1), refined.first().map_or(1, |r| r.len().max(1)));
    let mut out = vec![vec![0u8; w]; h];
    for (r, row) in refined.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            let (tr, tc) = (r * h / lh, c * w / lw);
            if (r, c) == (tr * lh / h, tc * lw / w) {
                out[tr][tc] = v;
            } else if refined[tr * lh / h][tc * lw / w] != v {
                return None; // the block disagrees — no policy resolves it
            }
        }
    }
    Some(out)
}

/// The MEASURED frame of a pair — (h_in, w_in, h_out, w_out) exact counts —
/// and its reduced ratio ((h_num, h_den), (w_num, w_den)).
pub type Frame = (usize, usize, usize, usize);
pub fn frame_ratio(f: &Frame) -> ((usize, usize), (usize, usize)) {
    let (hi, wi, ho, wo) = *f;
    let (gh, gw) = (gcd(ho, hi), gcd(wo, wi));
    ((ho / gh.max(1), hi / gh.max(1)), (wo / gw.max(1), wi / gw.max(1)))
}

/// THE CROSS-COMPLEX COBOUNDARY: embed both scenes into K_lcm and take the
/// ordinary per-cell difference. Returns (chain on K_lcm, the refined input
/// grid — the scene π evaluates over, the measured frame). Total for all
/// dims ≥ 1; equal frames yield exactly the planar delta.
pub fn refine_delta(a: &Grid, b: &Grid) -> Option<(Vec<(i64, i64, u8, u8)>, Grid, Frame)> {
    let (hi, wi) = (a.len(), a.first().map(|r| r.len()).unwrap_or(0));
    let (ho, wo) = (b.len(), b.first().map(|r| r.len()).unwrap_or(0));
    if hi == 0 || wi == 0 || ho == 0 || wo == 0 {
        return None;
    }
    if a.iter().any(|r| r.len() != wi) || b.iter().any(|r| r.len() != wo) {
        return None; // ragged grids carry no complex
    }
    let (lh, lw) = (lcm(hi, ho), lcm(wi, wo));
    let (ra, rb) = (embed(a, lh, lw), embed(b, lh, lw));
    let mut chain = Vec::new();
    for r in 0..lh {
        for c in 0..lw {
            if ra[r][c] != rb[r][c] {
                chain.push((r as i64, c as i64, ra[r][c], rb[r][c]));
            }
        }
    }
    Some((chain, ra, (hi, wi, ho, wo)))
}

/// A chain's witnessed placements inside a coboundary — every translation
/// whose cells all sit in the δ with exact transitions; the union footprint
/// is the chain's measured domain in that pair (engine organ; the entity's
/// study composes it, the court's evidence speaks it).
pub fn chain_footprint(
    chain: &[(i64, i64, u8, u8)],
    d: &[(i64, i64, u8, u8)],
) -> Vec<(i64, i64)> {
    let mut fp: Vec<(i64, i64)> = Vec::new();
    let Some(&(r0, c0, _, _)) = chain.first() else { return fp };
    for &(ar, ac, ab, aa) in d {
        if (ab, aa) != (chain[0].2, chain[0].3) {
            continue;
        }
        let (dr, dc) = (ar - r0, ac - c0);
        if chain.iter().all(|(r, c, b, a)| d.binary_search(&(r + dr, c + dc, *b, *a)).is_ok()) {
            fp.extend(chain.iter().map(|(r, c, _, _)| (r + dr, c + dc)));
        }
    }
    fp.sort_unstable();
    fp.dedup();
    fp
}

/// APPLY bound chains (a dialect: concrete rewrite chains each carrying an
/// optional selection term) to a grid. π-bound chains place ONLY where the
/// term's region proposes (the region's cells anchor the lead cell, the whole
/// footprint inside it — deduction, no sweep); unbound chains fall back to
/// the confirming sweep, and the caller's replay gate polices them.
pub fn apply_bound_chains(
    grid: &Grid,
    spoken: &[(Vec<(i64, i64, u8, u8)>, Option<crate::transducer::anchor::Sel>)],
) -> Grid {
    let mut out = grid.clone();
    let (rows, cols) = (grid.len() as i64, grid.first().map_or(0, |r| r.len()) as i64);
    let scene: Vec<(i64, i64, u8)> = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, &h)| (r as i64, c as i64, h)))
        .collect();
    let mut writes: Vec<(i64, i64, u8)> = Vec::new();
    for (chain, sel) in spoken {
        let Some(&(r0, c0, _, _)) = chain.first() else { continue };
        let candidates: Vec<(i64, i64)> = match sel {
            Some(t) => {
                let region = t.eval(&scene);
                region
                    .iter()
                    .map(|(rr, rc)| (rr - r0, rc - c0))
                    .filter(|(dr, dc)| {
                        chain
                            .iter()
                            .all(|(r, c, _, _)| region.binary_search(&(r + dr, c + dc)).is_ok())
                    })
                    .collect()
            }
            None => {
                let max_r = chain.iter().map(|(r, _, _, _)| *r).max().unwrap_or(0);
                let max_c = chain.iter().map(|(_, c, _, _)| *c).max().unwrap_or(0);
                (0..(rows - max_r).max(0))
                    .flat_map(|dr| (0..(cols - max_c).max(0)).map(move |dc| (dr, dc)))
                    .collect()
            }
        };
        for (dr, dc) in candidates {
            let fits = chain.iter().all(|(r, c, before, _)| {
                let (gr, gc) = (r + dr, c + dc);
                gr >= 0
                    && gc >= 0
                    && gr < rows
                    && gc < cols
                    && grid[gr as usize][gc as usize] == *before
            });
            if fits {
                for (r, c, _, after) in chain {
                    writes.push((r + dr, c + dc, *after));
                }
            }
        }
    }
    for (r, c, v) in writes {
        out[r as usize][c as usize] = v;
    }
    out
}

/// THE PROJECTION — the inverse deduction as a library organ: apply anchored
/// words to a world state and return the projected next state plus the indices
/// of the words that spoke (the prediction's sentence). Discipline:
/// - Only π-BOUND words speak (γ = (π, Δ) with a derived selection term —
///   the court-refined language). ⊤ words are unrefined and stay SILENT in
///   projection: prediction quality grows exactly as the court binds anchors.
/// - Placements are PROPOSED BY π (the region's cells anchor each variant's
///   lead cell; the whole footprint must sit inside the region); the
///   before-side only CONFIRMS. Deduction from measured topology, no sweep.
pub fn project_anchored(grid: &Grid, words: &[crate::transducer::element::Element]) -> (Grid, Vec<usize>) {
    let (rows, cols) = (grid.len() as i64, grid.first().map_or(0, |r| r.len()) as i64);
    // The FULL complex: π terms see background (holes, halos live on hue 0).
    let scene: Vec<(i64, i64, u8)> = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, &h)| (r as i64, c as i64, h)))
        .collect();
    let mut out = grid.clone();
    let mut spoke: Vec<usize> = Vec::new();
    let mut writes: Vec<(i64, i64, u8)> = Vec::new();
    for (wi, w) in words.iter().enumerate() {
        let Some(pi) = w.anchor() else { continue }; // ⊤ words are silent
        let region = pi.eval(&scene);
        if region.is_empty() {
            continue;
        }
        let mut said = false;
        for chain in w.variant_chains() {
            let Some(&(r0, c0, _, _)) = chain.first() else { continue };
            for &(rr, rc) in &region {
                let (dr, dc) = (rr - r0, rc - c0);
                let inside = chain
                    .iter()
                    .all(|(r, c, _, _)| region.binary_search(&(r + dr, c + dc)).is_ok());
                let fits = inside
                    && chain.iter().all(|(r, c, before, _)| {
                        let (gr, gc) = (r + dr, c + dc);
                        gr >= 0
                            && gc >= 0
                            && gr < rows
                            && gc < cols
                            && grid[gr as usize][gc as usize] == *before
                    });
                if fits {
                    for (r, c, _, after) in &chain {
                        writes.push((r + dr, c + dc, *after));
                    }
                    said = true;
                }
            }
        }
        if said {
            spoke.push(wi);
        }
    }
    for (r, c, v) in writes {
        out[r as usize][c as usize] = v;
    }
    (out, spoke)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Entity::project — the intelligence pull: an anchored genome word
    /// projects the next world state exactly where its π holds; ⊤ words stay
    /// silent (a young genome predicts humbly); the spoken sentence names the
    /// words that thought.
    #[test]
    fn projection_speaks_only_where_pi_holds() {
        use crate::transducer::anchor::Sel;
        use crate::transducer::element::from_residual;
        // A 1→2 recolour word, court-bound to "hue-1 cells touching hue-3".
        let bound = from_residual(&vec![(0, 0, 1, 2)]).with_anchor(Sel::Inter(
            Box::new(Sel::Hue(1)),
            Box::new(Sel::Star(Box::new(Sel::Hue(3)))),
        ));
        // An identical-substance word left ⊤ — unrefined, must stay silent.
        let top = from_residual(&vec![(0, 0, 1, 2)]);
        let genome = vec![top, bound];
        // World: two hue-1 cells; only (1,1) touches the hue-3 body.
        let world: Grid = vec![vec![1, 0, 0], vec![0, 1, 3], vec![0, 0, 0]];
        let (next, spoke) = project_anchored(&world, &genome);
        assert_eq!(next[1][1], 2, "the interface cell was projected forward");
        assert_eq!(next[0][0], 1, "the off-interface cell is untouched — no ambient flood");
        assert_eq!(spoke, vec![1], "the sentence names ONLY the anchored word");
        // A world where π holds nowhere projects the identity — humble silence.
        let calm: Grid = vec![vec![1, 0], vec![0, 0]];
        let (same, said) = project_anchored(&calm, &genome);
        assert_eq!(same, calm);
        assert!(said.is_empty());
    }

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
}
