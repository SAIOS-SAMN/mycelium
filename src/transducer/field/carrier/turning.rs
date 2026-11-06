// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! LAW-FIELD v1 §1a, §2 (a point, a motion; proposal 78.171 amendments (b),
//! (c)) — THE FRAME PLANES, A TURNING PLANE AND A TURNING: an angle exact in
//! degrees on a named plane, its winding kept, its motion over `ℚ(ζ_ℓ)`.

use super::cyclotomic::Cyclotomic;
use super::refusal::{FieldRefusal, Residue};
use crate::core::Q;
use crate::transducer::cga::{base_axis, Clifford};
use forms::coordinate::{Axis, Key, Plane, Square, DEGREES_PER_TURN};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive};

/// THE FIELD'S FRAME PLANES (78.171 §1a): plane `k` is held by its two
/// units, `e₊ₖ` squaring to +1 and `e₋ₖ` to −1, so the algebra is
/// `Cl(p, q)` with `p = q` the count of planes, its signature read per
/// plane from each value's negative mask, never a fixed `Cl(n+1, 1)`.
/// `e₊ₖ` is base generator `2k − 1` and `e₋ₖ` base generator `2k`, so no
/// unit of one plane is a unit of another.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FramePlanes {
    count: u32,
}

impl FramePlanes {
    /// THE FRAME OF `count` PLANES. Refused `PastFrame` with the first
    /// plane whose units the algebra's masks cannot hold.
    pub fn of(count: u32) -> Result<FramePlanes, FieldRefusal> {
        let frame = FramePlanes { count };
        match (1..=count).find(|&k| frame.unit(Square::Negative, k).is_none()) {
            Some(plane) => Err(FieldRefusal::PastFrame { plane }),
            None => Ok(frame),
        }
    }

    /// How many frame planes the field holds.
    pub fn count(&self) -> u32 {
        self.count
    }

    /// The generator mask of plane `k`'s unit of square `square`; `None`
    /// past the algebra's masks or for plane 0.
    fn unit(&self, square: Square, k: u32) -> Option<usize> {
        let k = usize::try_from(k).ok()?;
        let generator = match square {
            Square::Positive => k.checked_mul(2)?.checked_sub(1)?,
            Square::Negative => k.checked_mul(2)?,
        };
        base_axis(generator)
    }

    /// THE NEGATIVE MASK OF THE FIELD: every plane's `e₋ₖ`.
    pub fn negatives(&self) -> usize {
        (1..=self.count).filter_map(|k| self.unit(Square::Negative, k)).fold(0, |m, u| m | u)
    }

    /// PLANE `k`'S UNIT OF SQUARE `square` as a value of the field's
    /// algebra, at coefficient one. Refused `PastFrame` past the field's
    /// planes.
    pub fn unit_value(&self, square: Square, k: u32) -> Result<Clifford, FieldRefusal> {
        let mask = (k <= self.count).then(|| self.unit(square, k)).flatten().ok_or(FieldRefusal::PastFrame { plane: k })?;
        Clifford::blade(self.negatives(), mask, Q::one()).map_err(FieldRefusal::Mismatched)
    }
}

/// HOW TWO TURNING PLANES MEET: one plane (in the same orientation or
/// reversed), disjoint (no unit shared, so their turnings commute), or
/// sharing one unit (their turnings do not commute).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Meeting {
    Along,
    Against,
    Disjoint,
    Sharing,
}

/// A TURNING PLANE (FORMS-COORDINATE v4's H key): two frame units of one
/// square on two planes, `e₊ₖe₊ₗ` (written `e_+k+l`) or `e₋ₖe₋ₗ`, its
/// bivector `B` with `B² = −1` read from the units' squares, never computed
/// as a quantity. Its orientation is the order written: `e_+2+1` is
/// `e_+1+2` reversed.
#[derive(Clone, Debug)]
pub struct TurningPlane {
    key: Key,
    units: (usize, usize),
    bivector: Clifford,
}

impl TurningPlane {
    /// THE TURNING PLANE `e_{square k} e_{square l}` of `frame`. Refused
    /// `NotTurningPlane` where `k = l` or a unit has no writing, and
    /// `PastFrame` past the frame's planes.
    pub fn of(frame: &FramePlanes, square: Square, k: u32, l: u32) -> Result<TurningPlane, FieldRefusal> {
        let axis = |i| Axis::unit(square, i).map_err(FieldRefusal::NotTurningPlane);
        let key = Key::generator(axis(k)?, axis(l)?).map_err(FieldRefusal::NotTurningPlane)?;
        let (Some(Plane::Turning(_)), Some(a), Some(b)) = (key.plane(), frame.unit(square, k), frame.unit(square, l)) else {
            return Err(FieldRefusal::PastFrame { plane: k.max(l) });
        };
        if k.max(l) > frame.count {
            return Err(FieldRefusal::PastFrame { plane: k.max(l) });
        }
        let sign = match a < b {
            true => Q::one(),
            false => -Q::one(),
        };
        let bivector = Clifford::blade(frame.negatives(), a | b, sign).map_err(FieldRefusal::Mismatched)?;
        Ok(TurningPlane { key, units: (a, b), bivector })
    }

    /// The plane's key, as the coordinate form writes it (`e_+1+2`).
    pub fn key(&self) -> &Key {
        &self.key
    }

    /// The plane's bivector `B`, a value of the field's `Cl(p, q)`.
    pub fn bivector(&self) -> &Clifford {
        &self.bivector
    }

