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
//!
//! MG1 — each nucleus is read against the base's mirrors that bracket its
//! centre, and the form's mirrors — every distinct one bracketing any of its
//! cells' nuclei — are read as a reflection group by GR2: the grid's own
//! Coxeter reading beside the cells'. CO1 — the whole is a typed reading
//! the observer serves on demand ([`take_cells`]).

use crate::network::nerve::Keyed;
use crate::observe::{CellClosure, CellDual, CellEntry, CellProduct, Cells};
use crate::transducer::cech::{CellAlgebra, Closure, Coherence, Congruence, CoxeterReading, Permutes, Refusal};
use crate::transducer::cga::{self, ConstructorOrbit, DualFrame, MirrorReading, Mv, PairRegime};
use crate::transducer::sentence::Form;

/// ONE FORM'S CELLS, COMPREHENDED: each cell's orbit algebra, each cell's
/// nuclei against their bracketing mirrors (MG1), and the form's mirrors
/// read as a group.
pub(crate) struct Comprehension {
    pub(crate) algebras: Vec<CellAlgebra>,
    pub(crate) mirrors: Vec<Vec<Vec<MirrorReading>>>,
    pub(crate) planes: usize,
    pub(crate) mirror_group: Option<CoxeterReading>,
}

/// COMPREHEND A FORM'S CELLS (CG1, MG1), under the stroke's budget.
pub(crate) fn comprehend(
    form: &Form,
    walked: &[crate::transducer::sentence::WalkedCircuit],
    budget: &mut crate::physics::Budget,
) -> Comprehension {
    let algebras = cell_algebras(form, walked, budget);
    let n = form.n;
    let mirrors: Vec<Vec<Vec<MirrorReading>>> = form
        .cells
        .scoped()
        .map(|(_, c)| c.nuclei.iter().flatten().map(|s| cga::mirrors_between(s, n)).collect())
        .collect();
    // The form's mirrors: every distinct one bracketing a nucleus, keyed by
    // its own coordinates (axis, offset).
    let mut planes: Keyed<Mv> = Keyed::new();
    for m in mirrors.iter().flatten().flatten() {
        let mut key = vec![m.axis as u8];
        crate::transducer::address::put_bytes(&mut key, &m.k.numer().to_signed_bytes_le());
        crate::transducer::address::put_bytes(&mut key, &m.k.denom().to_signed_bytes_le());
        if !planes.contains(&key) {
            planes.insert(&key, cga::base_mirror(m.axis, &m.k));
        }
    }
    let plane_list: Vec<Mv> = planes.scoped().map(|(_, p)| p.clone()).collect();
    let mirror_group = match plane_list.is_empty() {
        true => None,
        false => Some(crate::transducer::cech::read_coxeter(&plane_list, &[], budget)),
    };
    Comprehension { algebras, mirrors, planes: plane_list.len(), mirror_group }
}

fn cell_closure(r: &CoxeterReading) -> CellClosure {
    match &r.closure {
        Closure::Finite { by_theorem: true, .. } => CellClosure::FiniteByTheorem,
        Closure::Finite { by_theorem: false, .. } => CellClosure::FiniteByCheck,
        Closure::Infinite(Refusal::Pair(..)) => CellClosure::InfiniteByPair,
        Closure::Infinite(Refusal::Affine(_)) => CellClosure::Affine,
        Closure::Infinite(Refusal::Hyperbolic(_)) => CellClosure::Hyperbolic,
        Closure::Infinite(Refusal::Product { .. }) => CellClosure::InfiniteByProduct,
        Closure::Unbounded { .. } => CellClosure::Unbounded,
        Closure::Unread => CellClosure::Unread,
    }
}

/// CO1 — THE TYPED READING THE OBSERVER SERVES: every cell as its algebra
/// read it, SY1's blade per product in its canonical coordinates.
pub(crate) fn cells_reading(c: &Comprehension) -> Cells {
    let ray = |m: &Mv| -> Vec<(u16, num_rational::BigRational)> {
        match m.canonical() {
            Some(r) => (0..r.slot_count())
                .filter(|&b| !num_traits::Zero::is_zero(r.coeff(b)))
                .map(|b| (b as u16, r.coeff(b).clone()))
                .collect(),
            None => Vec::new(),
        }
    };
    let cells = c
        .algebras
        .iter()
        .zip(&c.mirrors)
        .map(|(a, ms)| CellEntry {
            grade: a.grade as u8,
            signature: a.gram.as_ref().map(|g| ((g.signature.0 as u8, g.signature.1 as u8, g.signature.2 as u8), g.rank as u8)),
            pairs: a.gram.iter().flat_map(|g| g.pairs.iter().map(|(_, _, _, r)| *r)).collect(),
            coxeter: a.coxeter.as_ref().map(cell_closure),
            polars: a.polars.as_ref().map(cell_closure),
            dual: a.dual.as_ref().map(|d| match d {
                DualFrame::Frame(_) => CellDual::Frame,
                DualFrame::Redundant(_) => CellDual::Redundant,
                DualFrame::Degenerate => CellDual::Degenerate,
            }),
            products: a
                .products
                .iter()
                .map(|p| CellProduct {
                    order: p.order.iter().map(|&i| i as u8).collect(),
                    grades: p.grades.iter().map(|&g| g as u8).collect(),
                    det: p.det,
                    classed: crate::transducer::cech::product_class(p).is_some(),
                    seal: ray(&p.seal),
                })
                .collect(),
            classes: a.classes as u8,
            mirrors: ms
                .iter()
                .map(|n| n.iter().map(|m| (m.axis as u8, m.k.clone(), m.delta2.clone(), m.regime)).collect())
                .collect(),
        })
        .collect();
    Cells { mirror_planes: c.planes as u32, mirror_group: c.mirror_group.as_ref().map(cell_closure), cells }
}

