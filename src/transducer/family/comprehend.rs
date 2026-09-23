// 78.11, 78.14, CG1, CG1b, GR1, BD2 — EACH CERTIFIED CELL OF A FORM READ AS
// ITS ORBIT ALGEBRA: the circuits that cross it, their holonomies through the
// one composite reader (OR3), the cell's round, constructors and Gram form —
// and the census's reading of them by grade.
//
//! A circuit CROSSES a cell where one of its steps crosses a contact face
//! between two different bodies of the cell: it passes through the cell's
//! contact, where its transport meets the cell's round (78.13(3)). The
//! step's bodies are read off the face it crosses — the form's side carrier
//! of each contact face's body pair (BD2) — never derived from its member,
//! whose support can fall into several bodies (checklist (m)). One keyed
//! read per step, never a pair walk.

use crate::network::nerve::Keyed;
use crate::transducer::cech::{CellAlgebra, Coherence, Congruence, Permutes};
use crate::transducer::cga::{ConstructorOrbit, Mv, PairRegime};
use crate::transducer::sentence::Form;

/// EVERY CERTIFIED CELL OF `form`, READ AS ITS ORBIT ALGEBRA, under the
/// stroke's budget, in the cells' own key order. The circuits are the ones
/// the percept's own walk kept — `(faces crossed, composite)` — never walked
/// again.
pub(crate) fn cell_algebras(
    form: &Form,
    walked: &[crate::transducer::sentence::WalkedCircuit],
    budget: &mut crate::physics::Budget,
) -> Vec<CellAlgebra> {
    form.cells
        .scoped()
        .map(|(_, cell)| {
            let mut body_of: Keyed<usize> = Keyed::new();
            for (b, piece) in cell.bodies.iter().enumerate() {
                body_of.insert(piece, b);
            }
            // A step's crossing of the cell: the body it leaves and the body
            // it enters, off the face's oriented pair and the step's way.
            let crossing = |face: u16, forward: bool| -> Option<(usize, usize)> {
                let (x, y) = form.face_bodies.get(&u32::from(face).to_be_bytes())?;
                let (a, b) = (*body_of.get(x)?, *body_of.get(y)?);
                match (a == b, forward) {
                    (true, _) => None,
                    (false, true) => Some((a, b)),
                    (false, false) => Some((b, a)),
                }
            };
            let mut circuits: Vec<Option<Mv>> = Vec::new();
            let mut orders: Vec<Option<Vec<usize>>> = Vec::new();
            for (steps, composite) in walked {
                // CX1 — the circuit's order of the cell's bodies: each body
                // as the circuit's crossings first reach it; `None` where the
                // circuit does not reach every body of the cell.
                let mut order: Vec<usize> = Vec::new();
                for &(face, forward) in steps {
                    if let Some((from, to)) = crossing(face, forward) {
                        for b in [from, to] {
                            if !order.contains(&b) {
                                order.push(b);
                            }
                        }
                    }
                }
                if order.is_empty() {
                    continue;
                }
                circuits.push(composite.clone());
                orders.push((order.len() == cell.bodies.len()).then_some(order));
            }
            crate::transducer::cech::read_cell(
                cell.grade(),
                cell.meet().as_ref(),
                cell.nuclei.as_deref(),
                &circuits,
                &orders,
                budget,
            )
        })
        .collect()
}

/// ONE GRADE'S ROW OF THE CELLS' READING.
#[derive(Default)]
struct GradeRow {
    grade: usize,
    /// congruent, held ℤ₂, incongruent, unread, no round, no circuit.
    congruence: [u64; 6],
    /// all, moves, unread, no constructors, no circuit.
    permutes: [u64; 5],
    /// closes, held, open, unread.
    coherence: [u64; 4],
    capacity: Vec<usize>,
    /// closed, open, unbounded, unread.
    constructors: [u64; 4],
    /// The span signatures `(p, q, z)` read, with the vector rank.
    signatures: Vec<((usize, usize, usize), usize)>,
    /// Pairs by regime: finite, parabolic, boost, rosette, null.
    pairs: [u64; 5],
    /// GR2, by closure: finite by the theorem, finite by the check, infinite
    /// by a pair, affine, hyperbolic, by a product; unbounded; unread — the
    /// nuclei as mirrors, then the polars as mirrors.
    coxeter: [u64; 8],
    polars: [u64; 8],
    /// DF1: a dual frame, redundant, degenerate.
    dual: [u64; 3],
    /// CX1: per cell, the products kept and their distinct classes.
    products: Vec<(usize, usize)>,
}

