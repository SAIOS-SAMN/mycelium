//! 78.11, 78.12(2), MR1, MD1 — A LINEAR FUNCTION OF THE ALGEBRA'S VECTORS,
//! held as what it does: its image of each generator of `Cl(n+1, 1)`.
//!
//! No matrix carries it. The function is its images (Hestenes's linear
//! function, fixed by its values on a frame), and it is applied by linearity
//! — `f(x) = Σ_k x_k f(e_k)` with `x_k` the vector's own coefficients —
//! composed by applying one to the other's images, and read by the algebra's
//! own readers:
//!
//! - its TRACE is the vector derivative `∂_x · f(x) = Σ_k e^k · f(e_k)`;
//! - its CHARACTERISTIC POLYNOMIAL comes from the traces of its powers — the
//!   power sums of its values — by Newton's identities, exact over ℚ;
//! - its KERNEL is every vector it sends to zero: the solutions of the
//!   equations its images set (`core::span::solutions`), in the reduced
//!   basis the solution space fixes;
//! - a POLYNOMIAL in it is applied by Horner's rule, application after
//!   application — which is how a spectral projector is read, never through
//!   a change of basis and an inverse.
//!
//! A versor's action is `x ↦ M̂ x M̃ / w` — the twisted adjoint at its own
//! weight, the sandwich for an even versor and its negative for an odd one
//! (78.28(1)) — and a sum of actions (a bracket, an algebra's element) is a
//! linear function too, so one type carries both.

use super::multivector::{one, Mv};
use num_rational::BigRational;
use num_traits::Zero;

type Q = BigRational;

/// A LINEAR FUNCTION OF THE VECTORS OF `Cl(n+1, 1)`: its image of each
/// generator, the generators in ascending bit order (`modes::generators`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinearFunction {
    n: u8,
    masks: Vec<usize>,
    images: Vec<Mv>,
}

impl LinearFunction {
    /// The function whose image of each generator `f` reads — its vector
    /// part, the part a vector goes to.
    pub fn of(n: u8, f: impl Fn(&Mv) -> Mv) -> LinearFunction {
        let masks = super::modes::generators(n);
        let images = masks.iter().map(|&m| f(&Mv::blade(m, one())).grade(1)).collect();
        LinearFunction { n, masks, images }
    }

    /// The function whose image of the generator with mask `m` is `f(m)`.
    pub fn on_generators(n: u8, f: impl Fn(usize) -> Mv) -> LinearFunction {
        let masks = super::modes::generators(n);
        let images = masks.iter().map(|&m| f(m).grade(1)).collect();
        LinearFunction { n, masks, images }
    }

    /// The identity on the vectors.
    pub fn identity(n: u8) -> LinearFunction {
        LinearFunction::of(n, Mv::clone)
    }

    /// A VERSOR'S ACTION ON THE VECTORS at its own weight, `x ↦ M̂ x M̃ / w`
    /// (the sign of an odd versor carried, MR1b). `None` where the element is
    /// no versor: it has no action.
    pub fn of_versor(m: &Mv) -> Option<LinearFunction> {
        let w = super::weight_of(m)?;
        let inv = one() / &w;
        Some(LinearFunction::of(m.n(), |x| m.twisted(x).scale(&inv)))
    }

    /// The `n` of the algebra it acts in.
    pub fn n(&self) -> u8 {
        self.n
    }

    /// The generators it is read on, and its image of each.
    pub fn images(&self) -> &[Mv] {
        &self.images
    }

    /// `f(x)`, by linearity over `x`'s own coefficients.
    pub fn at(&self, x: &Mv) -> Mv {
        self.masks
            .iter()
            .zip(&self.images)
            .fold(Mv::zero(), |acc, (&m, y)| match x.coeff(m).is_zero() {
                true => acc,
                false => acc.add(&y.scale(x.coeff(m))),
            })
    }

    /// `self ∘ inner`: `inner` first. Both act in one algebra — a caller
    /// widens its versors to one `n` before it reads their actions, as every
    /// reader of a cell's algebra does; this, `plus` and `minus` compose
    /// images generator by generator.
    pub fn after(&self, inner: &LinearFunction) -> LinearFunction {
        LinearFunction { n: self.n, masks: self.masks.clone(), images: inner.images.iter().map(|y| self.at(y)).collect() }
    }

    fn zip(&self, other: &LinearFunction, f: impl Fn(&Mv, &Mv) -> Mv) -> LinearFunction {
        LinearFunction {
            n: self.n,
            masks: self.masks.clone(),
            images: self.images.iter().zip(&other.images).map(|(a, b)| f(a, b)).collect(),
        }
    }

    /// `f + g`.
    pub fn plus(&self, other: &LinearFunction) -> LinearFunction {
        self.zip(other, Mv::add)
    }

    /// `f − g`.
    pub fn minus(&self, other: &LinearFunction) -> LinearFunction {
        self.zip(other, Mv::sub)
    }

    /// `c · f`.
    pub fn scaled(&self, c: &Q) -> LinearFunction {
        LinearFunction { n: self.n, masks: self.masks.clone(), images: self.images.iter().map(|y| y.scale(c)).collect() }
    }

    /// `f − c`: the function less `c` times the identity.
    pub fn shifted(&self, c: &Q) -> LinearFunction {
        LinearFunction {
            n: self.n,
            masks: self.masks.clone(),
            images: self.masks.iter().zip(&self.images).map(|(&m, y)| y.sub(&Mv::blade(m, c.clone()))).collect(),
        }
    }

