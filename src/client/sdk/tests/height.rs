// 23.8, 45.2 — THE HEIGHT THE MIND READS: the book's height off a reply,
// advancing only, is the mind's TUniversal and what it stamps its dialect
// with.

use crate::client::sdk::*;
use crate::transducer::element::from_canonical;
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
    let word = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
    let tu = e.now_tu();
    e.epigenome.stand_dialect_at(word.clone(), 1, tu);
    assert_eq!(
        e.epigenome.dialect.get(word.address().as_bytes()).map(|d| d.last_used_tu),
        Some(9),
        "the dialect entry carries the height read, not zero"
    );
}

/// 78.49, OK1 — THE MIND'S K IS ITS LEDGER COORDINATE: one step per cycle in
/// which it acted, counted at the close by name; a cycle that acted on
/// nothing leaves it where it stood. It is volatile — the genome holds
/// crystallized knowledge, not a clock — so a mind booted from a genome
/// starts at zero.
#[test]
fn the_minds_k_counts_the_cycles_it_acted_in_and_is_not_persisted() {
    let mut e = Entity::open("127.0.0.1:1", Kernel::named("no-words"));
    e.act();
    assert!(e.close_cycle(), "a cycle that acted closes");
    assert!(!e.close_cycle(), "one that acted on nothing does not");
    e.act();
    assert!(e.close_cycle(), "and the next that acted");
    assert_eq!((e.epigenome.genome_len(), e.own_k()), (0, 2), "two orbits acted in, no word admitted");
    let (bytes, refused) = e.genome_bytes();
    assert_eq!(refused.total(), 0, "no word, nothing refused");
    let mut booted = Entity::open("127.0.0.1:1", Kernel::named("no-words"));
    assert_eq!(booted.restore_genome(&bytes), Ok(()), "a genome with no words restores");
    assert_eq!(booted.own_k(), 0, "and the booted mind's K is its own, from zero");
}

/// 78.49 — THE GENOME IS WHAT A COURT ADMITTED. A word a syllabus page
/// taught with the court's stamp is crystallized at that stamp and restores
/// at it; a word learned with no court's stamp is held in the mind and
/// refused by the writer by name — it is not crystallized knowledge until a
/// court rules it. A genome that carried a voice's ledger is refused by name.
#[test]
fn a_word_is_crystallized_at_the_courts_stamp_and_only_there() {
    let word = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
    let court = crate::chronometry::Stamp::new(7, [3; 32], crate::chronometry::Place { k: 2, i: 1 });
    let mut e = Entity::open("127.0.0.1:1", Kernel::named("learned-word"));
    let mut budget = e.stroke();
    let bytes = word.to_bytes().expect("the record writes");
    assert!(e.absorb_taught_at(&bytes, Some(court), &mut budget).is_some(), "the word is learned");
    assert_eq!(e.epigenome.admitted_at(word.address().as_bytes()), Some(court), "filed at the court's stamp");
    let (genome, refused) = e.genome_bytes();
    assert_eq!(refused.total(), 0, "nothing refused");
    let mut booted = Entity::open("127.0.0.1:1", Kernel::named("learned-word"));
    assert_eq!(booted.restore_genome(&genome), Ok(()), "the genome restores");
    assert_eq!(booted.epigenome.genome_len(), 1, "with its word");
    assert_eq!(booted.epigenome.admitted_at(word.address().as_bytes()), Some(court), "at the court's stamp");

    let mut unruled = Entity::open("127.0.0.1:1", Kernel::named("unruled"));
    assert!(unruled.absorb_taught(&bytes).is_some(), "a word with no court's stamp is learned");
    let (_, refused) = unruled.genome_bytes();
    assert_eq!(refused.unstamped, 1, "and not crystallized: no court admitted it");

    let mut ledgered = genome.clone();
    ledgered[..4].copy_from_slice(b"GEN6");
    let mut refused_boot = Entity::open("127.0.0.1:1", Kernel::named("learned-word"));
    assert!(refused_boot.restore_genome(&ledgered).is_err_and(|why| why.contains("voice's ledger")), "GEN6 refused by name");
    assert_eq!(refused_boot.epigenome.genome_len(), 0, "and nothing is partly restored");
}
