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
//! whose return is `bool`. And any return of the three verbs the
//! consolidation of 2026-09-08 deleted: `descends`, `pairs_descend`,
//! `dominant`.
//!
//! There is no sanctioned site for those three any more, so there is no
//! allow-list of files here. `lane_moves` is the reading and may be called
//! anywhere; what is forbidden is a function that answers "which of these
//! two wins" (`descends`), "did every pair descend" (`pairs_descend`), or
//! "which candidate is the minimum" (`dominant`). The last one selected
//! among CERTIFIED discharges, which 12.6, 16.16 step 5, T13 and T22 now
//! forbid outright: several discharges are a vortex.

use super::walker::Gate;
use std::path::Path;

/// The verbs that no longer exist. A line naming any of them is a hit
/// wherever it appears — including the files that used to be sanctioned to
/// call them, because the sanction was for a function that is gone.
const DELETED_VERBS: &[&str] = &["descends(", "pairs_descend(", "dominant("];

/// The readings a stage still owes, each with the item that owes it. The
/// gate is green on every commit and the list IS the remaining work: each
/// item deletes its own rows, and the stage ends when the list is empty.
/// Same device as `AWAITING_SPLIT` carried through Stage 8b. A row naming a
/// file the gate would not have hit is itself a hit.
///
/// EMPTY since S9.6: Phase A closed, and every row was deleted by the build
/// refusing the tree on it the moment its work landed.
const AWAITING_READING: &[(&str, &str)] = &[];

pub(crate) const GATE: Gate = Gate {
    name: "order",
    allow: &[],
    scan,
};

fn scan(path: &Path, src: &str, out: &mut Vec<String>) {
    let p = path.to_string_lossy().replace('\\', "/");
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
        for verb in DELETED_VERBS {
            if t.contains(verb) {
                hits.push(format!(
                    "{}:{}: `{verb}` — deleted 2026-09-08; a reading of μ⃗ answers per lane and \
                     nothing chooses among certified discharges (law 5, rule 4, 12.6)",
                    path.display(),
                    i + 1
                ));
            }
        }
    }
    report_owed(owed, &mut mine, out);
}

/// What an owed file's hits mean. A file some item still owes a reading is
/// EXPECTED to hit, and its hits are that item's work, not a refusal. A file
/// whose row no longer earns is the refusal: the row would otherwise outlive
/// the work it names.
fn report_owed(
    owed: Option<&(&str, &str)>,
    mine: &mut Vec<String>,
    out: &mut Vec<String>,
) {
    match owed {
        Some((file, item)) if !mine.is_empty() => {
            let _ = (file, item);
        }
        Some((file, item)) => out.push(format!(
            "{file}: AWAITING_READING says {item} owes it and the gate would not hit it — delete the row"
        )),
        None => out.append(mine),
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

    /// Each deleted verb is a hit ANYWHERE — including `snap.rs` and
    /// `lanes.rs`, which used to be sanctioned to call them. The sanction
    /// was for functions that no longer exist: `select_discharge` compares
    /// nothing, and the economy reads per lane.
    #[test]
    fn a_deleted_verb_is_a_hit_in_every_file() {
        for (src, name) in [
            ("if descends(&after, &before) {}\n", "descends"),
            ("if pairs_descend(&b, &a) {}\n", "pairs_descend"),
            ("match dominant(vals) {}\n", "dominant"),
        ] {
            for file in [
                "src/server/cognition/book/listings.rs",
                "src/transducer/sentence/pivot/snap.rs",
                "src/economy/lanes.rs",
            ] {
                let mut hits = Vec::new();
                super::scan(Path::new(file), src, &mut hits);
                assert_eq!(hits.len(), 1, "{name} in {file} is a hit: {hits:?}");
                assert!(hits[0].contains("12.6"), "and it names the chapter: {}", hits[0]);
            }
        }
    }

    /// The reading itself is not a hit: `lane_moves` may be called
    /// anywhere, which is the point of it.
    #[test]
    fn the_reading_is_not_a_hit() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/server/cognition/book/listings.rs"),
            "let r = lane_moves(&after, &before);\nlet risen = lanes_risen(&r);\n",
            &mut hits,
        );
        assert!(hits.is_empty(), "a per-lane reading is what the law asks for: {hits:?}");
    }

    /// An `AWAITING_READING` row whose file the gate would NOT hit is
    /// itself a hit — the row would otherwise outlive the work it names.
    /// The list is empty now, so this feeds the check a row directly.
    #[test]
    fn an_owed_row_that_hits_nothing_is_reported() {
        let owed = [("court/verdict/blind.rs", "S9.4: the descent clause")];
        let mut out = Vec::new();
        super::report_owed(owed.iter().find(|(f, _)| f.ends_with("blind.rs")), &mut Vec::new(), &mut out);
        assert_eq!(out.len(), 1, "the stale row is the hit: {out:?}");
        assert!(out[0].contains("delete the row"), "and says what to do: {}", out[0]);
    }
}
