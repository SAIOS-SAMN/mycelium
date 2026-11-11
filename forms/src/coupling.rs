// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE COUPLING (LAW-COUPLING v1, proposal 78.175; ruled by netstratum's
//! and mycelium's operators 2026-11-06 through MU-RELEASE §3a, at kernel
//! standing until the fixtures are joint-read agreed), extending LAW-FIELD
//! (78.171 §2 an orbit, §3.3: the orbit's shear is this coupling). Four
//! laws netstratum and xylarium aligned on, stated in netstratum's docs/121
//! §9–§13 (the dipoles, the range's field as one, the shear as the degree
//! of modulation, the state level as the spin balance) and docs/122 §2 (the
//! light ledger). The pole and the alignment are continuous readings in
//! `[−1, 1]`, never bare signs.
//!
//! SHARED DEGREES ARE ROUND (ruled 2026-11-06 by netstratum's operator,
//! matching mycelium's recommendation; 78.175 §1). A degree shared between
//! kernels is a rotation angle on a Clifford turning plane, the round
//! circle of LAW-COORDINATE v4, LAW-PASSAGE v3 and LAW-FIELD. An L1
//! projection is a NAMED READING ([`hemisphere_l1`], [`Parcel::arc_l1`]),
//! never written as plain degrees.
//!
//! F1 AND F2, ONE LAW: THE DIPOLE PARCEL, ITS FLUIDITY AND ITS VISCOSITY
//! ([`Parcel`], [`Parcels`], [`Dipole`]), PER PLANE (netstratum 3Q). A kind's frames
//! are `A = W = 3Z − U` held and `Q = U` open (`Parcel::kind`). A body
//! carries `(z_k, q_k)` on each of its turning planes, founded by the even
//! split `((3Z − U)/n, U/n)` over its `n` planes ([`Parcels::founded`]);
//! a bond or a peel changes one plane only ([`Parcels::bond`]). On each
//! plane, with `t = q/z`:
//! - the pole is `σ = (z − q)/(z + q)`, continuous in `[−1, 1]`: `+1` at
//!   the closed pole, 0 at the throat, `−1` at the open pole. It is a
//!   ratio, which no metric enters. It stands on the frames, never on the
//!   electrons' phase `R`, which is the parcel's phase. Where an angle is
//!   named for it, it is the L1 arc `θ_L1 = 180°·q/(z + q)`, labelled as
//!   an L1 arc ([`Parcel::arc_l1`]);
//! - the strength is the open share, the fluidity, `o = q/(z + q) = ν(t)`,
//!   with `ν(t) = t/(1 + t)` ([`open_share`]); the viscosity is the
//!   cohesion, the held share `z/(z + q) = 1 − ν(t) = ν(1/t)` (ruled
//!   2026-11-06: viscosity resists flow; [`held_share`], and per plane
//!   [`Parcels::cohesion`], a keyed vector never summed), both `½` at the throat. The
//!   inversion reverts it: `ν(1/t) = 1 − ν(t)`, the frames exchanged
//!   ([`Parcel::inverted`]), exchanging the fluidity and the viscosity.
//!
//! F3, THE DIPOLE COUPLING ON A TURNING PLANE ([`field`], [`pull`]). The
//! field of a body's range acts as one, on one named turning plane
//! (`forms::coordinate::Plane::Turning`):
//! `F = Σ w_j·σ_j·o_j·n_j·h_j / Σ w_j`, with `n = M/(|g| + M)` the ring
//! law's nearness ([`nearness`]) and `h = hemisphere_l1(Δ) = 1 − Δ/90°`
//! the alignment ([`Alignment`]), the named L1 reading of the round
//! separation `Δ ∈ [0°, 180°]` on that plane: aligned toward `+1`,
//! uncoupled at 90°, opposed toward `−1`. Each term stands in `[−1, 1]`, so `|F| ≤ 1`. A rate is a
//! share `x ∈ (−1, 1)` of the luminal bound and moves by LAW-COORDINATE's
//! flow law (`forms::coordinate::flow`), `x′ = x ⊕ y`, with
//! `y = −σ·F·o·(x̄ ⊖ x)` the pull toward the range's holding-weighted rate
//! `x̄` and `⊖` the flow law's inverse ([`relative`]): with `|F| ≤ 1`,
//! `o ≤ 1` and `|x̄ ⊖ x| < 1`, `|y| < 1`, so the rate stays inside the bound
//! by construction. A body alone in its range is untouched, and the pure Z
//! pole (`σ = 1`, `o = 0`) has no moment: its term is 0 and its push is 0,
//! so it neither couples nor is coupled. Its neighbours
//! are the bodies whose own enclosures meet ([`meets`]):
//! `|R_i − R_j| ≤ R_i·e_i + R_j·e_j`, `e = m/(m + H(<R))`.
//!
//! F4, THE LIGHT LEDGER ([`take`], [`ledger`]), the coupling's
//! conservation: light made = received + drained, every bell. Light is
//! received on a body's unlocked face, `1 − δ` on the continuum and `δ` on
//! the vacuum, `δ = H/(r + H)` the carry or pinch share
//! (`forms::passage::PassageState::share`); the rest drains. A ledger that does
//! not balance is refused `Unbalanced`, with its residue.
//!
//! EXACT. Every reading is exact over ℚ: no float, no trigonometric value,
//! no square read as a quantity, and every refusal is named
//! ([`CouplingRefusal`]).

