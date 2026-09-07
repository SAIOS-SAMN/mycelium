// Law 2 — MAPS: a hash map keys by a digest of content, which is identity
// by digest.
//
//! The carrier is a `Keyed` trie over A21 coordinates or a Vec of
//! relations (rule 9). Nothing is exempt: there is no map anywhere in
//! `src/`.

use super::walker::Gate;
use std::path::Path;

pub(crate) const GATE: Gate = Gate {
    name: "maps",
    allow: &[],
    scan,
};

fn scan(path: &Path, src: &str, hits: &mut Vec<String>) {
    for (i, line) in src.lines().enumerate() {
        for pat in ["HashMap", "BTreeMap", "HashSet", "BTreeSet"] {
            if line.contains(pat) {
                hits.push(format!(
                    "{}:{}: {pat} — the carrier is a Keyed trie or a Vec of relations (law 2, rule 9)",
                    path.display(),
                    i + 1
                ));
            }
        }
    }
}
