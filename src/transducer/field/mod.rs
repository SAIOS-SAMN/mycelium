// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 13, 25 — THE CONTEMPLATION: the Čech complex over the wells (held
//! thoughts) in a mind's field. Vertices are wells; a k-simplex is a
//! (k+1)-fold overlap of the wells' covers with shared remainder volume.


pub mod contemplation;
pub mod nerve_shape;
pub mod plan;
pub mod hold;
pub mod drive;
pub mod horizon;
pub mod well;
pub mod line;
pub mod meet;
pub mod merge;
pub mod pivot;
pub mod shed;
pub mod question;
pub mod cross_scene;
pub mod returned;
pub mod transport;

pub use contemplation::Contemplation;
pub use well::{Well, ClassRecord};
pub(crate) use horizon::horizon_put;
pub(crate) use well::*;
pub(crate) use merge::*;
pub use meet::{Adjunct, Decoherence, MeetGraph, FILL_ARMS, MEET_OWED};
pub use transport::TransportTally;
pub use plan::DocketReading;
pub use nerve_shape::{CoreReading, Shedding, ShedTally, Truncation};
pub use cross_scene::{CrossSceneTally, Unrelated};

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
