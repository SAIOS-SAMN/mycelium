// 78.110 E1 — THE ENCOUNTER'S FORCES: an orbit met by a neighbour in contact,
// its drag and its lift read exactly from the orbit's own modes.
//
//! The orbit's motion `O` splits into commuting modes (MR1), and its terms
//! are those modes by kind — a kind the orbit lacks is the identity:
//!
//! ```text
//!     m  magnetism                the elliptic modes' rays, multiplied
//!     r  repulsion of antipoles   the hyperbolic modes' rays (a boost's two poles)
//!     p  propagation of force     the null modes' rays (the luminal cone)
//!     b  centrifugal bounce       the elliptic return: the orders' common period
//!
//!     v = m·r                     velocity
//!     g = (m·r)·(p·b)⁻¹           gravity, the orbit's own mode
//!     F = B_n                     the neighbour's generator (its Cayley bivector)
//!     D = B_∥                     drag: F's part commuting with the orbit's mode (78.46)
//!     L = v·F·D⁻¹·g⁻¹             lift, the resistance acting last
//! ```
//!
//! WHERE AN INVERSE DOES NOT EXIST — the orbit's mode no blade with a non-zero
//! square to split by, drag zero or null, gravity no versor — LIFT IS DRAG
//! (as ruled: a lift with no inverse is missing physics; its inverse is
//! drag). A force no split reads acts whole as drag.
//! Nothing is approximated; every product and inverse is exact over ℚ.

use super::multivector::Mv;
use crate::network::nerve::coord::BivectorKind;
use num_rational::BigRational;
use num_traits::{One, Zero};

type Q = BigRational;

/// ONE ENCOUNTER, READ: the drag, the lift, and whether the lift fell to
/// drag for want of an inverse.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Encounter {
    pub drag: Mv,
    pub lift: Mv,
    pub fell_to_drag: bool,
}

impl Encounter {
    /// The lift tilts: it carries a bivector part (a plane neither the orbit
    /// nor the drag turns in moves). A lift that is a scalar changes rate only.
    pub fn tilts(&self) -> bool {
        !self.lift.grade(2).is_zero()
    }
}

/// WHY AN ENCOUNTER WAS NOT READ: the orbit's modes did not split (no versor,
/// or the stroke spent reading them), or the neighbour carries no generator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unencountered {
    OrbitUnread,
    NeighbourUnread,
}

/// `x⁻¹ = x̃ / (x x̃)`, where `x x̃` is a non-zero scalar; `None` otherwise —
/// the inverse the ruling names as missing.
fn inverse(x: &Mv) -> Option<Mv> {
    let n = x.geo(&x.reverse());
    let s = n.scalar_part();
    (!s.is_zero() && n.sub(&Mv::scalar(s.clone())).is_zero()).then(|| x.reverse().scale(&(Q::one() / s)))
}

fn lcm(a: u64, b: u64) -> u64 {
    let (mut x, mut y) = (a, b);
    while y != 0 {
        (x, y) = (y, x % y);
    }
    a / x * b
}

/// THE ENCOUNTER of `orbit` with `neighbour` (78.110): the neighbour's
/// generator split against the orbit's own mode into drag, and the lift
/// `v·F·D⁻¹·g⁻¹` — or the drag itself where no inverse exists.
pub fn encounter(orbit: &Mv, neighbour: &Mv, budget: &mut crate::physics::Budget) -> Result<Encounter, Unencountered> {
    let modes = match super::motion_modes(orbit, budget) {
        Some(Ok(modes)) => modes,
        _ => return Err(Unencountered::OrbitUnread),
    };
    let force = crate::transducer::cayley::bivector_of(neighbour).ok_or(Unencountered::NeighbourUnread)?;
    let one = Mv::scalar(Q::one());
    let product_of = |kind: BivectorKind| modes.iter().filter(|m| m.kind == kind).fold(one.clone(), |acc, m| acc.geo(&m.ray));
    let (m, r, p) = (product_of(BivectorKind::Elliptic), product_of(BivectorKind::Hyperbolic), product_of(BivectorKind::Parabolic));
    // The bounce: the elliptic return, the common period of the turns that
    // return; none returning, the identity.
    let period = modes.iter().filter(|m| m.kind == BivectorKind::Elliptic).filter_map(|m| m.order).fold(1u64, |l, o| lcm(l, u64::from(o)));
    let b = Mv::scalar(Q::from_integer(period.into()));
    let v = m.geo(&r);
    // THE DRAG: F's part commuting with the orbit's own mode (78.46). An
    // orbit whose mode is no blade with a non-zero square has nothing to
    // split by: the force acts whole as drag, and lift is drag.
    let mode = crate::transducer::cayley::bivector_of(orbit);
    let Some(split) = mode.as_ref().and_then(|b_o| super::lift_drag(b_o, &force)) else {
        return Ok(Encounter { drag: force.clone(), lift: force, fell_to_drag: true });
    };
    let drag = split.drag;
    // GRAVITY, `(m·r)·(p·b)⁻¹`, and the lift `v·F·D⁻¹·g⁻¹`, each inverse
    // exact or absent; absent, lift is drag.
    let lift = inverse(&p.geo(&b))
        .map(|pb| v.geo(&pb))
        .and_then(|g| Some((inverse(&drag)?, inverse(&g)?)))
        .map(|(d_inv, g_inv)| v.geo(&force).geo(&d_inv).geo(&g_inv));
    match lift {
        Some(lift) => Ok(Encounter { drag, lift, fell_to_drag: false }),
        None => Ok(Encounter { lift: drag.clone(), drag, fell_to_drag: true }),
    }
}
