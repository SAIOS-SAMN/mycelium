// AI-GENERATED CONTENT — see DISCLAIMER.md
//
// THE CLIENT SURFACE KEEPS THE RULINGS OF 2026-11-04/05 in its words: an
// angle is degrees on a named plane, the Concordance is signed by
// reconstruction, and a holding is a coordinate of one multi-dimensional
// ledger.
//
//! The client surface is what a third party reads first: the SDK crate
//! (`sdk/`, its sources, examples, tests and manifest), the reference mind
//! (`src/client/`, `src/bin/mycelium-client/`) and the README. Each phrase
//! below names a structure a ruling retired, so where it stands on the
//! surface it says the retired thing, in prose or in code alike, and the
//! gate reads every line, comments included.
//!
//! The phrases are exact and few, chosen so no lawful sentence holds one:
//! "a 90° rotation in the e₁e₂ plane" passes, "a quarter turn" does not.
//! History is not read: the CHANGELOG and `changes/` are outside the roots.

use std::path::{Path, PathBuf};

/// The phrases, matched without regard to case, with the ruling each
/// breaks.
const RETIRED: &[(&str, &str)] = &[
    ("quarter turn", "ruling 1 (2026-11-05): an angle is degrees on a named plane, the generator by its two axes"),
    ("quarter-turn", "ruling 1 (2026-11-05): an angle is degrees on a named plane, the generator by its two axes"),
    ("half turn", "ruling 1 (2026-11-05): an angle is degrees on a named plane, the generator by its two axes"),
    ("half-turn", "ruling 1 (2026-11-05): an angle is degrees on a named plane, the generator by its two axes"),
    ("clockwise", "ruling 1 (2026-11-05): a rotation is named by its plane's generator and its degrees, never a sense of the clock"),
    ("concordance hash", "ruling 4 (2026-11-05): the Concordance is signed clause by clause by reconstructing its fixtures, never a hash"),
    ("a wallet is flat", "ruling 2 (2026-11-05): a wallet is a reading of the holding's coordinate on one multi-dimensional ledger"),
    ("ledger row", "ruling 2 (2026-11-05): the ledger is multi-dimensional (lane × layer × grade); its events stand at coordinates, never rows"),
];

/// The files read: Rust sources, Markdown and manifests.
fn is_read(p: &Path) -> bool {
    p.extension().is_some_and(|x| x == "rs" || x == "md" || x == "toml")
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for e in entries {
        let e = e.map_err(|e| format!("{}: {e}", dir.display()))?;
        let p = e.path();
        // A build's output is no source of the surface.
        if p.is_dir() && p.file_name().is_some_and(|n| n == "target") {
            continue;
        }
        if p.is_dir() {
            walk(&p, out)?;
        } else if is_read(&p) {
            out.push(p);
        }
    }
    Ok(())
}

/// The client surface of the repository at `repo`: the SDK crate, the
/// reference mind, its binary and the README.
pub fn client_surface(repo: &Path) -> Vec<PathBuf> {
    vec![repo.join("sdk"), repo.join("src/client"), repo.join("src/bin/mycelium-client"), repo.join("README.md")]
}

/// Refuse each retired phrase on every line of the files under `roots` (a
/// root may be one file), naming the file, the line, the phrase and the
/// ruling.
pub fn lint_surface(roots: &[PathBuf], hits: &mut Vec<String>) -> Result<usize, String> {
    let mut files = Vec::new();
    for r in roots {
        if r.is_dir() {
            walk(r, &mut files)?;
        } else if r.is_file() {
            files.push(r.clone());
        }
    }
    for f in &files {
        let text = std::fs::read_to_string(f).map_err(|e| format!("{}: {e}", f.display()))?;
        for (n, line) in text.lines().enumerate() {
            let lower = line.to_lowercase();
            for (phrase, why) in RETIRED {
                if lower.contains(phrase) {
                    hits.push(format!("  {}:{}: `{phrase}` — {why}", f.display(), n + 1));
                }
            }
        }
    }
    Ok(files.len())
}
