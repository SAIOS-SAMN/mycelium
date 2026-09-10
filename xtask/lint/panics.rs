// MU-STANDARDS 2.4 — THE PANIC GATE: cognition refuses, it does not abort.
//
//! No production line under `src/` unwraps, expects, or panics. A refusal is
//! a reading (`None`, `Err`, a named `REFUSE_*`), and a poisoned lock is no
//! reading at all — the act is not served. Test code is stripped by the
//! walker before this gate reads a file; the allow-list is EMPTY.

use super::walker::Gate;
use std::path::Path;

pub(crate) const GATE: Gate = Gate {
    name: "panics",
    allow: &[],
    owed: &[],
    scan,
};

/// The spellings of an abort. `.unwrap_or` and `.unwrap_or_default` are not
/// among them: they read a default, which is rule 11's question and not
/// this gate's.
const ABORTS: [&str; 6] = [".unwrap()", ".expect(", "panic!(", "unreachable!(", "todo!(", "unimplemented!("];

fn scan(path: &Path, src: &str, hits: &mut Vec<String>) {
    for (i, line) in src.lines().enumerate() {
        for pat in ABORTS {
            if line.contains(pat) {
                hits.push(format!(
                    "{}:{}: {pat} — cognition refuses, it does not abort (MU-STANDARDS 2.4)",
                    path.display(),
                    i + 1
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    /// Each spelling of an abort is a hit on its own line; the gate names
    /// the standard it enforces.
    #[test]
    fn every_abort_spelling_is_a_hit() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/server/cognition/kernel.rs"),
            "let k = kernel.lock().unwrap();\nlet v = x.expect(\"v\");\npanic!(\"no\");\nunreachable!();\ntodo!();\nunimplemented!();\n",
            &mut hits,
        );
        assert_eq!(hits.len(), 6, "one per line: {hits:?}");
        assert!(hits.iter().all(|h| h.contains("2.4")), "each names the standard: {hits:?}");
    }

    /// A refusal that reads `None`, or a default read where rule 11 permits
    /// one, is not an abort.
    #[test]
    fn a_refusal_and_a_read_default_are_clean() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/server/cognition/kernel.rs"),
            "let Ok(k) = kernel.lock() else { return None };\nlet n = bytes.get(0).copied().unwrap_or_default();\nlet m = v.unwrap_or(0);\n",
            &mut hits,
        );
        assert!(hits.is_empty(), "a refusal is a reading: {hits:?}");
    }
}
