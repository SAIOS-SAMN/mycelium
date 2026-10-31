// 78.154 — the LEDGER, MAP and TRAVERSE sub-kinds of ACT_DOMAIN, the STAKE
// acts of ACT_WALLET and the CONNECTION_TAX refusal of ACT_UPLINK: each
// decodes into the same exact values it was written from, its signature
// verifying under its signer (Concordance 9.1), and a byte left over is none.

use crate::anastome::dialogue::atlas::{
    decode_ledger_page, decode_ledger_reply, decode_map_reading, decode_map_reply, encode_ledger, encode_ledger_page, encode_ledger_read, encode_ledger_reply, encode_map, encode_map_read,
    encode_map_reading, encode_map_reply, HeldEntry, HeldRecord, LedgerEntry, LedgerPage, LedgerRefusal, Ledgered, MapAct, MapEntry, MapRecord, MapRefusal, Mapped, Segment,
};
use crate::anastome::dialogue::crossing::{decode_traverse_reply, encode_traverse, encode_traverse_reply, CrossRefusal, Crossed, Declaration, Gas, Side, Traverse};
use crate::anastome::dialogue::domain::{decode_domain_request, DomainRequest};
use crate::anastome::dialogue::uplink::{decode_uplink_reply, encode_uplink_reply, TaxRefusal, Uplinked};
use crate::anastome::dialogue::wallet::{decode_stake, decode_stake_reply, decode_wallet, encode_stake, encode_stake_reply, StakeAct, StakeRefusal};
use crate::chronometry::tuniversal::{Origin, TPoint};

fn key(b: u8) -> ed25519_dalek::SigningKey {
    ed25519_dalek::SigningKey::from_bytes(&[b; 32])
}

fn at(h: u64) -> TPoint {
    TPoint { origin: Origin { genesis: [9; 32], epoch: 2 }, h, lanes: [h, 1, 2, 3], k: 4 }
}

fn verifies(signer: &ed25519_dalek::SigningKey, bytes: &[u8], sig: &[u8; 64]) -> bool {
    crate::court::identity::verify(&signer.verifying_key().to_bytes(), bytes, sig)
}

/// A LEDGER ENTRY, with and without applied crossings, reads back as the
/// entry it was, its signature verifying; the applied list rides only
/// where it is not empty; a count of zero written is no entry.
#[test]
fn ledger_entry_round_trips_and_verifies() {
    for applied in [Vec::new(), vec![([3; 32], 41u64), ([4; 32], 42)]] {
        let e = LedgerEntry { domain: [6; 32], u: 40, commitment: [1; 32], events: [2; 32], point: at(7), parent: [5; 32], applied, nonce: [8; 32] };
        let signed = e.clone().sign(&key(30));
        let mut frame = encode_ledger(&signed);
        let Some(DomainRequest::Ledger(back)) = decode_domain_request(&frame) else { panic!("a ledger entry") };
        assert_eq!(back.entry, e);
        assert!(verifies(&key(30), &back.entry.signed_bytes(), &back.sig));
        assert_eq!(back.entry.hash(), e.hash());
        frame.push(0);
        assert_eq!(decode_domain_request(&frame), None, "a byte left over");
    }
    let bare = LedgerEntry { domain: [6; 32], u: 1, commitment: [0; 32], events: [0; 32], point: at(1), parent: [0; 32], applied: Vec::new(), nonce: [0; 32] };
    let mut frame = encode_ledger(&bare.clone().sign(&key(30)));
    assert_eq!(frame.len(), 1 + 32 + 2 + 1 + 8 + 32 + 32 + 84 + 32 + 32 + 64, "the ruled bytes exactly");
    frame.push(0);
    assert_eq!(decode_domain_request(&frame), None, "an empty applied list is never written");
    assert_eq!(decode_domain_request(&encode_ledger_read([6; 32], 9)), Some(DomainRequest::ReadLedger { domain: [6; 32], after: 9 }));
    for r in [
        Ledgered::Sealed { entry: [7; 32], h: 12, point: at(9) },
        Ledgered::Refused(LedgerRefusal::NotReserve),
        Ledgered::Refused(LedgerRefusal::Parent),
        Ledgered::Refused(LedgerRefusal::LineRefused),
    ] {
        assert_eq!(decode_ledger_reply(&encode_ledger_reply(&r)), Some(r.clone()));
    }
    let held = HeldEntry { entry: bare.clone(), hash: bare.hash(), sealed: 3, court: at(2), dilation: Some((1, 15)) };
    let page = LedgerPage { entries: vec![held.clone(), HeldEntry { dilation: None, ..held }], more: true };
    assert_eq!(decode_ledger_page([6; 32], &encode_ledger_page(&page)), Some(page));
}

