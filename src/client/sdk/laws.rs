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

    /// 78.124 — THE CORE CLOCK at this hypha: its head stamp `h:S:G:T:H:K.i`,
    /// or `None` before its first act. A kernel's step clock is held to it:
    /// each kernel tick stamped with the core stamp it last read.
    pub fn clock(&self) -> io::Result<Option<crate::chronometry::CoreStamp>> {
        let reply = self.speak(crate::physics::laws::codec::encode_clock_request())?;
        crate::physics::laws::codec::decode_clock_reply(&reply)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the clock's reply did not decode"))
    }

    /// 78.125 — THE REGISTER'S PARAMETERS — the challenge window among them —
    /// by name, at the law version they are read at.
    pub fn parameters(&self) -> io::Result<(u8, Vec<(String, u64)>)> {
        let reply = self.speak(crate::physics::laws::codec::encode_parameters_request())?;
        crate::physics::laws::codec::decode_parameters_reply(&reply)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the parameters' reply did not decode"))
    }

    /// 78.122 — THE SHARED LEXICON this hypha defines: its version and every
    /// term. A kernel compares the version with its own.
    pub fn lexicon(&self) -> io::Result<(u8, Vec<crate::physics::laws::codec::ServedTerm>)> {
        let reply = self.speak(crate::physics::laws::codec::encode_lexicon_request())?;
        crate::physics::laws::codec::decode_lexicon_reply(&reply)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the lexicon's reply did not decode"))
    }

    /// 78.120 — ADOMAIN FOUNDED ON THE COURT: the genesis receipt's stamp and
    /// chain hash where it is sealed, or the refusal by name.
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

    /// 78.129 — A TIER'S BOND posted (topped up) or withdrawn by its domain's
    /// reserve: sealed with the bond now, or the refusal by name.
    pub fn bond(&self, b: &crate::anastome::dialogue::domain::SignedBond) -> io::Result<crate::anastome::dialogue::domain::Founded> {
        use crate::anastome::dialogue::domain::{decode_domain_reply, encode_bond};
        let reply = self.speak(encode_bond(b))?;
        decode_domain_reply(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the bond's reply did not decode"))
    }

    /// 78.129 — A TIER'S BOND as read: its μ per lane (zero: none posted).
    pub fn bond_of(&self, domain: [u8; 32], tier: [u8; 32]) -> io::Result<[u64; 4]> {
        use crate::anastome::dialogue::domain::{decode_bond_reading, encode_bond_read};
        let reply = self.speak(encode_bond_read(domain, tier))?;
        decode_bond_reading(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the bond reading did not decode"))
    }

    /// 78.132 — A UNIVERSE'S ORIGIN RECORDED, once per domain: sealed, or
    /// the refusal by name (`AlreadyOriginated` — never an edit).
    pub fn originate(&self, o: &crate::anastome::dialogue::domain::SignedOrigin) -> io::Result<crate::anastome::dialogue::domain::Founded> {
        use crate::anastome::dialogue::domain::{decode_domain_reply, encode_origin};
        let reply = self.speak(encode_origin(o))?;
        decode_domain_reply(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the origin's reply did not decode"))
    }

    /// 78.132 — A DOMAIN'S ORIGIN AS HELD, with the stamp and receipt it was
    /// sealed under; `None` where it has none.
    pub fn origin(&self, domain: [u8; 32]) -> io::Result<Option<crate::anastome::dialogue::domain::OriginReading>> {
        use crate::anastome::dialogue::domain::{decode_origin_reading, encode_origin_read};
        let reply = self.speak(encode_origin_read(domain))?;
        decode_origin_reading(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the origin reading did not decode"))
    }

    /// 78.127 — A DOMAIN'S RESERVE AS READ: its tier, a pending
    /// redesignation, and the reserve's μ balance.
    pub fn reserve_reading(&self, domain: [u8; 32]) -> io::Result<crate::anastome::dialogue::domain::ReserveReading> {
        use crate::anastome::dialogue::domain::{decode_reserve_reading, encode_reserve_read};
        let reply = self.speak(encode_reserve_read(domain))?;
        decode_reserve_reading(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the reserve reading did not decode"))
    }

    /// 78.119 — A TIER'S FOLD PUT TO THE COURT for admission: the receipt's
    /// stamp and chain hash where it is sealed, or the refusal by name. A
    /// reply that does not read is an error by name, never an admission.
    pub fn uplink_fold(&self, f: &crate::anastome::dialogue::uplink::SignedFold) -> io::Result<crate::anastome::dialogue::uplink::Uplinked> {
        use crate::anastome::dialogue::uplink::{decode_uplink_reply, encode_uplink};
        let reply = self.speak(encode_uplink(f))?;
        decode_uplink_reply(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the fold's reply did not decode"))
    }

    /// 78.125 — A FOLD CHALLENGED within its window: a demand, a replay, a
    /// default or a correction, answered by the receipt or the refusal by
    /// name. A reply that does not read is an error by name.
    pub fn challenge(&self, c: &crate::anastome::dialogue::challenge::SignedChallenge) -> io::Result<crate::anastome::dialogue::challenge::Challenged> {
        use crate::anastome::dialogue::challenge::{decode_challenge_reply, encode_challenge};
        let reply = self.speak(encode_challenge(c))?;
        decode_challenge_reply(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the challenge's reply did not decode"))
    }

    /// 78.126 — A HANDOFF ACT: an offer, its acceptance or refusal, its
    /// expiry — answered by the receipt and the handoff's state, or the
    /// refusal by name.
    pub fn handoff(&self, h: &crate::anastome::dialogue::handoff::SignedHandoff) -> io::Result<crate::anastome::dialogue::handoff::Handed> {
        use crate::anastome::dialogue::handoff::{decode_handoff_reply, encode_handoff};
        let reply = self.speak(encode_handoff(h))?;
        decode_handoff_reply(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the handoff's reply did not decode"))
    }

    /// 78.126 — A HANDOFF'S STANDING: its state, and whether its OUT and IN
    /// sides are consumed; `None` where the court holds no such handoff.
    pub fn handoff_read(&self, id: [u8; 32]) -> io::Result<Option<(crate::anastome::dialogue::handoff::State, bool, bool)>> {
        use crate::anastome::dialogue::handoff::{decode_read_reply, encode_handoff_read};
        let reply = self.speak(encode_handoff_read(id))?;
        decode_read_reply(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the handoff read did not decode"))
    }

    /// 78.128 — THE MOUTH LOOPS: every open mouth pair that closed a cycle of
    /// domains whose holonomy is not the identity, with that holonomy —
    /// curvature recorded.
    pub fn mouth_loops(&self) -> io::Result<Vec<crate::anastome::dialogue::handoff::Loop>> {
        use crate::anastome::dialogue::handoff::{decode_loops_reply, encode_loops_read};
        let reply = self.speak(encode_loops_read())?;
        decode_loops_reply(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the loops reply did not decode"))
    }

    /// 78.125 — A FOLD'S FINALITY at the court's head, by its receipt's
    /// stamp; — 78.129 — whether its tier stands bonded; — 78.133 — whether
    /// a toll is held on it.
    pub fn finality(&self, fold: crate::chronometry::Stamp) -> io::Result<crate::anastome::dialogue::challenge::FinalityReading> {
        use crate::anastome::dialogue::challenge::{decode_finality_reading, encode_finality_request};
        let reply = self.speak(encode_finality_request(fold))?;
        decode_finality_reading(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the finality reply did not decode"))
    }

    /// 78.125 — A TIER'S LINE HEAD on the court: where its next fold's span
    /// is replayed from, the stamp of the fold that set it (none: the zero
    /// head), and — 78.133 — the μ held toward it in toll escrow. Resync by
    /// it before folding; a head that disagrees with the client's would be
    /// falsified on demand.
    pub fn line_head(&self, tier: [u8; 32]) -> io::Result<crate::anastome::dialogue::challenge::LineHead> {
        use crate::anastome::dialogue::challenge::{decode_line_head_reply, encode_line_head_request};
        let reply = self.speak(encode_line_head_request(tier))?;
        decode_line_head_reply(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the line head reply did not decode"))
    }

    /// 78.125 — A DOMAIN'S OPEN ARBITRAGES: what a correction names and prices.
    pub fn open_obstructions(&self, domain: [u8; 32]) -> io::Result<Vec<crate::anastome::dialogue::challenge::OpenObstruction>> {
        use crate::anastome::dialogue::challenge::{decode_obstructions_reply, encode_obstructions_request};
        let reply = self.speak(encode_obstructions_request(domain))?;
        decode_obstructions_reply(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the obstructions reply did not decode"))
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
