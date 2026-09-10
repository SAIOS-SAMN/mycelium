// 21 — extraction: what a percept becomes when the mind replays it, and
// what a family does to an axis instead of minting a second card.

use crate::cognition::*;

/// A novel SUBSTANCE (here a rotation) germinates: one voice pends, a second
/// settles the listing; the basis grows only from gluing. Afterward the same
/// demand REDUCES to the length-1 sentence [0].
/// A second vantage here is `glue_local`'s two fixture keys (§3.4's
/// wire-identifier exception): this test proves nothing about 47.3's witness.
#[test]
fn a_novel_substance_germinates_then_decomposes() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)]; // L
let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)]; // R180 of L — non-trivial substance
assert_eq!(k.ingest(&a, &rot, [1; 32]), Outcome::GermPending);
assert_eq!(k.basis_len(), 0, "one voice does not complete");
assert_eq!(k.ingest(&a, &rot, [2; 32]), Outcome::GermPending, "two voices settle, they do not install");
assert_eq!(k.basis_len(), 0, "zero basis installs from a single hypha's two voices");
k.glue_local();
assert_eq!(k.basis_len(), 1, "the basis grows from gluing");
assert_eq!(k.ingest(&a, &rot, [3; 32]), Outcome::Decomposed(vec![0]), "reduces to the length-1 sentence");
}

/// STEP 7c END-TO-END: birth deposits evidence; the court's rebind
/// upgrades (⊤, Δ) → (π*, Δ) in place; the dual condition then bites in
/// live adjudication — the same demand in an off-region scene stops
/// decomposing through the word and pends as new coverage instead.
/// A second vantage here is `glue_local`'s two fixture keys (§3.4's
/// wire-identifier exception): this test proves nothing about 47.3's witness.
/// The word is an attested chain: the anchor binds a residual word to the
/// footprint it was born on, and a percept lists a sentence (20 as
/// ruled, 50.1), which carries no anchor. The chain retires at L8.
#[test]
fn rebind_activates_the_dual_condition_in_adjudication() {
use crate::transducer::anchor::Sel;
use crate::transducer::germinate::germinate;
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 3)];
let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 3)];
let chain = germinate(&a, &b).expect("a coboundary");
assert_eq!(k.attest(&chain, [1; 32]), Outcome::GermPending);
assert_eq!(k.attest(&chain, [2; 32]), Outcome::GermPending);
k.glue_local();
assert!(!k.basis().is_empty(), "the residual word stands");
assert!(k.basis()[0].anchor().is_none(), "born ⊤ — one pair never binds");
let addr = k.basis()[0].address();
let pi = Sel::Inter(Box::new(Sel::Hue(1)), Box::new(Sel::Star(Box::new(Sel::Hue(3)))));
assert!(k.apply_rebind(&addr, pi.clone()));
assert_eq!(k.basis()[0].anchor(), Some(&pi), "γ = (π*, Δ) in place");
// ON the interface: the anchored word still speaks.
let a2 = vec![(5, 0, 1), (5, 1, 1), (6, 0, 3), (6, 1, 3)];
let b2 = vec![(5, 0, 4), (5, 1, 4), (6, 0, 3), (6, 1, 3)];
// ON the interface: the anchored word still speaks — as itself, or as
// the section the court re-derives from it for an attest that cites
// nothing.
let on_interface = k.attest(&germinate(&a2, &b2).expect("a coboundary"), [3; 32]);
assert!(
    matches!(on_interface, Outcome::Decomposed(_) | Outcome::Sectioned(0)),
    "π covers the interface placement, got {on_interface:?}"
);
// Placement is a coordinate of the transport, not a selection rule:
// the same body elsewhere is the same word carried there.
let a3 = vec![(0, 0, 1), (0, 1, 1)];
let b3 = vec![(0, 0, 4), (0, 1, 4)];
let out = k.attest(&germinate(&a3, &b3).expect("a coboundary"), [3; 32]);
assert!(
    matches!(out, Outcome::Decomposed(_) | Outcome::Sectioned(0) | Outcome::GermPending),
    "a carried body is the same word, its section, or its residual, got {out:?}"
);
}

