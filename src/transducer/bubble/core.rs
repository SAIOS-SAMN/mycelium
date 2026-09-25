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
//! of degree at most one leaves: unique, read here, and applied by
//! `truncate` wherever degrees change — after a fill's merge and after a
//! shedding door removed anything — over the changed components only.
//! Nothing here spends the stroke.

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
    /// The nerve's `Ȟ¹` rank over ℚ. The elimination is exact at any size
    /// since STD1, so it always reads; the `None` arm stands only for the
    /// codec's sake until the TC1 readings are re-shaped.
    pub h1: Option<i64>,
    /// 2-simplices one of whose faces is no edge of the nerve (a remainder
    /// edge beside it): no boundary row, counted apart.
    pub unfaced: u64,
    /// What the complex's own construction rules out — a 2-simplex without
    /// three vertices, a link key naming no simplex — read as a finding,
    /// never skipped.
    pub malformed: u64,
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

/// TC1's fix — WHAT THE TRUNCATION DID, cumulative at the producer: the
/// thoughts it removed, the free vertices it refused (a class, congruence or
/// unspellable rotor on the one edge — 13.12's held coordinate, content off a
/// ring; a thought something glued, whose door is promotion or lapse), each
/// counted once per truncation that met it; the price lapses deferred while
/// a leaf stood in the ring's component; and the nerve's `Ȟ¹` after minus
/// before, summed — zero by the theorem, a finding otherwise — with the runs
/// whose rank did not read and the incidences no simplex answered.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Truncation {
    pub truncated: u64,
    pub held_by_class: u64,
    pub glued: u64,
    pub deferred: u64,
    pub rank_moved: i64,
    pub rank_unread: u64,
    pub malformed: u64,
}

/// Why a free vertex stands through the truncation.
enum Refusal {
    HeldByClass,
    Glued,
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

    /// Every vertex of the nerve and its edge degree.
    pub(crate) fn degrees_among_nerve(&self) -> Keyed<u64> {
        self.degrees_among(&self.nerve_vertices())
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
        self.two_core_within(self.nerve_vertices())
    }

    /// The 2-core of the subcomplex on `alive`. On a union of whole
    /// components it is the whole nerve's 2-core restricted to them: a
    /// removal in one component moves no degree in another.
    pub(crate) fn two_core_within(&self, mut alive: Keyed<()>) -> Keyed<()> {
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

    /// THE NERVE'S Ȟ¹ RANK over ℚ, the 2-simplices with no boundary row, and
    /// what was malformed.
    fn h1_rank(&self) -> (Option<i64>, u64, u64) {
        let mut malformed = 0u64;
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
                    let Some(s) = self.complex.simplex(&ek) else {
                        malformed += 1;
                        continue;
                    };
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
                let [a, b, cc] = s.vertices.as_slice() else {
                    malformed += 1;
                    continue;
                };
                // ∂[a b c] = [b c] − [a c] + [a b].
                let faces = [(key_of(&[b.clone(), cc.clone()]), 1i64), (key_of(&[a.clone(), cc.clone()]), -1), (key_of(&[a.clone(), b.clone()]), 1)];
                let row: Option<crate::core::lattice::SparseRow> =
                    faces.iter().map(|(k, sign)| index.get(k).map(|&i| (i, num_bigint::BigInt::from(*sign)))).collect();
                match row {
                    Some(mut r) => {
                        r.sort_unstable();
                        rows.push(r);
                    }
                    None => unfaced += 1,
                }
            }
        }
        // Exact at any size (STD1): the rank always reads.
        let rank = crate::core::lattice::sparse_rank(&rows);
        (Some(edges.len() as i64 - v + c - rank as i64), unfaced, malformed)
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
        (r.h1, r.unfaced, r.malformed) = self.h1_rank();
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

    /// The vertices `verts` and every vertex one edge from them: what a
    /// fill's merge or a removal of `verts` changes the degrees of, read
    /// before the change.
    pub(crate) fn touched_by(&self, verts: &[Vec<u8>]) -> Vec<Vec<u8>> {
        let mut out: Keyed<()> = Keyed::new();
        for v in verts {
            out.insert(v, ());
            for ek in self.complex.link(v, 1) {
                if let Some(s) = self.complex.simplex(&ek) {
                    for u in &s.vertices {
                        out.insert(u, ());
                    }
                }
            }
        }
        out.scoped().map(|(k, _)| k).collect()
    }

    /// The nerve's components over every edge that hold any standing vertex
    /// of `seeds`, as one vertex set.
    fn components_of(&mut self, seeds: &[Vec<u8>]) -> Keyed<()> {
        let mut seen: Keyed<()> = Keyed::new();
        let mut stack: Vec<Vec<u8>> = Vec::new();
        for s in seeds {
            if self.complex.sections.contains(s) && !seen.contains(s) {
                seen.insert(s, ());
                stack.push(s.clone());
            }
        }
        while let Some(x) = stack.pop() {
            for ek in self.complex.link(&x, 1) {
                let Some(s) = self.complex.simplex(&ek) else {
                    self.truncation.malformed += 1;
                    continue;
                };
                for u in &s.vertices {
                    if !seen.contains(u) {
                        seen.insert(u, ());
                        stack.push(u.clone());
                    }
                }
            }
        }
        seen
    }

