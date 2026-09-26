// 23.8, 24.1 — the refinement bid: a compression the mesh can replay cites
// ruled words only.

use crate::client::sdk::*;

/// Stage 11 I35, rule 11 — A REFINEMENT BID CITES RULED WORDS ONLY. The
/// genome holds two words and the dialect stands a word whose chain
/// deconstructs the second; `locate` cites over the working atlas, so the
/// cite's index runs past the genome. The bid reads such a cite as no bid.
/// It indexed the genome with it, and voice 4 died on the fleet of
/// 2026-09-08 ("index out of bounds: the len is 24 but the index is 76").
#[test]
fn a_refinement_bid_cites_only_ruled_words() {
use crate::transducer::element::{canonical_chain, from_canonical};
let chain_word = |links: &[(i64, i64, u8, u8)]| {
    from_canonical(&canonical_chain(links).expect("one entry per position is A5's chain"))
};
let mut entity = Entity::open("127.0.0.1:1", Kernel::named("refine"));
let w1 = chain_word(&[(0, 0, 1u8, 2u8)]);
let w2 = chain_word(&[(0, 0, 3u8, 4u8), (0, 1, 3, 4)]);
assert!(entity.absorb_taught(&w1.to_bytes().expect("the record writes")).is_some(), "the first word is learned by an act of the voice");
assert!(entity.absorb_taught(&w2.to_bytes().expect("the record writes")).is_some(), "the second word is learned by an act of the voice");
assert_eq!(entity.epigenome.genome_len(), 2, "two ruled words");
// A stood word carrying the second word's bar turned upright: another
// vertex (a translation alone would re-derive the genome word's address,
// M2) on the same chart, so the deconstruction reaches it by a versor. The
// mind expresses the residual and it stands in the dialect (24.3).
let bar = [(5i64, 5i64, 3u8, 4u8), (6, 5, 3, 4)];
assert!(entity.epigenome.express(&bar, 1, 1, &mut super::common::budget()), "the residual is claimed and stands");
assert_eq!(entity.dialect_len(), 1, "it stands in the dialect, not the genome");
// The second genome word deconstructs on the working atlas through the
// STOOD word alone. The claim used to be spelled `c.word >= 2` — "its
// index runs past the genome" — which is only true while the dialect is
// numbered above the genome in one snapshot's offset space. The claim
// itself is that the cited word is one the genome does not hold, and at a
// coordinate that is what it says.
let w2_addr = w2.address();
let located = entity.epigenome.locate(w2.sub(), Some(w2_addr.as_bytes()), &mut super::common::budget());
assert!(
    located
        .cites
        .iter()
        .any(|c| entity.epigenome.word_at(c.word.as_bytes()).is_none())
        && located.residual.is_empty(),
    "the deconstruction cites the stood word, which the genome does not hold: {:?}",
    located.cites.iter().map(|c| c.word.clone()).collect::<Vec<_>>()
);
// The cursor advances from 0 to that second word.
let bid = entity.seek_refinement();
assert!(
    bid.is_none(),
    "a deconstruction through a stood word is no refinement bid: {bid:?}"
);
}
