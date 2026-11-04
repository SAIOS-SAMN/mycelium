// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 47.1, 47.2 — ADMISSION: the signed prefix, the order of checks, and the
//! reconstructed object every admission takes (MU-HOSTING §5a.10).

pub mod admit;
pub mod prefix;
pub mod reconstructed;

pub use admit::*;
pub use prefix::*;
pub use reconstructed::*;

#[cfg(test)]
mod tests;
