// Rule 4 — THE CHOICE GATE: nothing is chosen by a scalar.
//
//! `min_by`, `max_by`, `sort_by`, `sort_unstable_by`, their `_key` forms
//! and an iterator's `.max()`/`.min()` order a candidate set by a number;
//! the doctrine keeps every certified candidate and lets a perception, a
//! glue or a ruling collapse them (rule 15). A canonical ordering by key is
//! an ordering, not a choice, and a coset's representative is a gauge
//! (50.2) — each is written as a fold on content, never as one of these.
//! The `OWED` table is the debt of 2026-09-10 (MU-STANDARDS 4.4).

use super::walker::Gate;
use std::path::Path;

pub(crate) const GATE: Gate = Gate {
    name: "choice",
    allow: &[],
    owed: &[
        ("bin/mycelium-server/transport.rs", 1),
        ("client/sdk/entity/learn.rs", 1),
        ("client/sdk/epigenome/atlas.rs", 1),
        ("client/sdk/epigenome/dialect.rs", 1),
        ("client/sdk/session.rs", 2),
        ("court/verdict/blind.rs", 2),
        ("court/verdict/selection.rs", 1),
        ("server/cognition/basis.rs", 1),
        ("server/cognition/book/listings.rs", 1),
        ("server/cognition/germinate/stroke.rs", 2),
        ("server/cognition/wire.rs", 2),
        ("transducer/anchor/derive.rs", 1),
        ("transducer/anchor/sel.rs", 12),
        ("transducer/beam.rs", 1),
        ("transducer/blade/body.rs", 1),
        ("transducer/blade/shells.rs", 6),
        ("transducer/element/carrier.rs", 2),
        ("transducer/element/residual.rs", 4),
        ("transducer/element/vertex.rs", 1),
        ("transducer/sentence/chain.rs", 1),
    ],
    scan,
};

const CHOICES: [&str; 10] = [
    ".min_by(",
    ".max_by(",
    ".min_by_key(",
    ".max_by_key(",
    ".sort_by(",
    ".sort_by_key(",
    ".sort_unstable_by(",
    ".sort_unstable_by_key(",
    ".max()",
    ".min()",
];

fn scan(path: &Path, src: &str, hits: &mut Vec<String>) {
    for (i, line) in src.lines().enumerate() {
        for pat in CHOICES {
            if line.contains(pat) {
                hits.push(format!(
                    "{}:{}: {pat} — nothing is chosen by a scalar; keep every certified candidate (rule 4)",
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

    /// Each ordering verb over a candidate set is a hit and names the rule.
    #[test]
    fn a_choice_by_scalar_is_a_hit() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/server/cognition/reduce.rs"),
            "let b = cands.iter().min_by_key(|c| c.cost);\nchords.sort_by_key(order);\nlet m = xs.iter().max();\n",
            &mut hits,
        );
        assert_eq!(hits.len(), 3, "one per line: {hits:?}");
        assert!(hits.iter().all(|h| h.contains("rule 4")), "each names the rule: {hits:?}");
    }

    /// A sort with no key is a canonical ordering of content, and a
    /// two-argument `max` is an accounting bound: neither is a choice.
    #[test]
    fn a_canonical_sort_and_an_accounting_bound_are_clean() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/server/cognition/reduce.rs"),
            "cells.sort_unstable();\nlet n = a.max(b);\n",
            &mut hits,
        );
        assert!(hits.is_empty(), "an ordering is not a choice: {hits:?}");
    }
}
