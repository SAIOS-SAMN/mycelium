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
pub mod tether;
pub mod tower;

pub use body::*;
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
    use crate::transducer::base::GRID_PLANE as P;
    use num_bigint::BigInt;
    use num_rational::BigRational;

    pub fn relative_rotor(a: &Blade, b: &Blade) -> Result<Pose, NoPose> {
        super::relative_rotor(a, b, P)
    }
    pub fn oblique_arc(a: &Blade, b: &Blade) -> Option<(BigInt, BigInt)> {
        super::oblique_arc(a, b, P)
    }
    pub fn deviatoric(b: &Blade) -> Option<[BigRational; 2]> {
        super::deviatoric(b, P)
    }
    pub fn quartic_harmonic(b: &Blade) -> Option<[BigRational; 2]> {
        super::quartic_harmonic(b, P)
    }
    pub fn third_harmonic(b: &Blade) -> Option<[BigRational; 2]> {
        super::third_harmonic(b, P)
    }
    pub fn chirality(b: &Blade) -> Option<(BigRational, BigRational)> {
        super::chirality(b, P)
    }
    pub fn dilation_certified(a: &Blade, b: &Blade) -> bool {
        super::dilation_certified(a, b, P)
    }
}
