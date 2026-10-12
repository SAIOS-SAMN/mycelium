// 78.112 H1, H2 — HEARING: a perception's rings read against each other
// harmonically, never traversed — each ring's echo through the rings larger
// than it, and the rings nested, the smallest inside the next larger, the
// largest on the outside, into harmonic balance.
//
//! A ring's SIZE is its return: its period `ℓ` where it closes; a ring that
//! never returns is larger than every ring that does — the carrier every
//! returning ring may sound inside. `s` is smaller than `L` where it returns
//! a whole number of times on one return of `L` (`ℓ_s < ℓ_L`, `ℓ_s | ℓ_L`),
//! or where `s` returns and `L` does not.
//!
//! THE ECHO of `s` backing through `L` is `E = Ω_L·Ω_s·Ω_L⁻¹·Ω_s⁻¹`: `s`
//! carried round `L`, against itself. The identity, `s` passes through
//! whole; else its modes say what `L` did to it — a turn, a gain, a spiral
//! (both at once), a delay.
//!
//! Each ring seats in the NEXT LARGER layer — the least size above it that
//! it divides — read against every ring of that layer, none chosen: whole
//! where some echo is the identity, with a tilt where none is. A ring with
//! no larger ring is on the outside; one that has larger rings but divides
//! none is a leftover. The rings are in harmonic BALANCE where every seated
//! ring sits whole and none is left over: every period then divides the
//! outer one, so the outer return is the joint return.

use super::compose::{Closure, Composition};
use crate::network::nerve::coord::BivectorKind;
use crate::transducer::cga::{motion_modes, Mv};
use num_traits::{One, Zero};

type Q = num_rational::BigRational;

/// A RING'S SIZE: its return, or none — the carrier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Size {
    Returns(u64),
    Carrier,
}

impl Size {
    fn of(c: Closure) -> Option<Size> {
        match c {
            Closure::Closes(l) => Some(Size::Returns(l)),
            Closure::Rosette | Closure::Unbound => Some(Size::Carrier),
            Closure::Unread => None,
        }
    }

    /// A HELD WORD'S SIZE, read off its coordinates: where every hole is a
    /// turn of finite order, it returns at the lcm of their orders; where any
    /// hole does not return (a boost, a slide, a rosette), it is a carrier.
    /// `None` for a word with no hole — it moves nothing and is no ring.
    pub fn of_coordinates(c: &super::coordinates::Coordinates) -> Option<Size> {
        if c.holes.is_empty() {
            return None;
        }
        let mut period = 1u64;
        for h in &c.holes {
            match h.winding {
                super::coordinates::Winding::Turn { ell, .. } => period = lcm(period, u64::from(ell)),
                _ => return Some(Size::Carrier),
            }
        }
        Some(Size::Returns(period))
    }

    /// THE SIZE'S COORDINATE, written so the byte order is the nesting order:
    /// the returning sizes by their period, ascending, then the carrier.
    pub fn key(self) -> Vec<u8> {
        match self {
            Size::Returns(l) => [&[0u8][..], &l.to_be_bytes()].concat(),
            Size::Carrier => vec![1],
        }
    }

    /// The size a [`Size::key`] writes.
    pub fn from_key(k: &[u8]) -> Option<Size> {
        match k {
            [1] => Some(Size::Carrier),
            [0, rest @ ..] => Some(Size::Returns(u64::from_be_bytes(rest.try_into().ok()?))),
            _ => None,
        }
    }

    /// Whether a ring of this size sounds inside a ring of size `outer`.
    pub(crate) fn inside(self, outer: Size) -> bool {
        match (self, outer) {
            (Size::Returns(a), Size::Returns(b)) => a < b && b % a == 0,
            (Size::Returns(_), Size::Carrier) => true,
            (Size::Carrier, _) => false,
        }
    }

    /// Whether this size is above `other` at all, dividing it or not.
    pub(crate) fn above(self, other: Size) -> bool {
        match (self, other) {
            (Size::Returns(a), Size::Returns(b)) => a > b,
            (Size::Carrier, Size::Returns(_)) => true,
            (_, Size::Carrier) => false,
        }
    }
}

