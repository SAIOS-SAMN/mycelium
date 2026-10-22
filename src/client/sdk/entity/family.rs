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
    /// Some held-out pair did not close: the rule fits, and is not known —
    /// with the first guard of the undetermined ruling it fails.
    Unknown(Unexempt),
    /// UNDETERMINED BY ITS PERSPECTIVES (the auditor's ruling, 2026-10-22):
    /// a clause rule the act reader stated over a TWO-pair family, proven on
    /// both pairs. Its siblings are one pair each, and the act reader states
    /// no clause from one pair ("a coordinate separating bodies on one pair
    /// separates nothing"), so a sibling's rule cannot be the family's: its
    /// held-out failure is no refutation. Not known, and not refuted — sent,
    /// for the court to judge on its withheld pairs.
    Undetermined,
    /// One pair: no sibling to derive it from.
    OnePerspective,
    /// A sibling extraction's stroke ran out before it ruled: not known and
    /// not refuted.
    Unread,
}

/// WHY A RULE ITS PERSPECTIVES DID NOT KNOW IS NOT UNDETERMINED: the first of
/// the ruling's three guards it fails (the auditor, 2026-10-22), read in the
/// order the ruling is stated from the family outward.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Unexempt {
    /// Guard 3: the family is not two pairs; its siblings could speak.
    Pairs,
    /// Guard 1: a vertex fit — the act reader stated no clause.
    Vertex,
    /// Guard 2: the clause rule does not close both pairs exactly.
    Unproven,
}

/// A FAMILY THOUGHT THROUGH: its rule where the form engine closed one, the
/// frames it was perceived on, and the stroke it was extracted on.
pub(crate) struct Thought {
    pub rule: Option<SentenceRule>,
    pub frames: Frames,
    pub budget: crate::physics::Budget,
}

/// A chain of painted sites, before and after: what a perception's
/// remainder and a test's unreached bodies are held as.
pub(crate) type Chain = Vec<(i64, i64, u8, u8)>;

/// ONE PAIR PERCEIVED AS ITS FORM, off the mind's field (MU-CLIENT §7.1 R1):
/// the form, the frames it was perceived on, the pair, and what it closes
/// and leaves on that pair.
pub(crate) struct PairPercept {
    pub form: crate::transducer::sentence::Form,
    pub frames: Frames,
    pub pair: Vec<crate::transducer::family::TrainPair>,
    pub mag: [u64; 4],
    pub remainders: Vec<Chain>,
}

/// THE FAMILY'S RULE: its frames perceived once (the memo answers a family
/// already perceived at this atlas root), the stroke sized by the family
/// (L8w), the form engine the only extractor — read on the language given.
pub(crate) fn extract_on(memo: &mut crate::transducer::family::ExtractMemo, epi: &Epigenome, id: &[u8], train: &Family) -> Thought {
    let gen = epi.atlas_root();
    let frames = match memo.holds(id, &gen) {
        true => Frames::new(),
        false => crate::transducer::family::perceive_family(train),
    };
    let mut budget = crate::physics::Physics::default().stroke_n(crate::transducer::family::extraction_spend(train, &frames));
    let rule = crate::transducer::family::extract_rule_cached(memo, id, train, gen, &mut budget, epi, Some(&frames));
    Thought { rule, frames, budget }
}

/// ONE PAIR AS ITS SCENE FORM on the language given, on a stroke sized by
/// the pair (L8w), with what it closes and leaves there; `None` where the
/// mind's presentation cannot factor it.
pub(crate) fn pair_percept(epi: &Epigenome, x: &crate::arc::Grid, y: &crate::arc::Grid) -> Option<PairPercept> {
    let pair = [(x.clone(), y.clone())];
    crate::arc::on_ground_of(&pair, || {
        let mut p = scene_percept(epi, x, y)?;
        let (mag, remainders) = crate::transducer::family::form_deflation(&p.form, &p.pair, epi);
        (p.mag, p.remainders) = (mag, remainders);
        Some(p)
    })
}

