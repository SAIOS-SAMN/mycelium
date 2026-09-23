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
use crate::observe::{
    CellClosure, CellCoherence, CellDual, CellEntry, CellOrbit, CellPermutes, CellProduct, CellRound, Cells,
    GradeProgress, Progress, Spend, Viscosity,
};
use crate::transducer::cech::{CellAlgebra, Closure, Coherence, Congruence, CoxeterReading, Permutes, Refusal};
use crate::transducer::cga::{self, ConstructorOrbit, DualFrame, MirrorReading, Mv};
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
            round: match a.congruence {
                Congruence::Congruent => CellRound::Kept,
                Congruence::HeldZ2 { .. } => CellRound::HeldZ2,
                Congruence::Incongruent { .. } => CellRound::Carried,
                Congruence::Unread { .. } => CellRound::Unread,
                Congruence::NoRound => CellRound::NoRound,
                Congruence::NoCircuit => CellRound::NoCircuit,
            },
            permutes: match a.permutes {
                Permutes::All => CellPermutes::All,
                Permutes::Moves(_) => CellPermutes::Moves,
                Permutes::Unread { .. } => CellPermutes::Unread,
                Permutes::NoConstructors => CellPermutes::NoConstructors,
                Permutes::NoCircuit => CellPermutes::NoCircuit,
            },
            coherence: match a.coherence {
                Coherence::Closes(_) => CellCoherence::Closes,
                Coherence::Held => CellCoherence::Held,
                Coherence::Open(_) => CellCoherence::Open,
                Coherence::Unread => CellCoherence::Unread,
            },
            capacity: a.capacity.map(|k| k as u32),
            orbit: match a.constructors {
                ConstructorOrbit::Closed { .. } => CellOrbit::Closed,
                ConstructorOrbit::Open { .. } => CellOrbit::Open,
                ConstructorOrbit::Unbounded { .. } => CellOrbit::Unbounded,
                ConstructorOrbit::Unread => CellOrbit::Unread,
            },
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

/// PG1 — THE PERCEPT'S PROGRESS toward E1's done-when: the circuits that
/// cross a contact face, those whose whole transport reads and is not
/// trivial, those unread (OR3); and the certified cells by grade, off the
/// typed cells.
pub(crate) fn progress(form: &Form, walked: &[crate::transducer::sentence::WalkedCircuit], cells: &Cells) -> Progress {
    use crate::network::nerve::coord::BivectorKind;
    let mut p = Progress { percepts: 1, ..Progress::default() };
    for (steps, composite) in walked {
        let contact =
            steps.iter().any(|&(f, _)| form.faces.get(usize::from(f)).is_some_and(|x| !x.interface.is_empty()));
        if !contact {
            continue;
        }
        p.contact_cycles += 1;
        match composite.as_ref().map(|m| m.class_invariants()) {
            None => p.contact_cycles_unread += 1,
            Some(Some((_, _, BivectorKind::Trivial))) => {}
            Some(Some(_)) => p.contact_cycles_curved += 1,
            // A composite whose class does not read is unread, never flat.
            Some(None) => p.contact_cycles_unread += 1,
        }
    }
    for c in &cells.cells {
        let at = match p.grades.binary_search_by_key(&c.grade, |g| g.grade) {
            Ok(at) => at,
            Err(at) => {
                p.grades.insert(at, GradeProgress { grade: c.grade, ..GradeProgress::default() });
                at
            }
        };
        let g = &mut p.grades[at];
        g.cells += 1;
        match c.round {
            CellRound::Kept => g.round_kept += 1,
            CellRound::HeldZ2 => g.held_z2 += 1,
            CellRound::Carried => g.refused += 1,
            CellRound::Unread | CellRound::NoRound | CellRound::NoCircuit => {}
        }
        match c.coxeter {
            Some(CellClosure::FiniteByTheorem | CellClosure::FiniteByCheck) => g.finite += 1,
            Some(CellClosure::Affine) => g.affine += 1,
            Some(CellClosure::InfiniteByPair | CellClosure::Hyperbolic | CellClosure::InfiniteByProduct) => {
                g.infinite += 1
            }
            Some(CellClosure::Unbounded | CellClosure::Unread) | None => {}
        }
        if c.dual == Some(CellDual::Frame) {
            g.dual_frames += 1;
        }
    }
    p
}

