// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 46, 78.141 — THE VOICE: the Ed25519 identity a mind speaks as and signs
// what it is paid for with; the mesh record a voice home keeps.

//! The identity a mind speaks as.
//!
//! A voice home holds `voice.key` (the 32-byte seed) and `mesh` (the mesh
//! record: the bootstrap hypha the voice dials). [`load_or_mint`] reads the
//! key or mints one; [`load_or_migrate_voice`] reads the mesh record or
//! writes it once.

/// A mind's stable mesh identity: the verifying key it speaks as, and the
/// signing key where it was seated on one.
pub use mycelium::sdk::Voice;

/// The voice's Ed25519 key pair, loaded from a seed file or minted there
/// (`VoiceKey::load_or_mint`, `VoiceKey::load_only`).
pub use mycelium::court::identity::HyphaKey as VoiceKey;

/// A 32-byte key written as 64 lowercase hex digits, and read back.
pub use mycelium::court::identity::{hex32, parse_hex32};

/// The mesh record of a voice home, read or written once
/// (`load_or_migrate_voice(home, key)`); `Err` names a configuration
/// refusal.
pub use mycelium::network::mesh::{load_or_migrate_voice, MeshRecord};

/// The Ed25519 key types a voice signs and verifies with.
pub use ed25519_dalek::{SigningKey, VerifyingKey};

/// THE VOICE AT `path`, loaded from its seed file or minted there: the voice
/// seated on the key, and the key's signing half, which signs the acts the
/// voice is paid for (a solve, a take, a stake).
pub fn load_or_mint(path: &str) -> (Voice, SigningKey) {
    let key = VoiceKey::load_or_mint(path);
    (Voice::from_key(key.sk.clone()), key.sk)
}
