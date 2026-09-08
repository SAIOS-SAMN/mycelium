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
    ("transducer/family/descent.rs", "S9.3, S9.5: pairs_descend, and the pay gate becomes closure"),
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    /// A function taking two lane vectors and returning `bool` is a hit:
    /// the reading is the answer, never a verdict (law 5).
    #[test]
    fn a_bool_over_two_lane_vectors_is_a_hit() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/server/cognition/market.rs"),
            "fn beats(a: &[u64; 4], b: &[u64; 4]) -> bool {\n    true\n}\n",
            &mut hits,
        );
        assert_eq!(hits.len(), 1, "the signature is the hit: {hits:?}");
        assert!(hits[0].contains("law 5"), "and it names the law: {}", hits[0]);
    }

    /// The same signature split across lines is still one hit: a gate that
    /// reads one line at a time would miss every wrapped signature.
    #[test]
    fn a_multi_line_signature_is_read_whole() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/server/cognition/market.rs"),
            "fn beats(\n    a: &[u64; 4],\n    b: &[u64; 4],\n) -> bool {\n    true\n}\n",
            &mut hits,
        );
        assert_eq!(hits.len(), 1, "wrapped is not hidden: {hits:?}");
    }

    /// `descends(` outside the sanctioned sites is a hit; inside one of the
    /// four it is the doctrine and passes. `snap.rs` is sanctioned and not
    /// owed, so it is the clean half — `lanes.rs` is BOTH, and a sanctioned
    /// file that is also owed reports its own row rather than nothing.
    #[test]
    fn descends_is_a_hit_outside_the_four_sanctioned_files() {
        let src = "if descends(&after, &before) {}\n";
        let mut hits = Vec::new();
        super::scan(Path::new("src/server/cognition/book/listings.rs"), src, &mut hits);
        assert_eq!(hits.len(), 1, "outside, a scalar order over the book: {hits:?}");

        let mut sanctioned = Vec::new();
        super::scan(Path::new("src/transducer/sentence/pivot/snap.rs"), src, &mut sanctioned);
        assert!(sanctioned.is_empty(), "12.6: the vortex holds what does not dominate: {sanctioned:?}");
    }

    /// An `AWAITING_READING` row whose file the gate would NOT hit is
    /// itself a hit — the row would otherwise outlive the work it names.
    /// This is the device that ends Phase A: the stage closes when the list
    /// is empty, and a row that stops earning says so on the next run.
    #[test]
    fn an_owed_row_that_hits_nothing_is_reported() {
        let mut out = Vec::new();
        super::scan(Path::new("src/court/verdict/blind.rs"), "fn clean() {}\n", &mut out);
        assert_eq!(out.len(), 1, "the stale row is the hit: {out:?}");
        assert!(out[0].contains("delete the row"), "and says what to do: {}", out[0]);
    }
}
