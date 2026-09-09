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
use crate::transducer::element::from_residual;
let mut entity = Entity::open("127.0.0.1:1", Kernel::named("refine"));
let w1 = from_residual(&vec![(0, 0, 1u8, 2u8)]);
let w2 = from_residual(&vec![(0, 0, 3u8, 4u8), (0, 1, 3, 4)]);
entity.epigenome.learn(&w1.to_bytes());
entity.epigenome.learn(&w2.to_bytes());
assert_eq!(entity.epigenome.genome_len(), 2, "two ruled words");
// A stood word carrying the second word's bar turned upright: another
// vertex (a translation alone would re-derive the genome word's address,
// M2) on the same chart, so the deconstruction reaches it by a versor. The
// mind expresses the residual and it stands in the dialect (24.3).
let bar = [(5i64, 5i64, 3u8, 4u8), (6, 5, 3, 4)];
assert!(entity.epigenome.express(&bar, 1, 1), "the residual is claimed and stands");
assert_eq!(entity.dialect_len(), 1, "it stands in the dialect, not the genome");
// The second genome word deconstructs on the working atlas through the
// stood word alone: its cite's index runs past the genome.
let located = entity.epigenome.locate(w2.sub(), Some(1));
assert!(
    located.cites.iter().any(|c| c.word >= 2) && located.residual.is_empty(),
    "the deconstruction cites the stood word, past the genome: {:?}",
    located.cites.iter().map(|c| c.word).collect::<Vec<_>>()
);
// The cursor advances from 0 to that second word.
let bid = entity.seek_refinement();
assert!(
    bid.is_none(),
    "a deconstruction through a stood word is no refinement bid: {bid:?}"
);
}
