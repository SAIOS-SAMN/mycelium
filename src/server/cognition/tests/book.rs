// 23 — THE BOOK and THE BASIS: what the mind holds, what it teaches, and
// what it refuses to order.

use super::common::*;
use crate::cognition::*;
use crate::transducer::germinate::germinate;
use crate::transducer::element::{self, Element};

/// An empty atlas is incident to the root prefix: the first quote
/// reads the book at depth zero.
#[test]
fn empty_atlas_is_incident_to_the_root_prefix() {
let k = std::sync::Mutex::new(Kernel::new());
{
    let mut g = k.lock().unwrap();
    let a = vec![(0, 0, 1u8), (0, 1, 1)];
    let b = vec![(0, 0, 2u8), (0, 1, 2)];
    assert!(matches!(speak_pair(&mut g, &a, &b, [1; 32]), Outcome::GermPending));
    // A second voice on the same sentence is a use of the listed address
    // (§3.1 as ruled): one listing, quoted.
    assert!(matches!(speak_pair(&mut g, &a, &b, [2; 32]), Outcome::GermPending));
}
let quoted = crate::synthesis::quote(&k, [1u8; 32], &[]).expect("quote");
assert!(!quoted.priced.is_empty(), "an empty atlas reads the book at depth zero");
}

/// THE SENTENCE IS TRANSPOSITION-INVARIANT (the win over lookup): install a
/// rotation, then the SAME rotation FAR AWAY is the same sentence (1.5: a
/// body's placement is quotiented from its form) and is a use of the held
/// word, no re-mint — a lookup on the coordinate-exact word would have
/// missed it. A pure translate is a rule of its own under 20 as ruled —
/// the form carries the carriage — so the chain-era claim that it "carries
/// no substance" is not made here.
/// A second vantage here is `glue_local`'s two fixture keys (§3.4's
/// wire-identifier exception): this test proves nothing about 47.3's witness.
#[test]
fn reduction_is_transposition_invariant() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)]; // R180 of L at origin
speak_pair(&mut k, &a, &rot, [1; 32]);
assert_eq!(speak_pair(&mut k, &a, &rot, [2; 32]), Outcome::GermPending);
k.glue_local();
// the SAME rotation, transposed +10 in col: the same sentence.
let a_far = vec![(0, 10, 1), (0, 11, 1), (1, 10, 1)];
let rot_far = vec![(0, 11, 1), (1, 10, 1), (1, 11, 1)];
assert_eq!(speak_pair(&mut k, &a_far, &rot_far, [3; 32]), Outcome::Decomposed(vec![k.basis()[0].address()]), "transposed → the same sentence");
assert_eq!(k.basis_len(), 1, "transposition mints nothing");
}

/// THE SYNTROPIC STROKE CONSOLIDATES: a generator the rest of the basis already
/// derives (a stale duplicate — what an import or a pre-consolidation era
/// leaves) is retired by the redundancy probe; an essential one survives it.
#[test]
fn the_syntropic_stroke_retires_a_redundant_generator() {
use crate::transducer::germinate::germinate;
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
k.basis_insert(element::from_canonical(&germinate(&a, &rot).unwrap()), [1; 32]);
let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
let rot_far: Vec<_> = rot.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
k.basis_insert(element::from_canonical_framed(&germinate(&a_far, &rot_far).unwrap(), &crate::arc::Frame::new(4, 4, 8, 8)), [7; 32]);
assert_eq!(k.basis_len(), 2);
k.optimize();
assert_eq!(k.basis_len(), 1, "the redundant rule is consolidated away");
k.optimize();
assert_eq!(k.basis_len(), 1, "an essential generator survives the probe");
}

/// The survivor of refine_stratum may sit at the last index. Removing an
/// earlier derivable sibling used to leave that index equal to len.
#[test]
fn refine_stratum_does_not_panic_when_the_survivor_is_last() {
use crate::transducer::element::{from_canonical, from_canonical_framed};
use crate::transducer::germinate::germinate;
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
k.basis_insert(from_canonical(&germinate(&a, &rot).unwrap()), [1; 32]);
let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
let rot_far: Vec<_> = rot.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
k.basis_insert(
    from_canonical_framed(&germinate(&a_far, &rot_far).unwrap(), &crate::arc::Frame::new(4, 4, 8, 8)),
    [7; 32],
);
k.basis_insert(
    from_canonical_framed(&germinate(&a_far, &rot_far).unwrap(), &crate::arc::Frame::new(3, 3, 6, 6)),
    [8; 32],
);
assert!(k.basis_len() >= 2, "two distinct addresses remain");
let survivor = k.basis()[k.basis_len() - 1].address();
k.refine_stratum(&survivor);
assert!(k.word_at(&survivor).is_some(), "the last card stays keyed");
assert!(k.basis_len() >= 1);
assert!(k.basis_len() < 3, "at least one earlier duplicate retired");
}

