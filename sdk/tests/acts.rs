// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.94, MU-PRECURSORS §4.2 — THE SDK SPEAKS EVERY ACT THE SERVER JUDGES:
// each act a party asks of a hypha (an observation, a transition, the seat
// relay, and the equivocation proof anyone may present) has an encoder on
// the SDK's surface, its first byte the act's coordinate and its code in
// the act table; and the acts the SDK gained here round-trip through both
// ends of their codecs by decoded exact values (Concordance 1.4).

use mycelium::physics::actions::{Action, Kind, ACTIONS};
use mycelium_sdk::house;
use mycelium_sdk::ACT_TABLE;

/// X1 (#160) — the crossing a universe act carries: one that moves no body,
/// at the default point, over one phase of period 1.
fn crossing() -> mycelium_sdk::forms::Crossing {
    use mycelium_sdk::forms::{Crossing, Place};
    Crossing::still(Place { at: mycelium::chronometry::tuniversal::TPoint::default(), parents: Vec::new() }, vec![1])
}

fn key(b: u8) -> ed25519_dalek::SigningKey {
    ed25519_dalek::SigningKey::from_bytes(&[b; 32])
}

const V: [u8; 32] = [3; 32];

/// A root under one key, measuring `lane`, as a receipt section: two lanes
/// give two roots at one place of one signer's line.
fn section(lane: u64, sig: u8) -> Option<house::ReceiptSection> {
    let mut c = mycelium::chronometry::ReceiptChain::with_signer([0x5c; 32]);
    let r = c.append(mycelium::chronometry::ActKind::List, [lane, 0, 0, 0], Vec::new(), [0; 32], Vec::new())?.clone();
    Some(house::ReceiptSection {
        prefix: mycelium::court::admission::SignedPrefix {
            genesis: [7; 32],
            epoch: 2,
            prev_receipt_hash: r.prev_hash(),
            at: r.stamp.place(),
            height: r.stamp.h,
            chart: vec![1, 2],
            payload: r.stamp.key(),
        },
        sig: [sig; 64],
        receipt: r,
    })
}

/// Two roots at one place of one signer's line.
fn fork() -> Option<(house::ReceiptSection, house::ReceiptSection)> {
    Some((section(1, 0x11)?, section(2, 0x22)?))
}

/// Is this action one a party asks of a hypha, and so one the SDK speaks?
/// Read off the action's kind: an observation, a transition, the seat
/// relay; and the equivocation proof, an exchange anyone may present
/// (MU-XSEPTUM, MU-HOSTING §5.3).
fn party_act(a: Action) -> bool {
    matches!(a.kind(), Kind::Observation | Kind::Transition | Kind::Relay) || a == Action::Equivocation
}

/// 78.94 — EVERY ACT A PARTY ASKS OF THE SERVER HAS AN SDK ENCODER. Each row
/// is the action and a frame written by an encoder the SDK exposes; the
/// frame leads with the action's coordinate, the code stands in the SDK's
/// act table, and the rows cover every party act of the action table.
#[test]
fn sdk_speaks_every_act_the_server_judges() {
    use mycelium_sdk::{laws, readings, session, task, wallet};
    let none: &[session::ChartOf<Vec<u8>>] = &[];
    let upto = readings::Place { k: 1, i: 0 };
    let fold = house::TierFold { tier: V, prefix: [0; 32], rows: Vec::new(), declared: Vec::new(), children: Vec::new(), handoffs: Vec::new(), transitions: Vec::new() };
    let (a, b) = fork().expect("two roots append");
    let rows: Vec<(Action, Vec<u8>)> = vec![
        (Action::Quote, session::encode_quote_request(&V).expect("a quote frames")),
        (Action::Handoff, house::encode_handoff_read([1; 32])),
        (Action::Evidence, session::encode_evidence_request(&V)),
        (Action::Task, task::encode_task_request(&V)),
        (Action::Solve, task::encode_solve_spoken(&key(9), &[5; 32], &[], None, None, &task::Perspective::of_sample(2)).expect("a solve frames")),
        (Action::Syllabus, session::encode_syllabus_request(&V)),
        (Action::Digests, session::encode_digests_request(&V)),
        (Action::Fetch, session::encode_fetch(&V, none, 1)),
        (Action::Probe, session::encode_probe(&V, [4u8; 32], &[]).expect("a probe frames")),
        (Action::Peers, session::encode_peers_request(&V)),
        (Action::Speech, session::encode_speech(&V, [5u8; 32], 0, &[]).expect("a speech frames")),
        (Action::Equivocation, house::equivocation_proof(&a, &b).expect("two roots at one place")),
        (Action::Receipts, readings::encode_receipts_request(&V, &V, None, upto)),
        (Action::Entries, readings::encode_entries_request(&V, &V, None, upto, 0)),
        (Action::Observe, readings::encode_query(b"").expect("a query frames")),
        (Action::Laws, laws::encode_register_request()),
        (Action::Why, readings::encode_why(mycelium::anastome::dialogue::why::WhyAsk::Own(upto))),
        (Action::Wallet, wallet::encode_wallet(&V)),
        (Action::Uplink, house::encode_uplink(&fold.sign_crossed(crossing(), &key(9)).expect("signs")).expect("a fold frames")),
        (Action::Domain, house::encode_map_read([1; 32])),
        (Action::Challenge, house::encode_line_head_request([1; 32])),
        (Action::Seat, house::encode_read_listing(&[1; 32], None)),
    ];
    for (a, frame) in &rows {
        assert!(frame.starts_with(&[a.byte()]), "{a:?}: the frame leads with the action's coordinate");
        assert!(ACT_TABLE.iter().any(|(code, _)| *code == a.byte()), "{a:?} stands in the SDK's act table");
        assert!(party_act(*a), "{a:?} is an act a party asks");
    }
    for a in ACTIONS.iter().copied().filter(|a| party_act(*a)) {
        assert!(rows.iter().any(|(r, _)| *r == a), "{a:?}: the server judges it and the SDK speaks no encoder for it");
    }
}

