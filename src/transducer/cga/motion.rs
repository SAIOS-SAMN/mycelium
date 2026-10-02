// 78.12(2), MR1 — THE MODES OF A MOTION, read off the motion's own action and
// never through a chart: the invariant planes of `R(x) = Ω x Ω⁻¹`, each
// with its rational `c_j`, its exact versor ray, its class and its closure;
// the rays verified to commute and to multiply back to `Ω`.
//
//! `R` is a rational orthogonal map for every rational versor, a linear
//! function of the vectors (`LinearFunction`), so `S = (R + R⁻¹)/2` is
//! self-adjoint and `A = (R − R⁻¹)/2` skew. `S`'s spectral split — its
//! characteristic polynomial from the traces of its powers, each value's
//! kernel, and each space's projector a polynomial in `S`, the same readers
//! that split a generator — gives the invariant spaces `V_j`, where `S = c_j`:
//! `cos θ_j` on a turn, `cosh θ_j` on a boost, `1` on the generalised kernel
//! a parabolic mode lives in. Each `c_j` is rational wherever the motion is:
//! `0` for the 90° turn, `−1` for the 180° turn, `3/5` for the `(3,4,5)`
//! turn — the points the inverse Cayley chart could not reach.
//!
//! THE MODE'S VERSOR RAY, exact. `K_j = ½ Σ_k e^k ∧ A(P_j e_k)` is the
//! bivector whose action `⟨x K_j⟩₁` is `A` on `V_j`, and the ray is
//!
//! ```text
//!     Ω_j = Σ_k (−K_j)^{∧k} / (k! (1 + c_j)^k)
//! ```
//!
//! — `(1 + c_j) − K_j` up to scale on one plane, which is `1 + tan(θ_j/2) B̂_j`
//! with no root taken; the wedge powers carry an isoclinic set whole, the
//! product over its planes whatever split of them is written, so none is
//! chosen (checklist (r)). AT THE 180° TURN `1 + c_j = 0` and `K_j = 0`: the
//! ray degenerates and the mode's versor is `V_j`'s own blade, `B̂_j` and
//! `−B̂_j` one rotation — the held `ℤ₂` by construction.
//!
//! VERIFIED, never assumed: the rays commute and their product is `Ω` as a
//! ray (EQ1), else `Defective` by name.

use super::linear::LinearFunction;
use super::modes::{generators, kind_of, rational_roots, spectral_projectors, Undecomposed};
use super::multivector::{one, Mv};
use crate::network::nerve::coord::BivectorKind;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

type Q = BigRational;

/// THE PERIOD OF A MOTION (PHY1), read off its modes and never off a letter
/// rotor's order: the least order at which every mode returns, or why one
/// never does — a turn that closes at no order the algebra admits (a
/// rosette), or a mode that does not turn at all (unbound: a transvection
/// or a null shear carries and never returns).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Period {
    Closes(u64),
    Rosette,
    Unbound,
}

/// The period of a motion whose modes are `modes`: unbound where any mode
/// is a transvection or a null shear, a rosette where a turn closes at no
/// order, else the least common multiple of the modes' orders. `None` where
/// that multiple passes its carrier.
pub fn period_of(modes: &[MotionMode]) -> Option<Period> {
    let open = |m: &&MotionMode| m.order.is_none();
    if modes.iter().filter(open).any(|m| matches!(m.kind, BivectorKind::Hyperbolic | BivectorKind::Parabolic)) {
        return Some(Period::Unbound);
    }
    if modes.iter().any(|m| m.order.is_none()) {
        return Some(Period::Rosette);
    }
    let gcd = |mut a: u64, mut b: u64| {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    };
    let mut l = 1u64;
    for o in modes.iter().filter_map(|m| m.order) {
        let o = u64::from(o);
        l = (l / gcd(l, o)).checked_mul(o)?;
    }
    Some(Period::Closes(l))
}

/// ONE MODE OF A MOTION: its versor ray, its class, the planes it spans (two
/// or more for an isoclinic set, held whole), the value `c` of `S` on its
/// space, and the least order at which the motion closes on that space —
/// `None` for a parabolic or hyperbolic mode, open by name, and for a turn
/// that closes at no order the algebra admits (a rosette).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MotionMode {
    pub ray: Mv,
    pub kind: BivectorKind,
    pub planes: usize,
    pub c: Q,
    pub order: Option<u32>,
    /// MR1b — the mode holds a reflection: its space is where the action is
    /// `−1` and its dimension is odd, so the action's determinant is `−1`
    /// there. Only an odd versor has one.
    pub reflects: bool,
}

/// MR1b — IS `m` ODD: every even grade vanishes and an odd one does not. An
/// odd versor is a product of an odd number of reflections, and it acts as
/// `x ↦ −m x m⁻¹`, orientation-reversing (`det R = −1`).
pub(super) fn is_odd(m: &Mv) -> bool {
    let top = u32::from(m.n()) + 2;
    let even_zero = (0..=top).step_by(2).all(|g| m.grade(g).is_zero());
    even_zero && !m.is_zero()
}