/// Restore keeps the lettered federation and drops a leaked basis card.
/// A second vantage here is `glue_pair`'s two fixture keys (§3.4's
/// wire-identifier exception): this test proves nothing about 47.3's witness.
#[test]
fn prune_drops_the_leaked_basis_and_keeps_the_ruled_cover() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
glue_pair(&mut k, &a, &b);
let kept = k.basis_len();
assert!(kept >= 1);
let leak_a = vec![(9, 0, 2), (9, 1, 2)];
let leak_b = vec![(9, 0, 3), (9, 1, 3)];
let leaked = crate::transducer::element::from_canonical(&germinate(&leak_a, &leak_b).unwrap());
k.leak_unruled(leaked);
assert_eq!(k.basis_len(), kept + 1);
let dropped = k.prune_basis_to_ruled();
assert_eq!(dropped, 1, "the leaked card leaves");
assert_eq!(k.basis_len(), kept, "the ruled cover remains");
k.leak_unruled(crate::transducer::element::from_canonical(
    &germinate(&[(8, 0, 2), (8, 1, 2)], &[(8, 0, 5), (8, 1, 5)]).unwrap(),
));
let snap = k.snapshot();
let mut booted = Kernel::new();
assert!(booted.restore(&snap));
assert_eq!(booted.basis_len(), kept, "restore skips the leaked basis and rebuilds from the federation");
}

/// An append shares the prior word's Arc; it does not clone the basis.
#[test]
fn language_append_shares_prior_word_arcs() {
let mut k = Kernel::new();
let w0 = crate::transducer::element::from_canonical(
    &germinate(&[(0, 0, 1), (0, 1, 1), (1, 0, 1)], &[(0, 1, 1), (1, 0, 1), (1, 1, 1)]).unwrap(),
);
let w1 = crate::transducer::element::from_canonical(
    &germinate(&[(4, 0, 2), (4, 1, 2), (5, 0, 2)], &[(4, 0, 3), (4, 1, 3), (5, 0, 3)]).unwrap(),
);
k.leak_unruled(w0);
let first = k.language();
assert_eq!(first.words.len(), 1);
let w0_addr = first.words().next().expect("one word").address();
let p0 = std::sync::Arc::as_ptr(&first.words.get(w0_addr.as_bytes()).expect("at its address").held);
assert!(
    std::sync::Arc::ptr_eq(&first, &k.language()),
    "a stable language is shared by pointer"
);
k.leak_unruled(w1);
let second = k.language();
assert_eq!(second.words.len(), 2);
assert_eq!(
    std::sync::Arc::as_ptr(&second.words.get(w0_addr.as_bytes()).expect("still at its address").held),
    p0,
    "the first word is the same Arc, not a cloned element"
);
}

/// Dialect/Slashed addresses are heard once; a cocycle row is unique
/// on (chart, pair, versor). The forged word is a spoken sentence with its
/// word vertices struck out — a tape of the Frame and a demand, which the
/// blind replay refuses as a chain (50.6, `REFUSE_NOT_A_SENTENCE`).
#[test]
fn heard_and_cocycles_do_not_re_row() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1)];
// The bar recoloured, a site painted beside it (50.5). H1b, 70.4 —
// separated: at `(0, 2)` the site is one hue-blind piece with the bar.
let b = vec![(0, 0, 2), (0, 1, 2), (0, 3, 3)];
speak_pair(&mut k, &a, &b, [1; 32]);
speak_pair(&mut k, &a, &b, [2; 32]);
let spoken = k.provisional.scoped().next().unwrap().1.clone();
let forged = strip_words(&spoken);
let mut cover = crate::network::nerve::Cover::new();
assert!(matches!(k.glue_fetched(forged.clone(), [2; 32], [1; 32], &mut cover), Glue::Slashed(_)));
// The simplices the complexes hold, each by its own key: a row that is not
// re-filed moves none of them.
let simplices = |k: &Kernel| k.complexes.scoped().map(|(_, c)| c.by_key.len()).sum::<usize>();
let n = simplices(&k);
assert!(k.heard.contains(&forged.address()));
assert!(matches!(k.glue_fetched(forged, [2; 32], [1; 32], &mut cover), Glue::Dialect));
assert_eq!(simplices(&k), n, "a heard slash is not re-rowed");
}

