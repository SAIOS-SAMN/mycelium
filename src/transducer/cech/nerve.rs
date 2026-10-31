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
//! over it are one maximal cell, of grade one less than the members' number, and
//! every element standing under the same members is a further witness of
//! the same cell, held as its keyed fibre. Faces are read off a cell, never
//! minted as subsets: a set of `m` members over one element is one cell, not
//! `2^m`. The certified triples of 58.1 are this read's grade-two instance.
//!
//! THE MEET IS THE CELL'S SHAPE (78.2 as corrected). A nucleus is a sphere
//! vector, so the wedge of the members' nuclei is the nuclei's MEET — the round
//! the bodies share, at its own grade, read with no square and no
//! coordinate. It is a projective blade, so the members' order changes only
//! its sign and no order is chosen for it. The cell's TRANSPORT is not here:
//! it is deposited by the circuit that closes on the cell, in that circuit's
//! own order, and released by `descend` (choice (b)); a certified cell no
//! circuit has closed carries its meet and no transport.

use crate::network::nerve::keyed::coord;
use crate::transducer::cga::Mv;

/// ONE CERTIFIED CELL: its members, each by its coordinates, every element of
/// the base that stands under exactly them, and the meet of the members' nuclei.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    /// The members, in the members' coordinate order (KB1): the caller's ordinals,
    /// by which it reads each member back, never the cell's identity.
    pub members: Vec<usize>,
    /// Each member's coordinates, in the same order — what the cell is keyed
    /// by (78.30(7), A21).
    pub coords: Vec<Vec<u8>>,
    /// The witnesses, in the base's own coordinate order.
    pub witnesses: Vec<coord::Compartment>,
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
    pub refused: Vec<crate::network::nerve::keyed::CoordinateRefusal>,
    /// Members standing at a witness whose coordinates did not read (KB1).
    pub unkeyed: u64,
    /// Witnesses whose members are one by the members' coordinates (KB1 R1).
    pub singular: u64,
}

/// ONE CHART OF A COVER: its section's coordinates — the carriage and the
/// footprint, as `section_key` writes them — and the elements it stands on.
pub type Chart = (Vec<u8>, Vec<coord::Compartment>);

/// READ THE NERVE OF ONE COVER. The support, the pairwise reading, the
/// certified cells and the cells' meets come off ONE read of the base, so the
/// numbers cannot drift apart (law 12). Each chart comes with its section's
/// coordinates, which key its cells (KB1): two carriages over one region are
/// two sections because the carriages differ, and nothing merges them.
pub fn read_nerve(charts: &[Chart]) -> NerveReading {
    let footprints: Vec<Vec<coord::Compartment>> = charts.iter().map(|(_, f)| f.clone()).collect();
    let support = support_of_cover(&footprints);
    let (cells, refused) = support.decoded();
    let shared = cells.iter().filter(|(_, s)| s.len() >= 2).count();
    // Each chart's nucleus, read once from its own sites.
    let nuclei: Vec<Option<Mv>> = footprints
        .iter()
        .map(|c| {
            let sites: Vec<crate::transducer::base::Site> = c.iter().map(|l| l.site.clone()).collect();
            crate::transducer::blade::shape_of(&sites).blade.nucleus()
        })
        .collect();
    let over = cells_over(
        cells.iter().map(|(cell, standing)| (cell.clone(), standing.as_slice())),
        |m| charts.get(m).map(|(k, _)| k.clone()),
        |members| {
            members.iter().map(|&m| nuclei.get(m).cloned().flatten()).collect::<Option<Vec<Mv>>>().and_then(|n| meet_of(&n))
        },
    );
    NerveReading {
        charts: charts.len(),
        cells: cells.len(),
        shared,
        certified: over.cells,
        refused,
        unkeyed: over.unkeyed,
        singular: over.singular,
    }
}

/// THE CELLS [`cells_over`] READ, and the members it could not key.
pub struct CellsOver {
    pub cells: Vec<Cell>,
    /// Members standing at a witness whose coordinates did not read: counted
    /// by name, and the members' standing set mints no cell (KB1, A.17).
    pub unkeyed: u64,
    /// Witnesses whose two or more members are one by the members' coordinates: the
    /// set is a single member and mints no cell (KB1 R1).
    pub singular: u64,
}

