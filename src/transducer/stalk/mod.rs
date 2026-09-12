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
pub trait AtlasRead {
    /// Budgeted one-level children of `inv`, resumed from `after`; the last
    /// child key visited is the cursor returned.
    fn children(
        &self,
        inv: &Invariant,
        after: Option<&[u8]>,
        budget: &mut Budget,
        f: &mut dyn FnMut(Invariant),
    ) -> Option<Vec<u8>>;
    /// The addresses held on `inv`.
    fn members(&self, inv: &Invariant) -> Vec<Vec<u8>>;
}

impl AtlasRead for Stalks {
    fn children(
        &self,
        inv: &Invariant,
        after: Option<&[u8]>,
        budget: &mut Budget,
        f: &mut dyn FnMut(Invariant),
    ) -> Option<Vec<u8>> {
        self.each_child(inv, after, budget, f)
    }

    fn members(&self, inv: &Invariant) -> Vec<Vec<u8>> {
        self.fiber(inv)
    }
}

use crate::transducer::alphabet::Rotor;
use crate::transducer::element::{self, Versor};
use crate::network::nerve::Keyed;
use crate::physics::Budget;

#[cfg(test)]
mod tests;
