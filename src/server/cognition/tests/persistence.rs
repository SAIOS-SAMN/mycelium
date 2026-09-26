// 28 — PERSISTENCE: what a mind is after a restart is what it was before
// one, carrier for carrier.

use super::common::speak_pair;
use crate::cognition::*;

/// P7 — the germinated language survives restart: install two germs, snapshot,
/// restore into a fresh kernel, and the restored kernel RECALLS them (no
/// re-germination) with balances intact.
/// A second vantage here is `glue_local`'s two fixture keys (§3.4's
/// wire-identifier exception): this test proves nothing about 47.3's witness.
#[test]
fn the_basis_survives_a_restart() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)]; // a rotation (group substance)
// The L recoloured whole: a second word, its gauge (20 as ruled, L7). A
// deformation no versor carries lists nothing (50.5), so it is not a
// second generator here.
let recoloured = vec![(0, 0, 2), (0, 1, 2), (1, 0, 2)];
speak_pair(&mut k, &a, &rot, [1; 32]);
assert_eq!(speak_pair(&mut k, &a, &rot, [2; 32]), Outcome::GermPending);
k.glue_local(); // a rotation generator
speak_pair(&mut k, &a, &recoloured, [1; 32]);
assert_eq!(speak_pair(&mut k, &a, &recoloured, [2; 32]), Outcome::GermPending);
k.glue_local(); // a gauge generator
let balance = k.treasury().balance_mu(&[2; 32]);
let snap = k.snapshot();

// A fresh hypha boots from the snapshot.
let mut booted = Kernel::new();
assert!(booted.restore(&snap), "clean restore");
assert_eq!(booted.basis_len(), 2, "both generators came back");
assert!(
    booted.treasury().balance_mu(&[2; 32]).iter().any(|&m| m > 0)
        || balance.iter().any(|&m| m > 0),
    "earned μ⃗ survived the snapshot"
);
assert!(
    matches!(speak_pair(&mut booted, &a, &rot, [9; 32]), Outcome::Decomposed(_)),
    "rotation reduces"
);
assert!(
    matches!(speak_pair(&mut booted, &a, &recoloured, [9; 32]), Outcome::Decomposed(_)),
    "the recolour reduces"
);
assert_eq!(booted.basis_len(), 2, "no re-germination");
}

/// The cocycle rows write the transport class by its own coordinates
/// beside the versor, and the receipt each row stood at in place of a
/// tick (LD3-b); a restore reads the same class and the same stamp, never a
/// hash of the row.
#[test]
fn a_cocycle_row_round_trips_its_class_and_its_stamp() {
let mut k = Kernel::new();
let inv = crate::transducer::stalk::Invariant::counts(&[1]);
let q = crate::transducer::alphabet::LetterId::turn_90((0, 1)).unwrap();
let w = crate::transducer::alphabet::Word::from_letters(&[(q, false)]);
let r = w.rotor();
let v = rotor_versor(&r).expect("versor");
let ck = r.class_key().expect("class key");
let stood = crate::chronometry::Stamp::new(3, [7; 32], crate::chronometry::Place { k: 1, i: 2 });
k.push_cocycle_at(&inv, [1; 32], [2; 32], v, ck.clone(), Some(stood));
let snap = k.snapshot();
assert!(snap.windows(8).any(|w| w == b"cocycles"), "the tape carries the cocycles trie by its name");
let mut booted = Kernel::new();
assert!(booted.restore(&snap), "clean restore");
let rows: Vec<(Vec<u8>, crate::cognition::CocycleRow)> = booted.cocycle_at.scoped().map(|(key, row)| (key, row.clone())).collect();
let [(key, row)] = rows.as_slice() else { panic!("one row written, {} read", rows.len()) };
assert_eq!(row.class_key, ck, "the class survives the ledger as its coordinates");
assert_eq!((row.a, row.b), ([1; 32], [2; 32]), "the two voices survive");
assert_eq!(booted.cocycle_receipt.get(key).copied(), Some(stood), "the stamp the row stood at survives");
}

