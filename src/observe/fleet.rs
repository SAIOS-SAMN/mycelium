// 78.16, VO1, SP1, PG1 — THE READINGS A LIVE FLEET IS ASKED BEFORE ITS SOAK:
// the viscosity a voice's bends read, the stroke each reader spent, and E1's
// progress — every one a typed reading, served on demand, never a grep.
//
//! - [`Viscosity`] (VO1) — VS1's arms per pair of planes per lane, summed
//!   over the voice's percepts pair by pair (never across pairs or lanes,
//!   law 5), and the ledger's entropy by lane.
//! - [`Spend`] (SP1) — per percept, the reads of the stroke each reader
//!   spent, and the readings each left unbounded (rule 10): whether
//!   comprehension starves perception, read on the stroke's own counters and
//!   never a clock (law 9).
//! - [`Progress`] (PG1) — E1's done-when per voice: contact cycles and those
//!   carrying non-trivial holonomy; certified cells by grade with the round
//!   kept (the coboundary's agreement at that grade, LC1's owed field), the
//!   round reversed, the refusals typed by grade, the closure classes and
//!   the dual frames.

use super::circuit::{holds, take_u16, take_u32, take_u8};

fn take_u64(b: &[u8], p: &mut usize) -> Option<u64> {
    let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    Some(v)
}

fn put_u64s(out: &mut Vec<u8>, vs: &[u64]) {
    for v in vs {
        out.extend_from_slice(&v.to_le_bytes());
    }
}

fn take_u64s<const N: usize>(b: &[u8], p: &mut usize) -> Option<[u64; N]> {
    let mut out = [0u64; N];
    for v in out.iter_mut() {
        *v = take_u64(b, p)?;
    }
    Some(out)
}

/// ONE PAIR OF PLANES ON ONE LANE (VS1): the lane's index in (𝒮, 𝒢, 𝒯, 𝒽),
/// the planes, the arms (inviscid, elastic, curved, viscous), and the trials
/// refused, unread and with no class.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ViscosityPair {
    pub lane: u8,
    pub planes: (u16, u16),
    pub arms: [u64; 4],
    pub refused: u64,
    pub unread: u64,
    pub no_class: u64,
}

/// VO1 — THE VISCOSITY A VOICE'S BENDS READ.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Viscosity {
    pub pairs: Vec<ViscosityPair>,
    /// The ledger's entropy by lane (`Metabolism::viscous`).
    pub entropy: [u64; 4],
}

impl Viscosity {
    /// Fold a later percept's reading: each pair's counts added to its own
    /// pair's, a new pair filed in order; the ledger's entropy, which is
    /// cumulative, taken from the later.
    pub fn absorb(&mut self, o: &Viscosity) {
        for p in &o.pairs {
            let key = (p.lane, p.planes);
            match self.pairs.binary_search_by_key(&key, |q| (q.lane, q.planes)) {
                Ok(at) => {
                    let q = &mut self.pairs[at];
                    for (a, b) in q.arms.iter_mut().zip(p.arms) {
                        *a += b;
                    }
                    q.refused += p.refused;
                    q.unread += p.unread;
                    q.no_class += p.no_class;
                }
                Err(at) => self.pairs.insert(at, p.clone()),
            }
        }
        self.entropy = o.entropy;
    }

    pub(crate) fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(self.pairs.len() as u32).to_le_bytes());
        for p in &self.pairs {
            out.push(p.lane);
            out.extend_from_slice(&p.planes.0.to_le_bytes());
            out.extend_from_slice(&p.planes.1.to_le_bytes());
            put_u64s(out, &p.arms);
            put_u64s(out, &[p.refused, p.unread, p.no_class]);
        }
        put_u64s(out, &self.entropy);
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Viscosity> {
        let m = take_u32(b, p)?;
        if !holds(b, *p, m, 5 + 7 * 8) {
            return None;
        }
        let mut pairs = Vec::with_capacity(m);
        for _ in 0..m {
            let lane = take_u8(b, p)?;
            let planes = (take_u16(b, p)?, take_u16(b, p)?);
            let arms = take_u64s::<4>(b, p)?;
            let [refused, unread, no_class] = take_u64s::<3>(b, p)?;
            pairs.push(ViscosityPair { lane, planes, arms, refused, unread, no_class });
        }
        Some(Viscosity { pairs, entropy: take_u64s::<4>(b, p)? })
    }
}

