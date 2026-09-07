// Law 2 — MAPS: a hash map in cognition is an identity by digest.

use std::fs;
use super::length::is_declared_test_file;
use std::path::Path;

pub(crate) fn lint_maps(root: &Path, hits: &mut Vec<String>) -> Result<(), String> {
    for d in ["server", "client", "transducer", "court"] {
        walk_maps(&root.join(d), hits)?;
    }
    Ok(())
}

pub(crate) fn walk_maps(path: &Path, hits: &mut Vec<String>) -> Result<(), String> {
    if path.is_dir() {
        let rd = match fs::read_dir(path) {
            Ok(rd) => rd,
            Err(_) => return Ok(()),
        };
        for ent in rd {
            walk_maps(&ent.map_err(|e| e.to_string())?.path(), hits)?;
        }
        return Ok(());
    }
    if path.extension().and_then(|s| s.to_str()) != Some("rs") {
        return Ok(());
    }
    if is_declared_test_file(path) {
        return Ok(());
    }
    let src = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut in_test = false;
    let mut pending_test = false;
    let mut depth = 0i32;
    for (i, line) in src.lines().enumerate() {
        let t = line.trim();
        if t.contains("#[cfg(test)]") {
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
        if t.starts_with("//") {
            continue;
        }
        for pat in ["HashMap", "BTreeMap", "HashSet", "BTreeSet"] {
            if t.contains(pat) {
                hits.push(format!(
                    "{}:{}: {pat} in cognition — the carrier is a Keyed trie or a Vec of relations (rule 9)",
                    path.display(),
                    i + 1
                ));
            }
        }
    }
    Ok(())
}
