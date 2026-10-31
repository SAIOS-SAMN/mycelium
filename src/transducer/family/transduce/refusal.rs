// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! WHY A TRANSDUCTION REFUSES, named (MU-DERIVATIONS D3.2 §F). A refusal
//! returns no record: a transduction is atomic by rollback of its result.

use super::Turn;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransductionRefusal {
    /// A frame with no rows or no columns has no enclosure.
    Empty,
    /// A frame whose rows disagree in length is not the extent it claims.
    Ragged,
    /// The stroke ended before the record did; `need` is the bound the two
    /// enclosures name ([`super::need`]), `None` where that bound passes
    /// `usize` and so no stroke names it.
    Spent { need: Option<usize> },
    /// An image arc would cross its enclosure's seam: nothing wraps (O5).
    Seam,
    /// A linear part outside the eight signed swaps of the two circles: a
    /// shear, or any map that moves the seam set.
    Oblique,
    /// A turn that does not land on a site boundary of the circle it is
    /// projected onto, named with that circle's period; never rounded.
    OffSite { turn: Turn, period: usize },
    /// A body the motion does not carry whole onto one body of the other
    /// frame: its image is split across, or merged with, other bodies (O7).
    /// `body` is the body's index in its frame.
    Mixed { body: usize },
    /// No signed swap carries every body of one frame onto the other's: the
    /// pair's change stands as its record (rule 7), never dropped.
    Unenclosed,
}
