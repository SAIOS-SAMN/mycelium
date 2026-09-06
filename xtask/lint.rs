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
        // Wire MTU is the byte size of one spend; physics names the number.
        if crate_dir == "network" && path.file_name().and_then(|s| s.to_str()) == Some("wire.rs") {
            lint_file(&path, hits, true)?;
        } else {
            lint_file(&path, hits, false)?;
        }
    }
    Ok(())
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
