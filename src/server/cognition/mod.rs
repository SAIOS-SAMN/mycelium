// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! COGNITION — the kernel loop. A demand (scene A → scene B) enters as raw
//! cells; the percept is a pair form (`pair_form` → `extract_rule_with`) on
//! the court as on the mind. A closed form is the word; a withheld remainder
//! is the residual. A novel word installs ONLY on two-organ CONVERGENCE
//! (`court::convergence`): one vantage dockets the residual, and a witness
//! that re-derives the same address from the same demand installs it.
//! Convergence is not a count of mouths -- the same voice twice converges
//! on nothing. No digest-keyed cognition (Anti-Flattening Law).

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

use crate::transducer::a21::{put_addr, take_addr, Address};
use crate::transducer::blade::components;
use crate::transducer::element::{self, Element};

#[cfg(test)]
mod tests;
