// Fixture constructors for the family's tests (MU-STANDARDS 3.3): a grid
// from rows of digits, and a row of coloured squares.

use crate::transducer::family::*;

pub fn grid(rows: &[&str]) -> Grid {
rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

pub fn squares(n: usize, hue: u8, cols: usize) -> Grid {
let mut g = vec![vec![0u8; cols]; 2];
for i in 0..n {
    let c = i * 3;
    if c + 1 < cols {
        g[0][c] = hue;
        g[0][c + 1] = hue;
        g[1][c] = hue;
        g[1][c + 1] = hue;
    }
}
g
}
