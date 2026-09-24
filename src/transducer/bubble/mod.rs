// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 13, 25 — THE CONTEMPLATION: the Čech complex over the thought bubbles a
//! mind holds. Vertices are bubbles; a k-simplex is a (k+1)-fold overlap
//! of their covers with shared remainder volume.


pub mod contemplation;
pub mod docket;
pub mod hold;
pub mod drive;
pub mod horizon;
pub mod idea;
pub mod meet;
pub mod merge;
pub mod shed;
pub mod transport;

pub use contemplation::Contemplation;
pub use idea::{Bubble, ClassRecord};
pub(crate) use horizon::horizon_put;
pub(crate) use idea::*;
pub(crate) use merge::*;
pub use meet::{Adjunct, Decoherence, MeetGraph, FILL_ARMS, MEET_OWED};
pub use transport::TransportTally;
pub use docket::DocketReading;

use crate::network::nerve::{coord, Keyed};
use crate::physics::Budget;
use crate::transducer::alphabet::{Order, Rotor};
use crate::transducer::cech::{self, CoboundaryOutcome, Complex, Fill, Section, Simplex};
use crate::transducer::element::{self, Element};
use crate::transducer::family::Crystal;
use crate::transducer::sentence::{self, Form, Link, PivotCertificate, Vertex};
use crate::transducer::stalk::{self, AtlasRead, Invariant, Standing};

#[cfg(test)]
mod tests;
