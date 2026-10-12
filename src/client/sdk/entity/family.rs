// 78.164 — THE FAMILY LAW: a family that entered the field (from the feed,
// the court's registry, or a task set before a seat) is thought through
// whole — its frames perceived, its rule extracted by the form engine, the
// form spoken to the court or held as the open work it is.
//
//! 21, 26 — a family the form engine does not close is a WITHHELD form, held
//! in the contemplation rather than dropped; nothing residual reaches the
//! wire. A sentence with demand vertices is uttered whole (50.5).

use crate::client::sdk::*;
use crate::transducer::family::{Family, Frames};
use crate::transducer::sentence::SentenceRule;

/// The magnitude of a family, per lane: the sum over its pairs of the
/// coboundary's lane magnitudes — what thinking it through draws.
pub(crate) fn family_magnitude(train: &Family) -> [u64; 4] {
    let mut mag = [0u64; 4];
    for (x, y) in train {
        if let Some(chain) = crate::transducer::family::refine::refine_delta(x, y) {
            let m = crate::transducer::element::measure_lanes(&chain);
            for l in 0..4 {
                #[expect(clippy::disallowed_methods, reason = "rule 11 — MEASURE_BOUND (element::carrier): a sum of chain measures over resident chains cannot pass u64")]
                let next = mag[l].saturating_add(m[l]);
                mag[l] = next;
            }
        }
    }
    mag
}

/// A FAMILY THOUGHT THROUGH: its rule where the form engine closed one, the
/// frames it was perceived on, and the stroke it was extracted on.
pub(crate) struct Thought {
    pub rule: Option<SentenceRule>,
    pub frames: Frames,
    pub budget: crate::physics::Budget,
}

impl Entity {
    /// THE FAMILY'S RULE: its frames perceived once (the memo answers a
    /// family already perceived at this atlas root), the stroke sized by the
    /// family (L8w), the form engine the only extractor.
    pub(crate) fn extract_family(&mut self, id: &[u8], train: &Family) -> Thought {
        let gen = self.epigenome.atlas_root();
        let frames = match self.board.memo.holds(id, &gen) {
            true => Frames::new(),
            false => crate::transducer::family::perceive_family(train),
        };
        let mut budget = self.stroke_n(crate::transducer::family::extraction_spend(train, &frames));
        let rule = crate::transducer::family::extract_rule_cached(&mut self.board.memo, id, train, gen, &mut budget, &self.epigenome, Some(&frames));
        Thought { rule, frames, budget }
    }

    /// A FAMILY THOUGHT THROUGH AND SPOKEN: the rule's form spoken to the
    /// court (`speak_form`), or the family withheld where no rule closed.
    /// Answers the thought and what the speech uttered.
    pub(crate) fn think_family(&mut self, id: &[u8], train: &Family) -> (Thought, Option<[u64; 4]>) {
        let mut t = self.extract_family(id, train);
        let uttered = match t.rule.as_ref() {
            Some(rule) => self.speak_form(rule, train, &t.frames, id, &mut t.budget),
            None => {
                self.withhold_unextracted(train, id);
                None
            }
        };
        (t, uttered)
    }

    /// SPEAK A RULE AS ITS CRYSTALLIZED FORM (chapter 30 step 4). The
    /// remainder is interpreted in the act that speaks it (78.81): an
    /// inversion the speech accepted IS the form spoken. A form carrying
    /// demands is uttered whole with them (50.5); a form with a remainder, an
    /// unmeasurable span, or no mint is held as thought; the wire never sees
    /// a nerve. Answers the magnitude uttered.
    pub(crate) fn speak_form(&mut self, rule: &SentenceRule, train: &Family, frames: &Frames, family: &[u8], budget: &mut crate::physics::Budget) -> Option<[u64; 4]> {
        use crate::transducer::family::trace;
        let (mag, remainders) = crate::transducer::family::form_deflation(&rule.form, train, &self.epigenome);
        let (joined, _) = crate::transducer::family::interpret::interpret(&rule.form, train, &remainders, &self.epigenome, budget);
        let (rule, mag, remainders) = match joined {
            Some((form, after)) => {
                let (m, _) = crate::transducer::family::form_deflation(&form, train, &self.epigenome);
                (std::borrow::Cow::Owned(SentenceRule { form, ratio: rule.ratio }), m, after)
            }
            None => (std::borrow::Cow::Borrowed(rule), mag, remainders),
        };
        let form = &rule.form;
        // 78.149 (a): the span is not μ; an unmeasurable one withholds.
        if self.span_of(form).is_none() {
            trace::trace("speak/span", trace::Wrote::Nothing, None);
            return None;
        }
        if form.demand_vertices().next().is_some() {
            trace::trace("speak/form", trace::Wrote::Demand, None);
            return self.speak_with_demands(&rule, (train, frames), family, &remainders, budget);
        }
        if remainders.iter().any(|r| !r.is_empty()) {
            trace::trace("speak/remainder", trace::Wrote::Nothing, None);
            self.withhold_perceived(form.clone(), frames, train, &remainders, mag, family, budget);
            return None;
        }
        let Some(word) = crate::transducer::sentence::as_element_form(form, mag) else {
            trace::trace("speak/mint", trace::Wrote::Nothing, None);
            self.hold_perceived(form.clone(), frames, train, &remainders, family, budget);
            return None;
        };
        if self.already_taught(&word) {
            trace::trace("speak/taught", trace::Wrote::Key(word.address().as_bytes()), None);
            self.remember_spoken(&word);
            return Some(word.magnitude());
        }
        trace::trace_bytes("speak/form", trace::Wrote::Key(word.address().as_bytes()), None, word.description().len() as i128);
        self.speak_with(std::slice::from_ref(&word), train, family, |_, _| {});
        Some(word.magnitude())
    }

