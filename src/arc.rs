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