/// WHAT A RING'S ECHO THROUGH A LARGER ONE IS.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Echo {
    /// `E = 1`: it passes through whole.
    Whole,
    /// Elliptic modes alone: a pitch shifted by an exact turn.
    Turn,
    /// Hyperbolic modes alone: a gain, or an attenuation.
    Gain,
    /// A turn and a gain at once — the loxodrome: the echo turning as it
    /// grows or fades.
    Spiral,
    /// A null mode and neither of the others: the slide, a delay.
    Delay,
    /// No inverse to carry it by, or its modes did not read.
    Unread,
}

/// WHERE ONE RING SITS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Seat {
    /// No ring is larger: it is on the outside.
    Outer,
    /// Seated in the next larger layer, some echo through it the identity.
    Whole,
    /// Seated in the next larger layer, no echo through it the identity: each
    /// echo by its kind.
    Tilted(Vec<Echo>),
    /// Larger rings stand, and it divides none of them.
    Leftover,
    /// Its closure is unread: no size, not heard.
    Unheard,
}

/// One perception's rings heard: each ring's seat, in the rings' order, and
/// whether they stand in harmonic balance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nesting {
    pub seats: Vec<Seat>,
    pub balanced: bool,
}

/// `x⁻¹ = x̃ / (x x̃)` where `x x̃` is a non-zero scalar; `None` otherwise.
fn inverse(x: &Mv) -> Option<Mv> {
    let n = x.geo(&x.reverse());
    let s = n.scalar_part();
    (!s.is_zero() && n.sub(&Mv::scalar(s.clone())).is_zero()).then(|| x.reverse().scale(&(Q::one() / s)))
}

/// H1 — THE ECHO of `s` backing through `outer`.
pub fn echo(s: &Mv, outer: &Mv, budget: &mut crate::physics::Budget) -> Echo {
    let (Some(s_inv), Some(outer_inv)) = (inverse(s), inverse(outer)) else { return Echo::Unread };
    let e = outer.geo(s).geo(&outer_inv).geo(&s_inv);
    if e.projective_eq(&Mv::scalar(Q::one())) {
        return Echo::Whole;
    }
    let Some(Ok(modes)) = motion_modes(&e, budget) else { return Echo::Unread };
    let has = |k: BivectorKind| modes.iter().any(|m| m.kind == k);
    match (has(BivectorKind::Elliptic), has(BivectorKind::Hyperbolic), has(BivectorKind::Parabolic)) {
        (true, true, _) => Echo::Spiral,
        (false, true, _) => Echo::Gain,
        (true, false, _) => Echo::Turn,
        (false, false, true) => Echo::Delay,
        (false, false, false) => Echo::Unread,
    }
}

/// H2 — ONE PERCEPTION'S RINGS NESTED: each seated in the next larger layer
/// by its echoes, the outer and leftover rings named, the balance read.
pub fn nest(rings: &[&Composition], budget: &mut crate::physics::Budget) -> Nesting {
    let sizes: Vec<Option<Size>> = rings.iter().map(|r| Size::of(r.closure)).collect();
    let mut seats = Vec::with_capacity(rings.len());
    for (s, size) in rings.iter().zip(&sizes) {
        let Some(size) = *size else {
            seats.push(Seat::Unheard);
            continue;
        };
        // THE NEXT LARGER LAYER: the least size it sounds inside.
        let next = sizes.iter().flatten().filter(|o| size.inside(**o)).fold(None, |least: Option<Size>, o| match least {
            Some(l) if !l.above(*o) => Some(l),
            _ => Some(*o),
        });
        let Some(layer) = next else {
            let larger = sizes.iter().flatten().any(|o| o.above(size));
            seats.push(if larger { Seat::Leftover } else { Seat::Outer });
            continue;
        };
        let mut echoes = Vec::new();
        let mut whole = false;
        for (outer, _) in rings.iter().zip(&sizes).filter(|(_, o)| **o == Some(layer)) {
            match echo(&s.omega, &outer.omega, budget) {
                Echo::Whole => {
                    whole = true;
                    break;
                }
                other => echoes.push(other),
            }
        }
        seats.push(if whole { Seat::Whole } else { Seat::Tilted(echoes) });
    }
    let balanced = seats.iter().all(|s| matches!(s, Seat::Outer | Seat::Whole | Seat::Unheard));
    Nesting { seats, balanced }
}

