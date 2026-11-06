// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// LAW-COUPLING v1 (proposal 78.175, F3; netstratum docs/121 §9.2, §10.2–
// §10.4, §11) — THE DIPOLE COUPLING ON A TURNING PLANE: the range's field
// as one, the pull it gives a body's rate by the flow law, and the
// neighbours, the bodies whose own enclosures meet.

use super::{composed, held, relative, share_of, CouplingRefusal, Dipole};
use crate::coordinate::{Key, Plane};
use crate::Q;
use num_traits::{One, Signed, Zero};
use std::cmp::Ordering;

/// HOW TWO DIPOLES STAND ON ONE NAMED TURNING PLANE: the alignment
/// `h ∈ [−1, 1]`, aligned toward `+1`, uncoupled at 0, opposed toward `−1`.
/// It is an input: the reading of `h` from a separation in degrees is held
/// until both operators rule which circle shared degrees stand on (78.175
/// §8).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Alignment {
    plane: Key,
    along: Q,
}

impl Alignment {
    /// The alignment `along` on `plane`. Refused `NotTurningPlane` where
    /// the key holds no turning, `NotAlignment` outside `[−1, 1]`.
    pub fn on(plane: Key, along: Q) -> Result<Alignment, CouplingRefusal> {
        match (plane.plane(), along.abs() > Q::one()) {
            (Some(Plane::Turning(_)), false) => Ok(Alignment { plane, along }),
            (Some(Plane::Turning(_)), true) => Err(CouplingRefusal::NotAlignment(Box::new(along))),
            (other, _) => Err(CouplingRefusal::NotTurningPlane { key: plane, plane: other }),
        }
    }

    /// The turning plane it is read on.
    pub fn plane(&self) -> &Key {
        &self.plane
    }

    /// `h`, in `[−1, 1]`.
    pub fn along(&self) -> &Q {
        &self.along
    }
}

/// ONE BODY OF A RANGE, as the field reads it: its holding `w > 0`, its
/// dipole, its nearness `n ∈ [0, 1]` and its alignment to the body read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Neighbour {
    holding: Q,
    dipole: Dipole,
    nearness: Q,
    alignment: Alignment,
}

impl Neighbour {
    /// The neighbour. Refused `NoHolding` where the holding is not
    /// positive, `NotNearness` with the nearness outside `[0, 1]`.
    pub fn of(holding: Q, dipole: Dipole, nearness: Q, alignment: Alignment) -> Result<Neighbour, CouplingRefusal> {
        if !holding.is_positive() {
            return Err(CouplingRefusal::NoHolding(Box::new(holding)));
        }
        if nearness.is_negative() || nearness > Q::one() {
            return Err(CouplingRefusal::NotNearness(Box::new(nearness)));
        }
        Ok(Neighbour { holding, dipole, nearness, alignment })
    }

    pub fn holding(&self) -> &Q {
        &self.holding
    }

    pub fn dipole(&self) -> &Dipole {
        &self.dipole
    }

    pub fn nearness(&self) -> &Q {
        &self.nearness
    }

    pub fn alignment(&self) -> &Alignment {
        &self.alignment
    }
}

/// THE RING LAW'S NEARNESS of a moment `M` at the gap `g`:
/// `n = M/(|g| + M)`, in `[0, 1]`. Refused `Negative` with `M < 0`,
/// `NoWhole` where `M` and `g` are both zero.
pub fn nearness(moment: &Q, gap: &Q) -> Result<Q, CouplingRefusal> {
    share_of(held(moment)?, &gap.abs())
}

/// THE RANGE'S FIELD AS ONE, on one turning plane.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Field {
    /// No body in the range: the body is untouched.
    Alone,
    /// The range's whole holding `Σ w` and its field
    /// `F = Σ w·σ·o·n·h / Σ w`, in `[−1, 1]`.
    Held { holding: Q, field: Q },
}

impl Field {
    /// The field's value: zero alone, `F` held.
    pub fn value(&self) -> Q {
        match self {
            Field::Alone => Q::zero(),
            Field::Held { field, .. } => field.clone(),
        }
    }
}