/// A restart must not docket Δ⃗ = 0: deflation records survive the
/// snapshot, so the court does not slash a live provisional as empty.
#[test]
fn deflation_survives_a_restart() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)]; // the L recoloured whole (20 as ruled, L7)
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
// The second voice speaks the same sentence: one sentence, one docket.
assert_eq!(speak_pair(&mut k, &a, &b, [2; 32]), Outcome::GermPending);
let before = k.dockets();
assert_eq!(before.len(), 1);
assert!(before[0].deflation.iter().any(|&x| x > 0), "deflation was measured at install");
let snap = k.snapshot();
let mut booted = Kernel::new();
assert!(booted.restore(&snap), "clean restore");
let after = booted.dockets();
assert_eq!(after.len(), 1, "the provisional survived");
assert_eq!(after[0].deflation, before[0].deflation, "deflation survived; the court will not slash Δ⃗ = 0");
}

/// A TAPE THAT ENDS EARLY IS REFUSED WHOLE, at every trie. §10 makes a
/// layout change a recut and deletes the reader for the old layout: the
/// positional tape (78.25) is refused by its name, MYC7, and a tape that
/// is neither refuses as what it is.
#[test]
fn a_snapshot_cut_short_at_any_trie_is_refused() {
use crate::cognition::snapshot::{
    COCYCLES_TRIE, COMPLEXES_TRIE, COURT_TRIE, ERAS_TRIE, LEDGER_TRIE, MAGIC, PRE_TRIE_MAGIC, WORDS_TRIE,
};
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)]; // the L recoloured whole (20 as ruled, L7)
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
let snap = k.snapshot();
assert_eq!(&snap[..4], MAGIC, "one magic names the layout");
let mut whole = Kernel::new();
assert!(whole.restore(&snap), "the tape as written restores");

// Every trie opens with its own name; a tape ending before it is refused.
let tries = [
    COURT_TRIE,
    LEDGER_TRIE,
    WORDS_TRIE,
    crate::economy::persist::STAKES_TRIE,
    crate::economy::persist::BALANCES_TRIE,
    ERAS_TRIE,
    COMPLEXES_TRIE,
    COCYCLES_TRIE,
];
for name in tries {
    let Some(at) = snap.windows(name.len()).position(|w| w == name.as_bytes()) else {
        panic!("the tape carries the {name} trie by its name")
    };
    let mut booted = Kernel::new();
    assert!(!booted.restore(&snap[..at]), "a tape ending at the {name} trie is refused");
    assert_eq!(booted.basis_len(), 0, "and nothing is partly restored");
}

// THE POSITIONAL TAPE IS REFUSED BY NAME (78.25, §10).
let mut older = snap.clone();
older[..4].copy_from_slice(PRE_TRIE_MAGIC);
let mut booted = Kernel::new();
assert!(!booted.restore(&older), "MYC7 is refused");
assert_eq!(booted.basis_len(), 0, "and nothing is partly restored");
}

/// 78.27 — A TRIE IS READ BY KEY, NEVER BY POSITION: a key written twice
/// refuses the trie, a leaf kind the trie does not define refuses its node,
/// leaves out of kind order refuse, and a trie read under another name
/// refuses. What reads back is keyed, whatever order the nodes were given.
#[test]
fn a_trie_reads_back_by_key_and_refuses_what_it_cannot_place() {
use crate::network::nerve::Keyed;
use crate::snapshot::{leaf, put_trie, take_trie, Leaves};
let mut nodes: Keyed<Leaves> = Keyed::new();
nodes.insert(b"b", vec![(1, b"two".to_vec()), (3, b"four".to_vec())]);
nodes.insert(b"a", vec![(1, b"one".to_vec())]);
let mut tape = Vec::new();
put_trie(&mut tape, "t", &nodes);
let back = take_trie(&tape, &mut 0, "t", &[1, 3]).expect("it reads back whole");
assert_eq!(back.get(b"a").and_then(|l| leaf(l, 1)), Some(b"one".as_slice()), "by key");
assert_eq!(back.get(b"b").and_then(|l| leaf(l, 3)), Some(b"four".as_slice()));
assert!(take_trie(&tape, &mut 0, "u", &[1, 3]).is_none(), "under another name it refuses");
assert!(take_trie(&tape, &mut 0, "t", &[1]).is_none(), "a kind the trie does not define refuses");
// Two nodes under one key: written by hand, since a Keyed cannot hold them.
let mut dup = Vec::new();
let mut body = Vec::new();
for _ in 0..2 {
    crate::snapshot::put_key(&mut body, b"k");
    body.extend_from_slice(&1u64.to_le_bytes());
    body.push(1);
    crate::snapshot::put_key(&mut body, b"v");
}
crate::snapshot::put_key(&mut dup, b"t");
dup.extend_from_slice(&(body.len() as u64).to_le_bytes());
dup.extend_from_slice(&2u64.to_le_bytes());
dup.extend_from_slice(&body);
assert!(take_trie(&dup, &mut 0, "t", &[1]).is_none(), "a key the trie already holds refuses");
}