/// Quotes are incidence on charts. Permuting the pending list does not
/// change the quoted addresses; Rank carries no scalar order.
#[test]
fn the_book_has_no_order() {
let one = |links: &[(i64, i64, u8, u8)]| {
    element::from_canonical(
        &element::canonical_chain(links).expect("one entry per position is A5's chain"),
    )
};
let e1 = one(&[(0, 0, 1, 2)]);
let e2 = one(&[(1, 0, 1, 3)]);
let e3 = one(&[(2, 0, 1, 4)]);
let keyed = |es: &[Element]| {
    let mut k = crate::network::nerve::Keyed::new();
    for e in es {
        k.insert(e.address(), e.clone());
    }
    k
};
let pending_a = keyed(&[e1.clone(), e2.clone(), e3.clone()]);
let pending_b = keyed(&[e3.clone(), e1.clone(), e2.clone()]);
let atlas = crate::transducer::stalk::Stalks::over_elements([&e1, &e2, &e3].into_iter());
let field = crate::network::nerve::Keyed::new();
let mut on_page = crate::network::nerve::Keyed::new();
for e in [&e1, &e2, &e3] {
    on_page.insert(e.address(), ());
}
let spoken = crate::network::nerve::Keyed::new();
let standing = crate::network::nerve::Keyed::new();
let quote = |p: &crate::network::nerve::Keyed<Element>| {
    quote_incidence(
        &BookView {
            pending: p,
            listing_atlas: &atlas,
            field: &field,
            on_page: &on_page,
            spoken: &spoken,
            standing_t: &standing,
        },
        &Ask { charts: &[], surface: &[0; 4], pressure: &[0; 4], budget: 10_000, voice: &[0; 32] },
    )
};
let (_, pa) = quote(&pending_a);
let (_, pb) = quote(&pending_b);
let addrs = |p: &[Priced]| p.iter().map(|i| i.word.address()).collect::<Vec<_>>();
assert_eq!(
    addrs(&pa),
    addrs(&pb),
    "quote is atlas incidence, equal under any permutation of pending"
);
assert!(!pa.is_empty(), "the book is served");
}

/// heard drops when the peer's stratum root moves. The forged word is a
/// sentence with its word vertices struck out, as in the test above.
#[test]
fn heard_drops_when_stratum_root_moves() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1)];
// Separated, as above (H1b, 70.4).
let b = vec![(0, 0, 2), (0, 1, 2), (0, 3, 3)];
speak_pair(&mut k, &a, &b, [1; 32]);
speak_pair(&mut k, &a, &b, [2; 32]);
let spoken = k.provisional.scoped().next().unwrap().1.clone();
let forged = strip_words(&spoken);
let mut cover = crate::network::nerve::Cover::new();
assert!(matches!(k.glue_fetched(forged.clone(), [2; 32], [1; 32], &mut cover), Glue::Slashed(_)));
assert_eq!(k.heard_len(), 1);
let inv = crate::transducer::stalk::invariant_of(&forged);
let mut moved = [0u8; 32];
moved[0] = 7;
k.confirm_peer_holds(&[crate::transducer::sheaf::StratumDigest {
    inv,
    count: 0,
    root: moved.to_vec(),
}], [2; 32]);
assert_eq!(k.heard_len(), 0, "a moved stratum root drops heard");
}

/// G3 (R6, 47.3). `listings_on` is the open listings on the simplex's
/// chart. A simplex's vertices are vantages — hyphas — and no speaker's
/// index gates what its chart holds: read as the meet of a voice-keyed
/// index over the simplex's vertices, every hypha vertex measures zero and
/// nothing fills although the chart holds two sections.
#[test]
fn listings_on_is_the_open_listings_on_the_chart() {
use crate::transducer::address::Address;
let mut k = Kernel::new();
let inv = crate::transducer::stalk::Invariant::counts(&[7]);
let (v1, v2) = ([1u8; 32], [2u8; 32]);
let both = Address::from(&b"both"[..]);
let only_v1 = Address::from(&b"only_v1"[..]);
let shut = Address::from(&b"shut"[..]);
let off_chart = Address::from(&b"off_chart"[..]);
for a in [&both, &only_v1, &shut] {
    k.chart_listings.nested_insert(inv.key(), a.as_bytes(), ());
}
for a in [&both, &only_v1, &shut, &off_chart] {
    k.open.insert(a.as_bytes(), ());
}
k.open.remove(shut.as_bytes());
k.record_spoken(both.clone(), v1);
k.record_spoken(both.clone(), v2);
k.record_spoken(only_v1.clone(), v1);

let held = k.listings_on(&inv);
assert!(held.contains(both.as_bytes()), "open, on the chart");
assert!(held.contains(only_v1.as_bytes()), "spoken by one voice, and on the chart all the same");
assert!(!held.contains(shut.as_bytes()), "a closed listing is not held");
assert!(!held.contains(off_chart.as_bytes()), "a listing off the chart is not held");
assert_eq!(held.len(), 2, "exactly the open, on-chart listings");
assert_eq!(k.listings_on(&crate::transducer::stalk::Invariant::counts(&[8])).len(), 0, "a chart with no listings holds nothing");
}

