// Rules 1 and 8 — ENUMERATION: a loop over a carrier reads a keyed source,
// or it is a search.

use std::fs;
use std::path::Path;

use super::length::is_declared_test_file;

pub(crate) fn walk(dir: &Path, crate_dir: &str, hits: &mut Vec<String>) -> Result<(), String> {
    let rd = fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for ent in rd {
        let ent = ent.map_err(|e| e.to_string())?;
        let path = ent.path();
        if path.is_dir() {
            walk(&path, crate_dir, hits)?;
            continue;
        }
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        // A split-out test module is test code in whole. The gates below
        // find `#[cfg(test)]` by scanning WITHIN a file and tracking brace
        // depth, so a test module moved to its own file carries no marker and
        // every gate fires on it. The exemption is checked, not assumed:
        // `is_declared_test_file` reads the owning module's declaration.
        if is_declared_test_file(&path) {
            continue;
        }
        // Wire MTU is the byte size of one spend; physics names the number.
        if crate_dir == "network" && path.file_name().and_then(|s| s.to_str()) == Some("wire.rs") {
            lint_file(&path, hits, true)?;
        } else {
            lint_file(&path, hits, false)?;
        }
    }
    Ok(())
}

/// Stage 8b's reading bound. Part VII's rule of verification is that the
/// auditor reads the function and its carriers END TO END, and grep is not
/// verification; a file no one reads whole is a file whose faults are found
/// by soak instead of by review. Every carrier fault of the G-set was
/// invisible at 8000 lines and obvious at the function.
///
/// 600 is the one literal this tree allows, and it is allowed because it
/// measures a person and not the mathematics (rule 5 bans constants that
/// stand in for a measurement of the world; this stands in for a reading).
/// It was 800 through Stage 8b's twenty-four splits and is 600 from S24
/// on, which is the tree's final reading bound (MU-STANDARDS 1.1).
///
/// A file still awaiting its split is named in `AWAITING_SPLIT` with the
/// row of Stage 8b that owes it, so the gate is green on every commit of
/// the stage and the list IS the remaining work: each split commit deletes
/// its own row, and the stage ends when the list is empty. A row whose file
/// is already under the bound, or no longer exists, is a stale row and is
/// itself a hit — the list cannot outlive what it excuses.
pub(crate) fn lint_file(path: &Path, hits: &mut Vec<String>, allow_spend: bool) -> Result<(), String> {
    let src = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut in_test = false;
    let mut pending_test = false;
    let mut depth = 0i32;
    for (i, line) in src.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.contains("#[cfg(test)]") {
            pending_test = true;
        }
        let opens = line.chars().filter(|&c| c == '{').count() as i32;
        let closes = line.chars().filter(|&c| c == '}').count() as i32;
        if pending_test && opens > 0 {
            in_test = true;
            pending_test = false;
            depth = opens - closes;
            if depth <= 0 {
                in_test = false;
            }
            continue;
        }
        if in_test {
            depth += opens - closes;
            if depth <= 0 {
                in_test = false;
            }
            continue;
        }
        let mut check = |pat: &str| {
            if line.contains(pat) {
                hits.push(format!("{}:{}: {pat}", path.display(), i + 1));
            }
        };
        check(".entries()");
        check(".for_each(");
        check(".for_each_from(");
        if !allow_spend {
            check("spend_unit()");
        }
        if path.components().any(|c| c.as_os_str() == "sentence") {
            check(".position(");
            check(".find(");
            check(".find_map(");
            check("Vec<bool>");
            check("unwrap_or");
        }
    }
    Ok(())
}
