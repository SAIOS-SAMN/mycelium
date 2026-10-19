// MU-CLIENT §4.4, §4.12, 78.168, §7.1 R1–R5 — THE WINDING CARRIER: the
// carrier of the mind's own motion, on its own circuit over its field, beside
// the carriers facing its counterparties (the hypha's, the conversation's,
// the seat's) and the perception carrier that perceives the scenes it takes
// in. It moves the orbits, binds the escaping ones, and ACTS AT EACH
// THOUGHT'S OWN RETURN — nowhere else. It replaces the tick (78.164, third
// amendment): no schedule of work, no outflow fired by an arrival or by the
// stroke itself, and no long law on it — a scene is perceived off the field
// and only its admission lands here (R1).
//
//! ONE WINDING:
//! 1. ADMISSION — what each carrier landed is taken where it landed, briefly:
//!    the court's answers, the conversation's hearings, the open book, a
//!    scene the metabolism can carry handed to the perception carrier, what
//!    that carrier read admitted, the seat's mail.
//! 2. THE FIELD MOVES — where a lane runs short, or nothing moved, the
//!    mind's metabolism relaxes and regenerates (R5); the held thoughts
//!    pivoted, what the pivots released vented into the field; then THE
//!    DOCKET TAKEN (R3), the one taker: every demand — a pivot's
//!    undischarged orbit, an open cell's escape, a relation that never
//!    returns — bound by what the field holds, each binding a close of a
//!    thought of the scene it stands in, and deliberated.
//! 3. THE RETURNS — the field's orbits that returned, the thoughts whose
//!    winding came round, and the demands bound, each with the scene it
//!    belongs to. K moves by exactly them (`close_cycle`).
//! 4. THE ACT AT A RETURN (R2) — what was held for a scene's return leaves
//!    where a thought of that scene returned, and nowhere else; what the
//!    mind holds for its own return (no scene) leaves at any return of the
//!    mind; and at a return the mind reads a waiting scene again, says what
//!    it owes its courts, its unheard words and its unjudged solves.
//! 5. THE NEXT SCENE is asked for (`pull_task`) only under TENSION — a demand
//!    on the plan, an orbit ringing, a reservoir uncondensed — where the
//!    metabolism stands regenerated and nothing waits to be perceived (the
//!    operator, 2026-10-17: "a full reserve over a settled field is
//!    equilibrium"); never by rest, never by a return. What any boundary
//!    lands is an OFFER, taken only where it couples to the field
//!    (`offer`); at a return, an offer that waited across the last one
//!    uncoupled lapses. A winding that moves nothing and returns nothing
//!    leaves the mind at rest.
//! 6. BOOKING — the stroke's physics on the reserve, and the court's dwell.
//!
//! The mind reports nothing about itself: its counterparties' acts are its
//! witness (MU-CLIENT §7.2); its observer reads it as it stands (`observe::live`).

use crate::client::sdk::*;
use crate::physics::StrokeOutcome;
use std::sync::atomic::AtomicBool;

