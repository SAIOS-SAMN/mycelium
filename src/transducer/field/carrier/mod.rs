// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! LAW-FIELD v1 (proposal 78.171, amended a–e), ROADMAP track 26 S4a and S7
//! stages 1 and 2 — THE FIELD'S CARRIER: a sample lifted into charts of an
//! atlas, transitions as its connection, every path a geodesic of its
//! curvature, an orbit bound by an attractor read by its holonomy per
//! revolution, a well's body as a held attractor, each stated whole.
//!
//! Built against the proposal as proposed; mycelium amends it here if the
//! joint reading changes the law. The six joint-read fixtures stand as
//! claims in `tests/law_field.rs`.

mod atlas;
mod cyclotomic;
mod enclosing;
mod holding;
mod holonomy;
mod lift;
mod refusal;
mod statement;
mod turning;

pub use atlas::{Atlas, Attractor, Chart, Curvature, Orbit, Precession, Transition};
pub use cyclotomic::Cyclotomic;
pub use enclosing::{at_pole, stand, Labels, Membrane};
pub use holding::{ExpansionRing, Inflation, WellBody, WellLayer, WellStage};
pub use holonomy::{Holonomy, Monodromy, Motion};
pub use lift::{Lifted, LiftedBody, Nested, Point};
pub use refusal::{FieldRefusal, Residue};
pub use statement::{admit, admit_well, reconstruct, reconstruct_well, StatementRefusal};
pub use turning::{FramePlanes, Meeting, Turning, TurningPlane};

#[cfg(test)]
mod tests;
