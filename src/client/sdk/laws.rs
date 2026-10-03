// 78.62 steps 2 and 3 — THE KERNEL'S SIDE OF THE LAW REGISTER: any kernel —
// a voice, a world, a game — asks its hypha for the universal laws, and puts
// a transition it composed to the court's verifier before it relies on it.
//
//! Both go through the raw act door (`Session::speak`) with the register's
//! own codec (`physics::laws::codec`), so the bytes a kernel sends are the
//! objects the court verifies. A reply that does not decode is an error by
//! name, never a default register or an admitted verdict.

use crate::physics::laws::codec::{decode_register_reply, encode_register_request, encode_verify_request, take_verdict, Served};
use crate::physics::laws::{Transition, Verdict};
use std::io;

impl super::Session {
    /// THE UNIVERSAL LAWS this hypha holds, each at its coordinate with its
    /// statement, at the register's version.
    pub fn laws(&self) -> io::Result<Vec<Served>> {
        let reply = self.speak(encode_register_request())?;
        decode_register_reply(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the register's reply did not decode"))
    }

    /// THE COURT'S VERDICT on a transition this kernel composed: every
    /// breach with its law. A reading — nothing is admitted by it.
    pub fn verify(&self, t: &Transition) -> io::Result<Verdict> {
        let reply = self.speak(encode_verify_request(t))?;
        let mut p = 0;
        match take_verdict(&reply, &mut p) {
            Some(v) if p == reply.len() => Ok(v),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "the verdict did not decode")),
        }
    }
}
