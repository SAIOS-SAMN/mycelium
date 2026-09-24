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
    /// The cell has no round, and why (ZM1): its nuclei did not read, or
    /// their meet is zero.
    NoRound(NoRound),
    /// No circuit crosses the cell: nothing was asked.
    NoCircuit,
}

/// WHY A CELL HAS NO ROUND (78.14, ZM1, 78.26(4)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoRound {
    /// A body's nucleus did not read back: no constructors, no meet.
    NucleiUnread,
    /// The nuclei read and their meet is zero: dependent spheres, or `n + 2`
    /// of them. Degeneration, never congruence.
    ZeroMeet,
}

/// The cell's nuclei did not read back: the Gram form has nothing to read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NucleiUnread;

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
    /// Every mode elliptic, these many of no admitted order: quasi-periodic,
    /// held.
    Held { unordered: usize },
    /// A mode of this class never returns.
    Open(BivectorKind),
    /// A circuit or its modes did not read, or none crossed the cell.
    Unread,
}

/// WHY A CIRCUIT CROSSING A CELL DID NOT READ (OR3's reason, 78.26(4)): the
/// edge half refused by name, the circuit riding a held coset, or a refusal
/// the traversal did not name — itself a reading.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CircuitUnread {
    Cone,
    NoPlace,
    NotRigid,
    Unrefracted,
    NoFace,
    TwoWays,
    Coset,
    Unnamed,
}

impl CircuitUnread {
    pub const ALL: [CircuitUnread; 8] = [
        CircuitUnread::Cone,
        CircuitUnread::NoPlace,
        CircuitUnread::NotRigid,
        CircuitUnread::Unrefracted,
        CircuitUnread::NoFace,
        CircuitUnread::TwoWays,
        CircuitUnread::Coset,
        CircuitUnread::Unnamed,
    ];

    pub fn label(self) -> &'static str {
        match self {
            CircuitUnread::Cone => "cone",
            CircuitUnread::NoPlace => "no-place",
            CircuitUnread::NotRigid => "not-rigid",
            CircuitUnread::Unrefracted => "unrefracted",
            CircuitUnread::NoFace => "no-face",
            CircuitUnread::TwoWays => "two-ways",
            CircuitUnread::Coset => "coset",
            CircuitUnread::Unnamed => "unnamed",
        }
    }
}

/// WHY A READ CIRCUIT'S MODES DID NOT READ (MR1): the holonomy is no versor,
/// or its action did not split — a factor with no rational root, a defective
/// part, or the budget spent reading a factor of this degree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModesUnread {
    NotVersor,
    Irrational(usize),
    Defective,
    Unbounded(usize),
}

/// WHY A CELL'S CAPACITY DID NOT READ (78.26(4)): no crossing circuit read; a
/// read holonomy with no action (not a versor); the budget spent at the k-th
/// product; an entry of the k-th product that does not fit the exact
/// elimination, by its place and its size in bits; or the elimination itself
/// overflowing at the k-th product.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CapacityRefused {
    NoneRead,
    NotVersor { circuit: usize },
    Spent { product: usize },
    Overflow { product: usize, entry: usize, bits: u64 },
    Elimination { product: usize },
}

/// CV1b — WHAT ONE CELL'S READERS COULD NOT HOLD, BY CONTENT (78.26(4)): the
/// observer's summary of the refusals the readers' own types carry, read off
/// them by [`CellAlgebra::refusals`] and never stored beside them (law 12).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CellRefusals {
    /// Each crossing circuit whose holonomy did not read, by OR3's reason.
    pub circuits: Vec<CircuitUnread>,
    /// Each read circuit whose modes did not read, by why.
    pub modes: Vec<ModesUnread>,
    /// Why the capacity did not read, where it did not.
    pub capacity: Option<CapacityRefused>,
    /// ZM1 — the cell's nuclei read and their meet is zero: dependent
    /// spheres, or `n + 2` of them. No round, and not congruence.
    pub zero_meet: bool,
    /// The cell's nuclei did not read back: no round, no constructors, no
    /// Gram form.
    pub nuclei: bool,
    /// Elliptic modes of no admitted order: what `Coherence::Held` holds.
    pub unordered: usize,
    /// Why the constructors' orbit did not read, where it did not.
    pub orbit: Option<cga::OrbitUnread>,
}

impl CellRefusals {
    /// Whether any reader of the cell refused.
    pub fn any(&self) -> bool {
        *self != CellRefusals::default()
    }
}

