// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 13, 25 — THE CONTEMPLATION: the Čech complex over the thought bubbles a
//! mind holds. Vertices are bubbles; a k-cell is a (k+1)-fold overlap of
//! their covers with shared remainder volume.


pub mod contemplation;
pub mod drive;
pub mod horizon;
pub mod idea;
pub mod merge;
pub mod shed;
pub mod tape;

pub use contemplation::Contemplation;
pub use idea::{Bubble, ClassRecord};
pub(crate) use horizon::horizon_put;
pub(crate) use idea::*;
pub(crate) use merge::*;

use crate::network::nerve::{Keyed, ROOT};
use crate::physics::Budget;
use crate::transducer::alphabet::{Order, Rotor};
use crate::transducer::cech::{self, Cell as CechCell, Complex, Fill, Section};
use crate::transducer::element::{self, Element};
use crate::transducer::family::Crystal;
use crate::transducer::sentence::{self, Form, PivotCertificate, Vertex};
use crate::transducer::stalk::{self, Invariant, Standing, Stalks};

#[cfg(test)]
mod tests;
