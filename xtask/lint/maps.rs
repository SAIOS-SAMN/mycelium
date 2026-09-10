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
    owed: &[],
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    /// The four std carriers are hits; the trie is not.
    /// F3ai: this gate's allow-list is empty too, so a green run said
    /// nothing about whether the scan still worked.
    #[test]
    fn a_std_map_is_a_hit_and_a_keyed_trie_is_not() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/server/cognition/kernel.rs"),
            "let a: HashMap<u8, u8> = HashMap::new();\nlet b: BTreeSet<u8> = BTreeSet::new();\n",
            &mut hits,
        );
        assert_eq!(hits.len(), 2, "one per line: {hits:?}");
        assert!(hits[0].contains("Keyed trie"), "the hit says what to use: {}", hits[0]);

        let mut clean = Vec::new();
        super::scan(
            Path::new("src/server/cognition/kernel.rs"),
            "let held: Keyed<Element> = Keyed::new();\n",
            &mut clean,
        );
        assert!(clean.is_empty(), "the trie is the carrier: {clean:?}");
    }
}
