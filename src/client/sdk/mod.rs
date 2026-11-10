// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 24 — THE MIND SDK: how a mind takes a stable mesh identity, perceives
//! a scene of its own, speaks a sentence to a hypha, and learns the word the
//! court teaches back. Painted sites in, sentences out (law 8).

mod vocabulary;
pub mod aperture;
pub mod chatter;
pub mod comprehension;
pub mod concluded;
pub mod genome;
mod house;
pub mod hypha_acts;
mod retrieval;
pub mod mind;
pub mod quote;
pub mod rule;
pub mod seat;
pub mod seat_mind;
pub mod session;
pub mod signed_receipts;
pub mod assertion;
pub mod voice;
pub mod working_atlas;

pub use aperture::*;
pub use comprehension::*;
pub use concluded::*;
pub use mind::seating::Sitting;
pub use mind::boot::GenomeReading;
/// The names a court task's reading is coded by, for the observer's typed
/// reading (`observe::TaskSeen`).
pub(crate) use mind::perceive::{PERSPECTIVES, READINGS, STOPS};
pub use mind::winding::Wound;
pub use genome::*;
// The mind's items by name: its sub-modules stay inside the SDK and are
// not re-exported with them.
pub use mind::{Mind, Owed, Pending};
pub(crate) use mind::HeldSaying;
pub use quote::*;
pub use session::*;
pub use signed_receipts::{Holding, SignedReceipts};
pub use assertion::*;
pub use voice::*;
pub use working_atlas::*;

use crate::anastome::dialogue;
use crate::transducer::family::SectionSample;
use crate::network::wire::{Lane, Delimited};
use crate::transducer::element::Element;
use std::cell::RefCell;
use std::io::{self, Read, Write};
use crate::network::channel::Channel;

#[cfg(test)]
mod tests;
