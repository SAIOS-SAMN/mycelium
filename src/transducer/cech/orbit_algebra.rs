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
//! ℚ-algebra the circuits' actions `R_c` generate on the vectors, read as
//! the grade of the span of the products (`core::span`), each product
//! joining where it lies outside, closed under the stroke's budget. The COUPLING is the pairs whose actions do not commute.

use crate::network::nerve::coord::BivectorKind;
use crate::network::nerve::Keyed;
use crate::transducer::cga::{self, ConstructorOrbit, GramReading, MotionMode, Mv};
use num_traits::Zero;

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

/// WHETHER THE MODES CLOSE TOGETHER (78.26(5)).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Coherence {
    /// Every ring resonant — every mode elliptic of finite order: the joint
    /// period, their lcm. A delay, not a loss.
    Closes(u32),
    /// The NON-RESONANT rings, each with its residue by content, in the
    /// circuits' order, none chosen (checklist (r)): a rosette, parabolic or
    /// boost mode never returns. The resonant rings stay on `Rings`.
    Open(Vec<Incoherence>),
    /// A circuit or its modes did not read, or none crossed the cell.
    Unread,
}

/// ONE CIRCUIT CROSSING A CELL (78.26(5)): the ring's own coordinates — its
/// steps, each face and its way, framed — its holonomy or OR3's reason, and
/// its hue, the steps' hue maps composed (`None` where it did not read).
#[derive(Clone, Debug)]
pub struct CellCircuit {
    pub ring: Vec<u8>,
    pub holonomy: Result<Mv, CircuitUnread>,
    pub hue: Option<Vec<(u8, u8)>>,
}

impl CellCircuit {
    /// A ring's coordinates from its steps: each overlap by its own
    /// coordinates (AT1) and its way, framed.
    pub fn ring_of(steps: &[(crate::network::nerve::coord::Overlap, bool)]) -> Vec<u8> {
        use crate::network::nerve::Coordinate;
        let mut k = Vec::new();
        for (o, forward) in steps {
            let mut step = o.encode().as_bytes().to_vec();
            step.push(u8::from(*forward));
            crate::transducer::address::put_key_bytes(&mut k, &step);
        }
        k
    }
}

/// ONE MODE OF A RING'S RESIDUE: its regime, the planes its ray turns in,
/// and its order where elliptic of finite order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResidueMode {
    pub kind: BivectorKind,
    pub plane: Vec<usize>,
    pub order: Option<u32>,
}

/// WHY A RING'S CLASS DID NOT READ: its holonomy does not split as
/// `T(a)·K(b)·D(ρ)·R` at all; it splits with a transversion `b ≠ 0`, or in
/// the inversion's cell, which the class vector has no coordinate for (A.13:
/// the key reads the `∞` pole's half — the placement, the dilation, the
/// turn — and nothing yet keys the origin pole's); its rotor has no
/// conjugacy invariants; or its hue did not read (a static contact's gauge
/// half, a held coset).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassUnread {
    NoSplit,
    Transversion,
    NoInvariants,
    HueUnread,
}

/// 78.26(5) — A NON-RESONANT RING, BY CONTENT: its coordinates, its class
/// (ClassVector::of_rotor on its holonomy split as the fill splits it, with
/// its hue), and every mode of its residue. Never a bare hole.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Incoherence {
    pub ring: Vec<u8>,
    pub class: Result<Box<crate::transducer::sentence::ClassVector>, ClassUnread>,
    pub modes: Vec<ResidueMode>,
}

