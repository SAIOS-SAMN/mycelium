// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 78.177 (ruled 2026-11-06), MU-CLIENT §5.6 (E2) — A HELD QUESTION STATED
//! AS A DEMAND A COURT CAN MEASURE. A question this mind raised on a heard
//! word that carried its evidence (`chatter::Evidence`, read true on its
//! scene) can be carried to a court; one with no evidence stays the
//! conversation's (E1: a family the court does not hold measures nothing).
//!
//! THE DEMAND'S BODIES ARE THE HEARD WORD'S (ruling §4a(2)): on each pair of
//! the scene the evidence names, the pair's residue links (`residue_of`, the
//! sites where before and after differ) standing at the sites of the word's
//! own section there (`word_local`, the reader every word is paid by). Each
//! pair's bodies stand as a `Source` demand over their chart, as an
//! unreached frame of an extraction stands (`family::finish`).
//!
//! The closer reaches the demand through the public `ACT_SPEECH` listing
//! (ruling §4a(3)), as any demand is reached: the court's quote, read by
//! chart, answered by a held closed form (`Mind::answer_demands`). It
//! asserts the closed twist; the asker, the listing's speaker, is paid
//! nothing (ruling §4a(4); `Court::pay_convergence`, 5.7(d)).

use crate::client::sdk::chatter::Evidence;
use crate::client::sdk::Mind;
use crate::transducer::element::Element;
use crate::transducer::sentence::{Demand, SectionLink};
use crate::transducer::stalk::Invariant;

/// ONE DEMAND A QUESTION STANDS AS: its kind, its chart, its bodies.
pub(crate) type Listed = (Demand, Invariant, Vec<SectionLink>);

/// THE HEARD WORD'S BODIES ON ITS SCENE, AS DEMANDS: per pair of `family`,
/// the pair's residue links at the sites of `word`'s section there, typed
/// as a `Source` over their chart. Empty where the word has no section on
/// the family (it is no connection, or its section is empty): a word true
/// nowhere there names no body a court could measure.
pub(crate) fn heard_bodies(word: &Element, family: &crate::transducer::family::Family) -> Vec<Listed> {
    let local = crate::transducer::family::local::word_local(word, family);
    let mut out = Vec::new();
    for ((x, y), at) in family.iter().zip(&local.sites) {
        if at.is_empty() {
            continue;
        }
        // A pair whose residue cannot be written on the family's one ground
        // names no body (track 26 S2).
        let Ok(residue) = crate::transducer::family::residue_of(Some(x), y) else { continue };
        let bodies: Vec<SectionLink> = residue.into_iter().filter(|l| at.contains(&crate::transducer::family::site_key(l.site))).collect();
        if bodies.is_empty() {
            continue;
        }
        let kind = Demand::Source { multiplicity: crate::transducer::sentence::pieces(&bodies).len() };
        if kind.typed().is_err() {
            continue;
        }
        out.push((kind, crate::transducer::stalk::invariant(&bodies), bodies));
    }
    out
}

/// 78.177 §4a(2) — THE HOST FORM WITH THE QUESTION ADDED: a sentence of the
/// asker's own, closed on the scene, its form read back and each demand
/// stood on it (`Form::stand_demand`, the one filer of a demand's bodies, as
/// `Form::contract` stands one). `None` where the host is not a closed
/// form, or there is no demand to add.
pub(crate) fn with_question(host: &Element, listed: Vec<Listed>) -> Option<crate::transducer::sentence::Form> {
    if listed.is_empty() {
        return None;
    }
    let mut f = crate::transducer::sentence::from_element_form(host).filter(|f| f.is_closed())?;
    for (kind, chart, bodies) in listed {
        f.stand_demand(kind, &chart, bodies);
    }
    Some(f)
}

impl Mind {
    /// 78.177 — THE SENTENCE THE QUESTION AT `key` IS LISTED BY, with the
    /// scene's family and its citation: the asker's own closed sentence on
    /// the evidence's scene (`Scenes::host_on`) with the question's demands
    /// added, minted as a contraction is (`mint_sentence` at the form's
    /// span). `None` where the question lists nothing or the mind spoke no
    /// closed sentence on that scene.
    pub(crate) fn question_sentence(&self, key: &[u8]) -> Option<(Element, Vec<crate::transducer::family::TrainPair>, Vec<u8>)> {
        use crate::transducer::family::mint_sentence;
        use crate::transducer::sentence::rule_of_form;
        let scene = self.questions.get(key)?.evidence.as_ref()?.scene.clone();
        let (family, _) = self.scenes.evidence(&scene)?;
        let host = self.scenes.host_on(&scene)?;
        let form = with_question(&host, self.listed_question(key))?;
        let span = self.contemplation.span_of(&form)?;
        let sentence = mint_sentence(&rule_of_form(form), &family, &self.language, span.span())?;
        Some((sentence, family, scene))
    }

    /// 78.177 §2 step 2 — THE ASKER LISTS ITS QUESTION: the sentence it is
    /// listed by spoken to the court under the evidence's scene
    /// (`ACT_SPEECH`), where the closer reaches it on the public listing.
    /// Answers whether a sentence was spoken.
    pub(crate) fn list_question(&mut self, key: &[u8]) -> bool {
        let Some((sentence, family, scene)) = self.question_sentence(key) else { return false };
        self.speak_with(std::slice::from_ref(&sentence), &family, &scene, |_, _| {});
        true
    }

    /// THE QUESTION AT `key` BEARS `evidence`: the heard word it was raised
    /// on carried it, and it read true on its scene.
    pub(crate) fn bear_evidence(&mut self, key: &[u8], evidence: Evidence) {
        if let Some(q) = self.questions.get_mut(key) {
            q.evidence = Some(evidence);
        }
    }

    /// THE DEMANDS THE QUESTION AT `key` IS LISTED AS: the bodies of the
    /// heard word its evidence carried, on the scene's family as this mind
    /// holds it (`Scenes::evidence`). Empty where no question stands at
    /// `key`, it bears no evidence, the record is not a word, or the scene
    /// is not held: such a question reaches no court.
    pub(crate) fn listed_question(&self, key: &[u8]) -> Vec<Listed> {
        let Some(ev) = self.questions.get(key).and_then(|q| q.evidence.as_ref()) else { return Vec::new() };
        let mut p = 0usize;
        let Some(word) = Element::from_bytes(&ev.word, &mut p).filter(|_| p == ev.word.len()) else { return Vec::new() };
        let Some((family, _)) = self.scenes.evidence(&ev.scene) else { return Vec::new() };
        heard_bodies(&word, &family)
    }
}
