// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Law 5, 12.6 — THE PHYSICS: pressure is the per-lane backlog of unresolved
//! demand (𝒮, 𝒢, 𝒯, 𝒽) and the metabolism rate is derived per lane from it,
//! so no lane is traded against another. Torsion is what will not close.

// The four lanes a germ carries content on — 𝒮 (spatial), 𝒢 (gauge), 𝒯
// (temporal), 𝒽 (holonomic/shell) — are read off the germ's exact per-lane
// support by `Element::lanes()` (transducer/element.rs), not derived here.

pub mod actions;
pub mod axioms;
pub mod budget;
mod calibration;
pub mod capacitor;
mod circuits;
pub mod fascicle;
mod lambda;
pub mod laws;
pub mod malleability;
pub mod metabolism;
pub mod outflow;
pub mod source;
mod stroke;
pub mod tensegrity;

#[cfg(test)]
mod tests;

pub use budget::{halt, halted, spent_here, Budget, Depleted, Reserve};
pub use calibration::{aperture_cycle, expansion_unit, idle_park, reserve_max, spend_unit, stroke_floor, STROKES_PER_RESERVE};
pub use circuits::Circuits;
pub use lambda::{digest_ceiling, from_transport, persist_dwell, transport_deadline, Lambda};
pub use stroke::{Physics, StrokeOutcome};
