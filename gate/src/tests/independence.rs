use super::super::euclidean::lint_euclidean;
use super::super::length::lint_file_length;
use std::path::PathBuf;

/// I4 — A TREE RED ON THE BOUND STILL REPORTS THE OTHER CHECKS. The gate
/// once stopped at the reading bound and never read further while the bound
/// was red, so refusals accumulated unseen behind it. Each check reads the
/// tree on its own: one file over the bound that also names a refused shape
/// is refused by both, the first not hiding the second.
#[test]
fn checks_report_vocabulary_under_red_bound() {
    let dir = tempfile::tempdir().expect("a directory to write into");
    let shape = concat!("bb", "ox");
    let mut long: String = std::iter::repeat_n("fn a() {}\n", 2501).collect();
    long.push_str(&format!("fn {shape}_of() {{}}\n"));
    std::fs::write(dir.path().join("long.rs"), long).expect("written");
    let roots = [PathBuf::from(dir.path())];
    let mut bound = Vec::new();
    lint_file_length(&roots, &mut bound).expect("the walk reads");
    let mut vocabulary = Vec::new();
    lint_euclidean(&roots, &mut vocabulary).expect("the walk reads");
    assert_eq!(bound.len(), 1, "the bound refuses the file: {bound:?}");
    assert_eq!(vocabulary.len(), 1, "and the vocabulary is still read: {vocabulary:?}");
}
