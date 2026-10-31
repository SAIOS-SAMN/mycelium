// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// CONCORDANCE §0 SIGNING — THE AGREEMENT'S HASH, SERVED, AND A KERNEL'S
// SIGNATURE OVER IT, VERIFIED: the hash is BLAKE3 of the exact bytes of
// `docs/MU-CONCORDANCE.md` at build time; a kernel signs it with its key.
//
//! From v0.3.0 the hash is served (`ACT_LAWS` mode 7) and a signature is
//! accepted and verified (`ACT_LAWS` mode 8); nothing is refused for lacking
//! one. Refusal of an unsigned kernel takes effect in the following release.

use ed25519_dalek::Verifier;

/// THE AGREEMENT'S EXACT BYTES, embedded at build time.
pub const CONCORDANCE: &[u8] = include_bytes!("../../../docs/MU-CONCORDANCE.md");

/// THE AGREEMENT'S HASH: BLAKE3 of [`CONCORDANCE`], computed by `build.rs`
/// over the same file (the tests claim it equals
/// `chronometry::payload_hash(CONCORDANCE)`).
pub const CONCORDANCE_HASH: [u8; 32] = include!(concat!(env!("OUT_DIR"), "/concordance_hash.rs"));

/// The domain a kernel's signature is taken over, ahead of the hash.
const SIGNED_CONCORDANCE: &[u8] = b"mycelium concordance v1";

/// The bytes a carried signature takes: key, hash, signature.
pub const SIGNATURE_LEN: usize = 32 + 32 + 64;

/// A KERNEL'S SIGNATURE OVER A VERSION OF THE CONCORDANCE: its key, the hash
/// it signed, and the Ed25519 signature over the domain and the hash.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConcordanceSignature {
    pub kernel_key: [u8; 32],
    pub hash: [u8; 32],
    pub sig: [u8; 64],
}

/// WHY A SIGNATURE IS REFUSED, by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConcordanceRefusal {
    /// The hash signed is not this build's [`CONCORDANCE_HASH`]: another
    /// version of the agreement.
    Mismatched,
    /// The signature does not verify under the kernel's key.
    Unsigned,
    /// The bytes carry no signature: a length other than
    /// [`SIGNATURE_LEN`], or a kernel key that is no Ed25519 point.
    Malformed,
}

impl ConcordanceRefusal {
    /// The refusal's byte on the wire.
    pub const fn byte(self) -> u8 {
        match self {
            ConcordanceRefusal::Mismatched => 1,
            ConcordanceRefusal::Unsigned => 2,
            ConcordanceRefusal::Malformed => 3,
        }
    }

    /// The refusal a byte names, or none.
    pub const fn of(b: u8) -> Option<ConcordanceRefusal> {
        match b {
            1 => Some(ConcordanceRefusal::Mismatched),
            2 => Some(ConcordanceRefusal::Unsigned),
            3 => Some(ConcordanceRefusal::Malformed),
            _ => None,
        }
    }
}

fn signed_bytes(hash: &[u8; 32]) -> Vec<u8> {
    let mut b = SIGNED_CONCORDANCE.to_vec();
    b.extend_from_slice(hash);
    b
}

impl ConcordanceSignature {
    /// A kernel's signature over `hash`, by its key.
    pub fn sign(key: &ed25519_dalek::SigningKey, hash: [u8; 32]) -> ConcordanceSignature {
        use ed25519_dalek::Signer;
        let sig = key.sign(&signed_bytes(&hash)).to_bytes();
        ConcordanceSignature { kernel_key: key.verifying_key().to_bytes(), hash, sig }
    }

    /// The signature's bytes: `[kernel key 32][hash 32][sig 64]`.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(SIGNATURE_LEN);
        out.extend_from_slice(&self.kernel_key);
        out.extend_from_slice(&self.hash);
        out.extend_from_slice(&self.sig);
        out
    }

    /// A signature read whole, or [`ConcordanceRefusal::Malformed`].
    pub fn from_bytes(b: &[u8]) -> Result<ConcordanceSignature, ConcordanceRefusal> {
        let malformed = ConcordanceRefusal::Malformed;
        if b.len() != SIGNATURE_LEN {
            return Err(malformed);
        }
        Ok(ConcordanceSignature {
            kernel_key: b.get(0..32).and_then(|s| s.try_into().ok()).ok_or(malformed)?,
            hash: b.get(32..64).and_then(|s| s.try_into().ok()).ok_or(malformed)?,
            sig: b.get(64..128).and_then(|s| s.try_into().ok()).ok_or(malformed)?,
        })
    }
}

/// VERIFY A KERNEL'S SIGNATURE against this build's agreement: the hash is
/// [`CONCORDANCE_HASH`], the key is an Ed25519 point, and the signature
/// verifies under it. Each refusal named.
pub fn verify_concordance_signature(s: &ConcordanceSignature) -> Result<(), ConcordanceRefusal> {
    let key = ed25519_dalek::VerifyingKey::from_bytes(&s.kernel_key).map_err(|_| ConcordanceRefusal::Malformed)?;
    if s.hash != CONCORDANCE_HASH {
        return Err(ConcordanceRefusal::Mismatched);
    }
    let sig = ed25519_dalek::Signature::from_bytes(&s.sig);
    key.verify(&signed_bytes(&s.hash), &sig).map_err(|_| ConcordanceRefusal::Unsigned)
}

/// The verdict on a carried signature's bytes, read and verified.
pub fn verify_concordance_bytes(b: &[u8]) -> Result<(), ConcordanceRefusal> {
    verify_concordance_signature(&ConcordanceSignature::from_bytes(b)?)
}
