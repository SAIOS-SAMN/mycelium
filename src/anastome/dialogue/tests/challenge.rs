// 78.125 — ACT_CHALLENGE's wire: every challenge, the finality and
// obstruction reads, and every answer read back whole; a byte left over is
// no request. And the register's parameters, at law version 4.

use crate::anastome::dialogue::challenge::*;
use crate::anastome::dialogue::uplink::ChildNet;
use crate::chronometry::Stamp;
use crate::transducer::cech::coefficient::Rates;

#[test]
fn every_challenge_and_every_answer_read_back_whole() {
    let key = ed25519_dalek::SigningKey::from_bytes(&[5u8; 32]);
    let fold = Stamp { h: 9, vk: [6u8; 32], k: 2, i: 1 };
    let entry = Entry { rows: vec![([1u8; 32], [0, 4, 0, 0])], declared: vec![([2u8; 32], [0, 4, 0, 0])] };
    let child = ChildNet {
        at: fold,
        rows: entry.rows.clone(),
        declared: entry.declared.clone(),
        included_rows: entry.rows.clone(),
        included_declared: entry.declared.clone(),
        rate: [(1, 1), (5, 9), (1, 1), (1, 1)],
    };
    for c in [
        Challenge::Demand { fold },
        Challenge::Evidence { fold, rows: entry.rows.clone(), declared: Vec::new(), children: vec![child], entries: vec![entry.clone()] },
        Challenge::Default { fold },
        Challenge::Correction { fold, child: vec![3u8; 64], holder: [7u8; 32], amount: -12 },
        Challenge::Open { fold, rows: entry.rows.clone(), declared: Vec::new(), children: Vec::new() },
        Challenge::Part { fold, entries: vec![entry.clone(), entry.clone()] },
    ] {
        let signed = c.sign(&key);
        let mut asked = encode_challenge(&signed);
        assert_eq!(decode_challenge_request(&asked), Some(ChallengeRequest::Signed(signed)));
        asked.push(0);
        assert_eq!(decode_challenge_request(&asked), None, "a byte left over is no request");
    }
    assert_eq!(decode_challenge_request(&encode_finality_request(fold)), Some(ChallengeRequest::Finality(fold)));
    assert_eq!(decode_challenge_request(&encode_obstructions_request([4u8; 32])), Some(ChallengeRequest::Obstructions([4u8; 32])));
    for a in [
        Challenged::Demanded { at: fold, receipt: [1u8; 32], deadline: 1033 },
        Challenged::Upheld { at: fold, receipt: [1u8; 32], prefix: [2u8; 32] },
        Challenged::Falsified { at: fold, receipt: [1u8; 32], unwound: 3 },
        Challenged::Discharged { at: fold, receipt: [1u8; 32] },
        Challenged::Refused(ChallengeRefusal::NotThePrefix),
        Challenged::Refused(ChallengeRefusal::Demanded),
        Challenged::Refused(ChallengeRefusal::NotOpened),
        Challenged::Streaming { head: [8u8; 32] },
    ] {
        assert_eq!(decode_challenge_reply(&encode_challenge_reply(&a)), Some(a.clone()), "{a:?}");
    }
    for f in [
        Finality::Final { upheld: true },
        Finality::Final { upheld: false },
        Finality::Provisional { until: 2048 },
        Finality::Blocked,
        Finality::Falsified,
        Finality::Unwound,
        Finality::Unrecorded,
        Finality::PreLine,
    ] {
        assert_eq!(decode_finality_reply(&encode_finality_reply(&f)), Some(f));
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

#[test]
fn the_register_serves_its_challenge_window_at_law_version_4() {
    use crate::physics::laws::codec::{decode_parameters_reply, encode_parameters_reply};
    let (version, params) = decode_parameters_reply(&encode_parameters_reply()).expect("reads whole");
    assert_eq!(version, 4);
    assert_eq!(params, vec![("challenge window".to_string(), crate::physics::laws::CHALLENGE_WINDOW)]);
}
