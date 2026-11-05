// MU-STANDARDS 7.1 — THE DOCUMENTS NAME WHAT EXISTS, read as tests over the
// repository's documents beside its code.

use std::path::Path;

/// The repository root: the gate's manifest sits one level under it.
fn repo() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("the gate's manifest sits in the repository")
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