/// A RING'S CLASS: its holonomy split as the fill splits a transition — the
/// carriage `T·D`, the rotor `(T·D)⁻¹ V` on the base, read at both poles
/// (A.13) — by the one class producer with the ring's own hue (78.26(5),
/// ruled). A holonomy carrying the origin pole's half is read as that, by
/// name, and never keyed as if it fixed infinity.
fn class_of_ring(
    holonomy: &Mv,
    hue: Option<&[(u8, u8)]>,
) -> Result<Box<crate::transducer::sentence::ClassVector>, ClassUnread> {
    use crate::transducer::element::{conformal_factors, rotor_of_factors};
    let hue = hue.ok_or(ClassUnread::HueUnread)?;
    let f = conformal_factors(holonomy).map_err(|_| ClassUnread::NoSplit)?;
    if f.inverted || f.transverses() {
        return Err(ClassUnread::Transversion);
    }
    let carriage = f.carriage();
    let rotor = rotor_of_factors(&f).map_err(|_| ClassUnread::NoSplit)?;
    crate::transducer::sentence::ClassVector::of_rotor(&rotor, carriage, crate::transducer::sentence::CycleType::of(hue))
        .map(Box::new)
        .ok_or(ClassUnread::NoInvariants)
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
/// read holonomy with no action (not a versor); or the budget spent at the
/// k-th product. The `Overflow` and `Elimination` arms retired with the
/// exact elimination at any size (STD1): nothing can raise them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CapacityRefused {
    NoneRead,
    NotVersor { circuit: usize },
    Spent { product: usize },
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

/// 78.26(5) — THE RINGS THROUGH A CELL, READ BY THEIR RESIDUE: each circuit
/// crossing the cell is a ring, and its holonomy's modes say whether the ring
/// is coherent and, where it is not, how. Three axes, each counted per ring
/// and never collapsed into one another:
///
/// - RESONANCE: coherent (the holonomy has no mode — nothing held); resonant
///   (every mode elliptic of finite order — the ring closes at the joint
///   period, a delay and not a loss); or not, by each kind of mode that never
///   returns — a rosette (elliptic of no finite order), parabolic, boost.
/// - LEVEL: harmonic (a mode does not resonate), luminal (the gauge refused at
///   the cone, OR3's `Cone`), holonomic (a mode carries the round off: around
///   the ring and back does not return what was perceived).
/// - KIND, per mode (A.17, 78.32, 74): D where a mode is a rosette — a rotor
///   at an angle no held word spells, RotorPrime's case; C where a mode is
///   parabolic or a boost, or the ring is luminal — another chart or vantage
///   may carry it. A resonant ring is neither: the alphabet's own turns spell
///   it and it closes at its period. Never P: an incoherence is a state.
///
/// A ring whose holonomy or modes did not read is not here: it is counted by
/// its reason on the refusals (`circuits`, `modes`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rings {
    pub coherent: usize,
    pub resonant: usize,
    pub rosette: usize,
    pub parabolic: usize,
    pub boost: usize,
    pub harmonic: usize,
    pub luminal: usize,
    pub holonomic: usize,
    pub d: usize,
    pub c: usize,
}

