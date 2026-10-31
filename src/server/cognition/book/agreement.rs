// MU-CLIENT §5.5, 78.161(5) — AGREED LANGUAGE ATTESTED: a voice speaks a word
// another mind heard and holds one loop with it on every ring, carrying that
// mind's signed HOLD; the court verifies the signature, verifies the word ON
// THE EVIDENCE — what it does on the court's own copy of the scene it is bound
// to — and seals the other mind's attestation on the word. Where that mind
// holds a word of its own on the same scene, another vertex with the same
// section, the court seals the CONVERGENCE: two constructions, one conclusion.
//
//! What the court checks: the frame opens under the signature of the mind it
//! names (78.161's chatter frame), it is a HOLD, it names exactly this word,
//! the mind is not the speaker, this court holds the word — ruled, or
//! provisional on its docket — and the word is TRUE ON ITS SCENE: decompressed
//! by its own letters onto the family it is bound to (`family::local::
//! word_local`, the reader its install is paid by), its section is not empty.
//! A word bound to no family, or with no section there, is evidenced by
//! nothing, and no attestation is sealed on it. The loops are the two minds'
//! (78.163); the court re-derives no conversation it was not in (law 8) — it
//! reads the word on its own family, as it reads every word it pays.
//!
//! CONVERGENCE (MU-CLIENT §5.5): the words bound to the same scene that the
//! attesting mind discovered (`task_listings`, the scene's words, read at the
//! scene's key) are read on the same family; each that is another vertex with
//! the same section — neither closes a site beyond the other — is sealed as
//! the speaker's `Attest` on the attesting mind's word, once. Same section,
//! different words: separate algorithmic constructions reaching one
//! conclusion, verified on the evidence, never one object read twice.

use crate::anastome::dialogue::chatter::{Chatter, Said};
use crate::cognition::*;
use crate::transducer::family::local::{word_local, Local};

/// ONE ATTESTATION SEALED: the word, the mind that attested it, the exact
/// deviation per lane that mind priced its hearing at (78.112 H5, signed in
/// its HOLD), and the attesting mind's own words on the scene that converged
/// with it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attested {
    pub word: Vec<u8>,
    pub by: [u8; 32],
    pub deviation: [num_rational::BigRational; 4],
    pub converged: Vec<Vec<u8>>,
}

/// Every pair's section empty: the word changed nothing true there.
fn empty(l: &Local) -> bool {
    l.sites.iter().all(crate::network::nerve::Keyed::is_empty)
}

/// A word the court measures by its own section (`Court::discharge_on`, W).
fn sectioned(word: &Element) -> bool {
    word.is_connection() && !word.is_colour_word()
}

impl Court {
    /// 78.161(5), MU-CLIENT §5.5 — EVERY AGREEMENT A SPEECH CARRIES, verified
    /// on the evidence and sealed, with the convergences it carries.
    pub fn attest_agreements(&mut self, act: &crate::anastome::dialogue::SpeechAct) -> Vec<Attested> {
        let mut sealed = Vec::new();
        for (tape, frame) in act.sentences.iter().zip(&act.agreements) {
            let Some(frame) = frame else { continue };
            let mut p = 0usize;
            let Some(e) = Element::from_bytes(tape, &mut p) else { continue };
            let address = e.address();
            let Some(Chatter { speaker: peer, said: Said::Hold { said, deviation, .. }, .. }) = Chatter::verify(frame) else { continue };
            if said.as_slice() != address.as_bytes() || peer == act.voice {
                continue;
            }
            if self.word_at(address.as_bytes()).is_none() && self.provisional.get(address.as_bytes()).is_none() {
                continue;
            }
            let key = attestation_key(address.as_bytes(), &peer);
            if self.attested.contains(&key) {
                continue;
            }
            // THE WORD ON ITS SCENE: the family it is bound to, its section
            // there. Bound to none, or true nowhere on it: no evidence, and
            // no attestation.
            let task = self.speech_task.get(address.as_bytes()).or_else(|| self.listing_task.get(address.as_bytes())).cloned();
            let family = task.as_ref().and_then(|t| self.task_family.get(t.as_bytes())).cloned();
            let (Some(task), Some(family)) = (task, family.filter(|_| sectioned(&e))) else { continue };
            let heard = word_local(&e, &family);
            if empty(&heard) {
                continue;
            }
            self.attested.insert(key, ());
            self.append_step(crate::chronometry::ActKind::Attest, address.as_bytes(), peer);
            // MU-CLIENT §5.6 (E3) — THE UNION'S ATTESTATION PAYS: the mind that
            // agreed is a party to the word's claim, as a second speaker of it
            // is (`list_sentence`), so its install pays every mind that agreed.
            // A word already installed was paid at its install, once.
            if self.provisional.contains(&address) && self.treasury.escrow_of(address.as_bytes()).is_some() {
                self.convergence.used_by(address.as_bytes(), peer);
                let anchor = self.anchor();
                self.treasury.escrow_award(&address, vec![peer], [0; 4], anchor);
                self.acts.pay("attestations joined to a claim", [0; 4]);
            }
            let converged = self.converge_on(&task, &family, &e, &heard, peer, act.voice);
            sealed.push(Attested { word: address.as_bytes().to_vec(), by: peer, deviation, converged });
        }
        sealed
    }

    /// MU-CLIENT §5.5 — THE ATTESTING MIND'S OWN WORDS ON THE SCENE THAT
    /// CONVERGE WITH `word`: bound to `task`, discovered by `peer`, another
    /// vertex, the same section on `family`. Each is sealed once as the
    /// speaker's `Attest` on it — the speaker's construction reached it —
    /// and answered.
    fn converge_on(
        &mut self,
        task: &crate::transducer::address::Address,
        family: &crate::transducer::family::Family,
        word: &Element,
        heard: &Local,
        peer: [u8; 32],
        speaker: [u8; 32],
    ) -> Vec<Vec<u8>> {
        let on_scene: Vec<crate::transducer::address::Address> = match self.task_listings.get(task.as_bytes()) {
            Some(v) => v.clone(),
            None => Vec::new(),
        };
        let mut out = Vec::new();
        for at in on_scene {
            if self.discoverer_of(at.as_bytes()) != Some(peer) {
                continue;
            }
            let Some(peer_word) = self.word_at(at.as_bytes()).or_else(|| self.provisional.get(at.as_bytes())).cloned() else { continue };
            if !sectioned(&peer_word) || peer_word.same_vertex(word) {
                continue;
            }
            let own = word_local(&peer_word, family);
            if !empty(&heard.beyond(&own, family)) || !empty(&own.beyond(heard, family)) {
                continue;
            }
            let key = attestation_key(at.as_bytes(), &speaker);
            if self.attested.contains(&key) {
                continue;
            }
            self.attested.insert(key, ());
            self.append_step(crate::chronometry::ActKind::Attest, at.as_bytes(), speaker);
            out.push(at.as_bytes().to_vec());
        }
        out
    }
}

/// An attestation's key: the word's address, then the attesting mind.
fn attestation_key(address: &[u8], peer: &[u8; 32]) -> Vec<u8> {
    let mut k = Vec::with_capacity(address.len() + 32);
    crate::transducer::address::put_key_bytes(&mut k, address);
    k.extend_from_slice(peer);
    k
}
