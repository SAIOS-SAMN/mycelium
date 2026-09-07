// AI-GENERATED CONTENT — see DISCLAIMER.md.
//! Enumeration lint: in the cognition directories a Keyed is not iterable.
//! Hits of the old walk surface or of spend_unit as a number fail the build.

use std::fs;
use std::path::{Path, PathBuf};

pub fn run() -> Result<(), String> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = if manifest.join("src/server").is_dir() {
        manifest.join("src")
    } else {
        manifest
            .parent()
            .ok_or("xtask has no parent")?
            .join("src")
    };
    let dirs = ["server", "client", "transducer", "network"];
    let mut hits = Vec::new();
    for d in dirs {
        walk(&root.join(d), d, &mut hits)?;
    }
    lint_file_length(&root, &mut hits)?;
    lint_blake3(&root, &mut hits)?;
    lint_squaring(&root, &mut hits)?;
    lint_qmatrix(&root, &mut hits)?;
    lint_clock(&root, &mut hits)?;
    if hits.is_empty() {
        return Ok(());
    }
    let mut msg = String::from("lint-enumeration: a loop without a keyed source:\n");
    for h in &hits {
        msg.push_str(h);
        msg.push('\n');
    }
    Err(msg)
}

fn walk(dir: &Path, crate_dir: &str, hits: &mut Vec<String>) -> Result<(), String> {
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
        // depth, so a test module moved to its own file under Stage 8's
        // reading bound carries no marker and every gate fires on it. The
        // exemption is checked, not assumed: the parent module must declare
        // this file under `#[cfg(test)]`, so a file merely NAMED tests.rs
        // is still linted.
        if path.file_name().and_then(|s| s.to_str()) == Some("tests.rs")
            && parent_declares_test_module(&path)
        {
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

/// Stage 8's reading bound. Part VII's rule of verification is that the
/// auditor reads the function and its carriers END TO END, and grep is not
/// verification; a file no one reads whole is a file whose faults are found
/// by soak instead of by review. Every carrier fault of the G-set was
/// invisible at 8000 lines and obvious at the function.
///
/// 2500 is the one literal this tree allows, and it is allowed because it
/// measures a person and not the mathematics (rule 5 bans constants that
/// stand in for a measurement of the world; this stands in for a reading).
/// A split moves code and changes nothing else, so the cost of obeying it
/// is a commit whose suite counts are byte-identical.
fn lint_file_length(root: &Path, hits: &mut Vec<String>) -> Result<(), String> {
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
                    "{}: {n} lines, over the {BOUND}-line reading bound (Stage 8) — split it",
                    path.display()
                ));
            }
        }
        Ok(())
    }
    walk_len(root, hits)
}

/// Does the module owning `tests.rs` declare it under `#[cfg(test)]`? The
/// declaration is the only place the attribute can live for a file module,
/// so this reads the fact rather than trusting the name.
fn parent_declares_test_module(path: &Path) -> bool {
    let Some(dir) = path.parent() else {
        return false;
    };
    let owner = dir.join("mod.rs");
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

fn lint_file(path: &Path, hits: &mut Vec<String>, allow_spend: bool) -> Result<(), String> {
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

/// A digest of mathematical structure is not a key. blake3 is allowed as a
/// KDF over a name (court/identity), a transmission digest of served bytes
/// (network cover roots, chronometry receipts, frame integrity), and nowhere
/// in cognition.
fn lint_blake3(root: &Path, hits: &mut Vec<String>) -> Result<(), String> {
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
fn lint_qmatrix(root: &Path, hits: &mut Vec<String>) -> Result<(), String> {
    for d in ["transducer", "server", "client", "court"] {
        walk_qmatrix(&root.join(d), hits)?;
    }
    Ok(())
}

/// No kernel reads a wall clock (45.4). SystemTime, Instant, and Duration
/// are refused in server, client, court, transducer, economy, physics.
fn lint_clock(root: &Path, hits: &mut Vec<String>) -> Result<(), String> {
    for d in ["transducer", "server", "client", "court"] {
        walk_clock(&root.join(d), hits)?;
    }
    walk_clock(&root.join("economy.rs"), hits)?;
    walk_clock(&root.join("physics.rs"), hits)?;
    Ok(())
}

fn walk_clock(path: &Path, hits: &mut Vec<String>) -> Result<(), String> {
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

fn walk_qmatrix(path: &Path, hits: &mut Vec<String>) -> Result<(), String> {
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
            for pat in [
                "QMatrix",
                "core::engine::Delta",
                "core::engine::Operator",
                "core::matrix::",
                "use crate::core::matrix",
                "use crate::core::engine::{Delta",
                "use crate::core::engine::{Operator",
            ] {
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

/// Any (BigRational, BigRational) or (Q, Q) lives only in pair_form,
/// speak_rule, orbit, and the sensor read-outs cell_of, carry_center,
/// carriage(rho, t). The conformal model (cga::base) and the blade's
/// moment/carry are the sensor itself (law 11).
fn lint_squaring(root: &Path, hits: &mut Vec<String>) -> Result<(), String> {
    for d in ["transducer", "server", "client", "court"] {
        walk_squaring(&root.join(d), hits)?;
    }
    Ok(())
}

fn walk_squaring(path: &Path, hits: &mut Vec<String>) -> Result<(), String> {
    if path.is_file() {
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
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

fn walk_blake3(path: &Path, hits: &mut Vec<String>) -> Result<(), String> {
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
