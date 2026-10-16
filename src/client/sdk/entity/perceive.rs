// MU-CLIENT §3.1, §4.2, §4.9, §7.1 R1/R4 — THE PERCEPTION CARRIER: a scene the
// mind takes in — a task's family, a feed's, a waiting scene read again — is
// perceived on its own circuit, OFF the mind's field, and only its admission
// touches the field.
//
//! THE LONG LAWS. Extraction, each hour of a family as its form, the rule's
//! perspectives (`ascertain_on`), the tests spoken through it, a solve's
//! home replay: each reads the mind's LANGUAGE as it stood when the scene
//! was taken (a snapshot of the epigenome) and moves a SCRATCH of the mind's
//! field (`mind::scratch_here`), so the winding carrier and every other
//! carrier go on while it runs. The carrier is the mind's one carrier type
//! (`hypha::Carrier`, as the court's and the seat's are): what it reads lands
//! as an answer does, and the winding admits it (`take_perceived`): the scratch's
//! flux, lanes, taught words, lexicon and covers land on the field
//! (`mind::admit`), the hours and withheld pairs are held under the scene's
//! citation, the rule's speech is spoken, and a rule its perspectives know
//! emits its solve once its replay is read here too.
//!
//! THE PACE IS THE MIND'S METABOLISM (the operator, 2026-10-15: "there should
//! be internal metabolism in the mind"): a scene is taken when the reserve
//! has regenerated on the lanes it costs; the carrier perceives one scene at
//! a time, and the next is asked for when it is taken (`court.rs`).

use crate::client::sdk::*;
use crate::transducer::sentence::SentenceRule;
use std::sync::Arc;

use super::family::{Ascertained, Chain, PairPercept};

/// WHAT A SCENE IS TAKEN AS: a court's task (its colour, its hours, its rule
/// and the comprehension that may emit a solve), a feed's family (its rule
/// spoken, or withheld), or a waiting scene read again in a language that
/// moved (its comprehension alone).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Reading {
    Task,
    Feed,
    Again,
}

/// ONE SCENE TAKEN IN: its citation, its demonstration pairs, its tests, and
/// what it is taken as.
pub(crate) struct Scene {
    pub(crate) id: Vec<u8>,
    pub(crate) train: Vec<crate::transducer::family::TrainPair>,
    pub(crate) tests: Vec<crate::arc::Grid>,
    pub(crate) reading: Reading,
}

/// A RULE'S SPEECH AS `speech_of` PREPARES IT: the rule to speak, what it
/// closes on the family, and the remainders it leaves.
type Prepared = (SentenceRule, [u64; 4], Vec<Chain>);

/// WHAT THE CARRIER READ OF A SCENE, for the winding carrier to admit.
pub(crate) struct Perceived {
    pub(crate) scene: Scene,
    admission: Option<crate::transducer::mind::Admission>,
    /// The family's colour word, where its fibre moved a hue.
    colour: Option<Element>,
    /// Each demonstration pair as its own hour's form (78.56 III).
    hours: Vec<PairPercept>,
    /// The pairs withheld: every pair where no rule closed, and every
    /// perspective the rule's siblings left uncovered (lifted).
    withheld: Vec<PairPercept>,
    lifted: Vec<PairPercept>,
    /// The rule's speech, prepared (`speech_of`), with the frames it was
    /// perceived on.
    speech: Option<(Prepared, crate::transducer::family::Frames)>,
    /// A rule its perspectives know, with the tests' outputs where it speaks
    /// them whole.
    known: Option<(SentenceRule, Option<Vec<crate::arc::Grid>>)>,
    /// The chains the tests' speech left, as their scenes' forms.
    chains: Vec<PairPercept>,
}

/// RINGS ANOTHER MIND SAID, WAITING TO BE READ: the speaker, what was said,
/// the words, and what their evidence read on their scene.
pub(crate) struct Hearing {
    speaker: [u8; 32],
    said: Vec<u8>,
    words: Vec<crate::client::sdk::chatter::Spoken>,
    evident: super::converge::Evident,
}

/// A SOLVE TO BE READ BEFORE IT IS SENT (`Entity::solve_task`'s home check),
/// on the carrier: the scene, the outputs, and the rule minted at its span.
pub(crate) struct Judge {
    pub(crate) scene: Scene,
    pub(crate) outputs: Vec<crate::arc::Grid>,
    pub(crate) rule: Element,
}

