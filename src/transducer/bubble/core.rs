// 78.37(2), TC1 — THE NERVE'S SHAPE, READ: its free vertices and free faces,
// its 2-core, the rank of its Ȟ¹; and, per shedding door, what a removal took
// that carried a loop.
//
//! The meet graph is read here as the NERVE of the held thoughts: its
//! vertices the complex's sections, its edges and 2-simplices the complex's
//! own, EVERY 2-simplex a cell whatever its fill — the 2-simplex is the
//! triple overlap and its fill the cochain's value. So the nerve's `Ȟ¹` is
//! topology, `E − V + C − rank ∂₂` over ℚ, and the classes a ring holds are
//! the cochain's, counted apart by the rings' reading.
//!
//! A FREE VERTEX is a thought on exactly one edge — on no ring and no
//! 2-simplex; a thought on no edge is ISOLATED, counted apart. A FREE FACE is
//! an edge that is a face of exactly one 2-simplex, or a 2-simplex a face of
//! exactly one 3-simplex. The 2-CORE is what iterated removal of every vertex
//! of degree at most one leaves: unique, read here and never applied (TC1's
//! fix applies it). Nothing here spends the stroke.

use super::*;

/// TC1 — THE NERVE'S SHAPE AT THE PUBLISH.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CoreReading {
    pub free_vertices: u64,
    pub isolated: u64,
    /// Free faces: edges (grade 1), 2-simplices (grade 2).
    pub free_faces: [u64; 2],
    /// The 2-core's vertices.
    pub core: u64,
    /// The nerve's `Ȟ¹` rank over ℚ; `None` where the elimination did not
    /// read (an entry past its width).
    pub h1: Option<i64>,
    /// 2-simplices one of whose faces is no edge of the nerve (a remainder
    /// edge beside it): no boundary row, counted apart.
    pub unfaced: u64,
}

/// TC1 — WHAT EACH SHEDDING DOOR REMOVED, cumulative at the producer: price
/// (the escrow's paid height passed), citation (a cited address lapsed),
/// unglued — each: thoughts removed, of those the ones standing in the
/// 2-core, and the nerve's `Ȟ¹` rank lost (before minus after, signed: a
/// removal can open a loop as well as close one), with the removals whose
/// rank did not read.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ShedTally {
    pub removed: [u64; 3],
    pub in_core: [u64; 3],
    pub rank_lost: [i64; 3],
    pub rank_unread: [u64; 3],
}

/// A shedding door, as the tally's index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Door {
    Price,
    Citation,
    Unglued,
}

impl Door {
    pub const ALL: [Door; 3] = [Door::Price, Door::Citation, Door::Unglued];

    pub fn index(self) -> usize {
        match self {
            Door::Price => 0,
            Door::Citation => 1,
            Door::Unglued => 2,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Door::Price => "price",
            Door::Citation => "citation",
            Door::Unglued => "unglued",
        }
    }
}

/// A simplex's key from its vertices.
fn key_of(verts: &[Vec<u8>]) -> Vec<u8> {
    crate::transducer::cech::simplex_key(&crate::transducer::cech::sorted_verts(verts)).as_bytes().to_vec()
}

impl Contemplation {
    /// Each vertex of `alive`, and its edge degree among `alive`: an edge
    /// counts where both its ends are alive.
    fn degrees_among(&self, alive: &Keyed<()>) -> Keyed<u64> {
        let mut deg: Keyed<u64> = Keyed::new();
        for (k, _) in alive.scoped() {
            deg.insert(&k, 0);
        }
        if let Some(layer) = self.complex.degrees.get(1) {
            for (_, s) in layer.scoped() {
                if !s.vertices.iter().all(|v| alive.contains(v)) {
                    continue;
                }
                for v in &s.vertices {
                    if let Some(d) = deg.get_mut(v) {
                        *d += 1;
                    }
                }
            }
        }
        deg
    }

    /// The nerve's vertices: the complex's sections.
    fn nerve_vertices(&self) -> Keyed<()> {
        let mut all: Keyed<()> = Keyed::new();
        for (k, _) in self.complex.sections.scoped() {
            all.insert(&k, ());
        }
        all
    }

    /// THE 2-CORE: the vertices iterated removal of degree ≤ 1 leaves, each
    /// round's degrees counted afresh among the vertices still standing.
    pub(crate) fn two_core(&self) -> Keyed<()> {
        let mut alive = self.nerve_vertices();
        loop {
            let deg = self.degrees_among(&alive);
            let low: Vec<Vec<u8>> = deg.scoped().filter(|(_, &d)| d <= 1).map(|(k, _)| k).collect();
            if low.is_empty() {
                return alive;
            }
            for v in low {
                alive.remove(&v);
            }
        }
    }

