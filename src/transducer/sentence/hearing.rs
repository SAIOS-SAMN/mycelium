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

/// What hearing reads, counted by name: the rings heard (a ring whose
/// closure is unread has no size and is not heard), those that return, the
/// outer and leftover rings, the seats whole and tilted, each tilted seat's
/// echoes by their kind, and the perceptions of two or more rings in balance
/// or not.
pub const HEARING: [&str; 13] = [
    "rings heard",
    "rings returning",
    "outer rings",
    "seated whole",
    "seated with a tilt",
    "leftover rings",
    "echo a turn",
    "echo a gain",
    "echo a spiral",
    "echo a delay",
    "echo unread",
    "perceptions balanced",
    "perceptions unbalanced",
];

static HEARD: [std::sync::atomic::AtomicU64; 13] = [const { std::sync::atomic::AtomicU64::new(0) }; 13];

fn note(name: &str) {
    if let Some(i) = HEARING.iter().position(|n| *n == name) {
        let _ = HEARD[i].fetch_update(std::sync::atomic::Ordering::Relaxed, std::sync::atomic::Ordering::Relaxed, |n| n.checked_add(1));
    }
}

/// The [`HEARING`] row since the process began.
pub fn hearing_tally() -> [u64; 13] {
    std::array::from_fn(|i| HEARD[i].load(std::sync::atomic::Ordering::Relaxed))
}

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

    /// Whether a ring of this size sounds inside a ring of size `outer`.
    fn inside(self, outer: Size) -> bool {
        match (self, outer) {
            (Size::Returns(a), Size::Returns(b)) => a < b && b % a == 0,
            (Size::Returns(_), Size::Carrier) => true,
            (Size::Carrier, _) => false,
        }
    }

    /// Whether this size is above `other` at all, dividing it or not.
    fn above(self, other: Size) -> bool {
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

/// H1, H2 AT THE VOICE: one perception's rings heard and counted on the
/// [`HEARING`] row.
pub fn hear(rings: &[&Composition], budget: &mut crate::physics::Budget) -> Nesting {
    let n = nest(rings, budget);
    for (seat, r) in n.seats.iter().zip(rings) {
        if *seat == Seat::Unheard {
            continue;
        }
        note("rings heard");
        if r.closure.closes() {
            note("rings returning");
        }
        match seat {
            Seat::Outer => note("outer rings"),
            Seat::Whole => note("seated whole"),
            Seat::Leftover => note("leftover rings"),
            Seat::Tilted(echoes) => {
                note("seated with a tilt");
                for e in echoes {
                    note(match e {
                        Echo::Turn => "echo a turn",
                        Echo::Gain => "echo a gain",
                        Echo::Spiral => "echo a spiral",
                        Echo::Delay => "echo a delay",
                        Echo::Whole | Echo::Unread => "echo unread",
                    });
                }
            }
            Seat::Unheard => {}
        }
    }
    if n.seats.iter().filter(|s| **s != Seat::Unheard).count() > 1 {
        note(if n.balanced { "perceptions balanced" } else { "perceptions unbalanced" });
    }
    n
}