/// THE BOARD: the boundaries this mind is open on, and what the field keeps
/// from stroke to stroke to meet them — the court's pace, the perception
/// carrier, and where its genome is written.
#[derive(Default)]
pub(crate) struct Board {
    pub(crate) court: super::court::CourtPace,
    pub(crate) feed: Option<super::feed::Feed>,
    pub(crate) body: Option<super::body::Body>,
    /// MU-CLIENT §7.1 R1 — the perception carrier, opened where the mind
    /// first takes a scene in.
    pub(crate) perceiver: Option<super::perceive::Perceiver>,
    /// Scenes and solves on the perception carrier not yet admitted: one at
    /// a time.
    pub(crate) scenes_perceiving: usize,
    /// MU-CLIENT §4.13 — HEARINGS WAITING FOR THE PERCEPTION CARRIER, by their
    /// coordinates (the speaker, then what was said: a thing said again by
    /// the same mind before it is read is one hearing, its latest saying in
    /// the place of the first), and whether one is on it: one at a time, as
    /// scenes are, each taken with a scratch of the field as it stands when
    /// it is taken. Bounded by the distinct things the mind's peers say in
    /// one traversal of the mind's orbit, never by a count: `hearings` are
    /// those heard in the traversal now running, `hearings_crossed` those
    /// that waited across its last return, which lapse at the next
    /// (`Entity::lapse_hearings`).
    pub(crate) hearings: crate::network::nerve::Keyed<super::perceive::Hearing>,
    pub(crate) hearings_crossed: crate::network::nerve::Keyed<super::perceive::Hearing>,
    pub(crate) hearing: bool,
    /// The hearings that lapsed unread across a return, since the mind began
    /// — a reading, never read back by cognition.
    pub(crate) hearings_lapsed: u64,
    /// MU-CLIENT §4.13 — THE RESERVE LOCKED IN THE FIELD'S RESERVOIRS, per
    /// lane: the strain they hold, taken out of the operational reserve at
    /// the vent and given back as it re-forms or condenses
    /// (`Entity::lock_reservoirs`).
    pub(crate) reservoir_locked: [u64; 4],
    /// MU-CLIENT §7.1 — a feed family that couples waits on the reserve: the
    /// mind's strokes regenerate it, so the mind does not block meanwhile.
    pub(crate) feed_waits: bool,
    /// The scenes taken in off the feed, by citation: their speech leaves at
    /// the mind's own return, not theirs (`speak_with`). Bounded by the
    /// families the feed held.
    pub(crate) feed_scenes: crate::network::nerve::Keyed<()>,
    /// MU-CLIENT §7.1 — THE OPEN QUESTIONS: residual chains a court taught or
    /// an older genome file held, by address — what perception could not
    /// say, never words (the operator, 2026-10-18). Volatile: a court teaches
    /// them again after a restart. Each is placed on the corpus and taken
    /// into thought where the focus meets it (`focus`); while any stands, the
    /// mind holds tension.
    pub(crate) open_residuals: crate::network::nerve::Keyed<Vec<(i64, i64, u8, u8)>>,
    /// The strata residual chains and rewrites stand on, by their chart
    /// invariant's key: the sheaf sync never fetches them, the mind holding
    /// residual chains as open questions and rewrites as the court's acts,
    /// never as words (`fetch_discordant`).
    pub(crate) residual_strata: crate::network::nerve::Keyed<()>,
    /// An open question waits on the reserve: the mind's strokes regenerate
    /// it, so the mind does not block meanwhile.
    pub(crate) question_waits: bool,
    /// The offers passed uncoupled since the mind began, by kind
    /// (`offer::PASSED`) — a reading, never read back by cognition.
    pub(crate) offers_passed: [u64; 2],
    /// WHERE EACH SCENE'S COMPREHENSION STOPPED since the mind began, by what
    /// it was taken as (`perceive::Reading`) and the stop (`perceive::Stop`) —
    /// a reading, never read back by cognition.
    pub(crate) stopped: [[u64; super::perceive::STOPS.len()]; 3],
    /// MU-CLIENT §7.1 — THE CORPUS the focus meets: every feed family and
    /// open question at its own coordinates (`focus`).
    pub(crate) corpus: super::focus::Corpus,
    genome_path: Option<String>,
    saved_len: Option<usize>,
}

/// WHAT ONE WINDING MOVED: the lanes its outcome moved (a winding that moved
/// none admitted nothing), and how many of the mind's orbits returned in it.
pub struct Wound {
    pub moved: [bool; 4],
    pub returns: u64,
}

impl Entity {
    /// THE CORPUS FEED at `home/feed` is a boundary of this mind.
    pub fn open_feed(&mut self, home: std::path::PathBuf) {
        let feed = super::feed::Feed::new(home);
        // A family arriving at the folder wakes the resting mind.
        feed.watch(self.hypha.waker());
        self.board.feed = Some(feed);
    }

    /// THE GENOME IS WRITTEN AT `path` whenever it gained a word, before the
    /// stroke that could run long, so a stop costs at most one stroke's
    /// learning.
    pub fn keep_genome(&mut self, path: String) {
        self.board.saved_len = Some(self.epigenome.genome_len());
        self.board.genome_path = Some(path);
    }

