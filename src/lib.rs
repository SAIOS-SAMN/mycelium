//! mycelium v0.1.0 — the linguistics engine on crystallized forms: the
//! exact algebra and its transduction, the wire and the ledger, and the
//! server, client and court roles that speak over them.

// Shared: exact algebra, transduction, wire, physics, chronometry.
pub mod core;
pub mod transducer;
pub mod voice;
pub mod network;
mod snapshot;
pub mod physics;
pub mod economy;
pub mod arc;
pub mod anastome;
pub mod chronometry;
/// §8.1 — the readings the live fleet holds about itself (the observer).
pub mod observe;

// Component roles.
pub mod server;
pub mod client;
pub mod court;

// Path stability across the component split: the historical flat module
// paths (`mycelium::cognition`, `::sdk`, `::registry`, `::synthesis`,
// `::verdict`) resolve to their role homes.
pub use client::sdk;
pub use court::verdict;
pub use server::{cognition, registry, synthesis};
