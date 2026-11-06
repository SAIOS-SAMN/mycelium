// MU-STANDARDS 7.1 — THE DOCUMENTS NAME WHAT EXISTS, read as tests over the
// repository's documents beside its code.

use super::super::documents::{anchor_of, cited, lint_documents, lint_fragments, lint_section, Cited, Section};
use std::path::{Path, PathBuf};

/// The repository root: the gate's manifest sits one level under it.
fn repo() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("the gate's manifest sits in the repository")
}

/// A cited path is an inline code span in path characters under a
/// top-level directory, its line or item suffix read off; a pattern, a
/// relative path, a fenced block and a span in no directory of the tree
/// are not read.
#[test]
fn cited_reads_paths_of_code_spans() {
    let text = "see `src/a.rs:12` and `gate/src/numbers.rs::Tree`, `docs/`\n\
                not `src/bin/{a,b}.rs`, `<module>/tests/fixtures.rs`, `proposals/x.md`, `cargo test`\n\
                ```sh\n`src/fenced.rs`\n```\n\
                `xtask/main.rs` was the gate\n";
    let read = cited(text);
    let paths: Vec<(usize, &str, bool)> = read.iter().map(|c| (c.line, c.path.as_str(), c.gone)).collect();
    assert_eq!(
        paths,
        [(1, "src/a.rs", false), (1, "gate/src/numbers.rs", false), (1, "docs/", false), (6, "xtask/main.rs", true)]
    );
    assert_eq!(cited("`src/x.rs`"), [Cited { line: 1, path: "src/x.rs".to_string(), gone: false }]);
}

/// A cited path that names nothing of the tree is refused with its document
/// and line; one that exists, or one whose line says it was removed or
/// retired, stands.
#[test]
fn lint_documents_refuses_missing_path() {
    let dir = tempfile::tempdir().expect("a directory to write into");
    std::fs::create_dir_all(dir.path().join("src")).expect("created");
    std::fs::write(dir.path().join("src/held.rs"), "").expect("written");
    let doc = dir.path().join("DOC.md");
    std::fs::write(
        &doc,
        "`src/held.rs` stands\n`src/gone.rs` is cited\n`src/old.rs` was removed\n`src/older.rs`, retired\n",
    )
    .expect("written");
    let mut hits = Vec::new();
    let read = lint_documents(dir.path(), &[PathBuf::from(&doc)], &mut hits).expect("the documents read");
    assert_eq!(read, 4, "four paths cited");
    assert_eq!(hits.len(), 1, "{hits:?}");
    assert!(hits[0].contains("DOC.md:2") && hits[0].contains("`src/gone.rs`"), "{hits:?}");
}

/// A changelog fragment is a non-empty markdown file opened by its `### `
/// heading; an empty file, a file opened otherwise, a heading with no
/// title and a file of another kind are each refused by name.
#[test]
fn lint_fragments_refuses_fragment_without_heading() {
    let dir = tempfile::tempdir().expect("a directory to write into");
    let files = [
        ("entry.md", "### A change\n- what it does\n"),
        ("empty.md", ""),
        ("prose.md", "A change without its heading\n"),
        ("level.md", "## A release, not an entry\n"),
        ("untitled.md", "### \n- what\n"),
        ("entry.txt", "### A change\n"),
    ];
    for (name, text) in files {
        std::fs::write(dir.path().join(name), text).expect("written");
    }
    let mut hits = Vec::new();
    let read = lint_fragments(dir.path(), &mut hits).expect("the fragments read");
    assert_eq!(read, 6);
    assert_eq!(hits.len(), 5, "{hits:?}");
    assert!(!hits.iter().any(|h| h.contains("entry.md")), "the entry stands: {hits:?}");
}

/// A heading's anchor drops punctuation and arrows, keeps letters and
/// digits, and writes each space as `-`, so an arrow between two spaces
/// leaves `--`.
#[test]
fn anchor_of_reads_rendered_anchor() {
    assert_eq!(
        anchor_of("9. The language's traversal: sensorium → mind → language → hypha → minds"),
        "9-the-languages-traversal-sensorium--mind--language--hypha--minds"
    );
    assert_eq!(anchor_of("5a.6 Keys versus topology"), "5a6-keys-versus-topology");
}