/// H1 ON THE STROKE THE ECHO'S OWN MODES NEED (78.111 C1b): the echo of `s`
/// through `outer`, read where it is heard and sized by nothing else.
pub fn echo_sized(s: &Mv, outer: &Mv) -> Echo {
    let (Some(s_inv), Some(outer_inv)) = (inverse(s), inverse(outer)) else { return Echo::Unread };
    let e = outer.geo(s).geo(&outer_inv).geo(&s_inv);
    if e.projective_eq(&Mv::scalar(Q::one())) {
        return Echo::Whole;
    }
    let Some(need) = crate::transducer::cga::modes_need(&e) else { return Echo::Unread };
    echo(s, outer, &mut crate::physics::Physics::default().stroke_n(need))
}

/// 78.112 H5 — A WORD HEARD ACROSS MINDS: where it seats in the hearer's
/// rings, the word as the hearer's language carries it there, and the rings
/// it was carried through.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Echoed {
    pub seat: Seat,
    /// The word carried round the layer it seated in — `Ω_L·Ω_s·Ω_L⁻¹` over
    /// each ring of the layer in its own coordinate order — so the loop said
    /// → heard is the echo itself: the identity where it seats whole. The
    /// word itself where it is the outer ring; `None` where it is leftover
    /// or unheard — not understood.
    pub carried: Option<Mv>,
    /// The rings of the layer it seated in, by their coordinates.
    pub through: Vec<Vec<u8>>,
}

/// H5 — HEAR A WORD IN ANOTHER MIND'S RINGS: `rings` the hearer's held words
/// by their size, in the order sizes nest (ascending periods, then the
/// carrier), each with its coordinate and motion. The word of `size` seats in
/// the NEXT LARGER layer — the least size above it it divides — read against
/// every ring there: WHOLE where some echo is the identity (understood
/// whole), TILTED where none is (understood shifted: the word carried round
/// each ring of the layer, the echo the loop). With no larger ring it is on
/// the OUTSIDE — the carrier the hearer's rings sound inside, understood as
/// it is; with larger rings it divides none of, LEFTOVER — not understood.
pub fn hear_across<'a>(word: &Mv, size: Size, rings: impl Iterator<Item = (Size, Vec<(Vec<u8>, &'a Mv)>)>) -> Echoed {
    let mut larger = false;
    for (layer, members) in rings {
        if !layer.above(size) {
            continue;
        }
        larger = true;
        if !size.inside(layer) {
            continue;
        }
        let through: Vec<Vec<u8>> = members.iter().map(|(k, _)| k.clone()).collect();
        if members.iter().any(|(_, ring)| echo_sized(word, ring) == Echo::Whole) {
            return Echoed { seat: Seat::Whole, carried: Some(word.clone()), through };
        }
        let mut echoes = Vec::with_capacity(members.len());
        let mut carried = Some(word.clone());
        for (_, ring) in &members {
            echoes.push(echo_sized(word, ring));
            carried = carried.and_then(|c| inverse(ring).map(|inv| ring.geo(&c).geo(&inv)));
        }
        return Echoed { seat: Seat::Tilted(echoes), carried, through };
    }
    match larger {
        true => Echoed { seat: Seat::Leftover, carried: None, through: Vec::new() },
        false => Echoed { seat: Seat::Outer, carried: Some(word.clone()), through: Vec::new() },
    }
}

