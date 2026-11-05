// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 47.1, 47.2 — ADMISSION: the signed prefix, the order of checks, the
//! reconstructed object every admission takes, and the digests typed by
//! sense (MU-HOSTING §5a.10); and the boundary's refusals at the universal
//! core tube (MU-HOSTING §5a.5).

pub mod admit;
pub mod boundary;
pub mod digest;
pub mod prefix;
pub mod reconstructed;

pub use admit::*;
pub use boundary::*;
pub use digest::*;
pub use prefix::*;
pub use reconstructed::*;

#[cfg(test)]
mod tests;
