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

/// A body's exact continuous support on one tower: a view of the body's
/// own coordinates, borrowed where the lift holds them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuousSupport<'a> {
    /// The tower the moments are read on (78.38(C)).
    pub tower: Tower,
    /// The centre, where the compartment's conformal points stand (16.4): the
    /// frame every versor of a carriage acts in.
    pub centre: Vec<Q>,
    /// The second moment on the body's `n` axes, normalised by `M₀` (A.12):
    /// order two on every plane, the base planes and every plane between them.
    /// Borrowed from the point tower on the sites; derived from the inertia
    /// on the region.
    pub second: std::borrow::Cow<'a, cga::LinearFunction>,
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

/// WHY A VERSOR DOES NOT CARRY ONE BODY ONTO THE OTHER BY COVARIANCE, each
/// by name: never an empty answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotCarried {
    /// The two supports stand on different towers or bases, or on a base
    /// whose axes the algebra does not hold (no frame to carry).
    Frames,
    /// The versor is not read at both poles (`conformal_factors`).
    Unfactored(crate::transducer::element::Unfactored),
    /// The versor moves infinity — a transversion, or the inversion's cell:
    /// A.13's action on the tower is not built, and nothing is approximated.
    MovesInfinity,
    /// The carried centre is not the other body's centre.
    Centre,
    /// The rotation has no weight: it is no ray.
    Weightless,
    /// A base axis's image leaves the body's `n` axes.
    OffAxes,
    /// The second moment is not carried.
    Second,
    /// An order up to the fit's differs.
    Order(Harmonic),
}

/// WHAT A CARRYING VERSOR'S COVARIANCE READ: the residue above the fit's
/// order, and — named, never passed over — the planes of the base whose
/// image is no plane of the base, whose orders three and four the tower
/// holds no harmonic to compare with (A.11 on `n` axes carries no rank-3 or
/// rank-4 tensor). Their order two is the tensor's, read whole above.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Covariance {
    pub residue: Vec<Residue>,
    pub unread: Vec<(usize, usize)>,
}

/// `z · w` over ℚ(i).
fn times(z: &[Q; 2], w: &[Q; 2]) -> [Q; 2] {
    [&z[0] * &w[0] - &z[1] * &w[1], &z[0] * &w[1] + &z[1] * &w[0]]
}

/// The base axis `e_{k+1}` as a vector of the algebra; `None` past the
/// algebra's axes.
fn axis(k: usize) -> Option<Mv> {
    Some(Mv::blade(cga::base_axis(k + 1)?, Q::one()))
}

/// A vector's part on the body's `n` base axes (a body's `n` is its sites'
/// arity, within the algebra).
fn on_base(y: &Mv, n: usize) -> Mv {
    cga::base_vector(&(1..=n).map_while(cga::base_axis).map(|a| y.coeff(a).clone()).collect::<Vec<Q>>())
}

