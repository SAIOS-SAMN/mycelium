// §8.1 — WHAT THE ENTITY HOLDS THAT NO LINE REPORTS: the reading the
// stroke line carries beside the counts, so that a process growing while
// every counted carrier is flat says which uncounted one grew.

use crate::client::sdk::*;

/// A fresh entity holds nothing it does not report; expressing a residual
/// adds one word to the expressed vocabulary and nothing else, and
/// expressing the same residual again adds none — the reading counts what
/// stands, never what was asked.
#[test]
fn carriers_count_one_word_per_expression_once() {
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("carriers"));
    assert_eq!(entity.carriers(), [0; 10], "a fresh entity holds nothing unreported");
    let residual = vec![(0i64, 0i64, 1u8, 2u8), (0, 1, 1, 2)];
    assert!(entity.epigenome.express(&residual, 1, 1, &mut super::common::budget()), "a residual with an A5 form is expressed");
    let after = entity.carriers();
    assert_eq!(after[6], 1, "one word expressed: {after:?}");
    assert_eq!(after[..6], [0; 6], "and nothing else moved: {after:?}");
    assert!(!entity.epigenome.express(&residual, 2, 2, &mut super::common::budget()), "the same body under a transport is already spoken");
    assert_eq!(entity.carriers()[6], 1, "so the expressed vocabulary does not grow on a repeat");
}
