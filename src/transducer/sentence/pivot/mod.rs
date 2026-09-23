// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A16, A20 — THE PIVOT ORBIT: the candidates a form's occupied axes offer,
//! the trial that opens one axis, the discharge it certifies, and the orbits
//! and axis index the move is read and written on.

pub mod census;
pub mod drive;
pub mod moves;
pub mod orbits;
pub mod snap;
pub mod trajectory;
pub mod trial;
pub mod viscosity;
pub mod walk;
pub mod words;

pub use census::*;
pub use moves::*;
pub use snap::*;
pub use trajectory::{finish_circuits, take_circuits, walked_circuits, WalkedCircuit};
pub use viscosity::{
    bend_plane_name, read_viscosity, viscosity_line, viscosity_reading, Arm, BendLane, PairViscosity,
};
pub(crate) use walk::*;
pub use words::*;

use super::class::*;
use super::form::*;
use super::transport::*;
use crate::network::nerve::Keyed;
use crate::physics::Budget;
use crate::transducer::alphabet::{Order, Rotor, Word};
use crate::transducer::element::{self, Element};
use crate::transducer::family::Crystal;
use crate::transducer::stalk::{self, Standing};
