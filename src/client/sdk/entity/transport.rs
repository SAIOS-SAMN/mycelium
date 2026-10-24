// The client's transport aperture: the impedance it reads and the rtt it
// holds, both in strokes and never in seconds (law 9).

use crate::client::sdk::*;

impl Entity {
    /// The perceived transport aperture (see [`TransportAperture`]) — the
    /// entity's answer to "why can I not speak to the server, and what can
    /// I do?": measured impedance, adapted pacing, and an inward turn under
    /// occlusion.
    pub(crate) fn transport(&self) -> TransportAperture {
        self.hypha.transport()
    }

    pub fn transport_rtt(&self) -> Option<u64> {
        self.hypha.rtt()
    }

    /// REST (MU-CLIENT §7.1, the operator 2026-10-17): blocked until a carrier
    /// lands something — never a clock — save while the court's dwell runs
    /// on a pressed lane, or a feed family that couples waits on the reserve:
    /// both are owed the mind's own strokes, so those strokes are parked at
    /// the measured transport until they are paid. A seated mind blocks too:
    /// its mail read is held at the relay to the court's next event and its
    /// landing wakes the mind (`read_mail_page`) — a timed rest for an unheld
    /// seat (cc8ce504) is retired, its wakes having read the relay ~2 400
    /// times a second with five minds (2026-10-22). So is a mind HOLDING
    /// TENSION ON A LANE RUN SHORT (2026-10-23): it asks the court only on a
    /// reserve regenerated on every lane, and the reserve regenerates on its
    /// own strokes — blocked, voice 4 sat short on G (7:0:56:7) with tension
    /// and asked for nothing, its woken siblings likewise. So is a mind with
    /// a COURT'S TASK WAITING where it landed (2026-10-24): it is taken in
    /// as the field moves, or lapses at the second return it waits across
    /// uncoupled (`lapse_offers`), and the field moves only on the mind's
    /// strokes — blocked, all five voices held an uncoupled task from 01:15,
    /// none lapsed, `pull_task` asked nothing past it, and the three courts'
    /// K stood. A mind with no tension still blocks: it may idle.
    ///
    /// THE QUIESCENCE LAW (the auditor's ruling, 2026-10-24): the mind blocks
    /// if and only if it is quiescent — nothing it holds needs its own
    /// strokes to progress (`owes_strokes`); otherwise it rests at the
    /// measured transport.
    pub fn rest(&self) {
        match self.owes_strokes() {
            true => self.hypha.rest(self.transport_rtt()),
            false => self.hypha.block(),
        }
    }

    /// WHAT THE MIND HOLDS THAT WAITS ON ITS OWN STROKES: the court's dwell
    /// on a pressed lane; a feed family or a question met and waiting on the
    /// reserve; a lane run short under tension, regenerating; a court's task
    /// on the board, taken in or lapsing as the field moves.
    pub(crate) fn owes_strokes(&self) -> bool {
        let dwell = self.board.court.dwelling();
        let feed = self.board.feed_waits;
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
