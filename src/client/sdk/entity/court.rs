// 78.164 — THE COURT, A BOUNDARY: what flows into the field from the hypha
// (the open book's listings and their demands, the registry's tasks, the
// language it teaches) and what flows out to it (sentences, probes, solves,
// colour words), paced by the surface the court itself returns.
//
//! The pace is the court's: a quote is asked when the language moved, the
//! book rose, or a lane's dwell elapsed — a receipt count, never a timer —
//! and the T lane neither holds the mind back nor releases it (C4). An
//! occluded wire asks nothing; the answer to impedance is patience. Every
//! exchange rides the hypha carrier (MU-CLIENT §5.1): the field asks and
//! goes on, and takes the answer at its next admission.

use crate::client::sdk::*;
use crate::physics::{Physics, StrokeOutcome};
use crate::transducer::element::measure_lanes;

/// THE COURT'S PACE, carried from tick to tick: the language and the book
/// height the court was last asked at, the surface it returned and the dwell
/// left on each lane, and where the sync last stood; and on the carrier
/// (MU-CLIENT §5.1), the asks in flight — one quote, one task pull, one sync,
/// one syllabus, a solve per task — and what landed for the tick to think:
/// the quote, the task, and what the probes the quote's demands raised
/// closed and pressed.
#[derive(Default)]
pub(crate) struct CourtPace {
    last_lang: Option<(usize, usize, usize)>,
    last_height: u64,
    last_quoted_height: u64,
    last_pressure: [u64; 4],
    pub(crate) dwell_remaining: [u64; 4],
    last_synced_lang: Option<(usize, usize, usize)>,
    last_synced_height: u64,
    pub(crate) quoting: bool,
    pub(crate) quoted: Option<YieldQuote>,
    pub(crate) pulling: bool,
    pub(crate) task: Option<crate::anastome::dialogue::TaskEnvelope>,
    pub(crate) syncing: bool,
    pub(crate) schooling: bool,
    pub(crate) solving: crate::network::nerve::Keyed<()>,
    probed: StrokeOutcome,
}

impl CourtPace {
    /// A pressed lane whose dwell has run out, the T lane excluded (C4).
    fn dwell_elapsed(&self) -> bool {
        [0usize, 1, 3].iter().any(|&l| self.last_pressure[l] > 0 && self.dwell_remaining[l] == 0)
    }

    /// The pressure the court last returned, the dwell's measure.
    pub(crate) fn pressure(&self) -> [u64; 4] {
        self.last_pressure
    }
}

impl Entity {
    /// THE COURT'S WORD FLOWS IN FIRST: every answer the carrier landed is
    /// taken, in the order it was asked, and what the probes it answered
    /// closed and pressed is this tick's.
    pub(crate) fn court_inflow(&mut self, outcome: &mut StrokeOutcome) {
        self.take_answers();
        let probed = std::mem::take(&mut self.board.court.probed);
        StrokeOutcome::add(&mut outcome.closed, &probed.closed);
    }

    /// 50.9, 21 step 3 — WHAT THE MIND OWES THE COURT flows out: the
    /// sentences its refusals contracted, and the forms a court waits on.
    pub(crate) fn court_owed(&mut self) {
        self.speak_contracted();
        self.speak_owed();
    }

    /// THE FEED'S NEXT FAMILY ENTERS, paced as the quote is, where the
    /// reserve can fund thinking it through: thought through whole and its
    /// form spoken. One family a tick.
    pub(crate) fn feed_inflow(&mut self, outcome: &mut StrokeOutcome) {
        let pace = &self.board.court;
        if pace.last_pressure != [0; 4] && !pace.dwell_elapsed() {
            return;
        }
        let Some(feed) = self.board.feed.as_mut() else { return };
        let Some((path, task)) = feed.peek() else { return };
        let mag = super::family::family_magnitude(&task.train);
        if !crate::economy::lanes_short(&self.physics().reserve_shortfall(Physics::lanes_of(&mag))).is_empty() {
            return;
        }
        if let Some(feed) = self.board.feed.as_mut() {
            feed.entered(&path);
        }
        self.physics_mut().bump(&mag);
        StrokeOutcome::add(&mut outcome.fed, &mag);
        // A task whose members have no A5 form has no identity to think by.
        let Some(id) = crate::registry::task_id(&task) else { return };
        if let (_, Some(uttered)) = self.think_family(&id, &task.train) {
            StrokeOutcome::add(&mut outcome.uttered, &uttered);
        }
    }

