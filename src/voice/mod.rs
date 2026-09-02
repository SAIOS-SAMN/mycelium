// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! VOICE — how a kernel speaks.
//!
//! A voice is an identity with a clock: the durable seat a kernel speaks
//! from, stamped with the universal time and the k-step it last read. It
//! speaks in the five forms of the anastome — percept, demand, quote,
//! teach, settle — and in the acts that carry them (probe, syllabus,
//! solve, digests, fetch). Every form is an utterance: operators carried,
//! verifiable by any listener in its own dialect. A voice never carries
//! its kernel's state; it carries what the kernel concluded.
//!
//! Entity, hypha, and court each hold one voice. The session
//! (`crate::client::sdk::Session`) is the mouth a voice speaks through
//! and the ear a reply is perceived through.

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
    /// Seat a voice by name — the emblem digest is its durable identity.
    pub fn named(name: &str) -> Voice {
        let identity = *blake3::hash(format!("mycelium-kernel:{name}").as_bytes()).as_bytes();
        Voice { name: name.to_string(), identity, k: 0 }
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
        dialogue::encode_demand(&self.identity, chain, cites)
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
