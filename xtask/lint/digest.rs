// Law 2 — THE DIGEST GATE: blake3 lives where bytes genuinely are the
// object, and nowhere else.

use std::fs;
use std::path::Path;

pub(crate) fn lint_blake3(root: &Path, hits: &mut Vec<String>) -> Result<(), String> {
    let deny = [
        "server",
        "client",
        "transducer",
        "anastome",
        "voice",
        "thought",
        "perception",
        "economy.rs",
        "bin",
    ];
    for d in deny {
        walk_blake3(&root.join(d), hits)?;
    }
    walk_blake3(&root.join("court").join("verdict.rs"), hits)?;
    walk_blake3(&root.join("court").join("ledger.rs"), hits)?;
    Ok(())
}

/// The Element's action is a versor. QMatrix, Delta, Operator, and
/// core::matrix do not enter cognition.
pub(crate) fn walk_blake3(path: &Path, hits: &mut Vec<String>) -> Result<(), String> {
    if path.is_file() {
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
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
            if line.contains("blake3::") {
                hits.push(format!("{}:{}: blake3 in cognition", path.display(), i + 1));
            }
        }
        return Ok(());
    }
    if !path.is_dir() {
        return Ok(());
    }
    let rd = match fs::read_dir(path) {
        Ok(rd) => rd,
        Err(_) => return Ok(()),
    };
    for ent in rd {
        let ent = ent.map_err(|e| e.to_string())?;
        walk_blake3(&ent.path(), hits)?;
    }
    Ok(())
}
