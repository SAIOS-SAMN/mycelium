// 50.8, A6, 16.4, 55.3, 78.38(C), A.12 — CONTINUOUS SUPPORT CORRESPONDENCE:
// the transition between two shapes read on their exact moment towers, never
// on where a carried compartment lands.
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
    /// The centre, where the compartment's conformal points stand (16.4): the
    /// frame every versor of a carriage acts in.
    pub centre: Vec<Q>,
    /// The second moment on the body's `n` axes, normalised by `M₀` (A.12):
    /// order two on every plane, the base planes and every plane between them.
    pub second: Vec<Vec<Q>>,
    /// The third and fourth harmonics on each plane of the base, in
    /// [`crate::transducer::base::planes`] order; `None` where one vanishes.
    pub upper: Vec<[Option<[Q; 2]>; 2]>,
}

/// THE PART OF A BODY A FIT LEAVES (ruled 2026-09-26): on the image plane,
/// at an order above the one the fit holds through, `h_b − ρ^m ω^m h_a`,
/// exact. A filled `k`-blow-up leaves the sensor block's own `S₄(k)/k²` at
/// order four. It is the transition's, carried beside its versor, never
/// absorbed — RM1's remainder, the price a fit pays.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Residue {
    pub plane: (usize, usize),
    pub order: Harmonic,
    pub vector: [Q; 2],
}

/// `z · w` over ℚ(i).
fn times(z: &[Q; 2], w: &[Q; 2]) -> [Q; 2] {
    [&z[0] * &w[0] - &z[1] * &w[1], &z[0] * &w[1] + &z[1] * &w[0]]
}

/// The base axis `e_{k+1}` as a vector of the algebra.
fn axis(k: usize) -> Mv {
    Mv::blade(cga::base_axis(k + 1), Q::one())
}

/// A vector's part on the body's `n` base axes.
fn on_base(y: &Mv, n: usize) -> Mv {
    cga::base_vector(&(0..n).map(|k| y.coeff(cga::base_axis(k + 1)).clone()).collect::<Vec<Q>>())
}

