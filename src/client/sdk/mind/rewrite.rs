// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-CLIENT §5.4, §1a step 8 (S3) — THE MIND PROPOSES A REWRITE.
//
//! A word the mind holds that the rest of the language composes without it
//! is reached by two derivations: its own, and the composition of what
//! cites it. The face between the two is a `Retire` rewrite (MU-ANASTOME
//! §4.1; 3.8, 27), and the mind asserts it to its court. It is a proposal:
//! the court offers it (`offer_rewrite`), and only on a second organ retires
//! the inputs that reduce to `Decomposed` over the rest of ITS language
//! (`reduce_excluding`). The mind adjoins nothing and retires nothing of
//! its own on the proposal. Nothing compares the two derivations by length
//! or count; the yield is the modes the retired word carried (78.162).

use crate::client::sdk::*;
use crate::transducer::address::Address;
use crate::transducer::alphabet::{Rotor, Word};
use crate::transducer::element::Versor;
use crate::transducer::rewrite::{mint, RewriteKind, RewritePayload};
use crate::transducer::stalk::{Invariant, Stalks};

impl Mind {
    /// THE RETIREMENT A RULED WORD ADMITS: where the words the courts ruled
    /// compose `word` without it (`Presentation::derived_without`, exact),
    /// the `Retire` rewrite naming it, citing the forms that compose it and
    /// freeing the lanes it carried. `None` where `word` is not ruled here,
    /// where the rest does not compose it, or where it carries nothing (a
    /// retirement of nothing is no work, and `mint` refuses it).
    pub fn retirement_of(&self, word: &Address) -> Option<Element> {
        let cites = self.language.vocabulary.read().derived_without(word, true)?;
        let freed = self.language.working_word(word.as_bytes())?.element().magnitude();
        let payload = RewritePayload::Retire { cites, freed };
        mint(RewriteKind::Retire, std::slice::from_ref(word), word.clone(), payload, freed)
    }

    /// A REFINE REWRITE PROPOSED FROM AN OPENED AXIS (Ruling 5, MU-CLIENT §5.4):
    /// carries the mind's `PivotBy::OpenAxis { face, seed }`. It refines a chart
    /// within a dimension that already stands, checked by peel.
    pub fn refine_of(
        &self,
        chart: &Invariant,
        axis: u64,
        before: &Address,
        after: &Address,
    ) -> Option<Element> {
        let refined = Stalks::refine_axis(chart, axis);
        if Stalks::peel(&refined) != *chart {
            return None;
        }
        let payload = RewritePayload::Refine { chart: chart.clone(), axis };
        mint(RewriteKind::Refine, std::slice::from_ref(before), after.clone(), payload, [0, 0, 0, 1])
    }

    /// AN AXIS-OPEN REWRITE PROPOSED FROM RESIDUE (Ruling 5, MU-CLIENT §5.4):
    /// germination's Extend, proposing a new dimension of perception where a
    /// persistent residue leaves an unabsorbed class. `class` is the orbit's
    /// holonomy rotor.
    pub fn axis_open_of(
        &self,
        class: &Rotor,
        degree: u8,
        before: &Address,
        after: &Address,
    ) -> Option<Element> {
        if class.is_identity() {
            return None;
        }
        let payload = RewritePayload::AxisOpen { class: Box::new(class.clone()), degree };
        mint(RewriteKind::AxisOpen, std::slice::from_ref(before), after.clone(), payload, [0, 0, 0, 1])
    }

    /// A TRANSLATION REWRITE PROPOSED FROM PLACEMENT (Ruling 5, MU-CLIENT §5.4):
    /// carries the mind's `PivotBy::Placement { face, translator }`. `a` and `b`
    /// are the two charts of the chord's overlap.
    pub fn translation_of(
        &self,
        a: Address,
        b: Address,
        gauge: Versor,
        before: &Address,
        after: &Address,
    ) -> Option<Element> {
        if a == b || !crate::transducer::cga::is_versor(&gauge) {
            return None;
        }
        let payload = RewritePayload::Translation { a, b, gauge };
        mint(RewriteKind::Translation, std::slice::from_ref(before), after.clone(), payload, [0, 2, 0, 0])
    }

    /// A REANCHOR REWRITE PROPOSED FROM RE-GAUGING (Ruling 5, MU-CLIENT §5.4):
    /// proposed only where the re-gauge moves an anchor coordinate on S while
    /// leaving the class invariant. `gamma` holds one word per vertex.
    pub fn reanchor_of(
        &self,
        gamma: Vec<Word>,
        before: &Address,
        after: &Address,
    ) -> Option<Element> {
        let non_id = gamma.iter().filter(|w| !w.is_identity()).count() as u64;
        if non_id == 0 {
            return None;
        }
        let payload = RewritePayload::Reanchor { gamma };
        mint(RewriteKind::Reanchor, std::slice::from_ref(before), after.clone(), payload, [non_id, 0, 0, 0])
    }

    /// ASSERT A PROPOSED REWRITE TO THE COURT (MU-CLIENT §5.4, #146):
    /// spoken once where neither this mind already spoke it nor the court
    /// already taught it. Returns true if the rewrite was spoken.
    pub fn propose_rewrite(&mut self, rw: Element) -> bool {
        if !rw.is_rewrite() || self.language.dialect.get(rw.address().as_bytes()).is_some() || self.already_taught(&rw) {
            return false;
        }
        self.speak(std::slice::from_ref(&rw), |_, _| {});
        true
    }

    /// 27, S3 — THE REWRITES A NEWLY HELD WORD MAKES: the word itself, and
    /// every ruled word standing on a hole or a hue mode it stands on
    /// (`Presentation::standing_with`, one keyed read each), read for the
    /// retirement each admits. Each found is asserted to the court once, as
    /// the rewrites the pivot mints are; one this mind already said or the
    /// court already taught is not said again. Answers the rewrites asserted.
    pub fn propose_rewrites(&mut self, word: &Element) -> Vec<Element> {
        let at = word.address();
        let mut candidates = vec![at.clone()];
        candidates.extend(self.language.vocabulary.read().standing_with(&at));
        let mut said = Vec::new();
        for c in &candidates {
            let Some(rw) = self.retirement_of(c) else { continue };
            if self.propose_rewrite(rw.clone()) {
                said.push(rw);
            }
        }
        said
    }
}
