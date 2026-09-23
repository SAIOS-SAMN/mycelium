// 78.11, 78.12(2), 78.14, CG1, CG1b, GR1 — COMPREHENSION IS THE ORBIT ALGEBRA
// OF A CELL: every circuit through it kept, each read by its modes off its
// own action; the ℚ-algebra of their actions, its rank the cell's capacity;
// per mode ray, whether it keeps the round and whether it keeps the
// constructors; coherence jointly; the constructors' Gram form; and their
// orbit under the circuits (CS1).
//
//! No sandwich and no equality with the boundary (78.11). A cell's RECORD is
//! its constructors, the bodies' nuclei; its ROUND, the meet, reads where the
//! configuration is, and its GRAM FORM reads its shape up to motion (78.14).
//!
//! TWO READINGS PER MODE RAY, both exact (78.14, CG1b):
//!
//! - KEEPS THE ROUND: `Ω M Ω⁻¹ = M`, the commutator `Ω M − M Ω = 0`, linear,
//!   no chart, no square. Where it does not vanish the bracket is a blade —
//!   the direction the round is carried, on the mode's plane, at the cell's
//!   grade — and it IS the refusal's content. `Ω M Ω⁻¹ = −M`, the round
//!   REVERSED, is the held `ℤ₂`'s arm, named apart and never incongruent.
//!   Vacuous at the top grade, where the meet is the pseudoscalar.
//! - KEEPS THE CONSTRUCTORS: a permutation `π` with `Ω s_a Ω⁻¹ ∼ s_π(a)`,
//!   found by keyed lookup of each carried nucleus's canonical ray — no
//!   search — and vacuous at no grade. Permutes ⇒ keeps the round up to
//!   sign; a turn inside the span keeps the round and moves every nucleus.
//!
//! A ZERO MEET IS NO ROUND: a zero blade commutes with everything, which is
//! degeneration, not invariance; the Gram form's span signature is the
//! reading in its place.
//!
//! COHERENCE IS JOINT: the modes close together where every elliptic order
//! is finite, at their least common multiple; a parabolic or hyperbolic mode
//! holds the cell open by name; a turn of no admitted order leaves it
//! quasi-periodic, HELD, never a refusal.
//!
//! THE CAPACITY IS COMPUTED, NEVER ADDED BY HAND: the rank of the unital
//! ℚ-algebra the circuits' actions `R_c` generate on the vectors, by
//! `sparse_rank` over the flattened matrices, the products closed under the
//! stroke's budget. The COUPLING is the pairs whose actions do not commute.

use crate::network::nerve::coord::BivectorKind;
use crate::network::nerve::Keyed;
use crate::transducer::cga::{self, ConstructorOrbit, GramReading, MotionMode, Mv};
use num_rational::BigRational;
use num_traits::Zero;

type Q = BigRational;

/// WHAT ONE MODE RAY DOES TO THE ROUND.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RoundKept {
    /// `Ω M Ω⁻¹ = M`.
    Kept,
    /// `Ω M Ω⁻¹ = −M`: the round reversed, the held `ℤ₂`'s arm.
    Reversed,
    /// Carried off: the bracket `Ω M − M Ω`, the direction it is carried.
    Carried(Mv),
    /// The cell has no round.
    NoRound,
}

/// ONE MODE RAY OF ONE CIRCUIT, READ AGAINST THE CELL: the circuit and the
/// mode (in the reader's own order), the mode's class and the planes its ray
/// turns in, what it does to the round, and the permutation of the cell's
/// constructors it enacts — `None` where it carries one off the set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModeReading {
    pub circuit: usize,
    pub mode: usize,
    pub kind: BivectorKind,
    pub plane: Vec<usize>,
    pub round: RoundKept,
    pub permutation: Option<Vec<usize>>,
}

/// WHETHER THE ROUND IS KEPT BY EVERY MODE OF EVERY CIRCUIT THROUGH THE CELL.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Congruence {
    /// Every circuit read, every mode ray keeps the round.
    Congruent,
    /// Every circuit read, none carries the round, and these many mode rays
    /// reverse it: the held `ℤ₂`, never a refusal.
    HeldZ2 { reversed: usize },
    /// Some mode ray carries the round, at this grade: every such mode kept,
    /// none chosen (checklist (r)).
    Incongruent { grade: usize, failures: Vec<ModeReading> },
    /// Circuits whose holonomy or modes did not read, and no read one failed.
    Unread { unread: usize },
    /// The cell has no round: a nucleus did not read, or the meet is zero.
    NoRound,
    /// No circuit crosses the cell: nothing was asked.
    NoCircuit,
}

