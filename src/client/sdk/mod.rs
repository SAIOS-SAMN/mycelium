// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 24 — THE KERNEL SDK: how a kernel takes a stable mesh identity, perceives
//! a scene of its own, speaks a sentence to a hypha, and learns the word the
//! court teaches back. Painted sites in, sentences out (law 8).

mod entity;
mod epigenome;
pub mod aperture;
pub mod chatter;
mod house;
pub mod kernel;
pub mod rule;
pub mod seat;
pub mod seat_mind;
pub mod session;
pub mod types;

pub use aperture::*;
pub use entity::body::Sit;
/// The names a court task's reading is coded by, for the observer's typed
/// reading (`observe::TaskSeen`).
pub(crate) use entity::perceive::{PERSPECTIVES, READINGS, STOPS};
pub use entity::winding::Wound;
pub use kernel::*;
pub use session::*;
pub use types::*;

use crate::anastome::dialogue;
use crate::arc::Grid;
use crate::network::wire::{Frame, Stratum};
use crate::transducer::element::Element;
use std::cell::RefCell;
use std::io::{self, Read, Write};
use std::net::TcpStream;

#[cfg(test)]
mod tests;