/// THE ACTION OF A MOTION ON THE VECTORS, `x ↦ ±Ω x Ω⁻¹` (the sign `−` for an
/// odd versor, MR1b) — the `R_c` whose ℚ-algebra is a cell's capacity (CG1),
/// a linear function of the vectors at the motion's own weight (78.28(2)).
/// `None` where the motion is no versor.
pub fn action_of(motion: &Mv) -> Option<LinearFunction> {
    LinearFunction::of_versor(motion)
}

/// MR1b — THE DETERMINANT OF THE ACTION, named on the reading: `−1` for an
/// odd versor (orientation-reversing), `+1` for an even one. `None` where
/// the element is not a versor (a versor is of one parity).
pub fn action_det(motion: &Mv) -> Option<i8> {
    super::is_versor(motion).then_some(())?;
    match is_odd(motion) {
        true => Some(-1),
        false => Some(1),
    }
}

/// The blade a basis spans: its vectors' outer product.
fn blade_of(space: &[Mv]) -> Mv {
    space.iter().zip(1u32..).fold(Mv::scalar(one()), |w, (v, k)| w.geo(v).grade(k))
}

/// The class of a space held whole, by the sign of its blade's weight: a
/// definite space turns, one holding the timelike direction boosts, a
/// degenerate one has no class here.
fn kind_of_space(blade: &Mv) -> Option<BivectorKind> {
    let w = blade.geo(&blade.reverse()).scalar_part();
    match (w.is_positive(), w.is_negative()) {
        (true, _) => Some(BivectorKind::Elliptic),
        (_, true) => Some(BivectorKind::Hyperbolic),
        _ => None,
    }
}

/// THE MODES OF A MOTION (MR1). `None` where `motion` is not a versor — it
/// has no action to read. The parabolic mode, where there is one, first;
/// then the rest by `c`, ascending. An empty list is the identity.
pub fn motion_modes(
    motion: &Mv,
    budget: &mut crate::physics::Budget,
) -> Option<Result<Vec<MotionMode>, Undecomposed>> {
    // The motion's action and its reverse's, each at the motion's weight
    // (78.28(2)): the reverse's `Ω̃^ x Ω` is the inverse's up to the weight.
    let r = LinearFunction::of_versor(motion)?;
    let r_inv = LinearFunction::of_versor(&motion.reverse())?;
    let half = Q::new(1.into(), 2.into());
    let s = r.plus(&r_inv).scaled(&half);
    let a = r.minus(&r_inv).scaled(&half);
    Some(split(motion, &s, &a, budget))
}

/// 78.62 — WHETHER A MOTION CARRIES CHARGE: whether some mode of it turns
/// or boosts — `c_j ≠ 1` on some invariant space — read off its own action
/// with no root taken and no budget spent. Every mode at `c_j = 1` is
/// exactly `S − I` nilpotent, `S = (R + R⁻¹)/2`, so the motion is UNCHARGED
/// where `(S − I)ᵈ = 0` at the vectors' dimension `d` — a translation (its
/// `S` is not the identity, its values all `1`) and the identity among
/// them. It replaces the scalar `μ = ⟨Ω⟩₀²/⟨ΩΩ̃⟩₀ − 1`, which squared one
/// angle out of a motion of several planes. `None` where the element is no
/// versor: it has no action, unread (rule 11).
pub fn charged(motion: &Mv) -> Option<bool> {
    let r = LinearFunction::of_versor(motion)?;
    let r_inv = LinearFunction::of_versor(&motion.reverse())?;
    let half = Q::new(1.into(), 2.into());
    let n = r.n();
    let off = r.plus(&r_inv).scaled(&half).minus(&LinearFunction::identity(n));
    Some(!off.power(generators(n).len()).is_zero())
}

/// THE ORDER OF A TURN WHOSE COSINE IS `c` (Niven): 2 at −1, 3 at −½, 4 at
/// 0, 6 at ½; `None` at any other rational value — no finite order. (1 is a
/// fixed space and has no turn to order.)
fn order_of_point(c: &Q) -> Option<u32> {
    let half = Q::new(1.into(), 2.into());
    match () {
        _ if *c == -one() => Some(2),
        _ if *c == -half.clone() => Some(3),
        _ if c.is_zero() => Some(4),
        _ if *c == half => Some(6),
        _ => None,
    }
}

