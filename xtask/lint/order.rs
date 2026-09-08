// Law 5, rule 4 — THE ORDER GATE: a comparison of polytonic objects returns
// the reading, never a verdict.
//
//! A `bool` from four lanes is the maximal flattening: the caller cannot act
//! on WHICH lane, so every caller that needs to know recomputes what the
//! bool threw away. `fits_book` was the proof — it returned one bit and its
//! caller then called `log_room_refused`, which read the room again to find
//! the short lane.
//!
//! Two hits. A function whose parameters carry two polytonic objects and
//! whose return is `bool`. And two μ⃗ compared against each other outside
//! the three sites 12.6 sanctions: the economy's accounting, the pivot's
//! trial (a candidate that raises any lane on any pair is refused), and
//! `select_discharge` (incomparable candidates are held as a vortex).

use super::walker::Gate;
use std::path::Path;

/// Where a μ⃗ comparison is doctrine, by (file, function). 12.6: `descends`
/// certifies that a pivot did not raise any lane and the T lane decides
/// shedding; it never orders candidates.
const COMPARES: &[(&str, &str)] = &[
    ("economy/lanes.rs", "the economy's own accounting, per lane"),
    ("transducer/sentence/pivot/drive.rs", "12.6: a trial refuses a candidate that raises any lane"),
    ("transducer/sentence/pivot/snap.rs", "12.6: the vortex holds what does not dominate"),
    ("transducer/family/descent.rs", "the deflation the economy reads, per pair"),
];

/// The readings Phase A of Stage 9 still owes, each with the item that owes
/// it. The gate is green on every commit of the stage and the list IS the
/// remaining work: each item deletes its own rows, and the stage ends when
/// the list is empty. Same device as `AWAITING_SPLIT` carried through Stage
/// 8b. A row naming a file the gate would not have hit is itself a hit.
const AWAITING_READING: &[(&str, &str)] = &[
    ("transducer/sentence/class.rs", "S9.1: Rank::admits becomes Rank::moved"),
    ("court/verdict/replay.rs", "S9.2, S9.4: rank equality, and the descent clause deleted"),
    ("court/verdict/blind.rs", "S9.4: two magnitude-nonzero checks and one per-lane remainder"),
    ("economy/lanes.rs", "S9.3: descends becomes lane_moves"),
    ("economy/settle.rs", "S9.3"),
    ("transducer/family/descent.rs", "S9.3, S9.5: pairs_descend, and the pay gate becomes closure"),
    ("client/sdk/types.rs", "S9.3: residual_descends"),
    ("client/sdk/entity/utter.rs", "S9.3"),
    ("network/nerve/cover.rs", "S9.6: room_dominates returns the shortfall"),
];

pub(crate) const GATE: Gate = Gate {
    name: "order",
    allow: &[],
    scan,
};

fn scan(path: &Path, src: &str, out: &mut Vec<String>) {
    let p = path.to_string_lossy().replace('\\', "/");
    let sanctioned = COMPARES.iter().any(|(f, _)| p.ends_with(f));
    let owed = AWAITING_READING.iter().find(|(f, _)| p.ends_with(f));
    let mut mine: Vec<String> = Vec::new();
    let hits = &mut mine;
    let mut sig = String::new();
    for (i, line) in src.lines().enumerate() {
        let t = line.trim();
        // A signature may span lines; accumulate from `fn` to the arrow.
        if t.contains("fn ") {
            sig.clear();
        }
        sig.push_str(t);
        sig.push(' ');
        if t.contains("-> bool") && sig.contains("fn ") {
            let two_lanes = sig.matches("[u64; 4]").count() >= 2
                || sig.matches("Rank").count() >= 2
                || sig.matches("ClassVector").count() >= 2;
            if two_lanes {
                hits.push(format!(
                    "{}:{}: a comparison of two polytonic objects returns `bool` — return the reading (law 5)",
                    path.display(),
                    i + 1
                ));
            }
            sig.clear();
        }
        if !sanctioned && (t.contains("descends(") || t.contains("pairs_descend(")) {
            hits.push(format!(
                "{}:{}: two μ⃗ compared outside the economy and the pivot (law 5, rule 4)",
                path.display(),
                i + 1
            ));
        }
    }
    match owed {
        // The item that owes this file is named; the hits are its work.
        Some((file, item)) if !mine.is_empty() => {
            let _ = (file, item);
        }
        Some((file, item)) => out.push(format!(
            "{file}: AWAITING_READING says {item} owes it and the gate would not hit it — delete the row"
        )),
        None => out.append(&mut mine),
    }
}