    /// HOW THIS PLANE MEETS `other`, read from the units the two name; the
    /// two values' signatures read first, refused `Mismatched` where they
    /// disagree.
    pub fn meeting(&self, other: &TurningPlane) -> Result<Meeting, FieldRefusal> {
        self.bivector.product(&other.bivector).map_err(FieldRefusal::Mismatched)?;
        let (a, b) = (self.units, other.units);
        Ok(match (a == b, a == (b.1, b.0), (a.0 | a.1) & (b.0 | b.1)) {
            (true, _, _) => Meeting::Along,
            (_, true, _) => Meeting::Against,
            (_, _, 0) => Meeting::Disjoint,
            _ => Meeting::Sharing,
        })
    }
}

impl PartialEq for TurningPlane {
    fn eq(&self, other: &TurningPlane) -> bool {
        self.key == other.key && self.units == other.units
    }
}

impl Eq for TurningPlane {}

/// THE DEGREES OF ONE WHOLE TURN, exact.
fn whole_turn() -> Q {
    Q::from_integer(BigInt::from(DEGREES_PER_TURN))
}

/// A TURNING (78.171 §2, a point's phase and an elliptic motion): an angle
/// `θ` exact in degrees on its named plane, held with its winding
/// `K = ⌊|θ|/360⌋` and never read mod 360, and its motion, the element
/// `ζ_ℓ^t` of `ℚ(ζ_ℓ)` for `θ = 360°·t/ℓ` in lowest terms. The plane's even
/// subalgebra `{a + bB}` is the field the motion stands in, `B` its
/// imaginary unit: the motion is `cos θ + B·sin θ`, which carries a vector
/// `v` of the plane to `v·(cos θ + B·sin θ)`, turned by `θ` from the
/// plane's first unit toward its second. No cosine or sine is read: the
/// element is `ζ_ℓ^t` itself.
///
/// The angle and the motion are held both: `5 × 72°` is `360°` with `K = 1`
/// while its motion is the identity. A turning has one constructor,
/// [`Turning::of`], which derives the motion from the angle, so no turning
/// holds a motion its angle does not name (`turning_motion_is_derived_from_its_angle`):
///
/// ```compile_fail,E0451
/// use mycelium::transducer::field::carrier::{Cyclotomic, Turning};
/// let _ = Turning { plane: todo!(), degrees: todo!(), motion: Cyclotomic::one() };
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Turning {
    plane: TurningPlane,
    degrees: Q,
    motion: Cyclotomic,
}

impl Turning {
    /// THE TURNING BY `degrees` ON `plane`, its motion `ζ_ℓ^t` read from
    /// `degrees / 360 = t/ℓ` in lowest terms. Refused `NotExact` where the
    /// conductor passes the machine word.
    pub fn of(plane: TurningPlane, degrees: Q) -> Result<Turning, FieldRefusal> {
        let share = &degrees / whole_turn();
        let unheld = || not_exact(Residue::PastWord { degrees: degrees.clone() });
        let conductor = share.denom().to_usize().ok_or_else(unheld)?;
        let power = share.numer().mod_floor(share.denom()).to_usize().ok_or_else(unheld)?;
        let motion = Cyclotomic::root(conductor, power).ok_or_else(unheld)?;
        Ok(Turning { plane, degrees, motion })
    }

    /// The plane the turning stands on.
    pub fn plane(&self) -> &TurningPlane {
        &self.plane
    }

    /// The angle in degrees, exact and signed, never reduced mod 360.
    pub fn degrees(&self) -> &Q {
        &self.degrees
    }

    /// The motion, `ζ_ℓ^t` in `ℚ(ζ_ℓ)`.
    pub fn motion(&self) -> &Cyclotomic {
        &self.motion
    }

    /// THE WINDING `K = ⌊|θ| / 360⌋` (LAW-COORDINATE v3 §1), in ℤ.
    pub fn winding(&self) -> BigInt {
        (self.degrees.abs() / whole_turn()).floor().to_integer()
    }

    /// THE MOTION'S ORDER: `ℓ / gcd(t, ℓ)`, read by exact products, `1` for
    /// a motion that is the identity (`0°`, `360°`, …).
    pub fn order(&self) -> Option<usize> {
        self.motion.order()
    }

    /// THE TURNING COMPOSED WITH `other` ON ONE PLANE: the angles summed,
    /// `other`'s negated where its plane is this one reversed, and the
    /// motion derived again from the sum, so the winding is kept. Refused
    /// `NotExact` naming both planes where they are not one plane: two
    /// planes compose only as a motion (`holonomy::Motion`).
    pub fn then(&self, other: &Turning) -> Result<Turning, FieldRefusal> {
        let arriving = match self.plane.meeting(&other.plane)? {
            Meeting::Along => other.degrees.clone(),
            Meeting::Against => -other.degrees.clone(),
            Meeting::Disjoint => return Err(not_exact(Residue::Apart { held: self.plane.key.clone(), arriving: other.plane.key.clone() })),
            Meeting::Sharing => return Err(not_exact(Residue::Uncommuted { held: self.plane.key.clone(), arriving: other.plane.key.clone() })),
        };
        Turning::of(self.plane.clone(), &self.degrees + arriving)
    }
}

/// The refusal `NotExact` with its residue.
fn not_exact(residue: Residue) -> FieldRefusal {
    FieldRefusal::NotExact { residue }
}
