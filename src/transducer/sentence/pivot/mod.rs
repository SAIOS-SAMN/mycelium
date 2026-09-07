// AI-GENERATED CONTENT — see DISCLAIMER.md.

//! Pivot loop, class vector, and rank.

//!
//! Four organs (Stage 8b S24): `snap.rs` what a pivot produces and the
//! discharge it certifies (A16, A20); `drive.rs` the pivot loop and its
//! candidates (A16); `loops.rs` the loops of a form and the faces a pivot
//! writes (12.2, 18.2); `words.rs` the axis index (18.2, A21).

pub mod drive;
pub mod loops;
pub mod snap;
pub mod words;

pub use loops::*;
pub use snap::*;
pub use words::*;

use super::class::*;
use super::form::*;
use super::tape::from_element_form;
use super::transport::*;
use crate::network::nerve::{Keyed, ROOT};
use crate::physics::Budget;
use crate::transducer::alphabet::{Order, Rotor, Word};
use crate::transducer::element::{self, Element};
use crate::transducer::family::Crystal;
use crate::transducer::stalk::{self, Standing};