/// A MAP ACT — a mouth, a throat, a local throat — and the map as read come
/// back whole; a local throat's frames read back as the exact versors they
/// were, its wire `[2][throat 32][keeper 32][frame_0][frame_1][N]`, and its
/// id the one derived from its throat key.
#[test]
fn map_act_and_reading_round_trip() {
    use crate::anastome::dialogue::atlas::{local_record_id, local_throat_id};
    use crate::transducer::cga::{base_vector, translator_at, Mv};
    let n = |v: i64| num_rational::BigRational::from_integer(v.into());
    let (f0, f1) = (translator_at(&[n(32), n(-17), n(0)]), translator_at(&[n(5), n(4), n(0)]));
    let local = MapRecord::LocalThroat { throat: [4; 32], keeper: [2; 32], frames: (f0.clone(), f1.clone()), freshness: 6 };
    let records = [
        MapRecord::Mouth { place: ([3; 32], 1), freshness: 8 },
        MapRecord::Segment { kind: Segment::Throat, a: ([3; 32], 0), b: ([3; 32], 1), span: (11, 4), keeper: Some([2; 32]) },
        MapRecord::Segment { kind: Segment::Tube, a: ([3; 32], 0), b: ([5; 32], 1), span: (1, 1), keeper: None },
        local.clone(),
    ];
    for record in records.clone() {
        let act = MapAct { domain: [6; 32], point: at(4), record, nonce: [1; 32] };
        let signed = act.clone().sign(&key(31));
        let Some(DomainRequest::Map(back)) = decode_domain_request(&encode_map(&signed)) else { panic!("a map act") };
        assert_eq!(back.act, act);
        assert!(verifies(&key(31), &back.act.signed_bytes(), &back.sig));
    }
    // The local throat's frames decode coefficient for coefficient, not
    // only projectively.
    let mut body = Vec::new();
    local.put(&mut body);
    let mut p = 0;
    let Some(MapRecord::LocalThroat { throat, keeper, frames: (b0, b1), freshness }) = MapRecord::take(&body, &mut p) else { panic!("a local throat") };
    assert_eq!((throat, keeper, freshness, p), ([4; 32], [2; 32], 6, body.len()));
    assert!(b0.sub(&f0).is_zero() && b1.sub(&f1).is_zero(), "the frames exactly");
    let mut material = [6u8; 32].to_vec();
    material.extend_from_slice(&[4; 32]);
    let lid = blake3::derive_key("mycelium local throat v1", &material);
    assert_eq!(local_throat_id(&[6; 32], &[4; 32]), lid);
    assert_eq!(local.id(&[6; 32]), local_record_id(&[6; 32], &lid), "held under its derived mouth id");
    assert_eq!(decode_domain_request(&encode_map_read([6; 32])), Some(DomainRequest::ReadMap([6; 32])));
    let places = |r: &MapRecord| match r {
        MapRecord::Mouth { .. } => vec![base_vector(&[n(1), n(2)])],
        MapRecord::LocalThroat { .. } => vec![base_vector(&[n(2), n(3)]), base_vector(&[n(5), n(4)])],
        MapRecord::Segment { .. } => Vec::<Mv>::new(),
    };
    let reading: Vec<MapEntry> = records.iter().map(|r| MapEntry { held: HeldRecord { record: r.clone(), point: at(3), sealed: 9 }, places: places(r) }).collect();
    let back = decode_map_reading(&encode_map_reading(&reading)).expect("reads back");
    assert_eq!(back, reading);
    for (b, r) in back.iter().zip(&reading) {
        assert!(b.places.iter().zip(&r.places).all(|(x, y)| x.sub(y).is_zero()), "each place exactly");
    }
    for r in [
        Mapped::Sealed { record: [1; 32], h: 4, point: at(4) },
        Mapped::Refused(MapRefusal::Keeper),
        Mapped::Refused(MapRefusal::Behind),
        Mapped::Refused(MapRefusal::NotVersor),
        Mapped::Refused(MapRefusal::NoGeometry),
        Mapped::Refused(MapRefusal::Unplaced),
    ] {
        assert_eq!(decode_map_reply(&encode_map_reply(&r)), Some(r.clone()));
    }
}

