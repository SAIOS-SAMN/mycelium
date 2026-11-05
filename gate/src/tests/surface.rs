// The fixtures below state the retired phrases in order to prove the check
// refuses them; this file is outside the surface the check reads.

use super::super::surface::lint_surface;
use std::path::PathBuf;

fn hits(files: &[(&str, &str)]) -> Vec<String> {
    let dir = tempfile::tempdir().expect("a directory to write into");
    for (path, text) in files {
        std::fs::write(dir.path().join(path), text).expect("written");
    }
    let mut h = Vec::new();
    lint_surface(&[PathBuf::from(dir.path())], &mut h).expect("the walk reads");
    h
}

/// Each retired phrase is refused wherever it stands, in a comment as in
/// code and in any case, with its file, line and ruling.
#[test]
fn lint_surface_refuses_each_retired_phrase() {
    let h = hits(&[
        ("a.rs", "/// A Quarter Turn, clockwise.\nfn a() {}\n"),
        ("b.md", "The kernel signs the Concordance hash.\nA wallet is flat by design; each ledger row is a balance.\n"),
        ("c.toml", "description = \"a half-turn\"\n"),
    ]);
    assert_eq!(h.len(), 6, "{h:?}");
    assert!(h.iter().any(|l| l.contains("a.rs:1") && l.contains("`quarter turn`") && l.contains("ruling 1")), "{h:?}");
    assert!(h.iter().any(|l| l.contains("a.rs:1") && l.contains("`clockwise`")), "{h:?}");
    assert!(h.iter().any(|l| l.contains("b.md:1") && l.contains("`concordance hash`") && l.contains("ruling 4")), "{h:?}");
    assert!(h.iter().any(|l| l.contains("b.md:2") && l.contains("`a wallet is flat`") && l.contains("ruling 2")), "{h:?}");
    assert!(h.iter().any(|l| l.contains("b.md:2") && l.contains("`ledger row`")), "{h:?}");
    assert!(h.iter().any(|l| l.contains("c.toml:1") && l.contains("`half-turn`")), "{h:?}");
}

/// The lawful wording passes: degrees on a named plane, signing by
/// reconstruction, the holding as a coordinate; and a file of another kind
/// is not read.
#[test]
fn lint_surface_passes_lawful_wording() {
    let h = hits(&[
        ("a.rs", "/// The rotation by 90° in the e₂e₁ plane; a turn of phase in ℚ/ℤ.\nfn a() {}\n"),
        ("b.md", "Signs the Concordance clause by clause by reconstructing its fixtures.\nA wallet is a reading of the holding's coordinate.\n"),
        ("c.txt", "a quarter turn, clockwise\n"),
    ]);
    assert!(h.is_empty(), "{h:?}");
}
