// This file is named as the check it proves, and the check skips a file of
// its own name: the fixtures below state the refused vocabulary.

use super::super::euclidean::lint_euclidean;
use std::path::PathBuf;

fn hits(files: &[(&str, &str)]) -> Vec<String> {
    let dir = tempfile::tempdir().expect("a directory to write into");
    for (path, text) in files {
        std::fs::write(dir.path().join(path), text).expect("written");
    }
    let mut h = Vec::new();
    lint_euclidean(&[PathBuf::from(dir.path())], &mut h).expect("the walk reads");
    h
}

/// A code line naming a grid-indexing shape is refused with its line and
/// the reason, once per word it names.
#[test]
fn lint_euclidean_refuses_grid_vocabulary_in_code() {
    let h = hits(&[("a.rs", "fn a() {}\nfn morton_key(bbox: u8) {}\n")]);
    assert_eq!(h.len(), 2, "{h:?}");
    assert!(h.iter().all(|l| l.contains("a.rs:2")), "{h:?}");
    assert!(h.iter().any(|l| l.contains("`morton`") && l.contains("law 2")), "{h:?}");
    assert!(h.iter().any(|l| l.contains("`bbox`") && l.contains("Anti-Flattening")), "{h:?}");
}

/// A comment or an attribute line only names the refusal, and is read as
/// prose; a file other than `.rs` is not read.
#[test]
fn lint_euclidean_ignores_comments_and_attributes() {
    let h = hits(&[
        ("a.rs", "// a quadtree is refused\n/// so is a voxel\n#[doc = \"octree\"]\nfn a() {}\n"),
        ("b.md", "fn morton_key() {}\n"),
    ]);
    assert!(h.is_empty(), "{h:?}");
}
