use super::super::drift::{lint_drift, Guard, Proof};
use std::path::Path;

/// A fixture tree: each `(path, text)` written under a fresh directory.
fn tree(files: &[(&str, &str)]) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("a directory to write into");
    for (path, text) in files {
        let p = dir.path().join(path);
        std::fs::create_dir_all(p.parent().expect("a parent")).expect("created");
        std::fs::write(p, text).expect("written");
    }
    dir
}

fn hits(dir: &Path, register: &[Guard]) -> Vec<String> {
    let mut h = Vec::new();
    lint_drift(dir, register, &mut h).expect("the tree reads");
    h
}

/// One entry guarding by `guard`, proved by `claim`.
fn entry(guard: &'static str, claim: &'static str, proof: Proof) -> Guard {
    Guard { regression: "a digest compared to admit", ruling: "a ruling", guard, claim, proof }
}

/// A type with no equality, its compile_fail doc test and a test, as the
/// tree writes them.
const GUARDED: &str = "/// The type has no `==` (`address_has_no_equality`):\n///\n/// ```compile_fail,E0369\n/// let _ = a == b;\n/// ```\npub struct Locator([u8; 32]);\n\n/// A claim.\n#[test]\nfn commitment_binds_only_a_reconstructed_object() {}\n";

/// An entry whose type is declared and whose claims stand is admitted.
#[test]
fn drift_admits_entry_with_type_and_claim() {
    let dir = tree(&[("src/digest.rs", GUARDED)]);
    let register = [
        entry("Locator", "address_has_no_equality", Proof::CompileFail),
        entry("Locator", "commitment_binds_only_a_reconstructed_object", Proof::Test),
    ];
    assert!(hits(dir.path(), &register).is_empty());
}

/// AN ENTRY MISSING ITS CLAIM IS REFUSED: named empty, or named and
/// absent from the tree, or present only as prose without its fence or its
/// `#[test]`.
#[test]
fn drift_refuses_entry_missing_its_claim() {
    let dir = tree(&[("src/digest.rs", GUARDED)]);
    let empty = hits(dir.path(), &[entry("Locator", "", Proof::Test)]);
    assert_eq!(empty.len(), 1, "{empty:?}");
    assert!(empty[0].contains("names no claim"), "{empty:?}");
    let absent = hits(dir.path(), &[entry("Locator", "locator_is_unwritten", Proof::Test)]);
    assert_eq!(absent.len(), 1, "{absent:?}");
    assert!(absent[0].contains("`locator_is_unwritten`"), "{absent:?}");
    let wrong_proof = hits(dir.path(), &[entry("Locator", "commitment_binds_only_a_reconstructed_object", Proof::CompileFail)]);
    assert_eq!(wrong_proof.len(), 1, "a test is not a compile_fail doc test: {wrong_proof:?}");
    let unfenced = tree(&[("src/a.rs", "/// `address_has_no_equality`, stated and never proved.\npub struct Locator;\n")]);
    let h = hits(unfenced.path(), &[entry("Locator", "address_has_no_equality", Proof::CompileFail)]);
    assert_eq!(h.len(), 1, "a claim named without its fence is missing: {h:?}");
}

/// AN ENTRY MISSING ITS TYPE IS REFUSED: named empty, or declared by no
/// struct, enum or trait (a longer name that contains it does not count).
#[test]
fn drift_refuses_entry_missing_its_type() {
    let dir = tree(&[("src/digest.rs", GUARDED), ("src/b.rs", "pub struct LocatorTable;\n")]);
    let empty = hits(dir.path(), &[entry("", "address_has_no_equality", Proof::CompileFail)]);
    assert_eq!(empty.len(), 1, "{empty:?}");
    assert!(empty[0].contains("names no guarding type"), "{empty:?}");
    let absent = hits(dir.path(), &[entry("Locat", "address_has_no_equality", Proof::CompileFail)]);
    assert_eq!(absent.len(), 1, "a prefix of a declared name is not the type: {absent:?}");
}
