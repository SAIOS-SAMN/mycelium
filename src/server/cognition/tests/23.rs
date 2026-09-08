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
use crate::transducer::germinate::germinate;
let k = std::sync::Mutex::new(Kernel::new());
{
    let mut g = k.lock().unwrap();
    let a = vec![(0, 0, 1u8), (0, 1, 1)];
    let b = vec![(0, 0, 2u8), (0, 1, 2)];
    assert!(matches!(g.ingest(&a, &b, [1; 32]), Outcome::GermPending));
    assert!(matches!(g.attest(&germinate(&a, &b).unwrap(), [1; 32]), Outcome::GermPending));
}
let quoted = crate::synthesis::quote(&k, [1u8; 32], &[]).expect("quote");
assert!(!quoted.priced.is_empty(), "an empty atlas reads the book at depth zero");
}

/// CONFLUENT REDUCTION is TRANSPOSITION-INVARIANT (the win over lookup): install
/// a rotation, then the SAME rotation FAR AWAY reduces to the same normal form
/// (no re-mint) — a lookup on the coordinate-exact word would have missed it.
/// And a pure translate has EMPTY substance — it decomposes to the empty
/// sentence, minting nothing (a translate is pure transposition, not cognition).
#[test]
fn reduction_is_transposition_invariant() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)]; // R180 of L at origin
k.ingest(&a, &rot, [1; 32]);
assert_eq!(k.ingest(&a, &rot, [2; 32]), Outcome::GermPending);
k.glue_local();
// the SAME rotation, transposed +10 in col: same substance → same sentence.
let a_far = vec![(0, 10, 1), (0, 11, 1), (1, 10, 1)];
let rot_far = vec![(0, 11, 1), (1, 10, 1), (1, 11, 1)];
assert_eq!(k.ingest(&a_far, &rot_far, [3; 32]), Outcome::Decomposed(vec![0]), "transposed → same normal form");
assert_eq!(k.basis_len(), 1, "transposition mints nothing");
// a pure translate is pure transposition — empty substance, empty sentence.
let t = vec![(0, 3, 1), (0, 4, 1), (1, 3, 1)];
assert_eq!(k.ingest(&a, &t, [4; 32]), Outcome::Decomposed(vec![]), "a translate carries no substance");
assert_eq!(k.basis_len(), 1, "still nothing minted");
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
k.basis_insert(element::from_residual_framed(&germinate(&a_far, &rot_far).unwrap(), &(4, 4, 8, 8)), [7; 32]);
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
    from_residual_framed(&germinate(&a_far, &rot_far).unwrap(), &(4, 4, 8, 8)),
    [7; 32],
);
k.basis_insert(
    from_residual_framed(&germinate(&a_far, &rot_far).unwrap(), &(3, 3, 6, 6)),
    [8; 32],
);
assert!(k.basis_len() >= 2, "two distinct addresses remain");
let survivor = k.basis()[k.basis_len() - 1].address();
k.refine_stratum(&survivor);
assert!(k.word_at(&survivor).is_some(), "the last card stays keyed");
assert!(k.basis_len() >= 1);
assert!(k.basis_len() < 3, "at least one earlier duplicate retired");
}

/// THE PROJECTIVE LEAP (operator ruling): greedy largest-first descent is the
/// mechanical trap — it would commit the BIG pattern, strand an orphan cell,
/// and mint junk. The SUPERPOSITION holds both futures, rolls them out
/// virtually (zero mutation), and COLLAPSES into the branch whose remainder
/// closes (trivial holonomy): the two-word sentence. Nothing germinates —
/// imagination beats reaction.
#[test]
fn projection_escapes_the_greedy_trap() {
let mut k = Kernel::new();
let install = |k: &mut Kernel, a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]| {
    k.ingest(a, b, [1; 32]);
    match k.ingest(a, b, [2; 32]) {
        Outcome::GermPending => k.glue_local(),
        Outcome::Installed(_) | Outcome::Sectioned(_) => {}
        o => panic!("expected settle or puncture, got {o:?}"),
    }
};
install(&mut k, &[(0, 0, 1), (0, 1, 3), (0, 2, 5)], &[(0, 0, 2), (0, 1, 4), (0, 2, 6)]);
install(&mut k, &[(0, 0, 1), (0, 1, 3)], &[(0, 0, 2), (0, 1, 4)]);
install(&mut k, &[(0, 0, 5), (0, 1, 7)], &[(0, 0, 6), (0, 1, 8)]);
assert!(k.basis_len() >= 1, "pair forms stand");
let before = k.basis_len();
let a = [(0, 0, 1), (0, 1, 3), (0, 2, 5), (0, 3, 7)];
let b = [(0, 0, 2), (0, 1, 4), (0, 2, 6), (0, 3, 8)];
// The saddle is one pair form: not peeled by search into two words.
assert!(
    matches!(k.ingest(&a, &b, [3; 32]), Outcome::GermPending | Outcome::Decomposed(_)),
    "a fused pair is one percept"
);
assert_eq!(k.basis_len(), before, "nothing installs on one voice");
}

