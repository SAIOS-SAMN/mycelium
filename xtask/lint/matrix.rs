// Law 11, rule 16 — THE MATRIX GATE: an action is an algebra element, never
// a linear representation on squared coordinates.
//
//! What the gate looks for is the SHAPE — a rational matrix by any name —
//! and the three reads only a matrix has. `core/lattice.rs` needs no
//! exemption: A18's ℤ rank is `Vec<Vec<i128>>`, integer rows, not a
//! rational representation of an action.

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

#[cfg(test)]
mod tests {
    use std::path::Path;

    /// Each shape that makes an action a table is a hit.
    /// F3ai: `QMatrix` and `realize(` were deleted from the tree, so this
    /// gate has had nothing to hit since — the only proof it still reads is
    /// a test that feeds it the pattern.
    #[test]
    fn a_matrix_realization_is_a_hit() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/transducer/element/action.rs"),
            "struct A { m: QMatrix }\nfn f(v: Vec<Vec<Q>>) {}\nfn g() { realize(&w); }\n",
            &mut hits,
        );
        assert_eq!(hits.len(), 3, "one per line: {hits:?}");
        assert!(hits.iter().all(|h| h.contains("law 11")), "each names the law: {hits:?}");
    }

    /// A versor is the action and passes.
    #[test]
    fn a_versor_is_clean() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/transducer/element/action.rs"),
            "pub spatial: Option<Versor>,\n",
            &mut hits,
        );
        assert!(hits.is_empty(), "an algebra element is not a table: {hits:?}");
    }
}