    /// 50.5 — THE SENTENCE WITH ITS DEMANDS, uttered whole: minted at its
    /// span, spoken, and — refused by the mint, by the court or by the wire —
    /// held as the open work it is, never let go. A sentence the court took
    /// is order built, booked by its own magnitude (78.69 fix 3).
    fn speak_with_demands(
        &mut self,
        rule: &SentenceRule,
        (train, frames): (&Family, &Frames),
        family: &[u8],
        remainders: &[Vec<(i64, i64, u8, u8)>],
        budget: &mut crate::physics::Budget,
    ) -> Option<[u64; 4]> {
        let form = &rule.form;
        let Some(deliberation) = self.span_of(form) else { return None };
        let Ok(word) = crate::transducer::family::mint_sentence_why(rule, train, &self.epigenome, deliberation.span()) else {
            self.hold_perceived(form.clone(), frames, train, remainders, family, budget);
            return None;
        };
        if self.already_taught(&word) {
            self.remember_spoken(&word);
            return Some(word.magnitude());
        }
        // Where the court's answer lands: a sentence it took is order built,
        // booked by its own magnitude (78.69 fix 3); one refused, or the court
        // unreached, is held as the open work it is.
        let mag = word.magnitude();
        let held = (form.clone(), frames.clone(), train.to_vec(), remainders.to_vec(), family.to_vec());
        self.speak_with(std::slice::from_ref(&word), train, family, move |me, heard| match heard {
            Ok(codes) if codes.as_slice() != [crate::anastome::dialogue::SPEECH_REFUSED] => me.physics_mut().syntropy(&mag),
            _ => {
                let (form, frames, train, remainders, family) = held;
                let mut budget = me.physics.stroke();
                me.hold_perceived(form, &frames, &train, &remainders, &family, &mut budget);
            }
        });
        Some(mag)
    }

    /// A family the form engine does not extract is withheld: each pair its
    /// own form, perceived on a stroke sized by the pair (L8w); a pair this
    /// mind's presentation cannot factor is left for a later tick.
    pub(crate) fn withhold_unextracted(&mut self, train: &Family, family: &[u8]) {
        for (a, b) in train {
            let pair = vec![(a.clone(), b.clone())];
            let frames = crate::transducer::family::perceive_family(&pair);
            let mut budget = self.stroke_n(crate::transducer::family::extraction_spend(&pair, &frames));
            let Some(f) = crate::transducer::family::scene_form_on(&self.epigenome, &mut budget, a, b, &frames) else { continue };
            let (mag, rem) = crate::transducer::family::form_deflation(&f, &pair, &self.epigenome);
            self.withhold_perceived(f, &frames, &pair, &rem, mag, family, &mut budget);
        }
    }

    /// THE TESTS SPOKEN THROUGH THE RULE: every test's output where the rule
    /// speaks it whole; a remainder a test leaves is held as thought.
    pub(crate) fn speak_tests(&mut self, rule: &SentenceRule, tests: &[crate::arc::Grid]) -> Option<Vec<crate::arc::Grid>> {
        tests
            .iter()
            .map(|x| {
                let spoken = crate::transducer::family::speak_rule(rule, x, &self.epigenome);
                if let Some(s) = &spoken {
                    if !s.remainder.is_empty() {
                        self.hold_chain(s.remainder.iter().map(|&(r, c, h)| (r, c, h, h)).collect());
                    }
                }
                spoken.filter(|s| s.remainder.is_empty()).and_then(|s| s.grid())
            })
            .collect()
    }

    /// THE SENTENCE A SOLVE CARRIES: the rule minted at its span, or `None`
    /// where the span is unmeasurable or the mint refuses.
    pub(crate) fn mint_solve(&mut self, rule: &SentenceRule, train: &Family) -> Option<Element> {
        let span = self.span_of(&rule.form)?.span();
        crate::transducer::family::mint_sentence_why(rule, train, &self.epigenome, span).ok()
    }
}
