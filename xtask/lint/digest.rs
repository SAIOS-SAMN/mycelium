// Law 2 — THE DIGEST GATE: a hash lives where bytes genuinely are the
// object, and nowhere else.
//
//! Identity is content (law 2, A21). The four files below hash bytes that
//! ARE the object — a receipt chain, a wire frame, a cold routing index
//! below the sovereignty boundary, a key derivation — and every other file
//! in the tree is refused a digest.

use super::walker::Gate;
use std::path::Path;

pub(crate) const GATE: Gate = Gate {
    name: "digest",
    allow: &[
        ("chronometry.rs", "receipt chaining, Part IX: the bytes are the object"),
        ("court/identity.rs", "key derivation, not identity of content"),
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
