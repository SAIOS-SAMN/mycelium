// Law 9 — THE CLOCK GATE: the ledger is the clock. Nothing in this tree
// reads a wall clock.
//
//! Every wait is denominated in strokes and every stamp in receipts. The
//! allow-list is EMPTY: no file in `src/` may read `SystemTime`, `Instant`
//! or `Duration`.

use super::walker::Gate;
use std::path::Path;

pub(crate) const GATE: Gate = Gate {
    name: "clock",
    allow: &[],
    owed: &[],
    scan,
};

fn scan(path: &Path, src: &str, hits: &mut Vec<String>) {
    for (i, line) in src.lines().enumerate() {
        for pat in ["SystemTime", "Instant", "Duration"] {
            if line.contains(pat) {
                hits.push(format!(
                    "{}:{}: {pat} — the ledger is the clock (law 9)",
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

    /// Each of the three wall-clock types is a hit, on its own line.
    /// F3ai: the clock gate's allow-list is empty, so nothing in `src/`
    /// exercised it and a broken scan would have read as a clean tree.
    #[test]
    fn every_wall_clock_type_is_a_hit() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/client/sdk/session.rs"),
            "let t = Instant::now();\nlet d = Duration::from_millis(5);\nlet s = SystemTime::now();\n",
            &mut hits,
        );
        assert_eq!(hits.len(), 3, "one per line: {hits:?}");
        assert!(hits.iter().all(|h| h.contains("law 9")), "each names the law: {hits:?}");
    }

    /// A wait denominated in strokes is what the law asks for and passes.
    #[test]
    fn a_stroke_wait_is_clean() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/client/sdk/session.rs"),
            "fn wait(strokes: u64) {\n    for _ in 0..strokes {}\n}\n",
            &mut hits,
        );
        assert!(hits.is_empty(), "strokes are the clock: {hits:?}");
    }
}
