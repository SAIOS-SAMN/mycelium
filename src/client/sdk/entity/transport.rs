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

    /// REST until the court's word lands or the idle park passes.
    pub fn rest(&self) {
        self.hypha.rest(self.transport_rtt());
    }
}
