// MU-CLIENT §4.5 — THE MIND'S PIVOTS ON ITS OWN COMPLEX: every change of
// dimension the contemplation makes is one of the six pivots, applied at its
// coordinate through `Complex::apply`, its outcome returned whole.
//
//! CONDENSE. Two held thoughts the transport relates stand in tension, and
//! the tension is a TETHER: condensed with its transition. Where a tether
//! already stands on the pair carrying the same transport, nothing changes.
//! Where the tension re-formed on another transport, the standing tether
//! DRAINS first — its strain vented into the field, its class cauterized out
//! where it carried one — and the new one condenses: the conjugate pair,
//! never an overwrite. A tether pinned beneath a read cell does not drain
//! (63.2): the read cell's class stands on it, and the new transport is not
//! written over it.

use super::*;
use crate::transducer::cech::{Applied, Condensed, Drained, Pivot};
use crate::transducer::element::Versor;

impl Contemplation {
    /// CONDENSE the tension between `a` and `b` into a tether carrying
    /// `rotor` and `carriage`, with `volume` per lane; a standing tether on
    /// another transport drains first. Answers what the condense did.
    pub(crate) fn tether(&mut self, a: &[u8], b: &[u8], rotor: Rotor, carriage: Versor, volume: [u64; 4]) -> Condensed {
        let pair = Pivot::Condense(a.to_vec(), b.to_vec(), Box::new(rotor.clone()), carriage.clone());
        match self.complex.apply(&pair, |_| volume, volume) {
            Applied::Condensed(Condensed::AlreadyTethered) if !self.complex.tethered_as(a, b, &rotor, &carriage) => {
                let standing = crate::transducer::cech::simplex_key(&crate::transducer::cech::sorted_verts([a, b])).as_bytes().to_vec();
                match self.complex.apply(&Pivot::Drain(standing), |_| volume, volume) {
                    Applied::Drained(Drained::Pinned) => Condensed::AlreadyTethered,
                    _ => match self.complex.apply(&pair, |_| volume, volume) {
                        Applied::Condensed(c) => c,
                        _ => Condensed::Absent,
                    },
                }
            }
            Applied::Condensed(c) => c,
            _ => Condensed::Absent,
        }
    }
}
