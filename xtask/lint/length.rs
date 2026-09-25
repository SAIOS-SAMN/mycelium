// MU-STANDARDS 1.1 — THE READING BOUND: no source file over 2500 lines,
// so a reviewer can read one whole.
//
// RAISED TO 2500 AND RETURNED, 2026-09-19. The raise landed at 3245041 on
// a word the operator gave the implementor directly and which never passed
// through the coordinator's console; every directive is relayed there, so
// the raise had no standing however the word was meant, and it is
// restored here. The argument for it — that a bound which times an organ
// split is deciding architecture by a number — is a case to be put through
// the console, not a change to be made on the strength of it.
//
// RAISED TO 2500, 2026-09-25, THROUGH THE CONSOLE. The operator's word was
// relayed verbatim to the coordinator and ruled into MU-STANDARDS 1.1 and
// 4.1 before this changed — the passage the 2026-09-19 raise lacked. No
// other rule moves: an organ is split where the document splits it (1.2),
// never where a number does.

use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn lint_file_length(roots: &[PathBuf], hits: &mut Vec<String>) -> Result<(), String> {
    const BOUND: usize = 2500;
    fn walk_len(dir: &Path, hits: &mut Vec<String>) -> Result<(), String> {
        let rd = match fs::read_dir(dir) {
            Ok(rd) => rd,
            Err(_) => return Ok(()),
        };
        for ent in rd {
            let ent = ent.map_err(|e| e.to_string())?;
            let path = ent.path();
            if path.is_dir() {
                walk_len(&path, hits)?;
                continue;
            }
            if path.extension().and_then(|s| s.to_str()) != Some("rs") {
                continue;
            }
            let n = fs::read_to_string(&path)
                .map_err(|e| format!("{}: {e}", path.display()))?
                .lines()
                .count();
            if n > BOUND {
                hits.push(format!(
                    "{}: {n} lines, over the {BOUND}-line reading bound — split it",
                    path.display()
                ));
            }
        }
        Ok(())
    }
    for root in roots {
        walk_len(root, hits)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::lint_file_length;
    use std::path::PathBuf;

    /// MU-STANDARDS 3.5: the bound has a test of its own. A tree of files
    /// under the bound passes; one over it is named with its line count.
    #[test]
    fn a_file_over_the_bound_is_named_and_one_under_it_is_not() {
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
}
