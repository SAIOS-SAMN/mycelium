// Law 11, rule 16 — THE MATRIX GATE: an action is an algebra element and
// never a linear representation on squared coordinates.

use std::fs;
use std::path::Path;

pub(crate) fn lint_qmatrix(root: &Path, hits: &mut Vec<String>) -> Result<(), String> {
    for d in ["transducer", "server", "client", "court"] {
        walk_qmatrix(&root.join(d), hits)?;
    }
    Ok(())
}

/// No kernel reads a wall clock (45.4). SystemTime, Instant, and Duration
/// are refused in server, client, court, transducer, economy, physics.
pub(crate) fn walk_qmatrix(path: &Path, hits: &mut Vec<String>) -> Result<(), String> {
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
            // `core::engine` and `core::matrix` are gone (F2f): `core`
            // exports `Q` and nothing else, so a pattern naming either is a
            // row that can never fire again. What remains is the SHAPE — a
            // rational matrix by any name, and the two reads that only a
            // matrix has.
            for pat in ["QMatrix", "Vec<Vec<Q>>", "as_operator", "realize("] {
                if line.contains(pat) {
                    hits.push(format!("{}:{}: {pat} in cognition", path.display(), i + 1));
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
        walk_qmatrix(&ent.path(), hits)?;
    }
    Ok(())
}
