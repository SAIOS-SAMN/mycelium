// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 18.2, A21, A20 — THE SENTENCE FORM: its vertices, faces and loops, the
//! tree gauge that flattens it, the structured keys it is indexed by, and
//! the certificate a pivot on it writes.

pub mod carrier;
pub mod certificate;
pub mod keys;

pub use carrier::*;
pub use certificate::*;
pub use keys::*;

use crate::transducer::sentence::chain::Cell;
use crate::transducer::sentence::class::{Axis, ClassVector};
use crate::transducer::sentence::transport::{relative_gauge, vertex_gauge, Transport};
use crate::network::nerve::{Keyed, ROOT};
use crate::transducer::alphabet::{self, Order, Rotor, Word};
use crate::transducer::element::{self, Versor};
use crate::transducer::peel::ShellKind;
use crate::transducer::stalk;
