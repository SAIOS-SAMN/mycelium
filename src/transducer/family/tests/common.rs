// Fixture constructors for the family's tests (MU-STANDARDS 3.3): a grid
// from rows of digits, and a row of coloured squares.

use crate::transducer::family::*;
use crate::transducer::sentence::Form;

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

pub fn budget() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 20)
}

/// THE CIPHER'S FORM OF ONE SCENE: derived against NO presentation, under
/// a stroke that reads the whole scene — the reading a mind that holds
/// nothing gets (50.2 step 2). Named so a reviewer sees at the call site
/// that no speaker's language is in the form — every test whose claim is
/// about perception rather than about what a mind holds builds its fixture
/// here, and a test about the language passes its own crystal to
/// `scene_form`.
///
/// IT IS A TEST FIXTURE AND NOT MOVE 1. It was `one_pair_form`, and 52.5
/// deletes `pair_form` per pair — the per-pair extraction arm, which is
/// gone: this calls `scene_form`, which calls `extract_rule_with` on a
/// one-pair family, the same production path every other caller takes. The
/// old name read as the deleted arm and was what P5's own DoD grep
/// (`pair_form(`) was guarding against finding. What it supplies over
/// `scene_form` is the crystal that holds nothing and the stroke, which is
/// a fixture's job.
pub fn cipher_form(a: &Grid, b: &Grid) -> Form {
    let pair = [(a.clone(), b.clone())];
    let frames = perceive_family(&pair);
    let mut budget = crate::physics::Physics::default().stroke_n(extraction_spend(&pair, &frames) << 4);
    // A crystal that holds nothing has nothing to factor through, so the
    // factorisation is measured the moment it starts and cannot end on
    // the budget: this expect is the absence of a presentation, not an
    // unwrap of one that might be there.
    scene_form_on(&NoCrystal, &mut budget, a, b, &frames).expect("a presentation of no generators measures at once")
}
