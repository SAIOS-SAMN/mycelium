// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! LAW-FIELD v1 §3.1–§3.2 (proposal 78.171) — A MOTION OF TURNINGS, ITS
//! HOLONOMY AND ITS MONODROMY: the product of a revolution's transitions, read per
//! named plane in degrees with its winding, and its order.

use super::refusal::{FieldRefusal, Residue};
use super::rotor::shared;
use super::turning::{Meeting, Turning, TurningPlane};
use forms::coordinate::Key;
use num_traits::Zero;

/// A MOTION OF THE FIELD held as turnings on pairwise disjoint planes, one
/// per plane, in the order of the planes' keys (H's shape: one component
/// per turning plane). Turnings on disjoint planes commute, so the motion
/// is their product in any order. A plane whose angle composes to exactly
/// `0°` holds no component. The identity holds none.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Motion {
    turnings: Vec<Turning>,
}

impl Motion {
    /// THE IDENTITY, the empty product: no turning on any plane.
    pub fn identity() -> Motion {
        Motion { turnings: Vec::new() }
    }

    /// THE MOTION OF `turnings` COMPOSED IN ORDER. Refused as
    /// [`Motion::then`] refuses.
    pub fn of(turnings: Vec<Turning>) -> Result<Motion, FieldRefusal> {
        turnings.into_iter().try_fold(Motion::identity(), |m, t| m.then(&Motion { turnings: vec![t] }))
    }

    /// The turnings, one per plane, in key order.
    pub fn turnings(&self) -> &[Turning] {
        &self.turnings
    }

    /// THIS MOTION, THEN `other`: each arriving turning composed with the
    /// held turning on its plane, or held beside the others where its plane
    /// is disjoint from every held plane. An arriving turning on a plane
    /// that shares one unit with one held plane composes with it by the law
    /// of fixture 11b (`rotor::shared`): the two motions' product `R₁R₂`,
    /// its rate read back and its one plane read, held as one turning on
    /// that plane (disjoint from every other held plane, since the arriving
    /// plane is), or as none. Refused `NotExact` with `IrrationalMotion`
    /// where either motion is not rational (its angle no whole number of
    /// quarter turns); with `Uncommuted` naming the two planes where the
    /// arriving plane shares a unit with more than one held plane, or
    /// either turning carries a winding; and `Mismatched` where two planes'
    /// signatures disagree.
    pub fn then(&self, other: &Motion) -> Result<Motion, FieldRefusal> {
        let mut held = self.turnings.clone();
        for arriving in other.turnings.iter().filter(|t| !t.degrees().is_zero()) {
            let mut joined = None;
            let mut sharing = None;
            for (i, h) in held.iter().enumerate() {
                match h.plane().meeting(arriving.plane())? {
                    Meeting::Along | Meeting::Against => joined = Some((i, h.then(arriving)?)),
                    Meeting::Disjoint => {}
                    Meeting::Sharing => match sharing {
                        None => sharing = Some(i),
                        Some(_) => {
                            return Err(FieldRefusal::NotExact {
                                residue: Residue::Uncommuted { held: h.plane().key().clone(), arriving: arriving.plane().key().clone() },
                            })
                        }
                    },
                }
            }
            if let Some(i) = sharing {
                let h = held.remove(i);
                let candidates: Vec<TurningPlane> = [Some(h.plane().clone()), Some(arriving.plane().clone()), h.plane().between(arriving.plane())].into_iter().flatten().collect();
                held.extend(shared(&h, arriving, &candidates)?);
                continue;
            }
            match joined.and_then(|(i, t)| held.get_mut(i).map(|slot| (slot, t))) {
                Some((slot, t)) => *slot = t,
                None => held.push(arriving.clone()),
            }
        }
        held.retain(|t| !t.degrees().is_zero());
        #[expect(clippy::disallowed_methods, reason = "rule 4 — a sort by a canonical coordinate: the plane's key written out")]
        held.sort_by(|a, b| a.plane().key().cmp(b.plane().key()));
        Ok(Motion { turnings: held })
    }
}

/// THE MONODROMY OF A CANDIDATE (78.171 §3.2; MU-SENSING §3.8): flat (order
/// 1, an exact return of the fibre), torsion of order `ℓ` (it closes after
/// `ℓ` circuits), or rupture (no finite order: it never closes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monodromy {
    Flat,
    Torsion { order: usize },
    Rupture,
}

/// THE HOLONOMY OF A REVOLUTION (78.171 §3.1): the product of its transitions,
/// held as the motion it composes to, read per plane: an exact angle in
/// degrees on the named plane, its winding in ℤ and its order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Holonomy {
    motion: Motion,
}

impl Holonomy {
    /// The holonomy of the composed `motion`.
    pub fn of(motion: Motion) -> Holonomy {
        Holonomy { motion }
    }

    /// The turnings it composes to, one per plane, in key order: none where
    /// the revolution returns exactly (a resonant orbit).
    pub fn turnings(&self) -> &[Turning] {
        self.motion.turnings()
    }

    /// THE ANGLE ON ONE PLANE: the one turning the holonomy holds. Refused
    /// `ScalarCollapse` with the planes it turns on where it turns on none
    /// or on several: an angle is stated on its plane, never without one,
    /// and several planes are never summed into one number.
    pub fn angle(&self) -> Result<&Turning, FieldRefusal> {
        match self.turnings() {
            [one] => Ok(one),
            all => Err(FieldRefusal::ScalarCollapse { planes: all.iter().map(|t| t.plane().key().clone()).collect() }),
        }
    }

    /// The planes the holonomy turns on, in key order.
    pub fn planes(&self) -> Vec<Key> {
        self.turnings().iter().map(|t| t.plane().key().clone()).collect()
    }

    /// THE MONODROMY: the least common multiple of the planes' orders,
    /// flat where it is 1 (every plane's motion the identity, whatever its
    /// winding), rupture where a plane's motion has no finite order.
    pub fn monodromy(&self) -> Monodromy {
        let order = self.turnings().iter().try_fold(1usize, |l, t| {
            let o = t.order()?;
            (l / num_integer::gcd(l, o)).checked_mul(o)
        });
        match order {
            Some(1) => Monodromy::Flat,
            Some(order) => Monodromy::Torsion { order },
            None => Monodromy::Rupture,
        }
    }
}
