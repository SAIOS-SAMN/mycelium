// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 24 — THE MIND SDK: how a mind takes a stable mesh identity, perceives
//! a scene of its own, speaks a sentence to a hypha, and learns the word the
//! court teaches back. Painted sites in, sentences out (law 8).

mod entity;
mod epigenome;
pub mod aperture;
pub mod chatter;
pub mod comprehension;
pub mod concluded;
pub mod genome;
mod house;
pub mod mind;
pub mod quote;
pub mod rule;
pub mod seat;
pub mod seat_mind;
pub mod session;
pub mod utterance;
pub mod voice;
pub mod working_atlas;

pub use aperture::*;
pub use comprehension::*;
pub use concluded::*;
pub use entity::body::Sitting;
/// The names a court task's reading is coded by, for the observer's typed
/// reading (`observe::TaskSeen`).
pub(crate) use entity::perceive::{PERSPECTIVES, READINGS, STOPS};
pub use entity::winding::Wound;
pub use genome::*;
pub use mind::*;
pub use quote::*;
pub use session::*;
pub use utterance::*;
pub use voice::*;
pub use working_atlas::*;

use crate::anastome::dialogue;
use crate::arc::Grid;
use crate::network::wire::{Frame, Stratum};
use crate::transducer::element::Element;
use std::cell::RefCell;
use std::io::{self, Read, Write};
use std::net::TcpStream;

#[cfg(test)]
mod tests;
