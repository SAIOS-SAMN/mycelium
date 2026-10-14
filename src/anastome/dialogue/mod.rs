// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 26 — THE DIALOGUE: the act table, the carriers every act shares, and one
//! codec per act family — percept, demand, quote, teach and settle, and the
//! acts that carry them. Every payload is exact geometry, never English.

pub mod acts;
pub mod atlas;
pub mod carrier;
pub mod challenge;
pub mod commitment;
pub mod crossing;
pub mod handoff;
pub mod cover;
pub mod demand;
pub mod docket;
pub mod equivocation;
pub mod fold;
pub mod offer;
pub mod percept;
pub mod present;
pub mod quote;
pub mod receipts;
pub mod root;
pub mod seat;
pub mod speech;
pub mod syllabus;
pub mod tube;
pub mod uplink;
pub mod domain;
pub mod endowment;
pub mod universe;
pub mod wallet;
pub mod why;

pub use acts::*;
pub use carrier::*;
pub use cover::*;
pub use demand::*;
pub use docket::*;
pub use equivocation::*;
pub use fold::*;
pub use offer::*;
pub use percept::*;
pub use present::*;
pub use quote::*;
pub use receipts::*;
pub use root::*;
pub use speech::*;
pub use syllabus::*;

use crate::transducer::address::{put_addr, take_addr, Address};

#[cfg(test)]
mod tests;
