// 78.160 P2 — THE MIND ASKS ITS COURT FOR WHAT IT CANNOT SAY: each word
// vertex of a form it extracted whose whole motion no held word cites is a
// residue — pressure on its language — and the court is asked to
// deconstruct that motion over the presentation it holds.
//
//! The ask carries the motion itself (Cl(4,1), `Mv`), never a chart or a
//! cursor; the answer carries the words the mind lacked and the citations,
//! and the mind absorbs the words as it absorbs a syllabus page, at their
//! admitting stamps. A citation whose powers the court read composing to the
//! motion is kept as a mineral (`Epigenome::presented`) and becomes the
//! mind's own only where its derivation re-derives it (`heard_citation`).
//! A motion is asked once per language: the asks clear when the atlas root
//! moves, so the mind asks again once it holds more.

use crate::client::sdk::*;

impl Entity {
    /// ASK THE COURT for every uncited motion of `form`; answers how many
    /// motions the court's citations compose to exactly. A court that does
    /// not answer, or answers what does not decode, teaches nothing.
    pub(crate) fn ask_presentation(&mut self, form: &crate::transducer::sentence::Form) -> usize {
        use crate::transducer::sentence::presentation::{is_still, motion_key, vertex_motion};
        use crate::transducer::sentence::Vertex;
        let root = self.epigenome.atlas_root();
        if self.epigenome.asked_root.as_ref() != Some(&root) {
            self.epigenome.asked = crate::network::nerve::Keyed::new();
            self.epigenome.asked_root = Some(root);
        }
        // THE RESIDUES: word vertices whose derivation cites nothing and whose
        // whole motion moves something — each stop counted by name.
        let note = |h: &mut [u64; crate::observe::HEARING.len()], name: &str| {
            crate::observe::count(h, &crate::observe::HEARING, name)
        };
        note(&mut self.hearing, "presentation: forms");
        let mut motions: Vec<crate::transducer::cga::Mv> = Vec::new();
        for (_, v) in form.vertices.scoped() {
            let Vertex::Word { derivation, .. } = v else { continue };
            note(&mut self.hearing, "presentation: vertices");
            if !derivation.is_spelled() {
                note(&mut self.hearing, "presentation: citing");
                continue;
            }
            match vertex_motion(v).filter(|m| !is_still(m)) {
                Some(m) => motions.push(m),
                None => note(&mut self.hearing, "presentation: still"),
            }
        }
        let voice = self.session.kernel().voice();
        let mut budget = self.physics.stroke();
        let mut composed = 0usize;
        for m in motions {
            let Some(key) = motion_key(&m) else { continue };
            if self.epigenome.asked.contains(&key) || self.epigenome.presented.contains(&key) {
                continue;
            }
            // What the mind's own ruled words already say is said from them:
            // no court is asked for it.
            if crate::transducer::family::Crystal::read_word(&self.epigenome, &m, &[]).is_some_and(|r| r.composes) {
                continue;
            }
            self.epigenome.asked.insert(&key, ());
            crate::observe::count(&mut self.hearing, &crate::observe::HEARING, "presentation: asked");
            let ask = dialogue::encode_presentation_request(&voice, &m);
            let Ok(body) = self.session.speak_keyed(&voice, ask) else { continue };
            let Some((words, answer)) = dialogue::decode_presentation_reply(&body) else { continue };
            for t in &words {
                if self.absorb_taught_at(&t.word, Some(t.at), &mut budget).is_some() {
                    crate::observe::count(&mut self.hearing, &crate::observe::HEARING, "presentation: words");
                }
            }
            if answer.read && answer.composes && !answer.cites.is_empty() {
                self.epigenome.presented.insert(&key, answer.cites);
                crate::observe::count(&mut self.hearing, &crate::observe::HEARING, "presentation: composes");
                composed += 1;
            }
        }
        composed
    }
}
