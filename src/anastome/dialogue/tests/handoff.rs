// 78.126 — ACT_HANDOFF's wire, and a fold's handoff sides: every act and
// every answer read back whole; a fold consuming sides writes them after its
// (possibly empty) children, and a fold with neither writes as before.

use crate::anastome::dialogue::handoff::*;
use crate::anastome::dialogue::uplink::{decode_uplink, decode_uplink_reply, encode_uplink, encode_uplink_reply, SideRefusal, TierFold, Uplinked};
use crate::chronometry::Stamp;

#[test]
fn every_handoff_act_and_answer_read_back_whole() {
    let key = ed25519_dalek::SigningKey::from_bytes(&[5u8; 32]);
    let offer = Offer {
        from_tier: [1u8; 32],
        to_tier: [2u8; 32],
        out_nets: vec![([3u8; 32], [-2, 0, 0, -1])],
        in_nets: vec![([4u8; 32], [2, 0, 0, 1])],
        charter: [0u8; 32],
        nonce: [9u8; 32],
    };
    assert_eq!(offer.conserves(), Ok(()));
    let id = offer.id();
    for h in [Handoff::Offer(Box::new(offer)), Handoff::Accept { id }, Handoff::Refuse { id }, Handoff::Expire { id }] {
        let signed = h.sign(&key);
        let mut asked = encode_handoff(&signed);
        assert_eq!(decode_handoff_request(&asked), Some(HandoffRequest::Signed(signed)));
        asked.push(0);
        assert_eq!(decode_handoff_request(&asked), None, "a byte left over is no act");
    }
    assert_eq!(decode_handoff_request(&encode_handoff_read(id)), Some(HandoffRequest::Read(id)));
    let at = Stamp { h: 7, vk: [6u8; 32], k: 2, i: 1 };
    for a in [
        Handed::Sealed { at, receipt: [1u8; 32], id, state: State::Offered { until: 1031 } },
        Handed::Sealed { at, receipt: [1u8; 32], id, state: State::Accepted },
        Handed::Sealed { at, receipt: [1u8; 32], id, state: State::Returned },
        Handed::Refused(HandoffRefusal::Unbalanced(Unbalanced::Lane { lane: 1 })),
        Handed::Refused(HandoffRefusal::Window),
    ] {
        assert_eq!(decode_handoff_reply(&encode_handoff_reply(&a)), Some(a.clone()), "{a:?}");
    }
    for r in [None, Some((State::Accepted, true, false)), Some((State::Offered { until: 9 }, false, false))] {
        assert_eq!(decode_read_reply(&encode_read_reply(r)), Some(r));
    }
    // A fold consuming sides and netting no children; a fold with neither.
    let tier = ed25519_dalek::SigningKey::from_bytes(&[8u8; 32]);
    let plain = TierFold {
        tier: tier.verifying_key().to_bytes(),
        prefix: [2u8; 32],
        rows: vec![([3u8; 32], [-2, 0, 0, -1])],
        declared: Vec::new(),
        children: Vec::new(),
        handoffs: Vec::new(),
    };
    let crossing = TierFold { handoffs: vec![(id, Side::Out)], ..plain.clone() };
    assert_ne!(plain.signed_bytes(), crossing.signed_bytes(), "the sides are signed");
    let signed = crossing.sign(&tier);
    let bytes = encode_uplink(&signed);
    assert_eq!(decode_uplink(&bytes), Some(signed));
    let lone = plain.sign(&tier);
    let mut padded = encode_uplink(&lone);
    padded.extend_from_slice(&[0, 0]);
    assert_eq!(decode_uplink(&padded), None, "an empty block is not the one writing");
    let refused = Uplinked::HandoffRefused { index: 2, why: SideRefusal::Consumed };
    assert_eq!(decode_uplink_reply(&encode_uplink_reply(&refused)), Some(refused));
}

/// 78.127 — conversions out and in, and the reserve's designation, read back
/// whole; `rate · g` exact or none; a genesis frame still reads as a genesis.
#[test]
fn every_conversion_and_the_reserve_read_back_whole() {
    use crate::anastome::dialogue::domain::{decode_domain_reply, decode_domain_request, encode_domain, encode_domain_reply, encode_reservation, DomainGenesis, DomainRequest, Founded, Reservation};
    let key = ed25519_dalek::SigningKey::from_bytes(&[5u8; 32]);
    let out = ConvertOut { tier: [1u8; 32], holder: [2u8; 32], identity: [3u8; 32], g: 10, rate: [(1, 2), (0, 1), (3, 5), (0, 1)], nonce: [4u8; 32] };
    assert_eq!(out.mu(), Some([5, 0, 6, 0]));
    assert_eq!(ConvertOut { rate: [(1, 3), (0, 1), (0, 1), (0, 1)], ..out.clone() }.mu(), None, "10/3 is no integer");
    assert_eq!(ConvertOut { g: 0, ..out.clone() }.mu(), None);
    let inn = ConvertIn { holder: [2u8; 32], tier: [1u8; 32], identity: [3u8; 32], g: 7, mu: [3, 0, 0, 0], nonce: [9u8; 32] };
    assert_ne!(out.id(), inn.id());
    for h in [Handoff::ConvertOut(Box::new(out)), Handoff::ConvertIn(Box::new(inn))] {
        let signed = h.sign(&key);
        assert_eq!(decode_handoff_request(&encode_handoff(&signed)), Some(HandoffRequest::Signed(signed)));
    }
    for r in [HandoffRefusal::NotReserve, HandoffRefusal::ReserveShort, HandoffRefusal::Inexact, HandoffRefusal::HolderShort] {
        assert_eq!(decode_handoff_reply(&encode_handoff_reply(&Handed::Refused(r))), Some(Handed::Refused(r)));
    }
    let reserve = Reservation { domain: [6u8; 32], tier: [1u8; 32], nonce: [0u8; 32] }.sign(&key);
    assert_eq!(decode_domain_request(&encode_reservation(&reserve)), Some(DomainRequest::Reserve(reserve)));
    let genesis = DomainGenesis { domain: [6u8; 32], tiers: vec![[1u8; 32]] }.sign(&key);
    assert_eq!(decode_domain_request(&encode_domain(&genesis)), Some(DomainRequest::Genesis(genesis)), "a genesis reads as it did");
    use crate::anastome::dialogue::domain::{decode_reserve_reading, encode_reserve_read, encode_reserve_reading, ReserveReading};
    assert_eq!(decode_domain_request(&encode_reserve_read([6u8; 32])), Some(DomainRequest::ReadReserve([6u8; 32])));
    for r in [ReserveReading::default(), ReserveReading { current: Some([1u8; 32]), pending: Some([2u8; 32]), balance: [3, 0, 9, 1] }] {
        assert_eq!(decode_reserve_reading(&encode_reserve_reading(&r)), Some(r.clone()));
    }
    let at = Stamp { h: 2, vk: [8u8; 32], k: 1, i: 0 };
    for f in [Founded::Reserved { at, receipt: [1u8; 32], now: false }, Founded::NotATier, Founded::NotFounded] {
        assert_eq!(decode_domain_reply(&encode_domain_reply(&f)), Some(f.clone()));
    }
}