/// THE FIELD OF `range` ON `plane`: `F = Σ w_j·σ_j·o_j·n_j·h_j / Σ w_j`,
/// against the range's whole holding. Refused `NotTurningPlane` where the
/// plane holds no turning, `PlanesDiffer` where a neighbour is aligned on
/// another plane.
pub fn field(plane: &Key, range: &[Neighbour]) -> Result<Field, CouplingRefusal> {
    if !matches!(plane.plane(), Some(Plane::Turning(_))) {
        return Err(CouplingRefusal::NotTurningPlane { key: plane.clone(), plane: plane.plane() });
    }
    if let Some(j) = range.iter().find(|j| j.alignment.plane() != plane) {
        return Err(CouplingRefusal::PlanesDiffer { field: plane.clone(), neighbour: j.alignment.plane().clone() });
    }
    let holding = range.iter().fold(Q::zero(), |s, j| s + &j.holding);
    match range.is_empty() {
        true => Ok(Field::Alone),
        false => {
            let pulled = range.iter().fold(Q::zero(), |s, j| s + &j.holding * &j.dipole.pole * &j.dipole.open * &j.nearness * j.alignment.along());
            Ok(Field::Held { field: pulled / &holding, holding })
        }
    }
}

/// THE PULL ON A BODY'S RATE: the gap `x̄ ⊖ x` to the range's
/// holding-weighted rate, the push `y = −σ·F·o·(x̄ ⊖ x)` and the rate past
/// it, `x′ = x ⊕ y`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pull {
    pub gap: Q,
    pub push: Q,
    pub rate: Q,
}

/// THE FIELD'S PULL on the body of dipole `body` at rate `x`, its range's
/// holding-weighted rate `x̄`: opposed dipoles lock (the rate drawn toward
/// `x̄`), aligned ones split (pushed from it); alone, the rate is kept.
/// Refused `NotShare` with a rate outside `(−1, 1)`.
pub fn pull(x: &Q, x_bar: &Q, body: &Dipole, field: &Field) -> Result<Pull, CouplingRefusal> {
    let gap = relative(x_bar, x)?;
    let push = -(&body.pole * field.value() * &body.open * &gap);
    let rate = composed(x, &push)?;
    Ok(Pull { gap, push, rate })
}

/// A BODY'S OWN ENCLOSURE: its reach `R`, its moment `m` and the hold
/// within its reach, `H(<R)`, each not below zero; its share
/// `e = m/(m + H(<R))` encloses `R·e` about it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Enclosure {
    pub reach: Q,
    pub moment: Q,
    pub hold: Q,
}

impl Enclosure {
    /// The reach it encloses, `R·e`. Refused `Negative` with a part below
    /// zero, `NoWhole` where `m + H(<R)` is zero.
    pub fn enclosed(&self) -> Result<Q, CouplingRefusal> {
        Ok(held(&self.reach)? * share_of(held(&self.moment)?, held(&self.hold)?)?)
    }
}

/// WHERE TWO ENCLOSURES STAND: overlapping, touching at the bound, or
/// apart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Meets {
    /// `|R_i − R_j| < R_i·e_i + R_j·e_j`: neighbours.
    Within,
    /// `|R_i − R_j| = R_i·e_i + R_j·e_j`: neighbours, at the bound.
    Bound,
    /// `|R_i − R_j| > R_i·e_i + R_j·e_j`: not neighbours.
    Apart,
}

/// THE NEIGHBOUR READING: the separation `|R_i − R_j|`, the reach the two
/// enclosures hold together `R_i·e_i + R_j·e_j`, and where they stand.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Meeting {
    pub separation: Q,
    pub reach: Q,
    pub meets: Meets,
}

/// WHETHER TWO BODIES ARE NEIGHBOURS: their own enclosures meet,
/// `|R_i − R_j| ≤ R_i·e_i + R_j·e_j`. Refused as [`Enclosure::enclosed`] is.
pub fn meets(a: &Enclosure, b: &Enclosure) -> Result<Meeting, CouplingRefusal> {
    let reach = a.enclosed()? + b.enclosed()?;
    let separation = (&a.reach - &b.reach).abs();
    let meets = match separation.cmp(&reach) {
        Ordering::Less => Meets::Within,
        Ordering::Equal => Meets::Bound,
        Ordering::Greater => Meets::Apart,
    };
    Ok(Meeting { separation, reach, meets })
}