impl<'a> ContinuousSupport<'a> {
    /// A body's continuous support on one tower, read off its coordinates.
    /// `None` for a body with no support.
    pub fn of(b: &'a Blade, tower: Tower) -> Option<Self> {
        let n = b.n();
        let centre = super::carry::vertex_centroid(b)?;
        let second = match tower {
            // THE REGION'S SECOND MOMENT AS THE FUNCTION IT IS, off the
            // body's inertia: `M(eᵢ) = Σ_j I_ij eⱼ / M₀`.
            Tower::Region => {
                let m0 = Q::from_integer(BigInt::from(i64::try_from(b.support.len()).ok()?));
                if m0.is_zero() {
                    return None;
                }
                let rows: Vec<Mv> = (0..n)
                    .map(|i| {
                        (0..n).try_fold(Mv::zero(), |acc, j| Some(acc.add(&axis(j)?.scale(&(b.moment2(i, j)? / &m0)))))
                    })
                    .collect::<Option<Vec<Mv>>>()?;
                let at = |mask: usize| (0..n).find(|&i| cga::base_axis(i + 1) == Some(mask));
                std::borrow::Cow::Owned(cga::LinearFunction::on_generators(u8::try_from(n).ok()?, |mask| match at(mask) {
                    Some(i) => rows[i].clone(),
                    None => Mv::zero(),
                }))
            }
            Tower::Sites => std::borrow::Cow::Borrowed(&b.towers.points.as_ref()?.m),
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
    /// refused by that name, never approximated.
    ///
    /// THE FIT IS READ THROUGH AN ORDER (ruled 2026-09-26, the cost of a fit):
    /// every order up to `through` must be carried exactly; above it the
    /// difference is not a refusal but the transition's RESIDUE, returned
    /// whole. Where the versor does not carry the body, why.
    pub fn matches(&self, other: &Self, v: &Mv, through: Harmonic) -> Result<Covariance, NotCarried> {
        let n = self.centre.len();
        if self.tower != other.tower || other.centre.len() != n {
            return Err(NotCarried::Frames);
        }
        let f = crate::transducer::element::conformal_factors(v).map_err(NotCarried::Unfactored)?;
        if f.inverted || f.transverses() {
            return Err(NotCarried::MovesInfinity);
        }
        if cga::base_at(&v.sandwich(&cga::point_at(&self.centre)), n).as_deref() != Some(other.centre.as_slice()) {
            return Err(NotCarried::Centre);
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
    pub(super) fn carries_linear(
        &self,
        other: &Self,
        rotation: &Mv,
        rho: &Q,
        through: Harmonic,
    ) -> Result<Covariance, NotCarried> {
        let n = self.centre.len();
        if self.tower != other.tower || other.centre.len() != n {
            return Err(NotCarried::Frames);
        }
        cga::weight_of(rotation).filter(|w| !w.is_zero()).ok_or(NotCarried::Weightless)?;
        // The frame's axes: a body standing on axes the algebra does not hold
        // has no frame to carry.
        let unit: Vec<Mv> = (0..n).map(axis).collect::<Option<Vec<Mv>>>().ok_or(NotCarried::Frames)?;
        let images: Vec<Mv> = unit.iter().map(|e| rotation.twisted(e)).collect();
        if !images.iter().all(|y| y.sub(&on_base(y, n)).is_zero()) {
            return Err(NotCarried::OffAxes);
        }
        let rho2 = rho * rho;
        let second_carried = images.iter().zip(&unit).all(|(y, e)| {
            let turned = rotation.twisted(&self.second.at(e)).scale(&rho2);
            other.second.at(y).sub(&turned).is_zero()
        });
        if !second_carried {
            return Err(NotCarried::Second);
        }
        let mut residue: Vec<Residue> = Vec::new();
        let mut unread: Vec<(usize, usize)> = Vec::new();
        let planes = crate::transducer::base::planes(n);
        let zero = [Q::zero(), Q::zero()];
        for (p, held) in planes.iter().zip(&self.upper) {
            // The plane's image, where it is a plane of the base: its upper
            // orders are held there and compared. One turned off every base
            // plane is carried as UNREAD by name, never passed as checked.
            let Some(carriage) = PlaneCarriage::of(rotation, rho, n, *p) else {
                unread.push(*p);
                continue;
            };
            let Some(theirs) = crate::transducer::base::plane_index(carriage.image).and_then(|ix| other.upper.get(ix))
            else {
                unread.push(*p);
                continue;
            };
            for (order, (mine, yours)) in [Harmonic::Three, Harmonic::Four].into_iter().zip(held.iter().zip(theirs)) {
                // `h_b − Ad(h_a)`: the residue at its own scale, exact. A
                // harmonic that vanishes is the zero vector there.
                let c = match mine {
                    Some(h) => carriage.carry(h, order),
                    None => zero.clone(),
                };
                let hb = match yours {
                    Some(h) => h,
                    None => &zero,
                };
                let diff = [&hb[0] - &c[0], &hb[1] - &c[1]];
                if diff[0].is_zero() && diff[1].is_zero() {
                    continue;
                }
                if order <= through {
                    return Err(NotCarried::Order(order));
                }
                residue.push(Residue { plane: carriage.image, order, vector: diff });
            }
        }
        Ok(Covariance { residue, unread })
    }
}

/// `Ad⁽ᵏ⁾` ON ONE PLANE: how a transition — its rotation ray and its scale —
/// carries a harmonic of the base plane `p` onto its image plane. The
/// rotation's sandwich turns `p`'s first axis to `w·ω` on the image plane
/// (the ray's weight `w`), a mirror conjugating first, and an order-`k`
/// harmonic goes to `(ρω)^k h`, the conjugate for a mirror — the graded
/// adjoint action, read by the sandwich and never by a matrix.
#[derive(Clone, Debug)]
pub struct PlaneCarriage {
    /// The base plane `p` is carried onto.
    pub image: (usize, usize),
    omega: [Q; 2],
    mirror: bool,
    w: Q,
}

impl PlaneCarriage {
    /// The carriage of plane `p` by `rotation` at scale `rho` on `n` base
    /// axes; `None` where the rotation has no weight or `p` is turned off
    /// every plane of the base.
    pub fn of(rotation: &Mv, rho: &Q, n: usize, p: (usize, usize)) -> Option<PlaneCarriage> {
        let masks: Vec<usize> = (1..=n).map(cga::base_axis).collect::<Option<Vec<usize>>>()?;
        let at = |y: &Mv, k: usize| -> Q { y.coeff(masks[k]).clone() };
        let w = cga::weight_of(rotation).filter(|w| !w.is_zero())?;
        let (yi, yj) = (rotation.twisted(&axis(p.0)?), rotation.twisted(&axis(p.1)?));
        let rows: Vec<usize> = (0..n).filter(|&k| !at(&yi, k).is_zero() || !at(&yj, k).is_zero()).collect();
        let &[k, l] = rows.as_slice() else { return None };
        crate::transducer::base::plane_index((k, l))?;
        let omega = [at(&yi, k) * rho, at(&yi, l) * rho];
        let mirror = at(&yi, k) * at(&yj, l) - at(&yi, l) * at(&yj, k) < Q::zero();
        Some(PlaneCarriage { image: (k, l), omega, mirror, w })
    }

    /// `Ad⁽ᵏ⁾(h) = (ρω)^k h / w^k`, `h` conjugated first under a mirror.
    pub fn carry(&self, h: &[Q; 2], order: Harmonic) -> [Q; 2] {
        let m = order.m().unsigned_abs();
        let start = match self.mirror {
            true => [h[0].clone(), -&h[1]],
            false => h.clone(),
        };
        let c = (0..m).fold(start, |z, _| times(&z, &self.omega));
        let wm = (0..m).fold(Q::one(), |acc, _| acc * &self.w);
        [&c[0] / &wm, &c[1] / &wm]
    }
}