thread_local! {
    /// The last percept's cells, read whole, until the observer takes them.
    static LAST: std::cell::RefCell<Option<Cells>> = const { std::cell::RefCell::new(None) };
}

/// Hold the last percept's cells for the observer.
pub(crate) fn hold_cells(cells: Cells) {
    LAST.with(|l| *l.borrow_mut() = Some(cells));
}

/// The last percept's cells, and stop holding them. `None` where no percept
/// read any since the last take — a reading not taken, never an empty one.
pub fn take_cells() -> Option<Cells> {
    LAST.with(|l| l.borrow_mut().take())
}

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
    /// MG1: each nucleus against each bracketing mirror — through its
    /// centre, cutting it, tangent, apart.
    mirrors: [u64; 4],
}

/// The tally's axis for a group's closure, off the one mapping
/// (`cell_closure`); an unread reading on the last axis.
fn closure_axis(r: &Option<CoxeterReading>) -> usize {
    match r.as_ref().map(cell_closure) {
        Some(CellClosure::FiniteByTheorem) => 0,
        Some(CellClosure::FiniteByCheck) => 1,
        Some(CellClosure::InfiniteByPair) => 2,
        Some(CellClosure::Affine) => 3,
        Some(CellClosure::Hyperbolic) => 4,
        Some(CellClosure::InfiniteByProduct) => 5,
        Some(CellClosure::Unbounded) => 6,
        Some(CellClosure::Unread) | None => 7,
    }
}

/// THE CELLS' READING BY GRADE, one row per grade, ascending, and the
/// form's mirrors.
#[derive(Default)]
pub(crate) struct CellTally {
    rows: Vec<GradeRow>,
    planes: usize,
    mirror_group: Option<CellClosure>,
}

impl CellTally {
    pub(crate) fn of(comprehension: &Comprehension) -> CellTally {
        let mut t = CellTally {
            planes: comprehension.planes,
            mirror_group: comprehension.mirror_group.as_ref().map(cell_closure),
            ..CellTally::default()
        };
        for (c, nuclei) in comprehension.algebras.iter().zip(&comprehension.mirrors) {
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
            for m in nuclei.iter().flatten() {
                row.mirrors[match m.regime {
                    PairRegime::Finite(2) => 0,
                    PairRegime::Parabolic => 2,
                    PairRegime::Boost => 3,
                    _ => 1,
                }] += 1;
            }
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
                    "grade {}: round kept {} held-z2 {} carried {} unread {} no-round {} no-circuit {} | constructors kept {} moved {} unread {} | coherence closes {} held {} open {} unread {} | capacity {:?} | orbit closed {} open {} unbounded {} unread {} | gram (sig, rank) {:?} pairs finite {} parabolic {} boost {} rosette {} null {} | coxeter {} | polars {} | dual frame {} redundant {} degenerate {} | products (kept, classes) {:?} | nuclei on mirrors: through {} cut {} tangent {} apart {}",
                    r.grade, cg[0], cg[1], cg[2], cg[3], cg[4], cg[5], pm[0], pm[1], pm[2], ch[0], ch[1], ch[2], ch[3],
                    r.capacity, co[0], co[1], co[2], co[3], r.signatures, pr[0], pr[1], pr[2], pr[3], pr[4],
                    axes(&r.coxeter), axes(&r.polars), r.dual[0], r.dual[1], r.dual[2], r.products,
                    r.mirrors[0], r.mirrors[1], r.mirrors[2], r.mirrors[3]
                )
            })
            .collect();
        let mirrors = match self.mirror_group {
            Some(g) => format!("mirrors: {} planes, group {}", self.planes, g.label()),
            None => "mirrors: none".into(),
        };
        match rows.is_empty() {
            true => "cells: none certified".into(),
            false => format!("cells: {} | {mirrors}", rows.join(" ; ")),
        }
    }
}
