// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 24 — THE KERNEL SDK: how a kernel takes a stable mesh identity, perceives
//! a scene of its own, speaks a demand to a hypha, and learns the word the
//! court teaches back. Cells in, verdict out.

mod entity;
mod epigenome;
pub mod aperture;
pub mod kernel;
pub mod session;
pub mod types;

pub use aperture::*;
pub use kernel::*;
pub use session::*;
pub use types::*;

use crate::anastome::dialogue;
use crate::arc::Grid;
use crate::cognition::{encode_percept, perceive};
use crate::network::wire::{Frame, Stratum};
use crate::transducer::element::Element;
use std::cell::RefCell;
use std::io::{self, Read, Write};
use std::net::TcpStream;

#[cfg(test)]
mod tests;