fn split(
    motion: &Mv,
    s: &LinearFunction,
    a: &LinearFunction,
    budget: &mut crate::physics::Budget,
) -> Result<Vec<MotionMode>, Undecomposed> {
    let n = s.n();
    let d = generators(n).len();
    let values = rational_roots(&s.characteristic(), budget)?;
    // EACH VALUE'S SPACE: the generalised eigenspace `ker((S − c)ᵈ)`, which
    // is the eigenspace itself everywhere but at `c = 1`, where a parabolic
    // mode's unipotent part lives.
    let mut spaces: Vec<(Q, Vec<Mv>)> = Vec::with_capacity(values.len());
    for c in &values {
        let shifted = s.shifted(c);
        let space = shifted.power(d).kernel();
        if *c != one() && shifted.kernel().len() != space.len() {
            return Err(Undecomposed::Defective);
        }
        spaces.push((c.clone(), space));
    }
    if spaces.iter().map(|(_, sp)| sp.len()).sum::<usize>() != d {
        return Err(Undecomposed::Defective);
    }
    // EACH SPACE'S PROJECTOR, A POLYNOMIAL IN `S`: every value but `1` by its
    // spectral polynomial, the generalised space at `1` — where a parabolic
    // mode's nilpotent part lives — as the identity less them.
    let semisimple: Vec<Q> = values.iter().filter(|c| **c != one()).cloned().collect();
    let unit = values.contains(&one()).then(one);
    let per_value = spectral_projectors(s, &semisimple, unit.as_ref());
    let onto_unit = per_value.iter().fold(LinearFunction::identity(n), |acc, p| acc.minus(p));
    let masks = s.masks().to_vec();
    let mut out: Vec<MotionMode> = Vec::new();
    for (c, space) in &spaces {
        let width = space.len();
        let projector = match semisimple.iter().position(|v| v == c) {
            Some(at) => &per_value[at],
            None => &onto_unit,
        };
        let a_j = a.after(projector);
        // `K_j = ½ Σ_k e^k ∧ A_j(e_k)`, with `e^k = e_k / e_k²`.
        let k_j = masks.iter().zip(a_j.images()).fold(Mv::zero(), |acc, (&m, image)| {
            let e = Mv::blade(m, one());
            let reciprocal = e.scale(&(one() / e.geo(&e).scalar_part()));
            acc.add(&reciprocal.geo(image).grade(2).scale(&Q::new(1.into(), 2.into())))
        });
        let plane: Vec<Mv> = space.clone();
        let lift = one() + c;
        let (ray, kind, planes) = match (k_j.is_zero(), lift.is_zero(), *c == one()) {
            // A FIXED SPACE: `S = 1` and no skew part — nothing turns there.
            (true, _, true) => continue,
            // THE 180° TURN: the space's own blade, the held `ℤ₂`. Of ODD
            // dimension it holds a reflection (MR1b): an odd versor's `−1`
            // space, its blade odd, the mirror's normal among it.
            (true, true, _) => {
                let blade = blade_of(&plane);
                (blade.clone(), kind_of_space(&blade).ok_or(Undecomposed::Defective)?, width / 2)
            }
            // `S = c ≠ ±1` with no skew part is not orthogonal: refused.
            (true, false, false) => return Err(Undecomposed::Defective),
            // A skew part at `c = −1` is a reflection-coupled unipotent
            // this reader does not carry: refused by name.
            (false, true, _) => return Err(Undecomposed::Defective),
            (false, false, parabolic) => {
                let minus = k_j.scale(&-one());
                let mut ray = Mv::scalar(one());
                let mut power = Mv::scalar(one());
                let mut k = 1u32;
                loop {
                    power = power.geo(&minus).grade(2 * k);
                    if power.is_zero() {
                        break;
                    }
                    let factorial = (1..=k).fold(one(), |f, i| f * Q::from_integer(i.into()));
                    let weight = one() / (factorial * num_traits::pow(lift.clone(), k as usize));
                    ray = ray.add(&power.scale(&weight));
                    k += 1;
                }
                let planes = if parabolic { 1 } else { width / 2 };
                let kind = match planes {
                    1 => kind_of(&k_j),
                    _ => kind_of_space(&blade_of(&plane)).ok_or(Undecomposed::Defective)?,
                };
                (ray, kind, planes)
            }
        };
        // THE ORDER IS READ OFF THE MODE'S RATIONAL POINT, never by taking
        // powers (78.62): on its own space the action turns by θ with
        // `cos θ = c ∈ ℚ`, and a rotation of finite order has a rational
        // cosine only at 1, ½, 0, −½, −1 (Niven) — orders 1, 6, 4, 3, 2. Any
        // other rational `c` never returns: a rosette, `None`.
        let order = match kind {
            BivectorKind::Elliptic => order_of_point(c),
            _ => None,
        };
        let reflects = *c == -one() && width % 2 == 1;
        out.push(MotionMode { ray, kind, planes, c: c.clone(), order, reflects });
    }
    // THE PARABOLIC MODE FIRST, then by `c` — the order `rational_roots`
    // read the values in, kept.
    let (parabolic, rest): (Vec<MotionMode>, Vec<MotionMode>) = out.into_iter().partition(|m| m.c == one());
    let out: Vec<MotionMode> = parabolic.into_iter().chain(rest).collect();
    // VERIFIED AGAINST THE MOTION: the rays commute and multiply back to it.
    for (i, x) in out.iter().enumerate() {
        for y in out.iter().skip(i + 1) {
            if !x.ray.geo(&y.ray).sub(&y.ray.geo(&x.ray)).is_zero() {
                return Err(Undecomposed::Defective);
            }
        }
    }
    let product = out.iter().fold(Mv::scalar(one()), |acc, m| acc.geo(&m.ray));
    if !product.projective_eq(motion) {
        return Err(Undecomposed::Defective);
    }
    Ok(out)
}
