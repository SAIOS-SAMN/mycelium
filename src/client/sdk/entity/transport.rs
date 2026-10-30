// The client's transport aperture: the impedance it reads and the rtt it
// holds, both in strokes and never in seconds (law 9).

use crate::client::sdk::*;

impl Entity {
    /// The perceived transport aperture (see [`TransportAperture`]) — the
    /// entity's answer to "why can the entity not speak to the server, and
    /// what can it do?": measured impedance, adapted pacing, and an inward turn under
    /// occlusion.
    pub(crate) fn transport(&self) -> TransportAperture {
        self.hypha.transport()
    }

    pub fn transport_rtt(&self) -> Option<u64> {
        self.hypha.rtt()
    }

    /// REST (MU-CLIENT §7.1; ruled 2026-10-24): the mind blocks until a
    /// carrier lands something — never a clock — where nothing it holds
    /// needs its own strokes to progress (`owes_strokes`); otherwise it parks
    /// at the measured transport. A seated mind's mail read is held at the
    /// relay and its landing wakes the mind (`read_mail_page`).
    pub fn rest(&self) {
        // The observer's asker waits while the carrier works and stops at a
        // park (`live::call`).
        crate::observe::live::park(true);
        match self.owes_strokes() {
            true => self.hypha.rest(self.transport_rtt()),
            false => self.hypha.block(),
        }
        crate::observe::live::park(false);
    }

    /// WHAT WAITS ON THE MIND'S OWN STROKES: the court's dwell on a pressed
    /// lane; a feed family or a question met and waiting on the reserve; a
    /// lane run short under tension (the reserve regenerates only on
    /// strokes); a court's task on the board (it is taken in or lapses only
    /// as strokes pass); hearings left queued past a stroke's budget.
    pub(crate) fn owes_strokes(&self) -> bool {
        let dwell = self.board.court.dwelling();
        let feed = self.board.feed_waits || self.board.hearings_wait;
        let questions = self.board.question_waits;
        let reserve = self.strained() && !crate::economy::lanes_short(&self.physics().reserve_shortfall([true; 4])).is_empty();
        let attention = self.board.court.task.is_some();
        dwell || feed || questions || reserve || attention
    }

    /// The waker of the mind's winding: whatever sends on it ends a rest.
    pub fn waker(&self) -> std::sync::mpsc::Sender<()> {
        self.hypha.waker()
    }
}
