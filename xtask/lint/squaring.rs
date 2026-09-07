// Law 11 — THE SQUARING GATE: a coordinate pair lives at the sensor
// boundary and nowhere above it.

use std::fs;
use std::path::Path;

use super::length::is_declared_test_file;

pub(crate) fn lint_squaring(root: &Path, hits: &mut Vec<String>) -> Result<(), String> {
    for d in ["transducer", "server", "client", "court"] {
        walk_squaring(&root.join(d), hits)?;
    }
    Ok(())
}


pub(crate) fn walk_squaring(path: &Path, hits: &mut Vec<String>) -> Result<(), String> {
    if path.is_file() {
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            return Ok(());
        }
        // A split-out test file carries no `#[cfg(test)]` marker for the
        // brace scan below to find, so the gate would read the whole of it
        // as production. The exemption is the same one `lint_file` uses and
        // is checked against the owning module's declaration, not the name.
        if is_declared_test_file(path) {
            return Ok(());
        }
        let src = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut in_test = false;
        let mut pending_test = false;
        let mut depth = 0i32;
        let mut fn_depth = 0i32;
        let mut allowed_fn = false;
        for (i, line) in src.lines().enumerate() {
            let t = line.trim();
            if !in_test && t.contains("#[cfg(test)]") {
                pending_test = true;
            }
            let opens = line.chars().filter(|&c| c == '{').count() as i32;
            let closes = line.chars().filter(|&c| c == '}').count() as i32;
            if t.contains("fn pair_form")
                || t.contains("fn pair_form_uncached")
                || t.contains("fn speak_rule")
                || t.contains("fn orbit(")
                || t.contains("fn cell_of")
                || t.contains("fn carry_center")
                || t.contains("fn carry_cell")
                || t.contains("fn carriage(")
                || t.contains("fn base(")
                || t.contains("fn centroid(")
                || t.contains("fn carry(")
                || t.contains("fn rotor_between")
                || t.contains("fn half_rotor")
                || t.contains("fn rotor_pair")
            {
                allowed_fn = true;
                fn_depth = 0;
            }
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
            if allowed_fn {
                fn_depth += opens - closes;
                if fn_depth <= 0 {
                    allowed_fn = false;
                }
                continue;
            }
            if t.starts_with("//") {
                continue;
            }
            let pair = t.contains("(Q, Q)")
                || t.contains("(Q,Q)")
                || t.contains("&(Q, Q)")
                || t.contains("BigRational, BigRational")
                || t.contains("BigRational, num_rational")
                || t.contains("num_rational::BigRational, num_rational::BigRational");
            // Checklist (f') names `(i64, i64)` beside the rational pairs,
            // and the gate read only the rationals. It cannot be matched by
            // TYPE -- `(i64, i64)` is the cell coordinate itself, legitimate
            // at the sensor boundary in blade.rs and chain.rs -- so it is
            // matched by ROLE, which is what (f') actually says: a pair
            // "carried as a placement, shift, or residue". A field named for
            // one of those roles and typed as a coordinate pair is a squared
            // placement whatever its components are (law 11).
            let role = t.contains("placement:") || t.contains("shift:") || t.contains("residue:");
            let int_pair = t.contains("(i64, i64)") || t.contains("(i64,i64)");
            if role && int_pair {
                hits.push(format!(
                    "{}:{}: (i64, i64) carried as a placement/shift/residue outside the sensor boundary",
                    path.display(),
                    i + 1
                ));
            }
            if pair {
                hits.push(format!(
                    "{}:{}: (Q, Q) outside pair_form/speak_rule/orbit/cell_of/carry_center/carriage",
                    path.display(),
                    i + 1
                ));
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
        walk_squaring(&ent.path(), hits)?;
    }
    Ok(())
}
