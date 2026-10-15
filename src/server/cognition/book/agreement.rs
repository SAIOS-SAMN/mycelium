// MU-CLIENT §5.5, 78.161(5) — AGREED LANGUAGE ATTESTED: a voice speaks a
// sentence another mind re-derived and holds one loop with it on every word,
// carrying that mind's signed HOLD; the court verifies the signature and
// seals the other mind's attestation on the word.
//
//! What the court checks, and nothing more: the frame opens under the
//! signature of the mind it names (78.161's chatter frame), it is a HOLD, it
//! names exactly this sentence, the mind is not the speaker, and this court
//! holds the word — ruled, or provisional on its docket. The loops
//! themselves are the two minds' (78.163); the court verifies that the
//! second mind signed them, never re-derives a conversation it was not in
//! (law 8). One `Attest` per (word, attesting mind), credited to that mind.

use crate::client::sdk::chatter::{Chatter, Said};
use crate::cognition::*;

impl Kernel {
    /// 78.161(5) — EVERY AGREEMENT A SPEECH CARRIES, verified and sealed:
    /// the attestations sealed, each as (word, attesting mind, the exact
    /// deviation per lane the attesting mind priced its hearing at — 78.112
    /// H5, signed in its HOLD).
    pub fn attest_agreements(&mut self, act: &crate::anastome::dialogue::SpeechAct) -> Vec<(Vec<u8>, [u8; 32], [num_rational::BigRational; 4])> {
        let mut sealed = Vec::new();
        for (tape, frame) in act.sentences.iter().zip(&act.agreements) {
            let Some(frame) = frame else { continue };
            let mut p = 0usize;
            let Some(e) = Element::from_bytes(tape, &mut p) else { continue };
            let addr = e.address();
            let Some(Chatter { speaker: peer, said: Said::Hold { said, deviation, .. }, .. }) = Chatter::open(frame) else { continue };
            if said.as_slice() != addr.as_bytes() || peer == act.voice {
                continue;
            }
            if self.word_at(addr.as_bytes()).is_none() && self.provisional.get(addr.as_bytes()).is_none() {
                continue;
            }
            let key = attestation_key(addr.as_bytes(), &peer);
            if self.attested.contains(&key) {
                continue;
            }
            self.attested.insert(key, ());
            self.append_step(crate::chronometry::ActKind::Attest, addr.as_bytes(), peer);
            // MU-CLIENT §5.6 (E3) — THE UNION'S ATTESTATION PAYS: the mind that
            // agreed is a party to the word's claim, as a second speaker of it
            // is (`list_sentence`), so its install pays every mind that agreed.
            // A word already installed was paid at its install, once.
            if self.provisional.contains(&addr) && self.treasury.escrow_of(addr.as_bytes()).is_some() {
                self.convergence.used_by(addr.as_bytes(), peer);
                let anchor = self.anchor();
                self.treasury.escrow_award(&addr, vec![peer], [0; 4], anchor);
                self.acts.pay("attestations joined to a claim", [0; 4]);
            }
            sealed.push((addr.as_bytes().to_vec(), peer, deviation));
        }
        sealed
    }
}

/// An attestation's key: the word's address, then the attesting mind.
fn attestation_key(addr: &[u8], peer: &[u8; 32]) -> Vec<u8> {
    let mut k = Vec::with_capacity(addr.len() + 32);
    crate::transducer::address::put_key_bytes(&mut k, addr);
    k.extend_from_slice(peer);
    k
}
