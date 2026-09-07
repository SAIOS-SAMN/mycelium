// MU-STANDARDS 4.2, 4.5 — ONE WALK over every source file, and a gate's
// scope stated as what it ALLOWS.
//
//! A gate used to carry its own list of directories to descend into, and
//! every walker returned `Ok` on a path that was neither a file nor a
//! directory. So when S12 and S15 turned `economy.rs` and
//! `court/verdict.rs` into directories, the digest gate stopped covering
//! either and the clock gate stopped covering the economy, silently, and a
//! green run meant nothing about them.
//!
//! Here the walk is one function over the whole of `src/` and `xtask/`, and
//! a gate says only which FILES it does not apply to. A named allow-list
//! entry that matches no walked file is itself a hit: the list cannot
//! outlive the file it excuses.

use std::fs;
use std::path::{Path, PathBuf};

/// One rule, its allow-list, and the scan it runs on a file's source.
pub(crate) struct Gate {
    /// What the rule is called in a hit line.
    pub(crate) name: &'static str,
    /// Path suffixes this gate does not apply to, each with the reason it
    /// is exempt. Matched against the walked path, so `network/wire.rs`
    /// names one file and never a directory.
    pub(crate) allow: &'static [(&'static str, &'static str)],
    /// Read one file's production source and push a hit per offending line.
    pub(crate) scan: fn(&Path, &str, &mut Vec<String>),
}

impl Gate {
    fn allows(&self, path: &Path) -> Option<&'static str> {
        let p = path.to_string_lossy().replace('\\', "/");
        self.allow.iter().find(|(f, _)| p.ends_with(f)).map(|(f, _)| *f)
    }
}

/// Every `.rs` file under the roots, in walk order.
pub(crate) fn collect(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let rd = match fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(_) => return Ok(()),
    };
    let mut entries: Vec<PathBuf> = Vec::new();
    for ent in rd {
        entries.push(ent.map_err(|e| e.to_string())?.path());
    }
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect(&path, out)?;
            continue;
        }
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(path);
        }
    }
    Ok(())
}

/// Run every gate over every file. A file the owning module declares under
/// `#[cfg(test)]` is test code in whole and no gate reads it (3.3). A gate's
/// allow-list row that matched nothing is reported.
pub(crate) fn run_gates(
    files: &[PathBuf],
    gates: &[Gate],
    hits: &mut Vec<String>,
) -> Result<(), String> {
    let mut matched: Vec<(&'static str, &'static str)> = Vec::new();
    for path in files {
        if super::length::is_declared_test_file(path) {
            continue;
        }
        let src = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let production = strip_tests(&src);
        for gate in gates {
            match gate.allows(path) {
                Some(row) => matched.push((gate.name, row)),
                None => (gate.scan)(path, &production, hits),
            }
        }
    }
    for gate in gates {
        for (file, why) in gate.allow {
            if !matched.iter().any(|(g, f)| *g == gate.name && f == file) {
                hits.push(format!(
                    "{}: allowed by the {} gate ({why}) and not found — delete the row",
                    file, gate.name
                ));
            }
        }
    }
    Ok(())
}

/// The production half of a file: every line outside a `#[cfg(test)]` block,
/// blanked rather than removed so a hit's line number is the file's own.
fn strip_tests(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut in_test = false;
    let mut pending = false;
    let mut depth = 0i32;
    for line in src.lines() {
        let t = line.trim();
        let opens = line.chars().filter(|&c| c == '{').count() as i32;
        let closes = line.chars().filter(|&c| c == '}').count() as i32;
        if !in_test && t.contains("#[cfg(test)]") {
            pending = true;
        }
        if pending && opens > 0 {
            in_test = true;
            pending = false;
            depth = opens - closes;
            if depth <= 0 {
                in_test = false;
            }
            out.push('\n');
            continue;
        }
        if in_test {
            depth += opens - closes;
            if depth <= 0 {
                in_test = false;
            }
            out.push('\n');
            continue;
        }
        if t.starts_with("//") {
            out.push('\n');
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}
