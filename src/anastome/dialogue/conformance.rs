// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-PRECURSORS §4 P5, MU-HOSTING §2.5 item 4 — THE CLIENT CONFORMANCE
// STATEMENT: every act a client speaks to a hypha, whether it MUST or MAY
// speak it, the reply it decodes and the exact fields it reads back.
//
//! The statement is data, kept here so a hypha serves the same object the
//! SDK publishes (`ACT_LAWS` mode 6, `physics::laws::codec`), and written in
//! exact bytes so a client in any language reads it whole. Conformance is by
//! decoded exact values, never golden bytes (Concordance 1.4): a client
//! conforms where every MUST act it sends decodes at the hypha to the values
//! it meant, and every reply decodes to the fields named here.

use super::{ACT_DOMAIN, ACT_LAWS, ACT_OBSERVE, ACT_PEERS, ACT_PROBE, ACT_QUOTE, ACT_RECEIPTS, ACT_ROWS, ACT_SEAT};
use super::{ACT_SOLVE, ACT_SPEECH, ACT_SYLLABUS, ACT_TASK, ACT_WALLET, ACT_WHY, WIRE_VERSION};
use crate::physics::laws::codec::{MODE_CLOCK, MODE_CONCORDANCE, MODE_LEXICON, MODE_PARAMETERS, MODE_REGISTER, MODE_SIGNATURE, MODE_STATEMENT};
use crate::physics::laws::concordance::CONCORDANCE_HASH;
use crate::physics::laws::LAW_VERSION;
use crate::transducer::address::{put_key_bytes, put_key_count, take_key_bytes, take_key_count};

/// WHETHER A CLIENT MUST SPEAK AN ACT, or may.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Need {
    /// Needed for the client loop: attach, take a task, solve, read the
    /// verdict, read the law version.
    Must,
    /// Spoken where the client chooses to.
    May,
}

impl Need {
    /// The need's byte on the wire.
    pub const fn byte(self) -> u8 {
        match self {
            Need::Must => 0,
            Need::May => 1,
        }
    }

    /// The need a byte names, or none.
    pub const fn of(b: u8) -> Option<Need> {
        match b {
            0 => Some(Need::Must),
            1 => Some(Need::May),
            _ => None,
        }
    }
}

/// ONE ACT A CLIENT SPEAKS: its code on the wire, its sub-mode where the act
/// has one, its name (the act's name in the SDK's `ACT_TABLE`, then the
/// sub-mode's where there is one), the need, the decoder its reply is read
/// by, and the exact fields that decoder reads back.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActRequirement {
    pub act: u8,
    pub mode: Option<u8>,
    pub name: &'static str,
    pub need: Need,
    pub reply: &'static str,
    pub fields: &'static [&'static str],
}

/// THE STATEMENT: the wire version, the law version and the Concordance's
/// hash a client pins to, and every act it speaks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Statement {
    pub wire_version: u16,
    pub law_version: u8,
    pub concordance_hash: [u8; 32],
    pub acts: &'static [ActRequirement],
}

impl Statement {
    /// The acts of one need, in the statement's order.
    pub fn of_need(&self, need: Need) -> impl Iterator<Item = &'static ActRequirement> {
        self.acts.iter().filter(move |a| a.need == need)
    }

    /// The act a name names, or none.
    pub fn act(&self, name: &str) -> Option<&'static ActRequirement> {
        self.acts.iter().find(|a| a.name == name)
    }
}

const fn must(act: u8, mode: Option<u8>, name: &'static str, reply: &'static str, fields: &'static [&'static str]) -> ActRequirement {
    ActRequirement { act, mode, name, need: Need::Must, reply, fields }
}

const fn may(act: u8, mode: Option<u8>, name: &'static str, reply: &'static str, fields: &'static [&'static str]) -> ActRequirement {
    ActRequirement { act, mode, name, need: Need::May, reply, fields }
}

/// THE CLIENT CONFORMANCE STATEMENT at this wire and law version: the MUST
/// acts of the client loop first, then every MAY act the reference mind and
/// the SDK's readings speak.
pub const STATEMENT: Statement = Statement {
    wire_version: WIRE_VERSION,
    law_version: LAW_VERSION,
    concordance_hash: CONCORDANCE_HASH,
    acts: &[
        must(ACT_TASK, None, "task", "decode_task_envelope", &["citation", "train", "tests"]),
        must(ACT_SOLVE, None, "solve", "decode_solve_verdict", &["code", "paid S:G:T:H", "refusal", "modes"]),
        must(ACT_LAWS, Some(MODE_REGISTER), "laws register", "decode_register_reply", &["law version", "laws"]),
        may(ACT_TASK, None, "task by citation", "decode_task_envelope", &["citation", "train", "tests"]),
        may(ACT_LAWS, Some(MODE_LEXICON), "laws lexicon", "decode_lexicon_reply", &["lexicon version", "terms"]),
        may(ACT_LAWS, Some(MODE_CLOCK), "laws clock", "decode_clock_reply", &["core stamp"]),
        may(ACT_LAWS, Some(MODE_PARAMETERS), "laws parameters", "decode_parameters_reply", &["law version", "parameters"]),
        may(ACT_LAWS, Some(MODE_STATEMENT), "laws statement", "decode_statement_reply", &["wire version", "law version", "concordance hash", "acts"]),
        may(ACT_LAWS, Some(MODE_CONCORDANCE), "laws concordance", "decode_concordance_reply", &["concordance hash"]),
        may(ACT_LAWS, Some(MODE_SIGNATURE), "laws signature", "decode_signature_reply", &["accepted, or the refusal by name"]),
        may(ACT_QUOTE, None, "quote", "decode_quote_reply", &["height", "surface S:G:T:H", "k", "work items"]),
        may(ACT_RECEIPTS, None, "receipts", "decode_receipts_reply", &["receipts"]),
        may(ACT_ROWS, None, "rows", "decode_rows_reply", &["height", "entries"]),
        may(ACT_OBSERVE, Some(crate::observe::MODE_QUERY), "observe query", "decode_reply", &["count", "held readings"]),
        may(ACT_WHY, None, "why", "decode_why_reply", &["stamp", "kind", "mu S:G:T:H", "named", "unheld", "refused"]),
        may(ACT_WALLET, None, "wallet", "decode_wallet_reply", &["court", "balance S:G:T:H", "bond S:G:T:H", "held S:G:T:H", "stamp"]),
        may(ACT_PROBE, None, "probe", "decode_probe_reply", &["settled S:G:T:H", "closure", "remaining S:G:T:H"]),
        may(ACT_PEERS, None, "peers", "decode_peers_reply", &["endpoints"]),
        may(ACT_SYLLABUS, None, "syllabus", "decode_syllabus_reply", &["taught"]),
        may(ACT_SPEECH, None, "speech", "decode_speech_reply", &["adjudications"]),
        may(ACT_DOMAIN, None, "domain reserve", "decode_reserve_reading", &["current tier", "pending tier", "balance S:G:T:H"]),
        may(ACT_SEAT, None, "seat mail", "decode_mail_reply_standing", &["epoch", "sequence", "mail"]),
    ],
};

