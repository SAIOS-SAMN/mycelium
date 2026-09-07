// The client's transport aperture: the impedance it reads and the rtt it
// holds, both in strokes and never in seconds (law 9).

use crate::client::sdk::*;

impl Entity {
    /// The perceived transport aperture (see [`TransportAperture`]) — the
    /// entity's answer to "why can I not speak to the server, and what can
    /// I do?": measured impedance, adapted pacing, and an inward turn under
    /// occlusion.
    pub fn transport(&self) -> TransportAperture {
        self.session.transport()
    }

    /// The measured transport impedance Z (consecutive failed exchanges).
    pub fn impedance(&self) -> u64 {
        self.session.impedance()
    }

    pub fn transport_rtt(&self) -> Option<u64> {
        self.session.rtt()
    }

}
