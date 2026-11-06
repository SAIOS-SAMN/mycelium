// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! LAW-FIELD v1 (proposal 78.171, amended a–e), ROADMAP track 26 S4a and S7
//! stage 1 — THE FIELD'S CARRIER: a sample lifted into charts of an atlas,
//! transitions as its connection, curvature as holonomy per lap, stated whole.
//!
//! Built against the proposal as proposed; mycelium amends it here if the
//! joint reading changes the law. The six joint-read fixtures stand as
//! claims in `tests/law_field.rs`.

mod atlas;
mod cyclotomic;
mod enclosing;
mod holonomy;
mod lift;
mod refusal;
mod statement;
mod turning;

pub use atlas::{Atlas, Attractor, Chart, Curvature, Orbit, Transition};
pub use cyclotomic::Cyclotomic;
pub use enclosing::{at_pole, stand, Labels, Membrane};
pub use holonomy::{Holonomy, Monodromy, Motion};
pub use lift::{Lifted, LiftedBody, Nested, Point};
pub use refusal::{FieldRefusal, Residue};
pub use statement::{admit, reconstruct, StatementRefusal};
pub use turning::{FramePlanes, Meeting, Turning, TurningPlane};

#[cfg(test)]
mod tests;
