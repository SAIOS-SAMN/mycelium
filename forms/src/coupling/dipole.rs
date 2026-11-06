// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// LAW-COUPLING v1 (proposal 78.175, F1 and F2; netstratum docs/121 §9.1,
// §10.1) — THE DIPOLE PARCEL, ITS FLUIDITY AND ITS VISCOSITY, PER PLANE: a
// parcel's frames, its pole and its open share (the fluidity; the held
// share is the viscosity), one law, each a ratio of the frames;
// a body's frames on each turning plane, founded by the even split.

use super::{half_turn, share_of, CouplingRefusal};
use crate::coordinate::{FormRefusal, Key, Plane, Vector};
use crate::Q;
use num_traits::{One, Signed, Zero};

/// A PARCEL'S FRAMES: `A` held (closed) and `Q` open, each not below zero
/// and not both zero. The pole stands on these frames, never on the
/// electrons' phase `R`, which is the parcel's phase.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parcel {
    held: Q,
    open: Q,
}

/// A DIPOLE: its pole `σ ∈ [−1, 1]`, closed against open, and its strength,
/// the open share `o ∈ [0, 1]`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dipole {
    pub pole: Q,
    pub open: Q,
}

impl Parcel {
    /// The parcel of `held` (`A`) and `open` (`Q`) frames. Refused
    /// `NotFrames` where one is below zero or both are zero.
    pub fn of(held: Q, open: Q) -> Result<Parcel, CouplingRefusal> {
        match held.is_negative() || open.is_negative() || (held.is_zero() && open.is_zero()) {
            true => Err(CouplingRefusal::NotFrames { held: Box::new(held), open: Box::new(open) }),
            false => Ok(Parcel { held, open }),
        }
    }

    /// THE PARCEL OF A KIND: `Z` and `U`, held `A = W = 3Z − U` and open
    /// `Q = U`. An open-heavy kind (`σ < 0`) needs `U > 3Z/2`. Refused
    /// `NotFrames` where `U > 3Z` or `U < 0`.
    pub fn kind(z: &Q, u: &Q) -> Result<Parcel, CouplingRefusal> {
        Parcel::of(Q::from_integer(3.into()) * z - u, u.clone())
    }

    /// The held frames `A`.
    pub fn held(&self) -> &Q {
        &self.held
    }

    /// The open frames `Q`.
    pub fn open(&self) -> &Q {
        &self.open
    }

    /// The ratio `t = Q/A`; none at the open pole, `A = 0`.
    pub fn ratio(&self) -> Option<Q> {
        (!self.held.is_zero()).then(|| &self.open / &self.held)
    }

    /// THE OPEN SHARE `o = Q/(A + Q) = ν(t)`, the parcel's strength, its
    /// fluidity. Its complement, the held share `A/(A + Q)`, is the
    /// viscosity (the cohesion).
    pub fn open_share(&self) -> Result<Q, CouplingRefusal> {
        share_of(&self.open, &self.held)
    }

    /// THE DIPOLE THE FRAMES HOLD: the pole `σ = (A − Q)/(A + Q)`, closed
    /// against open, continuous in `[−1, 1]` (a ratio, which no metric
    /// enters), and the open share `o = Q/(A + Q)`.
    pub fn dipole(&self) -> Result<Dipole, CouplingRefusal> {
        let open = self.open_share()?;
        // (A − Q)/(A + Q) = 1 − 2·Q/(A + Q): the held share less the open.
        let pole = Q::one() - Q::from_integer(2.into()) * &open;
        Ok(Dipole { pole, open })
    }

    /// THE HELD SHARE `c = A/(A + Q) = 1 − ν(t)`, the parcel's cohesion
    /// (the viscosity; ruled 2026-11-06), read on its frames: the inversion
    /// takes it to the open share.
    pub fn held_share(&self) -> Result<Q, CouplingRefusal> {
        share_of(&self.held, &self.open)
    }

    /// THE INVERSION: the frames exchanged, `t ↦ 1/t`, so `o ↦ 1 − o` and
    /// `σ ↦ −σ`.
    pub fn inverted(&self) -> Parcel {
        Parcel { held: self.open.clone(), open: self.held.clone() }
    }

    /// THE POLE'S L1 ARC, a named L1 reading and never a shared degree:
    /// `θ_L1 = 180°·q/(z + q)`, 0 at the closed pole, 90 at the throat, 180
    /// at the open pole, measured along the L1 diamond.
    pub fn arc_l1(&self) -> Result<Q, CouplingRefusal> {
        Ok(half_turn() * self.open_share()?)
    }
}