/// ONE PAIR AS ITS SCENE FORM alone, nothing measured on it: an hour of a
/// family, held with no bundle (78.56 III).
pub(crate) fn scene_percept(epi: &Epigenome, x: &crate::arc::Grid, y: &crate::arc::Grid) -> Option<PairPercept> {
    let pair = vec![(x.clone(), y.clone())];
    crate::arc::on_ground_of(&pair, || {
        let frames = crate::transducer::family::perceive_family(&pair);
        let mut budget = crate::physics::Physics::default().stroke_n(crate::transducer::family::extraction_spend(&pair, &frames));
        let form = crate::transducer::family::scene_form_on(epi, &mut budget, x, y, &frames)?;
        Some(PairPercept { form, frames, pair: pair.clone(), mag: [0; 4], remainders: Vec::new() })
    })
}

/// THE SCENE A CHAIN DESCRIBES, perceived as its form on the language given
/// (50.6 seam 1): its painted sites before against after in one frame.
/// `None` where it describes no scene or the presentation cannot factor it.
pub(crate) fn chain_percept(epi: &Epigenome, chain: &[(i64, i64, u8, u8)]) -> Option<PairPercept> {
    let scene = crate::transducer::family::scene_of_chain(chain)?;
    scene_percept(epi, &scene.0, &scene.1)
}

/// WHAT A RULE'S SPEECH PREPARES, on the language given (78.81): the
/// remainder interpreted in the act that speaks it — an inversion the speech
/// accepted IS the form spoken — with what it closes and leaves on the
/// family.
pub(crate) fn speech_of(epi: &Epigenome, rule: &SentenceRule, train: &Family, budget: &mut crate::physics::Budget) -> (SentenceRule, [u64; 4], Vec<Chain>) {
    let (mag, remainders) = crate::transducer::family::form_deflation(&rule.form, train, epi);
    let (joined, _) = crate::transducer::family::interpret::interpret(&rule.form, train, &remainders, epi, budget);
    match joined {
        Some((form, after)) => {
            let (m, _) = crate::transducer::family::form_deflation(&form, train, epi);
            (SentenceRule { form, ratio: rule.ratio }, m, after)
        }
        None => (rule.clone(), mag, remainders),
    }
}

/// A RULE KNOWN BY ITS PERSPECTIVES, NOT FITTED (MU-CLIENT §4.4), read on
/// the language given: each training pair held out in turn, the rule the
/// OTHER pairs derive (the form engine on them alone, uncached, on its own
/// stroke) spoken on the held-out input and asked to close it — no
/// remainder, its grid the held-out output. Known where every perspective
/// re-derives it. Answers the reading and the pairs a held-out perspective
/// left uncovered — what the siblings' rule said against the truth — which
/// the mind LIFTS into its sensorium where the reading lands, never drops.
pub(crate) fn ascertain_on(epi: &Epigenome, train: &Family, rule: &SentenceRule) -> (Ascertained, Vec<(crate::arc::Grid, crate::arc::Grid)>) {
    use crate::transducer::family::{extract_rule_on, extraction_spend, perceive_family};
    crate::observe::note_mind(|m| m.perspective[0] += 1);
    let mut lifts = Vec::new();
    if train.len() < 2 {
        crate::observe::note_mind(|m| m.perspective[3] += 1);
        return (Ascertained::OnePerspective, lifts);
    }
    let (mut failed, mut unread) = (Vec::new(), false);
    for k in 0..train.len() {
        let siblings: Vec<(crate::arc::Grid, crate::arc::Grid)> = train.iter().enumerate().filter(|(i, _)| *i != k).map(|(_, p)| p.clone()).collect();
        let frames = perceive_family(&siblings);
        let mut budget = crate::physics::Physics::default().stroke_n(extraction_spend(&siblings, &frames));
        let (x, y) = &train[k];
        let Some(rule) = extract_rule_on(&siblings, &frames, epi, &mut budget) else {
            // No rule from the siblings: where the stroke ran out it is
            // unread; otherwise nothing they share reaches this pair.
            match budget.remaining() == 0 {
                true => unread = true,
                false => {
                    failed.push(k);
                    crate::observe::note_mind(|m| m.disagree[0] += 1);
                    lifts.push((x.clone(), y.clone()));
                }
            }
            continue;
        };
        if let Err(Disagreed { kinds, said, demanded }) = disagreement_on(epi, &rule, x, y) {
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
            let from = match said {
                Some(g) => g,
                None => x.clone(),
            };
            lifts.push((from, y.clone()));
        }
    }
    // THE INVERSION: where the siblings missed a pair, that pair's own rule
    // is read back onto every sibling. A rule one perspective derives that
    // re-derives all the others is the family's invariant.
    for &k in &failed {
        let alone = vec![train[k].clone()];
        let frames = perceive_family(&alone);
        let mut budget = crate::physics::Physics::default().stroke_n(extraction_spend(&alone, &frames));
        let Some(rule) = extract_rule_on(&alone, &frames, epi, &mut budget) else { continue };
        let covers = train.iter().enumerate().filter(|(i, _)| *i != k).all(|(_, (x, y))| disagreement_on(epi, &rule, x, y).is_ok());
        if covers {
            crate::observe::note_mind(|m| m.perspective[7] += 1);
            return (Ascertained::KnownByInversion(Box::new(rule)), lifts);
        }
    }
    // THE TWO-PAIR CLAUSE RULE, UNDETERMINED (the auditor's ruling,
    // 2026-10-22, its three guards): exactly two pairs, so each sibling set
    // is one pair (guard 3); the family's rule is the act reader's clauses
    // (guard 1 — a vertex fit stays unknown); and it closes both pairs
    // exactly, no remainder, the grid each output (guard 2).
    let unexempt = match (train.len() == 2, rule.form.clauses.is_empty()) {
        _ if failed.is_empty() => None,
        (false, _) => Some(Unexempt::Pairs),
        (true, true) => Some(Unexempt::Vertex),
        (true, false) => (!train.iter().all(|(x, y)| disagreement_on(epi, rule, x, y).is_ok())).then_some(Unexempt::Unproven),
    };
    let (slot, outcome) = match (failed.is_empty(), unread) {
        (false, _) => match unexempt {
            None => (8, Ascertained::Undetermined),
            Some(why) => (2, Ascertained::Unknown(why)),
        },
        (true, true) => (4, Ascertained::Unread),
        (true, false) => (1, Ascertained::Known),
    };
    crate::observe::note_mind(|m| m.perspective[slot] += 1);
    (outcome, lifts)
}

