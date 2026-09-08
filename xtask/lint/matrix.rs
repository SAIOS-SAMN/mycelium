// Law 11, rule 16 — THE MATRIX GATE: an action is an algebra element, never
// a linear representation on squared coordinates.
//
//! `core::engine` and `core::matrix` are gone (F2f), so a pattern naming
//! either can never fire again. What remains is the SHAPE — a rational
//! matrix by any name — and the three reads only a matrix has, which are
//! the ones `element/tests/18.rs` used to check over `element/` alone.
//! `core/zlinalg.rs` needs no exemption: A18's ℤ rank is `Vec<Vec<i128>>`,
//! integer rows, not a rational representation of an action.

use super::walker::Gate;
use std::path::Path;

pub(crate) const GATE: Gate = Gate {
    name: "matrix",
    allow: &[],
    scan,
};

fn scan(path: &Path, src: &str, hits: &mut Vec<String>) {
    for (i, line) in src.lines().enumerate() {
        for pat in ["QMatrix", "Vec<Vec<Q>>", "fibers: Vec", "as_operator", "realize("] {
            if line.contains(pat) {
                hits.push(format!(
                    "{}:{}: {pat} — an action is an algebra element (law 11, rule 16)",
                    path.display(),
                    i + 1
                ));
            }
        }
    }
}