mod dipole;
mod field;
mod light;

pub use dipole::{Dipole, Parcel, Parcels};
pub use field::{field, meets, nearness, pull, Alignment, Enclosure, Field, Meeting, Meets, Neighbour, Pull};
pub use light::{ledger, take, unlocked, Face, Ledger, Taken};

use crate::coordinate::{flow, Key, Plane, DEGREES_PER_TURN};
use crate::passage::PassageRefusal;
use crate::Q;
use num_traits::{One, Signed, Zero};

/// THE SHELL'S LAW ID, as the shared-law table names it (MU-EQUATIONS,
/// *Shared laws*): the form of LAW-COUPLING.
pub const LAW_ID: &str = "FORMS-COUPLING";

/// The form's version: an amendment, or an operation added to the form,
/// raises it.
pub const LAW_VERSION: u16 = 1;

/// WHY A COUPLING READING IS REFUSED, by name and with what was read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CouplingRefusal {
    /// A rate outside `(−1, 1)`, the luminal bound's share: the value as
    /// presented.
    NotShare(Box<Q>),
    /// A light ledger whose made is not received plus drained: the three
    /// as stated, and the residue `made − received − drained`.
    Unbalanced { made: Box<Q>, received: Box<Q>, drained: Box<Q>, residue: Box<Q> },
    /// A holding in a range that is not positive: zero (no holding) or
    /// below it. The holding as presented.
    NoHolding(Box<Q>),
    /// Frames of which one is below zero, or none held at all: the held
    /// and the open frames.
    NotFrames { held: Box<Q>, open: Box<Q> },
    /// A ratio `t` below zero, where `ν(t)` holds no share.
    NotRatio(Box<Q>),
    /// A dipole stated with its pole outside `[−1, 1]` or its open share
    /// outside `[0, 1]`.
    NotDipole { pole: Box<Q>, open: Box<Q> },
    /// A round separation outside `[0°, 180°]`, where the hemisphere's L1
    /// reading is not taken: the degrees as presented.
    NotSeparation(Box<Q>),
    /// A body with no turning plane to hold its frames on.
    NoPlanes,
    /// A plane named twice in one body's planes.
    PlaneRepeated(Key),
    /// A bond or peel on a plane the body does not hold.
    NoPlane(Key),
    /// A nearness outside `[0, 1]`: the value as presented.
    NotNearness(Box<Q>),
    /// A moment, hold, reach or light below zero: the value as presented.
    Negative(Box<Q>),
    /// A share of no whole: its part and the rest, which sum to zero.
    NoWhole { part: Box<Q>, rest: Box<Q> },
    /// An alignment on a key that holds no turning: the key and its plane
    /// (`None` for one axis).
    NotTurningPlane { key: Key, plane: Option<Plane> },
    /// A neighbour aligned on another plane than the field's: both keys.
    PlanesDiffer { field: Key, neighbour: Key },
    /// The face's own refusal, by the passage's name.
    Passage(Box<PassageRefusal>),
    /// A per-plane reading's keyed vector refused by the coordinate form
    /// (other than a plane named twice, which is `PlaneRepeated`).
    Form(Box<crate::coordinate::FormRefusal>),
}

impl From<PassageRefusal> for CouplingRefusal {
    fn from(r: PassageRefusal) -> CouplingRefusal {
        CouplingRefusal::Passage(Box::new(r))
    }
}