/// 78.26(5) — THE RINGS BY THEIR RESIDUE, read off each crossing circuit's
/// holonomy and modes and the mode readings against the round.
fn read_rings(
    circuits: &[Result<Mv, CircuitUnread>],
    modes: &[Option<Vec<MotionMode>>],
    readings: &[ModeReading],
) -> Rings {
    let mut r = Rings::default();
    for (c, (h, ms)) in circuits.iter().zip(modes).enumerate() {
        let ms = match (h, ms) {
            (Err(CircuitUnread::Cone), _) => {
                r.luminal += 1;
                r.c += 1;
                continue;
            }
            // Unread by its reason, counted on the refusals.
            (Err(_), _) | (Ok(_), None) => continue,
            (Ok(_), Some(ms)) => ms,
        };
        let live: Vec<&MotionMode> = ms.iter().filter(|m| m.kind != BivectorKind::Trivial).collect();
        if live.is_empty() {
            r.coherent += 1;
            continue;
        }
        let has = |k: BivectorKind| live.iter().any(|m| m.kind == k);
        let rosette = live.iter().any(|m| m.kind == BivectorKind::Elliptic && m.order.is_none());
        let (parabolic, boost) = (has(BivectorKind::Parabolic), has(BivectorKind::Hyperbolic));
        match rosette || parabolic || boost {
            false => r.resonant += 1,
            true => {
                r.harmonic += 1;
                r.rosette += usize::from(rosette);
                r.parabolic += usize::from(parabolic);
                r.boost += usize::from(boost);
            }
        }
        r.holonomic += usize::from(readings.iter().any(|x| x.circuit == c && matches!(x.round, RoundKept::Carried(_))));
        // A resonant ring is spelled by the alphabet's own turns and closes at
        // its period: it demands nothing and counts in neither kind.
        r.d += usize::from(rosette);
        r.c += usize::from(parabolic || boost);
    }
    r
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
            // The rosettes the open rings carry: elliptic modes of no order.
            unordered: match &self.coherence {
                Coherence::Open(rings) => rings
                    .iter()
                    .flat_map(|r| &r.modes)
                    .filter(|m| m.kind == BivectorKind::Elliptic && m.order.is_none())
                    .count(),
                Coherence::Closes(_) | Coherence::Unread => 0,
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
    /// Pairs of read circuits whose actions do not commute; where the
    /// stroke was spent before every pair was composed, the pair it was
    /// spent at (`Spent`), never a partial count.
    pub coupled: Result<usize, CapacityRefused>,
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
    pub actions: Option<Vec<cga::LinearFunction>>,
    pub basis: Option<crate::core::span::Span>,
    /// 78.26(5) — the rings through the cell, by their residue.
    pub rings: Rings,
}

/// THE CAPACITY: the dimension of the unital algebra the actions generate —
/// the grade of the span of its products — closed on a worklist, one read of
/// the budget per product. Each product is one action applied after another
/// (`LinearFunction::after`), read as the ray it is; it JOINS the span only
/// where it lies outside it, and only a product that joined is multiplied
/// on. A refusal carries what it could not hold: the product the budget ran
/// out at. Exact at any size (STD1).
fn capacity(
    actions: &[cga::LinearFunction],
    budget: &mut crate::physics::Budget,
) -> Result<(usize, crate::core::span::Span), CapacityRefused> {
    let [first, ..] = actions else { return Err(CapacityRefused::NoneRead) };
    let identity = cga::LinearFunction::identity(first.n());
    let mut span = crate::core::span::Span::new();
    span.join(&identity.as_ray());
    let mut product = 0usize;
    let mut work: Vec<cga::LinearFunction> = vec![identity];
    while let Some(b) = work.pop() {
        for a in actions {
            product += 1;
            if !budget.spend() {
                return Err(CapacityRefused::Spent { product });
            }
            let p = b.after(a);
            if span.join(&p.as_ray()) {
                work.push(p);
            }
        }
    }
    Ok((span.grade(), span))
}

/// 78.68, 78.11 item 4 — WHY A CELL'S LIE RANK DID NOT READ: no circuit's
/// holonomy read; a holonomy with no Cayley chart point (the π pole, or a
/// weight with no rational root), by its circuit; or the stroke spent at the
/// k-th bracket.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LieRankRefused {
    NoneRead,
    NoChart { circuit: usize },
    Spent { bracket: usize },
}

/// 78.68, 78.11 item 4 — THE LIE RANK OF 𝔥_cell: the dimension over ℚ of
/// `𝔥 = ⟨B_c, [B_c, B_c′], [[B_c, B_c′], B_c″], …⟩_ℚ ⊂ so(n+1,1)`, the
/// Ambrose–Singer holonomy algebra of the cell. Each read circuit's
/// holonomy is taken to its Cayley bivector `B_c = (1 − H_c)(1 + H_c)⁻¹`
/// (`cga::cayley_inverse`), and the span is closed under the commutator on a
/// worklist — a bracket JOINS only where it lies outside the span, and only
/// a bracket that joined is bracketed on — one read of the budget per
/// bracket. At n = 3 the rank is at most 10, dim so(4,1).
///
/// This is not [`CellAlgebra::capacity`], which is the dimension of the
/// unital ASSOCIATIVE algebra the circuits' actions generate on the space.
/// The rank-step act (78.68) is a step of this one.
pub fn lie_rank(circuits: &[CellCircuit], budget: &mut crate::physics::Budget) -> Result<usize, LieRankRefused> {
    let holonomies: Vec<Option<&Mv>> = circuits.iter().map(|c| c.holonomy.as_ref().ok()).collect();
    let gens = cayley_generators(&holonomies)?;
    if gens.is_empty() {
        return Err(LieRankRefused::NoneRead);
    }
    Ok(lie_close(&[], &gens, budget)?.len())
}

