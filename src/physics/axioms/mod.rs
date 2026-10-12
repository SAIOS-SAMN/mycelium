// 56, MU-RECURSION §0.0 — THE AXIOM REGISTER: the founding principles, what
// this engine assumes under each, what it refuses to assume, and the reading
// that decides each (ruled 2026-09-22; organised under the founding principles
// 2026-10-12).
//
//! THE ORDER IS DERIVATION. The founding principles (`founding.rs`) are
//! first: HARMONIC (a turn returns), LUMINAL (null flux has no length) and
//! HOLONOMIC (a loop's composite is its content), with their corollaries
//! (every dimension is an exponent; no constant) and the physics still
//! undefined, named. Every other entry is read from one of them:
//!
//! - harmonic — A.11 multipole completeness and A.12 its extent (`ours.rs`),
//!   CX1 the Coxeter element's class (`coxeter.rs`);
//! - luminal — F1–F5 the fluid (`fluid.rs`), A.10 co-dimensionality across
//!   a luminal interface (`imported.rs`);
//! - holonomic — A.6 dimensional completeness and A.8 torsional persistence
//!   (`imported.rs`), RX1–RX7 reciprocity (`reciprocity.rs`), the routes
//!   (`routes.rs`);
//! - the bodies' equilibrium — A.10.1 tensegrity and A.13 momentum
//!   (`tensegrity.rs`, `ours.rs`); A.9 metric autonomy, per mind;
//! - the ledger — OK1, `K` a node's cycles closed by name (`ours.rs`);
//! - refused — A.4, A.7, A.6-bind-11 (`refused.rs`), each with why.
//!
//! A.1–A.10 are `ipsal-autic`'s register, NOT adopted wholesale: three are
//! REFUSED, each because it would flatten something this engine measures.
//! An axiom that cannot fail a test is a coordinate standing for no theorem
//! (T21(iii)); every entry is exercised or marked as work not yet built.

mod coxeter;
mod fluid;
mod founding;
mod imported;
mod ours;
mod reciprocity;
mod refused;
mod routes;
mod tensegrity;

pub use founding::{
    DIMENSION_IS_EXPONENT, HARMONIC_RETURN, HOLONOMIC_CONTENT, LUMINAL_NULL_FLUX, NO_CONSTANT, OPEN_PHYSICS, ORBIT_IS_A_RETURN,
};
pub use coxeter::CX1_TREE_CONJUGACY;
pub use fluid::{F1_NULL_HAS_NO_LENGTH, F2_FLUX_BY_STOKES, F3_FROZEN_IN, F4_QUANTUM_READ, F5_NO_LATTICE_ON_THE_SPHERE};
pub use imported::{
    A10_CO_DIMENSIONALITY, A6_DIMENSIONAL_COMPLETENESS, A8_TORSIONAL_PERSISTENCE,
    A9_METRIC_AUTONOMY,
};
pub use ours::{
    A11_HARMONIC_MULTIPOLE_COMPLETENESS, A12_IRREDUCIBLE_INVERSIVE_EXTENT,
    A13_INVERSIVE_MOMENTUM_DOCKETED, OK1_K_COUNTS_CLOSED_CYCLES,
};
pub use reciprocity::{
    RX1_RECIPROCITY, RX2_EXACTNESS, RX3_FILTRATION, RX4_COMPOSITION, RX5_COHERENCE, RX6_SPLIT, RX7_COHESION,
};
pub use refused::{A4_POSITIVE_DEFINITE_REFUSED, A6_BIND_11_REFUSED, A7_ABELIAN_RETURN_REFUSED};
pub use tensegrity::{
    A10_1_III_STRAIN, A10_1_II_BALANCE, A10_1_INTRINSIC_AXIS, A10_1_IV_FORCE, A10_1_I_CO_PRESENCE, A10_1_METRIC_RIGIDITY, A10_1_ORBITAL_TENSEGRITY,
    A10_1_STAR_SETS_SPIN, A10_1_T1_RECIPROCITY, A10_1_T2_COPLANARITY, A10_1_T3_ANTI_MONOCULARITY, A10_1_TORQUE,
    A10_1_VILLARCEAU_SUCCESSION, A13_MOMENTUM_CARRIED, R2_HYPHAL_FASCICLES,
};
pub use routes::{
    C1_RESOLUTION, C2_CONVERGENCE, R1_CARRIER, R2_CONSTRUCTORS_BUILD, R3_LOCALITY, R4_CURVATURE_IS_HOLONOMY,
    R5_BASING, ROUTE_CONJECTURES, T1_TWO_ROUTES, T2_DISCRETE_STOKES,
};

#[cfg(test)]
mod tests;
