// 13, 54.7, 72.1b, 78.2, NB1 — THE NERVE OF A COVER, READ OFF THE BASE: one
// certified cell per witness, at the grade its standing set has, carrying
// the meet of its members' nuclei.
//
//! A cell of the nerve is not a fact about the meet graph; it is a fact
//! about the base. `N(𝒰)` has a `k`-cell `[i₀ … i_k]` exactly where
//! `U_{i₀} ∩ … ∩ U_{i_k} ≠ ∅`, and the only honest proof of that is a
//! WITNESS — an element of the base standing in all of them at once.
//!
//! ONE CELL PER STANDING SET, NO ARITY WRITTEN IN (NB1 as ruled at 8fa57ce,
//! choice (a)). Each element of the base is read once; the members standing
//! over it are one maximal cell, of grade one less than their number, and
//! every element standing under the same members is a further witness of
//! the same cell, held as its keyed fibre. Faces are read off a cell, never
//! minted as subsets: a set of `m` members over one element is one cell, not
//! `2^m`. The certified triples of 58.1 are this read's grade-two instance.
//!
//! THE MEET IS THE CELL'S SHAPE (78.2 as corrected). A nucleus is a sphere
//! vector, so the wedge of the members' nuclei is their MEET — the round
//! the bodies share, at its own grade, read with no square and no
//! coordinate. It is a projective blade, so the members' order changes only
//! its sign and no order is chosen for it. The cell's TRANSPORT is not here:
//! it is deposited by the circuit that closes on the cell, in that circuit's
//! own order, and released by `descend` (choice (b)); a certified cell no
//! circuit has closed carries its meet and no transport.

use crate::network::nerve::keyed::coord;
use crate::transducer::cga::Mv;

/// ONE CERTIFIED CELL: its members, every element of the base that stands
/// under exactly them, and the meet of their nuclei.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    /// The members, ascending, in the cover's own ordinals.
    pub members: Vec<usize>,
    /// The witnesses, in the base's own coordinate order.
    pub witnesses: Vec<coord::Lamina>,
    /// `s₀ ∧ s₁ ∧ … ∧ s_k`; `None` where a member's nucleus did not read
    /// back (rule 11), the zero blade where the nuclei are dependent.
    pub meet: Option<Mv>,
}

impl Cell {
    /// A cell on `k + 1` members has grade `k`. A cell is minted only over
    /// two or more members, so the subtraction is total.
    pub fn grade(&self) -> usize {
        self.members.len() - 1
    }
}

/// THE MEET OF NUCLEI: the outer product, one nucleus at a time
/// (`Mv::wedge_vector`). Grassmann ends it — a dependent nucleus gives the
/// zero blade and past grade `n + 2` the exterior algebra is zero — so no
/// bound is written.
pub fn meet_of(nuclei: &[Mv]) -> Option<Mv> {
    let (first, rest) = nuclei.split_first()?;
    let mut blade = first.clone();
    for (g, s) in (1u32..).zip(rest) {
        blade = blade.wedge_vector(g, s);
    }
    Some(blade)
}

/// WHAT ONE COVER'S NERVE READ, AS ONE VALUE (58.1, ruled 2026-09-22).
///
/// Produced together, because each number is meaningless without the
/// others: no cells over no shared elements is a PARTITION, while cells of
/// grade one only are a genuine 1-complex whose charts meet in pairs and
/// never in threes (rule 11).
///
/// THE ORDINALS IN `cells` ARE THIS COVER'S OWN. A caller aggregating
/// several covers carries the cover beside each cell (rule 15).
#[derive(Default)]
pub struct NerveReading {
    /// Charts offered — one per cover piece, NOT deduplicated by
    /// footprint. Two carriages drawing on one source region are two
    /// sections over that region and two charts of the indexed cover.
    pub charts: usize,
    /// Elements of the base any chart stands on, and those with two or more
    /// charts over them.
    pub cells: usize,
    pub shared: usize,
    /// The certified cells, one per standing set.
    pub certified: Vec<Cell>,
    pub refused: Vec<crate::network::nerve::keyed::CoordinateError>,
}

