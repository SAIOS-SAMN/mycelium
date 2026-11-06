// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE TRANSDUCER (Parts I–IV) — what a scene becomes on the way to being a
//! word: blades lifted from compartments, versors that carry one shape onto
//! another, words, forms, and the charts they are filed under.

/// 71 — the base is ℚⁿ: a site and the count of its coordinates.
pub mod base;
/// MU-SENSING §2 — the sample: the sensor's only input, of any rank.
pub mod sample;
/// MU-SENSING §4 — the perspective: the roles of a sample's axes and of its
/// place in its family; no axis is privileged by the code.
pub mod perspective;
/// MU-SENSING §3.3 — the symbol set Σ: a family's distinct symbols, indexed
/// by first appearance in canonical address order, `e_∅` at `|Σ|`.
pub mod symbols;
/// MU-SENSING §2 — the rank-2 raster at intake: its ground, its painted
/// sites, its frame, and the bridge to the sample.
pub mod raster;
/// MU-SENSING §3.9, §5 — the sample the letters spell: a sample admitted
/// onto the alphabet's section, read by lines along its last axis.
pub mod spelled;
/// MU-SENSING §5; ruled 2026-11-06 on #32 (4) — past the alphabet's section,
/// a sample read through its section samples, one per plane and coordinate
/// of the rest; no letter is written for a larger rank.
pub mod sections;
/// 16.4 — the conformal model every transport is written in.
pub mod cga;
/// 52 — the Cayley chart: a versor's derivation is its bivector.
pub mod cayley;
/// 20, A6 — a body, its exact moduli, and its shells.
pub mod blade;
/// Chapter 7 — the residue as the exact spec of a missing operator.
pub mod germinate;
/// 18.3, 17.1, 17.2 — the word: four strata, a tape, an address.
pub mod element;
/// A17, T5 — that a torsion cycle closes: ∮ dX = 0 on the fibre.
pub mod torsion_witness;
/// Chapters 13, 14 — the Čech complex over held thoughts.
pub mod cech;
/// A9 — the selection term a family's evidence derives.
pub mod anchor;
/// A9 — candidate selections with the selections' exact invariant signatures.
pub mod candidates;
/// A6, A7, 17.4 — the chart a word is filed under.
pub mod stalk;
pub mod bytes;
/// 18.2, 12.6, A16, A20 — the form: orbits, the pivot, the certificate.
pub mod sentence;
/// Chapter 25 — the contemplation: what a mind holds at once.
pub mod field;
/// Chapters 20–22 — extraction and speech over a family of train pairs.
pub mod family;
/// Chapter 13 — reconciliation of two holders of the language, by stratum.
pub mod sheaf;
/// The shell-by-shell reading law: outer to inner, the residue riding in.
pub mod layers;
/// 1.1, 1.2, 16.1 — the seventeen letters and the rotor's canonical form.
pub mod alphabet;
/// A21, law 2 — structured keys: coordinates written out, never a digest.
pub mod address;
/// A12, M6 — comprehension by factorisation.
pub mod factor;
/// Chapter 27 — an optimisation spoken as a word.
pub mod rewrite;
/// MU-CLIENT §4.2 — a mind's own field, scoped to its tick.
pub mod mind;

#[cfg(test)]
mod tests;

pub use address::Address;