    /// The genome written where it is kept. LD0: one that does not write is
    /// said, never discarded.
    pub fn save_kept_genome(&mut self) -> Result<(), String> {
        let Some(path) = self.board.genome_path.clone() else { return Ok(()) };
        self.save_genome(&path).map(|_| self.board.saved_len = Some(self.epigenome.genome_len())).map_err(|e| e.to_string())
    }

    /// ONE WINDING OF THE WINDING CARRIER, thought in the mind's own field
    /// (MU-CLIENT §4.2), whose every law is a brief hold of it. `stop` is
    /// read between the laws.
    pub fn wind(&mut self, stop: &AtomicBool) -> Wound {
        let mut field = std::mem::take(&mut self.field);
        let wound = field.think(|| self.stroke_in_field(stop));
        self.field = field;
        wound
    }

    fn stroke_in_field(&mut self, stop: &AtomicBool) -> Wound {
        // The observer reads where the carrier stands at every law it enters
        // (`observe::live`, `entity::observer`).
        self.stroke_begins();
        if self.board.saved_len != Some(self.epigenome.genome_len()) {
            self.at_law("genome written");
            let _ = self.save_kept_genome();
        }
        let court = &mut self.board.court;
        for d in court.dwell_remaining.iter_mut() {
            if let Some(next) = d.checked_sub(1) {
                *d = next;
            }
        }
        let mut outcome = StrokeOutcome::default();
        let stopping = || stop.load(std::sync::atomic::Ordering::SeqCst);
        // 1. ADMISSION — what the carriers landed.
        self.at_law("admission: the court's answers");
        self.court_inflow(&mut outcome);
        self.at_law("admission: hearings priced");
        self.price_hearings();
        let occluded = matches!(self.transport(), TransportAperture::Occluded { .. });
        // AN OCCLUDED WIRE IS READ AGAIN, NEVER ABANDONED: the impedance
        // decays only on an exchange, so with nothing in flight one discovery
        // asks the nerve, and its answer or its refusal is the impedance read
        // again.
        if occluded && self.hypha.in_flight() == 0 {
            self.discover();
        }
        if !occluded && !stopping() {
            self.at_law("admission: the open book");
            self.quote_inflow(&mut outcome);
        }
        if !occluded && !stopping() {
            self.at_law("admission: a scene taken in");
            self.task_inflow(&mut outcome);
        }
        self.at_law("admission: what perception read");
        self.take_perceived();
        // MU-CLIENT §7.1 — THE CORPUS MET WHERE THE ORBITS STAND (`focus`):
        // read after perception's admission, so the orbits it carried — not
        // yet beaten at the vent — lead the focus, and the next item met
        // follows the last.
        if !stopping() {
            self.at_law("admission: what the focus meets");
            self.corpus_inflow(&mut outcome);
        }
        if !stopping() {
            self.at_law("admission: the seat's mail");
            self.body_tick();
        }
        // 2. THE FIELD MOVES. MU-CLIENT §4.12, R5 — REST IS METABOLISM: where
        // a lane runs short the mind relaxes and regenerates; where nothing
        // moved it does its inward work. Neither asks for a scene.
        let short = !crate::economy::lanes_short(&self.physics().reserve_shortfall([true; 4])).is_empty();
        if (short || outcome.moved() == [false; 4]) && !stopping() {
            self.at_law("motion: rest");
            self.relax(&mut outcome);
        }
        self.at_law("motion: the pivots");
        let _ = self.pivot_thoughts();
        StrokeOutcome::add(&mut outcome.learned, &self.take_learned_mu());
        let mut vent = self.stroke();
        self.at_law("motion: the vent");
        let _ = self.vent_into_field(&mut vent);
        // MU-CLIENT §4.13 — the dialect sheds what nothing live references,
        // the thoughts the field released having left.
        self.at_law("motion: the shed");
        let _ = self.shed();
        // The docket, the one taker: every demand bound, each binding a
        // close of a thought of its scene.
        self.at_law("motion: the docket taken");
        let bound = match stopping() {
            true => Vec::new(),
            false => self.take_docketed_demands(),
        };
        // 3. THE RETURNS, AND 4. THE ACT AT EACH RETURN.
        self.at_law("returns");
        let returns = self.act_on_returns(&mut vent, bound, !occluded);
        // 5. THE NEXT SCENE, by the metabolism.
        if !occluded && !stopping() {
            self.at_law("the next scene asked for");
            self.pull_task();
        }
        // The conversation runs on its own carrier (MU-CLIENT §5.1); it is
        // handed the mind's clock as the orbits move it.
        self.tell_conversation_the_clock();
        // 6. BOOKING.
        self.at_law("booking");
        self.book_tick(&outcome);
        let moved = outcome.moved();
        let lanes_short = self.physics().reserve_shortfall(moved);
        let wait: [bool; 4] = std::array::from_fn(|l| lanes_short[l] > 0);
        let pressure = self.board.court.pressure();
        self.board.court.dwell_remaining = self.physics().dwell(&pressure, wait);
        self.at_law("between windings");
        Wound { moved, returns }
    }