    /// Why the free vertex `v` stands, if it does: its one edge among
    /// `alive` holds a class, a congruence or an unspellable rotor, or
    /// something glued it.
    fn truncation_refusal(&self, v: &[u8], alive: &Keyed<()>) -> Option<Refusal> {
        for ek in self.complex.link(v, 1) {
            let Some(s) = self.complex.simplex(&ek) else { continue };
            if !s.vertices.iter().all(|u| alive.contains(u)) {
                continue;
            }
            if matches!(s.fill, Fill::Class(..) | Fill::Congruent(..) | Fill::Unspellable(..)) {
                return Some(Refusal::HeldByClass);
            }
        }
        match self.bubbles.get(v) {
            Some(b) if !b.glued_by.is_empty() => Some(Refusal::Glued),
            _ => None,
        }
    }

    /// TC1's fix — THE TRUNCATION: over the components holding `seeds`,
    /// every free vertex (a thought on exactly one edge) leaves, round after
    /// round with the degrees recounted, down to the unique 2-core. Isolated
    /// thoughts stand; a refused free vertex stands and is counted. A removed
    /// thought's remainder goes to the cavity at its chart and its escrow to
    /// the room; no act is sealed. The nerve's `Ȟ¹` is read around it.
    pub(crate) fn truncate(&mut self, seeds: &[Vec<u8>]) {
        let mut alive = self.components_of(seeds);
        if alive.is_empty() {
            return;
        }
        let mut refused: Keyed<()> = Keyed::new();
        let mut before: Option<Option<i64>> = None;
        loop {
            let deg = self.degrees_among(&alive);
            let free: Vec<Vec<u8>> =
                deg.scoped().filter(|(k, &d)| d == 1 && !refused.contains(k)).map(|(k, _)| k).collect();
            let mut doomed = Vec::new();
            for v in free {
                match self.truncation_refusal(&v, &alive) {
                    Some(Refusal::HeldByClass) => {
                        self.truncation.held_by_class += 1;
                        refused.insert(&v, ());
                    }
                    Some(Refusal::Glued) => {
                        self.truncation.glued += 1;
                        refused.insert(&v, ());
                    }
                    None => doomed.push(v),
                }
            }
            if doomed.is_empty() {
                break;
            }
            if before.is_none() {
                before = Some(self.h1_rank().0);
            }
            for v in &doomed {
                alive.remove(v);
                self.truncation.truncated += 1;
                match self.bubbles.remove(v) {
                    Some(b) => {
                        for (acc, e) in self.truncated_escrow.iter_mut().zip(&b.escrow) {
                            *acc = acc.saturating_add(*e);
                        }
                        self.retract_incidence(v, &b);
                        self.dirty.remove(v);
                        self.cavity.insert(super::shed::cavity_key(&b.remainder, v), b.remainder.to_vec());
                    }
                    None => self.complex.drop_vertex(v),
                }
            }
        }
        if let Some(b) = before {
            match (b, self.h1_rank().0) {
                (Some(b), Some(a)) => self.truncation.rank_moved += a - b,
                _ => self.truncation.rank_unread += 1,
            }
        }
    }

    /// THE PRICE DEFERS A RING WHILE A LEAF STANDS: of `doomed`, a thought
    /// in the 2-core whose component holds a free vertex not itself doomed
    /// stays, counted `deferred`, and lapses once the leaf is gone.
    pub(crate) fn defer_price(&mut self, doomed: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        if doomed.is_empty() {
            return doomed;
        }
        let core = self.two_core();
        if !doomed.iter().any(|k| core.contains(k)) {
            return doomed;
        }
        let mut dset: Keyed<()> = Keyed::new();
        for k in &doomed {
            dset.insert(k, ());
        }
        let deg = self.degrees_among(&self.nerve_vertices());
        // Each component's verdict, read once and kept for its vertices.
        let mut leafy: Keyed<bool> = Keyed::new();
        let mut out = Vec::new();
        for k in doomed {
            if !core.contains(&k) {
                out.push(k);
                continue;
            }
            let held = match leafy.get(&k) {
                Some(&l) => l,
                None => {
                    let comp = self.components_of(std::slice::from_ref(&k));
                    let l = comp.scoped().any(|(v, _)| deg.get(&v) == Some(&1) && !dset.contains(&v));
                    for (v, _) in comp.scoped() {
                        leafy.insert(&v, l);
                    }
                    l
                }
            };
            match held {
                true => self.truncation.deferred += 1,
                false => out.push(k),
            }
        }
        out
    }

    /// The escrow the truncation returned since last taken, for the room.
    pub fn take_truncated_escrow(&mut self) -> [u64; 4] {
        std::mem::take(&mut self.truncated_escrow)
    }
}
