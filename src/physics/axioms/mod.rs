// 56 — THE AXIOM REGISTER: what this engine assumes, what it refuses to
// assume, and the reading that decides each (ruled 2026-09-22).
//
//! A.1–A.10 are `ipsal-autic`'s register, whose own numbering marks
//! A.1–A.4 as carried forward unchanged from the convergence-engine and
//! A.5–A.10 as its additions. They are NOT adopted wholesale. Three are
//! REFUSED, and the refusals matter more than the imports: each would
//! flatten something this engine measures, and each is refused here with
//! a test that says why rather than a paragraph that asserts it.
//!
//! An axiom that cannot fail a test is a coordinate standing for no
//! theorem (T21(iii)). Every entry below is either exercised or is
//! explicitly marked as a statement about work not yet built.

mod coxeter;
mod imported;
mod ours;
mod reciprocity;
mod refused;
mod routes;
mod tensegrity;

pub use coxeter::CX1_TREE_CONJUGACY;
pub use imported::{
    A10_CO_DIMENSIONALITY, A6_DIMENSIONAL_COMPLETENESS, A8_TORSIONAL_PERSISTENCE,
    A9_METRIC_AUTONOMY,
};
pub use ours::{
    A11_HARMONIC_MULTIPOLE_COMPLETENESS, A12_IRREDUCIBLE_INVERSIVE_EXTENT,
    A13_INVERSIVE_MOMENTUM_DOCKETED, OK1_K_STEP_IS_ONE_ORBIT,
};
pub use reciprocity::{
    RX1_RECIPROCITY, RX2_EXACTNESS, RX3_FILTRATION, RX4_COMPOSITION, RX5_COHERENCE, RX6_SPLIT, RX7_COHESION,
};
pub use refused::{A4_POSITIVE_DEFINITE_REFUSED, A6_BIND_11_REFUSED, A7_ABELIAN_RETURN_REFUSED};
pub use tensegrity::{
    A10_1_III_STRAIN, A10_1_II_BALANCE, A10_1_INTRINSIC_AXIS, A10_1_IV_FORCE, A10_1_I_CO_PRESENCE, A10_1_METRIC_RIGIDITY, A10_1_ORBITAL_TENSEGRITY,
    A10_1_STAR_SETS_SPIN, A10_1_T1_RECIPROCITY, A10_1_T2_COPLANARITY, A10_1_T3_ANTI_MONOCULARITY, A10_1_TORQUE,
};
pub use routes::{
    C1_RESOLUTION, C2_CONVERGENCE, R1_CARRIER, R2_CONSTRUCTORS_BUILD, R3_LOCALITY, R4_CURVATURE_IS_HOLONOMY,
    R5_BASING, ROUTE_CONJECTURES, T1_TWO_ROUTES, T2_DISCRETE_STOKES,
};

#[cfg(test)]
mod tests;
