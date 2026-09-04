
//! mycelium v0.1.0 — topological cognitive engine.
//!
//! Source is organized by component (docs/THREE_BINARIES.md): shared exact
//! algebra and wire at the top level; role-specific modules under
//! `server/`, `client/`, and `court/`. Wire acts are the only coupling
//! between roles.

// Shared: exact algebra, transduction, wire, physics, chronometry.
pub mod core;
pub mod transducer;
pub mod perception;
pub mod thought;
pub mod voice;
pub mod network;
mod snapshot;
pub mod ccl;
pub mod physics;
pub mod economy;
pub mod arc;
pub mod anastome;
pub mod torsion;
pub mod chronometry;
pub mod cohesion;

// Component roles.
pub mod server;
pub mod client;
pub mod court;

// Path stability across the component split: the historical flat module
// paths (`mycelium::cognition`, `::sdk`, `::registry`, `::synthesis`,
// `::verdict`) resolve to their role homes.
pub use client::sdk;
pub use client::thinking;
pub use court::verdict;
pub use server::{cognition, registry, synthesis};
