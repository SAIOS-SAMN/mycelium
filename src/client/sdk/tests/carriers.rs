// §8.1 — WHAT THE ENTITY HOLDS THAT NO LINE REPORTS: the reading the
// stroke line carries beside the counts, so that a process growing while
// every counted carrier is flat says which uncounted one grew.

use crate::client::sdk::*;

/// A fresh entity holds nothing it does not report; expressing a residue
/// adds one word to the expressed vocabulary and nothing else, and
/// expressing the same residue again adds none — the reading counts what
/// stands, never what was asked.
#[test]
fn carriers_counts_one_word_per_expression_once() {
    let mut entity = Mind::open("127.0.0.1:1", Voice::named("carriers"));
    assert_eq!(entity.carriers(), [0; 10], "a fresh entity holds nothing unreported");
    use crate::transducer::sentence::tests::fixtures::link;
    let residue = vec![link(0, 0, 1, 2), link(0, 1, 1, 2)];
    assert!(entity.language.express(&residue, 1, 1, &mut super::fixtures::budget()), "a residue with an A5 form is expressed");
    let after = entity.carriers();
    assert_eq!(after[6], 1, "one word expressed: {after:?}");
    assert_eq!(after[..6], [0; 6], "and nothing else moved: {after:?}");
    assert!(!entity.language.express(&residue, 2, 2, &mut super::fixtures::budget()), "the same body under a transport is already spoken");
    assert_eq!(entity.carriers()[6], 1, "so the expressed vocabulary does not grow on a repeat");
}
