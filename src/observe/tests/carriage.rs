use super::super::*;
use super::common::*;

/// LD1 — THE CARRIAGE TRAVELS WHOLE, each record filed under its carrier and
/// kind in order, its largest kept.
#[test]
fn carriage_files_by_carrier_and_kind_and_round_trips() {
    let mut c = Carriage::default();
    c.add(CARRIER_FRAME, 0x0d, 40);
    c.add(CARRIER_DELTA, 12, 900);
    c.add(CARRIER_DELTA, 12, 100);
    c.add(CARRIER_AWARD, 0, 120);
    assert_eq!(
        c.rows.iter().map(|r| (r.carrier, r.kind, r.records, r.bytes, r.largest)).collect::<Vec<_>>(),
        [(CARRIER_AWARD, 0, 1, 120, 120), (CARRIER_DELTA, 12, 2, 1000, 900), (CARRIER_FRAME, 0x0d, 1, 40, 40)]
    );
    reads_back(&[Reading::Carriage(c)]);
}

/// LD0 step 4 — AN AWARD IS A FIELD OF ITS WORD'S ROW, measured at what the
/// field carries: the voices, μ⃗, the stamp and the receipt, never the
/// address. An award on a 70 000-byte word weighs what one on a 3-byte word
/// does, and the treasury's own snapshot carries no award at all.
#[test]
fn treasury_carriage_measures_award_flat_in_word() {
    use crate::transducer::address::Address;
    let mut t = crate::economy::Treasury::new();
    let a = (crate::chronometry::Stamp { h: 1, vk: [0; 32], k: 1, i: 0 }, [1u8; 32]);
    t.escrow_award(&Address::from_bytes([7u8; 3]), vec![[2; 32]], [4, 0, 0, 0], a);
    t.escrow_award(&Address::from_bytes(vec![9u8; 70_000]), vec![[2; 32]], [4, 0, 0, 0], a);
    let mut c = Carriage::default();
    t.carriage(&mut c);
    // The field: its tag, the voice count, one voice, μ⃗, the stamp, the receipt.
    let field = 1 + 4 + 32 + 32 + crate::chronometry::Stamp::LEN as u64 + 32;
    let [award] = c.rows.as_slice() else { panic!("one carrier, one kind: {:?}", c.rows) };
    assert_eq!((award.records, award.largest, award.bytes), (2, field, 2 * field), "flat in the word's size");
    let mut tries = Vec::new();
    t.put_tries(&mut tries);
    let (stakes, balances, held) = crate::economy::Treasury::take_tries(&tries, &mut 0).expect("the tries read back");
    let back = crate::economy::Treasury::from_parts(stakes, balances, held, t.entries().to_vec());
    assert!(back.escrow_of(&[7u8; 3]).is_none(), "the award rides its word's node, not the treasury's tries");
}

/// LD1, 78.24 — A LEDGER'S NODES ARE MEASURED BY THE ACT KIND THAT SEALED
/// THEM, and a node citing a word carries the same bytes whatever the size
/// of the word: the citation is the word's fibre and admitting stamp, never
/// a copy of it.
#[test]
fn of_chain_measures_nodes_by_kind_flat_in_word() {
    use crate::chronometry::{ActKind, Cite, ReceiptChain, Stamp};
    let small = crate::transducer::element::from_canonical(
        &crate::transducer::germinate::germinate(&[(0, 0, 1u8)], &[(0, 0, 2u8)]).expect("a small word"),
    );
    let pairs: Vec<(i64, i64, u8)> = (0..40).map(|i| (i, i % 7, 1u8)).collect();
    let after: Vec<(i64, i64, u8)> = (0..40).map(|i| (i, i % 7, 2u8)).collect();
    let large = crate::transducer::element::from_canonical(
        &crate::transducer::germinate::germinate(&pairs, &after).expect("a larger word"),
    );
    assert!(large.address().len() > small.address().len(), "two words of different sizes");
    let at = Stamp { h: 2, vk: [5; 32], k: 2, i: 0 };
    let chart = crate::transducer::stalk::Invariant::counts(&[1, 0, 0, 1]);
    let cite = |_: &crate::transducer::element::Element| Cite::Word { chart: chart.clone(), class: None, at };
    // Genesis first: it names no predecessor, so the two listing nodes after
    // it are the same shape apart from the word each cites. (78.94: kinds a
    // court seals; the stood and learned kinds of the voices' chains are
    // sealed by nothing.)
    let mut chain = ReceiptChain::with_signer([3; 32]);
    chain.append(ActKind::Attest, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends");
    chain.append(ActKind::List, [0; 4], vec![cite(&small)], [0; 32], Vec::new()).expect("appends");
    chain.append(ActKind::List, [0; 4], vec![cite(&large)], [0; 32], Vec::new()).expect("appends");
    let mut c = Carriage::default();
    c.of_chain(&chain);
    let listed = c.rows.iter().find(|r| r.kind == ActKind::List.byte()).expect("the listing nodes");
    assert_eq!(listed.records, 2);
    assert_eq!(listed.largest * 2, listed.bytes, "both carry the same bytes, whatever the word they cite");
    assert!(c.rows.iter().any(|r| r.kind == ActKind::Attest.byte() && r.records == 1), "and the attest node by its own kind");
}
