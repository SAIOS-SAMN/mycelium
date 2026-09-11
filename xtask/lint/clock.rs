// Law 9 — THE CLOCK GATE: the ledger is the clock. Nothing in this tree
// reads a wall clock.
//
//! Every wait is denominated in strokes and every stamp in receipts. No
//! file in `src/` may read `SystemTime` or `Instant`, and none may name a
//! `Duration` except the one that says what a stroke costs.
//!
//! THE ONE ALLOWANCE (the stroke-duration ruling, 2026-09-11). A stroke
//! had no duration at all: `Physics::spend` is arithmetic on the reserve,
//! so every "pay a stroke and retry" was three integer decrements and
//! every `yield_now` beside one returned at once. Measured on the fleet at
//! epoch 10 — 32,767 of 32,768 exchange cycles idle, a core pinned per
//! court, every peer read `silent` because its listener had not been
//! scheduled inside the microseconds a poll budget of yields allows, and
//! therefore no round trip measured, therefore no Λ, therefore no
//! eviction, therefore the book grew until the node died.
//!
//! `physics::STROKE_FLOOR` is the floor that fixes it, and it is NOT a
//! pace: the pace is still the stroke and the measured transport. It says
//! only that one stroke of WAITING costs the processor enough for the
//! thread the wait exists for to run. It lives in `physics` because that
//! is where the quantity belongs, and the gate holds everywhere else — a
//! second `Duration` anywhere in the tree is still a wall clock.

use super::walker::Gate;
use std::path::Path;

pub(crate) const GATE: Gate = Gate {
    name: "clock",
    allow: &[],
    owed: &[("physics/mod.rs", 2)],
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
