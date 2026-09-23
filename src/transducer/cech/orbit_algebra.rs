// 78.11, 78.12(2), CG1 — COMPREHENSION IS THE ORBIT ALGEBRA OF A CELL: every
// circuit through it kept, each read by its modes off its own action; the
// ℚ-algebra of their actions, its rank the cell's capacity; congruence per
// mode ray by commutator with the round; coherence jointly; and the
// constructors' orbit under the circuits (CS1).
//
//! No sandwich and no equality with the boundary (78.11). A cell's ROUND is
//! the meet of its bodies' nuclei; a circuit through it is a motion whose
//! modes are each an orbit on their own plane. The round is CONGRUENT with a
//! mode where the mode's ray commutes with it, `Ω_j M − M Ω_j = 0` — linear,
//! exact, no chart, no square — and superposed boundaries agree when every
//! mode ray of every circuit commutes with the round. A bracket that does
//! not vanish is a blade: the direction the round is carried, on the failing
//! mode's plane, at the cell's grade — and it IS the refusal's content.
//!
//! COHERENCE IS JOINT: the modes close together where every elliptic order
//! is finite, at their least common multiple; a parabolic or hyperbolic mode
//! holds the cell open by name; a turn of no admitted order leaves it
//! quasi-periodic, HELD, never a refusal.
//!
//! THE CAPACITY IS COMPUTED, NEVER ADDED BY HAND: the rank of the unital
//! ℚ-algebra the circuits' actions `R_c` generate on the vectors, by
//! `sparse_rank` over the flattened matrices, the products closed under the
//! stroke's budget. A cell whose circuits generate a one-dimensional algebra
//! beyond the unit is on a simple loop, and says so by rank. The COUPLING is
//! the pairs whose actions do not commute.

use crate::network::nerve::coord::BivectorKind;
use crate::transducer::cga::{self, ConstructorOrbit, MotionMode, Mv};
use num_rational::BigRational;
use num_traits::Zero;

type Q = BigRational;

/// ONE FAILING MODE: the circuit and the mode (in the reader's own order),
/// the mode's class and the planes its ray turns in, and the bracket — the
/// direction the round is carried.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Failure {
    pub circuit: usize,
    pub mode: usize,
    pub kind: BivectorKind,
    pub plane: Vec<usize>,
    pub bracket: Mv,
}

/// WHETHER THE ROUND IS KEPT BY EVERY MODE OF EVERY CIRCUIT THROUGH THE CELL.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Congruence {
    /// Every circuit read, every mode ray commutes with the round.
    Congruent,
    /// Some mode ray carries the round, at this grade: every failure kept,
    /// none chosen (checklist (r)).
    Incongruent { grade: usize, failures: Vec<Failure> },
    /// Circuits whose holonomy or modes did not read, and no read one failed.
    Unread { unread: usize },
    /// A nucleus did not read back: the cell has no round to keep.
    NoRound,
    /// No circuit crosses the cell: nothing was asked.
    NoCircuit,
}

/// WHETHER THE MODES CLOSE TOGETHER.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Coherence {
    /// Every mode elliptic of finite order: the joint period, their lcm.
    Closes(u32),
    /// Every mode elliptic, one of no admitted order: quasi-periodic, held.
    Held,
    /// A mode of this class never returns.
    Open(BivectorKind),
    /// A circuit or its modes did not read, or none crossed the cell.
    Unread,
}

/// ONE CELL'S ORBIT ALGEBRA (CG1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellAlgebra {
    pub grade: usize,
    /// Circuits crossing the cell, read or not.
    pub circuits: usize,
    /// The rank of the unital ℚ-algebra the read circuits' actions generate;
    /// `None` where none read, the budget was spent, or a coefficient did
    /// not fit the exact elimination (rule 11).
    pub capacity: Option<usize>,
    /// Pairs of read circuits whose actions do not commute.
    pub coupled: usize,
    pub congruence: Congruence,
    pub coherence: Coherence,
    /// The orbit of the cell's constructors — its nuclei — under its read
    /// circuits (CS1).
    pub constructors: ConstructorOrbit,
}

/// A matrix as one sparse integer row: its denominators cleared, reduced by
/// the gcd of what is left. `None` where an entry does not fit.
fn row_of(m: &[Vec<Q>]) -> Option<crate::core::lattice::SparseRow> {
    use num_bigint::BigInt;
    let lcm = m.iter().flatten().fold(BigInt::from(1), |l, c| num_integer::Integer::lcm(&l, c.denom()));
    let ints: Vec<BigInt> = m.iter().flatten().map(|c| (c * Q::from_integer(lcm.clone())).to_integer()).collect();
    let g = ints.iter().fold(BigInt::from(0), |g, x| num_integer::Integer::gcd(&g, x));
    let g = match g.is_zero() {
        true => BigInt::from(1),
        false => g,
    };
    let mut row = Vec::new();
    for (at, x) in ints.iter().enumerate() {
        if !x.is_zero() {
            row.push((at, i64::try_from(&(x / &g)).ok()?));
        }
    }
    Some(row)
}

