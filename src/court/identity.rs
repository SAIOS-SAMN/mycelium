// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! One ed25519 identity per hypha. Minted on first boot, persisted beside
//! the ledger. Signs per-lane cover roots and ruled roots only — never an
//! individual Offer, Installed, or fetch. The signature bounds who may be
//! slashed for a false root; replay decides what is believed.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::{OsRng, RngCore};

pub struct HyphaKey {
    pub sk: SigningKey,
    pub vk: VerifyingKey,
}

impl HyphaKey {
    pub fn load_or_mint(path: &str) -> Self {
        if let Ok(bytes) = std::fs::read(path) {
            if bytes.len() == 32 {
                let seed: [u8; 32] = bytes.as_slice().try_into().unwrap();
                let sk = SigningKey::from_bytes(&seed);
                let vk = sk.verifying_key();
                return HyphaKey { sk, vk };
            }
        }
        let mut seed = [0u8; 32];
        OsRng.fill_bytes(&mut seed);
        let sk = SigningKey::from_bytes(&seed);
        let vk = sk.verifying_key();
        let _ = std::fs::write(path, seed);
        HyphaKey { sk, vk }
    }

    pub fn vk_bytes(&self) -> [u8; 32] {
        self.vk.to_bytes()
    }

    /// KDF over a seat name: a seed for an ed25519 voice, not a digest of
    /// mathematical content. The identity carried everywhere is the vk.
    pub fn seed_from_name(name: &str) -> [u8; 32] {
        *blake3::hash(format!("mycelium-kernel:{name}").as_bytes()).as_bytes()
    }

    /// Verifying key of the voice seated at `name`.
    pub fn vk_from_name(name: &str) -> [u8; 32] {
        let seed = Self::seed_from_name(name);
        let sk = SigningKey::from_bytes(&seed);
        sk.verifying_key().to_bytes()
    }

    /// Log tag and ledger prefix: first two bytes of the verifying key.
    pub fn prefix(&self) -> String {
        let b = self.vk_bytes();
        format!("{:02x}{:02x}", b[0], b[1])
    }

    pub fn sign(&self, msg: &[u8]) -> [u8; 64] {
        self.sk.sign(msg).to_bytes()
    }
}

pub fn hex32(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn parse_hex32(s: &str) -> Option<[u8; 32]> {
    let s = s.trim();
    if s.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).ok()?;
    }
    Some(out)
}

pub fn parse_hex64(s: &str) -> Option<[u8; 64]> {
    let s = s.trim();
    if s.len() != 128 {
        return None;
    }
    let mut out = [0u8; 64];
    for i in 0..64 {
        out[i] = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).ok()?;
    }
    Some(out)
}

pub fn verify(vk: &[u8; 32], msg: &[u8], sig: &[u8; 64]) -> bool {
    let Ok(key) = VerifyingKey::from_bytes(vk) else {
        return false;
    };
    let sig = Signature::from_bytes(sig);
    key.verify(msg, &sig).is_ok()
}
