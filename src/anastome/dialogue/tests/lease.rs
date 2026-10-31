use super::super::lease::*;

fn key(b: u8) -> ed25519_dalek::SigningKey {
    ed25519_dalek::SigningKey::from_bytes(&[b; 32])
}

/// A LEASE AND ITS CLOSE CROSS THE WIRE AS SIGNED: each reads back as
/// written, neither reads as the other or as a stake, and a reply reads
/// back with its lease, its lanes or its refusal.
#[test]
fn encode_lease_and_close_round_trip_signed() {
    let payer = key(1);
    let act = LeaseAct {
        payer: payer.verifying_key().to_bytes(),
        provider: [9; 32],
        mu: [4, 0, 2, 1],
        window: 50,
        jobs: vec![[7; 32], [8; 32]],
        nonce: [3; 32],
    };
    let signed = act.clone().sign(&payer).expect("signs");
    let bytes = encode_lease(&signed).expect("writes");
    assert_eq!(decode_lease(&bytes), Some(signed.clone()), "the lease reads back");
    assert!(decode_close(&bytes).is_none(), "and never as a close");
    assert!(super::super::wallet::decode_stake(&bytes).is_none(), "nor as a stake");
    assert!(crate::court::identity::verify(&act.payer, &act.signed_bytes().expect("writes"), &signed.sig), "the payer's signature verifies");

    let close = CloseLease { payer: act.payer, lease: act.citation().expect("named"), nonce: [4; 32] }.sign(&payer);
    let cbytes = encode_close(&close);
    assert_eq!(decode_close(&cbytes), Some(close), "the close reads back");
    assert!(decode_lease(&cbytes).is_none(), "and never as a lease");

    for r in [Ok(([5; 32], [1, 2, 3, 4])), Err(LeaseRefusal::Leased), Err(LeaseRefusal::UnknownLease)] {
        assert_eq!(decode_lease_reply(&encode_lease_reply(&r)), Some(r), "the reply reads back");
    }
}