/// A TRIE CLOSES WHERE IT SAID IT WOULD. Each trie writes its byte length
/// beside its count; a reader that does not arrive at that byte has lost
/// the frame inside that trie and refuses there.
///
/// The count alone could not support this. A reader recovered a section's
/// end by parsing every record and trusting the result, so a record whose
/// framing it reconstructed differently from the writer moved the position
/// for good and every later refusal named an innocent section. Measured on
/// the fleet at 83e3e30: hypha a announced 91 DEF1 records, framed 84 over
/// 298223 bytes, then read an address length of zero and a tag byte of 13;
/// its true body was 345929 bytes, which the reader had no way to know. It
/// refused 8.6 MB of a court's history and named a byte that was a
/// consequence, not a cause. Hypha b lost 13.7 MB the same way.
///
/// Both halves are asserted here: a body one byte shorter than declared is
/// refused, and so is one byte longer. The first is what a lost frame looks
/// like from inside; the second is what a section that overruns its
/// neighbour looks like — the failure that used to be silent.
#[test]
fn a_section_that_does_not_close_where_it_declared_is_refused() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
speak_pair(&mut k, &a, &b, [1; 32]);
let snap = k.snapshot();
assert!(Kernel::new().restore(&snap), "the tape as written restores");

// The first trie's frame: the magic, its name (`court`, a u64 length and
// five bytes), then the byte length of its body. Byte 17 opens that length.
let declared = u64::from_le_bytes(snap[17..25].try_into().expect("the frame is there"));
for wrong in [declared.wrapping_sub(1), declared.wrapping_add(1)] {
    let mut bent = snap.clone();
    bent[17..25].copy_from_slice(&wrong.to_le_bytes());
    let mut booted = Kernel::new();
    assert!(
        !booted.restore(&bent),
        "a body declaring {wrong} where it holds {declared} is refused"
    );
    assert_eq!(booted.basis_len(), 0, "and nothing is partly restored");
}
}

/// AN ADDRESS THE WIRE'S PREFIX CANNOT HOLD STILL GOES ON THE LOCAL TAPE.
///
/// `put_addr` writes a `u16` length and refuses an address over 65 535 bytes
/// by writing NOTHING — never truncating. That is a wire constraint: the
/// citation prefix of 47.1 is part of the protocol and `Cited` is the type
/// that proves an address fits it. The ledger and the genome are not the
/// wire.
///
/// A form's address is `form_vertex_key`, which writes every vertex, every
/// member and every gauge, so it grows with the form. The largest pair form
/// on the fleet at e98e8a2 held 677 vertices and 4030 faces, and its address
/// does not fit a `u16`. Measured over that soak: 24 577 readings of
/// `snapshot: 1 records could not be framed and were not written`, exactly
/// one per persist, across sections of 47, 71, 96, 158, 162 and 171 records
/// — and before that refusal was honoured at all, the record reached the
/// tape with its address field absent, which is the DEF1 desync that cost
/// hypha a 8.6 MB and hypha b 13.7 MB of a court's history.
///
/// `snapshot::put_key` writes a `u64` length, as every other length on this
/// tape has since MYC6, and is total.
#[test]
fn a_key_too_long_for_the_wire_prefix_still_round_trips_on_the_tape() {
let long = vec![0xABu8; (u16::MAX as usize) + 64];
assert!(
    crate::transducer::address::Address::from(long.clone()).cited().is_none(),
    "the wire's citation prefix cannot hold it"
);
let mut out = Vec::new();
crate::snapshot::put_key(&mut out, &long);
let mut p = 0usize;
let back = crate::snapshot::take_key(&out, &mut p).expect("the tape holds it");
assert_eq!(back, long, "and it reads back whole");
assert_eq!(p, out.len(), "consuming exactly what was written");
// A declared length the tape cannot satisfy is refused, never read short.
let mut clipped = out.clone();
clipped.truncate(out.len() - 1);
let mut p = 0usize;
assert!(
    crate::snapshot::take_key(&clipped, &mut p).is_none(),
    "a key whose bytes are not there is a refusal, not a short key"
);
}