/// THE READERS [`Spend`] counts, in its writing's order, each with what it
/// reads. The ONE statement of the order (law 12).
pub const SPEND_READERS: [(&str, &str); 5] = [
    ("perception", "frames and quotients read, the form built"),
    ("orbit modes", "each kept orbit's modes by the action reader"),
    ("cells", "the cells' orbit algebras, Coxeter readings and products"),
    ("circuit modes", "each circuit step's modes"),
    ("viscosity", "VS1's bend trials"),
];

/// SP1 — THE STROKE, BY READER: over the voice's percepts, the reads each
/// reader spent, and the readings each left unbounded by a spent stroke.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Spend {
    pub percepts: u64,
    pub reads: [u64; 5],
    pub unbounded: [u64; 5],
}

impl Spend {
    pub fn absorb(&mut self, o: &Spend) {
        self.percepts += o.percepts;
        for (a, b) in self.reads.iter_mut().zip(o.reads) {
            *a += b;
        }
        for (a, b) in self.unbounded.iter_mut().zip(o.unbounded) {
            *a += b;
        }
    }

    pub(crate) fn put(&self, out: &mut Vec<u8>) {
        put_u64s(out, &[self.percepts]);
        put_u64s(out, &self.reads);
        put_u64s(out, &self.unbounded);
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Spend> {
        let percepts = take_u64(b, p)?;
        Some(Spend { percepts, reads: take_u64s::<5>(b, p)?, unbounded: take_u64s::<5>(b, p)? })
    }
}

/// ONE GRADE OF CERTIFIED CELLS (PG1).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GradeProgress {
    pub grade: u8,
    pub cells: u64,
    /// Every circuit read and every mode ray keeping the round: the
    /// coboundary's agreement at this grade.
    pub round_kept: u64,
    pub held_z2: u64,
    /// Refused with this grade named: a mode ray carrying the round.
    pub refused: u64,
    /// The constructors' group: finite, affine, other infinite.
    pub finite: u64,
    pub affine: u64,
    pub infinite: u64,
    pub dual_frames: u64,
}

/// PG1 — E1'S DONE-WHEN, PER VOICE.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Progress {
    pub percepts: u64,
    /// Circuits crossing a contact face; of them, those whose holonomy is
    /// read and not trivial; and those not read (OR3).
    pub contact_cycles: u64,
    pub contact_cycles_curved: u64,
    pub contact_cycles_unread: u64,
    pub grades: Vec<GradeProgress>,
}

impl Progress {
    pub fn absorb(&mut self, o: &Progress) {
        self.percepts += o.percepts;
        self.contact_cycles += o.contact_cycles;
        self.contact_cycles_curved += o.contact_cycles_curved;
        self.contact_cycles_unread += o.contact_cycles_unread;
        for g in &o.grades {
            match self.grades.binary_search_by_key(&g.grade, |h| h.grade) {
                Ok(at) => {
                    let h = &mut self.grades[at];
                    h.cells += g.cells;
                    h.round_kept += g.round_kept;
                    h.held_z2 += g.held_z2;
                    h.refused += g.refused;
                    h.finite += g.finite;
                    h.affine += g.affine;
                    h.infinite += g.infinite;
                    h.dual_frames += g.dual_frames;
                }
                Err(at) => self.grades.insert(at, g.clone()),
            }
        }
    }

    pub(crate) fn put(&self, out: &mut Vec<u8>) {
        put_u64s(out, &[self.percepts, self.contact_cycles, self.contact_cycles_curved, self.contact_cycles_unread]);
        out.extend_from_slice(&(self.grades.len() as u32).to_le_bytes());
        for g in &self.grades {
            out.push(g.grade);
            put_u64s(out, &[g.cells, g.round_kept, g.held_z2, g.refused, g.finite, g.affine, g.infinite, g.dual_frames]);
        }
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Progress> {
        let [percepts, contact_cycles, contact_cycles_curved, contact_cycles_unread] = take_u64s::<4>(b, p)?;
        let m = take_u32(b, p)?;
        if !holds(b, *p, m, 1 + 8 * 8) {
            return None;
        }
        let mut grades = Vec::with_capacity(m);
        for _ in 0..m {
            let grade = take_u8(b, p)?;
            let [cells, round_kept, held_z2, refused, finite, affine, infinite, dual_frames] = take_u64s::<8>(b, p)?;
            grades.push(GradeProgress { grade, cells, round_kept, held_z2, refused, finite, affine, infinite, dual_frames });
        }
        Some(Progress { percepts, contact_cycles, contact_cycles_curved, contact_cycles_unread, grades })
    }
}
