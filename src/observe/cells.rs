// 78.14, 78.15, CO1 — THE CELLS OF THE LAST PERCEPT, AS THEIR ORBIT ALGEBRAS
// READ THEM: per cell its grade, its constructors' Gram form and pairs, its
// Coxeter reading with the nuclei and with the polars as mirrors, its dual
// frame, the products kept per circuit with each sealing step's blade, and
// each nucleus against the base's mirrors it stands between (MG1) — served
// to the console on demand, never printed per epoch.

use super::circuit::{holds, put_int, take_int, take_u16, take_u32, take_u8};
use crate::transducer::cga::PairRegime;
use num_rational::BigRational;

/// HOW A SET OF CONSTRUCTORS' GROUP CLOSES (GR2), by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellClosure {
    FiniteByTheorem,
    FiniteByCheck,
    InfiniteByPair,
    Affine,
    Hyperbolic,
    InfiniteByProduct,
    Unbounded,
    Unread,
}

impl CellClosure {
    const ALL: [CellClosure; 8] = [
        CellClosure::FiniteByTheorem,
        CellClosure::FiniteByCheck,
        CellClosure::InfiniteByPair,
        CellClosure::Affine,
        CellClosure::Hyperbolic,
        CellClosure::InfiniteByProduct,
        CellClosure::Unbounded,
        CellClosure::Unread,
    ];

    /// The name the console prints.
    pub fn label(self) -> &'static str {
        match self {
            CellClosure::FiniteByTheorem => "finite (theorem)",
            CellClosure::FiniteByCheck => "finite (check)",
            CellClosure::InfiniteByPair => "infinite (pair)",
            CellClosure::Affine => "affine",
            CellClosure::Hyperbolic => "hyperbolic",
            CellClosure::InfiniteByProduct => "infinite (product)",
            CellClosure::Unbounded => "unbounded",
            CellClosure::Unread => "unread",
        }
    }

    fn code(self) -> u8 {
        match self {
            CellClosure::FiniteByTheorem => 0,
            CellClosure::FiniteByCheck => 1,
            CellClosure::InfiniteByPair => 2,
            CellClosure::Affine => 3,
            CellClosure::Hyperbolic => 4,
            CellClosure::InfiniteByProduct => 5,
            CellClosure::Unbounded => 6,
            CellClosure::Unread => 7,
        }
    }

    fn of(b: u8) -> Option<CellClosure> {
        Self::ALL.get(usize::from(b)).copied()
    }
}

/// A coded axis's byte: its place in the axis's own list of arms.
fn code_in<T: PartialEq + Copy>(all: &[T], x: T) -> u8 {
    all.iter().position(|&a| a == x).map_or(NONE, |i| i as u8)
}

fn take_coded<T: Copy>(all: &[T], b: &[u8], p: &mut usize) -> Option<T> {
    all.get(usize::from(take_u8(b, p)?)).copied()
}

/// HOW THE CELL'S ROUND FARES UNDER ITS CIRCUITS (CG1): kept by every
/// circuit and mode, reversed (held ℤ₂), carried off by a mode ray (the
/// refusal at this grade), unread, no round, no circuit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellRound {
    Kept,
    HeldZ2,
    Carried,
    Unread,
    NoRound,
    NoCircuit,
}

impl CellRound {
    pub const ALL: [CellRound; 6] =
        [CellRound::Kept, CellRound::HeldZ2, CellRound::Carried, CellRound::Unread, CellRound::NoRound, CellRound::NoCircuit];
}

/// WHETHER THE CIRCUITS KEEP THE CELL'S CONSTRUCTORS (CG1b).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellPermutes {
    All,
    Moves,
    Unread,
    NoConstructors,
    NoCircuit,
}

impl CellPermutes {
    pub const ALL: [CellPermutes; 5] = [
        CellPermutes::All,
        CellPermutes::Moves,
        CellPermutes::Unread,
        CellPermutes::NoConstructors,
        CellPermutes::NoCircuit,
    ];
}

