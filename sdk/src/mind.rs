// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 24 — THE REFERENCE MIND: a voice with its genome, conversing with its
// court through its own carriers.

//! The reference mind.
//!
//! A client that runs the library's own mind rather than its own opens one
//! with [`Mind::open`] (a fresh genome) or `Mind::open_persistent` (the
//! genome restored from a file, with what the file came to answered as a
//! [`GenomeReading`]).

/// The reference mind: a voice with its genome and epigenome.
pub use mycelium::sdk::Mind;

/// What a genome file came to when a mind was opened on it.
pub use mycelium::sdk::GenomeReading;
