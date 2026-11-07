// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! LAW-COORDINATE fixture 11b (proposal 78.173 §5, `FORMS-TURNING` v1),
//! ROADMAP track 26 S7 stage 3 — A ROTOR, A MOTION OF THE FIELD HELD BY ITS
//! RATE, and the composition of two turnings on planes that share a unit.
//!
//! A rotor holds an even generator `B`, its rate, with rational components
//! on its planes. Its motion `R = (1 + B)(1 − B)⁻¹` is read through
//! `forms::turning` whenever it is asked and never stored; two rotors
//! compose as `R₁R₂`, whole, and the rate is read back,
//! `B = (R − 1)(R + 1)⁻¹`. The planes are read from the rate's terms. A
//! rational rate names a Pythagorean angle, which is in general no rational
//! number of degrees (78.173 §8.3), so a rotor carries no degrees: its
//! angle on a plane is a reading of the rate, outside this carrier.
//!
//! A TURNING'S MOTION IS RATIONAL where its angle is a whole number of
//! quarter turns: `cos θ + B·sin θ` with both coefficients in ℚ and `θ` a
//! rational number of degrees holds only there (Niven). Two such turnings
//! on planes that share a unit compose by the law ([`shared`]): their
//! motions' product, read back as one turning on one plane, or as none.
//! Any other angle is refused `NotExact` with `Residue::IrrationalMotion`,
//! the plane and the angle. The degree convention is the carrier's own
//! (78.171: the motion `cos θ + B·sin θ`, one-sided); the factor of two
//! against `forms::turning::reading` is open in 78.173 §8.3.

use super::refusal::{FieldRefusal, Residue};
use super::turning::{Turning, TurningPlane};
use crate::core::Q;
use crate::transducer::cga::{base_axis, Clifford};
use forms::coordinate::{Axis, Key, Square};
use forms::turning::{compose, generator, motion, same};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, ToPrimitive, Zero};

/// A ROTOR: a motion of the field held by its rate `B`, an even generator
/// of `Cl(p, q)` with exact rational components. One constructor reads the
/// rate, one the motion; both are checked by the law, so no rotor holds a
/// rate that names no motion.
#[derive(Clone, Debug)]
pub struct Rotor {
    rate: Clifford,
}

impl PartialEq for Rotor {
    fn eq(&self, other: &Rotor) -> bool {
        same(&self.rate, &other.rate)
    }
}

impl Eq for Rotor {}

impl Rotor {
    /// THE ROTOR OF THE RATE `rate`. Refused as `forms::turning::motion`
    /// refuses: `NotGenerator` where a part is no generator's, `NoMotion`
    /// where `1 − B` has no inverse.
    pub fn of(rate: Clifford) -> Result<Rotor, FieldRefusal> {
        motion(&rate).map_err(FieldRefusal::Turning)?;
        Ok(Rotor { rate })
    }

    /// THE ROTOR OF THE MOTION `r`, its rate read back by the Cayley form.
    /// Refused as `forms::turning::generator` refuses: `NotEven` or
    /// `NotUnit` where `r` is no motion, `NoGenerator` where `R + 1` has no
    /// inverse (`R = −1` among them).
    pub fn of_motion(r: &Clifford) -> Result<Rotor, FieldRefusal> {
        Ok(Rotor { rate: generator(r).map_err(FieldRefusal::Turning)? })
    }

    /// The rate `B`.
    pub fn rate(&self) -> &Clifford {
        &self.rate
    }

    /// THE MOTION `R = (1 + B)(1 − B)⁻¹`, read from the rate.
    pub fn motion(&self) -> Result<Clifford, FieldRefusal> {
        motion(&self.rate).map_err(FieldRefusal::Turning)
    }

    /// THIS ROTOR, THEN `other`: the motions composed as `R₁R₂`, whole and
    /// never plane by plane, and the rate read back. Refused as
    /// [`Rotor::of_motion`] refuses where the composite has no rate.
    pub fn then(&self, other: &Rotor) -> Result<Rotor, FieldRefusal> {
        let composite = compose(&self.motion()?, &other.motion()?).map_err(FieldRefusal::Turning)?;
        Rotor::of_motion(&composite)
    }

    /// THE PLANES OF THE RATE: each part of grade 2 as the key of its two
    /// units, ascending (`e_+1+3`), with its exact component, in the
    /// rate's canonical order. A boost plane's key names its two units of
    /// opposite squares. Refused `NotPlanes` with the masks of the parts on
    /// no plane (grade 6 and above, in six or more frame units), and with
    /// the masks whose units the frame does not name.
    pub fn planes(&self) -> Result<Vec<(Key, Q)>, FieldRefusal> {
        let negatives = self.rate.negatives();
        let mut planes = Vec::new();
        let mut unplaned = Vec::new();
        for (mask, c) in self.rate.terms() {
            match key_of(negatives, mask) {
                Some(key) => planes.push((key, c.clone())),
                None => unplaned.push(mask),
            }
        }
        match unplaned.is_empty() {
            true => Ok(planes),
            false => Err(FieldRefusal::NotPlanes { masks: unplaned }),
        }
    }
}