/// THE CERTIFIED CELLS OVER AN ALREADY-READ SUPPORT: one per standing set of
/// two or more members, each element standing under it a witness, the meet
/// read by `meet`. KEYED BY THE MEMBERS' COORDINATES (KB1, 78.30(7), A21) —
/// `coords` reads each member's, the cell's key is them framed in the members' own
/// order, and the members and the meet follow that order — never by the
/// caller's ordinals, which name a position and not an object (rule 15). A
/// set read at many elements is one cell and the elements are its fibre; two
/// ordinals with one coordinate are one member. A single member standing
/// alone is a point of the nerve, not a cell.
pub fn cells_over<'a>(
    standing: impl Iterator<Item = (coord::Compartment, &'a [usize])>,
    coords: impl Fn(usize) -> Option<Vec<u8>>,
    meet: impl Fn(&[usize]) -> Option<Mv>,
) -> CellsOver {
    let mut by_coords: crate::network::nerve::Keyed<Cell> = crate::network::nerve::Keyed::new();
    let (mut unkeyed, mut singular) = (0u64, 0u64);
    for (witness, members) in standing {
        let mut keyed: crate::network::nerve::Keyed<usize> = crate::network::nerve::Keyed::new();
        let mut unread = 0u64;
        for &m in members {
            match coords(m) {
                Some(c) => {
                    let mut framed = Vec::new();
                    crate::transducer::address::put_key_bytes(&mut framed, &c);
                    keyed.insert(&framed, m);
                }
                None => unread += 1,
            }
        }
        if unread > 0 {
            unkeyed += unread;
            continue;
        }
        // One member standing alone is a point of the nerve, not a cell. Two
        // or more that are ONE by the members' coordinates are a reading: counted.
        match (members.len() >= 2, keyed.len() >= 2) {
            (_, true) => {}
            (true, false) => {
                singular += 1;
                continue;
            }
            (false, false) => continue,
        }
        let key: Vec<u8> = keyed.scoped().flat_map(|(k, _)| k).collect();
        match by_coords.get_mut(&key) {
            Some(cell) => cell.witnesses.push(witness),
            None => {
                let members: Vec<usize> = keyed.scoped().map(|(_, &m)| m).collect();
                let coords: Vec<Vec<u8>> = members.iter().filter_map(|&m| coords(m)).collect();
                let meet = meet(&members);
                by_coords.insert(&key, Cell { members, coords, witnesses: vec![witness], meet });
            }
        }
    }
    CellsOver { cells: by_coords.scoped().map(|(_, c)| c.clone()).collect(), unkeyed, singular }
}

/// THE NERVE READINGS OF MANY COVERS, SUMMED — and each cell kept beside the
/// cover whose chart ordinals it names.
///
/// A COVER IS ONE DEFORMATION AND ITS BASE IS THAT DEFORMATION'S OWN
/// SOURCE. Two covers' charts are elements of two different bases, so the covers'
/// supports are summed and never intersected with one another.
#[derive(Default)]
pub struct NerveTotals {
    pub charts: usize,
    pub cells: usize,
    pub shared: usize,
    /// `(cover ordinal, cell)`.
    pub certified: Vec<(usize, Cell)>,
    pub refused: Vec<crate::network::nerve::keyed::CoordinateRefusal>,
    pub unkeyed: u64,
    pub singular: u64,
}

impl NerveTotals {
    pub fn absorb(&mut self, ix: usize, other: NerveReading) {
        self.charts += other.charts;
        self.cells += other.cells;
        self.shared += other.shared;
        self.refused.extend(other.refused);
        self.unkeyed += other.unkeyed;
        self.singular += other.singular;
        self.certified.extend(other.certified.into_iter().map(|c| (ix, c)));
    }

    /// THE CERTIFIED CELLS BY GRADE — the nerve line's reading, by the one
    /// reader [`cells_by_grade`].
    pub fn by_grade(&self) -> Vec<(usize, u64, u64)> {
        cells_by_grade(self.certified.iter().map(|(_, c)| (c.grade(), c.meet.clone())))
    }

    /// The nerve line's cells segment: the certified cells by grade, then
    /// what did not key — compartment keys refused and members unkeyed (KB1).
    pub fn cells_line(&self) -> String {
        format!(
            "certified cells (grade cells meet≠0) {:?}, {} refused, {} unkeyed, {} singular",
            self.by_grade(),
            self.refused.len(),
            self.unkeyed,
            self.singular
        )
    }
}

/// CERTIFIED CELLS BY GRADE: `(grade, cells, cells whose meet is not the
/// zero blade)`, ascending by grade, over any carrier's `(grade, meet)` —
/// the ONE reader the cover nerve and the contact complex share (law 12,
/// NB1's return).
pub fn cells_by_grade(cells: impl Iterator<Item = (usize, Option<Mv>)>) -> Vec<(usize, u64, u64)> {
    let mut out: Vec<(usize, u64, u64)> = Vec::new();
    for (grade, meet) in cells {
        let live = u64::from(meet.is_some_and(|m| !m.is_zero()));
        match out.binary_search_by_key(&grade, |&(g, _, _)| g) {
            Ok(at) => {
                out[at].1 += 1;
                out[at].2 += live;
            }
            Err(at) => out.insert(at, (grade, 1, live)),
        }
    }
    out
}

/// THE SUPPORT OF A COVER, READ FROM THE BASE: `p ↦ charts(p)`.
///
/// One linear pass over every element of every chart, `O(elements ·
/// valence)`. The charts standing over an element are kept in the cover's
/// own order and deduplicated as they arrive, so the reading does not depend
/// on the order the elements were read.
pub fn support_of_cover(
    charts: &[Vec<coord::Compartment>],
) -> crate::network::nerve::Keyed<Vec<usize>, coord::Compartment> {
    let mut support: crate::network::nerve::Keyed<Vec<usize>, coord::Compartment> =
        crate::network::nerve::Keyed::new();
    for (ix, compartments) in charts.iter().enumerate() {
        for p in compartments {
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