fn closure_axis(r: &Option<crate::transducer::cech::CoxeterReading>) -> usize {
    use crate::transducer::cech::{Closure, Refusal};
    match r.as_ref().map(|r| &r.closure) {
        Some(Closure::Finite { by_theorem: true, .. }) => 0,
        Some(Closure::Finite { by_theorem: false, .. }) => 1,
        Some(Closure::Infinite(Refusal::Pair(..))) => 2,
        Some(Closure::Infinite(Refusal::Affine(_))) => 3,
        Some(Closure::Infinite(Refusal::Hyperbolic(_))) => 4,
        Some(Closure::Infinite(Refusal::Product { .. })) => 5,
        Some(Closure::Unbounded { .. }) => 6,
        Some(Closure::Unread) | None => 7,
    }
}

/// THE CELLS' READING BY GRADE, one row per grade, ascending.
#[derive(Default)]
pub(crate) struct CellTally {
    rows: Vec<GradeRow>,
}

impl CellTally {
    pub(crate) fn of(cells: &[CellAlgebra]) -> CellTally {
        let mut t = CellTally::default();
        for c in cells {
            let at = match t.rows.binary_search_by_key(&c.grade, |r| r.grade) {
                Ok(at) => at,
                Err(at) => {
                    t.rows.insert(at, GradeRow { grade: c.grade, ..GradeRow::default() });
                    at
                }
            };
            let row = &mut t.rows[at];
            row.congruence[match c.congruence {
                Congruence::Congruent => 0,
                Congruence::HeldZ2 { .. } => 1,
                Congruence::Incongruent { .. } => 2,
                Congruence::Unread { .. } => 3,
                Congruence::NoRound => 4,
                Congruence::NoCircuit => 5,
            }] += 1;
            row.permutes[match c.permutes {
                Permutes::All => 0,
                Permutes::Moves(_) => 1,
                Permutes::Unread { .. } => 2,
                Permutes::NoConstructors => 3,
                Permutes::NoCircuit => 4,
            }] += 1;
            row.coherence[match c.coherence {
                Coherence::Closes(_) => 0,
                Coherence::Held => 1,
                Coherence::Open(_) => 2,
                Coherence::Unread => 3,
            }] += 1;
            if let Some(k) = c.capacity {
                row.capacity.push(k);
            }
            row.constructors[match c.constructors {
                ConstructorOrbit::Closed { .. } => 0,
                ConstructorOrbit::Open { .. } => 1,
                ConstructorOrbit::Unbounded { .. } => 2,
                ConstructorOrbit::Unread => 3,
            }] += 1;
            row.coxeter[closure_axis(&c.coxeter)] += 1;
            if c.polars.is_some() {
                row.polars[closure_axis(&c.polars)] += 1;
            }
            match &c.dual {
                Some(crate::transducer::cga::DualFrame::Frame(_)) => row.dual[0] += 1,
                Some(crate::transducer::cga::DualFrame::Redundant(_)) => row.dual[1] += 1,
                Some(crate::transducer::cga::DualFrame::Degenerate) => row.dual[2] += 1,
                None => {}
            }
            row.products.push((c.products.len(), c.classes));
            if let Some(g) = &c.gram {
                row.signatures.push((g.signature, g.rank));
                for (_, _, _, r) in &g.pairs {
                    row.pairs[match r {
                        PairRegime::Finite(_) => 0,
                        PairRegime::Parabolic => 1,
                        PairRegime::Boost => 2,
                        PairRegime::Rosette => 3,
                        PairRegime::Null => 4,
                    }] += 1;
                }
            }
        }
        t
    }

    /// The census line's segment.
    pub(crate) fn line(&self) -> String {
        let rows: Vec<String> = self
            .rows
            .iter()
            .map(|r| {
                let (cg, pm, ch, co, pr) = (r.congruence, r.permutes, r.coherence, r.constructors, r.pairs);
                let axes = |x: &[u64; 8]| {
                    format!(
                        "finite(theorem) {} finite(check) {} infinite(pair) {} affine {} hyperbolic {} infinite(product) {} unbounded {} unread {}",
                        x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]
                    )
                };
                format!(
                    "grade {}: round kept {} held-z2 {} carried {} unread {} no-round {} no-circuit {} | constructors kept {} moved {} unread {} | coherence closes {} held {} open {} unread {} | capacity {:?} | orbit closed {} open {} unbounded {} unread {} | gram (sig, rank) {:?} pairs finite {} parabolic {} boost {} rosette {} null {} | coxeter {} | polars {} | dual frame {} redundant {} degenerate {} | products (kept, classes) {:?}",
                    r.grade, cg[0], cg[1], cg[2], cg[3], cg[4], cg[5], pm[0], pm[1], pm[2], ch[0], ch[1], ch[2], ch[3],
                    r.capacity, co[0], co[1], co[2], co[3], r.signatures, pr[0], pr[1], pr[2], pr[3], pr[4],
                    axes(&r.coxeter), axes(&r.polars), r.dual[0], r.dual[1], r.dual[2], r.products
                )
            })
            .collect();
        match rows.is_empty() {
            true => "cells: none certified".into(),
            false => format!("cells: {}", rows.join(" ; ")),
        }
    }
}