    /// THE NERVE'S Ȟ¹ RANK over ℚ, and the 2-simplices with no boundary row.
    fn h1_rank(&self) -> (Option<i64>, u64) {
        let v = self.complex.sections.len() as i64;
        let edges: Vec<Vec<u8>> = match self.complex.degrees.get(1) {
            Some(l) => l.scoped().map(|(k, _)| k).collect(),
            None => Vec::new(),
        };
        let mut index: Keyed<usize> = Keyed::new();
        for (i, k) in edges.iter().enumerate() {
            index.insert(k, i);
        }
        // Components over every edge.
        let mut seen: Keyed<()> = Keyed::new();
        let mut c = 0i64;
        for (k, _) in self.complex.sections.scoped() {
            if seen.contains(&k) {
                continue;
            }
            c += 1;
            let mut stack = vec![k.clone()];
            seen.insert(&k, ());
            while let Some(x) = stack.pop() {
                for ek in self.complex.link(&x, 1) {
                    let Some(s) = self.complex.simplex(&ek) else { continue };
                    for u in &s.vertices {
                        if !seen.contains(u) {
                            seen.insert(u, ());
                            stack.push(u.clone());
                        }
                    }
                }
            }
        }
        let (mut rows, mut unfaced) = (Vec::new(), 0u64);
        if let Some(layer) = self.complex.degrees.get(2) {
            for (_, s) in layer.scoped() {
                let [a, b, cc] = s.vertices.as_slice() else { continue };
                // ∂[a b c] = [b c] − [a c] + [a b].
                let faces = [(key_of(&[b.clone(), cc.clone()]), 1i64), (key_of(&[a.clone(), cc.clone()]), -1), (key_of(&[a.clone(), b.clone()]), 1)];
                let row: Option<crate::core::lattice::SparseRow> =
                    faces.iter().map(|(k, sign)| index.get(k).map(|&i| (i, *sign))).collect();
                match row {
                    Some(mut r) => {
                        r.sort_unstable();
                        rows.push(r);
                    }
                    None => unfaced += 1,
                }
            }
        }
        let rank = match rows.is_empty() {
            true => Some(0),
            false => crate::core::lattice::sparse_rank(&rows, edges.len()),
        };
        (rank.map(|r| edges.len() as i64 - v + c - r as i64), unfaced)
    }

    /// TC1 — the nerve's shape as it stands.
    pub fn core_reading(&self) -> CoreReading {
        let deg = self.degrees_among(&self.nerve_vertices());
        let mut r = CoreReading::default();
        for (_, &d) in deg.scoped() {
            match d {
                0 => r.isolated += 1,
                1 => r.free_vertices += 1,
                _ => {}
            }
        }
        // Free faces: count each face's cofaces one grade up.
        let mut cofaces: [Keyed<u64>; 2] = [Keyed::new(), Keyed::new()];
        for (g, counts) in cofaces.iter_mut().enumerate() {
            let Some(layer) = self.complex.degrees.get(g + 2) else { continue };
            for (_, s) in layer.scoped() {
                for skip in 0..s.vertices.len() {
                    let face: Vec<Vec<u8>> =
                        s.vertices.iter().enumerate().filter(|(i, _)| *i != skip).map(|(_, v)| v.clone()).collect();
                    let k = key_of(&face);
                    match counts.get_mut(&k) {
                        Some(n) => *n += 1,
                        None => {
                            counts.insert(&k, 1);
                        }
                    }
                }
            }
        }
        for (g, counts) in cofaces.iter().enumerate() {
            r.free_faces[g] = counts.scoped().filter(|(k, &n)| n == 1 && self.complex.degree_of(k) == Some(g + 1)).count() as u64;
        }
        r.core = self.two_core().len() as u64;
        (r.h1, r.unfaced) = self.h1_rank();
        r
    }

    /// A SHEDDING DOOR ABOUT TO REMOVE `doomed`: what stands in the 2-core and
    /// the nerve's rank, read before.
    pub(crate) fn shed_before(&self, doomed: &[Vec<u8>]) -> (u64, Option<i64>) {
        if doomed.is_empty() {
            return (0, None);
        }
        let core = self.two_core();
        (doomed.iter().filter(|k| core.contains(k)).count() as u64, self.h1_rank().0)
    }

    /// AND AFTER: counted on the door's own row.
    pub(crate) fn shed_after(&mut self, door: Door, removed: usize, (in_core, before): (u64, Option<i64>)) {
        if removed == 0 {
            return;
        }
        let d = door.index();
        self.shed.removed[d] += removed as u64;
        self.shed.in_core[d] += in_core;
        match (before, self.h1_rank().0) {
            (Some(b), Some(a)) => self.shed.rank_lost[d] += b - a,
            _ => self.shed.rank_unread[d] += 1,
        }
    }
}
