// Fixture constructors for the mind's tests (MU-STANDARDS 3.3): a scene, a
// glued pair, and a grid of digits, each built through the constructors the
// production code exports.

use crate::cognition::*;

pub fn scene(sr: i64, sc: i64) -> Vec<(i64, i64, u8)> {
vec![(sr, sc, 1), (sr, sc + 1, 1), (sr + 1, sc, 1)]
}

pub fn glue_pair(k: &mut Kernel, a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) {
k.ingest(a, b, [1; 32]);
let _ = k.ingest(a, b, [2; 32]);
k.glue_local();
}

/// 47.5's Λ on a mesh whose measured round trip to every home is one
/// stroke: `from_transport(1, aperture_cycle())`, on every listing. A test
/// that lapses states the transport it lapses against, because the kernel
/// names no constant and does not lapse against an unmeasured one.
pub fn one_stroke_transport(_addr: &[u8]) -> Option<u64> {
    Some(crate::physics::from_transport(1, crate::physics::aperture_cycle() as u64))
}

/// A home this node has measured no round trip to: nothing lapses.
pub fn unmeasured_transport(_addr: &[u8]) -> Option<u64> {
    None
}

/// A hypha's signing key from a name, for a receipt fixture that must be
/// SIGNED: 47.2 step 2 is a value (`Verified`), and a test reaches the
/// chain checks only through it. The seed is a KDF over the name, which is
/// what a fixture wants and is not an identity (46).
pub fn court_key(name: &str) -> crate::court::identity::HyphaKey {
    use ed25519_dalek::SigningKey;
    let seed = crate::court::identity::HyphaKey::seed_from_name(name);
    let sk = SigningKey::from_bytes(&seed);
    crate::court::identity::HyphaKey { vk: sk.verifying_key(), sk }
}

/// The prefix signed by `key` and verified under its own vk: the one way a
/// test hands `admit_receipt` step 2.
pub fn verified<'a>(
    prefix: &'a crate::court::admission::SignedPrefix,
    key: &crate::court::identity::HyphaKey,
) -> crate::court::admission::Verified<'a> {
    let sig = key.sign(&prefix.bytes());
    crate::court::admission::verify_signature(prefix, &key.vk_bytes(), &sig)
        .expect("a key verifies its own signature over its own prefix")
}

pub fn digits(rows: &[&str]) -> Vec<Vec<u8>> {
rows.iter()
    .map(|r| r.bytes().map(|b| b - b'0').collect())
    .collect()
}