/// WHAT THE HOME CHECK READ: the crossed rule to send (judged, or unjudged —
/// the court's replay resumes it, 78.158a U2); a residue no held letter
/// spells, with the chart it acts on, to construct; or nothing to send.
pub(crate) enum Verdict {
    Send(Box<Element>),
    Construct(crate::transducer::cga::Mv, Option<crate::transducer::stalk::Invariant>),
    Kept,
}

/// THE PERCEPTION CARRIER: the mind's carrier type (`hypha::Carrier`) over
/// the extraction memo it keeps between scenes.
pub(crate) type Perceiver = super::hypha::Carrier<crate::transducer::family::ExtractMemo>;

/// THE SCENE READ ON THE CARRIER, in its scratch field, on the language
/// given — every law the old pipeline ran on the field, run here.
fn perceive(memo: &mut crate::transducer::family::ExtractMemo, epi: &Epigenome, scene: Scene) -> Perceived {
    use super::family::{ascertain_on, chain_percept, extract_on, pair_percept, scene_percept, speech_of, tests_spoken};
    let mut read = Perceived {
        admission: None,
        colour: None,
        hours: Vec::new(),
        withheld: Vec::new(),
        lifted: Vec::new(),
        speech: None,
        known: None,
        chains: Vec::new(),
        scene,
    };
    let (id, train, tests) = (&read.scene.id, &read.scene.train, &read.scene.tests);
    if read.scene.reading == Reading::Task {
        // 78.53 — what the colour fibre did.
        read.colour = crate::transducer::family::fibre_relation(train).and_then(|s| crate::transducer::element::colour_word(&s));
    }
    let mut t = extract_on(memo, epi, id, train);
    // 78.56 III — each pair its own hour, held with no bundle.
    if read.scene.reading == Reading::Task && !t.frames.is_empty() {
        read.hours = train.iter().filter_map(|(a, b)| scene_percept(epi, a, b)).collect();
    }
    let Some(rule) = t.rule.take() else {
        if read.scene.reading != Reading::Again {
            read.withheld = train.iter().filter_map(|(a, b)| pair_percept(epi, a, b)).collect();
        }
        return read;
    };
    if read.scene.reading != Reading::Again {
        read.speech = Some((speech_of(epi, &rule, train, &mut t.budget), t.frames.clone()));
    }
    if read.scene.reading == Reading::Feed {
        return read;
    }
    // MU-CLIENT §4.9 — COMPREHENSION OF THE SCENE: a rule known by its
    // perspectives, its tests spoken through it.
    let (ascertained, lifts) = ascertain_on(epi, train);
    read.lifted = lifts.iter().filter_map(|(x, y)| pair_percept(epi, x, y)).collect();
    let known = match ascertained {
        Ascertained::Known => Some(rule),
        Ascertained::KnownByInversion(r) => Some(*r),
        _ => None,
    };
    if let Some(rule) = known {
        let (outputs, chains) = tests_spoken(epi, &rule, tests);
        read.chains = chains.iter().filter_map(|c: &Chain| chain_percept(epi, c)).collect();
        read.known = Some((rule, outputs));
    }
    read
}

