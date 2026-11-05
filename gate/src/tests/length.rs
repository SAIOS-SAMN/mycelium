use super::super::length::lint_file_length;
use std::path::PathBuf;

/// MU-STANDARDS 3.5: the bound has a test of its own. A tree of files
/// under the bound passes; one over it is named with its line count.
#[test]
fn lint_file_length_names_only_file_over_bound() {
    let dir = tempfile::tempdir().expect("a directory to write into");
    std::fs::write(dir.path().join("short.rs"), "fn a() {}\n").expect("written");
    let long: String = std::iter::repeat_n("fn a() {}\n", 2501).collect();
    std::fs::write(dir.path().join("long.rs"), long).expect("written");
    let mut hits = Vec::new();
    lint_file_length(&[PathBuf::from(dir.path())], &mut hits).expect("the walk reads");
    assert_eq!(hits.len(), 1, "one file is over the bound: {hits:?}");
    assert!(hits[0].contains("long.rs"), "and it is named: {hits:?}");
    assert!(hits[0].contains("2501 lines"), "with its length: {hits:?}");
}
