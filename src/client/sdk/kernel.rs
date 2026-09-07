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
    /// Seat a kernel by name. The seed is a KDF over the name; the identity
    /// carried everywhere is the verifying key.
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
