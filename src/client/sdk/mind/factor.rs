// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-SENSING §0, §3.9, §6.2; ROADMAP track 26 S4c — THE MIND SPEAKS WHAT IT
// PERCEIVED BEYOND THE ALPHABET: a task whose samples the letters do not
// spell (`Stop::BeyondAlphabet`) is still perceived at its own rank
// (`TaskPercept`), and every factorisation the mind holds of each train
// pair is stated to the court as a claim of factorisation, one claim per
// held factorisation: a vortex speaks per factorisation (rule 15), never
// reduced to one by a scalar.
//
//! Each claim states the ground the mind read where it read one symbol
//! (`Ground::One`); under a vortex of candidates, or none, it states no
//! ground, since the motion and the residue are read under none
//! (`Factorisation::of` reads no ground) and stating each candidate would
//! say the same claim again. The court reconstructs each claim and answers
//! a verdict (`dialogue::FactorVerdict`); an admitted claim pays nothing.
//!
//! Each claim states the perspective the percept was read under (track 26
//! S6; MU-SENSING §4): the court reads its copy of the pair through it.

use crate::client::sdk::*;
use crate::transducer::family::transduce::Ground;

impl Mind {
    /// THE CLAIMS OF FACTORISATION THE PERCEPT HELD UNDER `id` SPEAKS: per
    /// train pair, one signed claim per held factorisation, its motion and
    /// residue as the mind read them, in the order the vortex holds them.
    /// None where no percept is held under `id`, or where the voice holds
    /// no key (a claim names its voice by the voice's signature); a claim
    /// that does not fit its fields is not written.
    pub(crate) fn factorisation_claims(&self, id: &[u8; 32]) -> Vec<Vec<u8>> {
        let (Some(percept), Some(key)) = (self.board.percepts.get(id), self.hypha.voice().signer()) else { return Vec::new() };
        let ground = match &percept.ground {
            Ground::One(g) => Some(g.as_slice()),
            Ground::Vortex(_) | Ground::None => None,
        };
        let mut out = Vec::new();
        for (pair, p) in percept.train.iter().enumerate() {
            for f in p.factored.held() {
                if let Some(act) = dialogue::encode_factorisation(key, id, pair, &f.motion, &f.residue, &percept.perspective, ground) {
                    out.push(act);
                }
            }
        }
        out
    }

    /// SPEAK THE CLAIMS (`factorisation_claims`) on the court's carrier,
    /// each to the task's home; the verdict is read where it lands, and a
    /// claim the court does not admit is said with its verdict.
    pub(crate) fn speak_factorisations(&mut self, id: &[u8; 32]) {
        for act in self.factorisation_claims(id) {
            let task = *id;
            self.ask(move |s| s.speak_keyed(&task, act), move |_, body: io::Result<Vec<u8>>| {
                let Ok(body) = body else { return };
                let head: String = task.iter().take(4).map(|b| format!("{b:02x}")).collect();
                match dialogue::decode_factor_verdict(&body) {
                    Some(dialogue::FactorVerdict::Admitted) => {}
                    Some(dialogue::FactorVerdict::Residue { unclaimed, uncarried }) => {
                        eprintln!("  factorisation verdict · task {head}… · residue differs · unclaimed {} · uncarried {}", unclaimed.len(), uncarried.len());
                    }
                    Some(dialogue::FactorVerdict::Refused { reason }) => {
                        eprintln!("  factorisation verdict · task {head}… · refused · {}", String::from_utf8_lossy(&reason));
                    }
                    None => eprintln!("  factorisation verdict · task {head}… · no verdict read"),
                }
            });
        }
    }
}
