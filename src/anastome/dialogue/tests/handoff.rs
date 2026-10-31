// 78.126 — ACT_HANDOFF's wire, and a fold's handoff sides: every act and
// every answer read back whole; a fold consuming sides writes them after its
// (possibly empty) children, and a fold with neither writes as before.

use crate::anastome::dialogue::handoff::*;
use crate::anastome::dialogue::universe::{Geometry, Metric};
use crate::anastome::dialogue::uplink::{decode_uplink, decode_uplink_reply, encode_uplink, encode_uplink_reply, SideRefusal, TierFold, Uplinked};
use crate::chronometry::Stamp;

/// Every handoff act and every answer read back whole, and a fold's handoff
/// sides are signed and written after its children.
#[test]
fn handoff_acts_and_answers_round_trip_whole() {
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
    let id = offer.citation();
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

/// 78.128 — a mouth pair, its close, a traversal and the loops read back
/// whole; a traversal is the handoff its offer is.
#[test]
fn mouth_acts_and_loops_round_trip_whole() {
    let key = ed25519_dalek::SigningKey::from_bytes(&[5u8; 32]);
    let one = crate::transducer::element::identity_versor();
    let mouth = Mouth {
        from_tier: [1u8; 32],
        to_tier: [2u8; 32],
        keeper: [1u8; 32],
        from_frame: one.clone(),
        to_frame: one.clone(),
        toll: [3, 0, 0, 0],
        nonce: [4u8; 32],
        tariff: None,
        tube: None,
    };
    assert!(mouth.frames_hold());
    // 78.133 — a tariffed mouth is its own kind, its tariff in its id; the
    // flat mouth's id is what it was.
    let tariff = Tariff { rate_s: (3, 2), rate_h: (1, 7), split: vec![([1u8; 32], (2, 3)), ([5u8; 32], (1, 3))] };
    let tariffed = Mouth { toll: [0; 4], tariff: Some(tariff.clone()), ..mouth.clone() };
    assert_ne!(tariffed.citation(), Mouth { tariff: None, ..tariffed.clone() }.citation(), "the tariff is in the id");
    let changed = Tariff { split: vec![([1u8; 32], (1, 2)), ([5u8; 32], (1, 2))], ..tariff };
    assert_ne!(tariffed.citation(), Mouth { tariff: Some(changed), ..tariffed.clone() }.citation(), "a new split is a new mouth");
    let signed = Handoff::Mouth(Box::new(tariffed.clone())).sign(&key);
    assert_eq!(decode_handoff_request(&encode_handoff(&signed)), Some(HandoffRequest::Signed(signed)));
    // 78.134 — a tube is its own kind, with or without its tariff; its tube
    // in its id; the tube read and its reading back whole.
    let tube = Tube { mass: (3, 2), from_space: Geometry::Curved((1, 1)), to_space: Geometry::Curved((5, 3)), supersedes: Some([7u8; 32]), core: false };
    let tubed = Mouth { tube: Some(tube.clone()), ..tariffed.clone() };
    assert_ne!(tubed.citation(), tariffed.citation(), "the tube is in the id");
    // 78.140 — a tube with a torus end its own kind and tag, core or not.
    let torus = Geometry::Torus { periods: vec![(10, 1), (6, 1)], metric: Metric::L1 };
    let spaced = Mouth { tube: Some(Tube { to_space: torus.clone(), ..tube.clone() }), ..tubed.clone() };
    assert_ne!(spaced.citation(), tubed.citation(), "the space is in the id");
    let spaced_core = Mouth { tube: Some(Tube { from_space: torus, core: true, ..tube.clone() }), ..tubed.clone() };
    for m in [tubed.clone(), Mouth { tariff: None, ..tubed.clone() }, spaced, spaced_core] {
        let signed = Handoff::Mouth(Box::new(m)).sign(&key);
        assert_eq!(decode_handoff_request(&encode_handoff(&signed)), Some(HandoffRequest::Signed(signed)));
    }
    assert_eq!(decode_handoff_request(&encode_tube_read([3u8; 32])), Some(HandoffRequest::TubeRead([3u8; 32])));
    // 78.135 — the throat's bank acts, read back whole.
    for h in [Handoff::Feed { mouth: [3u8; 32], mu_s: 9, seq: 2 }, Handoff::Draw { mouth: [3u8; 32], mu_s: 1, seq: 3 }] {
        assert_eq!(h.citation(), [3u8; 32], "a bank act names its mouth");
        let signed = h.sign(&key);
        assert_eq!(decode_handoff_request(&encode_handoff(&signed)), Some(HandoffRequest::Signed(signed)));
    }
    let reading = TubeReading {
        from_place: crate::transducer::cga::hyperboloid_pole(),
        to_place: crate::transducer::cga::hyperboloid_pole(),
        from_level: Level::Curved(num_rational::BigRational::new(7.into(), 2.into())),
        to_level: Level::Flat { radius: num_rational::BigRational::new(3.into(), 1.into()), ratio: num_rational::BigRational::new(1.into(), 2.into()) },
        mass: (3, 2),
    };
    assert_eq!(decode_tube_reading(&encode_tube_reading(Some(&reading))), Some(Some(reading)));
    assert_eq!(decode_tube_reading(&encode_tube_reading(None)), Some(None));
    assert!(is_spin_plus_31(&one) && !is_spin_plus_31(&crate::transducer::cga::Mv::zero()));
    let offer = Offer { from_tier: [1u8; 32], to_tier: [2u8; 32], out_nets: vec![([3u8; 32], [-1, 0, 0, 0])], in_nets: vec![([4u8; 32], [1, 0, 0, 0])], charter: [0u8; 32], nonce: [9u8; 32] };
    let traverse = Handoff::Traverse { offer: Box::new(offer.clone()), mouth: mouth.citation() };
    assert_eq!(traverse.citation(), offer.citation(), "a traversal is the handoff its offer is");
    for h in [Handoff::Mouth(Box::new(mouth.clone())), Handoff::Close { id: mouth.citation() }, traverse] {
        let signed = h.sign(&key);
        assert_eq!(decode_handoff_request(&encode_handoff(&signed)), Some(HandoffRequest::Signed(signed)));
    }
    assert_eq!(decode_handoff_request(&encode_loops_read()), Some(HandoffRequest::Loops));
    let loops = vec![Loop { mouth: mouth.citation(), holonomy: one.clone(), tethered: true }, Loop { mouth: [6u8; 32], holonomy: one, tethered: false }];
    assert_eq!(decode_loops_reply(&encode_loops_reply(&loops)), Some(loops));
    for r in [
        HandoffRefusal::NoMouth,
        HandoffRefusal::Degenerate,
        HandoffRefusal::NotKeeper,
        HandoffRefusal::NotTheMouth,
        HandoffRefusal::Untariffed(Untariffed::ZeroDenominator),
        HandoffRefusal::Untariffed(Untariffed::Unordered),
        HandoffRefusal::Untariffed(Untariffed::NotWhole),
        HandoffRefusal::Untariffed(Untariffed::Flat),
        HandoffRefusal::NotALine,
        HandoffRefusal::NotIsometry,
        HandoffRefusal::MalformedTube,
        HandoffRefusal::NotSupersedable,
        HandoffRefusal::Untethered,
        HandoffRefusal::Starved,
        HandoffRefusal::Stale,
    ] {
        assert_eq!(decode_handoff_reply(&encode_handoff_reply(&Handed::Refused(r))), Some(Handed::Refused(r)));
    }
}

/// 78.127 — conversions out and in, and the reserve's designation, read back
/// whole; `rate · g` exact or none; a genesis frame still reads as a genesis.
#[test]
fn conversions_and_reserve_round_trip_whole() {
    use crate::anastome::dialogue::domain::{decode_domain_reply, decode_domain_request, encode_domain, encode_domain_reply, encode_reservation, DomainGenesis, DomainRequest, Founded, Reservation};
    let key = ed25519_dalek::SigningKey::from_bytes(&[5u8; 32]);
    let out = ConvertOut { tier: [1u8; 32], holder: [2u8; 32], identity: [3u8; 32], g: 10, rate: [(1, 2), (0, 1), (3, 5), (0, 1)], nonce: [4u8; 32] };
    assert_eq!(out.mu(), Some([5, 0, 6, 0]));
    assert_eq!(ConvertOut { rate: [(1, 3), (0, 1), (0, 1), (0, 1)], ..out.clone() }.mu(), None, "10/3 is no integer");
    assert_eq!(ConvertOut { g: 0, ..out.clone() }.mu(), None);
    let inn = ConvertIn { holder: [2u8; 32], tier: [1u8; 32], identity: [3u8; 32], g: 7, mu: [3, 0, 0, 0], nonce: [9u8; 32] };
    assert_ne!(out.citation(), inn.citation());
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
    use crate::anastome::dialogue::domain::{
        decode_bond_reading, decode_reserve_reading, encode_bond, encode_bond_read, encode_bond_reading, encode_reserve_read, encode_reserve_reading, Bond,
        ReserveReading,
    };
    for mu in [Some([4, 0, 2, 0]), None] {
        let b = Bond { domain: [6u8; 32], reserve: [1u8; 32], tier: [2u8; 32], mu, nonce: [3u8; 32] }.sign(&key);
        assert_eq!(decode_domain_request(&encode_bond(&b)), Some(DomainRequest::Bond(b)));
    }
    assert_eq!(decode_domain_request(&encode_bond_read([6u8; 32], [2u8; 32])), Some(DomainRequest::ReadBond { domain: [6u8; 32], tier: [2u8; 32] }));
    assert_eq!(decode_bond_reading(&encode_bond_reading(&[7, 0, 1, 0])), Some([7, 0, 1, 0]));
    let bonded = Founded::Bonded { at: Stamp { h: 2, vk: [8u8; 32], k: 1, i: 0 }, receipt: [1u8; 32], bond: [4, 0, 0, 0] };
    for f in [bonded, Founded::NotReserve, Founded::Short, Founded::Outstanding, Founded::Unbonded] {
        assert_eq!(decode_domain_reply(&encode_domain_reply(&f)), Some(f.clone()));
    }
    assert_eq!(decode_domain_request(&encode_reserve_read([6u8; 32])), Some(DomainRequest::ReadReserve([6u8; 32])));
    for r in [ReserveReading::default(), ReserveReading { current: Some([1u8; 32]), pending: Some([2u8; 32]), balance: [3, 0, 9, 1] }] {
        assert_eq!(decode_reserve_reading(&encode_reserve_reading(&r)), Some(r.clone()));
    }
    let at = Stamp { h: 2, vk: [8u8; 32], k: 1, i: 0 };
    for f in [Founded::Reserved { at, receipt: [1u8; 32], now: false }, Founded::NotATier, Founded::NotFounded] {
        assert_eq!(decode_domain_reply(&encode_domain_reply(&f)), Some(f.clone()));
    }
}
