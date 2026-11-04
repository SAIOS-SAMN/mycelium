use super::super::numbers::lint_numbers;
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

fn hits(dir: &Path) -> Vec<String> {
    lint_numbers(dir).expect("the tree reads").hits
}

/// A register naming `NAME` at `site` with `value`.
fn register(entries: &[(&str, &str, &str)]) -> String {
    let body: Vec<String> = entries
        .iter()
        .map(|(name, value, site)| {
            format!(
                "OpenNumber {{ name: \"{name}\", value: \"{value}\", site: \"{site}\", governs: Governs::Reading, doc: \"d\", derivation: \"o\", kind: OpenKind::Physics }}"
            )
        })
        .collect();
    format!("pub const OPEN_PHYSICS: [OpenNumber; {}] = [{}];\n", entries.len(), body.join(", "))
}

/// N4: a numeric item with a literal outside `{0, 1}` and no class tag is
/// refused, named with its literal.
#[test]
fn numbers_refuses_unclassified_literal_const() {
    let dir = tree(&[("src/a.rs", "pub const WINDOW: u64 = 1024;\n")]);
    let h = hits(dir.path());
    assert_eq!(h.len(), 1, "{h:?}");
    assert!(h[0].contains("`WINDOW`") && h[0].contains("[1024]") && h[0].contains("N4"), "{h:?}");
}

/// N3: an initializer of paths, `0`, `1`, operators and `T::MAX` is derived
/// and needs no class.
#[test]
fn numbers_admits_derived_const_without_literal() {
    let dir = tree(&[("src/a.rs", "pub const A: u64 = 1 << B;\npub const C: usize = usize::MAX - 1;\npub const D: u8 = 0;\n")]);
    assert!(hits(dir.path()).is_empty());
}

/// N2: a float item is refused outright, tagged or not.
#[test]
fn numbers_refuses_float_const() {
    let dir = tree(&[("src/a.rs", "/// FORMAT: a layout.\npub const F: f64 = 0.5;\n")]);
    let h = hits(dir.path());
    assert_eq!(h.len(), 1, "{h:?}");
    assert!(h[0].contains("N2") && h[0].contains("float"), "{h:?}");
}

/// N4: `MEASURED:` stands only under the instruments; at a law path it is
/// refused.
#[test]
fn numbers_refuses_measured_in_law_path() {
    let dir = tree(&[
        ("src/physics/a.rs", "/// MEASURED: a host reading.\npub const M: u64 = 64;\n"),
        ("src/bin/x/a.rs", "/// MEASURED: a host reading.\npub const M: u64 = 64;\n"),
    ]);
    let h = hits(dir.path());
    assert_eq!(h.len(), 1, "{h:?}");
    assert!(h[0].contains("src/physics/a.rs") && h[0].contains("MEASURED"), "{h:?}");
}

/// N4: `EXACT:` must name a `#[test]` fn of the tree.
#[test]
fn numbers_refuses_exact_tag_without_test() {
    let dir = tree(&[
        ("src/a.rs", "/// EXACT: aperture_equals_unit_cell\npub const A: u64 = 8;\n/// EXACT: missing_claim\npub const B: u64 = 8;\n"),
        ("src/tests/a.rs", "#[test]\nfn aperture_equals_unit_cell() {}\n"),
    ]);
    let h = hits(dir.path());
    assert_eq!(h.len(), 1, "{h:?}");
    assert!(h[0].contains("`B`") && h[0].contains("missing_claim"), "{h:?}");
}

/// N5: a `REGISTERED` item with no register entry is refused.
#[test]
fn numbers_refuses_registered_without_entry() {
    let dir = tree(&[
        ("src/physics/axioms/founding.rs", &register(&[])),
        ("src/a.rs", "/// REGISTERED: W\npub const W: u64 = 1024;\n"),
    ]);
    let h = hits(dir.path());
    assert_eq!(h.len(), 1, "{h:?}");
    assert!(h[0].contains("N5") && h[0].contains("no entry"), "{h:?}");
}

/// N5: an entry whose site holds no `REGISTERED` item is stale and refused.
#[test]
fn numbers_refuses_stale_register_entry() {
    let dir = tree(&[
        ("src/physics/axioms/founding.rs", &register(&[("W", "1024", "src/a.rs::W")])),
        ("src/a.rs", "pub const V: u64 = 0;\n"),
    ]);
    let h = hits(dir.path());
    assert_eq!(h.len(), 1, "{h:?}");
    assert!(h[0].contains("stale"), "{h:?}");
}

/// N5: an item whose initializer differs from the register's value is
/// refused, naming both.
#[test]
fn numbers_refuses_register_value_mismatch() {
    let dir = tree(&[
        ("src/physics/axioms/founding.rs", &register(&[("W", "1024", "src/a.rs::W")])),
        ("src/a.rs", "/// REGISTERED: W\npub const W: u64 = 2048;\n"),
    ]);
    let h = hits(dir.path());
    assert_eq!(h.len(), 1, "{h:?}");
    assert!(h[0].contains("2048") && h[0].contains("1024"), "{h:?}");
    let agreed = tree(&[
        ("src/physics/axioms/founding.rs", &register(&[("W", "768 * 1024", "src/a.rs::W")])),
        ("src/a.rs", "/// REGISTERED: W\npub const W: usize = 768 * 1024;\n"),
    ]);
    assert!(hits(agreed.path()).is_empty(), "a register agreeing with its item passes");
}

/// N1: items under `#[cfg(test)]` and files under `tests/` are outside the
/// gate.
#[test]
fn numbers_ignores_cfg_test_items() {
    let dir = tree(&[
        ("src/a.rs", "#[cfg(test)]\nconst A: u64 = 9;\n#[cfg(test)]\nmod t { const B: u64 = 9; }\n"),
        ("src/tests/b.rs", "const C: u64 = 9;\n"),
    ]);
    assert!(hits(dir.path()).is_empty());
}

/// NO CONSTANT over the tree itself: every literal-bearing numeric item of
/// the repository this gate sits in is classified, and the register agrees
/// with its sites.
#[test]
fn numbers_refuses_nothing_in_tree() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("the xtask manifest sits in the repository");
    let reading = lint_numbers(repo).expect("the tree reads");
    assert!(reading.hits.is_empty(), "the numbers gate refuses:\n{}", reading.hits.join("\n"));
}

/// N6: an inline literal in a law path is read into the inventory, outside a
/// codec and a byte index, and refuses nothing.
#[test]
fn numbers_inventories_inline_literal_in_law_path() {
    let dir = tree(&[(
        "src/physics/a.rs",
        "fn patience() -> u64 { 5 }\nfn encode_x(b: &[u8]) -> u8 { b[3] + 4 }\nfn at(b: &[u8]) -> u8 { b[7] }\n",
    )]);
    let reading = lint_numbers(dir.path()).expect("the tree reads");
    assert!(reading.hits.is_empty());
    assert_eq!(reading.inventory.len(), 1, "{:?}", reading.inventory);
    assert_eq!(reading.inventory[0].literal, "5");
}
