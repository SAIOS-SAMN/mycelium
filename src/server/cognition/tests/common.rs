// Fixture constructors for the mind's tests (MU-STANDARDS 3.3): a scene, a
// glued pair, and a grid of digits, each built through the constructors the
// production code exports.

use crate::cognition::*;

pub fn scene(sr: i64, sc: i64) -> Vec<(i64, i64, u8)> {
vec![(sr, sc, 1), (sr, sc + 1, 1), (sr + 1, sc, 1)]
}

pub fn glue_pair(k: &mut Kernel, a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) {
k.ingest(a, b, [1; 32]);
let _ = k.ingest(a, b, [2; 32]);
k.glue_local();
}

pub fn digits(rows: &[&str]) -> Vec<Vec<u8>> {
rows.iter()
    .map(|r| r.bytes().map(|b| b - b'0').collect())
    .collect()
}
