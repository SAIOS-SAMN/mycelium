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
use crate::transducer::rewrite::{mint, RewriteKind, RewritePayload};

impl Mind {
    /// THE RETIREMENT A RULED WORD ADMITS: where the words the courts ruled
    /// compose `word` without it (`Presentation::derived_without`, exact),
    /// the `Retire` rewrite naming it, citing the forms that compose it and
    /// freeing the lanes it carried. `None` where `word` is not ruled here,
    /// where the rest does not compose it, or where it carries nothing (a
    /// retirement of nothing is no work, and `mint` refuses it).
    pub(crate) fn retirement_of(&self, word: &Address) -> Option<Element> {
        let cites = self.language.vocabulary.read().derived_without(word, true)?;
        let freed = self.language.working_word(word.as_bytes())?.element().magnitude();
        let payload = RewritePayload::Retire { cites, freed };
        mint(RewriteKind::Retire, std::slice::from_ref(word), word.clone(), payload, freed)
    }

    /// 27, S3 — THE REWRITES A NEWLY HELD WORD MAKES: the word itself, and
    /// every ruled word standing on a hole or a hue mode it stands on
    /// (`Presentation::standing_with`, one keyed read each), read for the
    /// retirement each admits. Each found is asserted to the court once, as
    /// the rewrites the pivot mints are; one this mind already said or the
    /// court already taught is not said again. Answers the rewrites asserted.
    pub(crate) fn propose_rewrites(&mut self, word: &Element) -> Vec<Element> {
        let at = word.address();
        let mut candidates = vec![at.clone()];
        candidates.extend(self.language.vocabulary.read().standing_with(&at));
        let mut said = Vec::new();
        for c in &candidates {
            let Some(rw) = self.retirement_of(c) else { continue };
            if self.language.dialect.get(rw.address().as_bytes()).is_some() || self.already_taught(&rw) {
                continue;
            }
            self.speak(std::slice::from_ref(&rw), |_, _| {});
            said.push(rw);
        }
        said
    }
}
