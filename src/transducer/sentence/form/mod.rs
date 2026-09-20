// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 18.2, A21, A20, 50.9 — THE SENTENCE FORM: its vertices, faces and loops,
//! the tree gauge that flattens it, the structured keys it is indexed by,
//! the certificate a pivot on it writes, and the contraction a refusal
//! makes on it.

pub mod carrier;
pub mod certificate;
pub mod closure;
pub mod contract;
pub mod keys;
pub mod vertex;

pub use carrier::*;
pub use certificate::*;
pub use contract::*;
pub use keys::*;

use crate::transducer::sentence::chain::Link;
use crate::transducer::sentence::class::{Axis, ClassVector};
use crate::transducer::sentence::transport::{relative_gauge, FacePlace, Transport};
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::{self, Order, Rotor, Word};
use crate::transducer::element::{self, Versor};
use crate::transducer::peel::ShellKind;
use crate::transducer::stalk;
