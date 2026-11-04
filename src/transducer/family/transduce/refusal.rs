// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! WHY A TRANSDUCTION REFUSES, named (MU-DERIVATIONS D3.2 §F). A refusal
//! returns no record: a transduction is atomic by rollback of its result.
//! Each refusal keeps its meaning at every rank `n`.

use super::Turn;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransductionRefusal {
    /// An enclosure with no phase, or a phase of period 0.
    Empty,
    /// Two frames of different rank: a phase of one has no phase of the
    /// other to be read on, and none is padded or dropped (MU-SENSING §5).
    Rank { from: usize, to: usize },
    /// The stroke ended before the record did; `need` is the bound the two
    /// enclosures name ([`super::need`]), `None` where that bound passes
    /// `usize` and so no stroke names it.
    Spent { need: Option<usize> },
    /// An image arc would cross its enclosure's seam: nothing wraps (O5).
    Seam,
    /// A linear part outside the signed permutations of the `n` phases: a
    /// shear, a scale, or any map that moves the seam set.
    Oblique,
    /// A turn that does not land on a site boundary of the phase it is
    /// projected onto, named with that phase's period; never rounded.
    OffSite { turn: Turn, period: usize },
    /// A body the motion does not carry whole onto one body of the other
    /// frame: its image is split across, or merged with, other bodies (O7).
    /// `body` is the body's index in its frame.
    Mixed { body: usize },
    /// No factoring `extent = coarse · fine` with neither factor 1: the
    /// phase is not taken apart this way (MU-SENSING §3.6).
    Unfactored { extent: usize, coarse: usize, fine: usize },
    /// A speech whose sample is refused at writing, named by the sample's
    /// own refusal: a measure past the machine word.
    Sample(crate::transducer::sample::SampleRefusal),
    /// No signed permutation carries every body of one frame onto the
    /// other's: the pair's change stands as its record (rule 7), never
    /// dropped.
    Unenclosed,
    /// A motion stated onto an enclosure that is not the output's: its
    /// speech would stand in another frame than the one it is read against
    /// (MU-SENSING §3.9).
    OntoAnother,
    /// A residue link that does not stand on the motion's speech: a site
    /// outside its enclosure, or a symbol of another width.
    OffSpeech,
}
