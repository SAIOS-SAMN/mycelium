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
pub mod pivot_fibre;
mod circuits;
pub mod sphere_pair;
mod lambda;
pub mod laws;
pub mod malleability;
mod memory;
pub mod metabolism;
pub mod outflow;
pub mod source;
mod stroke;
pub mod rate;
pub mod equilibrium;

#[cfg(test)]
mod tests;

pub use budget::{halt, halted, spent_here, Budget, Depleted, Reserve};
#[cfg(test)]
pub use budget::unhalt_for_test;
pub use calibration::{
    aperture_cycle, boot_stroke, calibrate, calibration_of, capacity, cores_of, expansion_unit, footprint_of, idle_park, minimal_stroke, sealed,
    spend_unit, park_floor, park_floor_of, Calibration, Sealed, Uncalibrated, FootprintUnread, STROKES_PER_RESERVE,
};
pub use memory::{cgroup_limit_of, memory_of, Memory, MemorySource};
pub use circuits::Circuits;
pub use lambda::{from_transport, persist_dwell, transport_deadline, Lambda};
pub use stroke::{Physics, WindingOutcome};