/// WHETHER THE CIRCUITS' ALGEBRA CLOSES ON THE CELL (CG1b).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellCoherence {
    Closes,
    Held,
    Open,
    Unread,
}

impl CellCoherence {
    pub const ALL: [CellCoherence; 4] =
        [CellCoherence::Closes, CellCoherence::Held, CellCoherence::Open, CellCoherence::Unread];
}

/// HOW THE CONSTRUCTORS' ORBIT UNDER THE CIRCUITS CLOSES (CG1b).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellOrbit {
    Closed,
    Open,
    Unbounded,
    Unread,
}

impl CellOrbit {
    pub const ALL: [CellOrbit; 4] = [CellOrbit::Closed, CellOrbit::Open, CellOrbit::Unbounded, CellOrbit::Unread];
}

/// EN1 — A CELL'S ENCLOSURE: the round its bodies' centres join is FLAT (no
/// round, nothing to be inside), or a round against which every other body
/// of the form stands — inside, on, apart, wrapping it, unread — with the
/// cell whose enclosure wraps it most closely (the parse, 50.10).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellEnclosure {
    Flat,
    Round { bodies: [u32; 5], within: Option<u32> },
}

/// WHETHER A CELL HAS A DUAL FRAME (DF1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellDual {
    Frame,
    Redundant,
    Degenerate,
}

/// ONE PRODUCT OF A CELL'S CONSTRUCTORS IN A CIRCUIT'S ORDER (CX1): the
/// order, the grades it holds, the action's determinant, whether its class
/// was read, and SY1's blade — what the sealing step adds — in its canonical
/// coordinates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellProduct {
    pub order: Vec<u8>,
    pub grades: Vec<u8>,
    pub det: Option<i8>,
    pub classed: bool,
    pub seal: Vec<(u16, BigRational)>,
}

/// ONE CELL.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellEntry {
    pub grade: u8,
    pub round: CellRound,
    pub permutes: CellPermutes,
    pub coherence: CellCoherence,
    /// The circuits' bracket-closed algebra's rank; `None` where unread.
    pub capacity: Option<u32>,
    pub orbit: CellOrbit,
    /// The Gram form's span signature `(p, q, z)` and the vector rank;
    /// `None` where the nuclei did not read.
    pub signature: Option<((u8, u8, u8), u8)>,
    /// Each pair's regime, in the Gram form's pair order.
    pub pairs: Vec<PairRegime>,
    pub coxeter: Option<CellClosure>,
    pub polars: Option<CellClosure>,
    pub dual: Option<CellDual>,
    pub products: Vec<CellProduct>,
    pub classes: u8,
    /// MG1 — per nucleus, each bracketing mirror: `(axis, k, δ², regime)`.
    pub mirrors: Vec<Vec<(u8, BigRational, BigRational, PairRegime)>>,
    /// EN1 — its enclosure; `None` where its nuclei did not read.
    pub enclosure: Option<CellEnclosure>,
}

/// THE CELLS OF THE LAST PERCEPT, and the form's own mirror group (MG1): how
/// many distinct base mirrors bracket its nuclei, and how their group closes.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Cells {
    pub mirror_planes: u32,
    pub mirror_group: Option<CellClosure>,
    pub cells: Vec<CellEntry>,
}

const NONE: u8 = 0xff;

fn put_q(out: &mut Vec<u8>, q: &BigRational) {
    put_int(out, q.numer());
    put_int(out, q.denom());
}

fn take_q(b: &[u8], p: &mut usize) -> Option<BigRational> {
    let (n, d) = (take_int(b, p)?, take_int(b, p)?);
    (!num_traits::Zero::is_zero(&d)).then(|| BigRational::new(n, d))
}

fn put_regime(out: &mut Vec<u8>, r: PairRegime) {
    out.push(match r {
        PairRegime::Finite(m) => m,
        PairRegime::Parabolic => 10,
        PairRegime::Boost => 11,
        PairRegime::Rosette => 12,
        PairRegime::Null => 13,
    });
}

