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

/// WHAT ONE PERSPECTIVE DISAGREED ABOUT, read whole: every kind present (in
/// [`crate::observe::DISAGREE`]'s order), the grid the rule said, and the
/// demands its form spoke nothing for, by kind.
pub(crate) struct Disagreed {
    pub(crate) kinds: Vec<usize>,
    pub(crate) said: Option<crate::arc::Grid>,
    pub(crate) demanded: [u64; 6],
}

/// 78.167 A1 — WHY A SPEECH PUT CELLS IN THE WRONG PLACES, cell by cell,
/// each cause present once, in [`crate::observe::DISAGREE`]'s slots: a cell
/// the truth holds and the speech left ground is ERASED (4) where an input
/// body no vertex reached stood there, DISPLACED (6) where the input held a
/// body the speech did reach, UNCREATED (5) where the input was ground; a
/// cell the speech painted and the truth holds as ground is UNCLEARED (7).
/// Empty where every cell agrees on occupancy (the hues alone differ).
pub(crate) fn occupancy_causes(said: &crate::arc::Grid, x: &crate::arc::Grid, y: &crate::arc::Grid, unreached: &[usize]) -> Vec<usize> {
    let bodies = crate::transducer::family::perceive(x);
    let mut erased: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    for &i in unreached {
        let Some(b) = bodies.get(i) else { continue };
        for l in &b.compartments {
            if let Some((r, c)) = crate::transducer::family::on_grid(&l.site) {
                erased.insert(crate::transducer::family::site_key(r, c), ());
            }
        }
    }
    let mut kinds = [false; 4];
    for (r, (srow, trow)) in said.iter().zip(y).enumerate() {
        for (c, (s, t)) in srow.iter().zip(trow).enumerate() {
            let (Ok(ri), Ok(ci)) = (i64::try_from(r), i64::try_from(c)) else { continue };
            match (*s == 0, *t == 0) {
                (true, false) => {
                    let input = x.get(r).and_then(|row| row.get(c)).is_some_and(|&h| h != 0);
                    let at = match (erased.contains(&crate::transducer::family::site_key(ri, ci)), input) {
                        (true, _) => 0,
                        (false, true) => 2,
                        (false, false) => 1,
                    };
                    kinds[at] = true;
                }
                (false, true) => kinds[3] = true,
                _ => {}
            }
        }
    }
    [4, 5, 6, 7].into_iter().zip(kinds).filter(|(_, present)| *present).map(|(k, _)| k).collect()
}

/// WHAT A RULE'S PERSPECTIVES SAID OF IT (MU-CLIENT §4.4): each training pair
/// a perspective, the rule its siblings derive asked to close it — and, where
/// one does not, the INVERSION: the rule that pair alone derives asked to
/// close every sibling.
#[derive(Clone, Debug)]
pub(crate) enum Ascertained {
    /// Every held-out pair closed under the rule its siblings derived: the
    /// rule is an invariant of the family, known.
    Known,
    /// The siblings under-determined the rule, and one pair alone carries
    /// it: the rule that pair derives re-derives every other pair. Known by
    /// inversion — this rule, not the fit, is the family's invariant.
    KnownByInversion(Box<SentenceRule>),
    /// Some held-out pair did not close: the rule fits, and is not known.
    Unknown,
    /// One pair: no sibling to derive it from.
    OnePerspective,
    /// A sibling extraction's stroke ran out before it ruled: not known and
    /// not refuted.
    Unread,
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

