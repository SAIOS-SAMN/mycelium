// MU-CLIENT §4.4, §4.12, 78.168 — THE WINDING CARRIER: the carrier of the
// mind's own motion, on its own circuit over its field, beside the carriers
// facing its counterparties (the hypha's, the conversation's). Its stroke
// takes what those carriers landed, moves the field, and ACTS AT THE
// RETURNS of the mind's orbits — nowhere else. It replaces the tick
// (78.164, third amendment): no schedule of work, no outflow fired by an
// arrival or by the stroke itself.
//
//! ONE STROKE OF THE WINDING:
//! 1. ADMISSION — what each counterparty's carrier landed is taken where it
//!    landed: the court's answers, the conversation's hearings, the feed's
//!    family, the court's open book and its task (a scene perceived), the
//!    seat's mail. Answers to a counterparty's frame (a probe answering a
//!    listing, HEARD, ACK, HOLD) leave where the frame lands.
//! 2. THE FIELD MOVES — the docket's demands constructed, the held thoughts
//!    pivoted, what the pivots closed vented into the field; and where the
//!    reserve runs short or nothing arrived, the mind RESTS (relaxes,
//!    regenerates) and, resting, asks for a scene.
//! 3. THE RETURNS — the orbits of the field that returned and the thoughts
//!    whose winding came round in this stroke. K moves by exactly them
//!    (`close_cycle`).
//! 4. THE ACT AT A RETURN — only where something returned: the closed
//!    thoughts' words transcribed and said, what the mind owes its courts,
//!    its unheard words, its unjudged solves, and every speech, solve and
//!    SAY held for the return (`at_return`) leave now. The mind's outflow
//!    is bounded by its own periods (MU-CLIENT §4.13).
//! 5. BOOKING — the stroke's physics on the reserve, and the court's dwell.
//!
//! The mind reports nothing about itself: its counterparties' acts are its
//! witness (MU-CLIENT §7.2).

use crate::client::sdk::*;
use crate::physics::StrokeOutcome;
use std::sync::atomic::AtomicBool;

/// THE BOARD: the boundaries this mind is open on, and what the field keeps
/// from stroke to stroke to meet them — the court's pace, the extraction
/// memo, and where its genome is written.
#[derive(Default)]
pub(crate) struct Board {
    pub(crate) court: super::court::CourtPace,
    pub(crate) feed: Option<super::feed::Feed>,
    pub(crate) body: Option<super::body::Body>,
    pub(crate) memo: crate::transducer::family::ExtractMemo,
    genome_path: Option<String>,
    saved_len: Option<usize>,
}

/// WHAT ONE STROKE OF THE WINDING MOVED: the lanes its outcome moved (a
/// stroke that moved none admitted nothing), and how many of the mind's
/// orbits returned in it.
pub struct Wound {
    pub moved: [bool; 4],
    pub returns: u64,
}

