// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! CONTEMPLATION — the Čech complex over thought bubbles. Vertices are
//! bubbles; a k-cell is a (k+1)-fold overlap of bubble covers with shared
//! remainder volume. The same Cell and Complex types as germs-in-a-sentence
//! and hyphas-on-a-chart. Connections are computed on the link of one new
//! bubble through by_chart; bubbles are never enumerated.


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
use crate::transducer::stalk::{self, AtlasRead, Invariant, Standing};

#[cfg(test)]
mod tests;
