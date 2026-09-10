// 21 — extraction: what a spoken pair becomes when the court replays it,
// and what a family does to an axis instead of minting a second card.

use super::common::speak_pair;
use crate::cognition::*;

/// A novel SUBSTANCE (here a rotation) germinates: one voice pends, a second
/// settles the listing; the basis grows only from gluing. Afterward the same
/// sentence is a USE of the held word, the length-1 sentence [0].
/// A second vantage here is `glue_local`'s two fixture keys (§3.4's
/// wire-identifier exception): this test proves nothing about 47.3's witness.
#[test]
fn a_novel_substance_germinates_then_decomposes() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)]; // L
let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)]; // R180 of L — non-trivial substance
assert_eq!(speak_pair(&mut k, &a, &rot, [1; 32]), Outcome::GermPending);
assert_eq!(k.basis_len(), 0, "one voice does not complete");
assert_eq!(speak_pair(&mut k, &a, &rot, [2; 32]), Outcome::GermPending, "two voices settle, they do not install");
assert_eq!(k.basis_len(), 0, "zero basis installs from a single hypha's two voices");
k.glue_local();
assert_eq!(k.basis_len(), 1, "the basis grows from gluing");
assert_eq!(speak_pair(&mut k, &a, &rot, [3; 32]), Outcome::Decomposed(vec![0]), "a use of the length-1 sentence");
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

/// A shape deformation (L → bar) that NO versor carries is a pair the
/// eyes cannot open: no body pair admitted, no vertex that acts, so the
/// voice has no sentence and nothing is listed (50.5, L7) — never a
/// residual chain germinated as a word, which was this test's claim in the
/// chain era. The deformation is answered by the cover of 50.8 (L11), not
/// by a listing of its cells.
#[test]
fn a_shape_deformation_no_versor_carries_lists_nothing() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)]; // L
let b = vec![(0, 0, 1), (0, 1, 1), (0, 2, 1)]; // bar — no rigid motor
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::Pivot, "no sentence, no listing");
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
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending, "one pair, one sentence");
assert_eq!(speak_pair(&mut k, &a, &b, [2; 32]), Outcome::GermPending);
k.glue_local();
assert_eq!(k.basis_len(), 1, "the pair form is one word");
}

/// W2: the spoken pair is pair_form → extract_rule_with, minted by the
/// voice. The court lists the minted sentence as spoken.
#[test]
fn the_spoken_pair_is_the_extracted_pair_form() {
use crate::transducer::family::{cells_to_grid, extract_rule_with, mint_sentence, NoCrystal};
let a = vec![(0, 0, 1), (0, 1, 1)];
let b = vec![(0, 0, 2), (0, 1, 2)];
let family = vec![(cells_to_grid(&a), cells_to_grid(&b))];
let mut k = Kernel::new();
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
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
