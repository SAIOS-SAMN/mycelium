// Law 2 — THE DIGEST GATE: a hash lives where bytes genuinely are the
// object, and nowhere else.
//
//! Identity is content (law 2, A21). The files allowed below hash bytes
//! that ARE the object — a receipt chain, a wire frame, a cold routing
//! index — and every other file in the tree is refused a digest.

use super::walker::Gate;
use std::path::Path;

pub(crate) const GATE: Gate = Gate {
    name: "digest",
    allow: &[
        ("chronometry/receipt.rs", "receipt chaining, Part IX: the bytes are the object"),
        ("network/wire.rs", "frame integrity over bytes on the wire"),
        ("network/nerve/cover.rs", "the cold routing index, until A4 replaces the roots with heights"),
    ],
    scan,
};

fn scan(path: &Path, src: &str, hits: &mut Vec<String>) {
    for (i, line) in src.lines().enumerate() {
        if line.contains("blake3::") {
            hits.push(format!("{}:{}: blake3 — identity is content (law 2)", path.display(), i + 1));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    /// The gate hits a digest taken over content, and says which law.
    /// A gate nothing exercises is a costume: this one asserts both the hit
    /// and the clean case.
    #[test]
    fn a_blake3_call_is_a_hit() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/transducer/address.rs"),
            "fn key(b: &[u8]) -> [u8; 32] {\n    blake3::hash(b).into()\n}\n",
            &mut hits,
        );
        assert_eq!(hits.len(), 1, "the call is one hit: {hits:?}");
        assert!(hits[0].contains("law 2"), "and it names the law: {}", hits[0]);
    }

    /// A file that names the crate without calling it is not a hit: the
    /// gate reads `blake3::`, the call, and not the word.
    #[test]
    fn prose_about_hashing_is_not_a_hit() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/transducer/address.rs"),
            "/// Identity is content, never a blake3 digest of it.\nfn address() {}\n",
            &mut hits,
        );
        assert!(hits.is_empty(), "prose is not a call: {hits:?}");
    }
}
