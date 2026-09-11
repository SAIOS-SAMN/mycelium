// Rule 2 — THE COUNT GATE: a count is never a branch.
//
//! A decision that reads `.len()` or `.count()` against a number inside an
//! `if`, a `while` or a match guard is a boolean gate on a count, and the
//! count measures nothing. Emptiness of a fiber (`is_empty()`) is the
//! permitted reading and is not matched. The `OWED` table is the debt the
//! tree carried on 2026-09-10 (MU-STANDARDS 4.4) and only shrinks.

use super::walker::Gate;
use std::path::Path;

pub(crate) const GATE: Gate = Gate {
    name: "count",
    allow: &[],
    owed: &[
        ("anastome/dialogue/carrier.rs", 3),
        ("anastome/dialogue/cover.rs", 3),
        ("anastome/dialogue/docket.rs", 12),
        ("anastome/dialogue/equivocation.rs", 1),
        ("anastome/dialogue/fold.rs", 4),
        ("anastome/dialogue/percept.rs", 1),
        ("anastome/dialogue/quote.rs", 2),
        ("anastome/dialogue/receipts.rs", 7),
        ("anastome/dialogue/speech.rs", 2),
        ("anastome/dialogue/syllabus.rs", 4),
        ("src/arc.rs", 1),
        ("bin/mycelium-client/feed.rs", 1),
        ("bin/mycelium-client/main.rs", 1),
        ("bin/mycelium-server/admission.rs", 1),
        ("bin/mycelium-server/exchange.rs", 1),
        ("bin/mycelium-server/main.rs", 1),
        ("bin/mycelium-server/transport.rs", 2),
        ("chronometry/era.rs", 1),
        ("client/sdk/entity/learn.rs", 2),
        ("client/sdk/entity/quote.rs", 1),
        ("client/sdk/epigenome/comprehend.rs", 4),
        ("client/sdk/epigenome/persist.rs", 3),
        ("client/sdk/epigenome/shed.rs", 2),
        ("client/sdk/session.rs", 1),
        ("core/lattice.rs", 2),
        ("court/admission/prefix.rs", 1),
        ("court/identity.rs", 2),
        ("court/ledger.rs", 1),
        ("court/verdict/ruling.rs", 1),
        ("economy/persist.rs", 6),
        ("economy/settle.rs", 1),
        ("network/nerve/keyed/trie.rs", 3),
        ("network/nerve/router.rs", 1),
        ("network/wire.rs", 3),
        ("server/cognition/book/language.rs", 2),
        ("server/cognition/court/era.rs", 3),
        ("server/cognition/court/glue.rs", 1),
        ("server/cognition/court/phase.rs", 1),
        ("server/cognition/court/syllabus.rs", 1),
        ("server/cognition/market.rs", 1),
        ("server/cognition/reduce.rs", 2),
        ("server/cognition/snapshot/read.rs", 11),
        ("transducer/address.rs", 2),
        ("transducer/alphabet/word.rs", 2),
        ("transducer/beam.rs", 2),
        ("transducer/bubble/contemplation.rs", 1),
        ("transducer/bubble/merge.rs", 3),
        ("transducer/bubble/shed.rs", 1),
        ("transducer/bubble/tape.rs", 6),
        ("transducer/cech/coboundary.rs", 2),
        ("transducer/cech/complex.rs", 4),
        ("transducer/element/carrier.rs", 1),
        ("transducer/element/gauge.rs", 2),
        ("transducer/element/letters.rs", 3),
        ("transducer/element/vertex.rs", 1),
        ("transducer/family/descent.rs", 1),
        ("transducer/family/speak.rs", 4),
        ("transducer/rewrite.rs", 1),
        ("transducer/sentence/chart.rs", 3),
        ("transducer/sentence/demand.rs", 1),
        ("transducer/sentence/form/carrier.rs", 1),
        ("transducer/sheaf.rs", 1),
    ],
    scan,
};

const COMPARISONS: [&str; 6] = [" < ", " > ", " == ", " != ", " >= ", " <= "];

fn scan(path: &Path, src: &str, hits: &mut Vec<String>) {
    for (i, line) in src.lines().enumerate() {
        let t = line.trim();
        let counted = t.contains(".len()") || t.contains(".count()");
        if !counted {
            continue;
        }
        let branch = t.starts_with("if ")
            || t.starts_with("while ")
            || t.contains(" if ")
            || t.contains("matches!(")
            || t.contains("=> ");
        let compared = COMPARISONS.iter().any(|op| t.contains(op));
        if branch && compared {
            hits.push(format!(
                "{}:{}: a count decides a branch — the fiber decides, presence is read from a keyed set (rule 2)",
                path.display(),
                i + 1
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    /// A length compared inside an `if`, a `while` or a match guard is a
    /// hit; each names the rule.
    #[test]
    fn a_count_that_decides_a_branch_is_a_hit() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/server/cognition/basis.rs"),
            "if words.len() >= 2 {\n}\nwhile i < runs.len() {\n}\nSome(x) if x.count() == n => {}\n",
            &mut hits,
        );
        assert_eq!(hits.len(), 3, "one per line: {hits:?}");
        assert!(hits.iter().all(|h| h.contains("rule 2")), "each names the rule: {hits:?}");
    }

    /// Emptiness of a fiber is the permitted reading; a length read into a
    /// carrier size, not a branch, passes too.
    #[test]
    fn emptiness_and_a_carrier_size_are_clean() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/server/cognition/basis.rs"),
            "if fiber.is_empty() {\n}\nlet n = words.len() as u16;\nout.push(cells.len() as u8);\n",
            &mut hits,
        );
        assert!(hits.is_empty(), "emptiness is a reading, a size is a size: {hits:?}");
    }
}
