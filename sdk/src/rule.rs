// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.142, 78.143 — THE RULE SEAM: a rule stated as typed steps and clauses,
// or read off a family, minted as the element the court reconstructs.

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

/// One step of a rule, what a creation places, the six plane symmetries,
/// and why no rule stands.
pub use mycelium::sdk::rule::{Create, NoRule, Step, Symmetry};

/// Speak steps over a family as the sentence the court judges.
pub use mycelium::sdk::rule::sentence;

/// The clause language: selection predicates, the axis of the perspective's
/// plane a coordinate is read along, regions, comparisons, orders, ties,
/// paints and stops, and why a clause is not stated on a pair.
pub use mycelium::sdk::rule::{
    About, Act, Clause, Comparison, Coord, Extent, Order, Paint, PlaneAxis, Pred, RegionKind, Stop, Tie, Unstated, Value,
};

/// Read a rule off a family's train pairs, under a budget.
pub use mycelium::transducer::family::extract_rule;

/// Perceive a family's train pairs once: the frames an extraction reads.
pub use mycelium::transducer::family::{perceive_family, Frames};

/// The stroke an extraction needs, read off the family and its frames (the
/// budget is sized by the object, never by a chosen number; 78.111).
pub use mycelium::transducer::family::extraction_spend;

/// Mint a rule as the element the court reconstructs; the refusal named.
pub use mycelium::transducer::family::mint_sentence_why;

/// Speak a rule on one input; the speech's `sample()` is the output.
pub use mycelium::transducer::family::speak::{speak_rule, Speech};

/// The resolver of cited words a rule is minted and spoken against, and the
/// resolver that holds none.
pub use mycelium::transducer::family::{Crystal, NoCrystal};

/// One train pair: an input sample and its output sample, each on the
/// alphabet's section.
pub use mycelium::transducer::family::TrainPair;

/// A sample on the alphabet's section, the carrier a rule reads and speaks
/// (track 26 S4d); a source's lines enter it through `of_lines`.
pub use mycelium::transducer::spelled::SectionSample;

/// THE GROUND (MU-SENSING §3.4): a family's deduced reading (`One`,
/// `Vortex`, `None`). A rule is read, minted and spoken once per
/// perspective of it (`on_each_perspective_of`), and each claim states the
/// ground it was spoken under (`stated`, `task::encode_solve_spoken`); the
/// court reconstructs under it.
pub use mycelium::transducer::family::transduce::Ground;
pub use mycelium::transducer::raster::{ground_of, on_each_perspective_of, stated};

/// THE SYMBOL SET (MU-SENSING §3.3): Σ is over all of a family's posted
/// samples, its test inputs included. A rule read with the test inputs
/// posted (`on_test_inputs`) maps every symbol of the whole family.
pub use mycelium::transducer::symbols::{on_test_inputs, SymbolSet};

/// The rule an extraction states, and the mint's named refusal.
pub use mycelium::transducer::sentence::{ObstructionRefusal, SentenceRule};

/// The element a solve carries as its rule's tape (`Element::to_bytes`).
pub use mycelium::transducer::element::Element;

/// A multivector of `Cl(4,1)(ℚ)`: a motion step's versor.
pub use mycelium::transducer::cga::Mv;

/// The physics a budget is drawn from (`Physics::stroke_n`), and the budget
/// an extraction spends.
pub use mycelium::physics::{Budget, Physics};
