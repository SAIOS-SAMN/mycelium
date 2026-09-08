// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 26 — THE DIALOGUE: the act table, the carriers every act shares, and one
//! codec per act family — percept, demand, quote, teach and settle, and the
//! acts that carry them. Every payload is exact geometry, never English.

pub mod acts;
pub mod carrier;
pub mod cover;
pub mod demand;
pub mod docket;
pub mod equivocation;
pub mod offer;
pub mod percept;
pub mod quote;
pub mod receipts;
pub mod speech;
pub mod syllabus;

pub use acts::*;
pub use carrier::*;
pub use cover::*;
pub use demand::*;
pub use docket::*;
pub use equivocation::*;
pub use offer::*;
pub use percept::*;
pub use quote::*;
pub use receipts::*;
pub use speech::*;
pub use syllabus::*;

use crate::transducer::address::{put_addr, take_addr, Address};

#[cfg(test)]
mod tests;
