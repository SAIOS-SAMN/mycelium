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
assert_eq!(speak_pair(&mut k, &a_far, &rot_far, [3; 32]), Outcome::Decomposed(vec![0]), "transposed → the same sentence");
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
k.basis_insert(element::from_residual(&germinate(&a, &rot).unwrap()), [1; 32]);
let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
let rot_far: Vec<_> = rot.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
k.basis_insert(element::from_residual_framed(&germinate(&a_far, &rot_far).unwrap(), &crate::arc::Frame::new(4, 4, 8, 8)), [7; 32]);
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
use crate::transducer::element::{from_residual, from_residual_framed};
use crate::transducer::germinate::germinate;
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
k.basis_insert(from_residual(&germinate(&a, &rot).unwrap()), [1; 32]);
let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
let rot_far: Vec<_> = rot.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
k.basis_insert(
    from_residual_framed(&germinate(&a_far, &rot_far).unwrap(), &crate::arc::Frame::new(4, 4, 8, 8)),
    [7; 32],
);
k.basis_insert(
    from_residual_framed(&germinate(&a_far, &rot_far).unwrap(), &crate::arc::Frame::new(3, 3, 6, 6)),
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
let leaked = crate::transducer::element::from_residual(&germinate(&leak_a, &leak_b).unwrap());
k.leak_unruled(leaked);
assert_eq!(k.basis_len(), kept + 1);
let dropped = k.prune_basis_to_ruled();
assert_eq!(dropped, 1, "the leaked card leaves");
assert_eq!(k.basis_len(), kept, "the ruled cover remains");
k.leak_unruled(crate::transducer::element::from_residual(
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
let w0 = crate::transducer::element::from_residual(
    &germinate(&[(0, 0, 1), (0, 1, 1), (1, 0, 1)], &[(0, 1, 1), (1, 0, 1), (1, 1, 1)]).unwrap(),
);
let w1 = crate::transducer::element::from_residual(
    &germinate(&[(4, 0, 2), (4, 1, 2), (5, 0, 2)], &[(4, 0, 3), (4, 1, 3), (5, 0, 3)]).unwrap(),
);
k.leak_unruled(w0);
let first = k.language();
assert_eq!(first.words.len(), 1);
let p0 = std::sync::Arc::as_ptr(&first.words[0]);
assert!(
    std::sync::Arc::ptr_eq(&first, &k.language()),
    "a stable language is shared by pointer"
);
k.leak_unruled(w1);
let second = k.language();
assert_eq!(second.words.len(), 2);
assert_eq!(
    std::sync::Arc::as_ptr(&second.words[0]),
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
let b = vec![(0, 0, 2), (0, 1, 2), (0, 2, 3)]; // the bar recoloured, a cell created beside it (50.5)
speak_pair(&mut k, &a, &b, [1; 32]);
speak_pair(&mut k, &a, &b, [2; 32]);
let spoken = k.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().1.clone();
let forged = strip_words(&spoken);
let mut cover = crate::network::nerve::Cover::new();
assert!(matches!(k.glue_fetched(forged.clone(), [2; 32], [1; 32], &mut cover), Glue::Slashed(_)));
let n = k.cochain_bytes();
assert!(k.heard.contains(&forged.address()));
assert!(matches!(k.glue_fetched(forged, [2; 32], [1; 32], &mut cover), Glue::Dialect));
assert_eq!(k.cochain_bytes(), n, "a heard slash is not re-rowed");
}

/// Quotes are incidence on charts. Permuting the pending list does not
/// change the quoted addresses; Rank carries no scalar order.
#[test]
fn the_book_has_no_order() {
let e1 = element::from_residual(&vec![(0, 0, 1, 2)]);
let e2 = element::from_residual(&vec![(1, 0, 1, 3)]);
let e3 = element::from_residual(&vec![(2, 0, 1, 4)]);
let keyed = |es: &[Element]| {
    let mut k = crate::network::nerve::Keyed::new();
    for e in es {
        k.insert(e.address(), (e.clone(), 0u64));
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
let quote = |p: &crate::network::nerve::Keyed<(Element, u64)>| {
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
let b = vec![(0, 0, 2), (0, 1, 2), (0, 2, 3)]; // the bar recoloured, a cell created beside it (50.5)
speak_pair(&mut k, &a, &b, [1; 32]);
speak_pair(&mut k, &a, &b, [2; 32]);
let spoken = k.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().1.clone();
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

/// G3 (R6, 47.3). `listings_on` is the open listings on the cell's chart.
/// A cell's vertices are vantages — hyphas — and no speaker's index gates
/// what its chart holds: read as the meet of `spoken_by` over the cell's
/// vertices, keyed by voice, every hypha vertex measured zero and nothing
/// ever filled although charts held two sections.
#[test]
fn listings_on_is_the_open_listings_on_the_chart() {
use crate::transducer::address::Address;
let mut k = Kernel::new();
let inv = crate::transducer::stalk::Invariant(vec![7]);
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
assert_eq!(k.listings_on(&crate::transducer::stalk::Invariant(vec![8])).len(), 0, "a chart with no listings holds nothing");
}

/// The genome boundary. A taught word is CRYSTALLIZED on the receiver — it
/// enters the genome, the persisted stratum (24.1, chapter 6) — while the
/// epigenome's dialect is the transmutable one. `word_at` resolves for
/// replay and falls through basis → cavity → provisional on purpose; the
/// teaching path must not, or a voice is handed its own unratified dialect
/// back as language and files it as crystallized intelligence.
#[test]
fn the_syllabus_never_teaches_a_provisional_word() {
use crate::transducer::element::from_residual;
use crate::transducer::germinate::germinate;
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 2)];
let b = vec![(0, 0, 5), (0, 1, 2)];
let w = from_residual(&germinate(&a, &b).expect("a coboundary"));
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

// Installed, the same address teaches.
k.basis_insert(w, [1; 32]);
assert!(k.installed_word(&addr).is_some());
assert_eq!(
    k.serve_ruled(&inv, &[], 1 << 20).len(),
    1,
    "what the court installed is what it teaches"
);
}
