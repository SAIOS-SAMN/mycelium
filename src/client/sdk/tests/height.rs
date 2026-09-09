// 23.8, 45.2 — THE HEIGHT THE MIND READS: the book's height off a reply,
// advancing only, is the mind's TUniversal and what it stamps its dialect
// with.

use crate::client::sdk::*;
use crate::transducer::element::from_residual;
use crate::transducer::germinate::germinate;

#[test]
fn the_height_a_mind_reads_only_advances_and_stamps_what_it_stands() {
    let mut e = Entity::open("127.0.0.1:1", Kernel::named("height"));
    assert_eq!(e.now_tu(), 0, "nothing read yet");
    e.advance_height(5);
    assert_eq!(e.now_tu(), 5, "a quote at height 5 sets the mind's TUniversal");
    e.advance_height(3);
    assert_eq!(e.now_tu(), 5, "a lower reading moves nothing");
    e.advance_height(0);
    assert_eq!(e.now_tu(), 5, "an absent height moves nothing");
    e.advance_height(9);
    assert_eq!(e.now_tu(), 9, "and it only advances");
    // What the mind stands is stamped with the height it read.
    let word = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
    let tu = e.now_tu();
    e.epigenome.stand_dialect_at(word.clone(), 1, tu);
    assert_eq!(
        e.epigenome.dialect.get(word.address().as_bytes()).map(|d| d.last_used_tu),
        Some(9),
        "the dialect entry carries the height read, not zero"
    );
}
