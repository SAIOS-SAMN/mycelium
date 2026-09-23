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
//! moments in ℚ: volume/shell, centroid, deviatoric quadrupole Casimir invariant |H₂|²,
//! and hexadecapole Casimir invariant |H₄|². Under a conformal versor V = D(ρ)·R·T,
//! these invariants transform polynomially and are verified in closed form.

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
    /// Casimir invariant of the deviatoric quadrupole: |H₂|² = (Ixx - Iyy)² + 4 Ixy² in ℚ.
    pub h2_casimir: BigRational,
    /// Casimir invariant of the hexadecapole / spin-4 moment in ℚ.
    pub h4_casimir: BigRational,
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
        // Casimir invariant of H2: invariant under SO(2) rotation, scales as ρ⁸ under dilation.
        let h2 = shape.blade.harmonic_on_plane(plane, false)?;
        let h2_casimir = &h2[0] * &h2[0] + &h2[1] * &h2[1];

        // Casimir invariant of H4: scales as ρ¹⁶ under dilation.
        let h4 = shape.blade.harmonic4_on(plane, false)?;
        let h4_casimir = &h4[0] * &h4[0] + &h4[1] * &h4[1];

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
            h2_casimir,
            h4_casimir,
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

    /// Check if two continuous supports have matching orientation invariants
    /// under a Cl(4,0) Rotor across turned planes.
    pub fn matches_rotor(&self, other: &Self, rotor: &Rotor) -> bool {
        // Isometry: shell must match
        if self.shell != other.shell {
            return false;
        }
        if self.h2_casimir != other.h2_casimir {
            return false;
        }
        if self.h4_casimir != other.h4_casimir {
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
