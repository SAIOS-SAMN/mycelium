// 78.1, 78.3, LC1 — THE LEARNING CURVE, one mind at one `K`: its perspective
// by content and what its closures did, read off the mind's own genome and
// served to the console. Honest at zero; a field no producer yet reads is
// OWED, by the row that owes it, never printed as zero.
//
//! The three coordinates of a perspective (74, 78.1) are carried by what
//! they are, never by an orbital label:
//!
//! - the DIALECT SPAN — every stratum of the mind's genome atlas, by its
//!   chart key, with the words it holds;
//! - the COVER — the readings the mind's charts are cut by (53.2). OWED: the
//!   roster holds one reading, Bands, and a genome chart does not record the
//!   reading it was cut by, so every mind would print the same constant — a
//!   default standing in for a measurement (rule 11);
//! - the PLANES READ — the voice's own [`super::Planes`] reading, read
//!   beside this one and never copied into it (law 12).
//!
//! What the closures did: the generators a closure opened in this genome
//! (`Held::generators_opened`, the reader a court's `Kernel::adjoined`
//! shares), the genome's words, and of those the ones whose form closes
//! (50.1: coheres, no remainder, no demand).

use crate::client::sdk::Epigenome;

/// The fields LC1 names that no producer reads yet, each with the row that
/// owes it. The ONE statement of what is owed (law 12), read by every
/// renderer: a row that lands its producer moves its field into
/// [`Learning`] and off this list.
/// The coboundary's agreement per grade is PG1's (`Progress`, the round kept
/// per grade), read beside this one and never copied into it (law 12).
pub const OWED: [(&str, &str); 4] = [
    ("cover", "53.2's readings per mind: K4, K5 widen the roster past Bands"),
    ("warm-cold delta", "Y1, re-taken on the fleet"),
    ("distinct perspectives per class", "CV3"),
    ("mass per lane per layer per grade", "E5 (MS1, Phase P)"),
];

/// ONE MIND'S CURVE AT ONE `K`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Learning {
    /// The mind's own `K` when read.
    pub k: u64,
    /// The dialect span: each stratum of the genome atlas, by its chart key,
    /// and its words, in the key's own order.
    pub strata: Vec<(Vec<u8>, u64)>,
    /// THE RADIUS (78.23 as amended): the deepest A21 prefix the span
    /// reaches — the most axes any stratum's chart carries, chart-space
    /// distance being prefix depth. Read with the span, once; the clock
    /// carries it from here.
    pub radius: u64,
    /// Generators a closure opened in this genome (I1's reading, per mind).
    pub adjoined_by_closure: u64,
    /// Words the genome holds: the court-ruled words taught to this mind.
    pub installed: u64,
    /// Of them, those whose form closes.
    pub closing: u64,
    /// CS2, 78.32(c) — this mind's pivots against the cells their orbits
    /// cross, by content: the count the admission gate waits on.
    pub pivots: crate::transducer::sentence::PivotTally,
    /// 13.12, 78.26(6) — the contemplation's merges: merged, stayed two,
    /// unnamed (TP1's line, which adds decohered and collapsed).
    pub merges: crate::transducer::bubble::merge::MergeTally,
    /// 78.37(1), TP1 — the meet graph as it stands at `k`, and its
    /// decoherences since the contemplation began.
    /// Boxed: its counts are the reading's bulk, and the Reading enum holds
    /// every kind at the size of its largest.
    pub meet: Box<crate::transducer::bubble::MeetGraph>,
}

impl Learning {
    /// THE CURVE OF THIS MIND, read off its own genome at its own `k`, at the
    /// read, with its contemplation's pivot reading beside it; nothing is
    /// stored beside the carriers (law 12).
    pub fn of(
        epigenome: &Epigenome,
        k: u64,
        pivots: crate::transducer::sentence::PivotTally,
        merges: crate::transducer::bubble::merge::MergeTally,
        meet: crate::transducer::bubble::MeetGraph,
    ) -> Learning {
        let mut radius = 0u64;
        let strata = epigenome
            .atlas
            .strata()
            .into_iter()
            .map(|(inv, members)| {
                radius = radius.max(inv.0.len() as u64);
                (inv.key().as_bytes().to_vec(), members.len() as u64)
            })
            .collect();
        let (mut installed, mut closing, mut adjoined_by_closure) = (0u64, 0u64, 0u64);
        for held in epigenome.genome_words() {
            installed += 1;
            adjoined_by_closure += held.generators_opened();
            if held.form().is_some_and(|f| f.is_closed()) {
                closing += 1;
            }
        }
        Learning { k, strata, radius, adjoined_by_closure, installed, closing, pivots, merges, meet: Box::new(meet) }
    }

