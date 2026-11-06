// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! LAW-FIELD v1 §4 (proposal 78.171) — WHY THE FIELD'S CARRIER REFUSES, by
//! name and with what was read: the law's five refusals and the carrier's own.

use super::atlas::Chart;
use algebra::SignatureRefusal;
use forms::coordinate::{FormRefusal, Key};
use forms::passage::{PassageRefusal, Pole};

/// WHAT A MOTION OR AN ANGLE LEFT UNHELD: the residue `NotExact` names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Residue {
    /// Two turnings on planes that share one unit and are not one plane:
    /// they do not commute, and their product turns on a third plane this
    /// carrier holds no exact angle for. The plane held and the plane
    /// arriving.
    Uncommuted { held: Key, arriving: Key },
    /// Two turnings on disjoint planes asked to compose as one turning:
    /// they compose only as a motion of two components.
    Apart { held: Key, arriving: Key },
    /// An angle whose field `ℚ(ζ_ℓ)` passes the machine word: the angle.
    PastWord { degrees: crate::core::Q },
}

/// WHY THE FIELD'S CARRIER REFUSES (LAW-FIELD v1 §4, and the carrier's own
/// refusals after them), each with the reading that caused it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldRefusal {
    /// A motion or angle the carrier cannot hold exactly; the residue is
    /// named.
    NotExact { residue: Residue },
    /// A path through a flat region of the field, every step the identity:
    /// no attractor binds it, so it is no orbit. The charts of the path (an
    /// attractor's centre alone where it binds no orbit).
    Unbound { charts: Vec<Chart> },
    /// A state that needs a plane's pole as infinity outside every
    /// enclosure: the pole the state stands at.
    Unenclosed { pole: Pole },
    /// A reading that would collapse a multivector to one number: an angle
    /// asked of a holonomy that turns on these planes, never one plane
    /// alone.
    ScalarCollapse { planes: Vec<Key> },
    /// Two values whose signatures disagree on a generator both hold (the
    /// algebra's refusal), never coerced.
    Mismatched(SignatureRefusal),
    /// A plane named by units that hold no turning, or no writing (the
    /// coordinate form's refusal).
    NotTurningPlane(FormRefusal),
    /// A frame plane past the field's frame planes, or past the algebra's
    /// masks: the plane's index.
    PastFrame { plane: u32 },
    /// A path whose transitions do not chain chart to chart and return to
    /// the chart it left in one revolution: the step at which it broke.
    NotRevolution { at: usize },
    /// A chart the atlas does not hold.
    NotCharted { chart: Chart },
    /// A frame state the pole basis refuses (the passage form's refusal).
    Passage(PassageRefusal),
    /// A statement that does not decode: the byte at which it stopped.
    Malformed { at: usize },
}