    /// THE OPEN BOOK FLOWS IN: a quote that landed is read — each listing's
    /// demands are pressure at their charts, and a demand a held closed form
    /// reproduces is answered: the probe flows out, and where its closure
    /// lands it relieves the pressure it stood for and builds order. A demand
    /// on a lane the reserve cannot fund waits; a page beyond one tick is
    /// answered across quotes. Where the court is due to be asked again, the
    /// quote is asked (the language moved, the book rose, or a dwell
    /// elapsed). Then, while the mind holds no language, the syllabus is
    /// asked for.
    pub(crate) fn quote_inflow(&mut self, outcome: &mut StrokeOutcome) {
        let lang = self.atlas_standing_counts();
        if let Some(q) = self.board.court.quoted.take() {
            let pace = &mut self.board.court;
            pace.last_lang = Some(lang);
            // 23.8: a height read off a reply only advances.
            pace.last_quoted_height = pace.last_quoted_height.max(q.height);
            pace.last_height = pace.last_height.max(q.height);
            pace.last_pressure = q.surface;
            pace.dwell_remaining = q.surface;
            self.answer_demands(&q.clusters, outcome);
        }
        let pace = &self.board.court;
        let due = pace.last_lang != Some(lang) || pace.last_height > pace.last_quoted_height || pace.dwell_elapsed();
        if due {
            self.quote();
        }
        if crate::economy::lanes_short(&self.physics().reserve_shortfall([true; 4])).is_empty()
            && (self.epigenome.genome_len() == 0 || self.epigenome.atlas_is_empty())
        {
            self.pull_syllabus();
        }
    }

    /// EACH DEMAND OF EACH LISTING, answered by the held closed form that
    /// reproduces it on its own chart (50.5, L8r): one spend per form spoken,
    /// its probe's closure booked where it lands.
    fn answer_demands(&mut self, clusters: &[Cluster], outcome: &mut StrokeOutcome) {
        let mut spend = self.stroke();
        for cluster in clusters {
            let mut pos = 0usize;
            let Some(e) = crate::transducer::element::Element::from_bytes(&cluster.element, &mut pos) else { continue };
            let held = crate::transducer::element::Held::of(e);
            let Some(form) = held.form() else { continue };
            let demands: Vec<_> = form.demand_vertices().map(|(at, _, chart, links)| (at, chart.clone(), links.to_vec())).collect();
            for (demand, chart, links) in &demands {
                let mag = measure_lanes(links);
                if !crate::economy::lanes_short(&self.physics().reserve_shortfall(Physics::lanes_of(&mag))).is_empty() {
                    continue;
                }
                for addr in &self.epigenome.chart_stratum(chart) {
                    if !spend.spend() {
                        return;
                    }
                    let (Some(word), Some(st)) = (self.epigenome.working_word(addr), self.epigenome.standing_of(addr)) else { continue };
                    let Some(f) = word.form().cloned() else { continue };
                    if !f.is_closed() || !reproduces(&self.epigenome, &f, links) {
                        continue;
                    }
                    let target = crate::transducer::sentence::demand_at(cluster.address.as_bytes(), demand);
                    let cite = crate::anastome::dialogue::CiteWire { addr: word.address(), standing: st.as_u8(), transport: crate::transducer::element::identity_versor(), body: links.clone() };
                    StrokeOutcome::add(&mut outcome.uttered, &mag);
                    self.ask(
                        move |s| s.probe(&target, std::slice::from_ref(&cite)),
                        move |me, reading| {
                            let Ok(reading) = reading else { return };
                            match reading.closure {
                                crate::anastome::dialogue::ProbeClosure::Closed => {
                                    me.physics_mut().relieve(&mag);
                                    me.physics_mut().syntropy(&mag);
                                    StrokeOutcome::add(&mut me.board.court.probed.closed, &mag);
                                }
                                crate::anastome::dialogue::ProbeClosure::Residual(_) => me.physics_mut().bump(&mag),
                                crate::anastome::dialogue::ProbeClosure::Gone | crate::anastome::dialogue::ProbeClosure::Nothing => {}
                            }
                        },
                    );
                    break;
                }
            }
        }
    }

