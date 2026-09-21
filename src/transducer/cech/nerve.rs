// 13, 54.7 — THE NERVE OF A COVER, derived by WALKING THE BASE.
//
//! A simplex of the nerve is not a fact about the meet graph; it is a fact
//! about the base. `N(𝒰)` has a `k`-simplex `[i₀ … i_k]` exactly where
//! `U_{i₀} ∩ … ∩ U_{i_k} ≠ ∅`, and the only honest proof of that is a
//! WITNESS — a cell of the base standing in all of them at once.

use super::*;
use crate::network::nerve::keyed::coord;

/// A 2-SIMPLEX OF THE COVER, with the cell that certifies it.
///
/// `witness` is not decoration. It is the whole difference between this
/// simplex and a hole: three charts that pairwise meet can ring a pore
/// none of them covers, and a triangle filled there destroys a real `H₁`
/// class. The witness is the cell that proves the triple overlap is
/// inhabited, and it is carried so a reader can check the claim rather
/// than trust it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Certified {
    /// The three charts, in the cover's own order.
    pub charts: [usize; 3],
    /// A cell standing in all three.
    pub witness: coord::Lamina,
}

/// THE SUPPORT OF A COVER, READ FROM THE BASE: `p ↦ charts(p)`.
///
/// One linear pass over every cell of every chart, `O(cells · valence)`.
/// The charts standing over a cell are kept in the cover's own order and
/// deduplicated as they arrive, so a chart naming one cell twice counts
/// once and the reading does not depend on the order the cells were
/// walked.
///
/// Keyed by [`coord::Lamina`], which is a coordinate of the manifold: the
/// carrier is a trie over the cell's own bytes, not a digest of it, so a
/// prefix is a coarser stratum and nothing is lost to a hash (rule 9).
pub fn support_of_cover(
    charts: &[Vec<coord::Lamina>],
) -> crate::network::nerve::Keyed<Vec<usize>, coord::Lamina> {
    let mut support: crate::network::nerve::Keyed<Vec<usize>, coord::Lamina> =
        crate::network::nerve::Keyed::new();
    for (ix, cells) in charts.iter().enumerate() {
        for p in cells {
            match support.at_mut(p) {
                Some(standing) => {
                    if !standing.contains(&ix) {
                        standing.push(ix);
                    }
                }
                None => {
                    support.put(p, vec![ix]);
                }
            }
        }
    }
    support
}

/// THE CERTIFIED 2-SIMPLICES OF A COVER.
///
/// Every cell whose support names three or more charts contributes the
/// triples standing over it. A cell in `n` charts certifies every one of
/// its triples — they are all genuinely inhabited, by that very cell — and
/// the triples are emitted with the first witness that certified each, so
/// a simplex appears once however many cells certify it.
///
/// THIS IS NOT A CLIQUE SEARCH. Nothing here looks at the meet graph and
/// nothing enumerates combinations of charts: the walk is over the base,
/// and a triple exists in the output only because a cell of the base was
/// found standing in all three. A triple whose charts pairwise meet around
/// an uncovered pore never appears, which is the point — that triangle is
/// a hole, and filling it would kill a real homology class by fiat.
/// The refusals are returned BESIDE the simplices and never dropped. A
/// cell whose key does not decode is a cover this reader cannot walk, and
/// returning the triples alone would report a partial walk as a complete
/// one (rule 11).
pub fn certified_triples(
    charts: &[Vec<coord::Lamina>],
) -> (Vec<Certified>, Vec<crate::network::nerve::keyed::CoordinateError>) {
    let support = support_of_cover(charts);
    let (cells, refused) = support.decoded();
    // Keyed by the triple's own ordinals, so a simplex certified by many
    // cells is emitted once and the first witness is the one kept.
    let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    let mut out: Vec<Certified> = Vec::new();
    for (cell, standing) in cells {
        if standing.len() < 3 {
            continue;
        }
        // The charts over one cell, in the cover's order: every three of
        // them name a triple this cell inhabits. The bound is the cell's
        // own valence, not the cover's size.
        for a in 0..standing.len() {
            for b in (a + 1)..standing.len() {
                for c in (b + 1)..standing.len() {
                    let charts = [standing[a], standing[b], standing[c]];
                    let mut key = Vec::with_capacity(24);
                    for ix in charts {
                        key.extend_from_slice(&(ix as u64).to_le_bytes());
                    }
                    if seen.get(&key).is_some() {
                        continue;
                    }
                    seen.insert(&key, ());
                    out.push(Certified { charts, witness: cell });
                }
            }
        }
    }
    (out, refused)
}