/// A SOLVE THIS VOICE CAN STAND BEHIND, OR NONE SENT — THE HOME CHECK, read
/// on the carrier, off the field. A solve is a rule, and the court replays
/// it on the task's withheld pairs; so the voice first replays it on the
/// pairs it holds exactly as the court will:
/// - 78.142 — the word that CROSSES to the withheld pairs carries no overlap
///   between two of its training frames (`rule::crossing`); a rule that
///   cannot be re-minted without them is refuted at home;
/// - a rule its own train refutes is not sent (REFUTED AT HOME);
/// - a rule citing a word no court admitted is kept home, by name (CITES
///   UNHELD, 78.157 §3a), its citations resolved through the court-stamped
///   words alone;
/// - a replay no stroke here could read is SENT: unjudged at home is not
///   refuted, and the court resumes it (78.158a U2);
/// - a refusal is RESIDUE, never an exit (MU-CLIENT §4.4): a D names the
///   versor no held letter spells and the chart it acts on, constructed
///   there as the mind's own letter.
///
/// A rule the court already refused for this task is not sent again until
/// the voice's rule for it changes; that is read where the verdict lands
/// (`admit_verdict`). On the fleet (epoch 106) each voice sent ~600 solves
/// the court refused, the same rules round after round, with none of these
/// read.
pub(crate) fn judge_on(epi: &Epigenome, j: &Judge) -> Verdict {
    use crate::client::sdk::rule::{crossing, would_judge_with, NoRule};
    let Ok(crossed) = crossing(&j.rule, &j.scene.train, epi) else { return Verdict::Kept };
    let frames = crate::transducer::family::perceive_family(&j.scene.train);
    let mut budget = crate::physics::Physics::default().stroke_n(crate::transducer::family::extraction_spend(&j.scene.train, &frames));
    let resolve = |a: &[u8]| epi.court_word(a).and_then(|e| e.form().cloned());
    match would_judge_with(&crossed, &j.scene.train, &resolve, epi, &mut budget) {
        // Judged, or unjudged — the court's replay resumes it (U2).
        Ok(()) | Err(NoRule::Unread) => Verdict::Send(Box::new(crossed)),
        Err(NoRule::CitesUnheld(a)) => {
            eprintln!("  solve kept home: the rule cites {} no court admitted (cites unheld)", super::learn::hex_prefix(&a));
            Verdict::Kept
        }
        // MU-CLIENT §4.4 — A REFUSAL IS RESIDUE, NEVER AN EXIT: a D at home
        // names the versor no held letter spells and the chart it acts on.
        Err(NoRule::Refused(r)) => match &r.obstruction {
            crate::transducer::sentence::ClosureObstruction::Dialect { coset, chart } => {
                let residue = coset.iter().try_fold(None::<crate::transducer::cga::Mv>, |acc, (mask, q)| {
                    let term = crate::transducer::cga::Mv::blade(usize::try_from(*mask).ok()?, q.clone());
                    Some(Some(match acc {
                        Some(a) => a.add(&term),
                        None => term,
                    }))
                });
                match residue {
                    Some(Some(residue)) => Verdict::Construct(residue, chart.as_deref().and_then(crate::transducer::stalk::Invariant::from_key)),
                    _ => Verdict::Kept,
                }
            }
            _ => Verdict::Kept,
        },
        Err(_) => Verdict::Kept,
    }
}

impl Entity {
    /// The perception carrier, opened where the mind first takes a scene; it
    /// wakes the resting field through the hypha's own waker.
    fn perceiver(&mut self) -> &mut Perceiver {
        let wake = self.hypha.waker();
        self.board
            .perceiver
            .get_or_insert_with(|| super::hypha::Carrier::open("perception", crate::transducer::family::ExtractMemo::default(), wake, |_| {}, |_| {}))
    }

    /// The mind's language as it stands, for a carrier to read off the
    /// field.
    fn language(&self) -> Arc<Epigenome> {
        Arc::new(self.epigenome.clone())
    }

    /// Whether a scene or a solve is on the perception carrier, not yet
    /// admitted: one at a time (a hearing does not hold a scene back).
    pub(crate) fn perceiving(&self) -> bool {
        self.board.scenes_perceiving > 0
    }

    /// TAKE A SCENE IN: posted to the perception carrier with the language
    /// and a scratch of the field as they stand — the field stands in the
    /// thread's slots during a winding, and the scratch is taken of it there
    /// — and admitted where its reading lands (`admit_scene`).
    pub(crate) fn perceive_scene(&mut self, scene: Scene) {
        let language = self.language();
        let (mut field, from) = crate::transducer::mind::scratch_here();
        self.board.scenes_perceiving += 1;
        super::hypha::post(
            self.perceiver(),
            move |memo| {
                crate::observe::live::enter_carrier("perception", "a scene perceived off the field");
                let read = field.think(|| perceive(memo, &language, scene));
                crate::observe::live::enter_carrier("perception", "waiting for a scene");
                Perceived { admission: field.into_admission(&from), ..read }
            },
            |me, read| {
                me.scene_landed();
                me.admit_scene(read);
            },
        );
    }

    /// RINGS ANOTHER MIND SAID, to be read on the perception carrier off the
    /// field (MU-CLIENT §4.9, §7.1 R1) and understood where the reading
    /// lands, with what their evidence read on their scene (§5.5). MU-CLIENT
    /// §4.13 — ONE AT A TIME, BY ITS COORDINATES: the hearing waits on the
    /// board under the speaker and what was said, the same thing said again
    /// by the same mind standing in the place of the one waiting; the next is
    /// taken when the one on the carrier lands (`hear_next`).
    pub(crate) fn perceive_hearing(&mut self, speaker: [u8; 32], said: Vec<u8>, words: Vec<crate::client::sdk::chatter::Spoken>, evident: super::converge::Evident) {
        let mut key = speaker.to_vec();
        crate::transducer::address::put_key_bytes(&mut key, &said);
        // Said again, it is heard in this traversal, in the place of the first.
        self.board.hearings_crossed.remove(&key);
        self.board.hearings.insert(&key, Hearing { speaker, said, words, evident });
        self.hear_next();
    }

