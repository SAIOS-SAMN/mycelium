// CONCORDANCE §0 SIGNING — the agreement's hash is the hash of its exact
// bytes; a kernel's signature over it is accepted, and each refusal is
// named: another version's hash, a signature that does not verify, bytes
// that carry no signature. Served and presented on `ACT_LAWS` modes 7 and 8,
// read back by decoded exact values.

use super::super::codec::*;
use super::super::concordance::*;

fn kernel() -> ed25519_dalek::SigningKey {
    ed25519_dalek::SigningKey::from_bytes(&[7u8; 32])
}

/// The served hash is the payload hash of the agreement's embedded bytes.
#[test]
fn concordance_hash_equals_payload_hash_of_text() {
    assert_eq!(CONCORDANCE_HASH, crate::chronometry::payload_hash(CONCORDANCE));
    assert!(!CONCORDANCE.is_empty());
}

/// A kernel's signature over this build's hash is accepted, and survives
/// its bytes.
#[test]
fn signature_over_hash_is_accepted() {
    let s = ConcordanceSignature::sign(&kernel(), CONCORDANCE_HASH);
    assert_eq!(verify_concordance_signature(&s), Ok(()));
    assert_eq!(ConcordanceSignature::from_bytes(&s.to_bytes()), Ok(s));
    assert_eq!(verify_concordance_bytes(&s.to_bytes()), Ok(()));
}

/// A signature over another version's hash is refused as mismatched.
#[test]
fn signature_refuses_other_hash() {
    let mut other = CONCORDANCE_HASH;
    other[0] ^= 1;
    let s = ConcordanceSignature::sign(&kernel(), other);
    assert_eq!(verify_concordance_signature(&s), Err(ConcordanceRefusal::Mismatched));
}

/// A signature that does not verify under the key is refused as unsigned.
#[test]
fn signature_refuses_unverified_sig() {
    let mut s = ConcordanceSignature::sign(&kernel(), CONCORDANCE_HASH);
    s.sig[0] ^= 1;
    assert_eq!(verify_concordance_signature(&s), Err(ConcordanceRefusal::Unsigned));
    let other = ed25519_dalek::SigningKey::from_bytes(&[8u8; 32]);
    let s = ConcordanceSignature { kernel_key: other.verifying_key().to_bytes(), ..ConcordanceSignature::sign(&kernel(), CONCORDANCE_HASH) };
    assert_eq!(verify_concordance_signature(&s), Err(ConcordanceRefusal::Unsigned));
}

/// Bytes of another length, or a key that is no Ed25519 point, are refused
/// as malformed.
#[test]
fn signature_refuses_malformed_bytes() {
    let bytes = ConcordanceSignature::sign(&kernel(), CONCORDANCE_HASH).to_bytes();
    assert_eq!(verify_concordance_bytes(&bytes[..SIGNATURE_LEN - 1]), Err(ConcordanceRefusal::Malformed));
    let mut longer = bytes.clone();
    longer.push(0);
    assert_eq!(verify_concordance_bytes(&longer), Err(ConcordanceRefusal::Malformed));
    let off_curve = (0u8..=255).map(|b| [b; 32]).find(|k| ed25519_dalek::VerifyingKey::from_bytes(k).is_err()).expect("a key off the curve");
    let s = ConcordanceSignature { kernel_key: off_curve, ..ConcordanceSignature::sign(&kernel(), CONCORDANCE_HASH) };
    assert_eq!(verify_concordance_signature(&s), Err(ConcordanceRefusal::Malformed));
}

/// Modes 7 and 8 round-trip: the hash read whole, the signature presented
/// as its bytes, and each verdict read back by name.
#[test]
fn concordance_modes_round_trip() {
    assert_eq!(decode_laws_request(&encode_concordance_request()), Some(LawsRequest::Concordance));
    assert_eq!(decode_concordance_reply(&encode_concordance_reply()), Some(CONCORDANCE_HASH));
    assert_eq!(decode_concordance_reply(&encode_concordance_reply()[1..]), None);
    let s = ConcordanceSignature::sign(&kernel(), CONCORDANCE_HASH);
    assert_eq!(decode_laws_request(&encode_signature_request(&s)), Some(LawsRequest::Signature(s.to_bytes())));
    let verdicts = [Ok(()), Err(ConcordanceRefusal::Mismatched), Err(ConcordanceRefusal::Unsigned), Err(ConcordanceRefusal::Malformed)];
    for v in verdicts {
        assert_eq!(decode_signature_reply(&encode_signature_reply(v)), Some(v));
    }
    assert_eq!(decode_signature_reply(&[1, 0]), None, "a refusal byte no refusal names");
}
