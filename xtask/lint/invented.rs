// Rule 11 — THE INVENTED-VALUE GATE: exact or refused.
//
//! `unwrap_or`, `unwrap_or_else` and `unwrap_or_default` read a value that
//! was not there; `saturating_sub` and `saturating_mul` clamp one that
//! overflowed. Each invents. A lane sum (`saturating_add`) is accounting
//! and is not matched; a missing value is a typed refusal or `Unmeasured`.
//! The `OWED` table is the debt of 2026-09-10 (MU-STANDARDS 4.4).

use super::walker::Gate;
use std::path::Path;

pub(crate) const GATE: Gate = Gate {
    name: "invented",
    allow: &[],
    owed: &[
        ("anastome/dialogue/carrier.rs", 3),
        ("anastome/dialogue/cover.rs", 1),
        ("anastome/dialogue/docket.rs", 6),
        ("anastome/dialogue/fold.rs", 2),
        ("anastome/dialogue/receipts.rs", 3),
        ("anastome/dialogue/speech.rs", 2),
        ("anastome/dialogue/syllabus.rs", 2),
        ("src/arc.rs", 2),
        ("bin/mycelium-client/feed.rs", 2),
        ("bin/mycelium-client/main.rs", 3),
        ("bin/mycelium-client/speak.rs", 1),
        ("bin/mycelium-server/admission.rs", 1),
        ("bin/mycelium-server/backfill.rs", 1),
        ("bin/mycelium-server/door.rs", 4),
        ("bin/mycelium-server/exchange.rs", 4),
        ("bin/mycelium-server/genesis.rs", 3),
        ("bin/mycelium-server/judge/quote.rs", 1),
        ("bin/mycelium-server/judge/receipts.rs", 1),
        ("bin/mycelium-server/judge/rows.rs", 1),
        ("bin/mycelium-server/judge/syllabus.rs", 3),
        ("bin/mycelium-server/main.rs", 5),
        ("bin/mycelium-server/offer.rs", 1),
        ("bin/mycelium-server/transport.rs", 4),
        ("chronometry/chain.rs", 6),
        ("chronometry/era.rs", 1),
        ("client/sdk/entity/boot.rs", 1),
        ("client/sdk/entity/learn.rs", 5),
        ("client/sdk/entity/quote.rs", 5),
        ("client/sdk/entity/utter.rs", 2),
        ("client/sdk/epigenome/comprehend.rs", 3),
        ("client/sdk/epigenome/dialect.rs", 1),
        ("client/sdk/epigenome/persist.rs", 2),
        ("client/sdk/epigenome/shed.rs", 2),
        ("client/sdk/session.rs", 10),
        ("court/admission/admit.rs", 1),
        ("court/convergence.rs", 2),
        ("court/verdict/blind.rs", 3),
        ("court/verdict/replay.rs", 1),
        ("economy/escrow.rs", 2),
        ("economy/fold.rs", 1),
        ("economy/lanes.rs", 1),
        ("economy/persist.rs", 6),
        ("economy/treasury.rs", 8),
        ("network/mesh.rs", 5),
        ("network/nerve/cover.rs", 5),
        ("network/nerve/keyed/relation.rs", 1),
        ("network/nerve/keyed/trie.rs", 3),
        ("network/wire.rs", 1),
        ("src/physics.rs", 11),
        ("server/cognition/basis.rs", 8),
        ("server/cognition/book/language.rs", 1),
        ("server/cognition/book/listings.rs", 4),
        ("server/cognition/book/speech.rs", 4),
        ("server/cognition/book/tension.rs", 3),
        ("server/cognition/court/fills.rs", 1),
        ("server/cognition/court/glue.rs", 2),
        ("server/cognition/court/lapse.rs", 1),
        ("server/cognition/court/phase.rs", 4),
        ("server/cognition/court/replay.rs", 4),
        ("server/cognition/court/task.rs", 1),
        ("server/cognition/germinate/state.rs", 1),
        ("server/cognition/kernel.rs", 1),
        ("server/cognition/market.rs", 3),
        ("server/cognition/reduce.rs", 1),
        ("server/cognition/room.rs", 8),
        ("server/cognition/snapshot/read.rs", 12),
        ("server/cognition/snapshot/write.rs", 1),
        ("server/cognition/wire.rs", 2),
        ("server/registry.rs", 2),
        ("transducer/address.rs", 2),
        ("transducer/alphabet/plane.rs", 1),
        ("transducer/alphabet/rotor.rs", 1),
        ("transducer/anchor/sel.rs", 5),
        ("transducer/beam.rs", 1),
        ("transducer/blade/body.rs", 2),
        ("transducer/blade/shells.rs", 1),
        ("transducer/bubble/drive.rs", 2),
        ("transducer/bubble/idea.rs", 2),
        ("transducer/bubble/merge.rs", 5),
        ("transducer/bubble/shed.rs", 1),
        ("transducer/bubble/tape.rs", 1),
        ("transducer/cech/cell.rs", 1),
        ("transducer/cech/complex.rs", 1),
        ("transducer/cech/tape.rs", 1),
        ("transducer/element/carrier.rs", 2),
        ("transducer/element/gauge.rs", 2),
        ("transducer/element/residual.rs", 5),
        ("transducer/element/sensor.rs", 3),
        ("transducer/element/tape.rs", 2),
        ("transducer/element/vertex.rs", 4),
        ("transducer/family/descent.rs", 2),
        ("transducer/family/extract.rs", 2),
        ("transducer/family/orbit.rs", 1),
        ("transducer/family/pair.rs", 1),
        ("transducer/family/perceive.rs", 1),
        ("transducer/family/speak.rs", 3),
        ("transducer/peel.rs", 1),
        ("transducer/rewrite.rs", 8),
        ("transducer/sentence/demand.rs", 1),
        ("transducer/sentence/form/keys.rs", 1),
        ("transducer/sentence/pivot/snap.rs", 1),
        ("transducer/sentence/tape.rs", 1),
        ("transducer/stalk/chart.rs", 2),
        ("transducer/stalk/stalks.rs", 1),
        ("transducer/stalk/stratum.rs", 3),
        ("transducer/torsion_witness.rs", 2),
    ],
    scan,
};

