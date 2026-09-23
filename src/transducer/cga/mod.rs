//! A4 — CONFORMAL GEOMETRIC ALGEBRA: the exact multivector algebra `Cl(4,1)`
//! over ℚ, the conformal model of the three-coordinate base `(r, c, k)`
//! (51.2), where a body is an invertible dual sphere and a rigid motion is one
//! versor sandwich, so the quotient between two scene states exists.

mod embedding;
pub mod generic;
mod group;
mod hyperboloid;
mod multivector;
mod period;
mod planes;
mod spin;

pub use embedding::{
    base, boundary_dual, e_inf, e_o, hyperbolic_point, on_sphere, point, rotator,
    rotator_on_blade, sphere, translator,
};
pub use group::{dilator, is_versor, projective_ratio, reflector};
pub use hyperboloid::{frame_base_point, geodesic_transvection, hyperbolic_lamina, hyperboloid_pole};
pub use multivector::{Mv, BLADES, E1, E2, E3, EM, EP, PSEUDO};
pub use period::{compact_period, finite_order, Spectrum, CYCLOTOMIC_ORDERS};
pub use planes::{
    note_transition, plane_is_harmonic, plane_name, plane_tally, take_plane_tally, PlaneTally,
    BIVECTOR_PLANES, HARMONIC_PLANES, TRANSVECTION_PLANES,
};
pub use spin::{
    bivector_is_simple, bivector_split, cayley, inverse_even, reflector_vector, transvection,
    transvection_along, transvection_commutator, versor_of_reflections,
};

#[cfg(test)]
mod tests;
