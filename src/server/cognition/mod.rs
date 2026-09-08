// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 23.2, 47.3 — COGNITION, the kernel loop: a demand (scene A → scene B)
//! enters as raw cells, the percept is a pair form, a closed form is the
//! word, and a novel word installs only on two-organ CONVERGENCE.

mod basis;
mod book;
mod cocycle;
mod court;
mod germinate;
mod identity;
mod kernel;
mod keys;
mod language;
mod market;
mod reduce;
mod snapshot;
mod wire;

pub use crate::transducer::sentence::{cited_pattern, replay, verify, Cite};
pub use germinate::attest::Uttered;
pub use kernel::Kernel;
pub(crate) use keys::*;
pub(crate) use cocycle::CocycleRow;
pub(crate) use reduce::*;
pub use language::{
    Adoption, CriticalPair, FieldEntry, Glue, Language, ListingField, MarketView, NormalForm,
    Outcome, Priced, Rank, Reduction,
};
pub use reduce::{
    cohesion_over, normal_form, percept_elements, quote_incidence, rank_over, reduce_all,
    reduce_germ, reduce_proven, resolve_cites, taught_anchored_words, Ask, BookView,
};
pub use wire::{
    decode_demand, decode_percept, encode_demand, encode_percept, frame_of, germ_address, Demand,
    germ_bytes, outcome_code, perceive, Percept,
};

use crate::transducer::address::{put_addr, take_addr, Address};
use crate::transducer::blade::components;
use crate::transducer::element::{self, Element};

#[cfg(test)]
mod tests;
