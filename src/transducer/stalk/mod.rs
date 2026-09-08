// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE ATLAS OF THE BASE MANIFOLD — charts on M = P/G for the bundle
//! P → M whose structure group is the rational conformal group
//! Spin(4,1)(ℚ).
//!
//! The primary object is the principal bundle: the total space P is the
//! observed cochains, the base M is their conformal orbit classes, and the
//! fibre over a class is the group itself — every placement, rotation,
//! reflection, and dilation is a point on the fibre above its class,
//! connected to the base representative by a versor V. Congruence is the
//! exact statement π(Q₁) = π(Q₂) ⟺ ∃V: Q₂ = V Q₁ Ṽ, and
//! [`super::element::transition`] solves that V in closed form.
//!
//! This module is only the atlas on M. A residual word is charted by its
//! support's MODULI (`blade::moduli`): components and holes, the boundary's
//! turning count, boundary length over area, anisotropy, spread, the fourth
//! moment, the hue fractions — exact scale-free rationals invariant under
//! the whole group, computed with no group enumeration. A CONNECTION has no
//! support: it is charted by the conjugacy class of its linear block and
//! the cycle type of its gauge permutation, on the element itself. The
//! moduli are coordinates, never a classifier: they name the stratum (an
//! empty stratum is immediate novelty), and the versor decides and is
//! returned, never discarded. A stratum holding two words a versor relates
//! to the same point is the measurement that the coordinate set is too
//! coarse: an axis to extend, never a first-wins pick. Members are
//! ADDRESSES — the atlas is a function of the words held, never of the
//! order they were held in — reached through a keyed trie, no map, no
//! sorted tape, no sibling scan.

//!
//! Three organs (Stage 8b S25): `chart.rs` the invariant coordinates a word
//! is filed under (A6, A7); `stratum.rs` one level of the chart trie and
//! its refinement (A6); `stalks.rs` every word filed by chart with its
//! standing (A6, 23.4).

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
use crate::network::nerve::{Keyed, ROOT};
use crate::physics::Budget;

#[cfg(test)]
mod tests;
