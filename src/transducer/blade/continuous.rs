// 50.8, A6, 16.4, 55.3, 78.38(C), A.12 — CONTINUOUS SUPPORT CORRESPONDENCE:
// the transition between two shapes read on their exact moment towers, never
// on where a carried lamina lands.
//
//! The crystallographic restriction of discrete raster grids (ℤ² admits C₄/D₄;
//! ℤ³ admits O_h) limits lattice-to-lattice bijections to finite-order torsion
//! ({1, 2, 4} in 2D, {1, 2, 3, 4, 6} in 3D).
//!
//! A rotation by a non-crystallographic rational angle θ (such as the Pythagorean
//! prime angles π₅ with cos θ = 3/5, sin θ = 4/5, or π₁₃ with cos θ = 5/13, sin θ = 12/13)
//! does not map integer coordinates to integers.
//!
//! Continuous support correspondence relates bodies by their exact algebraic
//! moments in ℚ, on the tower whose similarity carries the pair (78.38(C)):
//! the centre, the whole second moment on the body's `n` axes, and the odd
//! and quartic harmonics on every plane. Under a versor they are TURNED by its
//! action and compared, never reduced to their squared norms first (SQ1-b4,
//! Axiom C as tightened), and the lattice is never consulted: the lattice is
//! read at the sensor (70.7, 76.1, 78.30(4)).

use crate::transducer::blade::body::Blade;
use crate::transducer::blade::tower::{harmonic, Harmonic, Tower};
use crate::transducer::cga::{self, Mv};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

type Q = BigRational;

/// A body's exact continuous support on one tower.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuousSupport {
    /// The tower the moments are read on (78.38(C)).
    pub tower: Tower,
    /// The centre, where the lamina's conformal points stand (16.4): the
    /// frame every versor of a carriage acts in.
    pub centre: Vec<Q>,
    /// The second moment on the body's `n` axes, normalised by `M₀` (A.12):
    /// order two on every plane, the base planes and every plane between them.
    pub second: Vec<Vec<Q>>,
    /// The third and fourth harmonics on each plane of the base, in
    /// [`crate::transducer::base::planes`] order; `None` where one vanishes.
    pub upper: Vec<[Option<[Q; 2]>; 2]>,
}

/// `z · w` over ℚ(i).
fn times(z: &[Q; 2], w: &[Q; 2]) -> [Q; 2] {
    [&z[0] * &w[0] - &z[1] * &w[1], &z[0] * &w[1] + &z[1] * &w[0]]
}

/// THE LINEAR ACTION OF A ROTATION RAY on the base's `n` axes: column `c` is
/// the image of `e_{c+1}` under the twisted adjoint (78.28(1)), divided by the
/// ray's weight — a turn or a mirror alike. `None` where the ray is no versor
/// or carries a base axis off the body's `n` axes.
fn action(r: &Mv, n: usize) -> Option<Vec<Vec<Q>>> {
    let w = cga::weight_of(r)?;
    if w.is_zero() {
        return None;
    }
    let cols = (0..n)
        .map(|c| {
            let image = r.twisted(&Mv::blade(cga::base_axis(c + 1), Q::one()));
            let col: Vec<Q> = (0..n).map(|k| image.coeff(cga::base_axis(k + 1)) / &w).collect();
            image.sub(&cga::base_vector(&col).scale(&w)).is_zero().then_some(col)
        })
        .collect::<Option<Vec<Vec<Q>>>>()?;
    Some((0..n).map(|row| cols.iter().map(|c| c[row].clone()).collect()).collect())
}

impl ContinuousSupport {
    /// Lift a body into its continuous support on one tower. `None` for a
    /// body with no support.
    pub fn of(b: &Blade, tower: Tower) -> Option<Self> {
        let n = b.n();
        let centre = super::carry::vertex_centroid(b)?;
        let second = match tower {
            Tower::Region => {
                let m0 = Q::from_integer(BigInt::from(i64::try_from(b.support.len()).ok()?));
                if m0.is_zero() {
                    return None;
                }
                (0..n)
                    .map(|i| (0..n).map(|j| b.moment2(i, j).map(|x| x / &m0)).collect::<Option<Vec<Q>>>())
                    .collect::<Option<Vec<Vec<Q>>>>()?
            }
            Tower::Sites => super::pose::point_second_moment(b, n)?,
        };
        let upper = crate::transducer::base::planes(n)
            .into_iter()
            .map(|p| [harmonic(b, p, tower, Harmonic::Three), harmonic(b, p, tower, Harmonic::Four)])
            .collect();
        Some(ContinuousSupport { tower, centre, second, upper })
    }

