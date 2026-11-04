// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 20, A6, 70.4 — THE VOLUMETRIC BLADE: a hue-blind connected component
//! lifted into Cl(3,1) as a non-null dual sphere carrying four orthogonal
//! strata, and the exact scale-free moduli read off its own moments.

pub mod body;
pub mod harmonics;
pub mod carry;
pub mod shells;
pub mod surgery;
pub mod continuous;
pub mod frame;
pub mod pose;
pub mod reciprocity;
pub mod tether;
pub mod tower;

pub use body::*;
pub use reciprocity::{reciprocal, NotReciprocal};
pub use carry::*;
pub use shells::*;
pub use surgery::*;
pub use continuous::*;
pub use frame::*;
pub use pose::*;
pub use tether::*;
pub use tower::*;

use crate::transducer::cga::{self, Mv};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;

#[cfg(test)]
mod tests;

/// THE BLADE READ ON THE SENSOR'S PLANE, named once for the tests (BQ4).
/// Every tower reader takes the plane it reads; the fixtures that read the
/// corpus's grid plane read through here, and everything else of the blade
/// is re-exported as it is.
#[cfg(test)]
pub mod sensor_plane {
    pub use super::*;
    use num_bigint::BigInt;
    use num_rational::BigRational;

    /// The sensor's plane of a fixture blade's base, as the default
    /// perspective assigns it (`perspective::family_plane`).
    fn p(b: &Blade) -> (usize, usize) {
        crate::transducer::perspective::family_plane(b.n()).expect("the fixture's base holds a plane")
    }

    pub fn relative_rotor(a: &Blade, b: &Blade, budget: &mut crate::physics::Budget) -> Result<Pose, NoPose> {
        super::relative_rotor(a, b, p(a), budget)
    }
    pub fn oblique_arc(a: &Blade, b: &Blade) -> Option<(BigInt, BigInt)> {
        super::oblique_arc(a, b, p(a))
    }
    pub fn deviatoric(b: &Blade) -> Option<[BigRational; 2]> {
        super::deviatoric(b, p(b))
    }
    pub fn quartic_harmonic(b: &Blade) -> Option<[BigRational; 2]> {
        super::quartic_harmonic(b, p(b))
    }
    pub fn third_harmonic(b: &Blade) -> Option<[BigRational; 2]> {
        super::third_harmonic(b, p(b))
    }
    pub fn chirality(b: &Blade) -> Option<(BigRational, BigRational)> {
        super::chirality(b, p(b))
    }
    pub fn dilation_certified(a: &Blade, b: &Blade) -> bool {
        super::dilation_certified(a, b, p(a))
    }
    pub fn dilation(a: &Blade, b: &Blade) -> Result<Dilation, (TowerRefusal, TowerRefusal)> {
        super::dilation(a, b, p(a))
    }
    pub fn chirality_on(b: &Blade, tower: Tower) -> Option<(BigRational, BigRational)> {
        super::chirality_on(b, p(b), tower)
    }
}
