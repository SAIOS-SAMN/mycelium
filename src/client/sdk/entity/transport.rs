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
    /// the measured transport until they are paid.
    pub fn rest(&self) {
        match self.board.court.dwelling() || self.board.feed_waits || self.board.question_waits {
            true => self.hypha.rest(self.transport_rtt()),
            false => self.hypha.block(),
        }
    }

    /// The waker of the mind's winding: whatever sends on it ends a rest.
    pub fn waker(&self) -> std::sync::mpsc::Sender<()> {
        self.hypha.waker()
    }
}
