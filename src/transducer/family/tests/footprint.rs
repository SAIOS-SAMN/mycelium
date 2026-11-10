// A9, #158 — A WORD'S FOOTPRINT IS THE SUPPORT OF ITS DESCRIPTION: the sites
// the word's speech of a pair's input changes, deduced by speaking the word,
// never declared; and the sites of a pair's change it misses.

use super::fixtures::{cipher_form, digits};
use crate::transducer::family::footprint::{footprint_of, missed, pair_change, Footprint};
use crate::transducer::family::NoCrystal;

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 20)
}

/// THE FOOTPRINT IS DEDUCED FROM THE WORD: two bodies carried two columns
/// along; spoken onto the input, the word changes exactly the sites the pair
/// changes, and it misses none of them.
#[test]
fn footprint_is_the_sites_the_words_speech_changes() {
    let (x, y) = (digits(&["1100200"]), digits(&["0011002"]));
    let form = cipher_form(&x, &y);
    let change = pair_change(&x, &y).expect("the change writes on 0");
    let Footprint::Read(fp) = footprint_of(&form, &x, Some(0), &NoCrystal, &mut stroke()) else { panic!("the word speaks its input") };
    assert_eq!(fp, change, "the word's support is the pair's change");
    assert!(missed(&fp, &change).is_empty(), "nothing the pair changes is missed");
}

/// A CHANGE THE WORD DOES NOT SPEAK IS MISSED, BY SITE: on a pair that also
/// paints a site the word leaves alone, the footprint misses exactly that
/// site.
#[test]
fn footprint_misses_a_change_the_word_does_not_speak() {
    let form = cipher_form(&digits(&["1100200"]), &digits(&["0011002"]));
    let (x, y) = (digits(&["11002000"]), digits(&["00110027"]));
    let change = pair_change(&x, &y).expect("the change writes on 0");
    let Footprint::Read(fp) = footprint_of(&form, &x, Some(0), &NoCrystal, &mut stroke()) else { panic!("the word speaks its input") };
    let gone = missed(&fp, &change);
    assert_eq!(gone.len(), 1, "one site the word leaves alone: {gone:?}");
    assert_eq!(gone.iter().map(|s| s.coordinates()).collect::<Vec<_>>(), vec![[0, 7]], "the painted 7");
}
