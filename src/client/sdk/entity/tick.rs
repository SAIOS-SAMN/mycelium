// 78.164 — THE TICK: the mind's field evolved one tick of TUniversal by the
// substrate's laws. The binary only opens the boundaries and hands the mind
// its ticks; what is done, and in what order, is the field's.
//
//! THE ORDER IS THE PHYSICS', never a schedule:
//! 1. ADMISSION — each boundary's inflow, paced by its own dwell: the
//!    court's answers its carrier landed are taken; what the mind owes the
//!    court flows out; the feed's family, the court's open book and its
//!    registry's task, and the body's mail flow in, each closed by the law
//!    its kind names (family, demand, seat) where the reserve can fund it.
//!    No exchange is waited on: the court's word arrives on its carrier, the
//!    other minds' on the conversation's (MU-CLIENT §5.1).
//! 2. TRANSPORT — the docketed demands constructed and the court's unjudged
//!    solves asked again.
//! 3. RELAXATION — where the reserve is short or nothing moved: the field's
//!    inward work (`relax`).
//! 4. CLOSURE AND BOOKING — the words learned met by the held thoughts, the
//!    orbit closed (K moves here and nowhere else), the tick's draw on the
//!    reserve, the flux transported and relieved, and the contacts' order.
//!    The mind reports nothing about itself: its counterparties' acts are
//!    its witness (MU-CLIENT §7.2).

use crate::client::sdk::*;
use crate::physics::StrokeOutcome;
use std::sync::atomic::AtomicBool;

/// THE BOARD: the boundaries this mind is open on, and what the field keeps
/// from tick to tick to meet them — the court's pace, the extraction memo,
/// and where its genome is written.
#[derive(Default)]
pub(crate) struct Board {
    pub(crate) court: super::court::CourtPace,
    pub(crate) feed: Option<super::feed::Feed>,
    pub(crate) body: Option<super::body::Body>,
    pub(crate) memo: crate::transducer::family::ExtractMemo,
    genome_path: Option<String>,
    saved_len: Option<usize>,
}

/// WHAT ONE TICK MOVED: the lanes its outcome moved; a tick that moved none
/// admitted nothing.
pub struct Ticked {
    pub moved: [bool; 4],
}

impl Entity {
    /// THE CORPUS FEED at `home/feed` is a boundary of this mind.
    pub fn open_feed(&mut self, home: std::path::PathBuf) {
        self.board.feed = Some(super::feed::Feed::new(home));
    }

    /// THE GENOME IS WRITTEN AT `path` whenever it gained a word, before the
    /// tick that could run long, so a stop costs at most one tick's learning.
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

    /// ONE TICK OF THE MIND'S FIELD (78.164), thought in the mind's own field
    /// (MU-CLIENT §4.2). `stop` is read inside the long laws, so a tick
    /// thinking a family ends at its next boundary.
    pub fn tick(&mut self, stop: &AtomicBool) -> Ticked {
        let mut field = std::mem::take(&mut self.field);
        let ticked = field.think(|| self.tick_in_field(stop));
        self.field = field;
        ticked
    }

    fn tick_in_field(&mut self, stop: &AtomicBool) -> Ticked {
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
        // 1. ADMISSION — the court's answers the carrier landed first.
        self.court_inflow(&mut outcome);
        let occluded = matches!(self.transport(), TransportAperture::Occluded { .. });
        let stopping = || stop.load(std::sync::atomic::Ordering::SeqCst);
        if !occluded {
            self.court_owed();
            self.feed_inflow(&mut outcome);
        }
        if !occluded && !stopping() {
            self.quote_inflow(&mut outcome);
        }
        if !occluded && !stopping() {
            self.task_inflow(&mut outcome, stop);
        }
        if !stopping() {
            self.body_tick(stop);
        }
        // 2. TRANSPORT.
        if !stopping() {
            self.take_docketed_demands();
            self.resubmit_unread_solves();
        }
        // 3. RELAXATION.
        let short = !crate::economy::lanes_short(&self.physics().reserve_shortfall([true; 4])).is_empty();
        if (short || outcome.moved() == [false; 4]) && !stopping() {
            self.relax(&mut outcome);
        }
        // 4. CLOSURE AND BOOKING.
        let _ = self.pivot_thoughts();
        StrokeOutcome::add(&mut outcome.learned, &self.take_learned_mu());
        self.close_cycle();
        // The tick's pivots reach the field — each flow on its line — and
        // what the field released relaxes (MU-CLIENT §4.5).
        let mut vent = self.stroke();
        let _ = self.vent_into_field(&mut vent);
        // The conversation runs on its own carrier (MU-CLIENT §5.1); it is
        // handed the mind's clock as the orbit moves it.
        self.tell_conversation_the_clock();
        self.book_tick(&outcome);
        let moved = outcome.moved();
        let short = self.physics().reserve_shortfall(moved);
        let wait: [bool; 4] = std::array::from_fn(|l| short[l] > 0);
        let pressure = self.board.court.pressure();
        self.board.court.dwell_remaining = self.physics().dwell(&pressure, wait);
        Ticked { moved }
    }

    /// PHY1, 78.71 — THE TICK'S PHYSICS BOOKED: one expansion on each lane
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
