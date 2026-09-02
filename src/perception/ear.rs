// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE EAR — the harmonics a body is heard on.
//!
//! The convergence engine heard on eleven harmonics: value spectrum,
//! spatial and geometric sensitivity, adjacency and boundary, and higher
//! torsion (D.SENSE.HEARING.1, the harmonic tuning). Here they are the
//! eleven scale-free coordinates of a body on the base manifold, exact in
//! ℚ and invariant under the whole conformal group: what a body sounds
//! like, whatever its size, place, or orientation. The atlas is stratified
//! by these axes, so hearing a body is reading its stratum.
//!
//! The ear hears a scene's bodies and a voice's carried bodies the same
//! way. A kernel's aperture ([`Tuning`]) is the running tuning of its
//! hearing: H_total = H_core + H_tuning, converging on what it has heard.

use crate::transducer::blade::{self, Blade};
use num_rational::BigRational;
use num_traits::Zero;

/// The kernel's aperture: eleven harmonic tunings as a running average of
/// what it has heard. Volatile by ruling — it re-forms each run.
pub use crate::client::sdk::Aperture as Tuning;

/// THE HARMONICS of a body, in axis order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Harmonics {
    /// h₀ — components (β₀).
    pub components: u64,
    /// h₁ — holes (β₁).
    pub holes: u64,
    /// h₂ — the boundary's turning count.
    pub turns: u64,
    /// h₃ — boundary length² over area (the isoperimetric ratio).
    pub isoperimetric: BigRational,
    /// h₄ — det(I)/tr(I)² (anisotropy of the inertia).
    pub anisotropy: BigRational,
    /// h₅ — tr(I)/n² (spread per area).
    pub spread: BigRational,
    /// h₆ — the isotropic fourth moment over n³.
    pub quartic: BigRational,
    /// h₇… — the hue fractions, sorted: the gauge spectrum.
    pub gauge: Vec<BigRational>,
}

impl Harmonics {
    /// Hear a body.
    pub fn of(b: &Blade) -> Harmonics {
        let (components, holes) = blade::topology(&b.cells);
        let n = b.shell.clone();
        let edges = BigRational::from_integer(num_bigint::BigInt::from(blade::boundary_length(&b.cells) as i64));
        let (ixx, iyy, ixy) = (&b.inertia[0], &b.inertia[1], &b.inertia[2]);
        let tr = ixx + iyy;
        let det = ixx * iyy - ixy * ixy;
        let mut gauge: Vec<BigRational> = b.gauge.iter().map(|(_, c)| c / &n).collect();
        gauge.sort();
        Harmonics {
            components,
            holes,
            turns: blade::turns(&b.cells) as u64,
            isoperimetric: &edges * &edges / &n,
            anisotropy: match tr.is_zero() {
                true => BigRational::zero(),
                false => &det / (&tr * &tr),
            },
            spread: &tr / (&n * &n),
            quartic: &b.quartic[0] / (&n * &n * &n),
            gauge,
        }
    }

    /// The harmonics laid out as the atlas's axes (each ratio as its
    /// numerator and denominator), in the order the strata refine by.
    pub fn axes(&self) -> Vec<u64> {
        let mut axes = vec![self.components, self.holes, self.turns];
        axes.extend(blade::axes_of(&self.isoperimetric));
        axes.extend(blade::axes_of(&self.anisotropy));
        axes.extend(blade::axes_of(&self.spread));
        axes.extend(blade::axes_of(&self.quartic));
        for f in &self.gauge {
            axes.extend(blade::axes_of(f));
        }
        axes
    }
}

/// HEAR a voice's carried body: lift it through the eye and read its
/// harmonics. None for an empty body.
pub fn hear(body: &[(i64, i64, u8, u8)]) -> Option<Harmonics> {
    let shape = crate::transducer::element::shape_of_chain(body)?;
    Some(Harmonics::of(&shape.blade))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A body sounds the same at every size, place, and orientation, and
    /// its harmonics are the atlas's axes.
    #[test]
    fn a_body_sounds_the_same_wherever_it_is() {
        let l: Vec<(i64, i64, u8, u8)> = vec![(0, 0, 1, 2), (1, 0, 1, 2), (2, 0, 1, 2), (2, 1, 1, 2)];
        let moved: Vec<(i64, i64, u8, u8)> = vec![(7, -3, 1, 2), (7, -2, 1, 2), (7, -1, 1, 2), (6, -1, 1, 2)];
        let (a, b) = (hear(&l).unwrap(), hear(&moved).unwrap());
        assert_eq!(a, b);
        assert_eq!(a.components, 1);
        assert_eq!(a.turns, 6);
        assert_eq!(a.axes(), crate::transducer::stalk::invariant(&l).0);
    }
}