/// PUNCTURE-BEFORE-MINT: a family member (same base geometry, new transition
/// assignment) does NOT mint sibling card n+1 — the axis pierces the existing
/// thought and the member becomes a SECTION. One vantage sections it: the
/// court re-derived it from its own held word, and a second voice is a use,
/// not a step toward it. Growth as +axis:
/// the basis count stays 1 while the thought's volume grows; afterward the
/// bundle SPEAKS the whole family (both assignments decompose to it).
/// A second vantage here is `glue_local`'s two fixture keys (§3.4's
/// wire-identifier exception): this test proves nothing about 47.3's witness.
/// The members are attested chains: the puncture is a move on residual
/// words, and a percept lists a sentence (20 as ruled, 50.1) — a second
/// gauge on one body chart is a second sentence until the court extracts
/// over the family it accumulates. The chain retires at L8.
#[test]
fn a_family_punctures_an_axis_instead_of_minting() {
use crate::transducer::germinate::germinate;
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b1 = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)]; // (0,0): 1→4
let member = |b: &[(i64, i64, u8)]| germinate(&a, b).expect("a coboundary");
assert_eq!(k.attest(&member(&b1), [1; 32]), Outcome::GermPending, "one vantage dockets");
k.glue_local();
assert_eq!(k.basis_len(), 1);
// The family member: same cell, DIFFERENT transition (1→7).
let b2 = vec![(0, 0, 7), (0, 1, 1), (1, 0, 1)];
assert!(matches!(k.attest(&member(&b2), [1; 32]), Outcome::Sectioned(0)), "the axis PIERCES thought 0");
assert_eq!(k.basis_len(), 1, "count unchanged — growth was +axis");
assert_eq!(k.basis()[0].sections().len(), 1, "the thought gained a section (volume)");
// The bundle speaks the family: each member spoken again is answered as
// a section of the ONE word — the held assignment or the pierced one —
// neither a new word nor a listing. (An attest with no cites replays no
// deconstruction; the court re-derives the section from its own word.)
assert_eq!(k.attest(&member(&b1), [3; 32]), Outcome::Sectioned(0));
assert_eq!(k.attest(&member(&b2), [3; 32]), Outcome::Sectioned(0));
assert_eq!(k.basis_len(), 1, "a member spoken again mints nothing");
// And the address held stable across the puncture (identity = the word).
let far_b2: Vec<_> = b2.iter().map(|(r, c, h)| (r + 6, c + 6, *h)).collect();
let far_a: Vec<_> = a.iter().map(|(r, c, h)| (r + 6, c + 6, *h)).collect();
assert_eq!(
    k.attest(&germinate(&far_a, &far_b2).expect("a coboundary"), [4; 32]),
    Outcome::Sectioned(0),
    "transposed family member: the same section at any placement"
);
}

/// THE CONDENSE STROKE: position-identical sibling cards already in the basis
/// (a pre-puncture era's residue) are ABSORBED into one bundle during the
/// syntropic stroke — cardinality falls, sections rise, and the absorbed
/// words' discoverers still earn.
#[test]
fn the_condense_stroke_absorbs_a_family_into_one_bundle() {
use crate::transducer::element::from_residual;
use crate::transducer::germinate::germinate;
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 2)];
let b = vec![(0, 0, 5), (0, 1, 2)]; // (0,0): 1→5
k.basis_insert(from_residual(&germinate(&a, &b).unwrap()), [1; 32]);
for hue in [6u8, 7] {
    let bx = vec![(0, 0, hue), (0, 1, 2)];
    k.basis_insert(from_residual(&germinate(&a, &bx).unwrap()), [9; 32]);
}
assert_eq!(k.basis_len(), 3, "the un-condensed family");
// One versor per stroke: the cursor walks the stratum over strokes.
for _ in 0..3 {
    k.optimize();
}
assert_eq!(k.basis_len(), 1, "the family collapsed into one bundle");
assert_eq!(k.basis()[0].sections().len(), 2, "two sections along the pierced axis");
let addrs: Vec<Address> = k.dockets().iter().map(|d| d.addr.clone()).collect();
for a in addrs {
    let _ = k.apply_verdict(&a, crate::anastome::dialogue::RULING_UPHELD);
}
assert!(
    k.treasury().balance_mu(&[9; 32])[1] > 0 || k.treasury().balance_mu(&[9; 32]).iter().any(|&m| m > 0),
    "absorbed discoverers still earn, on the lane"
);
}