/// READ THE NERVE OF ONE COVER. The support, the pairwise reading, the
/// certified cells and their meets come off ONE read of the base, so the
/// numbers cannot drift apart (law 12).
pub fn read_nerve(charts: &[Vec<coord::Lamina>]) -> NerveReading {
    let support = support_of_cover(charts);
    let (cells, refused) = support.decoded();
    let shared = cells.iter().filter(|(_, s)| s.len() >= 2).count();
    // Each chart's nucleus, read once from its own sites.
    let nuclei: Vec<Option<Mv>> = charts
        .iter()
        .map(|c| {
            let sites: Vec<crate::transducer::base::Site> = c.iter().map(|l| l.site.clone()).collect();
            crate::transducer::blade::shape_of(&sites).blade.nucleus()
        })
        .collect();
    let certified = cells_over(cells.iter().map(|(cell, standing)| (cell.clone(), standing.as_slice())), |members| {
        members.iter().map(|&m| nuclei.get(m).cloned().flatten()).collect::<Option<Vec<Mv>>>().and_then(|n| meet_of(&n))
    });
    NerveReading { charts: charts.len(), cells: cells.len(), shared, certified, refused }
}

/// THE CERTIFIED CELLS OVER AN ALREADY-READ SUPPORT: one per standing set of
/// two or more members, each element standing under it a witness, the
/// meet read by `meet`. Keyed by the members' own coordinates, so a set read
/// at many elements is one cell and the elements are its fibre.
pub fn cells_over<'a>(
    standing: impl Iterator<Item = (coord::Lamina, &'a [usize])>,
    meet: impl Fn(&[usize]) -> Option<Mv>,
) -> Vec<Cell> {
    let mut by_members: crate::network::nerve::Keyed<Cell> = crate::network::nerve::Keyed::new();
    for (witness, members) in standing {
        if members.len() < 2 {
            continue;
        }
        let mut members = members.to_vec();
        members.sort_unstable();
        let key: Vec<u8> = members.iter().flat_map(|&m| (m as u64).to_be_bytes()).collect();
        match by_members.get_mut(&key) {
            Some(cell) => cell.witnesses.push(witness),
            None => {
                let meet = meet(&members);
                by_members.insert(&key, Cell { members, witnesses: vec![witness], meet });
            }
        }
    }
    by_members.scoped().map(|(_, c)| c.clone()).collect()
}

/// THE NERVE READINGS OF MANY COVERS, SUMMED — and each cell kept beside the
/// cover whose chart ordinals it names.
///
/// A COVER IS ONE DEFORMATION AND ITS BASE IS THAT DEFORMATION'S OWN
/// SOURCE. Two covers' charts are elements of two different bases, so their
/// supports are summed and never intersected with one another.
#[derive(Default)]
pub struct NerveTotals {
    pub charts: usize,
    pub cells: usize,
    pub shared: usize,
    /// `(cover ordinal, cell)`.
    pub certified: Vec<(usize, Cell)>,
    pub refused: Vec<crate::network::nerve::keyed::CoordinateError>,
}

impl NerveTotals {
    pub fn absorb(&mut self, ix: usize, other: NerveReading) {
        self.charts += other.charts;
        self.cells += other.cells;
        self.shared += other.shared;
        self.refused.extend(other.refused);
        self.certified.extend(other.certified.into_iter().map(|c| (ix, c)));
    }

    /// THE CERTIFIED CELLS BY GRADE: `(grade, cells, cells whose meet is not
    /// the zero blade)`, ascending by grade — the nerve line's reading.
    pub fn by_grade(&self) -> Vec<(usize, u64, u64)> {
        let mut out: Vec<(usize, u64, u64)> = Vec::new();
        for (_, c) in &self.certified {
            let live = u64::from(c.meet.as_ref().is_some_and(|m| !m.is_zero()));
            match out.binary_search_by_key(&c.grade(), |&(g, _, _)| g) {
                Ok(at) => {
                    out[at].1 += 1;
                    out[at].2 += live;
                }
                Err(at) => out.insert(at, (c.grade(), 1, live)),
            }
        }
        out
    }
}

/// THE SUPPORT OF A COVER, READ FROM THE BASE: `p ↦ charts(p)`.
///
/// One linear pass over every element of every chart, `O(elements ·
/// valence)`. The charts standing over an element are kept in the cover's
/// own order and deduplicated as they arrive, so the reading does not depend
/// on the order the elements were read.
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