impl Entity {
    /// THE CORPUS FEED at `home/feed` is a boundary of this mind.
    pub fn open_feed(&mut self, home: std::path::PathBuf) {
        self.board.feed = Some(super::feed::Feed::new(home));
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

    /// ONE STROKE OF THE WINDING CARRIER, thought in the mind's own field
    /// (MU-CLIENT §4.2). `stop` is read inside the long laws, so a stroke
    /// thinking a family ends at its next boundary.
    pub fn wind(&mut self, stop: &AtomicBool) -> Wound {
        let mut field = std::mem::take(&mut self.field);
        let wound = field.think(|| self.stroke_in_field(stop));
        self.field = field;
        wound
    }

    fn stroke_in_field(&mut self, stop: &AtomicBool) -> Wound {
        if self.board.saved_len != Some(self.epigenome.genome_len()) {
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
        self.court_inflow(&mut outcome);
        self.price_hearings();
        let occluded = matches!(self.transport(), TransportAperture::Occluded { .. });
        // AN OCCLUDED WIRE IS READ AGAIN, NEVER ABANDONED: the impedance
        // decays only on an exchange, so with nothing in flight one discovery
        // asks the nerve, and its answer or its refusal is the impedance read
        // again.
        if occluded && self.hypha.in_flight() == 0 {
            self.discover();
        }
        if !occluded {
            self.feed_inflow(&mut outcome);
        }
        if !occluded && !stopping() {
            self.quote_inflow(&mut outcome);
        }
        if !occluded && !stopping() {
            self.task_inflow(&mut outcome, stop);
        }
        if !stopping() {
            self.body_tick();
        }
        // 2. THE FIELD MOVES — the docket constructed, the held thoughts
        // pivoted, and REST where the reserve runs short or nothing arrived.
        if !stopping() {
            self.take_docketed_demands();
        }
        let short = !crate::economy::lanes_short(&self.physics().reserve_shortfall([true; 4])).is_empty();
        let resting = short || outcome.moved() == [false; 4];
        if resting && !stopping() {
            self.relax(&mut outcome);
        }
        let _ = self.pivot_thoughts();
        StrokeOutcome::add(&mut outcome.learned, &self.take_learned_mu());
        let mut vent = self.stroke();
        let _ = self.vent_into_field(&mut vent);
        // 3. THE RETURNS, AND 4. THE ACT AT A RETURN.
        let returns = self.act_on_returns(&mut vent, !occluded);
        // A RESTING MIND ASKS FOR A SCENE: its field holds nothing that moved,
        // so it perceives (MU-CLIENT §4.12) — never a pull fired by the
        // stroke itself.
        if (resting || returns > 0) && !occluded && !stopping() {
            self.pull_task();
        }
        // The conversation runs on its own carrier (MU-CLIENT §5.1); it is
        // handed the mind's clock as the orbits move it.
        self.tell_conversation_the_clock();
        // 5. BOOKING.
        self.book_tick(&outcome);
        let moved = outcome.moved();
        let short = self.physics().reserve_shortfall(moved);
        let wait: [bool; 4] = std::array::from_fn(|l| short[l] > 0);
        let pressure = self.board.court.pressure();
        self.board.court.dwell_remaining = self.physics().dwell(&pressure, wait);
        Wound { moved, returns }
    }

    /// MU-CLIENT §4.4(4) — THE RETURNS READ, AND THE ACT AT THEM: the
    /// stroke's returns (the field's orbits that came round, the thoughts
    /// whose winding did), K stepped by exactly them, and — where any
    /// returned and the wire is open — the act at the return. Answers how
    /// many returned.
    pub(crate) fn act_on_returns(&mut self, budget: &mut crate::physics::Budget, open: bool) -> u64 {
        let returns = self.returns_of_stroke(budget);
        self.close_cycle(returns);
        if returns > 0 && open {
            self.act_at_return();
        }
        returns
    }

    /// MU-CLIENT §4.4(4), §4.13 — WHAT THE MIND SAYS, HELD FOR ITS RETURN BY
    /// ITS COORDINATES `at`: `say` leaves at the next stroke in which one of
    /// the mind's orbits returns. Said again before that return, the latest
    /// saying stands in the place of the first — one thing said once.
    pub(crate) fn hold_for_return(&mut self, at: &[u8], say: impl FnOnce(&mut Entity) + 'static) {
        match self.at_return_by.get(at).copied().and_then(|i| self.at_return.get_mut(i)) {
            Some(slot) => *slot = Box::new(say),
            None => {
                self.at_return_by.insert(at, self.at_return.len());
                self.at_return.push(Box::new(say));
            }
        }
    }

    /// THE STROKE'S RETURNS (MU-CLIENT §4.4): the orbits of the mind's field
    /// that returned since the last stroke, and the thoughts whose winding
    /// came round — each transcribed against the mind's own letters and its
    /// word said (held for the act at this return). Answers how many.
    fn returns_of_stroke(&mut self, budget: &mut crate::physics::Budget) -> u64 {
        let field = match crate::transducer::mind::with_field(|f| f.take_returns()) {
            Ok(n) => n,
            // No field stands in the thread's slot: none of its orbits is
            // read as returned (rule 11 — absence, never a count).
            Err(_) => 0,
        };
        let came_round = u64::try_from(self.contemplation.closed.len()).ok();
        let thoughts = self.transcribe_closed(budget);
        // MU-CLIENT §5.4 (E1) — each under the scene its thought was
        // perceived in.
        for (word, family) in thoughts {
            let _ = self.speak_word_on(&word, &[], &family);
        }
        // MU-CLIENT §4.4(3) — what each returned thought holds against its
        // orbit (its slips) is worked at its return.
        self.contemplation.work_returned();
        // 78.167 D1 — the given meanings a newly known word stood beside,
        // reduced again where the mind's orbits return.
        let rederived = crate::transducer::sentence::rung::re_reduce_given();
        if rederived > 0 {
            crate::observe::note_mind(|m| m.asks[4] += rederived);
        }
        match came_round.and_then(|t| field.checked_add(t)) {
            Some(n) => n,
            // A count past its carrier is refused: the field's own returns
            // stand.
            None => field,
        }
    }

    /// MU-CLIENT §4.4(4) — THE ACT AT A RETURN: a scene waiting on its
    /// comprehension read again, what the mind owes its courts, its words no
    /// court has taught, its unjudged solves, and every speech, solve and SAY
    /// held for the return leave now, and nowhere else.
    fn act_at_return(&mut self) {
        self.comprehend_waiting();
        self.court_owed();
        let mut budget = self.physics.stroke();
        self.speak_unheard(&mut budget);
        self.resubmit_unread_solves();
        // What was held for the return leaves: every speech, solve and SAY,
        // in the order the mind said them.
        self.at_return_by = crate::network::nerve::Keyed::new();
        for say in std::mem::take(&mut self.at_return) {
            say(self);
        }
    }

    /// PHY1, 78.71 — THE STROKE'S PHYSICS BOOKED: one expansion on each lane
    /// its outcome moved, drawn from the one reserve (𝒯 is drawn by the flux
    /// transported, never a unit); the orbits' relief, the transport on 𝒯,
    /// and the contacts' order.
    fn book_tick(&mut self, outcome: &StrokeOutcome) {
        let seen = crate::transducer::family::pairing::contact_lanes();
        let mut moved = outcome.moved();
        moved[2] = false;
        let _ = self.physics_mut().spend(moved);
        let closed: [u64; 4] = std::array::from_fn(|l| match (l, seen.0[l].checked_sub(self.physics().contacts_mark[0][l])) {
            (2, Some(d)) => d,
            _ => 0,
        });
        let (relieved, transported) = crate::transducer::family::pairing::flux_lanes();
        let on_t: [u64; 4] = std::array::from_fn(|l| if l == 2 { transported[l] } else { 0 });
        let _ = self.physics_mut().book_flux((relieved, on_t), closed);
        self.physics_mut().book_contacts(seen);
    }
}
