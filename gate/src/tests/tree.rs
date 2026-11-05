// MU-STANDARDS 4.1 — THE GATE OVER THIS REPOSITORY: each check of the crate
// read against the tree it sits in. Each is a test of its own, so a red
// check never hides another (I4).

use super::super::drift::{lint_drift, REGISTER};
use super::super::euclidean::lint_euclidean;
use super::super::length::lint_file_length;
use super::super::numbers::lint_numbers;
use std::path::{Path, PathBuf};

/// The repository root: the gate's manifest sits one level under it.
fn repo() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("the gate's manifest sits in the repository")
}

/// The roots the reading bound and the vocabulary read: every workspace
/// member's source and the gate itself.
fn bounded() -> Vec<PathBuf> {
    let up = repo();
    vec![up.join("src"), up.join("algebra/src"), up.join("forms/src"), up.join("sdk/src"), up.join("gate")]
}

/// MU-STANDARDS 1.1 — no source file of the tree is over the reading bound.
#[test]
fn lint_file_length_refuses_nothing_in_tree() {
    let mut hits = Vec::new();
    lint_file_length(&bounded(), &mut hits).expect("the tree reads");
    assert!(hits.is_empty(), "mycelium gates refused this tree:\n{}", hits.join("\n"));
}

/// Law 2, rule 15, the Anti-Flattening Law — no code line of the tree names
/// a grid-indexing shape.
#[test]
fn lint_euclidean_refuses_nothing_in_tree() {
    let mut hits = Vec::new();
    lint_euclidean(&bounded(), &mut hits).expect("the tree reads");
    assert!(hits.is_empty(), "mycelium gates refused this tree:\n{}", hits.join("\n"));
}

/// NO CONSTANT over the tree itself: every literal-bearing numeric item of
/// the repository this gate sits in is classified, and the register agrees
/// with its sites. N6, the inline literals of the law paths, is the
/// inventory this test writes to its output (`--nocapture` shows it); it
/// refuses nothing yet.
#[test]
fn numbers_refuses_nothing_in_tree() {
    let reading = lint_numbers(repo()).expect("the tree reads");
    for i in &reading.inventory {
        eprintln!("{}:{}: {}", i.file, i.line, i.literal);
    }
    eprintln!(
        "N6 inventory: {} inline literals; N1–N5: {} classified items, {} refused",
        reading.inventory.len(),
        reading.items,
        reading.hits.len()
    );
    assert!(reading.hits.is_empty(), "the numbers gate refuses:\n{}", reading.hits.join("\n"));
}

/// THE DRIFT REGISTER: each registered guard's type is declared and its
/// claim is proved in this tree.
#[test]
fn drift_register_holds_in_this_tree() {
    let mut hits = Vec::new();
    lint_drift(repo(), REGISTER, &mut hits).expect("the tree reads");
    assert!(hits.is_empty(), "{hits:?}");
}