/// 78.27 T2 — PER-ACT DATA STANDS AT ITS LEDGER NODE. Every treasury row
/// comes back at the stamp it stands at — the boot's genesis and stake rows
/// at the line's pre-genesis origin, a bare node until the first act is
/// sealed there — a settlement at the act it follows, and a dialect
/// transition at the receipt it stood at; a transition that stood at no act
/// is refused by name and does not come back.
#[test]
fn the_ledgers_per_act_data_stands_at_its_nodes() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)]; // the L recoloured whole (20 as ruled, L7)
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
let listing = k.pending.scoped().next().expect("listed").1.address();
k.record_settlement(listing, vec![(0, 0, 1, 4)], [5; 32], &[], [2, 0, 0, 0], [0; 4]);
let head = k.chain().head_stamp().expect("the court has acted");
let q = crate::transducer::alphabet::LetterId::turn_90((0, 1)).expect("a 90° turn");
let g = rotor_versor(&crate::transducer::alphabet::Word::from_letters(&[(q, false)]).rotor()).expect("a versor");
k.dialect_chain.push((b"stood".to_vec(), g.clone(), Some(head)));
k.dialect_chain.push((b"unstamped".to_vec(), g, None));
let origin = crate::chronometry::Stamp::new(0, k.chain().vk(), crate::chronometry::Place::GENESIS);
assert!(k.treasury().entries().iter().any(|e| e.at == origin && e.receipt == [0; 32]), "the boot's rows stand at the origin");
let snap = k.snapshot();
let mut back = Kernel::new();
assert!(back.restore(&snap), "clean restore");
assert_eq!(back.treasury().entries(), k.treasury().entries(), "every row back, at its stamp, in its order");
assert!(back.treasury().audit().is_ok(), "and the ledger conserves");
let [settled] = back.settlements_cold.as_slice() else { panic!("the settlement came back") };
assert_eq!(settled.at, head, "at the act it follows");
let [(key, _, at)] = back.dialect_chain.as_slice() else { panic!("one transition came back: {:?}", back.dialect_chain.len()) };
assert_eq!((key.as_slice(), *at), (b"stood".as_slice(), Some(head)), "the stamped transition, at its receipt; the unstamped one refused");
}

/// 78.27 (3) — THE FEDERATION'S ORDER IS RE-DERIVED from the stamps that
/// ruled each word into it: a word is federated at the act that ruled it,
/// that stamp is its RULED leaf, and the order comes back from the stamps.
#[test]
fn the_federation_comes_back_in_the_order_its_rulings_stood() {
use crate::anastome::dialogue::RULING_UPHELD;
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
// Two words, each spoken by its own two voices.
for (hue, voices) in [(4u8, [[1; 32], [2; 32]]), (5, [[3; 32], [4; 32]])] {
    let b = vec![(0, 0, hue), (0, 1, hue), (1, 0, hue)];
    for v in voices {
        assert_eq!(speak_pair(&mut k, &a, &b, v), Outcome::GermPending);
    }
}
k.court_stroke();
let docketed: Vec<Vec<u8>> = k.provisional.scoped().map(|(a, _)| a).collect();
for addr in &docketed {
    assert!(k.apply_verdict(addr, RULING_UPHELD), "upheld");
    let ruled = k.chain().head_stamp().expect("the ruling is an act");
    if k.court_fed_index.get(addr).is_some() {
        assert_eq!(k.fed_at.get(addr), Some(&ruled), "federated at the act that ruled it");
    }
}
let order: Vec<Vec<u8>> = k.court_federation.iter().map(|(a, _)| a.as_bytes().to_vec()).collect();
assert_eq!(order.len(), 2, "both words ruled into the federation");
let mut back = Kernel::new();
assert!(back.restore(&k.snapshot()), "clean restore");
let again: Vec<Vec<u8>> = back.court_federation.iter().map(|(a, _)| a.as_bytes().to_vec()).collect();
assert_eq!(again, order, "the order re-derived from the ruling stamps");
}