/// WHETHER EVERY MODE OF EVERY CIRCUIT PERMUTES THE CELL'S CONSTRUCTORS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Permutes {
    /// Every circuit read, every mode ray permutes the constructors.
    All,
    /// These `(circuit, mode)` carry a constructor off the set.
    Moves(Vec<(usize, usize)>),
    /// Circuits did not read, and no read one moved a constructor.
    Unread { unread: usize },
    /// The cell's constructors did not read.
    NoConstructors,
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

/// ONE CELL'S ORBIT ALGEBRA (CG1, CG1b, GR1).
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
    /// Every read mode ray against the cell.
    pub modes: Vec<ModeReading>,
    pub congruence: Congruence,
    pub permutes: Permutes,
    pub coherence: Coherence,
    /// The constructors' Gram form (GR1); `None` where they did not read.
    pub gram: Option<GramReading>,
    /// The orbit of the cell's constructors under its read circuits (CS1).
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

fn ray_key(v: &Mv) -> Option<Vec<u8>> {
    let mut key = Vec::new();
    v.canonical()?.put_canonical(&mut key);
    Some(key)
}

/// THE PERMUTATION A RAY ENACTS ON THE CONSTRUCTORS, by keyed lookup of each
/// carried constructor's canonical ray; `None` where one lands off the set
/// or two land on one.
fn permutation(ray: &Mv, nuclei: &[Mv], index: &Keyed<usize>) -> Option<Vec<usize>> {
    let mut pi = Vec::with_capacity(nuclei.len());
    for s in nuclei {
        let carried = cga::carry(ray, s)?;
        let at = *index.get(&ray_key(&carried)?)?;
        if pi.contains(&at) {
            return None;
        }
        pi.push(at);
    }
    Some(pi)
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
    // A zero meet is no round (78.14).
    let round = round.filter(|m| !m.is_zero());
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
    let mut index: Keyed<usize> = Keyed::new();
    for (a, s) in nuclei.into_iter().flatten().enumerate() {
        if let Some(k) = ray_key(s) {
            index.insert(&k, a);
        }
    }
    let readings: Vec<ModeReading> = modes
        .iter()
        .enumerate()
        .flat_map(|(c, ms)| ms.iter().flatten().enumerate().map(move |(j, mode)| (c, j, mode)))
        .map(|(c, j, mode)| {
            let round = match round {
                None => RoundKept::NoRound,
                Some(m) => {
                    let (forward, back) = (mode.ray.geo(m), m.geo(&mode.ray));
                    match (forward.sub(&back).is_zero(), forward.add(&back).is_zero()) {
                        (true, _) => RoundKept::Kept,
                        (_, true) => RoundKept::Reversed,
                        _ => RoundKept::Carried(forward.sub(&back)),
                    }
                }
            };
            let permutation = nuclei.and_then(|ns| permutation(&mode.ray, ns, &index));
            ModeReading { circuit: c, mode: j, kind: mode.kind, plane: plane_of(mode), round, permutation }
        })
        .collect();
    let congruence = match (round, circuits.is_empty()) {
        (_, true) => Congruence::NoCircuit,
        (None, false) => Congruence::NoRound,
        (Some(_), false) => {
            let failures: Vec<ModeReading> =
                readings.iter().filter(|r| matches!(r.round, RoundKept::Carried(_))).cloned().collect();
            let reversed = readings.iter().filter(|r| r.round == RoundKept::Reversed).count();
            match (failures.is_empty(), unread, reversed) {
                (false, _, _) => Congruence::Incongruent { grade, failures },
                (true, 0, 0) => Congruence::Congruent,
                (true, 0, reversed) => Congruence::HeldZ2 { reversed },
                (true, unread, _) => Congruence::Unread { unread },
            }
        }
    };
    let permutes = match (nuclei, circuits.is_empty()) {
        (_, true) => Permutes::NoCircuit,
        (None, false) => Permutes::NoConstructors,
        (Some(_), false) => {
            let moves: Vec<(usize, usize)> =
                readings.iter().filter(|r| r.permutation.is_none()).map(|r| (r.circuit, r.mode)).collect();
            match (moves.is_empty(), unread) {
                (false, _) => Permutes::Moves(moves),
                (true, 0) => Permutes::All,
                (true, unread) => Permutes::Unread { unread },
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
    let gram = nuclei.map(cga::read_gram);
    let generators: Vec<Mv> = read.into_iter().cloned().collect();
    let constructors = match nuclei {
        Some(ns) => cga::constructor_orbit(ns, &generators, budget),
        None => ConstructorOrbit::Unread,
    };
    CellAlgebra {
        grade,
        circuits: circuits.len(),
        capacity,
        coupled,
        modes: readings,
        congruence,
        permutes,
        coherence,
        gram,
        constructors,
    }
}
