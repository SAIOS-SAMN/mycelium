// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE KERNEL SDK — how an external kernel joins the mesh and DEVELOPS on it.
//! Sodalitum, netstratum, or any newcomer links this to: take a stable mesh
//! identity, PERCEIVE a scene pair (segment it — perception is the kernel's, never
//! the hypha's), SUBMIT the A→B demand to a hypha, receive the judge's VERDICT, and
//! LEARN the taught ANASTOME element so no dialect drifts from the mesh. No engine
//! internals leak across this seam: cells in, verdict out. Ported from the old
//! mycelium SDK (Session / kernel emblem / observe) onto the v0.1.0 percept wire.

//!
//! Five organs (Stage 8b S17): `kernel.rs` the voice this client speaks as;
//! `session.rs` its contact with the mesh (6.2, 24.3); `aperture.rs` how
//! open that contact is (24.1); `types.rs` the shapes it reads and answers
//! with; and `entity/` and `epigenome/` beside them.

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
