// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A16, A20 — THE PIVOT LOOP: the candidates a form's occupied axes offer,
//! the trial that opens one axis, the discharge it certifies, and the loops
//! and axis index the move is read and written on.

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
