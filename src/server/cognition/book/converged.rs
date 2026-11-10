// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! K3 (78.177 §4a(1), ruled 2026-11-06) — THE MINDS WHOSE WORDS CONVERGED
//! WITH AN INSTALLED ONE, read when the install pays and never stored. A
//! mind converged with the word at `address` where this court sealed its
//! `Attest` on that word (`attest_agreements`, K1–K2) and it discovered a
//! word of its own on the same scene that is another vertex with the same
//! section (`converge_on`'s own reading). Its share of the frame is its own
//! word's section on the family (`word_local`, O1). The install pays them
//! out of its composition premium (`Treasury::pay_converging`).

use super::agreement::{attestation_key, empty, sectioned};
use crate::cognition::*;
use crate::transducer::family::local::word_local;

impl Court {
    /// K3 — EACH MIND WHOSE WORD CONVERGED WITH `word` AT `address`, with
    /// its share of the frame. Empty where the word is bound to no family,
    /// has no section, or met no other mind's construction.
    pub(crate) fn converging_minds(&self, address: &[u8], word: &Element) -> Vec<([u8; 32], crate::economy::Mu)> {
        let Some(task) = self.speech_task.get(address).or_else(|| self.listing_task.get(address)).cloned() else { return Vec::new() };
        let Some(family) = self.task_family.get(task.as_bytes()) else { return Vec::new() };
        if !sectioned(word) {
            return Vec::new();
        }
        let own = word_local(word, family);
        if empty(&own) {
            return Vec::new();
        }
        let discoverer = self.discoverer_of(address);
        let on_scene: &[crate::transducer::address::Address] = match self.task_listings.get(task.as_bytes()) {
            Some(v) => v,
            None => &[],
        };
        let mut out: Vec<([u8; 32], crate::economy::Mu)> = Vec::new();
        for at in on_scene {
            let Some(mind) = self.discoverer_of(at.as_bytes()) else { continue };
            if Some(mind) == discoverer || out.iter().any(|(m, _)| *m == mind) || !self.attested.contains(&attestation_key(address, &mind)) {
                continue;
            }
            let Some(theirs) = self.word_at(at.as_bytes()).or_else(|| self.provisional.get(at.as_bytes())) else { continue };
            if !sectioned(theirs) || theirs.same_vertex(word) {
                continue;
            }
            let local = word_local(theirs, family);
            if !empty(&own.beyond(&local, family)) || !empty(&local.beyond(&own, family)) {
                continue;
            }
            out.push((mind, local.mu));
        }
        out
    }
}
