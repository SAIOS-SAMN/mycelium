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
    /// MU-CLIENT §4.13 — THE COURT'S GRAVITY ON THE MIND'S FIELD: the
    /// deficit its last deferral named, per lane, standing until the court
    /// answers a quote; the mind's beat drags on these lanes
    /// (`Field::beat_under`).
    pub(crate) deficit: [u64; 4],
    last_synced_height: u64,
    pub(crate) quoting: bool,
    pub(crate) quoted: Option<YieldQuote>,
    pub(crate) pulling: bool,
    /// MU-CLIENT §7.1 — the task the court sent, with its offer (`offer`).
    pub(crate) task: Option<(crate::anastome::dialogue::TaskEnvelope, super::offer::Offer)>,
    pub(crate) syncing: bool,
    pub(crate) schooling: bool,
    pub(crate) solving: crate::network::nerve::Keyed<()>,
    probed: StrokeOutcome,
    /// The solves this mind was paid for since it began (a bounty or a floor
    /// filed): the one event that raises its balance at a court.
    pub(crate) earned: u64,
    /// THE STRATA FETCHED FOR NOTHING, by chart key: the court's root for the
    /// stratum when its fetch brought no word, and `earned` then. Every word
    /// it lacks there costs a lane this learner is short on, and the court
    /// defers it unpaid; it is fetched again only once the court's root moves
    /// or the mind is paid (2026-10-22: 439 156 and 806 768 deferrals on
    /// courts a and c, the same strata asked at every sync). One entry per
    /// chart, the atlas its bound.
    pub(crate) starved: crate::network::nerve::Keyed<(Vec<u8>, u64)>,
}

impl CourtPace {
    /// A pressed lane whose dwell has run out, the T lane excluded (C4).
    fn dwell_elapsed(&self) -> bool {
        [0usize, 1, 3].iter().any(|&l| self.last_pressure[l] > 0 && self.dwell_remaining[l] == 0)
    }

    /// Whether a dwell still runs on a lane the court pressed: strokes are
    /// owed to it before the court is asked again.
    pub(crate) fn dwelling(&self) -> bool {
        self.last_pressure.iter().zip(&self.dwell_remaining).any(|(p, d)| *p > 0 && *d > 0)
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
            // A deferral's deficit stands as the court's gravity; a quote
            // the court answered relieves it.
            pace.deficit = match q.deficit {
                Some(d) => d,
                None => [0; 4],
            };
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
                        move |entity, reading| {
                            let Ok(reading) = reading else { return };
                            match reading.closure {
                                crate::anastome::dialogue::ProbeClosure::Closed => {
                                    entity.physics_mut().relieve(&mag);
                                    entity.physics_mut().syntropy(&mag);
                                    StrokeOutcome::add(&mut entity.board.court.probed.closed, &mag);
                                }
                                crate::anastome::dialogue::ProbeClosure::Residual(_) => entity.physics_mut().bump(&mag),
                                crate::anastome::dialogue::ProbeClosure::Gone | crate::anastome::dialogue::ProbeClosure::Nothing => {}
                            }
                        },
                    );
                    break;
                }
            }
        }
    }

    /// MU-CLIENT §3.1, §7.1 R4 — A TASK IS PERCEPTION, AND AN OFFER: a scene
    /// the court's registry sent is TAKEN IN where it couples to the mind
    /// (`offer`) and the mind's metabolism can carry it — the reserve
    /// regenerated on the lanes it costs (the mind's internal
    /// metabolism) — and perceived on the perception
    /// carrier, off the field (R1): its colour, its hours under its citation
    /// (78.56 III), its rule spoken, and its comprehension, which may emit a
    /// solve (`entity/perceive.rs`). A scene that couples to nothing, or that
    /// the metabolism cannot yet carry, waits where it landed, and one that
    /// still couples to nothing at its second return lapses
    /// (`lapse_offers`); one scene is perceived at a time.
    pub(crate) fn task_inflow(&mut self, outcome: &mut StrokeOutcome) {
        if self.perceiving() {
            return;
        }
        let Some((envelope, offer)) = self.board.court.task.as_ref() else { return };
        if !self.couples(&envelope.id, &offer.charts) {
            return;
        }
        let mag = super::family::family_magnitude(&envelope.train);
        if !crate::economy::lanes_short(&self.physics().reserve_shortfall(Physics::lanes_of(&mag))).is_empty() {
            return;
        }
        let Some((crate::anastome::dialogue::TaskEnvelope { id, train, tests }, _)) = self.board.court.task.take() else { return };
        self.physics_mut().bump(&mag);
        StrokeOutcome::add(&mut outcome.fed, &mag);
        self.perceive_scene(super::perceive::Scene { id: id.to_vec(), train, tests, reading: super::perceive::Reading::Task });
    }

    /// AT A RETURN, ONE WAITING SCENE READ AGAIN: the least by citation after
    /// the last read. A scene none of whose thoughts still stands lapses with
    /// them; one whose OWN language has not moved since its last reading —
    /// the words filed on the charts its bodies stand on (`scene_language`)
    /// — is left at rest (nothing new to comprehend it by); otherwise it is
    /// taken in again on the perception carrier, comprehension alone, and
    /// waits again where its solve is not emitted.
    ///
    /// The gate read the WHOLE language's root, which the mind's dialect
    /// moves at nearly every return as it stands and sheds its own words: a
    /// waiting scene was read again whether or not anything new could speak
    /// on it — 461 re-reads in an hour on one voice (observed on the
    /// development deployment, 2026-10-20), each holding the perception carrier from a new scene.
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
        let unmoved = match self.comprehending.get(&id) {
            Some((task, at)) => self.scene_language(&task.train) == *at,
            None => true,
        };
        if unmoved {
            return;
        }
        // Read again only where the perception carrier is free; otherwise it
        // waits as it was, to be read at a later return.
        if self.perceiving() {
            return;
        }
        let Some((task, _)) = self.comprehending.remove(&id) else { return };
        self.perceive_scene(super::perceive::Scene { id: task.id.to_vec(), train: task.train, tests: task.tests, reading: super::perceive::Reading::Again });
    }

    /// THE COURT'S LANGUAGE FLOWS IN, where the COURT'S language may have
    /// moved — its book rose since the last sync, a lane's dwell elapsed, or
    /// the mind holds no language — never because the mind's own motion moved
    /// (ruled 2026-10-18: a thought held or a dialect word stood is
    /// the mind's, and a sync brings nothing for it; counted, every family a
    /// large mind perceived re-ran a minutes-long sync) and never because a
    /// tick rested: discovery, the syllabus and the sheaf sync by discordant
    /// stratum.
    pub(crate) fn language_inflow(&mut self) {
        let pace = &self.board.court;
        let due = pace.last_height > pace.last_synced_height || pace.dwell_elapsed() || self.epigenome.atlas_is_empty();
        if !due {
            return;
        }
        self.discover();
        self.pull_syllabus();
        self.sync_genome();
        let pace = &mut self.board.court;
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
