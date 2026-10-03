//! A4, 71 — CONFORMAL GEOMETRIC ALGEBRA: the exact multivector algebra
//! `Cl(n+1, 1)` over ℚ, the conformal model of the base ℚⁿ — at the corpus's
//! `n = 3`, `Cl(4,1)` on `(r, c, k)` (51.2) — where a body is an invertible
//! dual sphere and a rigid motion is one versor sandwich, so the quotient
//! between two scene states exists. The product is written once for every
//! `n`, and a value carries the `n` it belongs to.

mod constructors;
mod dual;
mod embedding;
mod enclosure;
mod fixed;
mod flow;
mod gram;
mod group;
mod hyperboloid;
mod linear;
mod mirrors;
mod modes;
mod motion;
mod motor;
mod multivector;
mod period;
mod perturbation;
mod planes;
mod spin;

pub use constructors::{
    carry, construct, constructor_orbit, is_constructor, Constructed, ConstructorOrbit, NotConstructed, OrbitUnread,
};
pub(crate) use constructors::{constructors_of_base_turn, TurnUnspelled};
pub use dual::{contract_to_face, dual_frame, wedge, DualFrame};
pub use embedding::{
    base_at, base_vector, boundary_dual, e_inf, e_o, hyperbolic_point, on_sphere, point_at, rotator,
    rotator_on_blade, sphere_at, translator_at,
};
pub use enclosure::{centre_point, containment, enclosure, nesting, Containment};
pub use fixed::{centre_of, Centre, PoleAt};
pub use flow::{FlowKind, FlowPoint};
pub use gram::{gram_form, pair_regime, read_gram, signature, vector_rank, GramReading, PairRegime};
pub use group::{dilator, is_versor, projective_ratio, reflector, weight_of};
pub use hyperboloid::{frame_base_point, geodesic_transvection, hyperbolic_compartment, hyperboloid_pole};
pub use linear::{ray_of, LinearFunction};
pub use mirrors::{base_mirror, mirrors_between, sphere_centre, MirrorReading};
pub use modes::{mode_order, modes, Mode, Undecomposed};
pub use motion::{action_det, action_of, charged, motion_modes, period_of, unfold, MotionMode, Period};
pub use motor::{
    motor_tally, note_motor, note_step, note_unread, take_motor_tally, KindReading, Motor, MotorAction, MotorTally,
    StepClass, StepReading, TransitionKind, VectorReading,
};
pub use multivector::{base_axis, blades, pseudo, slots, Mv, E1, E2, E3, EM, EP};
pub use period::{compact_period, cyclotomic_orders, finite_order, Spectrum};
pub use perturbation::{lift_drag, Alignment, LiftDrag};
pub use planes::{
    bivector_planes, blade_name, harmonic_planes, is_transvection_plane, note_transition, occupancy,
    parabolic_name, plane_is_harmonic, plane_name, plane_tally, take_plane_tally, transvection_planes,
    Occupancy, PlaneTally, Pole,
};
pub use spin::{
    bivector_is_simple, bivector_split, cayley, cayley_inverse, inverse_even, reflector_vector, transvection,
    transvection_along, transvection_commutator, versor_of_reflections,
};

#[cfg(test)]
mod tests;
/// BQ3 — the algebra's oracle at `n = 3`: its readings, and the golden file's
/// writer the soak runs by flag (STD1).
pub mod oracle;
/// The corpus's `n = 3` named for the tests' fixtures (PH2, BQ4).
#[cfg(test)]
pub mod at_three;