/// THE PERCEPT'S READERS AFTER ITS FORM, each held typed for the observer
/// and none of it printed but the orbits' lines (RC2): each kept orbit
/// (OR1), each certified cell as its orbit algebra off the circuits this
/// percept walked (CG1, CO1), the circuits step by step (PL1), each held
/// loop's faces bent per pair of planes per lane (VS1, VO1), and the
/// percept's progress (PG1). SP1 — each reader's spend is the stroke's own
/// count before and after it, never a clock (law 9); perception's spend and
/// the frames it left unreached are the caller's.
pub(crate) fn read_and_hold(form: &Form, budget: &mut crate::physics::Budget, perceived: u64, unreached: u64) {
    use crate::transducer::cga::Undecomposed;
    use crate::transducer::sentence::{ModesRead, OrbitLine};
    let mut spend = Spend { percepts: 1, ..Spend::default() };
    spend.reads[0] = perceived;
    spend.unbounded[0] = unreached;
    let at = budget.remaining();
    for orbit in crate::transducer::sentence::take_orbits(budget) {
        if let OrbitLine::Read(r) = &orbit {
            if let ModesRead::Undecomposed(Undecomposed::Unbounded(_)) = r.modes {
                spend.unbounded[1] += 1;
            }
        }
        eprintln!("  {}", orbit.line());
    }
    spend.reads[1] = (at - budget.remaining()) as u64;
    let at = budget.remaining();
    let walked = crate::transducer::sentence::walked_circuits();
    let cells = cells_reading(&comprehend(form, &walked, budget));
    spend.reads[2] = (at - budget.remaining()) as u64;
    spend.unbounded[2] = cells_unbounded(&cells);
    let at = budget.remaining();
    spend.unbounded[3] = crate::transducer::sentence::finish_circuits(budget);
    spend.reads[3] = (at - budget.remaining()) as u64;
    let at = budget.remaining();
    let viscosity = crate::transducer::sentence::read_viscosity(form, budget);
    spend.reads[4] = (at - budget.remaining()) as u64;
    spend.unbounded[4] = viscosity.iter().map(|p| p.unread).sum();
    let progress = progress(form, &walked, &cells);
    hold(cells, crate::transducer::sentence::viscosity_reading(&viscosity), spend, progress);
}

/// The readings a cell's comprehension left unbounded by a spent stroke: a
/// group whose closure the check did not finish, an orbit of constructors
/// not closed within the stroke.
pub(crate) fn cells_unbounded(cells: &Cells) -> u64 {
    cells
        .cells
        .iter()
        .map(|c| {
            u64::from(c.coxeter == Some(CellClosure::Unbounded))
                + u64::from(c.polars == Some(CellClosure::Unbounded))
                + u64::from(c.orbit == CellOrbit::Unbounded)
        })
        .sum::<u64>()
        + u64::from(cells.mirror_group == Some(CellClosure::Unbounded))
}

/// THE PERCEPT'S TYPED READINGS, held until the observer takes them: the
/// last percept's cells (each percept's replaces the last), and the
/// viscosity, spend and progress summed over every percept since the last
/// take (78.16, RC2 — the percept prints none of them).
#[derive(Default)]
struct Held {
    cells: Option<Cells>,
    viscosity: Option<Viscosity>,
    spend: Option<Spend>,
    progress: Option<Progress>,
}

thread_local! {
    static HELD: std::cell::RefCell<Held> = std::cell::RefCell::new(Held::default());
}

/// Hold one percept's readings for the observer.
pub(crate) fn hold(cells: Cells, viscosity: Viscosity, spend: Spend, progress: Progress) {
    HELD.with(|h| {
        let mut h = h.borrow_mut();
        h.cells = Some(cells);
        match h.viscosity.as_mut() {
            Some(v) => v.absorb(&viscosity),
            None => h.viscosity = Some(viscosity),
        }
        match h.spend.as_mut() {
            Some(s) => s.absorb(&spend),
            None => h.spend = Some(spend),
        }
        match h.progress.as_mut() {
            Some(p) => p.absorb(&progress),
            None => h.progress = Some(progress),
        }
    });
}

/// The last percept's cells, and stop holding them. `None` where no percept
/// read any since the last take — a reading not taken, never an empty one.
pub fn take_cells() -> Option<Cells> {
    HELD.with(|h| h.borrow_mut().cells.take())
}

/// VO1 — the viscosity read since the last take.
pub fn take_viscosity() -> Option<Viscosity> {
    HELD.with(|h| h.borrow_mut().viscosity.take())
}

/// SP1 — the stroke by reader since the last take.
pub fn take_spend() -> Option<Spend> {
    HELD.with(|h| h.borrow_mut().spend.take())
}

/// PG1 — the progress read since the last take.
pub fn take_progress() -> Option<Progress> {
    HELD.with(|h| h.borrow_mut().progress.take())
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

