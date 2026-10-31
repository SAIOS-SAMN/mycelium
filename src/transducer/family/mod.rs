// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 20, 21, 52.5 — THE MOVES ON A FAMILY OF TRAIN PAIRS: the family's frames
//! perceived once, its one form read as a quotient per chart, and that
//! form spoken back onto a scene. A family it does not close is a withheld
//! form.

pub mod acts;
pub mod bridge;
mod carriage;
mod chord;
mod comprehend;
pub(crate) use comprehend::pivot_cells;
pub mod contact;
pub mod correspondence;
pub mod cover;
pub mod covered;
mod crystal;
pub mod cursor;
pub mod deflation;
pub mod derivation;
pub mod extract;
mod fibre;
mod fibre_relation;
mod frame;
pub mod torsion_quotient;
pub mod flux;
pub mod hyperfluid;
pub mod incidence;
pub mod finish;
pub mod confluence;
pub mod incongruence;
pub mod interpret;
pub mod invert;
mod keys;
pub mod local;
pub mod luminal;
pub mod memo;
pub mod orbit;
pub mod pair;
mod percept;
pub mod perceive;
pub mod pairing;
pub mod rebase;
pub mod reconstruct;
pub mod refine;
pub mod scene;
pub mod pieces;
pub mod span;
pub mod speak;
pub mod statement;
pub mod trace;
mod train;

#[cfg(test)]
mod tests;

pub(crate) use correspondence::*;
pub use deflation::*;
pub use derivation::*;
pub use finish::extraction_spend;
pub use carriage::{carriage, carriage_with_core, Carriage};
pub use crystal::{Crystal, NoCrystal};
pub use fibre_relation::{fibre_map, fibre_relation};
pub use frame::{family_ratio, frame_vertex};
pub(crate) use keys::{member_key, member_pair_key, read_member_key, read_member_pair_key, site_key};
#[cfg(test)]
pub use tests::common::cipher_form;
pub use train::{Family, TrainPair};
pub use extract::*;
pub(crate) use incidence::*;
pub use memo::*;
pub use orbit::*;
pub use pairing::*;
pub(crate) use rebase::*;
pub use perceive::*;
pub use scene::*;
pub use speak::*;

use crate::network::nerve::{coord, Coordinate};

use super::blade::{self, Shape};
use super::element::{self, Element, Versor};
use super::base::Site;
use super::sentence::{Form, Link, Vertex};
use super::stalk::Invariant;
