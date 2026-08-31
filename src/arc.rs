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
}