/// THE CAPACITY: the rank of the unital algebra the actions generate, the
/// products closed on a keyed worklist, one read of the budget per product.
fn capacity(actions: &[Vec<Vec<Q>>], budget: &mut crate::physics::Budget) -> Option<usize> {
    let [first, ..] = actions else { return None };
    let d = first.len();
    let width = d * d;
    let mut rows = vec![row_of(&cga::identity_matrix(d))?];
    let mut rank = 1usize;
    let mut work: Vec<Vec<Vec<Q>>> = vec![cga::identity_matrix(d)];
    while let Some(b) = work.pop() {
        for a in actions {
            if !budget.spend() {
                return None;
            }
            let p = cga::matrix_mul(&b, a);
            rows.push(row_of(&p)?);
            let r = crate::core::lattice::sparse_rank(&rows, width)?;
            match r > rank {
                true => {
                    rank = r;
                    work.push(p);
                }
                false => {
                    rows.pop();
                }
            }
        }
    }
    Some(rank)
}

/// The planes a mode's ray turns in: its bivector part's non-zero blades.
fn plane_of(m: &MotionMode) -> Vec<usize> {
    let b = m.ray.grade(2);
    (0..b.slot_count()).filter(|&k| !b.coeff(k).is_zero()).collect()
}

fn lcm(a: u32, b: u32) -> u32 {
    let (mut x, mut y) = (a, b);
    while y != 0 {
        (x, y) = (y, x % y);
    }
    a / x * b
}

/// READ ONE CELL'S ORBIT ALGEBRA: its grade, its round, its constructors,
/// and the holonomy of every circuit crossing it (`None` where a circuit
/// did not read, OR3), under the stroke's budget.
pub fn read_cell(
    grade: usize,
    round: Option<&Mv>,
    nuclei: Option<&[Mv]>,
    circuits: &[Option<Mv>],
    budget: &mut crate::physics::Budget,
) -> CellAlgebra {
    let read: Vec<&Mv> = circuits.iter().flatten().collect();
    let modes: Vec<Option<Vec<MotionMode>>> = circuits
        .iter()
        .map(|h| match h.as_ref().map(|h| cga::motion_modes(h, budget)) {
            Some(Some(Ok(ms))) => Some(ms),
            _ => None,
        })
        .collect();
    let unread = modes.iter().filter(|m| m.is_none()).count();
    // Every action in the one algebra the cell's circuits span — the join of
    // their algebras, a value never narrowed — so the matrices compose.
    let n = read.iter().fold(0u8, |n, h| n.max(h.n()));
    let actions: Option<Vec<Vec<Vec<Q>>>> = read.iter().map(|h| cga::action_of(&h.widened(n))).collect();
    let capacity = actions.as_deref().and_then(|a| capacity(a, budget));
    let coupled = match &actions {
        Some(a) => (0..a.len())
            .flat_map(|i| ((i + 1)..a.len()).map(move |j| (i, j)))
            .filter(|&(i, j)| cga::matrix_mul(&a[i], &a[j]) != cga::matrix_mul(&a[j], &a[i]))
            .count(),
        None => 0,
    };
    let congruence = match (round, circuits.is_empty()) {
        (_, true) => Congruence::NoCircuit,
        (None, false) => Congruence::NoRound,
        (Some(m), false) => {
            let failures: Vec<Failure> = modes
                .iter()
                .enumerate()
                .flat_map(|(c, ms)| ms.iter().flatten().enumerate().map(move |(j, mode)| (c, j, mode)))
                .filter_map(|(c, j, mode)| {
                    let bracket = mode.ray.geo(m).sub(&m.geo(&mode.ray));
                    (!bracket.is_zero()).then(|| Failure { circuit: c, mode: j, kind: mode.kind, plane: plane_of(mode), bracket })
                })
                .collect();
            match (failures.is_empty(), unread) {
                (false, _) => Congruence::Incongruent { grade, failures },
                (true, 0) => Congruence::Congruent,
                (true, unread) => Congruence::Unread { unread },
            }
        }
    };
    let coherence = match (circuits.is_empty(), unread) {
        (false, 0) => {
            let all: Vec<&MotionMode> = modes.iter().flatten().flatten().collect();
            match all.iter().find(|m| m.kind != BivectorKind::Elliptic) {
                Some(m) => Coherence::Open(m.kind),
                None => match all.iter().map(|m| m.order).collect::<Option<Vec<u32>>>() {
                    Some(orders) => Coherence::Closes(orders.into_iter().fold(1, lcm)),
                    None => Coherence::Held,
                },
            }
        }
        _ => Coherence::Unread,
    };
    let generators: Vec<Mv> = read.into_iter().cloned().collect();
    let constructors = match nuclei {
        Some(n) => cga::constructor_orbit(n, &generators, budget),
        None => ConstructorOrbit::Unread,
    };
    CellAlgebra { grade, circuits: circuits.len(), capacity, coupled, congruence, coherence, constructors }
}