    /// A RULE KNOWN BY ITS PERSPECTIVES, NOT FITTED (MU-CLIENT §4.4): each
    /// training pair is held out in turn, as the court holds its pairs out;
    /// the rule the OTHER pairs derive (the form engine on them alone,
    /// uncached, on its own stroke) is spoken on the held-out input and
    /// asked to close it — no remainder, and its grid the held-out output.
    /// Known where every perspective re-derives it: the family's invariant,
    /// not a fit to all of its pairs at once. A held-out pair that does not
    /// close is LIFTED (`lift_perspective`), never dropped.
    pub(crate) fn ascertain(&mut self, id: &[u8], train: &Family) -> Ascertained {
        use crate::transducer::family::{extract_rule_on, extraction_spend, perceive_family};
        crate::observe::note_mind(|m| m.perspective[0] += 1);
        if train.len() < 2 {
            crate::observe::note_mind(|m| m.perspective[3] += 1);
            return Ascertained::OnePerspective;
        }
        let (mut failed, mut unread) = (Vec::new(), false);
        for k in 0..train.len() {
            let siblings: Vec<(crate::arc::Grid, crate::arc::Grid)> = train.iter().enumerate().filter(|(i, _)| *i != k).map(|(_, p)| p.clone()).collect();
            let frames = perceive_family(&siblings);
            let mut budget = self.stroke_n(extraction_spend(&siblings, &frames));
            let (x, y) = &train[k];
            let Some(rule) = extract_rule_on(&siblings, &frames, &self.epigenome, &mut budget) else {
                // No rule from the siblings: where the stroke ran out it is
                // unread; otherwise nothing they share reaches this pair.
                match budget.remaining() == 0 {
                    true => unread = true,
                    false => {
                        failed.push(k);
                        crate::observe::note_mind(|m| m.disagree[0] += 1);
                        self.lift_perspective(x, y, id);
                    }
                }
                continue;
            };
            if let Err(Disagreed { kinds, said, demanded }) = self.disagreement(&rule, x, y) {
                failed.push(k);
                crate::observe::note_mind(|m| {
                    for kind in &kinds {
                        if let Some(n) = m.disagree.get_mut(*kind) {
                            *n += 1;
                        }
                    }
                    for (n, d) in m.demanded_kinds.iter_mut().zip(demanded) {
                        *n += d;
                    }
                });
                let from = match &said {
                    Some(g) => g,
                    None => x,
                };
                self.lift_perspective(from, y, id);
            }
        }
        // THE INVERSION: where the siblings missed a pair, that pair's own
        // rule is read back onto every sibling. A rule one perspective
        // derives that re-derives all the others is the family's invariant.
        for &k in &failed {
            let alone = vec![train[k].clone()];
            let frames = perceive_family(&alone);
            let mut budget = self.stroke_n(extraction_spend(&alone, &frames));
            let Some(rule) = extract_rule_on(&alone, &frames, &self.epigenome, &mut budget) else { continue };
            let covers = train.iter().enumerate().filter(|(i, _)| *i != k).all(|(_, (x, y))| self.disagreement(&rule, x, y).is_ok());
            if covers {
                crate::observe::note_mind(|m| m.perspective[7] += 1);
                return Ascertained::KnownByInversion(Box::new(rule));
            }
        }
        let (slot, outcome) = match (failed.is_empty(), unread) {
            (false, _) => (2, Ascertained::Unknown),
            (true, true) => (4, Ascertained::Unread),
            (true, false) => (1, Ascertained::Known),
        };
        crate::observe::note_mind(|m| m.perspective[slot] += 1);
        outcome
    }

    /// WHAT ONE PERSPECTIVE DISAGREED ABOUT: the rule spoken on its input
    /// against its output. `Ok` where it closes; otherwise the kinds, in
    /// [`crate::observe::DISAGREE`]'s order — it spoke nothing, spoke with a
    /// remainder, laid down another frame, put cells in the wrong places (by
    /// cause, 78.167 A1), or coloured the right cells otherwise — with the
    /// grid it said, where it said one, and the demands it spoke nothing for.
    fn disagreement(&self, rule: &SentenceRule, x: &crate::arc::Grid, y: &crate::arc::Grid) -> Result<(), Disagreed> {
        let refused = |kind: usize, said: Option<crate::arc::Grid>, demanded: [u64; 6]| Disagreed { kinds: vec![kind], said, demanded };
        let Some(speech) = crate::transducer::family::speak::speak_rule(rule, x, &self.epigenome) else { return Err(refused(1, None, [0; 6])) };
        let said = speech.grid();
        if !speech.remainder.is_empty() {
            return Err(refused(2, said, speech.demanded));
        }
        let Some(g) = said else { return Err(refused(1, None, speech.demanded)) };
        if crate::transducer::family::speak::residual_of(Some(&g), y).is_empty() {
            return Ok(());
        }
        if g.len() != y.len() || g.iter().zip(y).any(|(a, b)| a.len() != b.len()) {
            return Err(refused(3, Some(g), speech.demanded));
        }
        let kinds = occupancy_causes(&g, x, y, &speech.unreached);
        let kinds = match kinds.is_empty() {
            true => vec![8],
            false => kinds,
        };
        Err(Disagreed { kinds, said: Some(g), demanded: speech.demanded })
    }

    /// A PERSPECTIVE THE SIBLINGS DID NOT COVER, LIFTED INTO THE SENSORIUM:
    /// what their rule said against what was true is perceived as one pair —
    /// its frames, its scene form, its deflation — and held under the task,
    /// exactly as a pair no rule closed is (`withhold_unextracted`). Its
    /// bodies enter perception's cells; what escapes there is demanded and
    /// constructed. The 2-D gap becomes the mind's own structure.
    fn lift_perspective(&mut self, said: &crate::arc::Grid, truth: &crate::arc::Grid, id: &[u8]) {
        let pair = vec![(said.clone(), truth.clone())];
        let frames = crate::transducer::family::perceive_family(&pair);
        let mut budget = self.stroke_n(crate::transducer::family::extraction_spend(&pair, &frames));
        let Some(f) = crate::transducer::family::scene_form_on(&self.epigenome, &mut budget, said, truth, &frames) else { return };
        let (mag, rem) = crate::transducer::family::form_deflation(&f, &pair, &self.epigenome);
        self.withhold_perceived(f, &frames, &pair, &rem, mag, id, &mut budget);
        crate::observe::note_mind(|m| m.perspective[5] += 1);
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