/// Each read holonomy's Cayley bivector `B = (1 − H)(1 + H)⁻¹`, in order;
/// an unread holonomy (`None`) is passed over, and one with no chart point
/// refuses by its index.
pub fn cayley_generators(holonomies: &[Option<&Mv>]) -> Result<Vec<Mv>, LieRankRefused> {
    let mut gens = Vec::new();
    for (circuit, h) in holonomies.iter().enumerate() {
        let Some(h) = h else { continue };
        gens.push(cga::cayley_inverse(h).ok_or(LieRankRefused::NoChart { circuit })?);
    }
    Ok(gens)
}

/// THE LIE CLOSURE OF A HELD BASIS WITH NEW GENERATORS: `held` is a basis
/// already closed under the commutator (a span this closure built), `new`
/// the generators to adjoin; the result is a basis of the smallest Lie
/// algebra over ℚ holding both, the held basis first. A generator or a
/// bracket JOINS only where it lies outside the span, and only what joined
/// is bracketed on, so a held algebra re-closes in no more brackets than its
/// new dimensions need; one read of the budget per bracket.
pub fn lie_close(held: &[Mv], new: &[Mv], budget: &mut crate::physics::Budget) -> Result<Vec<Mv>, LieRankRefused> {
    let n = held.iter().chain(new).fold(0u8, |n, g| n.max(g.n()));
    let mut span = crate::core::span::Span::new();
    let mut basis: Vec<Mv> = Vec::new();
    for g in held {
        let g = g.widened(n);
        if span.join(&cga::ray_of(&g)) {
            basis.push(g);
        }
    }
    let mut work: Vec<Mv> = Vec::new();
    for g in new {
        let g = g.widened(n);
        if span.join(&cga::ray_of(&g)) {
            basis.push(g.clone());
            work.push(g);
        }
    }
    let mut bracket = 0usize;
    while let Some(x) = work.pop() {
        let against = basis.clone();
        for y in &against {
            bracket += 1;
            if !budget.spend() {
                return Err(LieRankRefused::Spent { bracket });
            }
            // [X, Y] = XY − YX, a bivector again: the commutator of two
            // bivectors has no scalar or grade-4 part.
            let c = x.geo(y).sub(&y.geo(&x));
            if !c.is_zero() && span.join(&cga::ray_of(&c)) {
                basis.push(c.clone());
                work.push(c);
            }
        }
    }
    Ok(basis)
}

/// 78.26(2) — DOES A CELL'S CLOSED ALGEBRA CONTAIN THIS FUNCTION: it lies in
/// the algebra's span. `None` where the cell's algebra did not read or the
/// two act in algebras of different `n`.
pub(crate) fn algebra_contains(cell: &CellAlgebra, f: &cga::LinearFunction) -> Option<bool> {
    let span = cell.basis.as_ref()?;
    let [first, ..] = cell.actions.as_deref()? else { return None };
    if first.n() != f.n() {
        return None;
    }
    Some(span.holds(&f.as_ray()))
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
    actions: Result<Vec<cga::LinearFunction>, CapacityRefused>,
    rank: Result<(usize, crate::core::span::Span), CapacityRefused>,
}

/// READ A CELL'S RANK — the cell's dimension — off its crossing circuits,
/// under the stroke's budget. Read before any mode of any cell, so a
/// budget that runs out leaves every cell its dimension with the modes
/// unread by name, never the reverse.
pub fn read_rank(circuits: &[CellCircuit], budget: &mut crate::physics::Budget) -> CellRank {
    let read: Vec<&Mv> = circuits.iter().filter_map(|c| c.holonomy.as_ref().ok()).collect();
    // Every action in the one algebra the cell's circuits span — the join of
    // their algebras, a value never narrowed — so the actions compose.
    let n = read.iter().fold(0u8, |n, h| n.max(h.n()));
    let actions: Result<Vec<cga::LinearFunction>, CapacityRefused> = read
        .iter()
        .enumerate()
        .map(|(circuit, h)| cga::action_of(&h.widened(n)).ok_or(CapacityRefused::NotVersor { circuit }))
        .collect();
    let rank = actions.as_deref().map_err(|why| *why).and_then(|a| capacity(a, budget));
    CellRank { actions, rank }
}

