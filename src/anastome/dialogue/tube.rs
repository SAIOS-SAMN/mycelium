// 78.134 — A WORMHOLE AS A TUBE (the coordinator's rulings, the operator's
// anti-flattening rule), moved from `handoff.rs` and, 78.140, given a
// GEOMETRY per end: a curved face (`ℍ³` of radius `R`) or a flat TORUS face
// (axis-aligned periods), each end's frame a motion of its own space and its
// horizon read in its own space.
//
//! One throat, one MASS `m` — the invariant both faces share. A curved face's
//! horizon is the level set `{p : p·v = −c}` about its place, `c = cosh(2m/R)`;
//! a torus face's is the sphere of RADIUS `2m` about its place, read with its
//! RATIO `2m / L_min` to the torus's shortest period, which must stay below
//! `½` — a sphere fits its fundamental domain, or it meets its own image.

use super::universe::Geometry;
use crate::transducer::cga::Mv;
use num_rational::BigRational;

fn u64_at(b: &[u8], p: &mut usize) -> Option<u64> {
    let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    Some(v)
}

fn put_q(out: &mut Vec<u8>, (n, d): (u64, u64)) {
    out.extend_from_slice(&n.to_le_bytes());
    out.extend_from_slice(&d.to_le_bytes());
}

fn take_q(b: &[u8], p: &mut usize) -> Option<(u64, u64)> {
    Some((u64_at(b, p)?, u64_at(b, p)?))
}

/// 78.134 — A TUBE: each mouth stands at its frame's place in its domain's
/// space, its throat sized by its MASS (units are mass); each end's horizon
/// level is DERIVED in that end's space (78.140: its GEOMETRY), never stored
/// as the truth. A tube may SUPERSEDE an untethered mouth, citing it; the old
/// record stays.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tube {
    pub mass: (u64, u64),
    pub from_space: Geometry,
    pub to_space: Geometry,
    pub supersedes: Option<[u8; 32]>,
    /// 78.139 — a CORE TUBE (kind 16): a galactic core's throat to the core
    /// of the universe it bounced into, kept by BOTH reserves.
    pub core: bool,
}

impl Tube {
    /// Both ends curved: the writing 78.134 sealed (`[mass][R from][R to]
    /// [supersedes]`), so no curved tube's id or stored bytes move.
    pub fn curved(&self) -> bool {
        matches!((&self.from_space, &self.to_space), (Geometry::Curved(_), Geometry::Curved(_)))
    }

    /// The tube's one writing: a curved tube's as 78.134 wrote it; 78.140 —
    /// a tube with a torus end `[mass][from geometry][to geometry]
    /// [supersedes]`, under its own handoff kind and tag.
    pub fn bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        put_q(&mut out, self.mass);
        match (&self.from_space, &self.to_space) {
            (Geometry::Curved(a), Geometry::Curved(b)) => {
                put_q(&mut out, *a);
                put_q(&mut out, *b);
            }
            (a, b) => {
                a.put(&mut out);
                b.put(&mut out);
            }
        }
        match &self.supersedes {
            None => out.push(0),
            Some(id) => {
                out.push(1);
                out.extend_from_slice(id);
            }
        }
        out
    }

    /// A tube read back: `spaced` for the writing with a torus end.
    pub fn take(b: &[u8], p: &mut usize, spaced: bool) -> Option<Tube> {
        let mass = take_q(b, p)?;
        let (from_space, to_space) = match spaced {
            false => (Geometry::Curved(take_q(b, p)?), Geometry::Curved(take_q(b, p)?)),
            true => (Geometry::take(b, p)?, Geometry::take(b, p)?),
        };
        let flag = *b.get(*p)?;
        *p += 1;
        let supersedes = match flag {
            0 => None,
            1 => {
                let id: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
                *p += 32;
                Some(id)
            }
            _ => return None,
        };
        let t = Tube { mass, from_space, to_space, supersedes, core: false };
        // A spaced writing names a torus end, or it is a curved tube's.
        (spaced != t.curved()).then_some(t)
    }

    /// Mass a positive exact rational and both spaces well formed.
    pub fn holds(&self) -> bool {
        super::universe::positive(self.mass) && self.from_space.holds() && self.to_space.holds()
    }
}

/// 78.134 — WHETHER A FRAME IS A MOTION OF CURVED SPACE: an element of
/// `Spin⁺(3,1)`, the isometries of `ℍ³` inside `Cl(4,1)` — even, a versor,
/// commuting with `e₊` (the axis that makes the sheet the sheet), and
/// orthochronous (it carries the pole onto the UPPER sheet).
pub fn is_spin_plus_31(f: &Mv) -> bool {
    use crate::transducer::cga::{is_versor, EM, EP};
    use num_traits::Signed;
    if f.is_zero() || !is_versor(f) {
        return false;
    }
    let ep = Mv::blade(EP, BigRational::from_integer(1.into()));
    let fixes_axis = f.geo(&ep).sub(&ep.geo(f)).is_zero();
    let upper = crate::transducer::cga::frame_base_point(f).is_some_and(|v| v.coeff(EM).is_positive());
    is_even(f) && fixes_axis && upper
}

fn is_even(f: &Mv) -> bool {
    [1u32, 3, 5].iter().all(|g| f.grade(*g).is_zero())
}

