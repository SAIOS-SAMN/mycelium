// 23.8, 45.2 — THE HEIGHT THE MIND READS: the book's height off a reply,
// advancing only, is the mind's TUniversal and what it stamps its dialect
// with.

use crate::client::sdk::*;
use crate::transducer::element::from_canonical;
use crate::transducer::germinate::germinate;

/// The height a mind reads only advances, and what the mind stands is
/// stamped with the height it read.
#[test]
fn advance_height_only_advances_and_stamps_dialect() {
    let mut e = Mind::open("127.0.0.1:1", Voice::named("height"));
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

/// 78.49, OK1, MU-CLIENT §4.4(4) — THE MIND'S K IS ITS COMPLETED ORBITS: the
/// close steps it by exactly the returns of the stroke, and a stroke in which
/// nothing returned leaves it where it stood — whatever the mind did in it:
/// an act moves no clock. It is volatile — the genome holds crystallized
/// knowledge, not a clock — so a mind booted from a genome starts at zero.
#[test]
fn close_cycle_counts_returns_without_persisting_k() {
    let mut e = Mind::open("127.0.0.1:1", Voice::named("no-words"));
    e.act();
    assert!(!e.close_cycle(0), "an act with no return closes nothing");
    assert_eq!(e.own_k(), 0, "acting moved no clock");
    assert!(e.close_cycle(1), "one orbit returned: the stroke closes");
    assert!(e.close_cycle(2), "two returned in one stroke");
    assert_eq!((e.epigenome.genome_len(), e.own_k()), (0, 3), "three returns, no word admitted");
    let (bytes, refused) = e.genome_bytes();
    assert_eq!(refused.total(), 0, "no word, nothing refused");
    let mut booted = Mind::open("127.0.0.1:1", Voice::named("no-words"));
    assert_eq!(booted.restore_genome(&bytes), Ok(()), "a genome with no words restores");
    assert_eq!(booted.own_k(), 0, "and the booted mind's K is its own, from zero");
}

/// A REWRITE IS AN ACT ON THE LANGUAGE, NEVER A WORD OF IT. A syllabus page
/// carrying a court's `Retire` back teaches the mind nothing: the genome does
/// not take it, and its stratum is one the sheaf sync never fetches. Where a
/// language holds one, the genome writer leaves it out by name (observed on the
/// development deployment, 2026-10-19: rewrites were 88% of a 48 MB genome).
#[test]
fn absorb_taught_never_admits_rewrite_to_genome() {
    let word = super::common::two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
    let other = super::common::two_body_sentence([&[(5, 6), (5, 6)], &[(7, 8), (7, 8)]]);
    let payload = crate::transducer::rewrite::RewritePayload::Retire { cites: vec![other.address()], freed: word.magnitude() };
    let rewrite = crate::transducer::rewrite::mint(crate::transducer::rewrite::RewriteKind::Retire, &[word.address()], word.address(), payload, word.magnitude())
        .expect("the retirement mints");
    assert!(rewrite.is_rewrite(), "a rewrite");
    let tape = rewrite.to_bytes().expect("the record writes");
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("rewritten"));
    let _ = mind.absorb_taught(&tape);
    assert!(!mind.epigenome.holds(rewrite.address().as_bytes()) && mind.epigenome.genome_len() == 0, "never learned into the genome");
    let stratum = crate::transducer::stalk::invariant_of(&rewrite).key();
    assert!(mind.board.residual_strata.contains(stratum.as_bytes()), "its stratum is one the sheaf sync never fetches");
    let mut epi = Epigenome::new();
    if epi.learn(&tape, &mut super::common::budget()).is_some() && epi.holds(rewrite.address().as_bytes()) {
        epi.admit_all_at(crate::chronometry::Stamp::new(1, [7; 32], crate::chronometry::Place::GENESIS));
        let (frozen, refused) = epi.crystallize();
        assert_eq!(refused.rewrite, 1, "a rewrite the language holds is left out of the genome by name");
        let mut back = Epigenome::new();
        assert!(back.uncrystallize(&frozen).is_some() && !back.holds(rewrite.address().as_bytes()), "and never restored as a word");
    }
}

