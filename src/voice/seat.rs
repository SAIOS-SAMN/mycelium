// 45, 46 — THE SEAT: the identity a kernel speaks from, its own receipt
// chain, and the forms it utters.

use crate::anastome::dialogue::{self, CiteWire};
use crate::chronometry::{ActKind, ReceiptChain};

/// A kernel's voice: its seat and its clock.
#[derive(Clone, Debug)]
pub struct Voice {
    name: String,
    identity: [u8; 32],
    /// THIS VOICE'S OWN RECEIPT CHAIN (45.1): one receipt per act it has
    /// had admitted — a form stood, an utterance spoken, a page learned —
    /// each naming what was admitted and linked to the one before. Its K
    /// is the chain's length. Every duration a kernel reads is a count of
    /// its own K (45.4), so this is never a number read off another
    /// kernel's reply: a quote carries the BOOK's k-step, which is a
    /// coordinate on the hypha's chain and not this voice's clock. It was a
    /// bare `u64` stepped by `advance()`, a count with no receipt behind it
    /// (Stage 11 I25).
    chain: ReceiptChain,
}

impl Voice {
    /// Seat a voice by name, FOR TESTS ONLY. The seed is a KDF over the
    /// name, so the identity is a function of a string: reproducible, which
    /// is what a fixture wants, and derivable by anyone who knows the name,
    /// which is not an identity (46). A real voice is a keypair from the
    /// genesis manifest, seated by `from_identity`.
    #[cfg(test)]
    pub fn named(name: &str) -> Voice {
        let identity = crate::court::identity::HyphaKey::vk_from_name(name);
        Voice { name: name.to_string(), identity, chain: ReceiptChain::with_vk(identity) }
    }

    /// Seat a voice on verifying-key bytes (home/voice.key).
    pub fn from_identity(identity: [u8; 32]) -> Voice {
        Voice {
            name: format!("{:02x}{:02x}", identity[0], identity[1]),
            identity,
            chain: ReceiptChain::with_vk(identity),
        }
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

    /// The k-step this voice speaks at: the receipts on its own chain.
    pub fn k(&self) -> u64 {
        self.chain.next_step_index()
    }

    /// The voice's own receipt chain, read whole: what it stood, uttered
    /// and learned, in the order it was admitted.
    pub fn chain(&self) -> &ReceiptChain {
        &self.chain
    }

    /// AN ACT OF THIS VOICE WAS ADMITTED (45.1): a form stood in the
    /// dialect, an utterance the hypha did not refuse, a page learned. A
    /// receipt naming the kind and what was admitted (`delta`, the act's
    /// address written out) is sealed on this voice's chain under this
    /// voice's key, and the new K is the coordinate that act carries; the
    /// kind's name is the operator digest, never reasoned on. A voice that
    /// speaks nothing seals nothing, and no reply from another kernel moves
    /// this. The chain cites no peer receipt yet: the mind admits none
    /// (47.2 on the mind is open), so its height is its own K and its
    /// parents are empty.
    /// `mu` is what this act measured, per lane (45.1). The caller holds
    /// the object it is sealing and reads `magnitude()` off it; a voice
    /// that seals an address it holds nothing for seals the zero vector,
    /// which is the reading and not a default.
    pub fn seal(&mut self, kind: ActKind, mu: [u64; 4], delta: &[u8]) -> u64 {
        self.chain.append(kind, mu, delta, kind.operator(), Vec::new());
        self.k()
    }

    // ── the forms and the acts that carry them ───────────────────────────
    // The percept and the demand that stood here carried this voice's
    // painted sites and chains to a court (law 8, ruled 2026-09-10); a voice speaks
    // sentences (`Entity::speak`), probes, quotes, learns and solves.

    /// A QUOTE request: "what is the highest-yield work?" `None` where the
    /// act does not fit the lengths its frame writes (47.1).
    pub fn quote(&self) -> Option<Vec<u8>> {
        dialogue::encode_quote_request(&self.identity)
    }

    /// A PROBE: the concluded sentence on a listing.
    pub fn probe(&self, target: &[u8; 32], cites: &[CiteWire]) -> Vec<u8> {
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
