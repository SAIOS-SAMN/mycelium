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

/// R1 (LD3-c) — A VOICE WITH A LEDGER AND NO WORDS KEEPS ITS K ACROSS A
/// BOOT. An empty words trie is a genome with no words, not a refusal, so
/// the ledger behind it restores and K continues from where it stood.
#[test]
fn a_voice_with_a_ledger_and_no_words_boots_with_its_k_continued() {
    let mut e = Entity::open("127.0.0.1:1", Kernel::named("no-words"));
    e.session.seal(crate::chronometry::ActKind::Uttered, [0, 1, 0, 0], Vec::new());
    assert!(matches!(e.close_orbit(), crate::chronometry::Close::Sealed(_)), "the orbit closes by name");
    e.session.seal(crate::chronometry::ActKind::Uttered, [0, 1, 0, 0], Vec::new());
    assert!(matches!(e.close_orbit(), crate::chronometry::Close::Sealed(_)), "and the next");
    assert_eq!((e.epigenome.genome_len(), e.own_k()), (0, 2), "a ledger of two orbits and no word");
    let (bytes, refused) = e.genome_bytes().expect("the ledger writes");
    assert_eq!(refused.total(), 0, "no word, nothing refused");
    let mut booted = Entity::open("127.0.0.1:1", Kernel::named("no-words"));
    assert_eq!(booted.restore_genome(&bytes), Ok(()), "a genome with no words restores");
    assert_eq!(booted.own_k(), 2, "and the voice's K continues from where it stood");
}

/// THE STAMPS A GENOME'S WORDS STAND AT ARE ITS OWN LEDGER'S (78.27): a
/// word the voice learned is admitted by its own Learned node, restores at
/// that node's stamp, and a word filed on this voice's line at a stamp its
/// ledger does not hold refuses the file.
#[test]
fn a_learned_word_restores_at_its_own_ledgers_stamp() {
    let word = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
    let mut e = Entity::open("127.0.0.1:1", Kernel::named("learned-word"));
    assert!(e.absorb_taught(&word.to_bytes()).is_some(), "the word is learned");
    let at = e.session.voice().admitted_at(word.address().as_bytes()).expect("admitted by the Learned node");
    assert!(e.session.voice().chain().node(&at).is_some(), "at a node of this voice's own ledger");
    let (bytes, refused) = e.genome_bytes().expect("the ledger writes");
    assert_eq!(refused.total(), 0, "nothing refused");
    let mut booted = Entity::open("127.0.0.1:1", Kernel::named("learned-word"));
    assert_eq!(booted.restore_genome(&bytes), Ok(()), "the genome restores");
    assert_eq!(booted.epigenome.genome_len(), 1, "with its word");
    assert_eq!(booted.session.voice().admitted_at(word.address().as_bytes()), Some(at), "at the stamp it was admitted at");

    // The same word filed at a stamp of this voice's line that its ledger
    // does not hold: the file is refused.
    let vk = e.session.voice().identity();
    let (words, _) = e.epigenome.crystallize(|_| Some(crate::chronometry::Stamp::new(99, vk, crate::chronometry::Place { k: 9, i: 0 })));
    let mut forged = words;
    e.session.voice().chain().put_tries(&mut forged).expect("the ledger writes");
    let mut refused_boot = Entity::open("127.0.0.1:1", Kernel::named("learned-word"));
    assert!(refused_boot.restore_genome(&forged).is_err_and(|why| why.contains("does not hold")), "a stamp its ledger does not hold refuses the file");
    assert_eq!((refused_boot.epigenome.genome_len(), refused_boot.own_k()), (0, 0), "and nothing is partly restored");
}