/// 78.112 H6 — ONE HARMONIC RING MET BY ONE SEEN RAY: the circuit in orbit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InOrbit {
    /// The ring, by its place among the rings heard.
    pub ring: usize,
    /// The ray's part commuting with every elliptic plane of the ring — the
    /// light along the axis, the twist's ADVANCE.
    pub advance: Mv,
    /// The ray's part that is not — the light CROSSING the plane, the impact.
    pub crossing: Mv,
    /// The advance per elliptic return, `ℓ_e·D`: the PITCH.
    pub pitch: Mv,
    /// The elliptic return `ℓ_e`: the common period of the ring's turns.
    pub bounce: u64,
    /// The encounter's law (78.110): gravity the ring's mode, force the ray,
    /// lift `(v·F)/(g·D)` — drag where no inverse exists.
    pub encounter: Result<crate::transducer::cga::Encounter, crate::transducer::cga::Unencountered>,
}

fn lcm(a: u64, b: u64) -> u64 {
    let (mut x, mut y) = (a, b);
    while y != 0 {
        (x, y) = (y, x % y);
    }
    a / x * b
}

/// H6 — EVERY HARMONIC RING AGAINST THE SEEN RAYS IT WAS CARRIED ON. A ring
/// is harmonic where it holds an elliptic mode of finite order; a ray is a
/// space whose generator is null (`d∧e_∞`, 78.97: seen through the
/// luminal). Each ray is split against each of the ring's elliptic planes
/// in turn (78.46; the planes commute, so the order is no choice): what
/// commutes with them all is the advance, the rest the crossing. The
/// encounter reads the law beside it.
pub fn orbit(rings: &[&Composition], budget: &mut crate::physics::Budget) -> Vec<InOrbit> {
    let mut out = Vec::new();
    for (ring, r) in rings.iter().enumerate() {
        let planes: Vec<Mv> = r
            .modes
            .iter()
            .filter(|m| m.kind == BivectorKind::Elliptic && m.order.is_some())
            .map(|m| m.ray.grade(2))
            .collect();
        if planes.is_empty() {
            continue;
        }
        let bounce = r.modes.iter().filter(|m| m.kind == BivectorKind::Elliptic).filter_map(|m| m.order).fold(1u64, |l, o| lcm(l, u64::from(o)));
        for space in &r.spaces {
            // THE LIGHT IN A SPACE IS ITS NULL MODES (78.112 H6, read live
            // 2026-10-06): a seen edge's transport is the carriage `D·T`, the
            // dilation kept (78.97), whose whole generator is never null —
            // asked whole, no ray was met on any voice. Each null flux of the
            // space is a ray (`cech::flux`, the field's own reader, law 12).
            let Some(rays) = crate::transducer::cech::flux::null_fluxes(space, budget) else { continue };
            for ray in rays {
                let advance = planes.iter().try_fold(ray.clone(), |f, b| crate::transducer::cga::lift_drag(b, &f).map(|s| s.drag));
                let Some(advance) = advance else { continue };
                let crossing = ray.sub(&advance);
                let pitch = advance.scale(&Q::from_integer(bounce.into()));
                let encounter = crate::transducer::cga::encounter(&r.omega, space, budget);
                out.push(InOrbit { ring, advance, crossing, pitch, bounce, encounter });
            }
        }
    }
    out
}

/// One perception's rings heard: (H7b) the torsion the hearing wound — every
/// crossing composed as one strain, in the order the rings were heard;
/// `None` where no ray crossed a plane.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Heard {
    pub wound: Option<Mv>,
}

/// H7b — THE TORSION A HEARING WINDS: each ray's crossing (the impact) as the
/// versor its generator derives (`1 + C + ½ C∧C`, exact, 52.1), the later
/// after the earlier. A crossing that derives no versor winds nothing.
pub fn wind(met: &[InOrbit]) -> Option<Mv> {
    met.iter()
        .filter(|m| !m.crossing.is_zero())
        .filter_map(|m| crate::transducer::cayley::versor_of_bivector(&m.crossing))
        .fold(None, |held: Option<Mv>, v| match held {
            Some(h) => Some(v.geo(&h)),
            None => Some(v),
        })
}

/// H6, H7b AT THE VOICE: one perception's rings met by its seen rays, and
/// the torsion the hearing wound handed back for the perception's vertex.
pub fn hear(rings: &[&Composition], budget: &mut crate::physics::Budget) -> Heard {
    Heard { wound: wind(&orbit(rings, budget)) }
}