/// MU-HOSTING §5.3, MU-XSEPTUM — AN EQUIVOCATION PROOF ROUND-TRIPS, AND
/// ONLY A FORK IS ONE. Two roots signed under one key at one place frame as
/// the act and read back, on the court's side, as the two sections in the
/// order presented; one receipt twice, or two places, is no proof. The
/// court's answers read back as ruled.
#[test]
fn equivocation_proof_round_trips() {
    let (a, b) = fork().expect("two roots append");
    let proof = house::equivocation_proof(&a, &b).expect("a fork at one place is a proof");
    assert_eq!(proof.get(1), Some(&house::EQUIVOCATION_VERSION));
    assert_eq!(Some(proof.clone()), house::encode_equivocation(&a, &b), "the proof is the act's own encoding");
    let (ga, gb) = mycelium::anastome::dialogue::decode_equivocation(&proof).expect("the court reads both sections");
    assert_eq!((ga.sig, gb.sig), (a.sig, b.sig), "in the order presented");
    let hash = |s: &house::ReceiptSection| s.receipt.chain_hash.bytes();
    assert_eq!((hash(&ga), hash(&gb)), (hash(&a), hash(&b)), "each receipt whole");
    assert_eq!(house::equivocation_proof(&a, &a.clone()), None, "one receipt twice is no fork");
    let mut later = b.clone();
    later.receipt.stamp.i += 1;
    assert_eq!(house::equivocation_proof(&a, &later), None, "two places of the line are no equivocation");
    let mut other = b;
    other.receipt.stamp.vk = [0x5d; 32];
    assert_eq!(house::equivocation_proof(&a, &other), None, "two signers are no equivocation");
    for r in [house::Ruled::Applied, house::Ruled::Inert, house::Ruled::Refused(b"origin".to_vec())] {
        use mycelium::anastome::dialogue::encode_ruled;
        let body = encode_ruled(&r).expect("an answer frames");
        let read = house::decode_ruled(&body).expect("the answer reads");
        assert_eq!(encode_ruled(&read), Some(body), "read back as the answer it was");
    }
}

/// 78.126–78.134 — THE HANDOFF FAMILY ROUND-TRIPS: a signed handoff, its
/// standing read, the orbits and a tube read as the court reads each frame;
/// the court's answers as the SDK reads them.
#[test]
fn handoff_acts_round_trip() {
    use mycelium::anastome::dialogue::handoff::{decode_handoff_request, encode_handoff_reply, encode_orbits_reply, encode_read_reply, HandoffRequest};
    let signed = house::Handoff::Accept { id: [6; 32] }.sign(&key(8));
    assert_eq!(decode_handoff_request(&house::encode_handoff(&signed).expect("encodes")), Some(HandoffRequest::Signed(signed)));
    assert_eq!(decode_handoff_request(&house::encode_handoff_read([6; 32])), Some(HandoffRequest::Read([6; 32])));
    assert_eq!(decode_handoff_request(&house::encode_orbits_read()), Some(HandoffRequest::Orbits));
    assert_eq!(decode_handoff_request(&house::encode_tube_read([7; 32])), Some(HandoffRequest::TubeRead([7; 32])));
    let stamp = mycelium::chronometry::Stamp { h: 4, vk: V, k: 2, i: 1 };
    for h in [
        house::Handed::Sealed { at: stamp, receipt: [1; 32], id: [6; 32], state: house::HandoffState::Offered { opened: 4 } },
        house::Handed::Refused(house::HandoffRefusal::UnknownId),
    ] {
        assert_eq!(house::decode_handoff_reply(&encode_handoff_reply(&h)), Some(h));
    }
    let read = Some((house::HandoffState::Accepted, true, false));
    assert_eq!(house::decode_read_reply(&encode_read_reply(read)), Some(read));
    assert_eq!(house::decode_read_reply(&encode_read_reply(None)), Some(None));
    assert_eq!(house::decode_orbits_reply(&encode_orbits_reply(&[])), Some(Vec::new()));
    let no_tube = mycelium::anastome::dialogue::tube::encode_tube_reading(None);
    assert_eq!(house::decode_tube_reading(&no_tube), Some(None));
}

