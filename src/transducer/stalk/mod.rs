// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! M4, A6 — THE ATLAS OF THE BASE MANIFOLD: charts on M = P/G for the
//! bundle whose structure group is the rational conformal group. A word is
//! filed under exact scale-free coordinates; the members are addresses.

pub mod chart;
pub mod stalks;
pub mod stratum;

pub use chart::*;
pub use stalks::*;
pub(crate) use stratum::*;

/// A READ OF AN ATLAS (24.1): the two walks a thought takes over whatever
/// language a mind holds — the genome alone, or the working atlas of
/// three strata. Both are keyed reads: the budgeted children of one chart
/// and the members of one chart. Nothing is enumerated.
/// WHAT AN ATLAS IS ASKED, and it is one question: the addresses held on a
/// chart.
///
/// `children(inv, after, budget, f)` stood beside it — a budgeted page of
/// one stratum's siblings, resumed from a cursor — and its only caller was
/// `Bubble::expand_with`, which used it to walk outward from an idea's own
/// chart and meet whatever it found. That is adjacency in a trie, not
/// incidence in a sheaf: a Čech simplex is the overlap of two sections over
/// ONE chart, and an idea's charts are already named by its own vertices.
/// The expansion reads `members` at those coordinates now, so the walk, its
/// budget pacing, its resume cursor and this method are all gone together.
///
/// A reader that answers "what is held here" needs no cursor and no budget,
/// because it is a keyed read of one fibre and not a traversal of anything.
pub trait AtlasRead {
    /// The addresses held on `inv`.
    fn members(&self, inv: &Invariant) -> Vec<Vec<u8>>;
}

impl AtlasRead for Stalks {
    fn members(&self, inv: &Invariant) -> Vec<Vec<u8>> {
        self.fiber(inv)
    }
}

use crate::transducer::alphabet::Rotor;
use crate::transducer::element::{self, Versor};
use crate::network::nerve::Keyed;

#[cfg(test)]
mod tests;
