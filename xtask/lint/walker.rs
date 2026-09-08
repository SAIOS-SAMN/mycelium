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
//! Here the walk is one function over the whole of `src/`, and a gate says
//! only which FILES it does not apply to. `xtask/` is outside it: the
//! content rules are about cognition, and a gate that forbids a pattern has
//! to contain that pattern to search for it. The reading bound is the one
//! rule that covers `xtask/`, and `mod.rs` runs it separately.
//!
//! An allow row is EARNED. A row naming a file that does not exist is a
//! hit, and so is a row naming a file the gate would not have hit: a list
//! cannot outlive what it excuses, whether the file went away or the reason
//! did.

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
    // A row is EARNED, not merely present: it is kept only if the file it
    // names exists AND the gate would actually have hit it. A row that
    // excuses nothing is the same fault as a row naming a deleted file,
    // reached from the other side — it reads as a sanctioned exception and
    // sanctions nothing.
    let mut earned: Vec<(&'static str, &'static str)> = Vec::new();
    for path in files {
        if super::length::is_declared_test_file(path) {
            continue;
        }
        let src = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let production = strip_tests(&src);
        for gate in gates {
            match gate.allows(path) {
                Some(row) => {
                    let mut would = Vec::new();
                    (gate.scan)(path, &production, &mut would);
                    if !would.is_empty() {
                        earned.push((gate.name, row));
                    }
                }
                None => (gate.scan)(path, &production, hits),
            }
        }
    }
    for gate in gates {
        for (file, why) in gate.allow {
            if !earned.iter().any(|(g, f)| *g == gate.name && f == file) {
                hits.push(format!(
                    "{file}: the {} gate allows it ({why}) and would not have hit it — delete the row",
                    gate.name
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
            out.push('\n');
            continue;
        }
        if pending {
            // A DECLARATION, not a block: `#[cfg(test)] mod tests;` is the
            // shape 3.3 mandates and it carries no brace. Left armed, the
            // arm fires on the NEXT braced item in the file — production
            // code, blanked and then read by no gate. Disarm on the
            // semicolon; a test block starts only where an item opens.
            if opens == 0 && t.ends_with(';') {
                pending = false;
                out.push('\n');
                continue;
            }
            if opens > 0 {
                in_test = true;
                pending = false;
                depth = opens - closes;
                if depth <= 0 {
                    in_test = false;
                }
                out.push('\n');
                continue;
            }
            // Still inside the governed item's own attributes and
            // signature, which have not opened the body yet. Those lines
            // are the test item's too — a `#[cfg(test)]` function whose
            // parameters span lines carries its types there.
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

#[cfg(test)]
mod tests {
    use super::strip_tests;

    /// F3f: `#[cfg(test)]` on a DECLARATION disarms. Left armed, the next
    /// braced item in the file — production code — was blanked and read by
    /// no gate. Seven files carry code after that shape, the server
    /// binary's `main` with 396 lines of it.
    #[test]
    fn a_test_declaration_does_not_swallow_the_next_item() {
        let src = "#[cfg(test)]\nmod tests;\n\nfn production() {\n    let t = Instant::now();\n}\n";
        let out = strip_tests(src);
        assert!(
            out.contains("Instant"),
            "production code after `mod tests;` must reach the gates:\n{out}"
        );
    }

    /// A `#[cfg(test)]` item whose signature spans lines is test code from
    /// the attribute to the closing brace. `sentence/transport.rs`'s
    /// `apply_linear` carries a `(BigRational, BigRational)` in its
    /// parameters, three lines before its body opens.
    #[test]
    fn a_multi_line_test_signature_is_test_code() {
        let src = "#[cfg(test)]\npub fn apply_linear(\n    t: &(BigRational, BigRational),\n) -> Option<()> {\n    None\n}\n\nfn production() {\n    let ok = 1;\n}\n";
        let out = strip_tests(src);
        assert!(!out.contains("BigRational"), "the signature is test code:\n{out}");
        assert!(out.contains("let ok = 1"), "and production survives it:\n{out}");
    }

    /// And `#[cfg(test)]` on a BLOCK still strips the block whole.
    #[test]
    fn a_test_block_is_stripped_whole() {
        let src = "#[cfg(test)]\nmod tests {\n    fn hidden() {\n        let t = Instant::now();\n    }\n}\n\nfn production() {\n    let ok = 1;\n}\n";
        let out = strip_tests(src);
        assert!(!out.contains("Instant"), "the block is stripped:\n{out}");
        assert!(out.contains("let ok = 1"), "and production survives it:\n{out}");
    }
}
