// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE VOLUMETRIC BLADE — perception → graded Cl(3,1) body.
//!
//! A perceived scene segments into connected components; each lifts to a
//! polytonic BLADE: a non-null dual SPHERE (guaranteed invertible, so the
//! geometric quotient forms) carrying four ORTHOGONAL strata — spatial extent
//! (𝒮, a Cl(3,1) multivector), gauge hue (𝒢), temporal index (𝒯), shell
//! topology (𝒽). Ported from the validated mycelium transducer and RE-GROUNDED
//! for the Anti-Flattening Law: gauge and cells are `Vec`-of-relations, never
//! maps — this is cognition, so no `BTreeMap`/`BTreeSet` lives here.
//!
//! The shell is the body's own moments: count, centroid, the second moment
//! (inertia), and the fourth moment, each integrated over the unit cells as
//! a body, never summed over cell centers as points. Orientation is read
//! from the moment harmonics (the anisotropy of the inertia rotates at twice
//! the body's angle, the fourth harmonic at four times), so the transport
//! between two bodies is a quotient of their moments — no corner, no vertex
//! of the sensor's sample enters the algebra. The sample stays on the body
//! only as the support the transport is verified on.

//!
//! Four organs (Stage 8b S20, F2r3): `harmonics.rs` the eleven scale-free
//! coordinates a body is heard on (20, A6); `body.rs` a body lifted into the algebra
//! and the invariants read off it (16, 20); `carry.rs` the versor that
//! carries one shape onto another (16.4); `shells.rs` the outline, core,
//! ring and exact magnifications of a body (20).

pub mod body;
pub mod harmonics;
pub mod carry;
pub mod shells;

pub use body::*;
pub use carry::*;
pub use shells::*;

use crate::transducer::cga::{self, Mv};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;

#[cfg(test)]
mod tests;
