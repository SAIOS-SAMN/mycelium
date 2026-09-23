//! A4, 71 — CONFORMAL GEOMETRIC ALGEBRA: the exact multivector algebra
//! `Cl(n+1, 1)` over ℚ, the conformal model of the base ℚⁿ — at the corpus's
//! `n = 3`, `Cl(4,1)` on `(r, c, k)` (51.2) — where a body is an invertible
//! dual sphere and a rigid motion is one versor sandwich, so the quotient
//! between two scene states exists. The product is written once for every
//! `n`, and a value carries the `n` it belongs to.

mod embedding;
mod group;
mod hyperboloid;
mod multivector;
mod period;
mod planes;
mod spin;

pub use embedding::{
    base_at, base_vector, boundary_dual, e_inf, e_o, hyperbolic_point, on_sphere, point_at, rotator,
    rotator_on_blade, sphere_at, translator_at,
};
pub use group::{dilator, is_versor, projective_ratio, reflector};
pub use hyperboloid::{frame_base_point, geodesic_transvection, hyperbolic_lamina, hyperboloid_pole};
pub use multivector::{base_axis, blades, pseudo, slots, Mv, E1, E2, E3, EM, EP};
pub use period::{compact_period, cyclotomic_orders, finite_order, Spectrum};
pub use planes::{
    bivector_planes, harmonic_planes, is_transvection_plane, note_transition, plane_is_harmonic,
    plane_name, plane_tally, take_plane_tally, transvection_planes, PlaneTally,
};
pub use spin::{
    bivector_is_simple, bivector_split, cayley, inverse_even, reflector_vector, transvection,
    transvection_along, transvection_commutator, versor_of_reflections,
};

#[cfg(test)]
mod tests;
/// The corpus's `n = 3` named for the tests' fixtures (PH2, BQ4).
#[cfg(test)]
pub mod at_three;