/// The degrees of a half turn on a round plane, 180.
fn half_turn() -> Q {
    Q::from_integer(DEGREES_PER_TURN.into()) / Q::from_integer(2.into())
}

/// THE HEMISPHERE'S L1 READING of a round separation `Δ ∈ [0°, 180°]` on a
/// turning plane: `hemisphere_l1(Δ) = 1 − Δ/90°`, first degree and exact.
/// It is a named L1 reading of a round angle, never itself a degree: aligned
/// at 0° (`+1`), uncoupled at 90° (0), opposed at 180° (`−1`). Refused `NotSeparation` with `Δ` outside `[0°, 180°]`.
pub fn hemisphere_l1(separation: &Q) -> Result<Q, CouplingRefusal> {
    let half = half_turn();
    match separation.is_negative() || separation > &half {
        true => Err(CouplingRefusal::NotSeparation(Box::new(separation.clone()))),
        false => Ok(Q::one() - separation * Q::from_integer(2.into()) / half),
    }
}

/// THE OPEN SHARE OF A RATIO, `ν(t) = t/(1 + t)` for `t = Q/A ≥ 0`; the
/// inversion reverts it, `ν(1/t) = 1 − ν(t)`. Refused `NotRatio` with
/// `t < 0`.
pub fn open_share(t: &Q) -> Result<Q, CouplingRefusal> {
    match t.is_negative() {
        true => Err(CouplingRefusal::NotRatio(Box::new(t.clone()))),
        false => Ok(t / (Q::one() + t)),
    }
}

/// THE HELD SHARE OF A RATIO, the cohesion (the viscosity; ruled
/// 2026-11-06): `c(t) = 1/(1 + t) = A/(A + Q)` for `t = Q/A ≥ 0`, the
/// complement of the fluidity, `c(t) = 1 − ν(t)`, and for `t > 0` the
/// fluidity of the inverted ratio, `c(t) = ν(1/t)`: the passage's
/// `t ↦ 1/t` (78.172) exchanges the two. Both are `½` at the throat
/// (`t = 1`); `c(0) = 1` at the closed pole. A reading of LAW-COUPLING v1's
/// own inversion, read per plane and never summed across planes. Refused
/// `NotRatio` with `t < 0`.
pub fn held_share(t: &Q) -> Result<Q, CouplingRefusal> {
    match t.is_negative() {
        true => Err(CouplingRefusal::NotRatio(Box::new(t.clone()))),
        false => Ok(Q::one() / (Q::one() + t)),
    }
}

/// A rate checked a share of the luminal bound, `(−1, 1)`.
fn rate(x: &Q) -> Result<&Q, CouplingRefusal> {
    match x.abs() >= Q::one() {
        true => Err(CouplingRefusal::NotShare(Box::new(x.clone()))),
        false => Ok(x),
    }
}

/// THE FLOW LAW'S INVERSE, `a ⊖ b = (a − b)/(1 − a·b) = a ⊕ (−b)`, for rates
/// in `(−1, 1)`: the rate `a` read from `b`. Refused `NotShare` with the
/// first rate outside `(−1, 1)`.
pub fn relative(a: &Q, b: &Q) -> Result<Q, CouplingRefusal> {
    let back = -rate(b)?;
    flow(rate(a)?, &back).map_err(|_| CouplingRefusal::NotShare(Box::new(b.clone())))
}

/// `x ⊕ y`, the flow law, refused `NotShare` with a rate outside `(−1, 1)`.
fn composed(x: &Q, y: &Q) -> Result<Q, CouplingRefusal> {
    flow(rate(x)?, rate(y)?).map_err(|_| CouplingRefusal::NotShare(Box::new(y.clone())))
}

/// `part/(part + rest)`, refused `NoWhole` where the two sum to zero.
fn share_of(part: &Q, rest: &Q) -> Result<Q, CouplingRefusal> {
    let whole = part + rest;
    match whole.is_zero() {
        true => Err(CouplingRefusal::NoWhole { part: Box::new(part.clone()), rest: Box::new(rest.clone()) }),
        false => Ok(part / whole),
    }
}

/// A quantity checked not below zero, refused `Negative` otherwise.
fn held(x: &Q) -> Result<&Q, CouplingRefusal> {
    match x.is_negative() {
        true => Err(CouplingRefusal::Negative(Box::new(x.clone()))),
        false => Ok(x),
    }
}
