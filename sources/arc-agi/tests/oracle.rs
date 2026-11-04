// BQ3, 71.2(3) — THE ORACLE, VERIFIED: every reading of the algebra at n = 3
// taken again and compared, line for line, with the golden file.
//
// The readings and the golden file's writer live in `cga::oracle`; the
// writer is `mycelium-soak --write-oracle-golden DIR`, never a test (STD1: no
// `#[ignore]` in the tree; §6.4: verification never writes). The harvest's
// corpus slice is read here, through the corpus's decoder (ROADMAP track
// 26, S1; moved from the core's `transducer::cga::tests::oracle`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use mycelium::transducer::cga::oracle::{inputs, readings, GOLDEN, TASKS};

/// THE ALGEBRA REPRODUCES ITS ORACLE AT n = 3: every reading taken again
/// and compared, line for line, with the golden file.
#[test]
fn readings_agrees_with_golden_oracle() {
    let path = format!("{}/../../{GOLDEN}", env!("CARGO_MANIFEST_DIR"));
    let Ok(golden) = std::fs::read_to_string(&path) else {
        panic!("{path} does not read: the oracle has not been written (mycelium-soak --write-oracle-golden)")
    };
    let want: Vec<&str> = golden.lines().collect();
    let dir = mycelium_source_arc_agi::corpus("training");
    let names = mycelium_source_arc_agi::task_names(&dir).expect("the corpus stands");
    let tasks: Vec<_> = names
        .iter()
        .take(TASKS)
        .filter_map(|name| std::fs::read_to_string(format!("{dir}/{name}")).ok())
        .filter_map(|json| mycelium_source_arc_agi::decode(&json))
        .collect();
    let got = readings(&inputs(&tasks));
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
