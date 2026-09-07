// Law 9 — THE CLOCK GATE: the ledger is the clock. Nothing in this tree
// reads a wall clock.
//
//! Every wait is denominated in strokes and every stamp in receipts. The
//! allow-list is EMPTY since A9 took the client's lap telemetry: there is
//! no file in `src/` that may read `SystemTime`, `Instant` or `Duration`.

use super::walker::Gate;
use std::path::Path;

pub(crate) const GATE: Gate = Gate {
    name: "clock",
    allow: &[],
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