    /// MU-CLIENT §4.13 — A HEARING WAITS AT MOST ACROSS ONE RETURN: at the
    /// mind's return (`act_on_returns`), the hearings that already waited
    /// across the last return lapse unread — the sensory buffer, never a
    /// ledger; a peer that needs agreement says its word again — and those
    /// heard in the traversal the return closes cross it, to wait until the
    /// next. None lapses the instant it lands. The return is the event; no
    /// clock is read. Answers how many lapsed.
    pub(crate) fn lapse_hearings(&mut self) -> usize {
        let crossed = std::mem::take(&mut self.board.hearings);
        let lapsed = std::mem::replace(&mut self.board.hearings_crossed, crossed);
        lapsed.len()
    }

    /// Every hearing waiting, in either traversal.
    pub(crate) fn hearings_waiting(&self) -> usize {
        self.board.hearings.len() + self.board.hearings_crossed.len()
    }

    /// THE NEXT HEARING TAKEN, where none is on the carrier — the least by
    /// its coordinates over every hearing waiting, never by its arrival
    /// (rule 15) — posted with what the mind remembers hearing and a scratch
    /// of its field as they stand now.
    fn hear_next(&mut self) {
        if self.board.hearing {
            return;
        }
        let first = |set: &crate::network::nerve::Keyed<Hearing>| set.scoped().next().map(|(k, _)| k);
        let taken = match (first(&self.board.hearings), first(&self.board.hearings_crossed)) {
            (Some(a), Some(b)) if b < a => self.board.hearings_crossed.remove(&b),
            (Some(a), _) => self.board.hearings.remove(&a),
            (None, Some(b)) => self.board.hearings_crossed.remove(&b),
            (None, None) => None,
        };
        let Some(Hearing { speaker, said, words, evident }) = taken else { return };
        self.board.hearing = true;
        let (mut field, from) = crate::transducer::mind::scratch_here();
        let mut remembered = self.remembered.clone();
        super::hypha::post(
            self.perceiver(),
            move |_| {
                crate::observe::live::enter_carrier("perception", "rings another mind said, read off the field");
                let reads = field.think(|| super::chatter::read_rings(&mut remembered, words));
                crate::observe::live::enter_carrier("perception", "waiting for a scene");
                (reads, field.into_admission(&from))
            },
            move |me, (reads, admission)| {
                me.board.hearing = false;
                let before = crate::transducer::sentence::rung::lexicon();
                if let Some(a) = admission {
                    let mut budget = me.physics.stroke();
                    let _ = crate::transducer::mind::admit(a, &mut budget);
                }
                me.understand_read(speaker, said, reads, before, evident);
                me.hear_next();
            },
        );
    }

    /// A SOLVE TO BE READ BEFORE IT IS SENT, posted to the perception
    /// carrier and admitted where its verdict lands.
    fn judge_solve(&mut self, judge: Judge) {
        let language = self.language();
        self.board.scenes_perceiving += 1;
        super::hypha::post(
            self.perceiver(),
            move |_| {
                crate::observe::live::enter_carrier("perception", "a solve read before it is sent");
                let verdict = judge_on(&language, &judge);
                crate::observe::live::enter_carrier("perception", "waiting for a scene");
                (judge, verdict)
            },
            |me, (judge, verdict)| {
                me.scene_landed();
                me.admit_verdict(judge, verdict);
            },
        );
    }

    /// A scene or a solve landed off the perception carrier.
    fn scene_landed(&mut self) {
        if let Some(n) = self.board.scenes_perceiving.checked_sub(1) {
            self.board.scenes_perceiving = n;
        }
    }

    /// WHAT THE PERCEPTION CARRIER READ, ADMITTED where it lands, in the
    /// order it was taken.
    pub(crate) fn take_perceived(&mut self) {
        super::hypha::take_from(self, |e| e.board.perceiver.as_mut());
    }

    /// Every scene, solve and hearing posted to the perception carrier,
    /// admitted: the field waits on that carrier. A guard's only — nothing
    /// the mind does waits on a carrier (MU-CLIENT §5.1).
    #[cfg(test)]
    pub(crate) fn settle_perception(&mut self) {
        loop {
            let before = match self.board.perceiver.as_ref() {
                Some(p) if p.in_flight() > 0 => p.in_flight(),
                _ => return,
            };
            self.take_perceived();
            if self.board.perceiver.as_ref().is_some_and(|p| p.in_flight() == before) {
                self.hypha.wait_landing();
            }
        }
    }

