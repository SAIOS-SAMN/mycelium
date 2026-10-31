// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 78.152 — THE TUNIVERSAL LAW: three keepers sign parent-linked readings of
//! the keepers' courts (TU-1, TU-2); a point is final on two keepers' verified
//! readings (TU-3); a disagreement is a residual on the keeper triangle,
//! never averaged (TU-4); the final point is written `origin/h:S:G:T:H:K`
//! (TU-5) and read by every act (TU-7).

pub mod keepers;
pub mod ops;
pub mod point;
pub mod reading;
pub mod record;
pub mod shelf;

#[cfg(test)]
mod tests;

pub use keepers::{Admitted, Edge, KeeperRefusal, Keepers, Residual};
pub use point::{Origin, TPoint};
pub use reading::{Cited, Tick, TickReading};
pub use record::{LastSigned, Recorded};
pub use shelf::{Seen, TShelf};