    /// A TASK FROM THE COURT'S REGISTRY ENTERS (PoUW): its colour word and a
    /// colour theorem spoken, its hours held one pair at a time under the
    /// family's coordinate (78.56 III), the family thought through, every
    /// motion its form could not cite put to the court (78.160 P2), the
    /// tests spoken through the rule, and the solve — the rule minted at its
    /// span — flowing out to the court.
    /// The task that landed is perceived this stroke; the next is asked for
    /// only where the mind rests or returns (the winding carrier).
    pub(crate) fn task_inflow(&mut self, outcome: &mut StrokeOutcome, stop: &std::sync::atomic::AtomicBool) {
        let landed = self.board.court.task.take();
        let Some(crate::anastome::dialogue::TaskEnvelope { id, train, tests }) = landed else { return };
        let mag = super::family::family_magnitude(&train);
        if crate::economy::lanes_short(&self.physics().reserve_shortfall(Physics::lanes_of(&mag))).is_empty() {
            self.physics_mut().bump(&mag);
            StrokeOutcome::add(&mut outcome.fed, &mag);
        }
        self.speak_colour(&id, &train);
        let mut colour = self.stroke();
        self.speak_colour_theorem(&mut colour);
        let mut t = self.extract_family(&id, &train);
        if !t.frames.is_empty() {
            for (a, b) in &train {
                if stop.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }
                let pair = [(a.clone(), b.clone())];
                let hour = crate::transducer::family::perceive_family(&pair);
                let mut hour_budget = self.stroke_n(crate::transducer::family::extraction_spend(&pair, &hour));
                if let Some(f) = crate::transducer::family::scene_form_on(&self.epigenome, &mut hour_budget, a, b, &hour) {
                    self.hold_perceived_from(f, &hour, (&[][..], &pair[..]), &[], &id, &mut hour_budget);
                }
            }
        }
        let Some(rule) = t.rule.take() else {
            self.withhold_unextracted(&train, &id);
            self.await_comprehension(crate::anastome::dialogue::TaskEnvelope { id, train, tests });
            return;
        };
        self.ask_presentation(&rule.form);
        if let Some(uttered) = self.speak_form(&rule, &train, &t.frames, &id, &mut t.budget) {
            StrokeOutcome::add(&mut outcome.uttered, &uttered);
        }
        let envelope = crate::anastome::dialogue::TaskEnvelope { id, train, tests };
        if !self.comprehend(&envelope, rule, &mut t.budget) {
            self.await_comprehension(envelope);
        }
    }

    /// MU-CLIENT §4.9 — COMPREHENSION OF A SCENE, which may emit a solve: a
    /// rule is solved with only where its perspectives KNOW it — every
    /// training pair re-derived by the rule its siblings derive — and its
    /// tests spoken through it without remainder. A fit its own perspectives
    /// do not re-derive is not sent; the pairs they missed were lifted into
    /// the mind as structure (`ascertain`). `true` where the solve was
    /// emitted (held for the mind's return).
    fn comprehend(&mut self, task: &crate::anastome::dialogue::TaskEnvelope, rule: crate::transducer::sentence::SentenceRule, budget: &mut crate::physics::Budget) -> bool {
        let rule = match self.ascertain(&task.id, &task.train) {
            super::family::Ascertained::Known => rule,
            super::family::Ascertained::KnownByInversion(r) => *r,
            _ => return false,
        };
        let Some(outs) = self.speak_tests(&rule, &task.tests) else { return false };
        let spoken = self.mint_solve(&rule, &task.train);
        self.solve_task(&task.id, &task.train, &outs, spoken.as_ref(), budget).is_some()
    }

    /// A SCENE NOT YET COMPREHENDED WAITS BESIDE ITS THOUGHTS (MU-CLIENT
    /// §3.1, §4.9): kept by its citation with the language root it was last
    /// read at, read again at the mind's returns (`comprehend_waiting`), and
    /// gone when its last thought lapses — never by a count.
    fn await_comprehension(&mut self, task: crate::anastome::dialogue::TaskEnvelope) {
        let root = self.epigenome.atlas_root();
        self.comprehending.insert(task.id, (task, root));
    }

    /// AT A RETURN, ONE WAITING SCENE READ AGAIN: the least by citation after
    /// the last read. A scene none of whose thoughts still stands lapses with
    /// them; one the mind's language has not moved since its last reading is
    /// left as it was (nothing new to comprehend it by); otherwise its rule is
    /// extracted again and comprehended, and the scene leaves once its solve
    /// is emitted.
    pub(crate) fn comprehend_waiting(&mut self) {
        let next = match self.comprehending.after(&self.comprehended_after).next() {
            Some((k, _)) => Some(k),
            None => self.comprehending.scoped().next().map(|(k, _)| k),
        };
        let Some(id) = next else { return };
        self.comprehended_after = id.clone();
        if !self.contemplation.holds_family(&id) {
            self.comprehending.remove(&id);
            return;
        }
        let root = self.epigenome.atlas_root();
        if self.comprehending.get(&id).is_some_and(|(_, at)| *at == root) {
            return;
        }
        let Some((task, _)) = self.comprehending.remove(&id) else { return };
        let mut t = self.extract_family(&task.id, &task.train);
        let solved = match t.rule.take() {
            Some(rule) => self.comprehend(&task, rule, &mut t.budget),
            None => false,
        };
        if !solved {
            self.comprehending.insert(task.id, (task, root));
        }
    }

    /// THE COURT'S LANGUAGE FLOWS IN, where it is due — the language changed
    /// since the last sync, the book rose, a lane's dwell elapsed, or the
    /// mind holds no language — never because a tick rested: discovery, the
    /// syllabus and the sheaf sync by discordant stratum.
    pub(crate) fn language_inflow(&mut self) {
        let lang = self.atlas_standing_counts();
        let pace = &self.board.court;
        let due = pace.last_synced_lang != Some(lang) || pace.last_height > pace.last_synced_height || pace.dwell_elapsed() || self.epigenome.atlas_is_empty();
        if !due {
            return;
        }
        self.discover();
        self.pull_syllabus();
        self.sync_genome();
        let pace = &mut self.board.court;
        pace.last_synced_lang = Some(lang);
        pace.last_synced_height = pace.last_height;
    }
}

/// DOES THIS HELD FORM ANSWER THE DEMAND (50.5, L8q) — THE COURT'S OWN
/// READING (`Kernel::settle_demand`): the demand's bodies framed as a scene
/// at their own corner, the form spoken on it at the identity placement
/// (L14), and every link's AFTER colour said at its own site.
fn reproduces(epi: &Epigenome, f: &crate::transducer::sentence::Form, links: &[(i64, i64, u8, u8)]) -> bool {
    let Some(((r0, c0), scene, _)) = crate::transducer::family::scene_at_of_chain(links) else { return false };
    let placement = crate::transducer::alphabet::Word::identity();
    let Some(sp) = crate::transducer::family::Crystal::speak_form(epi, f, &scene, &placement) else { return false };
    let Some(spoken) = sp.grid() else { return false };
    // An offset below the corner, or past `i64`, is off the spoken grid.
    let at = |x: i64, o: i64| x.checked_sub(o).and_then(|d| usize::try_from(d).ok());
    links.iter().all(|&(r, c, _, a)| at(r, r0).and_then(|dr| spoken.get(dr)).and_then(|row| row.get(at(c, c0)?)).copied() == Some(a))
}