/// WHAT ONE PERSPECTIVE DISAGREED ABOUT, on the language given: the rule
/// spoken on its input against its output. `Ok` where it closes; otherwise
/// the kinds, in [`crate::observe::DISAGREE`]'s order — it spoke nothing,
/// spoke with a remainder, laid down another frame, put cells in the wrong
/// places (by cause, 78.167 A1), or coloured the right cells otherwise — with
/// the grid it said, where it said one, and the demands it spoke nothing for.
pub(crate) fn disagreement_on(epi: &Epigenome, rule: &SentenceRule, x: &crate::arc::Grid, y: &crate::arc::Grid) -> Result<(), Disagreed> {
    let refused = |kind: usize, said: Option<crate::arc::Grid>, demanded: [u64; 6]| Disagreed { kinds: vec![kind], said, demanded };
    let Some(speech) = crate::transducer::family::speak::speak_rule(rule, x, epi) else { return Err(refused(1, None, [0; 6])) };
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

/// THE TESTS SPOKEN THROUGH THE RULE, on the language given: every test's
/// output where the rule speaks it whole, and the chains a test's speech
/// left — its remainder, and (78.167 B) its unreached bodies — for the mind
/// to hold as thought. WHOLE MEANS EVERY BODY SPOKEN FOR: a speech that left
/// a body unreached, or a creation, a merge or a reshape unspoken (its
/// form's demands of those kinds), is no answer, and the solve waits for a
/// rule that speaks for them (a clause statement, 78.167 C, says what
/// happens to every body).
pub(crate) fn tests_spoken(epi: &Epigenome, rule: &SentenceRule, tests: &[crate::arc::Grid]) -> (Option<Vec<crate::arc::Grid>>, Vec<Chain>) {
    let mut chains = Vec::new();
    let outputs = tests
        .iter()
        .map(|x| {
            let spoken = crate::transducer::family::speak_rule(rule, x, epi);
            if let Some(s) = &spoken {
                if !s.remainder.is_empty() {
                    chains.push(s.remainder.iter().map(|&(r, c, h)| (r, c, h, h)).collect());
                }
                if !s.unreached.is_empty() {
                    let bodies = crate::transducer::family::perceive(x);
                    let chain: Chain = s
                        .unreached
                        .iter()
                        .filter_map(|&i| bodies.get(i))
                        .flat_map(|b| b.compartments.iter().filter_map(|l| crate::transducer::family::on_grid(&l.site).map(|(r, c)| (r, c, l.hue, l.hue))))
                        .collect();
                    crate::observe::note_mind(|m| m.withheld_tests[0] += 1);
                    chains.push(chain);
                }
            }
            // Creation (3), merge (2) and reshape (4) say what happens to
            // occupancy, and a demand of theirs spoke nothing.
            let occupancy_unspoken = spoken.as_ref().is_some_and(|s| s.demanded[2] + s.demanded[3] + s.demanded[4] > 0);
            if occupancy_unspoken {
                crate::observe::note_mind(|m| m.withheld_tests[1] += 1);
            }
            spoken.filter(|s| s.remainder.is_empty() && s.unreached.is_empty() && !occupancy_unspoken).and_then(|s| s.grid())
        })
        .collect();
    (outputs, chains)
}

impl Entity {
    /// SPEAK A RULE AS ITS CRYSTALLIZED FORM (chapter 30 step 4). The
    /// remainder is interpreted in the act that speaks it (78.81): an
    /// inversion the speech accepted IS the form spoken. A form carrying
    /// demands is uttered whole with them (50.5); a form with a remainder, an
    /// unmeasurable span, or no mint is held as thought; the wire never sees
    /// a nerve. Answers the magnitude uttered.
    pub(crate) fn speak_form(&mut self, rule: &SentenceRule, train: &Family, frames: &Frames, family: &[u8], budget: &mut crate::physics::Budget) -> Option<[u64; 4]> {
        let prepared = speech_of(&self.epigenome, rule, train, budget);
        self.speak_prepared(prepared, train, frames, family, budget)
    }

    /// THE PREPARED SPEECH SPOKEN (`speech_of` read it, here or on the
    /// perception carrier): an unmeasurable span withholds; a form carrying
    /// demands is uttered whole with them (50.5); a form with a remainder, or
    /// no mint, is held as thought; otherwise its word is spoken under its
    /// scene. Answers the magnitude uttered.
    pub(crate) fn speak_prepared(
        &mut self,
        (rule, mag, remainders): (SentenceRule, [u64; 4], Vec<Chain>),
        train: &Family,
        frames: &Frames,
        family: &[u8],
        budget: &mut crate::physics::Budget,
    ) -> Option<[u64; 4]> {
        use crate::transducer::family::trace;
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
        let deliberation = self.span_of(form)?;
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

    /// ONE PAIR PERCEIVED, WITHHELD under its scene: held with the frames it
    /// was perceived from, what it leaves its demand (`withhold_perceived`).
    pub(crate) fn withhold_pair(&mut self, p: PairPercept, family: &[u8]) {
        let mut budget = self.stroke_n(crate::transducer::family::extraction_spend(&p.pair, &p.frames));
        self.withhold_perceived(p.form, &p.frames, &p.pair, &p.remainders, p.mag, family, &mut budget);
    }

    /// A PERSPECTIVE THE SIBLINGS DID NOT COVER, LIFTED INTO THE SENSORIUM:
    /// what their rule said against what was true, perceived as one pair and
    /// withheld under the task exactly as a pair no rule closed is. Its bodies
    /// enter perception's cells; what escapes there is demanded and
    /// constructed. The 2-D gap becomes the mind's own structure.
    pub(crate) fn lift_pair(&mut self, p: PairPercept, id: &[u8]) {
        self.withhold_pair(p, id);
        crate::observe::note_mind(|m| m.perspective[5] += 1);
    }

    /// THE SENTENCE A SOLVE CARRIES: the rule minted at its span, or `None`
    /// where the span is unmeasurable or the mint refuses.
    pub(crate) fn mint_solve(&mut self, rule: &SentenceRule, train: &Family) -> Option<Element> {
        let span = self.span_of(&rule.form)?.span();
        crate::transducer::family::mint_sentence_why(rule, train, &self.epigenome, span).ok()
    }
}
