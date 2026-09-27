// 45, 46, 78.49 — THE SEAT: the identity a mind speaks from, and the acts its
// speech carries. A voice only speaks: it is the signal of the mind in
// cohesive speech. It holds no ledger — the ledger is the court's, which
// seals and adjudicates — and no clock: the mind's K is the mind's own.

use crate::anastome::dialogue::{self, CiteWire};

/// A mind's voice: the identity it speaks from.
#[derive(Clone, Debug)]
pub struct Voice {
    name: String,
    identity: [u8; 32],
}

impl Voice {
    /// Seat a voice by name, FOR TESTS ONLY. The seed is a KDF over the
    /// name, so the identity is a function of a string: reproducible, which
    /// is what a fixture wants, and derivable by anyone who knows the name,
    /// which is not an identity (46). A real voice is a keypair from the
    /// genesis manifest, seated by `from_identity`.
    #[cfg(test)]
    pub fn named(name: &str) -> Voice {
        Voice::from_identity(crate::court::identity::HyphaKey::vk_from_name(name))
    }

    /// Seat a voice on verifying-key bytes (home/voice.key).
    pub fn from_identity(identity: [u8; 32]) -> Voice {
        Voice { name: format!("{:02x}{:02x}", identity[0], identity[1]), identity }
    }

    /// Mint or restore an ed25519 voice at `path` (32-byte seed, same as a hypha key).
    pub fn load_or_mint(path: &str) -> Voice {
        let key = crate::court::identity::HyphaKey::load_or_mint(path);
        Voice::from_identity(key.vk_bytes())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn identity(&self) -> [u8; 32] {
        self.identity
    }

    // ── the acts speech carries ──────────────────────────────────────────
    // The percept and the demand that stood here carried this voice's
    // painted sites and chains to a court (law 8, ruled 2026-09-10); a voice
    // speaks sentences (`Entity::speak`), probes, quotes, learns and solves.

    /// A QUOTE request: "what is the highest-yield work?" `None` where the
    /// act does not fit the lengths its frame writes (47.1).
    pub fn quote(&self) -> Option<Vec<u8>> {
        dialogue::encode_quote_request(&self.identity)
    }

    /// A PROBE: the concluded sentence on a listing. `None` where a cited
    /// address cannot be framed (LD0).
    pub fn probe(&self, target: &[u8; 32], cites: &[CiteWire]) -> Option<Vec<u8>> {
        dialogue::encode_probe(&self.identity, target, cites)
    }

    /// A SYLLABUS request: the ratified stream from this voice's cursor.
    pub fn syllabus(&self) -> Vec<u8> {
        dialogue::encode_syllabus_request(&self.identity)
    }

    /// A SOLVE: the outputs this voice speaks for a dispatched family.
    /// `None` where the act does not fit the lengths its frame writes.
    pub fn solve(&self, id: &[u8; 32], outputs: &[crate::arc::Grid]) -> Option<Vec<u8>> {
        dialogue::encode_solve(&self.identity, id, outputs)
    }

    /// A TASK request: the next family to think through.
    pub fn task(&self) -> Vec<u8> {
        dialogue::encode_task_request(&self.identity)
    }
}