/// ONE ACT AS A SERVED STATEMENT CARRIES IT, owned.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActRead {
    pub act: u8,
    pub mode: Option<u8>,
    pub name: String,
    pub need: Need,
    pub reply: String,
    pub fields: Vec<String>,
}

/// A STATEMENT READ OFF THE WIRE, owned.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatementRead {
    pub wire_version: u16,
    pub law_version: u8,
    pub concordance_hash: [u8; 32],
    pub acts: Vec<ActRead>,
}

impl StatementRead {
    /// The statement as a read of it would carry it: the comparison a served
    /// statement is held to.
    pub fn of(s: &Statement) -> StatementRead {
        let acts = s
            .acts
            .iter()
            .map(|a| ActRead {
                act: a.act,
                mode: a.mode,
                name: a.name.to_string(),
                need: a.need,
                reply: a.reply.to_string(),
                fields: a.fields.iter().map(|f| f.to_string()).collect(),
            })
            .collect();
        StatementRead { wire_version: s.wire_version, law_version: s.law_version, concordance_hash: s.concordance_hash, acts }
    }
}

/// THE STATEMENT'S BYTES: `[wire version u16 LE][law version][concordance
/// hash 32][count]` then per act `[act][0 | 1 mode][need][name][reply][field
/// count][field …]`, every text length-prefixed by its own coordinate.
pub fn encode_statement(s: &Statement) -> Vec<u8> {
    let mut out = s.wire_version.to_le_bytes().to_vec();
    out.push(s.law_version);
    out.extend_from_slice(&s.concordance_hash);
    put_key_count(&mut out, s.acts.len());
    for a in s.acts {
        out.push(a.act);
        match a.mode {
            None => out.push(0),
            Some(m) => out.extend_from_slice(&[1, m]),
        }
        out.push(a.need.byte());
        put_key_bytes(&mut out, a.name.as_bytes());
        put_key_bytes(&mut out, a.reply.as_bytes());
        put_key_count(&mut out, a.fields.len());
        a.fields.iter().for_each(|f| put_key_bytes(&mut out, f.as_bytes()));
    }
    out
}

/// The reply `ACT_LAWS` mode 6 answers: this tree's [`STATEMENT`].
pub fn encode_statement_reply() -> Vec<u8> {
    encode_statement(&STATEMENT)
}

fn take_u8(b: &[u8], p: &mut usize) -> Option<u8> {
    let v = *b.get(*p)?;
    *p += 1;
    Some(v)
}

fn take_text(b: &[u8], p: &mut usize) -> Option<String> {
    String::from_utf8(take_key_bytes(b, p)?).ok()
}

/// A statement read whole: a byte left over, a need no byte names, or a text
/// that is not UTF-8 is no statement.
pub fn decode_statement_reply(b: &[u8]) -> Option<StatementRead> {
    let wire_version = u16::from_le_bytes(b.get(0..2)?.try_into().ok()?);
    let mut p = 2;
    let law_version = take_u8(b, &mut p)?;
    let concordance_hash: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
    p += 32;
    let n = take_key_count(b, &mut p)?;
    let acts = (0..n)
        .map(|_| {
            let act = take_u8(b, &mut p)?;
            let mode = match take_u8(b, &mut p)? {
                0 => None,
                1 => Some(take_u8(b, &mut p)?),
                _ => return None,
            };
            let need = Need::of(take_u8(b, &mut p)?)?;
            let name = take_text(b, &mut p)?;
            let reply = take_text(b, &mut p)?;
            let m = take_key_count(b, &mut p)?;
            let fields = (0..m).map(|_| take_text(b, &mut p)).collect::<Option<Vec<_>>>()?;
            Some(ActRead { act, mode, name, need, reply, fields })
        })
        .collect::<Option<Vec<_>>>()?;
    (p == b.len()).then_some(StatementRead { wire_version, law_version, concordance_hash, acts })
}
