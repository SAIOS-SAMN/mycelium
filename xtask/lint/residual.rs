// Law 8, 50.9 — THE UTTERANCE GATE: no utterance path calls from_residual.
//
//! A voice speaks sentences and nothing else (law 8). `from_residual` and
//! `from_residual_framed` on the mind's side of the wire — the entity, the
//! session, the client's arms — compose a chain into a word, which is the
//! seam the ruling of 2026-09-10 severed. What remains of them there is a
//! listing key, a measure and a comprehension that reaches no court, each
//! named on the allow-list with its reason (MU-STANDARDS 4.5).

use super::walker::Gate;
use std::path::Path;

pub(crate) const GATE: Gate = Gate {
    name: "residual",
    allow: &[
        ("client/sdk/entity/utter.rs", "the served chain's own listing key (17.2), the one address a cite passes over; never uttered"),
        ("client/sdk/epigenome/comprehend.rs", "comprehension names the element it lacks (24.2), spoken to no court"),
        ("client/sdk/epigenome/dialect.rs", "the dialect's own key for a chain it holds; never uttered"),
    ],
    owed: &[],
    scan,
};

const SEAMS: [&str; 2] = ["from_residual(", "from_residual_framed("];

fn scan(path: &Path, src: &str, hits: &mut Vec<String>) {
    let p = path.to_string_lossy().replace('\\', "/");
    if !(p.contains("/client/") || p.contains("/bin/mycelium-client/")) {
        return;
    }
    for (i, line) in src.lines().enumerate() {
        for pat in SEAMS {
            if line.contains(pat) {
                hits.push(format!(
                    "{}:{}: {pat} on the utterance path — a voice speaks sentences, never a chain composed into a word (law 8, 50.9)",
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

    /// A chain composed into a word on the mind's side of the wire is a
    /// hit and names the law; the same call on the court's side is not
    /// this gate's to read.
    #[test]
    fn a_residual_composed_on_the_utterance_path_is_a_hit() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/client/sdk/entity/think.rs"),
            "let w = from_residual(&chain);\nlet f = from_residual_framed(&chain, &frame);\n",
            &mut hits,
        );
        assert_eq!(hits.len(), 2, "one per line: {hits:?}");
        assert!(hits.iter().all(|h| h.contains("law 8")), "each names the law: {hits:?}");
        let mut court = Vec::new();
        super::scan(Path::new("src/server/cognition/market.rs"), "let w = from_residual(&chain);\n", &mut court);
        assert!(court.is_empty(), "the court's side is read by other gates: {court:?}");
    }
}
