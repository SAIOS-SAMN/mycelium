// 78.12(2), MR1 — THE MODES OF A MOTION, read off the motion's own action and
// never through a chart: the invariant planes of `R(x) = Ω x Ω⁻¹`, each
// with its rational `c_j`, its exact versor ray, its class and its closure;
// the rays verified to commute and to multiply back to `Ω`.
//
//! `R` is a rational orthogonal map for every rational versor, so
//! `S = (R + R⁻¹)/2` is self-adjoint and `A = (R − R⁻¹)/2` skew. `S`'s
//! spectral split — the same Faddeev–LeVerrier and `nullspace` that read a
//! generator's modes — gives the invariant spaces `V_j`, where `S = c_j`:
//! `cos θ_j` on a turn, `cosh θ_j` on a boost, `1` on the generalised kernel
//! a parabolic mode lives in. Each `c_j` is rational wherever the motion is:
//! `0` for the quarter turn, `−1` for the half turn, `3/5` for the `(3,4,5)`
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
//! chosen (checklist (r)). AT THE HALF TURN `1 + c_j = 0` and `K_j = 0`: the
//! ray degenerates and the mode's versor is `V_j`'s own blade, `B̂_j` and
//! `−B̂_j` one rotation — the held `ℤ₂` by construction.
//!
//! VERIFIED, never assumed: the rays commute and their product is `Ω` as a
//! ray (EQ1), else `Defective` by name.

use super::modes::{characteristic, column, generators, identity, invert, kind_of, mul, order_on, rational_roots, Undecomposed};
use super::multivector::{one, Mv};
use crate::network::nerve::coord::BivectorKind;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

type Q = BigRational;

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
}

/// `x ↦ a x b` on the vectors, as the matrix of its columns.
fn action(a: &Mv, b: &Mv, masks: &[usize]) -> Vec<Vec<Q>> {
    let d = masks.len();
    let cols: Vec<Vec<Q>> =
        masks.iter().map(|&m| column(&a.geo(&Mv::blade(m, one())).geo(b).grade(1), masks)).collect();
    (0..d).map(|i| (0..d).map(|j| cols[j][i].clone()).collect()).collect()
}

/// THE ACTION OF A MOTION ON THE VECTORS, `x ↦ Ω x Ω⁻¹`, as the matrix of its
/// columns over the algebra's generators — the `R_c` whose ℚ-algebra is a
/// cell's capacity (CG1). `None` where the motion does not invert.
pub fn action_of(motion: &Mv) -> Option<Vec<Vec<Q>>> {
    let inverse = motion.versor_inverse()?;
    Some(action(motion, &inverse, &generators(motion.n())))
}

fn vector(coords: &[Q], masks: &[usize]) -> Mv {
    masks.iter().zip(coords).fold(Mv::zero(), |acc, (&m, c)| acc.add(&Mv::blade(m, c.clone())))
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

fn sub_scalar(a: &[Vec<Q>], c: &Q) -> Vec<Vec<Q>> {
    a.iter()
        .enumerate()
        .map(|(i, row)| row.iter().enumerate().map(|(j, x)| if i == j { x - c } else { x.clone() }).collect())
        .collect()
}

/// THE MODES OF A MOTION (MR1). `None` where `motion` is not a versor — it
/// has no action to read. The parabolic mode, where there is one, first;
/// then the rest by `c`, ascending. An empty list is the identity.
pub fn motion_modes(
    motion: &Mv,
    budget: &mut crate::physics::Budget,
) -> Option<Result<Vec<MotionMode>, Undecomposed>> {
    let inverse = motion.versor_inverse()?;
    super::is_versor(motion)?;
    let masks = generators(motion.n());
    let r = action(motion, &inverse, &masks);
    let r_inv = action(&inverse, motion, &masks);
    let half = Q::new(1.into(), 2.into());
    let s: Vec<Vec<Q>> =
        r.iter().zip(&r_inv).map(|(x, y)| x.iter().zip(y).map(|(p, q)| (p + q) * &half).collect()).collect();
    let a: Vec<Vec<Q>> =
        r.iter().zip(&r_inv).map(|(x, y)| x.iter().zip(y).map(|(p, q)| (p - q) * &half).collect()).collect();
    Some(split(motion, &masks, &s, &a, budget))
}

fn split(
    motion: &Mv,
    masks: &[usize],
    s: &[Vec<Q>],
    a: &[Vec<Q>],
    budget: &mut crate::physics::Budget,
) -> Result<Vec<MotionMode>, Undecomposed> {
    let d = masks.len();
    let values = rational_roots(&characteristic(s), budget)?;
    // EACH VALUE'S SPACE: the generalised eigenspace `ker((S − c)ᵈ)`, which
    // is the eigenspace itself everywhere but at `c = 1`, where a parabolic
    // mode's unipotent part lives.
    let mut spaces: Vec<(Q, Vec<Vec<Q>>)> = Vec::with_capacity(values.len());
    for c in values {
        let shifted = sub_scalar(s, &c);
        let mut power = identity(d);
        for _ in 0..d {
            power = mul(&power, &shifted);
        }
        let space = super::nullspace(power, d);
        if c != one() && super::nullspace(shifted, d).len() != space.len() {
            return Err(Undecomposed::Defective);
        }
        spaces.push((c, space));
    }
    let columns: Vec<Vec<Q>> = spaces.iter().flat_map(|(_, sp)| sp.iter().cloned()).collect();
    if columns.len() != d {
        return Err(Undecomposed::Defective);
    }
    let t: Vec<Vec<Q>> = (0..d).map(|i| columns.iter().map(|c| c[i].clone()).collect()).collect();
    let t_inv = invert(&t).ok_or(Undecomposed::Defective)?;
    let metric: Vec<Q> = masks.iter().map(|&m| Mv::blade(m, one()).geo(&Mv::blade(m, one())).scalar_part()).collect();
    let mut out: Vec<MotionMode> = Vec::new();
    let mut at = 0usize;
    for (c, space) in &spaces {
        let width = space.len();
        let indicator: Vec<Vec<Q>> = (0..d)
            .map(|i| (0..d).map(|j| if i == j && i >= at && i < at + width { one() } else { Q::zero() }).collect())
            .collect();
        at += width;
        let a_j = mul(a, &mul(&mul(&t, &indicator), &t_inv));
        // `K_j = ½ Σ_k e^k ∧ A_j(e_k)`, with `e^k = e_k / e_k²`.
        let k_j = (0..d).fold(Mv::zero(), |acc, k| {
            let image: Vec<Q> = (0..d).map(|i| a_j[i][k].clone()).collect();
            let wedge = Mv::blade(masks[k], one() / &metric[k]).geo(&vector(&image, masks)).grade(2);
            acc.add(&wedge.scale(&Q::new(1.into(), 2.into())))
        });
        let plane: Vec<Mv> = space.iter().map(|v| vector(v, masks)).collect();
        let lift = one() + c;
        let (ray, kind, planes) = match (k_j.is_zero(), lift.is_zero(), *c == one()) {
            // A FIXED SPACE: `S = 1` and no skew part — nothing turns there.
            (true, _, true) => continue,
            // THE HALF TURN: the space's own blade, the held `ℤ₂`.
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
        let order = match kind {
            BivectorKind::Elliptic => order_on(motion, &plane),
            _ => None,
        };
        out.push(MotionMode { ray, kind, planes, c: c.clone(), order });
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
