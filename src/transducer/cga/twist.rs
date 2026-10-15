// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-CLIENT §4.4(1), §4.12 — THE TWIST OF THE PULL: the rotation about an
//! attractor that carries where an orbit was predicted to stand onto where it
//! is perceived. An orbit is made unique by its shear — gravity and velocity
//! pull it off a perfect circle — and the twist is the rotation of that pull:
//! never a residue to refuse, the orbit precessing.
//!
//! Built of two reflections, exact over ℚ:
//! - `m₁ = P − S` (each at unit weight, `−X·e_∞ = 1`): the hyperplane that
//!   bisects the two, reflecting one onto the other. It holds the
//!   attractor's centre `C` exactly where `C` stands equally far from both —
//!   which is what a rotation of the pull is; where it does not, the body was
//!   carried off the attractor and no twist exists.
//! - `m₂`: the plane through `C` and `S` in the plane `P`, `S` and `C` span,
//!   holding `S` and `C` still.
//!
//! `τ = m₂ m₁` is even: a rotation about `C`, carrying `P` onto `S`. It is
//! verified, never assumed — `τ P τ̃ ∝ S` and `τ C τ̃ ∝ C` — and refused where
//! either fails, where `P` and `S` stand at different sizes, or where `P`,
//! `S` and `C` are collinear (the plane of the rotation is then not named by
//! the bodies, and no plane is chosen for them).

use super::embedding::{base_at, base_vector, e_inf, point_at};
use super::multivector::Mv;
use num_rational::BigRational as Q;
use num_traits::Zero;

fn dot(a: &[Q], b: &[Q]) -> Q {
    a.iter().zip(b).fold(Q::zero(), |acc, (x, y)| acc + x * y)
}

/// `x` at unit weight (`−x·e_∞ = 1`); `None` at infinity.
fn unit(x: &Mv) -> Option<Mv> {
    let w = -x.geo(&e_inf()).scalar_part();
    (!w.is_zero()).then(|| x.scale(&(Q::from_integer(1.into()) / w)))
}

/// THE TWIST ABOUT `centre` CARRYING `from` ONTO `to` — two points or two
/// rounds of one size of the base of `n` axes. `None` where they coincide
/// (no twist), where they stand at different sizes or at different
/// distances from the centre (carried off the attractor), or where the three
/// are collinear.
pub fn twist_about(centre: &[Q], from: &Mv, to: &Mv, n: usize) -> Option<Mv> {
    let (p, s) = (unit(from)?, unit(to)?);
    if p.sub(&s).is_zero() || !p.norm2().eq(&s.norm2()) {
        return None;
    }
    let c = point_at(centre);
    let m1 = p.sub(&s);
    // The bisector holds the centre: the two stand equally far from it.
    if !m1.geo(&c).scalar_part().is_zero() {
        return None;
    }
    let (pe, se) = (base_at(&p, n)?, base_at(&s, n)?);
    let u: Vec<Q> = pe.iter().zip(centre).map(|(x, c)| x - c).collect();
    let v: Vec<Q> = se.iter().zip(centre).map(|(x, c)| x - c).collect();
    let vv = dot(&v, &v);
    if vv.is_zero() {
        return None;
    }
    let k = dot(&u, &v) / &vv;
    let normal: Vec<Q> = u.iter().zip(&v).map(|(a, b)| a - &k * b).collect();
    if normal.iter().all(Zero::is_zero) {
        return None;
    }
    let m2 = base_vector(&normal).add(&e_inf().scale(&dot(&normal, centre)));
    let tau = m2.geo(&m1);
    let carries = tau.sandwich(&p).projective_eq(&s);
    let keeps = tau.sandwich(&c).projective_eq(&c);
    (carries && keeps).then_some(tau)
}