    /// ONE SCENE'S READING ADMITTED: its flux, lanes, taught words, lexicon
    /// and covers land on the field; its colour spoken; its hours held under
    /// its citation; the pairs no rule closed and the perspectives left
    /// uncovered withheld there; the rule's speech spoken; and where its
    /// perspectives know the rule and it speaks every test whole, the solve
    /// minted at its span and read before it is sent. A scene its
    /// comprehension did not reach waits beside its thoughts.
    fn admit_scene(&mut self, p: Perceived) {
        let mut budget = self.physics.stroke();
        if let Some(a) = p.admission {
            let _ = crate::transducer::mind::admit(a, &mut budget);
        }
        let Scene { id, train, tests, reading } = p.scene;
        // 78.53 — WHAT THE COLOUR FIBRE DID, minted as a colour word on the
        // carrier: stood in the dialect and spoken WITH THE SCENE it was
        // seen in, so the court re-derives it from its own copy of the
        // family. A colour word is its relation, so one already spoken or in
        // the genome is found by its address (every colour word shares the
        // motionless vertex; a vertex test would call them all one). Then
        // 78.55, the language's closure under its colour words.
        if let Some(word) = p.colour {
            let addr = word.address();
            if !self.already_taught_addr(addr.as_bytes()) && !self.epigenome.holds(addr.as_bytes()) {
                self.speak_with(std::slice::from_ref(&word), &train, &id, |_, _| {});
            }
        }
        if reading == Reading::Task {
            self.speak_colour_theorem(&mut budget);
        }
        for h in p.hours {
            self.hold_scene(h, &id);
        }
        for w in p.withheld {
            self.withhold_pair(w, &id);
        }
        for l in p.lifted {
            self.lift_pair(l, &id);
        }
        for c in p.chains {
            self.hold_scene(c, &[]);
        }
        if let Some((prepared, frames)) = p.speech {
            self.ask_presentation(&prepared.0.form);
            let _ = self.speak_prepared(prepared, &train, &frames, &id, &mut budget);
        }
        let scene = Scene { id, train, tests, reading };
        if reading == Reading::Feed {
            return;
        }
        match p.known {
            Some((rule, Some(outputs))) => match self.mint_solve(&rule, &scene.train) {
                Some(minted) => self.judge_solve(Judge { scene, outputs, rule: minted }),
                None => self.await_scene(scene),
            },
            _ => self.await_scene(scene),
        }
    }

    /// A SOLVE'S HOME CHECK ADMITTED: a crossed rule this task has not
    /// already had refused is submitted (held for the scene's return); a
    /// residue no letter spells is constructed where it acts; a scene whose
    /// solve was not sent waits beside its thoughts.
    fn admit_verdict(&mut self, j: Judge, v: Verdict) {
        let Ok(id) = <[u8; 32]>::try_from(j.scene.id.as_slice()) else { return };
        match v {
            Verdict::Send(rule) => {
                let refused = self.solve_refused.get(&id).is_some_and(|a| a == rule.address().as_bytes());
                // Submitted, it is held for the return of a thought of its
                // scene (R2) and leaves there.
                if refused || self.submit_solve_rule(&id, &j.outputs, &rule).is_err() {
                    self.await_scene(j.scene);
                }
            }
            Verdict::Construct(residue, chart) => {
                let mut budget = self.physics.stroke();
                self.construct_and_adjoin(&residue, chart.as_ref(), &j.scene.id, &mut budget);
                self.await_scene(j.scene);
            }
            Verdict::Kept => self.await_scene(j.scene),
        }
    }

    /// A SCENE NOT YET COMPREHENDED WAITS BESIDE ITS THOUGHTS (MU-CLIENT
    /// §3.1, §4.9): kept by its citation with the language root it was read
    /// at, read again at the mind's returns where the language moved. A
    /// scene named by no task's citation is not kept.
    fn await_scene(&mut self, scene: Scene) {
        let Ok(id) = <[u8; 32]>::try_from(scene.id.as_slice()) else { return };
        let root = self.epigenome.atlas_root();
        let envelope = crate::anastome::dialogue::TaskEnvelope { id, train: scene.train, tests: scene.tests };
        self.comprehending.insert(id, (envelope, root));
    }
}