/// 78.154 — TRAVERSE, THE LEDGER AND THE MAP ROUND-TRIP: a signed traverse
/// and the ledger and map reads as the court reads each frame; the court's
/// answers as the SDK reads them.
#[test]
fn traverse_ledger_and_map_round_trip() {
    use mycelium::anastome::dialogue::atlas::{encode_ledger_reply, encode_map_reading, encode_map_reply};
    use mycelium::anastome::dialogue::crossing::encode_traverse_reply;
    use mycelium::anastome::dialogue::domain::{decode_domain_request, DomainRequest};
    use mycelium_sdk::session::TPoint;
    let t = house::Traverse {
        from: [1; 32],
        payer: [2; 32],
        mouth: [3; 32],
        side: 0,
        at: TPoint::default(),
        point_a: TPoint::default(),
        point_b: TPoint::default(),
        entry_a: [4; 32],
        u_a: 1,
        entry_b: [5; 32],
        u_b: 2,
        route: vec![[6; 32]],
        debit: None,
        credit: None,
        nonce: [7; 32],
    };
    let signed = t.sign(&key(2));
    assert_eq!(decode_domain_request(&house::encode_traverse(&signed)), Some(DomainRequest::Traverse(Box::new(signed))));
    let refused = house::Crossed::Refused(house::CrossRefusal::UnknownDomain);
    assert_eq!(house::decode_traverse_reply(&encode_traverse_reply(&refused)), Some(refused));
    let entry = house::LedgerEntry { domain: [6; 32], u: 1, commitment: [1; 32], events: [2; 32], point: TPoint::default(), parent: [0; 32], applied: Vec::new(), nonce: [8; 32] };
    let signed = entry.sign(crossing(), &key(30)).expect("signs");
    assert_eq!(decode_domain_request(&house::encode_ledger(&signed).expect("encodes")), Some(DomainRequest::Ledger(Box::new(signed))));
    assert_eq!(decode_domain_request(&house::encode_ledger_read([6; 32], 9)), Some(DomainRequest::ReadLedger { domain: [6; 32], after: 9 }));
    assert_eq!(decode_domain_request(&house::encode_map_read([6; 32])), Some(DomainRequest::ReadMap([6; 32])));
    for r in [house::Ledgered::Sealed { entry: [7; 32], h: 12, point: TPoint::default() }, house::Ledgered::Refused(house::LedgerRefusal::Parent)] {
        assert_eq!(house::decode_ledger_reply(&encode_ledger_reply(&r)), Some(r));
    }
    let mapped = house::Mapped::Sealed { record: [7; 32], h: 3, point: TPoint::default() };
    assert_eq!(house::decode_map_reply(&encode_map_reply(&mapped)), Some(mapped));
    assert_eq!(house::decode_map_reading(&encode_map_reading(&[])), Some(Vec::new()));
}

/// CONCORDANCE 4.5 — THE CHALLENGE WINDOW'S READS ROUND-TRIP: a fold's
/// finality, a tier's line head and a domain's open obstructions as the
/// court reads each frame; the court's answers as the SDK reads them.
#[test]
fn challenge_reads_round_trip() {
    use mycelium::anastome::dialogue::challenge::{
        decode_challenge_request, encode_finality_reply, encode_line_head_reply, encode_obstructions_reply, ChallengeRequest,
    };
    let fold = house::FoldAt::sealed(mycelium::chronometry::Stamp { h: 9, vk: V, k: 1, i: 0 });
    assert_eq!(decode_challenge_request(&house::encode_finality_request(fold.clone())), Some(ChallengeRequest::Finality(fold.clone())));
    assert_eq!(decode_challenge_request(&house::encode_line_head_request([2; 32])), Some(ChallengeRequest::LineHead([2; 32])));
    assert_eq!(decode_challenge_request(&house::encode_obstructions_request([3; 32])), Some(ChallengeRequest::Obstructions([3; 32])));
    for (f, bonded) in [(house::Finality::Final { upheld: true }, true), (house::Finality::Blocked, false)] {
        assert_eq!(house::decode_finality_reply(&encode_finality_reply(&f, bonded)), Some((f, bonded)));
    }
    let head = house::LineHead { head: [4; 32], at: Some(fold), held: mycelium_sdk::wallet::Mu::zero() };
    assert_eq!(house::decode_line_head_reply(&encode_line_head_reply(&head)), Some(head));
    assert_eq!(house::decode_obstructions_reply(&encode_obstructions_reply(&[])), Some(Vec::new()));
}