/// A section is refused by name where its home lacks a heading, and where a
/// linking document holds no link to the first heading's anchor; a fenced
/// heading is no heading.
#[test]
fn lint_section_refuses_missing_heading_and_link() {
    let dir = tempfile::tempdir().expect("a directory to write into");
    let files = [
        ("HOME.md", "# Home\n\n## 2. The part: a → b\n\n```\n### 2.1 Fenced\n```\n"),
        ("LINKS.md", "see [the part](HOME.md#2-the-part-a--b)\n"),
        ("BARE.md", "see HOME.md\n"),
    ];
    for (name, text) in files {
        std::fs::write(dir.path().join(name), text).expect("written");
    }
    let section = Section {
        home: "HOME.md",
        headings: &["2. The part: a → b", "2.1 Fenced"],
        linked_from: &["LINKS.md", "BARE.md"],
    };
    let mut hits = Vec::new();
    lint_section(dir.path(), &section, &mut hits).expect("the documents read");
    assert_eq!(hits.len(), 2, "{hits:?}");
    assert!(hits.iter().any(|h| h.contains("HOME.md") && h.contains("2.1 Fenced")), "{hits:?}");
    assert!(hits.iter().any(|h| h.contains("BARE.md") && h.contains("HOME.md#2-the-part-a--b")), "{hits:?}");
}

/// The rows of a markdown table's section: the lines of `text` from the
/// heading `heading` to the next `## ` heading that open with `|`, each
/// split into its trimmed cells.
fn table_rows(text: &str, heading: &str) -> Vec<Vec<String>> {
    text.lines()
        .skip_while(|l| l.trim() != heading)
        .skip(1)
        .take_while(|l| !l.starts_with("## "))
        .filter(|l| l.trim_start().starts_with('|'))
        .map(|l| l.trim().trim_matches('|').split('|').map(|c| c.trim().to_string()).collect())
        .collect()
}

/// The shared-law table's forms rows, `(id, version)` as written: the rows
/// of `## 11. Shared laws` whose first cell is a forms law id (`FORMS-…`).
fn forms_rows(text: &str) -> Vec<(String, String)> {
    table_rows(text, "## 11. Shared laws")
        .iter()
        .filter_map(|cells| match cells.as_slice() {
            [id, version, ..] => {
                let id = id.trim_matches('`');
                id.starts_with("FORMS-").then(|| (id.to_string(), version.clone()))
            }
            _ => None,
        })
        .collect()
}

/// The table is read by its id and version cells, the forms rows only: a
/// row of another law is passed over, and a forms row at another version
/// reads as written, so it differs from the shell's own.
#[test]
fn forms_rows_reads_id_and_version_cells() {
    let text = "## 11. Shared laws\n\n| Id | Version | Text |\n|---|---|---|\n| L0 momentum | 5 | x |\n| `FORMS-FRAME` | 2 | y |\n\n## 12. Next\n\n| `FORMS-SHARES` | 1 | z |\n";
    assert_eq!(forms_rows(text), [("FORMS-FRAME".to_string(), "2".to_string())]);
    assert!(!forms::LAW_IDS.contains(&("FORMS-FRAME", 2)), "a version the shell does not hold");
}

/// MU-EQUATIONS §11 — THE SHARED-LAW TABLE NAMES THE FORMS AS THEY ARE: its
/// rows whose id is a forms law id (`FORMS-…`) name exactly the ids and
/// versions of `forms::LAW_IDS`, in its order, so a shell's version raised
/// in code and not in the table (or the reverse) is refused.
#[test]
fn shared_law_table_equals_forms_law_ids() {
    let text = std::fs::read_to_string(repo().join("docs/MU-EQUATIONS.md")).expect("MU-EQUATIONS reads");
    let held: Vec<(String, String)> = forms::LAW_IDS.iter().map(|(id, v)| (id.to_string(), v.to_string())).collect();
    assert_eq!(forms_rows(&text), held, "MU-EQUATIONS §11's forms rows against forms::LAW_IDS");
}
