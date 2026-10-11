// MU-CLIENT §5.4, S3 — THE MIND PROPOSES A RETIREMENT: a word its ruled
// words reproduce exactly is minted as a `Retire` rewrite naming it, citing
// what speaks it, and freeing what it carried.

use crate::client::sdk::*;
use crate::transducer::element::{canonical_chain, from_canonical};
use crate::transducer::rewrite::{Rewrite, RewriteKind, RewritePayload};

fn chain_word(links: &[(i64, i64, u8, u8)]) -> crate::transducer::element::Element {
    from_canonical(&canonical_chain(links).expect("one entry per position is A5's chain"))
}

/// A WORD THE GENOME REPRODUCES RETIRES, ITS MEANING UNCHANGED. The genome
/// holds a horizontal bar and the same bar turned upright — a second word
/// the first reaches by a versor (a translation alone would be the same
/// address, M2). The cursor comes round to a bar that decomposes over the
/// other with no residue: the mind mints a `Retire` naming it as both what
/// it rewrote and what it reached, citing the other bar, never itself,
/// freeing exactly the lanes the word carried — and the court's blind
/// replay of the tape refuses nothing.
#[test]
fn a_word_its_ruled_words_reproduce_is_proposed_for_retirement() {
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("retire"));
    let one = chain_word(&[(0, 0, 3u8, 4u8), (0, 1, 3, 4)]);
    let two = chain_word(&[(0, 0, 3u8, 4u8), (1, 0, 3, 4)]);
    for w in [&one, &two] {
        assert!(entity.absorb_taught(&w.to_bytes().expect("the record writes")).is_some(), "learned");
    }
    let rw = (0..entity.epigenome.genome_len()).find_map(|_| entity.retirement()).expect("one bar retires over the other");
    let r = Rewrite::from_element(&rw).expect("a rewrite tape");
    assert_eq!(r.kind, RewriteKind::Retire);
    let RewritePayload::Retire { cites, freed } = &r.payload else { panic!("a retirement's payload") };
    assert_eq!(r.before.len(), 1, "one word retires");
    assert!(r.before.contains(&r.after), "the meaning is unchanged: what it reached is what it rewrote");
    let retired = r.after.addr().clone();
    let word = [&one, &two].into_iter().find(|w| w.address() == retired).expect("one of the two bars");
    assert!(!cites.is_empty() && cites.iter().all(|c| c != &retired), "cited by what speaks it, never itself");
    assert_eq!(*freed, word.magnitude(), "it frees exactly what it carried");
    assert_eq!(crate::transducer::rewrite::replay(&rw), None, "the court's blind replay of the tape refuses nothing");
}
