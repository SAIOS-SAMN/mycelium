// 78.62 steps 2 and 3 — THE KERNEL'S SIDE OF THE LAW REGISTER: any kernel —
// a voice, a domain, a game — asks its hypha for the universal laws, and puts
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

    /// 78.92 — WHY ONE ACT ON THE HYPHA'S LEDGER STANDS: the act and every
    /// word it names, each with the reasons read off the word. `None` where
    /// the hypha holds no act at the coordinates asked.
    pub fn why(&self, ask: crate::anastome::dialogue::why::WhyAsk) -> io::Result<Option<crate::anastome::dialogue::why::Why>> {
        let reply = self.speak(crate::anastome::dialogue::why::encode_why(ask))?;
        crate::anastome::dialogue::why::decode_why_reply(&reply)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the why reply did not decode"))
    }

    /// 78.93 B1 — `who`'S WALLET across every court this session knows: each
    /// court's account, and the courts that did not answer or answered bytes
    /// that do not read — counted, never read as empty accounts.
    pub fn wallet(&self, who: &[u8; 32]) -> crate::anastome::dialogue::wallet::Wallet {
        use crate::anastome::dialogue::wallet::{decode_wallet_reply, encode_wallet, Wallet};
        let mut w = Wallet { who: *who, ..Wallet::default() };
        for addr in self.peers() {
            match self.exchange_at(&addr, encode_wallet(who)).ok().and_then(|r| decode_wallet_reply(&r)) {
                Some(a) => w.accounts.push(a),
                None => w.unanswered += 1,
            }
        }
        w
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

    /// 78.120 — A DOMAIN FOUNDED ON THE COURT: the genesis receipt's stamp and
    /// chain hash where it is sealed, or the refusal by name.
    pub fn found_domain(&self, w: &crate::anastome::dialogue::domain::SignedDomain) -> io::Result<crate::anastome::dialogue::domain::Founded> {
        use crate::anastome::dialogue::domain::{decode_domain_reply, encode_domain};
        let reply = self.speak(encode_domain(w))?;
        decode_domain_reply(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the domain's reply did not decode"))
    }

    /// 78.119 — A TIER'S FOLD PUT TO THE COURT for admission: the receipt's
    /// stamp and chain hash where it is sealed, or the refusal by name. A
    /// reply that does not read is an error by name, never an admission.
    pub fn uplink_fold(&self, f: &crate::anastome::dialogue::uplink::SignedFold) -> io::Result<crate::anastome::dialogue::uplink::Uplinked> {
        use crate::anastome::dialogue::uplink::{decode_uplink_reply, encode_uplink};
        let reply = self.speak(encode_uplink(f))?;
        decode_uplink_reply(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the fold's reply did not decode"))
    }

    /// 78.118 — THE COURT'S VERDICTS on many transitions in one frame, one
    /// per transition in the order sent. A reading — nothing is admitted. A
    /// reply whose count is not the batch's is an error by name.
    pub fn verify_batch(&self, ts: &[Transition]) -> io::Result<Vec<Verdict>> {
        let reply = self.speak(crate::physics::laws::codec::encode_batch_request(ts))?;
        match crate::physics::laws::codec::decode_batch_reply(&reply) {
            Some(vs) if vs.len() == ts.len() => Ok(vs),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "the batch's verdicts did not decode one per transition")),
        }
    }
}
