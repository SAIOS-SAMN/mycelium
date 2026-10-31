// 78.160 P1 — THE PRESENTATION SERVED BY THE PHYSICS: a mind's residue
// deconstructed over the generators this court ruled, and the words that
// deconstruct it taught at the learner's expense.
//
//! The court verifies and teaches; it composes nothing for the mind (law 8).
//! What it reads is the motion the mind sent, against the presentation it
//! already holds (`Court::presentation`): the holes the motion names select
//! the generators, and the exact product of the generators' powers is the certificate.
//! A generator the mind was already taught is drag — held, free, not sent
//! again; one it lacks is lift, and the learner pays its magnitude, as the
//! fetch and the syllabus charge it (78.60(d)).

use crate::cognition::*;

/// WHAT ONE ASK WAS ANSWERED WITH: the words served, and the reading.
pub struct Presented {
    pub words: Vec<crate::anastome::dialogue::Taught>,
    pub answer: crate::anastome::dialogue::PresentationAnswer,
}

impl Court {
    /// 78.160 P1 — A MIND'S RESIDUE, DECONSTRUCTED: the motion read against
    /// this court's presentation on the hold's stroke, and each cited form
    /// the asker lacks taught, within `bytes`, while its balance pays for it.
    /// A motion whose coordinates do not read is answered unread, by name.
    pub fn present(
        &mut self,
        voice: [u8; 32],
        motion: &crate::transducer::cga::Mv,
        bytes: usize,
        reads: &mut crate::physics::Budget,
    ) -> Presented {
        use crate::anastome::dialogue::{PresentationAnswer, Taught};
        use crate::observe::{count, count_by, FETCH};
        count(&mut self.acts.fetch, &FETCH, "presentation: asked");
        let d = match self.presentation.deconstruct(motion, reads) {
            Ok(d) => d,
            Err(_) => {
                count(&mut self.acts.fetch, &FETCH, "presentation: unread");
                return Presented { words: Vec::new(), answer: PresentationAnswer::default() };
            }
        };
        count_by(&mut self.acts.fetch, &FETCH, "presentation: generators cited", d.cites.len() as u64);
        if d.composes {
            count(&mut self.acts.fetch, &FETCH, "presentation: composes");
        }
        let mut words = Vec::new();
        let mut used = 0usize;
        let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for (form, _, _) in &d.cites {
            let addr = form.as_bytes();
            if seen.contains(addr) {
                continue;
            }
            seen.insert(addr, ());
            // DRAG: a generator this court already taught the asker is held,
            // and is neither sent nor charged again.
            let key = super::syllabus::taught_key(addr, &voice);
            if self.taught_voice.contains(&key) {
                continue;
            }
            let Some((mag, tape)) = self.installed_word(addr).and_then(|w| Some((w.magnitude(), w.to_bytes()?))) else {
                continue;
            };
            let Some(at) = self.admitted_at(addr) else { continue };
            let width = crate::chronometry::Stamp::LEN + 4 + tape.len();
            if used + width > bytes || words.len() == usize::from(u8::MAX) {
                break;
            }
            // LIFT: the learner pays the word's magnitude from its own
            // account; a short learner is not taught it, and says so.
            let short = crate::economy::shortfall(&mag, &self.treasury.balance_mu(&voice));
            if !crate::economy::lanes_short(&short).is_empty() {
                count(&mut self.acts.fetch, &FETCH, "presentation: learner short");
                continue;
            }
            if mag.iter().any(|&c| c > 0) {
                let anchor = self.anchor();
                if let Err(why) = self.treasury.pay(voice, self.identity, mag, anchor) {
                    eprintln!("  presentation: a learner's payment refused after its balance was read: {why:?}");
                    continue;
                }
            }
            used += width;
            self.taught_voice.insert(key, ());
            count(&mut self.acts.fetch, &FETCH, "presentation: words served");
            words.push(Taught { at, word: tape });
        }
        let answer = PresentationAnswer { read: true, cites: d.cites, composes: d.composes, residue: d.residue, holes: d.holes };
        Presented { words, answer }
    }
}