/// A TRAVERSE, its declarations and its answer read back whole; both
/// declarations are signed over the crossing, never over the payer's act.
#[test]
fn traverse_round_trips_whole() {
    let mut t = Traverse {
        from: [6; 32],
        payer: key(40).verifying_key().to_bytes(),
        mouth: [3; 32],
        side: 0,
        at: at(9),
        point_a: at(4),
        point_b: at(6),
        entry_a: [1; 32],
        u_a: 100,
        entry_b: [2; 32],
        u_b: 7,
        route: vec![[8; 32], [9; 32]],
        debit: None,
        credit: None,
        nonce: [5; 32],
    };
    let lanes = [5, 2, 0, 3];
    let debit = Declaration { lanes, sig: ed25519_dalek::Signer::sign(&key(24), &t.debit_bytes(&lanes)).to_bytes() };
    let credit = Declaration { lanes, sig: ed25519_dalek::Signer::sign(&key(25), &t.credit_bytes(&lanes)).to_bytes() };
    let crossing = t.crossing();
    t.debit = Some(debit);
    t.credit = Some(credit);
    assert_eq!(t.crossing(), crossing, "the declarations are no part of the crossing");
    let signed = t.clone().sign(&key(40));
    let Some(DomainRequest::Traverse(back)) = decode_domain_request(&encode_traverse(&signed)) else { panic!("a traverse") };
    assert_eq!(back.traverse, t);
    assert!(verifies(&key(40), &back.traverse.signed_bytes(), &back.sig));
    let d = back.traverse.debit.as_ref().expect("the debit");
    assert!(verifies(&key(24), &back.traverse.debit_bytes(&d.lanes), &d.sig));
    for r in [
        Crossed::Sealed { crossing, h: 30, point: at(9), gas: Gas { s: (11, 4), g: 2, t: 2, h: 3 }, price: [1; 4], fee: [2, 2, 2, 3] },
        Crossed::Refused(CrossRefusal::NotYetAtH(Side::B)),
        Crossed::Refused(CrossRefusal::Unmapped(Side::A)),
        Crossed::Refused(CrossRefusal::Mismatched),
    ] {
        assert_eq!(decode_traverse_reply(&encode_traverse_reply(&r)), Some(r.clone()));
    }
}

/// A STAKE ACT is a wallet frame no read decodes as; its answer reads back.
#[test]
fn stake_act_round_trips_never_as_wallet() {
    let s = StakeAct { who: key(40).verifying_key().to_bytes(), unstake: true, mu: [1, 2, 3, 4], nonce: [7; 32] }.sign(&key(40));
    let frame = encode_stake(&s);
    assert_eq!(decode_stake(&frame), Some(s.clone()));
    assert_eq!(decode_wallet(&frame), None, "never read as a wallet read");
    assert!(verifies(&key(40), &s.act.signed_bytes(), &s.sig));
    for r in [Ok([1, 0, 0, 2]), Err(StakeRefusal::Claimed), Err(StakeRefusal::Spent)] {
        assert_eq!(decode_stake_reply(&encode_stake_reply(&r)), Some(r));
    }
    for why in [TaxRefusal::Source { lane: 2 }, TaxRefusal::Short, TaxRefusal::Carried] {
        assert_eq!(decode_uplink_reply(&encode_uplink_reply(&Uplinked::Tax(why))), Some(Uplinked::Tax(why)));
    }
}