    pub(crate) fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.k.to_le_bytes());
        out.extend_from_slice(&self.radius.to_le_bytes());
        out.extend_from_slice(&(self.strata.len() as u32).to_le_bytes());
        for (key, n) in &self.strata {
            out.extend_from_slice(&(key.len() as u32).to_le_bytes());
            out.extend_from_slice(key);
            out.extend_from_slice(&n.to_le_bytes());
        }
        for v in [self.adjoined_by_closure, self.installed, self.closing] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        let t = &self.pivots;
        for v in [t.reached, t.derivable, t.demanded, t.adjoin, t.starved, t.unread_fibre, t.unread_algebra] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        let m = &self.merges;
        for v in [m.merged, m.stayed, m.unnamed] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        // TP1 — the meet graph, field by field in its own order.
        let g = &self.meet;
        for v in [g.vertices, g.thoughts, g.components] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for v in g.edges {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out.extend_from_slice(&g.cycle_rank.to_le_bytes());
        for v in g.rings.iter().chain(&g.orbits) {
            out.extend_from_slice(&v.to_le_bytes());
        }
        let d = &g.decoherence;
        for v in d.decohered.iter().chain(&d.classed).chain(&d.rejoined).flatten() {
            out.extend_from_slice(&v.to_le_bytes());
        }
        let t = &g.transport;
        for v in [
            g.advanced,
            g.sources,
            t.fixed[0],
            t.fixed[1],
            t.collapsed,
            t.collapsed_filled,
            t.held,
            t.classified,
            t.unsectioned,
            t.ungauged,
            t.unclassed,
            g.docket.pending,
            g.docket.open,
            g.docket.owed,
            g.docket.unwritten,
            g.docket.answered,
            g.docket.taken,
        ] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        // TC1 — the nerve's shape, its Ȟ¹ flagged where it did not read, and
        // the shedding doors.
        let c = &g.core;
        for v in [c.free_vertices, c.isolated, c.free_faces[0], c.free_faces[1], c.core, c.unfaced, c.malformed] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        match c.h1 {
            Some(r) => {
                out.push(1);
                out.extend_from_slice(&r.to_le_bytes());
            }
            None => out.push(0),
        }
        let s = &g.shed;
        for v in s.removed.iter().chain(&s.in_core).chain(&s.rank_unread) {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for v in s.rank_lost {
            out.extend_from_slice(&v.to_le_bytes());
        }
        let t = &g.truncation;
        for v in [t.truncated, t.held_by_class, t.glued, t.deferred] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out.extend_from_slice(&t.rank_moved.to_le_bytes());
        for v in [t.rank_unread, t.malformed] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        // FX1 — the flux: a window or a count that did not read flagged.
        let f = &g.flux;
        let opt = |out: &mut Vec<u8>, x: Option<u64>| match x {
            Some(n) => {
                out.push(1);
                out.extend_from_slice(&n.to_le_bytes());
            }
            None => out.push(0),
        };
        for v in [f.closes, f.orbit.traversed, f.orbit.zero, f.orbit.held] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        match &f.aperture {
            Some(w) => {
                out.push(1);
                for v in [w.traversed, w.zero, w.held] {
                    out.extend_from_slice(&v.to_le_bytes());
                }
            }
            None => out.push(0),
        }
        for v in [f.gone, f.unfilamented, f.credited, f.acyclic, f.unfiled, f.unsealed, f.unnamed] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for (lanes, n) in f.extent.iter().zip(&f.extent_n) {
            for v in lanes.iter().chain(std::iter::once(n)) {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        out.extend_from_slice(&f.dialect.0.to_le_bytes());
        opt(out, f.dialect.1);
        opt(out, f.dialect.2);
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Learning> {
        let u64_at = |b: &[u8], p: &mut usize| -> Option<u64> {
            let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(v)
        };
        let u32_at = |b: &[u8], p: &mut usize| -> Option<usize> {
            let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
            *p += 4;
            Some(v as usize)
        };
        let keyed = |b: &[u8], p: &mut usize| -> Option<Vec<(Vec<u8>, u64)>> {
            let m = u32_at(b, p)?;
            // Each entry is at least a length and a count; a count the body
            // cannot hold is refused before anything is reserved for it.
            if b.len().checked_sub(*p)? < m.checked_mul(12)? {
                return None;
            }
            let mut out = Vec::with_capacity(m);
            for _ in 0..m {
                let len = u32_at(b, p)?;
                let key = b.get(*p..*p + len)?.to_vec();
                *p += len;
                out.push((key, u64_at(b, p)?));
            }
            Some(out)
        };
        let k = u64_at(b, p)?;
        let radius = u64_at(b, p)?;
        let strata = keyed(b, p)?;
        let (adjoined_by_closure, installed, closing) = (u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?);
        let pivots = crate::transducer::sentence::PivotTally {
            reached: u64_at(b, p)?,
            derivable: u64_at(b, p)?,
            demanded: u64_at(b, p)?,
            adjoin: u64_at(b, p)?,
            starved: u64_at(b, p)?,
            unread_fibre: u64_at(b, p)?,
            unread_algebra: u64_at(b, p)?,
        };
        let merges = crate::transducer::bubble::merge::MergeTally {
            merged: u64_at(b, p)?,
            stayed: u64_at(b, p)?,
            unnamed: u64_at(b, p)?,
        };
        let six = |b: &[u8], p: &mut usize| -> Option<[u64; 6]> {
            let mut xs = [0u64; 6];
            for x in xs.iter_mut() {
                *x = u64_at(b, p)?;
            }
            Some(xs)
        };
        // A decoherence count: by cut (filled, congruent), then by adjunct.
        let two = |b: &[u8], p: &mut usize| -> Option<[[u64; 2]; 2]> {
            Some([[u64_at(b, p)?, u64_at(b, p)?], [u64_at(b, p)?, u64_at(b, p)?]])
        };
        let (vertices, thoughts, components) = (u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?);
        let edges = six(b, p)?;
        let cycle_rank = u64_at(b, p)? as i64;
        let (rings, orbits) = (six(b, p)?, six(b, p)?);
        let decoherence = crate::transducer::bubble::Decoherence {
            decohered: two(b, p)?,
            classed: two(b, p)?,
            rejoined: two(b, p)?,
        };
        let (advanced, sources) = (u64_at(b, p)?, u64_at(b, p)?);
        let transport = crate::transducer::bubble::TransportTally {
            fixed: [u64_at(b, p)?, u64_at(b, p)?],
            collapsed: u64_at(b, p)?,
            collapsed_filled: u64_at(b, p)?,
            held: u64_at(b, p)?,
            classified: u64_at(b, p)?,
            unsectioned: u64_at(b, p)?,
            ungauged: u64_at(b, p)?,
            unclassed: u64_at(b, p)?,
        };
        let docket = crate::transducer::bubble::DocketReading {
            pending: u64_at(b, p)?,
            open: u64_at(b, p)?,
            owed: u64_at(b, p)?,
            unwritten: u64_at(b, p)?,
            answered: u64_at(b, p)?,
            taken: u64_at(b, p)?,
        };
        let (free_vertices, isolated, faces_1, faces_2, core_size) =
            (u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?);
        let (unfaced, malformed) = (u64_at(b, p)?, u64_at(b, p)?);
        let h1 = match b.get(*p)? {
            0 => {
                *p += 1;
                None
            }
            1 => {
                *p += 1;
                Some(u64_at(b, p)? as i64)
            }
            _ => return None,
        };
        let core = crate::transducer::bubble::CoreReading {
            free_vertices,
            isolated,
            free_faces: [faces_1, faces_2],
            core: core_size,
            h1,
            unfaced,
            malformed,
        };
        let three = |b: &[u8], p: &mut usize| -> Option<[u64; 3]> { Some([u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?]) };
        let (removed, in_core, rank_unread) = (three(b, p)?, three(b, p)?, three(b, p)?);
        let lost = three(b, p)?;
        let shed = crate::transducer::bubble::ShedTally {
            removed,
            in_core,
            rank_lost: [lost[0] as i64, lost[1] as i64, lost[2] as i64],
            rank_unread,
        };
        // TC1's fix — the truncation, its Ȟ¹ movement signed.
        let truncation = crate::transducer::bubble::Truncation {
            truncated: u64_at(b, p)?,
            held_by_class: u64_at(b, p)?,
            glued: u64_at(b, p)?,
            deferred: u64_at(b, p)?,
            rank_moved: u64_at(b, p)? as i64,
            rank_unread: u64_at(b, p)?,
            malformed: u64_at(b, p)?,
        };
        // FX1 — the flux, as written.
        let flag = |b: &[u8], p: &mut usize| -> Option<bool> {
            let f = match b.get(*p)? {
                0 => false,
                1 => true,
                _ => return None,
            };
            *p += 1;
            Some(f)
        };
        let window = |b: &[u8], p: &mut usize| -> Option<crate::transducer::bubble::FluxWindow> {
            Some(crate::transducer::bubble::FluxWindow { traversed: u64_at(b, p)?, zero: u64_at(b, p)?, held: u64_at(b, p)? })
        };
        let mut flux = crate::transducer::bubble::FluxTally { closes: u64_at(b, p)?, orbit: window(b, p)?, ..Default::default() };
        if flag(b, p)? {
            flux.aperture = Some(window(b, p)?);
        }
        [flux.gone, flux.unfilamented, flux.credited, flux.acyclic, flux.unfiled, flux.unsealed, flux.unnamed] =
            [u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?];
        for (lanes, n) in flux.extent.iter_mut().zip(flux.extent_n.iter_mut()) {
            for l in lanes.iter_mut() {
                *l = u64_at(b, p)?;
            }
            *n = u64_at(b, p)?;
        }
        flux.dialect.0 = u64_at(b, p)?;
        if flag(b, p)? {
            flux.dialect.1 = Some(u64_at(b, p)?);
        }
        if flag(b, p)? {
            flux.dialect.2 = Some(u64_at(b, p)?);
        }
        let meet = crate::transducer::bubble::MeetGraph {
            vertices,
            thoughts,
            components,
            edges,
            cycle_rank,
            rings,
            orbits,
            decoherence,
            advanced,
            sources,
            transport,
            docket,
            core,
            shed,
            truncation,
            flux,
        };
        Some(Learning { k, strata, radius, adjoined_by_closure, installed, closing, pivots, merges, meet: Box::new(meet) })
    }
}