const INVENTIONS: [&str; 5] = [
    ".unwrap_or(",
    ".unwrap_or_else(",
    ".unwrap_or_default(",
    ".saturating_sub(",
    ".saturating_mul(",
];

fn scan(path: &Path, src: &str, hits: &mut Vec<String>) {
    for (i, line) in src.lines().enumerate() {
        for pat in INVENTIONS {
            if line.contains(pat) {
                hits.push(format!(
                    "{}:{}: {pat} — exact or refused; a missing value is a refusal, never a default (rule 11)",
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

    /// Each spelling of an invented value is a hit and names the rule.
    #[test]
    fn an_invented_value_is_a_hit() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/server/cognition/reduce.rs"),
            "let k = kb.unwrap_or(0);\nlet g = inv.unwrap_or_else(Rotor::identity);\nlet t = now.saturating_sub(k);\n",
            &mut hits,
        );
        assert_eq!(hits.len(), 3, "one per line: {hits:?}");
        assert!(hits.iter().all(|h| h.contains("rule 11")), "each names the rule: {hits:?}");
    }

    /// A refusal and a lane sum pass: the one propagates, the other is
    /// accounting.
    #[test]
    fn a_refusal_and_a_lane_sum_are_clean() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/server/cognition/reduce.rs"),
            "let Some(k) = kb else { return None };\nv[l] = v[l].saturating_add(add[l]);\n",
            &mut hits,
        );
        assert!(hits.is_empty(), "a refusal invents nothing: {hits:?}");
    }
}