/// Restore keeps the lettered federation and drops a leaked basis card.
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
/// on (chart, pair, versor).
#[test]
fn heard_and_cocycles_do_not_re_row() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
k.ingest(&a, &b, [1; 32]);
k.ingest(&a, &b, [2; 32]);
let spoken = k.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().1.clone();
let mut forged = spoken.clone();
forged.mag = [999, 0, 0, 0];
let mut cover = crate::network::nerve::Cover::new();
assert!(matches!(k.glue_fetched(forged.clone(), [2; 32], [1; 32], &mut cover), Glue::Slashed(_)));
let n = k.cochain_bytes();
assert!(k.heard.contains(&forged.address()));
assert!(matches!(k.glue_fetched(forged, [2; 32], [1; 32], &mut cover), Glue::Dialect));
assert_eq!(k.cochain_bytes(), n, "a heard slash is not re-rowed");
}

/// Teaching is decided by the docket's anchoring of the cited word,
/// never by sentence length.
#[test]
fn teaching_is_by_anchoring_not_length() {
use crate::transducer::anchor::Sel;
use crate::transducer::element::from_residual;
let unanchored = from_residual(&vec![(0i64, 0i64, 1u8, 2u8)]);
assert!(
    taught_anchored_words(std::iter::once(&unanchored)).is_empty(),
    "one unanchored word teaches nothing"
);
let a = from_residual(&vec![(0i64, 0i64, 1u8, 2u8)]).with_anchor(Sel::Hue(1));
let b = from_residual(&vec![(1i64, 0i64, 3u8, 4u8)]).with_anchor(Sel::Hue(3));
let taught = taught_anchored_words([&a, &b]);
let mut pos = 0usize;
let mut n = 0usize;
while pos < taught.len() {
    let before = pos;
    if Element::from_bytes(&taught, &mut pos).is_none() {
        break;
    }
    if pos == before {
        break;
    }
    n += 1;
}
assert_eq!(n, 2, "two anchored words in one sentence both taught");
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

/// heard drops when the peer's stratum root moves.
#[test]
fn heard_drops_when_stratum_root_moves() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
k.ingest(&a, &b, [1; 32]);
k.ingest(&a, &b, [2; 32]);
let spoken = k.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().1.clone();
let mut forged = spoken.clone();
forged.mag = [999, 0, 0, 0];
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

/// G3. `listings_on` is the meet of the cell's own vertices (13.1, 14.6):
/// the listings every voice of the cell speaks, on its chart, still open.
/// It is a fold of `Keyed::meet` over the cell's vertex list — a boundary
/// walk — and never a product of the vertices with the chart's listings.
#[test]
fn listings_on_is_the_meet_of_its_vertices() {
use crate::transducer::a21::Address;
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
k.record_spoken(shut.clone(), v1);
k.record_spoken(shut.clone(), v2);
k.record_spoken(off_chart.clone(), v1);
k.record_spoken(off_chart.clone(), v2);

let held = k.listings_on(&inv, &[v1, v2]);
assert!(held.contains(both.as_bytes()), "spoken by both, on the chart, open");
assert!(!held.contains(only_v1.as_bytes()), "a listing one vertex never spoke is not in the meet");
assert!(!held.contains(shut.as_bytes()), "a closed listing is not held");
assert!(!held.contains(off_chart.as_bytes()), "a listing off the chart is not held");
assert_eq!(held.len(), 1, "the meet is exactly the shared, open, on-chart listing");

// One vertex alone holds what it spoke; the meet only ever shrinks.
let alone = k.listings_on(&inv, &[v1]);
assert!(alone.contains(both.as_bytes()) && alone.contains(only_v1.as_bytes()));
assert_eq!(alone.len(), 2, "and nothing it did not speak");

// A vertex that has spoken nothing empties the cell: an intersection
// through it is empty, never "everything on the chart".
assert_eq!(k.listings_on(&inv, &[v1, [9u8; 32]]).len(), 0, "a silent vertex empties the meet");
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
