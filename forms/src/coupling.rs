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
//! HELD (78.175 §8): netstratum's amendment of 2026-11-06 reads the pole
//! and the alignment from degrees through the L1 cosine, `1 − θ/90°` on the
//! L1 diamond. The degrees of LAW-COORDINATE, LAW-PASSAGE and LAW-FIELD are
//! rotation angles on Clifford planes (the round circle, `ζ_ℓ`), and the two
//! readings agree only at multiples of 45°: which circle shared degrees
//! stand on waits on both operators' ruling. Until then this form holds no
//! map from degrees to a pole or an alignment: the alignment `h` is an
//! input in `[−1, 1]`, and the pole is read from the frames as a ratio,
//! which no metric enters.
//!
//! F1 AND F2, ONE LAW: THE DIPOLE PARCEL AND ITS VISCOSITY ([`Parcel`],
//! [`Dipole`]). A parcel's frames are `A = W = 3Z − U` held and `Q = U`
//! open (`Parcel::kind`), `t = Q/A`.
//! - The pole is `σ = (A − Q)/(A + Q)`, continuous in `[−1, 1]`: `+1` at
//!   the closed pole, 0 at the throat, `−1` at the open pole. It stands on
//!   the frames, never on the electrons' phase `R`, which is the parcel's
//!   phase.
//! - The strength is the open share `o = Q/(A + Q) = ν(t)`, with
//!   `ν(t) = t/(1 + t)` ([`open_share`]), and the inversion reverts it:
//!   `ν(1/t) = 1 − ν(t)`, the frames exchanged ([`Parcel::inverted`]).
//!
//! F3, THE DIPOLE COUPLING ON A TURNING PLANE ([`field`], [`pull`]). The
//! field of a body's range acts as one, on one named turning plane
//! (`forms::coordinate::Plane::Turning`):
//! `F = Σ w_j·σ_j·o_j·n_j·h_j / Σ w_j`, with `n = M/(|g| + M)` the ring
//! law's nearness ([`nearness`]) and `h ∈ [−1, 1]` the alignment on that
//! plane ([`Alignment`]): aligned toward `+1`, uncoupled at 0, opposed
//! toward `−1`. Each term stands in `[−1, 1]`, so `|F| ≤ 1`. A rate is a
//! share `x ∈ (−1, 1)` of the luminal bound and moves by LAW-COORDINATE's
//! flow law (`forms::coordinate::flow`), `x′ = x ⊕ y`, with
//! `y = −σ·F·o·(x̄ ⊖ x)` the pull toward the range's holding-weighted rate
//! `x̄` and `⊖` the flow law's inverse ([`relative`]): with `|F| ≤ 1`,
//! `o ≤ 1` and `|x̄ ⊖ x| < 1`, `|y| < 1`, so the rate stays inside the bound
//! by construction. A body alone in its range is untouched. Its neighbours
//! are the bodies whose own enclosures meet ([`meets`]):
//! `|R_i − R_j| ≤ R_i·e_i + R_j·e_j`, `e = m/(m + H(<R))`.
//!
//! F4, THE LIGHT LEDGER ([`take`], [`ledger`]), the coupling's
//! conservation: light made = received + drained, every bell. Light is
//! received on a body's unlocked face, `1 − δ` on the continuum and `δ` on
//! the vacuum, `δ = H/(r + H)` the carry or pinch share
//! (`forms::passage::Frame::share`); the rest drains. A ledger that does
//! not balance is refused `Unbalanced`, with its residue.
//!
//! EXACT. Every reading is exact over ℚ: no float, no trigonometric value,
//! no square read as a quantity, and every refusal is named
//! ([`CouplingRefusal`]).

mod dipole;
mod field;
mod light;

pub use dipole::{Dipole, Parcel};
pub use field::{field, meets, nearness, pull, Alignment, Enclosure, Field, Meeting, Meets, Neighbour, Pull};
pub use light::{ledger, take, unlocked, Face, Ledger, Taken};

use crate::coordinate::{flow, Key, Plane};
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
    /// An alignment outside `[−1, 1]`: the value as presented.
    NotAlignment(Box<Q>),
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
}

impl From<PassageRefusal> for CouplingRefusal {
    fn from(r: PassageRefusal) -> CouplingRefusal {
        CouplingRefusal::Passage(Box::new(r))
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
