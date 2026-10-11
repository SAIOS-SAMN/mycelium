// 78.127, 78.137 — A HOUSE'S ACTS ON ITS HYPHA: a domain founded, its
// reserve designated, the reserve read back. House tooling beside the mind
// (the durable-admission drill, `examples/durable_drill.rs`), never the mind's
// cognition (MU-CLIENT §2.3). Each goes through the raw act door with the
// domain codec; a reply that does not decode is an error by name.

use std::io;

impl super::Session {
    /// 78.122 — A DOMAIN FOUNDED: sealed, already founded, or refused by name.
    pub fn found_domain(&self, w: &crate::anastome::dialogue::domain::SignedDomain) -> io::Result<crate::anastome::dialogue::domain::Founded> {
        use crate::anastome::dialogue::domain::{decode_domain_reply, encode_domain};
        let reply = self.speak(encode_domain(w))?;
        decode_domain_reply(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the domain's reply did not decode"))
    }

    /// 78.127 — A DOMAIN'S RESERVE TIER DESIGNATED: sealed, and whether it
    /// holds now or from the current reserve tier's next fold; or the
    /// refusal by name.
    pub fn reserve(&self, r: &crate::anastome::dialogue::domain::SignedReservation) -> io::Result<crate::anastome::dialogue::domain::Founded> {
        use crate::anastome::dialogue::domain::{decode_domain_reply, encode_reservation};
        let reply = self.speak(encode_reservation(r))?;
        decode_domain_reply(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the reservation's reply did not decode"))
    }

    /// 78.127 — A DOMAIN'S RESERVE as the court reads it.
    pub fn reserve_reading(&self, domain: [u8; 32]) -> io::Result<crate::anastome::dialogue::domain::ReserveReading> {
        use crate::anastome::dialogue::domain::{decode_reserve_reading, encode_reserve_read};
        let reply = self.speak(encode_reserve_read(domain))?;
        decode_reserve_reading(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the reserve reading did not decode"))
    }
}
