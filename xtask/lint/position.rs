// Rule 15 — THE POSITION GATE: when unsure, hold; never choose by position.
//
//! `.first()`, `.last()` and `.nth(0)` pick a member of a carrier by where
//! it happens to sit — the same mathematics on two nodes then names two
//! objects (12.6, G4b). A wire tag byte and a FIFO drain head are the
//! readings the rule permits; every other site is debt in the `OWED` table
//! of 2026-09-10 (MU-STANDARDS 4.4), which only shrinks.

use super::walker::Gate;
use std::path::Path;

pub(crate) const GATE: Gate = Gate {
    name: "position",
    allow: &[],
    owed: &[
        ("anastome/dialogue/carrier.rs", 1),
        ("anastome/dialogue/cover.rs", 5),
        ("anastome/dialogue/demand.rs", 4),
        ("anastome/dialogue/docket.rs", 7),
        ("anastome/dialogue/equivocation.rs", 1),
        ("anastome/dialogue/fold.rs", 3),
        ("anastome/dialogue/offer.rs", 3),
        ("anastome/dialogue/percept.rs", 3),
        ("anastome/dialogue/quote.rs", 3),
        ("anastome/dialogue/receipts.rs", 3),
        ("anastome/dialogue/speech.rs", 4),
        ("anastome/dialogue/syllabus.rs", 2),
        ("src/arc.rs", 3),
        ("bin/mycelium-client/arms/feed.rs", 2),
        ("bin/mycelium-client/feed.rs", 1),
        ("bin/mycelium-client/speak.rs", 3),
        ("bin/mycelium-server/admission.rs", 1),
        ("bin/mycelium-server/exchange.rs", 1),
        ("bin/mycelium-server/judge/fold.rs", 1),
        ("bin/mycelium-server/judge/mod.rs", 2),
        ("bin/mycelium-server/main.rs", 1),
        ("chronometry/chain.rs", 6),
        ("chronometry/era.rs", 1),
        ("client/sdk/entity/quote.rs", 2),
        ("client/sdk/entity/think.rs", 1),
        ("client/sdk/entity/utter.rs", 1),
        ("client/sdk/epigenome/shed.rs", 1),
        ("client/sdk/session.rs", 1),
        ("court/verdict/replay.rs", 2),
        ("server/cognition/court/era.rs", 1),
        ("server/cognition/court/fills.rs", 1),
        ("server/cognition/court/fold.rs", 2),
        ("server/cognition/market.rs", 1),
        ("server/synthesis.rs", 3),
        ("transducer/address.rs", 1),
        ("transducer/alphabet/rotor.rs", 1),
        ("transducer/alphabet/word.rs", 4),
        ("transducer/anchor/sel.rs", 1),
        ("transducer/bubble/contemplation.rs", 1),
        ("transducer/bubble/drive.rs", 1),
        ("transducer/bubble/idea.rs", 1),
        ("transducer/bubble/merge.rs", 2),
        ("transducer/cga.rs", 1),
        ("transducer/element/letters.rs", 5),
        ("transducer/element/residual.rs", 1),
        ("transducer/family/extract.rs", 1),
        ("transducer/family/perceive.rs", 3),
        ("transducer/rewrite.rs", 2),
        ("transducer/sentence/chart.rs", 2),
        ("transducer/sentence/class.rs", 1),
    ],
    scan,
};

const POSITIONS: [&str; 3] = [".first()", ".last()", ".nth(0)"];

fn scan(path: &Path, src: &str, hits: &mut Vec<String>) {
    for (i, line) in src.lines().enumerate() {
        for pat in POSITIONS {
            if line.contains(pat) {
                hits.push(format!(
                    "{}:{}: {pat} — a member by position is a choice; read the fiber, hold every member (rule 15)",
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

    /// Each read by position is a hit and names the rule.
    #[test]
    fn a_read_by_position_is_a_hit() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/server/cognition/reduce.rs"),
            "let v = vs.first();\nlet w = vs.last();\nlet u = it.nth(0);\n",
            &mut hits,
        );
        assert_eq!(hits.len(), 3, "one per line: {hits:?}");
        assert!(hits.iter().all(|h| h.contains("rule 15")), "each names the rule: {hits:?}");
    }

    /// A keyed read is what the rule asks for and passes.
    #[test]
    fn a_keyed_read_is_clean() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/server/cognition/reduce.rs"),
            "let v = held.get(&key);\nfor (k, v) in fiber.fiber(ROOT) {}\n",
            &mut hits,
        );
        assert!(hits.is_empty(), "a keyed read chooses nothing: {hits:?}");
    }
}
