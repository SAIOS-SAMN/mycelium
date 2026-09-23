// 50.8, A6, 16.4 — CONTINUOUS SUPPORT CORRESPONDENCE: exact rational
// moment invariants, volume scaling, and infinite-order Pythagorean prime rotors.
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
//! moments in ℚ: volume/shell, centroid, and the harmonics themselves — the
//! deviatoric quadrupole `H₂` and the hexadecapole `H₄` as vectors, carried
//! whole. Under a rotor the harmonics are TURNED by it and compared, never
//! reduced to their squared norms first (SQ1-b4, Axiom C as tightened).

use crate::transducer::alphabet::{Rotor, SLOT_PLANES_SIX};
use crate::transducer::blade::body::Shape;
use num_rational::BigRational;
use num_traits::{One, Zero};

/// An exact continuous support representation of a body in ℚ³ with polynomial moment invariants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuousSupport {
    /// Exact volume/shell in ℚ.
    pub shell: BigRational,
    /// Exact centroid in ℚⁿ at the body's own `n`.
    pub centroid: Vec<BigRational>,
    /// Inertia tensor moments in the blade's order (`inertia_pairs(n)`:
    /// Ixx, Iyy, Ixy, Izz, Ixz, Iyz at `n = 3`).
    pub inertia: Vec<BigRational>,
    /// The deviatoric quadrupole `H₂ = (Ixx − Iyy, 2Ixy)` on the grid plane,
    /// carried whole: it turns at twice the body's angle.
    pub h2: [BigRational; 2],
    /// The hexadecapole `H₄` on the grid plane, carried whole: it turns at
    /// four times the body's angle.
    pub h4: [BigRational; 2],
    /// Scale-free anisotropy ratio det(I)/tr(I)² in ℚ (invariant under both rotation and dilation).
    pub anisotropy: BigRational,
    /// Spread per area tr(I)/n² in ℚ.
    pub spread: BigRational,
}

impl ContinuousSupport {
    /// Lift a Shape into its continuous support representation.
    pub fn of(shape: &Shape) -> Option<Self> {
        let shell = shape.blade.shell.clone();
        let centroid = shape.blade.centroid()?;
        let inertia = shape.blade.inertia.clone();

        // On the sensor's plane, named (BQ4): the support's correspondence
        // is the planar reading its callers compare.
        let plane = crate::transducer::base::GRID_PLANE;
        // The two harmonics, carried as the vectors they are (SQ1-b4).
        let h2 = shape.blade.harmonic_on_plane(plane, false)?;
        let h4 = shape.blade.harmonic4_on(plane, false)?;

        let (ixx, iyy, ixy) = (&inertia[0], &inertia[1], &inertia[2]);
        let tr = ixx + iyy;
        let det = ixx * iyy - ixy * ixy;
        let anisotropy = if tr.is_zero() {
            BigRational::zero()
        } else {
            &det / (&tr * &tr)
        };
        let spread = if shell.is_zero() {
            BigRational::zero()
        } else {
            &tr / (&shell * &shell)
        };

        Some(ContinuousSupport {
            shell,
            centroid,
            inertia,
            h2,
            h4,
            anisotropy,
            spread,
        })
    }

    /// Exact similarity correspondence under scaling factor ρ in ℚ.
    pub fn is_similar_to(&self, other: &Self, rho: &BigRational) -> bool {
        let rho2 = rho * rho;
        // Volume scales as ρ² in 2D
        if other.shell != &self.shell * &rho2 {
            return false;
        }

        // Scale-free anisotropy ratio det(I)/tr(I)² is invariant
        if other.anisotropy != self.anisotropy {
            return false;
        }

        true
    }

    /// DOES THE ROTOR CARRY THIS BODY'S HARMONICS ONTO THE OTHER'S (SQ1-b4).
    ///
    /// The rotor is IN HAND, so it is applied: its block on the grid plane is
    /// `B = w·Rot(θ)` with `det B = w²`, `H₂` turns at `2θ` and `H₄` at `4θ`,
    /// and the comparisons are exact with no weight divided and no norm:
    ///
    /// ```text
    ///     det(B)  · H₂(other) = B² · H₂(self)
    ///     det(B)² · H₄(other) = B⁴ · H₄(self)
    /// ```
    ///
    /// This compared `|H₂|²` and `|H₄|²`, the squared norms, with the rotor
    /// beside it unused — a turn by ANY angle passed, since a norm cannot see
    /// the angle. A rotor that moves the grid plane out of itself has no block
    /// there, and relates no harmonics read on it: refused.
    pub fn matches_rotor(&self, other: &Self, rotor: &Rotor) -> bool {
        // Isometry: shell must match
        if self.shell != other.shell {
            return false;
        }
        let grid = (crate::transducer::alphabet::PLANE_ROW, crate::transducer::alphabet::PLANE_COL);
        let Some(b) = crate::transducer::alphabet::rotor_block_on(rotor, grid) else {
            return false;
        };
        let apply = |m: &[[BigRational; 2]; 2], v: &[BigRational; 2]| -> [BigRational; 2] {
            [&m[0][0] * &v[0] + &m[0][1] * &v[1], &m[1][0] * &v[0] + &m[1][1] * &v[1]]
        };
        let det = &b[0][0] * &b[1][1] - &b[0][1] * &b[1][0];
        let scaled = |k: &BigRational, v: &[BigRational; 2]| [k * &v[0], k * &v[1]];
        let turn2 = |v: &[BigRational; 2]| apply(&b, &apply(&b, v));
        let turn4 = |v: &[BigRational; 2]| turn2(&turn2(v));
        if scaled(&det, &other.h2) != turn2(&self.h2) {
            return false;
        }
        if scaled(&(&det * &det), &other.h4) != turn4(&self.h4) {
            return false;
        }

        // Verify across turned planes that rotor block relates the harmonics
        for &plane in &SLOT_PLANES_SIX {
            let Some(block) = crate::transducer::alphabet::rotor_block_on(rotor, plane) else {
                continue;
            };
            let det = &block[0][0] * &block[1][1] - &block[0][1] * &block[1][0];
            let rho2 = &block[0][0] * &block[0][0] + &block[1][0] * &block[1][0];
            if rho2.is_zero() || !(det / rho2).is_one() {
                return false;
            }
        }
        true
    }
}