impl CellAlgebra {
    /// THE REFUSALS BY CONTENT, read off the readers' own typed answers.
    pub fn refusals(&self) -> CellRefusals {
        CellRefusals {
            circuits: self.circuits_unread.clone(),
            modes: self.modes_unread.clone(),
            capacity: self.capacity.err(),
            zero_meet: self.congruence == Congruence::NoRound(NoRound::ZeroMeet),
            nuclei: self.gram.is_err(),
            unordered: match self.coherence {
                Coherence::Held { unordered } => unordered,
                _ => 0,
            },
            orbit: match self.constructors {
                ConstructorOrbit::Unread(why) => Some(why),
                _ => None,
            },
        }
    }
}

/// ONE CELL'S ORBIT ALGEBRA (CG1, CG1b, GR1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellAlgebra {
    pub grade: usize,
    /// Circuits crossing the cell, read or not.
    pub circuits: usize,
    /// The rank of the unital ℚ-algebra the read circuits' actions generate;
    /// where it did not read, why — none read, a holonomy with no action,
    /// the budget spent at the k-th product, an entry that did not fit the
    /// exact elimination (78.26(4), rule 11).
    pub capacity: Result<usize, CapacityRefused>,
    /// Pairs of read circuits whose actions do not commute.
    pub coupled: usize,
    /// Every read mode ray against the cell.
    pub modes: Vec<ModeReading>,
    pub congruence: Congruence,
    pub permutes: Permutes,
    pub coherence: Coherence,
    /// The constructors' Gram form (GR1), or that the nuclei did not read.
    pub gram: Result<GramReading, NucleiUnread>,
    /// The orbit of the cell's constructors under its read circuits (CS1).
    pub constructors: ConstructorOrbit,
    /// CX1 — the product of the cell's constructors in each crossing
    /// circuit's order, one per circuit that orders every body of the cell;
    /// none chosen.
    pub products: Vec<super::CoxeterProduct>,
    /// The distinct classes among the kept products (C2a).
    pub classes: usize,
    /// GR2 — the constructors read as a reflection group, the nuclei as
    /// mirrors; and (78.15(5)) the polars as mirrors, where the dual frame
    /// stands.
    pub coxeter: Option<super::CoxeterReading>,
    pub polars: Option<super::CoxeterReading>,
    /// DF1 — the dual frame, `descend`'s reader, or why there is none.
    pub dual: Option<cga::DualFrame>,
    /// OR3 — each crossing circuit whose holonomy did not read, by its reason.
    pub circuits_unread: Vec<CircuitUnread>,
    /// MR1 — each read circuit whose modes did not read, by why.
    pub modes_unread: Vec<ModesUnread>,
    /// 78.26(2) — the read circuits' actions, in the order of the cell's
    /// read circuits, and the closed algebra's basis rows where the capacity
    /// read: what the cross-cell bracket is read against. Volatile, the
    /// percept's own, never persisted. `None` where an action did not read.
    pub actions: Option<Vec<Vec<Vec<Q>>>>,
    pub basis: Option<Vec<crate::core::lattice::SparseRow>>,
}

/// A matrix as one sparse integer row: its denominators cleared, reduced by
/// the gcd of what is left. `Err` names the entry that does not fit, by its
/// place and its size in bits.
fn row_of(m: &[Vec<Q>]) -> Result<crate::core::lattice::SparseRow, (usize, u64)> {
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
            let q = x / &g;
            row.push((at, i64::try_from(&q).map_err(|_| (at, q.bits()))?));
        }
    }
    Ok(row)
}

