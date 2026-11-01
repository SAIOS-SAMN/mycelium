// 78.125 — ACT_CHALLENGE's wire: every challenge, the finality and
// obstruction reads, and every answer read back whole; a byte left over is
// no request. And the register's parameters, at law version 5.

use crate::anastome::dialogue::challenge::*;
use crate::anastome::dialogue::uplink::{Child, ChildNet, FoldAt};
use crate::chronometry::Stamp;
use crate::transducer::cech::coefficient::Rates;

/// Every challenge and every answer reads back whole, and a byte left over
/// is no request.
#[test]
fn challenge_round_trips_with_answers() {
    let key = ed25519_dalek::SigningKey::from_bytes(&[5u8; 32]);
    let stamp = Stamp { h: 9, vk: [6u8; 32], k: 2, i: 1 };
    let entry = Entry { rows: vec![([1u8; 32], [0, 4, 0, 0])], declared: vec![([2u8; 32], [0, 4, 0, 0])] };
    let child = ChildNet {
        child: Child::Sealed(FoldAt::sealed(stamp)),
        rows: entry.rows.clone(),
        declared: entry.declared.clone(),
        included_rows: entry.rows.clone(),
        included_declared: entry.declared.clone(),
        rate: [(1, 1), (5, 9), (1, 1), (1, 1)],
    };
    // 78.138 — a fold the court sealed, and a line carried inside one.
    for fold in [FoldAt::sealed(stamp), FoldAt { stamp, path: vec![2, 0] }] {
        for c in [
            Challenge::Demand { fold: fold.clone() },
            Challenge::Evidence { fold: fold.clone(), rows: entry.rows.clone(), declared: Vec::new(), children: vec![child.clone()], entries: vec![entry.clone()] },
            Challenge::Default { fold: fold.clone() },
            Challenge::Correction { fold: fold.clone(), child: vec![3u8; 64], holder: [7u8; 32], amount: -12 },
            Challenge::Open { fold: fold.clone(), rows: entry.rows.clone(), declared: Vec::new(), children: Vec::new() },
            Challenge::Part { fold: fold.clone(), entries: vec![entry.clone(), entry.clone()] },
            Challenge::Withdraw { fold: fold.clone(), demand: Stamp { h: 12, ..stamp } },
        ] {
            let signed = c.sign(&key);
            let mut asked = encode_challenge(&signed);
            assert_eq!(decode_challenge_request(&asked), Some(ChallengeRequest::Signed(signed)));
            asked.push(0);
            assert_eq!(decode_challenge_request(&asked), None, "a byte left over is no request");
        }
        assert_eq!(decode_challenge_request(&encode_finality_request(fold.clone())), Some(ChallengeRequest::Finality(fold)));
    }
    let fold = FoldAt::sealed(stamp);
    assert_eq!(decode_challenge_request(&encode_obstructions_request([4u8; 32])), Some(ChallengeRequest::Obstructions([4u8; 32])));
    assert_eq!(decode_challenge_request(&encode_line_head_request([5u8; 32])), Some(ChallengeRequest::LineHead([5u8; 32])));
    for (head, at, held) in [([0u8; 32], None, [0; 4]), ([9u8; 32], Some(FoldAt { stamp, path: vec![1] }), [3, 0, 0, u64::MAX])] {
        let r = LineHead { head, at, held: held.into() };
        assert_eq!(decode_line_head_reply(&encode_line_head_reply(&r)), Some(r));
    }
    for a in [
        Challenged::Demanded { at: stamp, receipt: [1u8; 32], deadline: Some(1033) },
        Challenged::Demanded { at: stamp, receipt: [1u8; 32], deadline: None },
        Challenged::Upheld { at: stamp, receipt: [1u8; 32], prefix: [2u8; 32] },
        Challenged::Falsified { at: stamp, receipt: [1u8; 32], unwound: 3 },
        Challenged::Discharged { at: stamp, receipt: [1u8; 32] },
        Challenged::Refused(ChallengeRefusal::NotThePrefix),
        Challenged::Refused(ChallengeRefusal::Demanded),
        Challenged::Refused(ChallengeRefusal::NotOpened),
        Challenged::Streaming { head: [8u8; 32] },
        Challenged::Reopened { at: stamp, receipt: [1u8; 32], closes: Some(2057) },
        Challenged::Reopened { at: stamp, receipt: [1u8; 32], closes: None },
        Challenged::Refused(ChallengeRefusal::Withdrawn),
        Challenged::Refused(ChallengeRefusal::NotTheDemand),
    ] {
        assert_eq!(decode_challenge_reply(&encode_challenge_reply(&a)), Some(a.clone()), "{a:?}");
    }
    for f in [
        Finality::Final { upheld: true },
        Finality::Final { upheld: false },
        Finality::Provisional { opened: 1024, closed: Some(2048) },
        Finality::Provisional { opened: 1024, closed: None },
        Finality::Blocked,
        Finality::Falsified,
        Finality::Unwound,
        Finality::Unrecorded,
        Finality::PreLine,
    ] {
        for bonded in [false, true] {
            assert_eq!(decode_finality_reply(&encode_finality_reply(&f, bonded)), Some((f, bonded)));
            for toll_held in [false, true] {
                let r = FinalityReading { finality: f, bonded, toll_held };
                assert_eq!(decode_finality_reading(&encode_finality_reading(&r)), Some(r));
            }
        }
    }
    let open = vec![OpenObstruction {
        fold,
        child: vec![3u8; 64],
        parent: vec![4u8; 64],
        holonomy: Rates::of([(1, 1), (1, 2), (1, 1), (1, 1)]).expect("positive"),
    }];
    assert_eq!(decode_obstructions_reply(&encode_obstructions_reply(&open)), Some(open));
    // THE TIER LINE: a head is the chain of its entries, from where it began.
    let head = line_head([0; 32], std::slice::from_ref(&entry));
    assert_eq!(line_head(head, &[]), head, "no entry, no step");
    assert_ne!(line_head([0; 32], &[entry.clone(), entry]), head, "each entry steps the head");
}

/// The register serves at law version 5 the frame budget, the one open
/// protocol number, and no challenge window: the window is a sealed reach
/// and return of every other keeper, not a number.
#[test]
fn parameters_reply_serves_frame_budget() {
    use crate::physics::laws::codec::{decode_parameters_reply, encode_parameters_reply};
    let (version, params) = decode_parameters_reply(&encode_parameters_reply()).expect("reads whole");
    assert_eq!(version, 5);
    assert_eq!(params, vec![("frame budget".to_string(), crate::network::wire::BODY_BUDGET as u64)]);
}
