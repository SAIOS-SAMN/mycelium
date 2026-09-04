// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! One ed25519 identity per hypha. Minted on first boot, persisted beside
//! the ledger. Offer and Installed are signed; the verifying key rides Cover.

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

    pub fn sign(&self, msg: &[u8]) -> [u8; 64] {
        self.sk.sign(msg).to_bytes()
    }
}

pub fn verify(vk: &[u8; 32], msg: &[u8], sig: &[u8; 64]) -> bool {
    let Ok(key) = VerifyingKey::from_bytes(vk) else {
        return false;
    };
    let sig = Signature::from_bytes(sig);
    key.verify(msg, &sig).is_ok()
}