/// THE CAPACITY: the rank of the unital algebra the actions generate, the
/// products closed on a keyed worklist, one read of the budget per product.
/// A refusal carries what it could not hold: the product the budget ran out
/// at, the entry that did not fit, the elimination that overflowed.
fn capacity(
    actions: &[Vec<Vec<Q>>],
    budget: &mut crate::physics::Budget,
) -> Result<(usize, Vec<crate::core::lattice::SparseRow>), CapacityRefused> {
    let [first, ..] = actions else { return Err(CapacityRefused::NoneRead) };
    let d = first.len();
    let width = d * d;
    let overflow = |product: usize| move |(entry, bits)| CapacityRefused::Overflow { product, entry, bits };
    let mut rows = vec![row_of(&cga::identity_matrix(d)).map_err(overflow(0))?];
    let mut rank = 1usize;
    let mut product = 0usize;
    let mut work: Vec<Vec<Vec<Q>>> = vec![cga::identity_matrix(d)];
    while let Some(b) = work.pop() {
        for a in actions {
            product += 1;
            if !budget.spend() {
                return Err(CapacityRefused::Spent { product });
            }
            let p = cga::matrix_mul(&b, a);
            rows.push(row_of(&p).map_err(overflow(product))?);
            let r = crate::core::lattice::sparse_rank(&rows, width).ok_or(CapacityRefused::Elimination { product })?;
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
    // The rows kept are the closed algebra's basis: each raised the rank.
    Ok((rank, rows))
}

/// A matrix's row as the capacity writes it, for the bracket reading.
pub(crate) fn algebra_row(m: &[Vec<Q>]) -> Option<crate::core::lattice::SparseRow> {
    row_of(m).ok()
}

/// 78.26(2) — DOES A CELL'S CLOSED ALGEBRA CONTAIN THIS MATRIX: its basis
/// rows and the matrix's row do not raise the rank. `None` where the cell's
/// algebra did not read, the widths differ, or the row does not fit.
pub(crate) fn algebra_contains(cell: &CellAlgebra, m: &[Vec<Q>]) -> Option<bool> {
    let basis = cell.basis.as_ref()?;
    let [first, ..] = cell.actions.as_deref()? else { return None };
    let width = first.len() * first.len();
    if m.len() * m.len() != width {
        return None;
    }
    let mut rows = basis.clone();
    rows.push(algebra_row(m)?);
    Some(crate::core::lattice::sparse_rank(&rows, width)? == basis.len())
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

/// A CELL'S ALGEBRA, READ FIRST (78.11 items 4 and 5, CV1b F1 and F1b):
/// the read circuits' actions in the one algebra they span, and the rank of
/// the unital algebra they generate with its closed basis — or why not.
#[derive(Clone, Debug)]
pub struct CellRank {
    actions: Result<Vec<Vec<Vec<Q>>>, CapacityRefused>,
    rank: Result<(usize, Vec<crate::core::lattice::SparseRow>), CapacityRefused>,
}

/// READ A CELL'S RANK — the cell's dimension — off its crossing circuits,
/// under the stroke's budget. Read before any mode of any cell, so a
/// budget that runs out leaves every cell its dimension with the modes
/// unread by name, never the reverse.
pub fn read_rank(circuits: &[Result<Mv, CircuitUnread>], budget: &mut crate::physics::Budget) -> CellRank {
    let read: Vec<&Mv> = circuits.iter().flatten().collect();
    // Every action in the one algebra the cell's circuits span — the join of
    // their algebras, a value never narrowed — so the matrices compose.
    let n = read.iter().fold(0u8, |n, h| n.max(h.n()));
    let actions: Result<Vec<Vec<Vec<Q>>>, CapacityRefused> = read
        .iter()
        .enumerate()
        .map(|(circuit, h)| cga::action_of(&h.widened(n)).ok_or(CapacityRefused::NotVersor { circuit }))
        .collect();
    let rank = actions.as_deref().map_err(|why| *why).and_then(|a| capacity(a, budget));
    CellRank { actions, rank }
}

/// READ ONE CELL'S ORBIT ALGEBRA: its grade, its round, its constructors,
/// the holonomy of every circuit crossing it (`Err` with OR3's reason where
/// a circuit did not read), and each circuit's order of the cell's
/// constructors (`None` where it does not visit every body), under the
/// stroke's budget — its rank first, then the rest.
pub fn read_cell(
    grade: usize,
    round: Option<&Mv>,
    nuclei: Option<&[Mv]>,
    circuits: &[Result<Mv, CircuitUnread>],
    orders: &[Option<Vec<usize>>],
    budget: &mut crate::physics::Budget,
) -> CellAlgebra {
    let rank = read_rank(circuits, budget);
    read_cell_after(rank, grade, round, nuclei, circuits, orders, budget)
}

/// THE REST OF ONE CELL'S ORBIT ALGEBRA, its rank already read: the modes,
/// congruence, permutation, coherence, the Gram form, the constructors'
/// orbit and the Coxeter reading.
pub fn read_cell_after(
    rank: CellRank,
    grade: usize,
    round: Option<&Mv>,
    nuclei: Option<&[Mv]>,
    circuits: &[Result<Mv, CircuitUnread>],
    orders: &[Option<Vec<usize>>],
    budget: &mut crate::physics::Budget,
) -> CellAlgebra {
    let circuits_unread: Vec<CircuitUnread> = circuits.iter().filter_map(|h| h.as_ref().err().copied()).collect();
    let mut modes_unread: Vec<ModesUnread> = Vec::new();
    // WHY THERE IS NO ROUND, where there is none (ZM1, 78.26(4)): the nuclei
    // did not read, or they read and their meet is zero — dependent spheres,
    // or `n + 2` of them. A zero blade commutes with everything, which is
    // degeneration and not congruence.
    let no_round = match round {
        None => Some(NoRound::NucleiUnread),
        Some(m) if m.is_zero() => Some(NoRound::ZeroMeet),
        Some(_) => None,
    };
    let round = round.filter(|m| !m.is_zero());
    let read: Vec<&Mv> = circuits.iter().flatten().collect();
    let (capacity, basis) = match rank.rank {
        Ok((k, rows)) => (Ok(k), Some(rows)),
        Err(why) => (Err(why), None),
    };
    let actions = rank.actions.ok();
    let modes: Vec<Option<Vec<MotionMode>>> = circuits
        .iter()
        .map(|h| match h.as_ref().map(|h| cga::motion_modes(h, budget)) {
            Ok(Some(Ok(ms))) => Some(ms),
            Ok(Some(Err(why))) => {
                modes_unread.push(match why {
                    cga::Undecomposed::Irrational(d) => ModesUnread::Irrational(d),
                    cga::Undecomposed::Defective => ModesUnread::Defective,
                    cga::Undecomposed::Unbounded(d) => ModesUnread::Unbounded(d),
                });
                None
            }
            Ok(None) => {
                modes_unread.push(ModesUnread::NotVersor);
                None
            }
            Err(_) => None,
        })
        .collect();
    let unread = modes.iter().filter(|m| m.is_none()).count();
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
    let congruence = match (round, circuits.is_empty(), no_round) {
        (_, true, _) => Congruence::NoCircuit,
        (None, false, Some(why)) => Congruence::NoRound(why),
        (None, false, None) => Congruence::NoRound(NoRound::NucleiUnread),
        (Some(_), false, _) => {
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
                    None => Coherence::Held { unordered: all.iter().filter(|m| m.order.is_none()).count() },
                },
            }
        }
        _ => Coherence::Unread,
    };
    let gram = nuclei.map(cga::read_gram).ok_or(NucleiUnread);
    let generators: Vec<Mv> = read.into_iter().cloned().collect();
    let constructors = match nuclei {
        Some(ns) => cga::constructor_orbit(ns, &generators, budget),
        None => ConstructorOrbit::NoConstructors,
    };
    // CX1 — the product per circuit that orders every body; GR2 — the
    // group, with the products' certain refusals; DF1 — the dual frame, and
    // the polars read as mirrors where it stands.
    //
    // The group is read first; the products' modes only where it is not
    // already refused with certainty — a refused group's tether is named by
    // its pair regime, and reading its modes would spend the stroke on what
    // is known (rule 10).
    let first = nuclei.map(|ns| super::read_coxeter(ns, &[], budget));
    let refused = matches!(first.as_ref().map(|r| &r.closure), Some(super::Closure::Infinite(_)));
    let products: Vec<super::CoxeterProduct> = match nuclei {
        Some(ns) => orders.iter().flatten().filter_map(|o| super::coxeter_product(ns, o, !refused, budget)).collect(),
        None => Vec::new(),
    };
    let mut class_keys: Keyed<()> = Keyed::new();
    for p in &products {
        if let Some(k) = super::product_class(p) {
            class_keys.insert(&k, ());
        }
    }
    // A product of infinite order refuses what the pairs did not (CX1).
    let coxeter = first.map(|r| match products.iter().enumerate().find_map(|(i, p)| p.infinite.map(|k| (i, k))) {
        Some((circuit, kind)) if !refused => {
            super::CoxeterReading { closure: super::Closure::Infinite(super::Refusal::Product { circuit, kind }), ..r }
        }
        _ => r,
    });
    let dual = nuclei.map(cga::dual_frame);
    let polars = match &dual {
        Some(cga::DualFrame::Frame(ps)) => Some(super::read_coxeter(ps, &[], budget)),
        _ => None,
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
        classes: class_keys.len(),
        products,
        coxeter,
        polars,
        dual,
        circuits_unread,
        modes_unread,
        actions,
        basis,
    }
}