    /// MU-CLIENT §4.4(4), §7.1 R2 — THE RETURNS READ, AND THE ACT AT EACH:
    /// the winding's returns (the field's orbits that came round, the
    /// thoughts whose winding did, the demands the docket's taker `bound`),
    /// K moved by exactly them, and — where any returned and the wire is
    /// open — the act at the return, each scene's held outflow leaving at its
    /// own thought's return. Answers how many returned.
    pub(crate) fn act_on_returns(&mut self, budget: &mut crate::physics::Budget, bound: Vec<Vec<u8>>, open: bool) -> u64 {
        let (returns, mut scenes) = self.returns_of_stroke(budget);
        // MU-CLIENT §4.13, 78.168 — a thought the field released (its orbit
        // dissipated, sealed, or a perception's returned) is bound, as a
        // demand bound is: its scene's return.
        let mut bound = bound;
        bound.extend(self.contemplation.take_released_scenes());
        let bindings = u64::try_from(bound.len()).ok();
        for s in bound {
            if !scenes.contains(&s) {
                scenes.push(s);
            }
        }
        let returns = match bindings.and_then(|b| returns.checked_add(b)) {
            Some(n) => n,
            None => returns,
        };
        self.close_cycle(returns);
        // MU-CLIENT §4.13 — the hearings left unread across a return lapse,
        // and §7.1 the offers left uncoupled across one.
        if returns > 0 {
            if let Ok(lapsed) = u64::try_from(self.lapse_hearings()) {
                let _ = crate::physics::metabolism::step_or_refuse(&mut self.board.hearings_lapsed, lapsed);
            }
            self.lapse_offers();
        }
        if returns > 0 && open {
            self.act_at_return(&scenes);
        }
        returns
    }

    /// MU-CLIENT §4.4(4), §4.13, §7.1 R2 — WHAT THE MIND SAYS, HELD FOR THE
    /// RETURN OF THE THOUGHT IT SPEAKS FOR: `scene` is that thought's scene
    /// (its family's citation; empty for what the mind says for itself — a
    /// seat's call, a letter of no scene — which leaves at the mind's next
    /// return of any orbit), `at` the saying's own coordinates. Said again
    /// before that return, the latest saying stands in the place of the
    /// first — one thing said once.
    pub(crate) fn hold_for_return(&mut self, at: &[u8], scene: &[u8], say: impl FnOnce(&mut Entity) + 'static) {
        let held = match self.at_return.get_mut(scene) {
            Some(h) => h,
            None => {
                self.at_return.insert(scene, Vec::new());
                match self.at_return.get_mut(scene) {
                    Some(h) => h,
                    None => return,
                }
            }
        };
        match held.iter_mut().find(|(a, _)| a.as_slice() == at) {
            Some(slot) => slot.1 = Box::new(say),
            None => held.push((at.to_vec(), Box::new(say))),
        }
    }

    /// How many sayings are held for returns, over every scene.
    pub(crate) fn held_for_returns(&self) -> usize {
        self.at_return.scoped().map(|(_, h)| h.len()).sum()
    }