    /// DOES THE VERSOR CARRY THIS BODY ONTO THE OTHER — the transition between
    /// two shapes, one edge of a cover, read on the tower (SQ1-b4, 55.3, A.12).
    /// Congruence is the cover's reading over many shapes (24.4), which the
    /// cycles of these edges feed.
    ///
    /// The versor is IN HAND, so it is applied. Read at both poles
    /// (`conformal_factors`), `V = T·D(ρ)·R`, with `A` the action of `R` on the
    /// base's axes — orthogonal, `det A = ±1`, a turn or a mirror:
    ///
    /// ```text
    ///     centre     V(c_a) = c_b
    ///     order 2    M_b = ρ² A M_a Aᵀ                    on every plane at once
    ///     order m    h_b^(m) = ρ^m w^m h_a^(m)            det +1 on the plane
    ///                h_b^(m) = ρ^m w^m conj(h_a^(m))      det −1: a mirror conjugates
    /// ```
    ///
    /// with `w` the image of the plane's first axis on the base plane the
    /// action carries it onto. A harmonic vanishing on one side and not the
    /// other is a difference of shape; on both, agreement. The chirality `Z =
    /// h₂²·conj(h₄)` is carried exactly when its orders are, so it is not a
    /// second gate.
    ///
    /// A plane the action turns off every base plane — the frame planes under
    /// an oblique turn of the grid plane — has no held harmonic to compare its
    /// image with: its order two is the tensor's above, and its orders three
    /// and four are not held by the tower, which reads them on the base's
    /// planes only (A.11 on `n` axes carries no rank-3 or rank-4 tensor).
    ///
    /// A versor that moves infinity — a transversion, or the inversion's cell —
    /// acts on the tower non-linearly, and A.13 is docketed and not built:
    /// refused, never approximated.
    pub fn matches(&self, other: &Self, v: &Mv) -> bool {
        let n = self.centre.len();
        if self.tower != other.tower || other.centre.len() != n {
            return false;
        }
        let Ok(f) = crate::transducer::element::conformal_factors(v) else {
            return false;
        };
        if f.inverted || f.transverses() {
            return false;
        }
        if cga::base_at(&v.sandwich(&cga::point_at(&self.centre)), n).as_deref() != Some(other.centre.as_slice()) {
            return false;
        }
        let Some(a) = action(&f.rotation, n) else {
            return false;
        };
        let rho2 = &f.rho * &f.rho;
        let turned = |r: usize, c: usize| -> Q {
            (0..n).fold(Q::zero(), |acc, k| {
                acc + (0..n).fold(Q::zero(), |row, l| row + &a[r][k] * &self.second[k][l] * &a[c][l])
            })
        };
        if !(0..n).all(|r| (0..n).all(|c| other.second[r][c] == &rho2 * turned(r, c))) {
            return false;
        }
        let planes = crate::transducer::base::planes(n);
        for (p, held) in planes.iter().zip(&self.upper) {
            let rows: Vec<usize> = (0..n).filter(|&r| !a[r][p.0].is_zero() || !a[r][p.1].is_zero()).collect();
            // The plane's image, where it is a plane of the base.
            let &[k, l] = rows.as_slice() else { continue };
            let Some(theirs) = crate::transducer::base::plane_index((k, l)).and_then(|ix| other.upper.get(ix)) else {
                return false;
            };
            let w = [a[k][p.0].clone(), a[l][p.0].clone()];
            let mirror = &a[k][p.0] * &a[l][p.1] - &a[k][p.1] * &a[l][p.0] < Q::zero();
            let carried = |h: &[Q; 2], m: u32| -> [Q; 2] {
                let start = match mirror {
                    true => [h[0].clone(), -&h[1]],
                    false => h.clone(),
                };
                (0..m).fold(start, |z, _| {
                    let t = times(&z, &w);
                    [&t[0] * &f.rho, &t[1] * &f.rho]
                })
            };
            for (m, (mine, yours)) in [3u32, 4].into_iter().zip(held.iter().zip(theirs)) {
                let agrees = match (mine, yours) {
                    (None, None) => true,
                    (Some(h), Some(hb)) => carried(h, m) == *hb,
                    _ => false,
                };
                if !agrees {
                    return false;
                }
            }
        }
        true
    }
}
