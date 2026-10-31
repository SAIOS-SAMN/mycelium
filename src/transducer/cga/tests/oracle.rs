// BQ3, 71.2(3) — THE ORACLE, VERIFIED: every reading of the algebra at n = 3
// taken again and compared, line for line, with the golden file.
//
// The readings and the golden file's writer live in `cga::oracle`; the
// writer is `mycelium-soak --write-oracle-golden`, never a test (STD1: no
// `#[ignore]` in the tree; §6.4: verification never writes).

use crate::transducer::cga::oracle::{inputs, readings, GOLDEN};

/// THE ALGEBRA REPRODUCES ITS ORACLE AT n = 3: every reading taken again
/// and compared, line for line, with the golden file.
#[test]
fn readings_reproduce_golden_oracle() {
    let Ok(golden) = std::fs::read_to_string(GOLDEN) else {
        panic!("{GOLDEN} does not read: the oracle has not been written (mycelium-soak --write-oracle-golden)")
    };
    let want: Vec<&str> = golden.lines().collect();
    let got = readings(&inputs());
    assert!(got.iter().any(|l| l.starts_with("canonical corpus")), "the harvest read no corpus versor");
    let first = want.iter().zip(&got).position(|(w, g)| *w != g.as_str());
    assert!(
        first.is_none() && want.len() == got.len(),
        "the algebra moved at line {:?} of {} (want {}, got {}): {:?} against {:?}",
        first,
        want.len(),
        want.len(),
        got.len(),
        first.map(|i| want[i]),
        first.map(|i| &got[i])
    );
    eprintln!("  oracle: {} readings reproduced", got.len());
}
