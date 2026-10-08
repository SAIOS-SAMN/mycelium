// 24 — THE CLIENT KERNEL: the voice this client speaks as.


/// A kernel's stable mesh IDENTITY. The name deterministically fixes a 32-byte
/// address (its "voice"/seat), so a kernel keeps the same identity across
/// reconnects (residency — the seat outlives the socket). Consilience needs two
/// DISTINCT kernels to agree, so distinct names give distinct voices.
///
/// 78.141 — a kernel seated on its KEY signs what names it as a payee (a
/// solve); one seated on verifying-key bytes alone speaks, and what it would
/// be paid for is refused unsigned.
#[derive(Clone, Debug)]
pub struct Kernel {
    pub(crate) name: String,
    pub(crate) voice: [u8; 32],
    signer: Option<ed25519_dalek::SigningKey>,
}

impl Kernel {
    /// Seat a kernel by name, FOR TESTS ONLY. The seed is a KDF over the
    /// name, so the identity is a function of a string: reproducible, which
    /// is what a fixture wants, and derivable by anyone who knows the name,
    /// which is not an identity (46). A real voice is a keypair from the
    /// genesis manifest, seated by `from_key`.
    #[cfg(test)]
    pub fn named(name: &str) -> Self {
        let sk = ed25519_dalek::SigningKey::from_bytes(&crate::court::identity::HyphaKey::seed_from_name(name));
        Kernel { name: name.to_string(), voice: sk.verifying_key().to_bytes(), signer: Some(sk) }
    }
    /// Seat a kernel on verifying-key bytes alone: it speaks, it signs nothing.
    pub fn from_voice(voice: [u8; 32]) -> Self {
        Kernel {
            name: format!("{:02x}{:02x}", voice[0], voice[1]),
            voice,
            signer: None,
        }
    }
    /// 78.141 — seat a kernel on its key (home/voice.key): its voice is the
    /// key's verifying half, and it signs.
    pub fn from_key(sk: ed25519_dalek::SigningKey) -> Self {
        let voice = sk.verifying_key().to_bytes();
        Kernel { signer: Some(sk), ..Kernel::from_voice(voice) }
    }
    pub fn voice(&self) -> [u8; 32] {
        self.voice
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    /// The key this kernel signs with, where it was seated on one.
    pub fn signer(&self) -> Option<&ed25519_dalek::SigningKey> {
        self.signer.as_ref()
    }
}