fn take_regime(b: &[u8], p: &mut usize) -> Option<PairRegime> {
    match take_u8(b, p)? {
        m @ (2 | 3 | 4 | 6) => Some(PairRegime::Finite(m)),
        10 => Some(PairRegime::Parabolic),
        11 => Some(PairRegime::Boost),
        12 => Some(PairRegime::Rosette),
        13 => Some(PairRegime::Null),
        _ => None,
    }
}

fn put_closure(out: &mut Vec<u8>, c: Option<CellClosure>) {
    out.push(match c {
        Some(c) => c.code(),
        None => NONE,
    });
}

fn take_closure(b: &[u8], p: &mut usize) -> Option<Option<CellClosure>> {
    match take_u8(b, p)? {
        NONE => Some(None),
        c => Some(Some(CellClosure::of(c)?)),
    }
}

fn put_bytes(out: &mut Vec<u8>, v: &[u8]) {
    out.extend_from_slice(&(v.len() as u32).to_le_bytes());
    out.extend_from_slice(v);
}

fn take_bytes(b: &[u8], p: &mut usize) -> Option<Vec<u8>> {
    let len = take_u32(b, p)?;
    let v = b.get(*p..p.checked_add(len)?)?.to_vec();
    *p += len;
    Some(v)
}

impl Cells {
    pub(crate) fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.mirror_planes.to_le_bytes());
        put_closure(out, self.mirror_group);
        out.extend_from_slice(&(self.cells.len() as u32).to_le_bytes());
        for c in &self.cells {
            out.push(c.grade);
            out.push(code_in(&CellRound::ALL, c.round));
            out.push(code_in(&CellPermutes::ALL, c.permutes));
            out.push(code_in(&CellCoherence::ALL, c.coherence));
            match c.capacity {
                Some(k) => {
                    out.push(1);
                    out.extend_from_slice(&k.to_le_bytes());
                }
                None => out.push(0),
            }
            out.push(code_in(&CellOrbit::ALL, c.orbit));
            match c.signature {
                Some(((p, q, z), r)) => out.extend_from_slice(&[1, p, q, z, r]),
                None => out.push(0),
            }
            out.extend_from_slice(&(c.pairs.len() as u32).to_le_bytes());
            for r in &c.pairs {
                put_regime(out, *r);
            }
            put_closure(out, c.coxeter);
            put_closure(out, c.polars);
            out.push(match c.dual {
                Some(CellDual::Frame) => 0,
                Some(CellDual::Redundant) => 1,
                Some(CellDual::Degenerate) => 2,
                None => NONE,
            });
            out.extend_from_slice(&(c.products.len() as u32).to_le_bytes());
            for pr in &c.products {
                put_bytes(out, &pr.order);
                put_bytes(out, &pr.grades);
                out.push(match pr.det {
                    Some(1) => 1,
                    Some(_) => 2,
                    None => 0,
                });
                out.push(u8::from(pr.classed));
                out.extend_from_slice(&(pr.seal.len() as u32).to_le_bytes());
                for (blade, q) in &pr.seal {
                    out.extend_from_slice(&blade.to_le_bytes());
                    put_q(out, q);
                }
            }
            out.push(c.classes);
            out.extend_from_slice(&(c.mirrors.len() as u32).to_le_bytes());
            for ms in &c.mirrors {
                out.extend_from_slice(&(ms.len() as u32).to_le_bytes());
                for (axis, k, d2, r) in ms {
                    out.push(*axis);
                    put_q(out, k);
                    put_q(out, d2);
                    put_regime(out, *r);
                }
            }
            match &c.enclosure {
                None => out.push(NONE),
                Some(CellEnclosure::Flat) => out.push(0),
                Some(CellEnclosure::Round { bodies, within }) => {
                    out.push(1);
                    for b in bodies {
                        out.extend_from_slice(&b.to_le_bytes());
                    }
                    match within {
                        Some(w) => {
                            out.push(1);
                            out.extend_from_slice(&w.to_le_bytes());
                        }
                        None => out.push(0),
                    }
                }
            }
        }
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Cells> {
        let mirror_planes = take_u32(b, p)? as u32;
        let mirror_group = take_closure(b, p)?;
        let m = take_u32(b, p)?;
        if !holds(b, *p, m, 8) {
            return None;
        }
        let mut cells = Vec::with_capacity(m);
        for _ in 0..m {
            let grade = take_u8(b, p)?;
            let round = take_coded(&CellRound::ALL, b, p)?;
            let permutes = take_coded(&CellPermutes::ALL, b, p)?;
            let coherence = take_coded(&CellCoherence::ALL, b, p)?;
            let capacity = match take_u8(b, p)? {
                0 => None,
                1 => Some(take_u32(b, p)? as u32),
                _ => return None,
            };
            let orbit = take_coded(&CellOrbit::ALL, b, p)?;
            let signature = match take_u8(b, p)? {
                0 => None,
                1 => Some(((take_u8(b, p)?, take_u8(b, p)?, take_u8(b, p)?), take_u8(b, p)?)),
                _ => return None,
            };
            let np = take_u32(b, p)?;
            if !holds(b, *p, np, 1) {
                return None;
            }
            let pairs = (0..np).map(|_| take_regime(b, p)).collect::<Option<Vec<_>>>()?;
            let coxeter = take_closure(b, p)?;
            let polars = take_closure(b, p)?;
            let dual = match take_u8(b, p)? {
                0 => Some(CellDual::Frame),
                1 => Some(CellDual::Redundant),
                2 => Some(CellDual::Degenerate),
                NONE => None,
                _ => return None,
            };
            let nprod = take_u32(b, p)?;
            if !holds(b, *p, nprod, 14) {
                return None;
            }
            let mut products = Vec::with_capacity(nprod);
            for _ in 0..nprod {
                let order = take_bytes(b, p)?;
                let grades = take_bytes(b, p)?;
                let det = match take_u8(b, p)? {
                    0 => None,
                    1 => Some(1),
                    2 => Some(-1),
                    _ => return None,
                };
                let classed = take_u8(b, p)? == 1;
                let ns = take_u32(b, p)?;
                if !holds(b, *p, ns, 10) {
                    return None;
                }
                let mut seal = Vec::with_capacity(ns);
                for _ in 0..ns {
                    seal.push((take_u16(b, p)?, take_q(b, p)?));
                }
                products.push(CellProduct { order, grades, det, classed, seal });
            }
            let classes = take_u8(b, p)?;
            let nm = take_u32(b, p)?;
            if !holds(b, *p, nm, 4) {
                return None;
            }
            let mut mirrors = Vec::with_capacity(nm);
            for _ in 0..nm {
                let k = take_u32(b, p)?;
                if !holds(b, *p, k, 18) {
                    return None;
                }
                let mut ms = Vec::with_capacity(k);
                for _ in 0..k {
                    ms.push((take_u8(b, p)?, take_q(b, p)?, take_q(b, p)?, take_regime(b, p)?));
                }
                mirrors.push(ms);
            }
            let enclosure = match take_u8(b, p)? {
                NONE => None,
                0 => Some(CellEnclosure::Flat),
                1 => {
                    let mut bodies = [0u32; 5];
                    for v in bodies.iter_mut() {
                        *v = take_u32(b, p)? as u32;
                    }
                    let within = match take_u8(b, p)? {
                        0 => None,
                        1 => Some(take_u32(b, p)? as u32),
                        _ => return None,
                    };
                    Some(CellEnclosure::Round { bodies, within })
                }
                _ => return None,
            };
            cells.push(CellEntry {
                enclosure,
                grade,
                round,
                permutes,
                coherence,
                capacity,
                orbit,
                signature,
                pairs,
                coxeter,
                polars,
                dual,
                products,
                classes,
                mirrors,
            });
        }
        Some(Cells { mirror_planes, mirror_group, cells })
    }
}
