// THE READING BOUND (MU-STANDARDS 1.1): no source file over 600 lines, and
// the list of what is still over it.

use std::fs;
use std::path::{Path, PathBuf};

pub(crate) const AWAITING_SPLIT: &[(&str, &str)] = &[
    ("src/core/rcf.rs", "F2"),
];



pub(crate) fn lint_file_length(roots: &[PathBuf], hits: &mut Vec<String>) -> Result<(), String> {
    const BOUND: usize = 600;
    fn owed(path: &Path) -> Option<&'static str> {
        let p = path.to_string_lossy().replace('\\', "/");
        AWAITING_SPLIT
            .iter()
            .find(|(f, _)| p.ends_with(f))
            .map(|(_, row)| *row)
    }
    fn walk_len(dir: &Path, hits: &mut Vec<String>, seen: &mut Vec<String>) -> Result<(), String> {
        let rd = match fs::read_dir(dir) {
            Ok(rd) => rd,
            Err(_) => return Ok(()),
        };
        for ent in rd {
            let ent = ent.map_err(|e| e.to_string())?;
            let path = ent.path();
            if path.is_dir() {
                walk_len(&path, hits, seen)?;
                continue;
            }
            if path.extension().and_then(|s| s.to_str()) != Some("rs") {
                continue;
            }
            let n = fs::read_to_string(&path)
                .map_err(|e| format!("{}: {e}", path.display()))?
                .lines()
                .count();
            match owed(&path) {
                Some(row) => {
                    seen.push(row.to_string());
                    if n <= BOUND {
                        hits.push(format!(
                            "{}: {n} lines, already under the {BOUND}-line bound — delete its {row} row from AWAITING_SPLIT",
                            path.display()
                        ));
                    }
                }
                None if n > BOUND => hits.push(format!(
                    "{}: {n} lines, over the {BOUND}-line reading bound — split it",
                    path.display()
                )),
                None => {}
            }
        }
        Ok(())
    }
    let mut seen: Vec<String> = Vec::new();
    for root in roots {
        walk_len(root, hits, &mut seen)?;
    }
    for (file, row) in AWAITING_SPLIT {
        if !seen.iter().any(|r| r == row) {
            hits.push(format!(
                "{file}: named by {row} in AWAITING_SPLIT and not found — delete the row"
            ));
        }
    }
    Ok(())
}

/// Is this file test code the compiler only builds under `#[cfg(test)]`?
/// Two shapes count, and both are CHECKED rather than trusted to a name:
/// `<module>/tests.rs`, whose owner is `<module>/mod.rs`; and
/// `<module>/tests/<chapter>.rs` (MU-STANDARDS 3.3), whose owner is the same
/// `<module>/mod.rs` one directory further up. In both the owner must carry
/// `#[cfg(test)] mod tests;`, so a file merely NAMED `tests.rs`, or one
/// dropped into a directory called `tests` that nothing declares, is still
/// linted in full.
pub(crate) fn is_declared_test_file(path: &Path) -> bool {
    let Some(dir) = path.parent() else {
        return false;
    };
    let named_tests_rs = path.file_name().and_then(|s| s.to_str()) == Some("tests.rs");
    let in_tests_dir = dir.file_name().and_then(|s| s.to_str()) == Some("tests");
    let owner = if named_tests_rs {
        dir.join("mod.rs")
    } else if in_tests_dir {
        match dir.parent() {
            Some(up) => up.join("mod.rs"),
            None => return false,
        }
    } else {
        return false;
    };
    let src = match fs::read_to_string(&owner) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let mut saw_cfg_test = false;
    for line in src.lines() {
        let t = line.trim();
        if t.contains("#[cfg(test)]") {
            saw_cfg_test = true;
            continue;
        }
        if t == "mod tests;" || t == "pub mod tests;" {
            return saw_cfg_test;
        }
        if !t.is_empty() && !t.starts_with("//") {
            saw_cfg_test = false;
        }
    }
    false
}
