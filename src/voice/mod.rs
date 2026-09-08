// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 45, 46 — THE VOICE: an identity with a clock, the durable seat a kernel
//! speaks from, stamped with the k-step it last read. It utters the five
//! forms and the acts that carry them, and never its kernel's state.

use crate::anastome::dialogue::{self, CiteWire};

/// A kernel's voice: its seat and its clock.
#[derive(Clone, Debug)]
pub struct Voice {
    name: String,
    identity: [u8; 32],
    /// The k-step this voice last read from the judge — its temporal
    /// coordinate on the mesh's chain.
    k: u64,
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
        Voice { name: name.to_string(), identity, k: 0 }
    }

    /// Seat a voice on verifying-key bytes (home/voice.key).
    pub fn from_identity(identity: [u8; 32]) -> Voice {
        Voice {
            name: format!("{:02x}{:02x}", identity[0], identity[1]),
            identity,
            k: 0,
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

    /// The k-step this voice speaks at.
    pub fn k(&self) -> u64 {
        self.k
    }

    /// The voice read the judge's clock.
    pub fn at(&mut self, k: u64) {
        self.k = k;
    }

    // ── the five forms and the acts that carry them ──────────────────────

    /// A PERCEPT: two scenes as bodies with the frame and this voice's
    /// proof per germ.
    pub fn percept(
        &self,
        a: &[Vec<(i64, i64, u8)>],
        b: &[Vec<(i64, i64, u8)>],
        frame: &crate::arc::Frame,
        proof: &[Vec<CiteWire>],
    ) -> Vec<u8> {
        let mut out = vec![dialogue::ACT_PERCEPT];
        out.extend_from_slice(&crate::cognition::encode_percept(a, b, &self.identity, frame, proof));
        out
    }

    /// A DEMAND: a residual with this voice's citations (empty: irreducible
    /// for this voice).
    pub fn demand(&self, chain: &[(i64, i64, u8, u8)], cites: &[CiteWire]) -> Vec<u8> {
        dialogue::encode_demand_at(&self.identity, self.k, chain, cites)
    }

    /// A QUOTE request: "what is the highest-yield work?"
    pub fn quote(&self) -> Vec<u8> {
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
    pub fn solve(&self, id: &[u8; 32], outputs: &[crate::arc::Grid]) -> Vec<u8> {
        dialogue::encode_solve(&self.identity, id, outputs)
    }

    /// A TASK request: the next family to think through.
    pub fn task(&self) -> Vec<u8> {
        dialogue::encode_task_request(&self.identity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A voice's identity is its name's digest; its clock moves as it reads.
    #[test]
    fn a_voice_is_a_seat_with_a_clock() {
        let mut v = Voice::named("worker-a");
        assert_eq!(v.identity(), Voice::named("worker-a").identity());
        assert_ne!(v.identity(), Voice::named("worker-b").identity());
        assert_eq!(v.k(), 0);
        v.at(4208);
        assert_eq!(v.k(), 4208);
        assert_eq!(v.quote()[0], dialogue::ACT_QUOTE);
    }
}