/// A GENOME FILE HOLDING MORE THAN THE GENOME IS WRITTEN AGAIN. A file an
/// older writer wrote with a rewrite at its citation restores without it,
/// and the mind keeping that genome holds it unwritten — its file is written
/// at the first stroke, though no word was gained — so the file is the genome
/// and never more; a file holding only words is kept as it is.
#[test]
fn keep_genome_rewrites_file_restore_left_short() {
    let word = super::common::two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
    let payload = crate::transducer::rewrite::RewritePayload::Retire { cites: vec![word.address()], freed: word.magnitude() };
    let rewrite = crate::transducer::rewrite::mint(crate::transducer::rewrite::RewriteKind::Retire, &[word.address()], word.address(), payload, word.magnitude())
        .expect("the retirement mints");
    // The file an older writer wrote: the rewrite at its own citation, the
    // trie the genome file is.
    let at = crate::chronometry::Stamp::new(1, [7; 32], crate::chronometry::Place::GENESIS);
    let mut key = Vec::new();
    crate::chronometry::Cite::word(&rewrite, at).put_canonical(&mut key).expect("its citation writes");
    let mut nodes: crate::network::nerve::Keyed<crate::snapshot::Leaves> = crate::network::nerve::Keyed::new();
    nodes.insert(key, vec![(1, rewrite.to_bytes().expect("the record writes"))]);
    let mut older = b"GEN7".to_vec();
    crate::snapshot::put_trie(&mut older, "words", &nodes);
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("rewritten-file"));
    assert_eq!(mind.restore_genome(&older), Ok(()), "the older file restores");
    assert_eq!(mind.epigenome.genome_len(), 0, "without its rewrite");
    mind.keep_genome("unused".to_string());
    assert!(mind.board.genome_unwritten, "the file held more than the genome: written again at the first stroke");
    let (plain, _) = Epigenome::new().crystallize();
    let mut fresh = Mind::open("127.0.0.1:1", Voice::named("plain-file"));
    assert_eq!(fresh.restore_genome(&plain), Ok(()), "a file of words alone restores");
    fresh.keep_genome("unused".to_string());
    assert!(!fresh.board.genome_unwritten, "and is kept as it is");
}

/// 78.49 — THE GENOME IS WHAT A COURT ADMITTED. A word a syllabus page
/// taught with the court's stamp is crystallized at that stamp and restores
/// at it; a word learned with no court's stamp is held in the mind and
/// refused by the writer by name — it is not crystallized knowledge until a
/// court rules it. A genome that carried a voice's ledger is refused by name.
#[test]
fn genome_bytes_crystallizes_word_only_at_court_stamp() {
    // A sentence of the language: a residual chain is an open question, never
    // a word (MU-CLIENT §7.1), and never reaches the genome.
    let word = super::common::two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
    let court = crate::chronometry::Stamp::new(7, [3; 32], crate::chronometry::Place { k: 2, i: 1 });
    let mut e = Mind::open("127.0.0.1:1", Voice::named("learned-word"));
    let mut budget = e.stroke();
    let bytes = word.to_bytes().expect("the record writes");
    assert!(e.absorb_taught_at(&bytes, Some(court), &mut budget).is_some(), "the word is learned");
    assert_eq!(e.epigenome.admitted_at(word.address().as_bytes()), Some(court), "filed at the court's stamp");
    let (genome, refused) = e.genome_bytes();
    assert_eq!(refused.total(), 0, "nothing refused");
    let mut booted = Mind::open("127.0.0.1:1", Voice::named("learned-word"));
    assert_eq!(booted.restore_genome(&genome), Ok(()), "the genome restores");
    assert_eq!(booted.epigenome.genome_len(), 1, "with its word");
    assert_eq!(booted.epigenome.admitted_at(word.address().as_bytes()), Some(court), "at the court's stamp");

    let mut unruled = Mind::open("127.0.0.1:1", Voice::named("unruled"));
    assert!(unruled.absorb_taught(&bytes).is_some(), "a word with no court's stamp is learned");
    let (_, refused) = unruled.genome_bytes();
    assert_eq!(refused.unstamped, 1, "and not crystallized: no court admitted it");

    // A COURT THAT RESTARTED SERVES ITS SYLLABUS AGAIN: the booted mind holds
    // the word at its address, so the page is heard by holding it — nothing
    // learned, measured or admitted again, and the word keeps the stamp it
    // was admitted at (`file_admitted`).
    let again = crate::chronometry::Stamp::new(9, [3; 32], crate::chronometry::Place { k: 4, i: 1 });
    let _ = booted.take_learned_mu();
    let mut budget = booted.stroke();
    assert_eq!(booted.absorb_taught_at(&bytes, Some(again), &mut budget), Some(Comprehension::Held), "a held word re-served is heard by holding it");
    assert_eq!(booted.take_learned_mu(), [0; 4], "the page brought nothing the mind did not hold");
    assert_eq!(booted.epigenome.genome_len(), 1, "nothing admitted again");
    assert_eq!(booted.epigenome.admitted_at(word.address().as_bytes()), Some(court), "a word admitted once keeps its stamp");
    let mut fresh = Mind::open("127.0.0.1:1", Voice::named("learned-word"));
    let mut budget = fresh.stroke();
    let _ = fresh.absorb_taught_at(&bytes, Some(court), &mut budget);
    assert_ne!(fresh.take_learned_mu(), [0; 4], "a word not held measures what it brought");

    let mut ledgered = genome.clone();
    ledgered[..4].copy_from_slice(b"GEN6");
    let mut refused_boot = Mind::open("127.0.0.1:1", Voice::named("learned-word"));
    assert!(refused_boot.restore_genome(&ledgered).is_err_and(|why| why.contains("voice's ledger")), "GEN6 refused by name");
    assert_eq!(refused_boot.epigenome.genome_len(), 0, "and nothing is partly restored");
}