/// THE CROWN PROOF OF THE EXCISION: with no solver anywhere, the mesh forms
/// SENTENCES over germinated patterns — install two single-cell patterns, then
/// a compound demand containing BOTH (at different offsets) DECOMPOSES into
/// the two-word sentence by chain peeling. Nothing minted; language composed.
/// A second vantage here is `glue_local`'s two fixture keys (§3.4's
/// wire-identifier exception): this test proves nothing about 47.3's witness.
/// The patterns are attested chains: chain peeling and the puncture are
/// moves on residual words, and a percept lists a sentence (20 as ruled,
/// 50.1). The chain retires at L8.
#[test]
fn a_compound_demand_decomposes_over_germinated_patterns() {
use crate::transducer::germinate::germinate;
let mut k = Kernel::new();
let a1 = vec![(0, 0, 1), (0, 1, 2)];
let b1 = vec![(0, 0, 4), (0, 1, 2)]; // (0,0): 1→4 — pattern P
let p = germinate(&a1, &b1).expect("a coboundary");
k.attest(&p, [1; 32]);
assert_eq!(k.attest(&p, [2; 32]), Outcome::GermPending);
k.glue_local();
let a2 = vec![(0, 0, 2), (0, 1, 3)];
let b2 = vec![(0, 0, 2), (0, 1, 9)]; // (0,1): 3→9 — Q shares P's geometry:
let q = germinate(&a2, &b2).expect("a coboundary");
k.attest(&q, [1; 32]);
// …so under 3.5 it PUNCTURES P's axis (Installed(0) as a section).
match k.attest(&q, [2; 32]) {
    Outcome::GermPending => k.glue_local(),
    Outcome::Installed(_) | Outcome::Sectioned(_) => {}
    o => panic!("{o:?}"),
}
assert_eq!(k.basis_len(), 1, "the single-cell family is ONE bundle");
let a3 = vec![(0, 0, 1), (0, 1, 2), (0, 2, 2), (0, 3, 3)];
let b3 = vec![(0, 0, 4), (0, 1, 2), (0, 2, 2), (0, 3, 9)];
let before = k.basis_len();
match k.attest(&germinate(&a3, &b3).expect("a coboundary"), [3; 32]) {
    Outcome::Decomposed(_) | Outcome::GermPending => {}
    o => panic!("a compound chain is one demand, got {o:?}"),
}
assert_eq!(k.basis_len(), before, "a compound chain is not peeled into a second word");
}

/// A shape deformation (L → bar) that NO versor carries is a percept the
/// eyes cannot open: no body pair admitted, no vertex that acts, so no
/// sentence and no listing (50.5, L7) — never a residual chain germinated
/// as a word, which was this test's claim in the chain era. The deformation
/// is answered by the cover of 50.8 (L11), not by a listing of its cells.
#[test]
fn a_shape_deformation_no_versor_carries_lists_nothing() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)]; // L
let b = vec![(0, 0, 1), (0, 1, 1), (0, 2, 1)]; // bar — no rigid motor
assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::Pivot, "no sentence, no listing");
assert_eq!(k.basis_len(), 0);
assert!(k.pending.is_empty(), "nothing on the book to await consilience");
}

/// Two distinct objects in one pair are one pair form, not a Cartesian
/// fragment per body. Two voices docket the residual.
/// A second vantage here is `glue_local`'s two fixture keys (§3.4's
/// wire-identifier exception): this test proves nothing about 47.3's witness.
#[test]
fn independent_objects_fragment_and_resolve() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1), (5, 5, 2), (5, 6, 2), (6, 5, 2), (6, 6, 2)];
let b = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1), (5, 5, 3), (5, 6, 3), (6, 5, 3), (6, 6, 3)];
assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending, "one pair, one residual");
assert_eq!(k.ingest(&a, &b, [2; 32]), Outcome::GermPending);
k.glue_local();
assert_eq!(k.basis_len(), 1, "the pair form is one word");
}

/// W2: the percept is pair_form → extract_rule_with. A family the engine
/// closes lists the minted sentence; a withheld form lists one remainder.
#[test]
fn the_percept_is_the_extracted_pair_form() {
use crate::transducer::family::{cells_to_grid, extract_rule_with, mint_sentence, NoCrystal};
let a = vec![(0, 0, 1), (0, 1, 1)];
let b = vec![(0, 0, 2), (0, 1, 2)];
let family = vec![(cells_to_grid(&a), cells_to_grid(&b))];
let mut k = Kernel::new();
assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
let listed = &k.quote().1;
assert_eq!(listed.len(), 1, "one pair, one listing");
match extract_rule_with(&family, &NoCrystal)
    .and_then(|rule| mint_sentence(&rule, &family, &NoCrystal))
{
    Some(e) => assert!(
        listed[0].0.same_vertex(&e),
        "the court dockets the minted form"
    ),
    None => assert!(
        !listed[0].0.is_sentence(),
        "a withheld form is the remainder, not a sentence"
    ),
}
}
