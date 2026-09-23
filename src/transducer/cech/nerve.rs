// 13, 54.7 — THE NERVE OF A COVER, derived by WALKING THE BASE.
//
//! A simplex of the nerve is not a fact about the meet graph; it is a fact
//! about the base. `N(𝒰)` has a `k`-simplex `[i₀ … i_k]` exactly where
//! `U_{i₀} ∩ … ∩ U_{i_k} ≠ ∅`, and the only honest proof of that is a
//! WITNESS — a cell of the base standing in all of them at once.

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

/// WHAT ONE COVER'S NERVE READ, AS ONE VALUE (58.1, ruled 2026-09-22).
///
/// The four numbers are one reading and are produced together, because
/// each is meaningless without the others: `0 triples` over `0 shared`
/// cells is a PARTITION — the nerve is a set of isolated points and no
/// attaching machinery can ever find a 2-cell — while `0 triples` over
/// many shared cells is a genuine 1-complex whose charts meet in pairs and
/// never in threes. Read separately and reported separately, those two
/// would be one number and would say the wrong thing (rule 11).
///
/// THE ORDINALS IN `triples` ARE THIS COVER'S OWN. A `Certified.charts`
/// indexes the slice `read_nerve` was handed, so a caller aggregating
/// several covers must carry the cover beside the triple or the three
/// ordinals name charts of no particular cover (rule 15).
#[derive(Default)]
pub struct NerveReading {
    /// Charts offered — one per cover piece, NOT deduplicated by
    /// footprint. Two carriages drawing on one source region are two
    /// sections over that region and two charts of the indexed cover;
    /// folding them into one erases replication, which is the only case
    /// where a transition between two copies of one shape exists at all.
    pub charts: usize,
    /// Cells of the base any chart stands on, and those with two or more
    /// charts over them.
    pub cells: usize,
    pub shared: usize,
    pub triples: Vec<Certified>,
    pub refused: Vec<crate::network::nerve::keyed::CoordinateError>,
}

/// READ THE NERVE OF ONE COVER. The support, the pairwise reading and the
/// certified 2-simplices come off ONE walk of the base, so the four
/// numbers cannot drift apart (law 12).
pub fn read_nerve(charts: &[Vec<coord::Lamina>]) -> NerveReading {
    let support = support_of_cover(charts);
    let (cells, refused) = support.decoded();
    let shared = cells.iter().filter(|(_, s)| s.len() >= 2).count();
    let triples = triples_over(&cells);
    NerveReading { charts: charts.len(), cells: cells.len(), shared, triples, refused }
}

/// THE NERVE READINGS OF MANY COVERS, SUMMED — and each triple kept beside
/// the cover whose chart ordinals it names.
///
/// A COVER IS ONE DEFORMATION AND ITS BASE IS THAT DEFORMATION'S OWN
/// SOURCE. Two covers' charts are cells of two different bases — each
/// recentred at its own frame zero — so their supports are summed and
/// never intersected with one another. Pooling them into one
/// `support_of_cover` would read a coincidence of two recentrings as an
/// overlap, which is the same defect the cover's own meet index avoids by
/// keying the two SIDES apart.
#[derive(Default)]
pub struct NerveTotals {
    pub charts: usize,
    pub cells: usize,
    pub shared: usize,
    /// `(cover ordinal, simplex)` — the ordinals inside a [`Certified`]
    /// index that cover's own chart order and name nothing without it.
    pub triples: Vec<(usize, Certified)>,
    pub refused: Vec<crate::network::nerve::keyed::CoordinateError>,
}

impl NerveTotals {
    pub fn absorb(&mut self, ix: usize, other: NerveReading) {
        self.charts += other.charts;
        self.cells += other.cells;
        self.shared += other.shared;
        self.refused.extend(other.refused);
        self.triples.extend(other.triples.into_iter().map(|t| (ix, t)));
    }
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
    let r = read_nerve(charts);
    (r.triples, r.refused)
}

/// The triples standing over an already-decoded support. Split out so
/// [`read_nerve`] and [`certified_triples`] are ONE producer of them and
/// cannot drift apart, and so the base is walked once per reading rather
/// than once per number taken off it (law 12).
fn triples_over(cells: &[(coord::Lamina, &Vec<usize>)]) -> Vec<Certified> {
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
                    out.push(Certified { charts, witness: cell.clone() });
                }
            }
        }
    }
    out
}