/// THE FRAME UNIT OF ONE GENERATOR BIT under `negatives`: plane `k`'s
/// `e₊ₖ` is base generator `2k − 1` and `e₋ₖ` base generator `2k`
/// (`FramePlanes`); its square read from the value's negative mask must
/// agree. `None` for a bit no frame unit holds.
fn unit_of(negatives: usize, bit: usize) -> Option<Axis> {
    let g = (1..).map_while(|g| base_axis(g).map(|m| (g, m))).find(|(_, m)| *m == bit)?.0;
    let (square, k) = match g % 2 {
        1 => (Square::Positive, g.div_ceil(2)),
        _ => (Square::Negative, g / 2),
    };
    let held = match negatives & bit {
        0 => Square::Positive,
        _ => Square::Negative,
    };
    (held == square).then(|| Axis::unit(square, u32::try_from(k).ok()?).ok()).flatten()
}

/// The key of a grade-2 mask, its units ascending; `None` for any other
/// grade, or a bit no frame unit holds.
fn key_of(negatives: usize, mask: usize) -> Option<Key> {
    if mask.count_ones() != 2 {
        return None;
    }
    let low = mask & mask.wrapping_neg();
    Key::generator(unit_of(negatives, low)?, unit_of(negatives, mask ^ low)?).ok()
}

/// THE TURNING'S MOTION AS A VALUE, `cos θ + B·sin θ` on its plane, where
/// it is rational: `θ` a whole number of quarter turns. Refused `NotExact`
/// with `IrrationalMotion`, the plane and the angle, at any other angle.
fn rational_motion(t: &Turning) -> Result<Clifford, FieldRefusal> {
    let quarters = t.degrees() / Q::from_integer(BigInt::from(90));
    let irrational = || FieldRefusal::NotExact { residue: Residue::IrrationalMotion { plane: t.plane().key().clone(), degrees: t.degrees().clone() } };
    if !quarters.is_integer() {
        return Err(irrational());
    }
    let b = t.plane().bivector();
    let (cos, sin) = match quarters.to_integer().mod_floor(&BigInt::from(4)).to_u8() {
        Some(0) => (1, 0),
        Some(1) => (0, 1),
        Some(2) => (-1, 0),
        Some(3) => (0, -1),
        _ => return Err(irrational()),
    };
    let scalar = Clifford::blade(b.negatives(), 0, Q::from_integer(BigInt::from(cos))).map_err(FieldRefusal::Mismatched)?;
    let turned = b.product(&Clifford::blade(b.negatives(), 0, Q::from_integer(BigInt::from(sin))).map_err(FieldRefusal::Mismatched)?).map_err(FieldRefusal::Mismatched)?;
    scalar.sum(&turned).map_err(FieldRefusal::Mismatched)
}

/// TWO TURNINGS ON PLANES THAT SHARE A UNIT, `held` then `arriving`,
/// composed by the law: their rational motions' product `R₁R₂`, the rate
/// read back as a [`Rotor`] and its one plane read, as one turning on that
/// plane by the composite's angle (the carrier's `cos θ + B·sin θ`, so a
/// rate `±1` is `±90°`), or as none where the composite is the identity.
/// The plane is one of `candidates` (the two planes and the plane between
/// them), in the orientation the key names; reversed, the angle reads with
/// the opposite sign.
///
/// Refused `NotExact`: with `IrrationalMotion` where either angle is no
/// whole number of quarter turns; with `Uncommuted` naming both planes
/// where either carries a winding, which a composite on another plane does
/// not keep (78.173 §8.3: how `K` is held beside `R` is open).
pub(super) fn shared(held: &Turning, arriving: &Turning, candidates: &[TurningPlane]) -> Result<Option<Turning>, FieldRefusal> {
    let first = rational_motion(held)?;
    let then = rational_motion(arriving)?;
    let uncommuted = || FieldRefusal::NotExact { residue: Residue::Uncommuted { held: held.plane().key().clone(), arriving: arriving.plane().key().clone() } };
    if !held.winding().is_zero() || !arriving.winding().is_zero() {
        return Err(uncommuted());
    }
    let composite = compose(&first, &then).map_err(FieldRefusal::Turning)?;
    let planes = Rotor::of_motion(&composite)?.planes()?;
    let (key, rate) = match planes.as_slice() {
        [] => return Ok(None),
        [one] => one,
        _ => return Err(uncommuted()),
    };
    let quarter = Q::from_integer(BigInt::from(90));
    let one = Q::one();
    let angle = match (rate == &one, rate == &-one.clone()) {
        (true, _) => quarter,
        (_, true) => -quarter,
        _ => return Err(uncommuted()),
    };
    for plane in candidates {
        let [x, y] = plane.key().axes() else { return Err(uncommuted()) };
        let reversed = Key::generator(y.clone(), x.clone()).map_err(FieldRefusal::NotTurningPlane)?;
        match (plane.key() == key, &reversed == key) {
            (true, _) => return Turning::of(plane.clone(), angle.clone()).map(Some),
            (_, true) => return Turning::of(plane.clone(), -angle.clone()).map(Some),
            _ => {}
        }
    }
    Err(uncommuted())
}
