// 24 — THE VOICE: the identity a mind speaks as.


/// A mind's stable mesh IDENTITY. The name deterministically fixes a 32-byte
/// address (its voice/seat), so a mind keeps the same identity across
/// reconnects (residency — the seat outlives the socket). Consilience needs two
/// DISTINCT voices to agree, so distinct names give distinct voices.
///
/// 78.141 — a voice seated on its KEY signs what names it as a payee (a
/// solve); one seated on verifying-key bytes alone speaks, and what it would
/// be paid for is refused unsigned.
#[derive(Clone, Debug)]
pub struct Voice {
    key: [u8; 32],
    signer: Option<ed25519_dalek::SigningKey>,
}

impl Voice {
    /// Seat a voice by name, FOR TESTS ONLY. The seed is a KDF over the
    /// name, so the identity is a function of a string: reproducible, which
    /// is what a fixture wants, and derivable by anyone who knows the name,
    /// which is not an identity (46). A real voice is a keypair from the
    /// genesis manifest, seated by `from_key`.
    #[cfg(test)]
    pub(crate) fn named(name: &str) -> Self {
        let sk = ed25519_dalek::SigningKey::from_bytes(&crate::court::identity::HyphaKey::seed_from_name(name));
        Voice { key: sk.verifying_key().to_bytes(), signer: Some(sk) }
    }
    /// Seat a voice on verifying-key bytes alone: it speaks, it signs nothing.
    pub fn from_voice(key: [u8; 32]) -> Self {
        Voice { key, signer: None }
    }
    /// 78.141 — seat a voice on its key (home/voice.key): its identity is the
    /// key's verifying half, and it signs.
    pub fn from_key(sk: ed25519_dalek::SigningKey) -> Self {
        let key = sk.verifying_key().to_bytes();
        Voice { signer: Some(sk), ..Voice::from_voice(key) }
    }
    /// The 32-byte verifying key this voice speaks as.
    pub fn id(&self) -> [u8; 32] {
        self.key
    }
    /// The key this voice signs with, where it was seated on one.
    pub(crate) fn signer(&self) -> Option<&ed25519_dalek::SigningKey> {
        self.signer.as_ref()
    }
}