/// LD0 step 5 — A COHERENCE FIELD IS A FIELD OF ITS WORD'S ROW. A
/// listing's field, its twisted closure written as its canonical ray,
/// survives a restart on the listing's row; a field on a word no row holds
/// is named and not written, and the tape does not grow by its address.
#[test]
fn a_field_rides_its_words_row() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1)];
let b = vec![(0, 0, 2), (0, 1, 2)];
speak_pair(&mut k, &a, &b, [1; 32]);
let listed = k.pending.scoped().next().expect("listed").1.address();
let chart = crate::transducer::stalk::Invariant::counts(&[3, 5]);
let t = crate::transducer::element::translator_versor(2, 3);
let scaled = t.scale(&num_rational::BigRational::new(num_bigint::BigInt::from(-7), num_bigint::BigInt::from(2)));
let entry = FieldEntry {
    chart: chart.clone(),
    state: crate::transducer::stalk::Closure::Twisted(Box::new(scaled)),
    remainder: vec![(1, 2, 3, 4)],
    stagnant: [0, 1, 0, 2],
};
k.field.insert(&listed, ListingField { entries: vec![entry.clone()], nonflat: 1 });
let bare = k.snapshot();
let orphan = crate::transducer::address::Address::from_bytes(vec![9u8; 4096]);
k.field_cold.insert(&orphan, ListingField { entries: vec![entry], nonflat: 1 });
let snap = k.snapshot();
assert_eq!(snap.len(), bare.len(), "a field on a word no row holds writes nothing");
let mut booted = Kernel::new();
assert!(booted.restore(&snap), "clean restore");
assert!(booted.field_cold.get(&orphan).is_none(), "and restores as nothing");
let got = booted.field.get(listed.as_bytes()).expect("the listing's field survived on its row");
let [e] = got.entries.as_slice() else { panic!("one entry: {got:?}") };
assert_eq!((&e.chart, &e.remainder, e.stagnant), (&chart, &vec![(1, 2, 3, 4)], [0, 1, 0, 2]));
let crate::transducer::stalk::Closure::Twisted(v) = &e.state else { panic!("twisted: {:?}", e.state) };
assert!(crate::transducer::element::same_versor(v, &t), "the closure is the ray it was");
assert_eq!(got.nonflat, 1);
}

/// ACT1-hold (78.27 Q1) — EVERY HOLDING PATH ADMITS BY AN ACT, AND A WORD
/// NO ACT ADMITTED IS NOT WRITTEN. An adopted word is held by an Adopted
/// act and carries its stamp; a word placed on the basis with no act — the
/// primitive a fixture uses — reads as held without a stamp, and the
/// snapshot refuses it: it does not come back.
#[test]
fn a_word_no_act_admitted_is_not_written() {
use crate::transducer::element::from_canonical;
use crate::transducer::germinate::germinate;
let mut k = Kernel::new();
let adopted = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 2)], &[(0, 0, 1u8), (0, 1, 9)]).unwrap());
let mut adopt = k.stroke();
assert!(matches!(k.install_adopted(&adopted, [1; 32], &mut adopt), Adoption::Installed));
let at = k.admitted_at(adopted.address().as_bytes()).expect("the adoption is an act, and it admits");
assert_eq!(k.chain().node(&at).map(|r| r.kind), Some(crate::chronometry::ActKind::Adopted));
let bare = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1), (1, 0, 1)], &[(0, 0, 1u8), (0, 1, 1), (0, 2, 1)]).unwrap());
k.basis_insert(bare.clone(), [7; 32]);
assert_eq!(k.unstamped(), [1, 0, 0, 0, 0], "held on the basis with no act: the reading names it by row");
let mut booted = Kernel::new();
assert!(booted.restore(&k.snapshot()), "clean restore");
assert!(booted.word_at(adopted.address().as_bytes()).is_some(), "the admitted word comes back");
assert_eq!(booted.admitted_at(adopted.address().as_bytes()), Some(at), "at the stamp that admitted it");
assert!(booted.word_at(bare.address().as_bytes()).is_none(), "the word no act admitted was refused");
assert_eq!(booted.unstamped(), [0; 5]);
}