    /// Whether it sends every vector to zero.
    pub fn is_zero(&self) -> bool {
        self.images.iter().all(Mv::is_zero)
    }

    /// `fᵏ`, `f⁰` the identity.
    pub fn power(&self, k: usize) -> LinearFunction {
        (0..k).fold(LinearFunction::identity(self.n), |acc, _| self.after(&acc))
    }

    /// THE TRACE, the vector derivative's reading `∂_x · f(x)`: the sum of
    /// each generator's own coefficient in its image.
    pub fn trace(&self) -> Q {
        self.masks.iter().zip(&self.images).fold(Q::zero(), |acc, (&m, y)| acc + y.coeff(m))
    }

    /// THE CHARACTERISTIC POLYNOMIAL `det(x − f)`, coefficients low to high:
    /// the power sums `pₖ = tr(fᵏ)` gathered by Newton's identities into the
    /// elementary symmetric functions of the values, `eₖ = (1/k) Σ_{i=1}^{k}
    /// (−1)^{i−1} e_{k−i} pᵢ`, and `det(x − f) = Σ (−1)^k eₖ x^{d−k}`. Exact over
    /// ℚ, no root taken.
    pub fn characteristic(&self) -> Vec<Q> {
        let d = self.masks.len();
        let mut p: Vec<Q> = Vec::with_capacity(d + 1);
        p.push(Q::zero());
        let mut fk = LinearFunction::identity(self.n);
        for _ in 1..=d {
            fk = self.after(&fk);
            p.push(fk.trace());
        }
        let mut e: Vec<Q> = vec![one()];
        for k in 1..=d {
            let mut s = Q::zero();
            for i in 1..=k {
                let term = &e[k - i] * &p[i];
                s = match i % 2 {
                    1 => s + term,
                    _ => s - term,
                };
            }
            e.push(s / Q::from_integer(num_bigint::BigInt::from(k)));
        }
        let mut coeff = vec![Q::zero(); d + 1];
        for (k, ek) in e.iter().enumerate() {
            coeff[d - k] = match k % 2 {
                0 => ek.clone(),
                _ => -ek.clone(),
            };
        }
        coeff
    }

    /// THE KERNEL: every vector the function sends to zero, as the reduced
    /// basis of the solutions of the equations its images set — for each
    /// generator `eᵢ`, `Σ_j xⱼ ⟨f(eⱼ)⟩ᵢ = 0`.
    pub fn kernel(&self) -> Vec<Mv> {
        let d = self.masks.len();
        let rows: Vec<Vec<Q>> =
            self.masks.iter().map(|&mi| self.images.iter().map(|y| y.coeff(mi).clone()).collect()).collect();
        crate::core::span::solutions(rows, d).iter().map(|v| self.vector(v)).collect()
    }

    /// A POLYNOMIAL IN THE FUNCTION, `Σ cₖ fᵏ` (coefficients low to high), by
    /// Horner's rule.
    pub fn polynomial(&self, coeff: &[Q]) -> LinearFunction {
        let id = LinearFunction::identity(self.n);
        coeff.iter().rev().fold(id.scaled(&Q::zero()), |acc, c| self.after(&acc).plus(&id.scaled(c)))
    }

    /// The vector with these coefficients on the generators.
    pub fn vector(&self, coords: &[Q]) -> Mv {
        self.masks.iter().zip(coords).fold(Mv::zero(), |acc, (&m, c)| acc.add(&Mv::blade(m, c.clone())))
    }

    /// The generators, as masks in ascending bit order.
    pub fn masks(&self) -> &[usize] {
        &self.masks
    }

    /// THE FUNCTION AS ONE VECTOR OF THE SPACE OF LINEAR FUNCTIONS — its
    /// images' coefficients in turn — as the ray `core::span` joins: its
    /// denominators cleared and its content divided out, exact.
    pub fn as_ray(&self) -> crate::core::span::Vector {
        let d = self.masks.len();
        ray(self
            .images
            .iter()
            .enumerate()
            .flat_map(|(j, y)| self.masks.iter().enumerate().map(move |(i, &m)| (j * d + i, y.coeff(m))))
            .collect())
    }
}

/// AN ELEMENT OF THE ALGEBRA AS THE RAY IT IS, for `core::span`: its
/// coefficients on every blade, denominators cleared and content divided
/// out, exact.
pub fn ray_of(v: &Mv) -> crate::core::span::Vector {
    ray((0..v.slot_count()).map(|k| (k, v.coeff(k))).collect())
}

/// Exact rational coordinates as the primitive integer ray on them.
fn ray(coords: Vec<(usize, &Q)>) -> crate::core::span::Vector {
    use num_bigint::BigInt;
    let flat: Vec<(usize, &Q)> = coords.into_iter().filter(|(_, c)| !c.is_zero()).collect();
    let lcm = flat.iter().fold(BigInt::from(1), |l, (_, c)| num_integer::Integer::lcm(&l, c.denom()));
    let ints: Vec<(usize, BigInt)> =
        flat.iter().map(|(at, c)| (*at, (*c * Q::from_integer(lcm.clone())).to_integer())).collect();
    let g = ints.iter().fold(BigInt::from(0), |g, (_, x)| num_integer::Integer::gcd(&g, x));
    match g.is_zero() {
        true => ints,
        false => ints.into_iter().map(|(at, x)| (at, x / &g)).collect(),
    }
}