/// READ ONE CELL'S ORBIT ALGEBRA: its grade, its round, its constructors,
/// every circuit crossing it — its ring, its holonomy (`Err` with OR3's
/// reason where it did not read) and its hue — and each circuit's order of
/// the cell's constructors (`None` where it does not visit every body), under
/// the stroke's budget — its rank first, then the rest.
pub fn read_cell(
    grade: usize,
    round: Option<&Mv>,
    nuclei: Option<&[Mv]>,
    circuits: &[CellCircuit],
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
    crossing: &[CellCircuit],
    orders: &[Option<Vec<usize>>],
    budget: &mut crate::physics::Budget,
) -> CellAlgebra {
    // Each circuit's holonomy, or OR3's reason, in the crossing order.
    let holonomies: Vec<Result<Mv, CircuitUnread>> = crossing.iter().map(|c| c.holonomy.clone()).collect();
    let circuits = holonomies.as_slice();
    let circuits_unread: Vec<CircuitUnread> = circuits.iter().filter_map(|h| h.as_ref().err().copied()).collect();
    let mut modes_unread: Vec<ModesUnread> = Vec::new();
    // WHY THERE IS NO ROUND, where there is none (ZM1, 78.26(4)): the nuclei
    // did not read, or they read and their meet is zero — dependent spheres,
    // or `n + 2` of them. A zero blade commutes with everything, which is
    // degeneration and not congruence.
    // The round's reading is ONE value, the round or why there is none, read
    // once here and matched exhaustively below: no arm invents a reason.
    let round_read: Result<&Mv, NoRound> = match round {
        None => Err(NoRound::NucleiUnread),
        Some(m) if m.is_zero() => Err(NoRound::ZeroMeet),
        Some(m) => Ok(m),
    };
    let round = round_read.ok();
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
    // EACH PAIR'S COMMUTATOR IS TWO COMPOSITIONS, AND DRAWS ONE READ. It
    // drew none, so a cell crossed by k circuits composed k² exact linear
    // functions outside any stroke: on the live wire (2026-09-29) a voice
    // reading 5 315 circuits sat in this line for minutes of one stroke,
    // every stack sample in `LinearFunction::after`.
    let coupled = match &actions {
        Some(a) => {
            let mut product = 0usize;
            let mut count = Ok(0usize);
            'pairs: for i in 0..a.len() {
                for j in (i + 1)..a.len() {
                    product += 1;
                    if !budget.spend() {
                        count = Err(CapacityRefused::Spent { product });
                        break 'pairs;
                    }
                    if !a[i].after(&a[j]).minus(&a[j].after(&a[i])).is_zero() {
                        count = count.map(|c| c + 1);
                    }
                }
            }
            count
        }
        None => Ok(0),
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
    let rings = read_rings(circuits, &modes, &readings);
    let congruence = match (round_read, circuits.is_empty()) {
        (_, true) => Congruence::NoCircuit,
        (Err(why), false) => Congruence::NoRound(why),
        (Ok(_), false) => {
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
    // 78.26(5) — COHERENCE RING BY RING: a resonant ring contributes its
    // orders to the joint period; a non-resonant one is an incoherence held
    // by content. Every circuit and its modes read here (`unread` is zero),
    // so each ring is paired with its own modes.
    let coherence = match (circuits.is_empty(), unread) {
        (false, 0) => {
            let mut orders: Vec<u32> = Vec::new();
            let mut open: Vec<Incoherence> = Vec::new();
            for (ring, ms) in crossing.iter().zip(&modes).filter_map(|(r, m)| Some((r, m.as_ref()?))) {
                let live: Vec<&MotionMode> = ms.iter().filter(|m| m.kind != BivectorKind::Trivial).collect();
                let resonant = live.iter().all(|m| m.kind == BivectorKind::Elliptic && m.order.is_some());
                match (resonant, &ring.holonomy) {
                    (true, _) => orders.extend(live.iter().filter_map(|m| m.order)),
                    (false, Ok(h)) => open.push(Incoherence {
                        ring: ring.ring.clone(),
                        class: class_of_ring(h, ring.hue.as_deref()),
                        modes: live
                            .iter()
                            .map(|m| ResidueMode { kind: m.kind, plane: plane_of(m), order: m.order })
                            .collect(),
                    }),
                    // Modes read only off a read holonomy (above).
                    (false, Err(_)) => {}
                }
            }
            match open.is_empty() {
                true => Coherence::Closes(orders.into_iter().fold(1, lcm)),
                false => Coherence::Open(open),
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
        rings,
    }
}