/// THE SECOND MOMENT AS THE MAP IT IS, `M(x) = ⟨(d·x) d⟩`: the vector the
/// tower sends `x` to, formed on the body's `n` axes.
fn moment(m: &[Vec<Q>], x: &Mv, n: usize) -> Mv {
    cga::base_vector(
        &(0..n)
            .map(|l| (0..n).fold(Q::zero(), |acc, k| acc + x.coeff(cga::base_axis(k + 1)) * &m[k][l]))
            .collect::<Vec<Q>>(),
    )
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
            Tower::Sites => b.towers.points.as_ref()?.m.clone(),
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
    /// (`conformal_factors`), `V = T·D(ρ)·R`, a turn or a mirror, each reading
    /// formed by the sandwich (`carries_linear`):
    ///
    /// ```text
    ///     centre     V(c_a) = c_b
    ///     order 2    M_b(R̂ e_k R̃) = ρ² R̂ M_a(e_k) R̃        every axis, every plane at once
    ///     order m    h_b^(m) = ρ^m ω^m h_a^(m)             the plane's orientation kept
    ///                h_b^(m) = ρ^m ω^m conj(h_a^(m))       reversed: a mirror conjugates
    /// ```
    ///
    /// with `ω` the image of the plane's first axis on the base plane the
    /// sandwich carries it onto. A harmonic vanishing on one side and not the
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
    ///
    /// THE FIT IS READ THROUGH AN ORDER (ruled 2026-09-26, the cost of a fit):
    /// every order up to `through` must be carried exactly; above it the
    /// difference is not a refusal but the transition's RESIDUE, returned
    /// whole — `None` where the versor does not carry the body through
    /// `through`.
    pub fn matches(&self, other: &Self, v: &Mv, through: Harmonic) -> Option<Vec<Residue>> {
        let n = self.centre.len();
        if self.tower != other.tower || other.centre.len() != n {
            return None;
        }
        let f = crate::transducer::element::conformal_factors(v).ok()?;
        if f.inverted || f.transverses() {
            return None;
        }
        if cga::base_at(&v.sandwich(&cga::point_at(&self.centre)), n).as_deref() != Some(other.centre.as_slice()) {
            return None;
        }
        self.carries_linear(other, &f.rotation, &f.rho, through)
    }

    /// THE COVARIANCE OF THE LINEAR PART, the rotation ray `R` and the scale
    /// `ρ` in hand — the transition's own candidates know both, so nothing is
    /// factored per candidate. Every reading is a vector the SANDWICH forms
    /// (law 11, 3.9(c)): no matrix carries the action.
    ///
    /// - Each base axis's image `R̂ e_k R̃` stays on the body's `n` axes.
    /// - Order two: `M_b(R̂ e_k R̃) = ρ² · R̂ M_a(e_k) R̃` for every `e_k` —
    ///   both sides at the ray's weight `w`, so nothing is divided.
    /// - A plane's image is the sandwich of its two axes: where it is a base
    ///   plane `(k, l)`, the first axis's image there is `w·ω` and the pair's
    ///   orientation the mirror's sign, and `w^m · h_b^(m) = (ρ w ω)^m ·
    ///   h_a^(m)` (a mirror conjugating `h_a` first).
    ///
    /// Orders up to `through` must hold exactly; each order above it that
    /// does not is returned as a [`Residue`], `h_b − ρ^m ω^m h_a` exact on the
    /// image plane — the part of `b` the fit leaves, never absorbed. Order two
    /// is carried by every fit the certificate passes, so the tensor is read
    /// exactly always.
    pub(super) fn carries_linear(&self, other: &Self, rotation: &Mv, rho: &Q, through: Harmonic) -> Option<Vec<Residue>> {
        let n = self.centre.len();
        if self.tower != other.tower || other.centre.len() != n {
            return None;
        }
        let w = cga::weight_of(rotation)?;
        if w.is_zero() {
            return None;
        }
        let images: Vec<Mv> = (0..n).map(|k| rotation.twisted(&axis(k))).collect();
        if !images.iter().all(|y| y.sub(&on_base(y, n)).is_zero()) {
            return None;
        }
        let rho2 = rho * rho;
        let second_carried = images.iter().enumerate().all(|(k, y)| {
            let turned = rotation.twisted(&moment(&self.second, &axis(k), n)).scale(&rho2);
            moment(&other.second, y, n).sub(&turned).is_zero()
        });
        if !second_carried {
            return None;
        }
        let mut residue: Vec<Residue> = Vec::new();
        let planes = crate::transducer::base::planes(n);
        for (p, held) in planes.iter().zip(&self.upper) {
            let (yi, yj) = (&images[p.0], &images[p.1]);
            let at = |y: &Mv, k: usize| y.coeff(cga::base_axis(k + 1)).clone();
            let rows: Vec<usize> = (0..n).filter(|&k| !at(yi, k).is_zero() || !at(yj, k).is_zero()).collect();
            // The plane's image, where it is a plane of the base.
            let &[k, l] = rows.as_slice() else { continue };
            let theirs = crate::transducer::base::plane_index((k, l)).and_then(|ix| other.upper.get(ix))?;
            let omega = [&at(yi, k) * rho, &at(yi, l) * rho];
            let mirror = at(yi, k) * at(yj, l) - at(yi, l) * at(yj, k) < Q::zero();
            let zero = [Q::zero(), Q::zero()];
            let carried = |h: Option<&[Q; 2]>, m: u32| -> [Q; 2] {
                let Some(h) = h else { return zero.clone() };
                let start = match mirror {
                    true => [h[0].clone(), -&h[1]],
                    false => h.clone(),
                };
                (0..m).fold(start, |z, _| times(&z, &omega))
            };
            for (order, m, (mine, yours)) in [(Harmonic::Three, 3u32), (Harmonic::Four, 4)]
                .into_iter()
                .zip(held.iter().zip(theirs))
                .map(|((o, m), pair)| (o, m, pair))
            {
                // `w^m · (h_b − ρ^m ω^m h_a)`, then the weight divided out:
                // the residue at its own scale, exact.
                let wm = (0..m).fold(Q::one(), |acc, _| acc * &w);
                let c = carried(mine.as_ref(), m);
                // A harmonic that vanishes on `b` is the zero vector there.
                let hb = match yours {
                    Some(h) => h,
                    None => &zero,
                };
                let diff = [(&hb[0] * &wm - &c[0]) / &wm, (&hb[1] * &wm - &c[1]) / &wm];
                if diff[0].is_zero() && diff[1].is_zero() {
                    continue;
                }
                if order <= through {
                    return None;
                }
                residue.push(Residue { plane: (k, l), order, vector: diff });
            }
        }
        Some(residue)
    }
}
