// Law 9 — THE CLOCK GATE: no kernel, court, client or binary reads a wall
// clock; the ledger is the clock.

use std::fs;
use std::path::Path;

pub(crate) fn lint_clock(root: &Path, hits: &mut Vec<String>) -> Result<(), String> {
    for d in ["transducer", "server", "client", "court"] {
        walk_clock(&root.join(d), hits)?;
    }
    walk_clock(&root.join("economy.rs"), hits)?;
    walk_clock(&root.join("physics.rs"), hits)?;
    Ok(())
}


pub(crate) fn walk_clock(path: &Path, hits: &mut Vec<String>) -> Result<(), String> {
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
            if !in_test && t.contains("#[cfg(test)]") {
                pending_test = true;
            }
            let opens = line.chars().filter(|&c| c == '{').count() as i32;
            let closes = line.chars().filter(|&c| c == '}').count() as i32;
            if pending_test {
                if opens > 0 {
                    in_test = true;
                    pending_test = false;
                    depth = opens - closes;
                    if depth <= 0 {
                        in_test = false;
                    }
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
            for pat in ["SystemTime", "Instant", "Duration"] {
                if line.contains(pat) {
                    hits.push(format!(
                        "{}:{}: {pat} in a kernel file (clock gate)",
                        path.display(),
                        i + 1
                    ));
                }
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
        walk_clock(&ent.path(), hits)?;
    }
    Ok(())
}
