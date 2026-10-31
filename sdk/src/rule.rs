// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.142, 78.143 — THE RULE SEAM: a rule stated as typed steps and clauses,
// or read off a family, minted as the element the court replays.

//! The rule seam.
//!
//! Two routes reach the element a solve carries. A client that reads its
//! own rule states it as [`Step`]s — motions of `Cl(4,1)(ℚ)`, hue maps, and
//! the selections, regions, creations and crops of the clause language —
//! and [`sentence`] speaks them as the sentence the court judges. A client
//! that reads the family with the library's extractor calls
//! [`extract_rule`], mints the rule with [`mint_sentence_why`] and speaks it
//! on each test input with [`speak_rule`]. Extraction runs on a [`Budget`]
//! drawn from a [`Physics`] stroke.

/// One step of a rule, what a creation places, the six grid symmetries,
/// and why no rule stands.
pub use mycelium::sdk::rule::{Create, NoRule, Step, Symmetry};

/// Speak steps over a family as the sentence the court judges.
pub use mycelium::sdk::rule::sentence;

/// The clause language: selection predicates, regions, comparisons, orders,
/// ties, paints and stops, and why a clause is not stated on a pair.
pub use mycelium::sdk::rule::{
    About, Act, Clause, Comparison, Coord, Extent, Order, Paint, Pred, Region, Stop, Tie, Unstated, Value,
};

/// Read a rule off a family's train pairs, under a budget.
pub use mycelium::transducer::family::extract_rule;

/// Mint a rule as the element the court replays; the refusal named.
pub use mycelium::transducer::family::mint_sentence_why;

/// Speak a rule on one input; the speech's `grid()` is the output.
pub use mycelium::transducer::family::speak::{speak_rule, Speech};

/// The resolver of cited words a rule is minted and spoken against, and the
/// resolver that holds none.
pub use mycelium::transducer::family::{Crystal, NoCrystal};

/// One train pair: an input grid and its output grid.
pub use mycelium::transducer::family::TrainPair;

/// The rule an extraction states, and the mint's named refusal.
pub use mycelium::transducer::sentence::{ObstructionRefusal, SentenceRule};

/// The element a solve carries as its rule's tape (`Element::to_bytes`).
pub use mycelium::transducer::element::Element;

/// A grid of hues, row by row.
pub use mycelium::arc::Grid;

/// A multivector of `Cl(4,1)(ℚ)`: a motion step's versor.
pub use mycelium::transducer::cga::Mv;

/// The physics a budget is drawn from (`Physics::stroke_n`), and the budget
/// an extraction spends.
pub use mycelium::physics::{Budget, Physics};
