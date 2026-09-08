// 24 — THE CLIENT KERNEL: the voice this client speaks as.


/// A kernel's stable mesh IDENTITY. The name deterministically fixes a 32-byte
/// address (its "voice"/seat), so a kernel keeps the same identity across
/// reconnects (residency — the seat outlives the socket). Consilience needs two
/// DISTINCT kernels to agree, so distinct names give distinct voices.
#[derive(Clone, Debug)]
pub struct Kernel {
    pub(crate) name: String,
    pub(crate) voice: [u8; 32],
}

impl Kernel {
    /// Seat a kernel by name, FOR TESTS ONLY. The seed is a KDF over the
    /// name, so the identity is a function of a string: reproducible, which
    /// is what a fixture wants, and derivable by anyone who knows the name,
    /// which is not an identity (46). A real voice is a keypair from the
    /// genesis manifest, seated by `from_voice`.
    #[cfg(test)]
    pub fn named(name: &str) -> Self {
        let voice = crate::court::identity::HyphaKey::vk_from_name(name);
        Kernel { name: name.to_string(), voice }
    }
    /// Seat a kernel on verifying-key bytes (home/voice.key).
    pub fn from_voice(voice: [u8; 32]) -> Self {
        Kernel {
            name: format!("{:02x}{:02x}", voice[0], voice[1]),
            voice,
        }
    }
    pub fn voice(&self) -> [u8; 32] {
        self.voice
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
