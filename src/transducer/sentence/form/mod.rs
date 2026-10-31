// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 18.2, A21, A20, 50.9 — THE SENTENCE FORM: its vertices, faces and orbits,
//! the tree gauge that flattens it, the structured keys it is indexed by,
//! the certificate a pivot on it writes, and the contraction a refusal
//! makes on it.

pub mod atlas;
pub mod axis;
pub mod carrier;
pub mod certificate;
pub mod clause;
pub mod face;
pub mod fibre;
pub mod closure;
pub mod contract;
pub mod index;
pub mod keys;
pub mod rule_element;
pub mod spectrum;
pub mod vertex;

pub(crate) use axis::*;
pub(crate) use vertex::demand_coordinates;
pub use carrier::*;
pub use certificate::*;
pub use fibre::*;
pub use contract::*;
pub use keys::*;
pub use spectrum::*;

use crate::transducer::sentence::chain::Link;
use crate::transducer::sentence::class::{Axis, ClassVector};
use crate::transducer::sentence::transport::{relative_gauge, FacePlace, Transport};
use crate::network::nerve::{coord, Coordinate, Keyed};
use crate::transducer::alphabet::{self, Rotor, Word};
use crate::transducer::element::{self, Versor};
use crate::transducer::peel::ShellKind;
use crate::transducer::stalk;