    /// THE WINDING'S RETURNS (MU-CLIENT §4.4): the orbits of the mind's field
    /// that returned since the last winding, and the thoughts whose winding
    /// came round — each transcribed against the mind's own letters and its
    /// word said under its scene (held for that scene's return). Answers how
    /// many, and the scenes they returned in.
    fn returns_of_stroke(&mut self, budget: &mut crate::physics::Budget) -> (u64, Vec<Vec<u8>>) {
        let field = match crate::transducer::mind::with_field(|f| f.take_returns()) {
            Ok(n) => n,
            // No field stands in the thread's slots: none of its orbits is
            // read as returned (rule 11 — absence, never a count).
            Err(_) => 0,
        };
        let came_round = u64::try_from(self.contemplation.closed.len()).ok();
        let mut scenes: Vec<Vec<u8>> = Vec::new();
        for (_, family) in &self.contemplation.closed {
            if !scenes.contains(family) {
                scenes.push(family.clone());
            }
        }
        self.at_law("returns: thoughts transcribed");
        let thoughts = self.transcribe_closed(budget);
        // MU-CLIENT §5.4 (E1) — each under the scene its thought was
        // perceived in.
        for (word, family) in thoughts {
            let _ = self.speak_word_on(&word, &[], &family);
        }
        // MU-CLIENT §4.4(3) — what each returned thought holds against its
        // orbit (its slips) is worked at its return.
        self.at_law("returns: slips worked");
        self.contemplation.work_returned();
        // 78.167 D1 — the given meanings a newly known word stood beside,
        // reduced again where the mind's orbits return.
        self.at_law("returns: given meanings reduced again");
        let rederived = crate::transducer::sentence::rung::re_reduce_given();
        if rederived > 0 {
            crate::observe::note_mind(|m| m.asks[4] += rederived);
        }
        let n = match came_round.and_then(|t| field.checked_add(t)) {
            Some(n) => n,
            // A count past its carrier is refused: the field's own returns
            // stand.
            None => field,
        };
        (n, scenes)
    }

    /// MU-CLIENT §4.4(4), §7.1 R2 — THE ACT AT A RETURN: a scene waiting on
    /// its comprehension read again, what the mind owes its courts, its words
    /// no court has taught and its unjudged solves; then what was held for
    /// the return of each scene in `scenes` leaves, and what the mind held
    /// for its own return (no scene) — and nothing held for a scene whose
    /// thought did not return.
    fn act_at_return(&mut self, scenes: &[Vec<u8>]) {
        self.at_law("act at a return: a waiting scene read again");
        self.comprehend_waiting();
        self.at_law("act at a return: what the courts are owed");
        self.court_owed();
        let mut budget = self.physics.stroke();
        self.at_law("act at a return: unheard words");
        self.speak_unheard(&mut budget);
        self.at_law("act at a return: unread solves");
        self.resubmit_unread_solves();
        self.at_law("act at a return: what was held leaves");
        let mut leaving: Vec<HeldSaying> = Vec::new();
        for scene in std::iter::once(&Vec::new()).chain(scenes) {
            if let Some(held) = self.at_return.remove(scene) {
                leaving.extend(held.into_iter().map(|(_, say)| say));
            }
        }
        for say in leaving {
            say(self);
        }
    }

    /// PHY1, 78.71 — THE STROKE'S PHYSICS BOOKED: one expansion on each lane
    /// its outcome moved, drawn from the one reserve (𝒯 never by a unit: it
    /// is withheld by the flux the field holds open, `lock_reservoirs`); the
    /// tubes' relief, and the contacts' order.
    fn book_tick(&mut self, outcome: &StrokeOutcome) {
        let seen = crate::transducer::family::pairing::contact_lanes();
        let mut moved = outcome.moved();
        moved[2] = false;
        let _ = self.physics_mut().spend(moved);
        let (relieved, _) = crate::transducer::family::pairing::flux_lanes();
        self.physics_mut().book_flux(relieved);
        self.physics_mut().book_contacts(seen);
    }
}
