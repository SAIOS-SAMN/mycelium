// 78.154 (4), 78.152 TU-6, issue #159 — RETRIEVE, `ACT_DOMAIN` sub-kind 19:
// a staker's signed coordinate retrieval of the receipts at a final point's
// tick, priced at the read price and claimed on its stake, answered beside
// the court's signed section over its head, which the retriever checks.

use crate::anastome::dialogue::domain::{decode_domain_request, DomainGenesis, DomainRequest, Founded};
use crate::anastome::dialogue::retrieve::{
    answered_payload, decode_retrieve_reply, encode_retrieve, encode_retrieve_reply, verify_retrieved, RetrievalBreach, Retrieve, RetrieveRefusal, RetrieveReply,
    Retrieved,
};
use crate::anastome::dialogue::wallet::StakeAct;
use crate::anastome::dialogue::ReceiptSection;
use crate::chronometry::tuniversal::{Origin, TPoint};
use crate::chronometry::ActKind;
use crate::court::admission::SignedPrefix;
use crate::court::identity::HyphaKey;
use crate::server::cognition::Court;
use ed25519_dalek::SigningKey;

fn key(b: u8) -> SigningKey {
    SigningKey::from_bytes(&[b; 32])
}

fn vk(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

/// A final point of the mesh at tick `h`.
fn fp(h: u64) -> TPoint {
    TPoint { origin: Origin { genesis: [9; 32], epoch: 0 }, h, lanes: [h, 0, 0, 0].into(), k: h }
}

/// The court bound to `name`'s key with domain 4 founded, a few strokes on
/// its line, and staker 60 funded and staked `[4, 4, 4, 4]`.
fn court(name: &str) -> (Court, HyphaKey) {
    let hk = super::fixtures::court_key(name);
    let mut k = Court::new();
    k.bind_identity(hk.verifying_bytes());
    let g = DomainGenesis { domain: vk(&key(4)), tiers: vec![vk(&key(24))] };
    assert!(matches!(k.hear_domain(&g.sign(&key(4))), Founded::Sealed { .. }));
    for _ in 0..3 {
        k.append_step(ActKind::List, [1; 32], [2; 32]);
        k.orbit();
    }
    let staker = key(60);
    let anchor = k.anchor();
    k.treasury.genesis(vk(&staker), [4, 4, 4, 4], anchor);
    assert!(k.stake(&StakeAct { who: vk(&staker), unstake: false, mu: [4, 4, 4, 4].into(), nonce: [1; 32] }.sign(&staker)).is_ok());
    (k, hk)
}

/// The staker's retrieval of the receipts at tick `h`, citing `fp(cited)`.
fn ask(h: u64, cited: u64, nonce: u8) -> Retrieve {
    Retrieve { domain: vk(&key(4)), staker: vk(&key(60)), point: fp(h), cited: fp(cited), nonce: [nonce; 32] }
}

/// The court's own section over its head, signed over the request, as the
/// door writes it (`sign_position`).
fn section(k: &Court, hk: &HyphaKey, r: &Retrieve) -> ReceiptSection {
    let head = k.chain.head().expect("the court acted");
    let prefix = SignedPrefix {
        genesis: [9; 32],
        epoch: 0,
        prev_receipt_hash: head.prev_hash(),
        at: head.stamp.place(),
        height: head.stamp.h,
        chart: vec![],
        payload: answered_payload(r),
    };
    let sig = hk.sign(&prefix.bytes().expect("the prefix frames"));
    ReceiptSection { prefix, sig, receipt: head }
}

/// THE RETRIEVAL ROUND-TRIPS THE WIRE: sub-kind 19 decodes to the signed
/// request exactly, and the served reply, receipts and section, reads back.
#[test]
fn retrieve_round_trips_as_domain_sub_kind_19() {
    let s = ask(3, 5, 1).sign(&key(60));
    let Some(DomainRequest::Retrieve(back)) = decode_domain_request(&encode_retrieve(&s)) else { panic!("a retrieval") };
    assert_eq!(*back, s);
    assert!(back.verifies(), "the staker's signature stands");
    let (mut k, hk) = court("retrieve-wire");
    let h = k.chain.head_height();
    let s = ask(h, h, 2).sign(&key(60));
    let Retrieved::Served { fee, receipts } = k.retrieve_signed(&s, Some(fp(h))) else { panic!("served") };
    let sec = section(&k, &hk, &s.retrieve);
    let body = encode_retrieve_reply(&Retrieved::Served { fee: fee.clone(), receipts: receipts.clone() }, Some(&sec)).expect("written");
    let Some(RetrieveReply::Served { fee: f, receipts: r, section: back }) = decode_retrieve_reply(&body) else { panic!("read back") };
    assert_eq!((f, r.len()), (fee, receipts.len()));
    assert_eq!((back.prefix.bytes(), back.sig), (sec.prefix.bytes(), sec.sig), "the section reads back whole");
    assert!(encode_retrieve_reply(&Retrieved::Served { fee: [0; 4].into(), receipts }, None).is_none(), "a served answer is never unsigned");
    let refused = encode_retrieve_reply(&Retrieved::Refused(RetrieveRefusal::Short), None).expect("a refusal");
    assert!(matches!(decode_retrieve_reply(&refused), Some(RetrieveReply::Refused(RetrieveRefusal::Short))));
}

/// A RETRIEVAL IS PRICED AND REFUSED BY NAME: served at the tick, one unit on
/// H per receipt claimed on the stake; refused unsigned, for an unknown
/// domain, at two origins or above the cited point, not final, or resent
/// with the same nonce.
#[test]
fn retrieve_claims_read_price_and_refuses_by_name() {
    let (mut k, _) = court("retrieve-price");
    let h = k.chain.head_height();
    let s = ask(h, h, 1).sign(&key(60));
    let Retrieved::Served { fee, receipts } = k.retrieve_signed(&s, Some(fp(h))) else { panic!("served") };
    assert!(!receipts.is_empty() && receipts.iter().all(|r| r.stamp.h == h), "the receipts at the tick");
    assert_eq!(k.claims_on(&vk(&key(60))), fee, "the fee answered is the claim drawn");
    let mut forged = ask(h, h, 2).sign(&key(60));
    forged.sig[0] ^= 1;
    let refusal = |r: Retrieved| match r {
        Retrieved::Refused(why) => Some(why),
        Retrieved::Served { .. } => None,
    };
    assert_eq!(refusal(k.retrieve_signed(&forged, Some(fp(h)))), Some(RetrieveRefusal::Unsigned));
    let other = Retrieve { domain: [7; 32], ..ask(h, h, 3) }.sign(&key(60));
    assert_eq!(refusal(k.retrieve_signed(&other, Some(fp(h)))), Some(RetrieveRefusal::UnknownDomain));
    assert_eq!(refusal(k.retrieve_signed(&ask(h, h - 1, 4).sign(&key(60)), Some(fp(h)))), Some(RetrieveRefusal::Origins), "above the point cited");
    let elsewhere = Retrieve { cited: TPoint { origin: Origin { genesis: [8; 32], epoch: 0 }, ..fp(h) }, ..ask(h, h, 5) }.sign(&key(60));
    assert_eq!(refusal(k.retrieve_signed(&elsewhere, Some(fp(h)))), Some(RetrieveRefusal::Origins));
    assert_eq!(refusal(k.retrieve_signed(&ask(h, h + 1, 6).sign(&key(60)), Some(fp(h)))), Some(RetrieveRefusal::NotFinal));
    assert_eq!(refusal(k.retrieve_signed(&s, Some(fp(h)))), Some(RetrieveRefusal::Overflow), "a resent nonce draws no second claim");
}

/// THE RETRIEVER VERIFIES THE ANSWER: the court's section verifies under its
/// key, is one claim with its head, signs over this request, and every
/// receipt re-derives at the tick and under the head. A forged receipt, a
/// section over another request or under another key does not stand.
#[test]
fn retriever_verifies_section_and_receipts_under_head() {
    let (mut k, hk) = court("retrieve-verify");
    let h = k.chain.head_height();
    let s = ask(h, h, 1).sign(&key(60));
    let Retrieved::Served { receipts, .. } = k.retrieve_signed(&s, Some(fp(h))) else { panic!("served") };
    let sec = section(&k, &hk, &s.retrieve);
    let court = hk.verifying_bytes();
    assert_eq!(verify_retrieved(&s.retrieve, &court, &receipts, &sec), Ok(()));
    let mut forged = receipts.clone();
    forged[0].mu[0] ^= 1;
    assert_eq!(verify_retrieved(&s.retrieve, &court, &forged, &sec), Err(RetrievalBreach::Forged { at: forged[0].stamp }), "a forged receipt fails");
    assert_eq!(verify_retrieved(&ask(h, h, 9), &court, &receipts, &sec), Err(RetrievalBreach::OtherRequest));
    assert_eq!(verify_retrieved(&s.retrieve, &[3; 32], &receipts, &sec), Err(RetrievalBreach::Unsigned));
    let earlier = ask(h - 1, h, 1);
    assert!(matches!(verify_retrieved(&earlier, &court, &receipts, &section(&k, &hk, &earlier)), Err(RetrievalBreach::OffTick { .. })));
}
