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
    lint_maps(&root, &mut hits)?;
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
/// 800 is the one literal this tree allows, and it is allowed because it
/// measures a person and not the mathematics (rule 5 bans constants that
/// stand in for a measurement of the world; this stands in for a reading).
/// It goes to 600 in the last commit of Stage 8b (MU-STANDARDS 1.1).
///
/// A file still awaiting its split is named in `AWAITING_SPLIT` with the
/// row of Stage 8b that owes it, so the gate is green on every commit of
/// the stage and the list IS the remaining work: each split commit deletes
/// its own row, and the stage ends when the list is empty. A row whose file
/// is already under the bound, or no longer exists, is a stale row and is
/// itself a hit — the list cannot outlive what it excuses.
const AWAITING_SPLIT: &[(&str, &str)] = &[
    ("src/transducer/alphabet.rs", "S8"),
    ("src/transducer/family/mod.rs", "S9"),
    ("src/network/nerve.rs", "S10"),
    ("src/server/cognition/court.rs", "S11"),
    ("src/economy.rs", "S12"),
    ("src/client/sdk/entity.rs", "S13"),
    ("src/server/cognition/book.rs", "S14"),
    ("src/court/verdict.rs", "S15"),
    ("src/transducer/cech.rs", "S16"),
    ("src/client/sdk/mod.rs", "S17"),
    ("src/transducer/family/tests.rs", "S18"),
    ("src/client/sdk/epigenome.rs", "S19"),
    ("src/transducer/blade.rs", "S20"),
    ("src/bin/mycelium-client.rs", "S21"),
    ("src/server/cognition/germinate.rs", "S22"),
    ("src/client/sdk/tests.rs", "S23"),
    ("src/transducer/sentence/pivot.rs", "S24"),
];

fn lint_file_length(root: &Path, hits: &mut Vec<String>) -> Result<(), String> {
    const BOUND: usize = 800;
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
                    "{}: {n} lines, over the {BOUND}-line reading bound (Stage 8b) — split it",
                    path.display()
                )),
                None => {}
            }
        }
        Ok(())
    }
    let mut seen: Vec<String> = Vec::new();
    walk_len(root, hits, &mut seen)?;
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
fn is_declared_test_file(path: &Path) -> bool {
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

/// Rule 9 and checklist (g): `HashMap`, `BTreeMap` and `HashSet` do not
/// appear in cognition. A flat map keyed by a digest collapses a dimension
/// at the storage boundary, and the only operation left on a bucket of
/// points is equality lookup (ANTI_FLATTENING_LAW). There was NO gate for
/// this — the whole rule rested on review, and four transport tables had
/// already landed in `client/sdk`.
///
/// The sovereignty boundary is the one exemption and it is a directory, not
/// a judgement: `network/` frames the wire and indexes the cold store, and
/// those are I/O, not reasoning operands.
fn lint_maps(root: &Path, hits: &mut Vec<String>) -> Result<(), String> {
    for d in ["server", "client", "transducer", "court"] {
        walk_maps(&root.join(d), hits)?;
    }
    Ok(())
}

fn walk_maps(path: &Path, hits: &mut Vec<String>) -> Result<(), String> {
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
