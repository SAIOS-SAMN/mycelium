// THE GATE REFUSES EUCLIDEAN GRID HEURISTICS (law 2, rule 6, rule 15, the
// Anti-Flattening Law).
//
//! A coordinate in this engine is a geometric invariant over ℚ — a
//! canonical rotor (A1), a projective chart, a lamina translation, a class
//! vector's occupied axes. It is never a bin, a cell index, or a curve
//! through a bounding volume.
//!
//! The primitives below are computer science's answers to a DIFFERENT
//! question: how to index points in a flat, axis-aligned, finite-precision
//! box. Every one of them assumes a Euclidean grid and a chosen origin,
//! and every one of them answers "where does this fall" with a POSITION
//! rather than with the object's own coordinates. Rule 15 refuses the
//! position; law 2 refuses the substitution; the Anti-Flattening Law
//! refuses the box.
//!
//! WHY A LINT AND NOT A CLIPPY RULE. `disallowed-types` names items that
//! exist — `HashMap`, `BTreeSet`, `Instant` — and those are already
//! refused in `clippy.toml`. These are shapes a writer would introduce
//! under the shapes' OWN names: a `fn morton_key`, a `struct Bbox`, a
//! `fn interleave_bits`. Nothing in the compiler knows those names, so the
//! refusal is on the vocabulary, read out of the source.
//!
//! WHAT THIS CANNOT CATCH, stated so it is not mistaken for coverage. It
//! reads names, not structure. A key that inlines a cited sub-complex
//! rather than naming its coordinate is the defect that inflated a word to
//! 46 KB, and no vocabulary check sees it — that is a property test's job
//! (`sentence/tests/properties.rs`), and the reading belongs on a running deployment.

use std::path::{Path, PathBuf};

/// The vocabulary, with the reason each is refused. A hit is reported with
/// its reason so the writer is told WHY and not merely stopped.
const REFUSED: &[(&str, &str)] = &[
    ("morton", "law 2: a Z-order curve is a position in a flat grid, not a coordinate"),
    ("z_order", "law 2: a Z-order curve is a position in a flat grid, not a coordinate"),
    ("zorder", "law 2: a Z-order curve is a position in a flat grid, not a coordinate"),
    ("hilbert_curve", "law 2: a space-filling curve indexes a box, and the manifold is not one"),
    ("interleave_bit", "rule 6: coordinates are written out, never braided into one integer"),
    ("bit_interleav", "rule 6: coordinates are written out, never braided into one integer"),
    ("bbox", "Anti-Flattening: an axis-aligned box is a Euclidean crutch over a projective chart"),
    ("bounding_box", "Anti-Flattening: an axis-aligned box is a Euclidean crutch over a projective chart"),
    ("aabb", "Anti-Flattening: an axis-aligned box is a Euclidean crutch over a projective chart"),
    ("quadtree", "rule 15: a subdivision index is a position; charts are the engine's partition"),
    ("octree", "rule 15: a subdivision index is a position; charts are the engine's partition"),
    ("kdtree", "rule 15: a subdivision index is a position; charts are the engine's partition"),
    ("kd_tree", "rule 15: a subdivision index is a position; charts are the engine's partition"),
    ("rtree", "rule 15: a subdivision index is a position; charts are the engine's partition"),
    ("r_tree", "rule 15: a subdivision index is a position; charts are the engine's partition"),
    ("spatial_hash", "law 2 and Anti-Flattening 1: a digest is transport, never a coordinate"),
    ("grid_cell", "Anti-Flattening 3: axes are fiber coordinates, not standalone cell tables"),
    ("voxel", "Anti-Flattening 5: a fixed lattice bin flattens the 4D degrees of freedom"),
];

/// A line that only NAMES the refusal — this file, and a doc comment
/// explaining why a shape is refused — is a reading and not a use. The
/// gate would otherwise refuse the sentence that states the rule.
fn is_prose(line: &str) -> bool {
    let t = line.trim_start();
    t.starts_with("//") || t.starts_with("*") || t.starts_with("#![") || t.starts_with("#[")
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for e in entries {
        let e = e.map_err(|e| format!("{}: {e}", dir.display()))?;
        let p = e.path();
        if p.is_dir() {
            walk(&p, out)?;
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
    Ok(())
}

/// Refuse the Euclidean grid vocabulary anywhere in the tree's code.
pub fn lint_euclidean(roots: &[PathBuf], hits: &mut Vec<String>) -> Result<(), String> {
    // This file states the vocabulary in order to refuse it, and the gate
    // must not refuse the statement of its own rule.
    let own = Path::new(file!()).file_name().map(std::ffi::OsString::from);
    let mut files = Vec::new();
    for r in roots {
        if r.is_dir() {
            walk(r, &mut files)?;
        }
    }
    for f in files {
        if f.file_name().map(std::ffi::OsString::from) == own {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&f) else { continue };
        for (n, line) in text.lines().enumerate() {
            if is_prose(line) {
                continue;
            }
            let lower = line.to_ascii_lowercase();
            for (word, why) in REFUSED {
                if lower.contains(word) {
                    hits.push(format!(
                        "  {}:{}: `{word}` — {why}",
                        f.display(),
                        n + 1
                    ));
                }
            }
        }
    }
    Ok(())
}
