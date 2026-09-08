// AI-GENERATED CONTENT — see DISCLAIMER.md.

//! Sentence form: vertices, faces, loops, and the tree gauge.

//!
//! Three organs (Stage 8b S28): `carrier.rs` the form's vertices, faces and
//! loops (18.2); `keys.rs` the A21 keys it is indexed by; `certificate.rs`
//! the pivot certificate (A20).

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