/// 78.140 — WHETHER A FRAME IS A MOTION OF FLAT SPACE: a rotation and a
/// translation — even, a versor, carrying the point at infinity to itself at
/// the frame's own weight, `F e∞ F̃ = (F F̃) e∞`. A dilation scales `e∞` and a
/// transversion moves it: neither is a motion of a torus. The scale is the
/// content here, so the image is compared EXACTLY — `Mv`'s `==` is
/// projective, and would let a dilation through.
pub fn is_euclidean_motion(f: &Mv) -> bool {
    use num_traits::Zero;
    let Some(w) = crate::transducer::cga::weight_of(f) else { return false };
    let inf = crate::transducer::cga::e_inf();
    !w.is_zero() && is_even(f) && f.sandwich(&inf).sub(&inf.scale(&w)).is_zero()
}

/// `x` brought into `[0, l)` by whole periods, exact.
fn wrapped(x: &BigRational, l: &BigRational) -> BigRational {
    x - (x / l).floor() * l
}

/// 78.140 — A MOUTH'S PLACE IN ITS FACE'S SPACE, its frame a motion of that
/// space: a curved face's place on the sheet (`v² = −1`); a torus face's the
/// base point `F e_o F̃` on the torus's axes, read modulo its periods — a
/// point of the fundamental domain. `None` where the frame is no motion of
/// the space, or stands off the torus's axes.
pub fn place_in(space: &Geometry, frame: &Mv) -> Option<Mv> {
    match space {
        Geometry::Curved(_) => is_spin_plus_31(frame).then(|| crate::transducer::cga::frame_base_point(frame))?,
        Geometry::Torus { periods, .. } => {
            if !is_euclidean_motion(frame) {
                return None;
            }
            let at = crate::transducer::cga::base_at(&frame.sandwich(&crate::transducer::cga::e_o()), 3)?;
            let (on, off) = at.split_at(periods.len());
            if off.iter().any(|x| *x != BigRational::from_integer(0.into())) {
                return None;
            }
            let reduced = on
                .iter()
                .zip(periods)
                .map(|(x, (n, d))| (*d > 0).then(|| wrapped(x, &BigRational::new((*n).into(), (*d).into()))))
                .collect::<Option<Vec<_>>>()?;
            Some(crate::transducer::cga::base_vector(&reduced))
        }
    }
}

/// 78.140 — A THROAT'S LEVEL IN ONE FACE (the coordinator's ruling (i)):
/// CURVED, the horosphere level `c = cosh(2m/R)` (the upper bound of its
/// certified enclosure); FLAT, the sphere's RADIUS `2m` in the face's units
/// and its RATIO `2m / L_min`, which a fitting throat keeps below `½`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Level {
    Curved(BigRational),
    Flat { radius: BigRational, ratio: BigRational },
}

impl Level {
    /// Whether the throat fits its face: always in a curved face; in a torus
    /// face, its sphere clear of its own image (`ratio < ½`).
    pub fn fits(&self) -> bool {
        match self {
            Level::Curved(_) => true,
            Level::Flat { ratio, .. } => ratio < &BigRational::new(1.into(), 2.into()),
        }
    }

    fn put(&self, out: &mut Vec<u8>) {
        match self {
            Level::Curved(c) => {
                out.push(0);
                Mv::scalar(c.clone()).to_bytes(out);
            }
            Level::Flat { radius, ratio } => {
                out.push(1);
                Mv::scalar(radius.clone()).to_bytes(out);
                Mv::scalar(ratio.clone()).to_bytes(out);
            }
        }
    }

    fn take(b: &[u8], p: &mut usize) -> Option<Level> {
        let tag = *b.get(*p)?;
        *p += 1;
        match tag {
            0 => Some(Level::Curved(Mv::from_bytes(b, p)?.scalar_part())),
            1 => Some(Level::Flat { radius: Mv::from_bytes(b, p)?.scalar_part(), ratio: Mv::from_bytes(b, p)?.scalar_part() }),
            _ => None,
        }
    }
}

/// 78.134 — A TUBE AS READ, derived and never stored: each mouth's place in
/// its face's space, each end's horizon LEVEL from the throat's mass in that
/// end's space (78.140), and the mass. A traversal entering at `from_level`
/// emerges at `to_level`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TubeReading {
    pub from_place: Mv,
    pub to_place: Mv,
    pub from_level: Level,
    pub to_level: Level,
    pub mass: (u64, u64),
}

/// `[0]` no tube; `[1][from place][to place][from level][to level][mass n d]`,
/// each level `[0][c]` or `[1][radius][ratio]`, each rational a scalar
/// multivector — the canonical rational.
pub fn encode_tube_reading(r: Option<&TubeReading>) -> Vec<u8> {
    let Some(r) = r else { return vec![0] };
    let mut out = vec![1];
    r.from_place.to_bytes(&mut out);
    r.to_place.to_bytes(&mut out);
    r.from_level.put(&mut out);
    r.to_level.put(&mut out);
    put_q(&mut out, r.mass);
    out
}

pub fn decode_tube_reading(b: &[u8]) -> Option<Option<TubeReading>> {
    let (flag, rest) = b.split_first()?;
    match flag {
        0 => return rest.is_empty().then_some(None),
        1 => {}
        _ => return None,
    }
    let mut p = 0;
    let from_place = Mv::from_bytes(rest, &mut p)?;
    let to_place = Mv::from_bytes(rest, &mut p)?;
    let from_level = Level::take(rest, &mut p)?;
    let to_level = Level::take(rest, &mut p)?;
    let mass = take_q(rest, &mut p)?;
    (p == rest.len()).then_some(Some(TubeReading { from_place, to_place, from_level, to_level, mass }))
}