/// A BODY'S FRAMES PER TURNING PLANE (netstratum 3Q): `(z_k, q_k)` on each
/// plane, keyed by the plane's generator, in the order founded. A bond or a
/// peel changes one plane only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parcels {
    planes: Vec<(Key, Parcel)>,
}

impl Parcels {
    /// THE BODY OF KIND `(Z, U)` FOUNDED on `planes` by the even split: each
    /// plane holds `((3Z − U)/n, U/n)`, `n` the planes' count. Refused
    /// `NoPlanes` with none, `NotTurningPlane` on a key that holds no
    /// turning, `PlaneRepeated` on a plane named twice, and as
    /// [`Parcel::kind`] is.
    pub fn founded(z: &Q, u: &Q, planes: &[Key]) -> Result<Parcels, CouplingRefusal> {
        let whole = Parcel::kind(z, u)?;
        let n = Q::from_integer(planes.len().into());
        if planes.is_empty() {
            return Err(CouplingRefusal::NoPlanes);
        }
        let mut held: Vec<(Key, Parcel)> = Vec::with_capacity(planes.len());
        for key in planes {
            if !matches!(key.plane(), Some(Plane::Turning(_))) {
                return Err(CouplingRefusal::NotTurningPlane { key: key.clone(), plane: key.plane() });
            }
            if held.iter().any(|(k, _)| k == key) {
                return Err(CouplingRefusal::PlaneRepeated(key.clone()));
            }
            held.push((key.clone(), Parcel { held: &whole.held / &n, open: &whole.open / &n }));
        }
        Ok(Parcels { planes: held })
    }

    /// The planes and their frames, in the order founded.
    pub fn planes(&self) -> &[(Key, Parcel)] {
        &self.planes
    }

    /// THE BODY'S COHESION, PER PLANE: each plane's held share
    /// `c_k = z_k/(z_k + q_k)` ([`Parcel::held_share`]), a keyed vector
    /// over its turning planes, its components never summed into one
    /// another nor averaged.
    pub fn cohesion(&self) -> Result<Vector, CouplingRefusal> {
        self.per_plane(Parcel::held_share)
    }

    /// THE BODY'S FLUIDITY, PER PLANE: each plane's open share
    /// `ν(t_k) = q_k/(z_k + q_k)` ([`Parcel::open_share`]), keyed as
    /// [`Parcels::cohesion`] is and never summed.
    pub fn fluidity(&self) -> Result<Vector, CouplingRefusal> {
        self.per_plane(Parcel::open_share)
    }

    /// One reading per plane, keyed by the plane.
    fn per_plane(&self, read: fn(&Parcel) -> Result<Q, CouplingRefusal>) -> Result<Vector, CouplingRefusal> {
        let components = self.planes.iter().map(|(k, p)| Ok((k.clone(), read(p)?))).collect::<Result<Vec<_>, CouplingRefusal>>()?;
        Vector::of(components).map_err(|e| match e {
            FormRefusal::KeyRepeated { key } => CouplingRefusal::PlaneRepeated(key),
            other => CouplingRefusal::Form(Box::new(other)),
        })
    }

    /// The frames on `plane`, if the body holds it.
    pub fn on(&self, plane: &Key) -> Option<&Parcel> {
        self.planes.iter().find(|(k, _)| k == plane).map(|(_, p)| p)
    }

    /// A BOND ON ONE PLANE: `closed` open frames pair and close there,
    /// `q ↦ q − closed`, `z ↦ z + closed`; a negative `closed` is a peel.
    /// Every other plane is unchanged. Refused `NoPlane` on a plane the
    /// body does not hold, and `NotFrames` where the plane's frames would
    /// fall below zero.
    pub fn bond(&self, plane: &Key, closed: &Q) -> Result<Parcels, CouplingRefusal> {
        let mut planes = self.planes.clone();
        let (_, at) = planes.iter_mut().find(|(k, _)| k == plane).ok_or_else(|| CouplingRefusal::NoPlane(plane.clone()))?;
        *at = Parcel::of(&at.held + closed, &at.open - closed)?;
        Ok(Parcels { planes })
    }
}

impl Dipole {
    /// A dipole stated by its pole and open share, as a kernel carries one.
    /// Refused `NotDipole` with the pole outside `[−1, 1]` or the open share
    /// outside `[0, 1]`.
    pub fn stated(pole: Q, open: Q) -> Result<Dipole, CouplingRefusal> {
        match pole.abs() > Q::one() || open.is_negative() || open > Q::one() {
            true => Err(CouplingRefusal::NotDipole { pole: Box::new(pole), open: Box::new(open) }),
            false => Ok(Dipole { pole, open }),
        }
    }
}