/// The genome boundary. A taught word is CRYSTALLIZED on the receiver — it
/// enters the genome, the persisted stratum (24.1, chapter 6) — while the
/// epigenome's dialect is the transmutable one. `word_at` resolves for
/// replay and falls through basis → cavity → provisional on purpose; the
/// teaching path must not, or a voice is handed its own unratified dialect
/// back as language and files it as crystallized intelligence.
#[test]
fn the_syllabus_never_teaches_a_provisional_word() {
use crate::transducer::element::from_canonical;
use crate::transducer::germinate::germinate;
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 2)];
let b = vec![(0, 0, 5), (0, 1, 2)];
let w = from_canonical(&germinate(&a, &b).expect("a coboundary"));
let addr = w.address();
let inv = crate::transducer::stalk::invariant_of(&w);

// The word stands only as provisional, and its address reaches the
// ruled cover: exactly the state the soak found, basis 0 with a
// syllabus serving.
k.provisional.insert(&addr, w.clone());
k.ruled.admit(&inv, addr.clone());
assert!(k.word_at(&addr).is_some(), "replay still resolves it");
assert!(k.installed_word(&addr).is_none(), "but nothing installed it");

let served = k.serve_ruled(&inv, &[], 1 << 20);
assert!(served.is_empty(), "a provisional word is not language and is not taught");

// Installed, the same address teaches — at the stamp of the act that
// admitted it (78.24 (2b)), the locator the page carries beside it.
k.basis_insert(w, [1; 32]);
assert!(k.installed_word(&addr).is_some());
assert!(k.serve_ruled(&inv, &[], 1 << 20).is_empty(), "a word no act admitted has no locator and is not taught");
k.file_admitted(&addr, crate::chronometry::Stamp { h: 1, vk: [1; 32], k: 1, i: 0 });
assert_eq!(
    k.serve_ruled(&inv, &[], 1 << 20).len(),
    1,
    "what the court installed is what it teaches"
);
}

/// X3, rule 1 — THE COHESION READING IS CARRIED, NOT READ WHOLE. C and I are
/// written when a listing's closure is bound (`Rank::put_closure`), for the
/// same reason `ListingField::nonflat` is written by `write_field`: a read
/// that counts what the write already knew reads the whole of a carrier that
/// grows with the book. `cohesion_over` read the whole closure map per call, and
/// `synthesis`'s rank line read it a second time, with its own filter, to
/// print one of the two numbers.
///
/// Pinned against the walk itself, so the carried counts cannot drift from
/// the carrier: whatever the book holds, the two readings agree — including
/// after a listing is re-bound at a new closure, and across a restore.
#[test]
fn the_cohesion_counts_are_carried_and_agree_with_the_carrier() {
let walk = |r: &Rank| {
    let mut c = 0usize;
    let mut i = 0usize;
    for (_, closed) in r.closure_carrier().scoped() {
        match closed {
            Some(_) => c += 1,
            None => i += 1,
        }
    }
    (c, i)
};
let mut k = Kernel::new();
assert_eq!(k.closure_now().counts(), (0, 0), "an empty book has neither");

// Two voices on one pair list it; a third voice lists another.
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
speak_pair(&mut k, &a, &rot, [1; 32]);
speak_pair(&mut k, &a, &rot, [2; 32]);
let c = vec![(0, 0, 3), (0, 1, 4)];
let d = vec![(0, 0, 4), (0, 1, 3)];
speak_pair(&mut k, &c, &d, [3; 32]);
let rank = k.closure_now();
assert_eq!(rank.counts(), walk(&rank), "the carried counts are the carrier's");
assert_eq!(
    rank.counts().0 + rank.counts().1,
    rank.closure_len(),
    "every measured listing is in one of the two"
);
assert_eq!(
    cohesion_over(&rank),
    (rank.counts().0 as u64, rank.counts().1 as u64),
    "and the named reading is those two"
);

// A word installed closes what it derives: a listing is re-bound at a base
// point, and the counts move with it rather than double-count.
k.glue_local();
let rank = k.closure_now();
assert_eq!(rank.counts(), walk(&rank), "after a rebind, still the carrier's");
assert_eq!(rank.counts().0 + rank.counts().1, rank.closure_len());

// And across the ledger: the restore binds every closure through the same
// door, so a court that boots reports what it reported before.
let before = k.closure_now().counts();
let snap = k.snapshot();
let mut k2 = Kernel::new();
assert!(k2.restore(&snap), "the ledger restores");
let back = k2.closure_now();
assert_eq!(back.counts(), walk(&back), "restored, the counts are the carrier's");
assert_eq!(back.counts(), before, "and are what was written");
}
