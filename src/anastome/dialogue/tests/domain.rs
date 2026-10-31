// 78.120 — ACT_DOMAIN's wire: a signed genesis and every answer read back
// whole; a byte left over is none.

use crate::anastome::dialogue::domain::{decode_domain, decode_domain_reply, encode_domain, encode_domain_reply, Founded, DomainGenesis};
use crate::chronometry::Stamp;

/// A signed genesis and every answer read back whole, and a byte left over
/// is none.
#[test]
fn domain_genesis_round_trips_with_answers() {
    let key = ed25519_dalek::SigningKey::from_bytes(&[2u8; 32]);
    let g = DomainGenesis { domain: key.verifying_key().to_bytes(), tiers: vec![[1u8; 32], [3u8; 32]] };
    let signed = g.sign(&key);
    let mut asked = encode_domain(&signed);
    assert_eq!(decode_domain(&asked), Some(signed));
    asked.push(0);
    assert_eq!(decode_domain(&asked), None, "a byte left over is none");
    let at = Stamp { h: 4, vk: [5u8; 32], k: 1, i: 0 };
    for f in [
        Founded::Sealed { at, receipt: [7u8; 32] },
        Founded::Unsigned,
        Founded::Unordered,
        Founded::AlreadyFounded,
        Founded::TierHeld,
        Founded::LineRefused,
    ] {
        assert_eq!(decode_domain_reply(&encode_domain_reply(&f)), Some(f.clone()), "{f:?}");
    }
}
