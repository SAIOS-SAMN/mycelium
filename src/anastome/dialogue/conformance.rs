// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-PRECURSORS §4 P5, MU-HOSTING §2.5 item 4 — THE CLIENT CONFORMANCE
// STATEMENT: every act a client speaks to a hypha, per profile (a mind, or a
// kernel's uplink), whether it MUST or MAY speak it, the reply it decodes and
// the exact fields it reads back.
//
//! The statement is data, kept here so a hypha serves the same object the
//! SDK publishes (`ACT_LAWS` mode 6, `physics::laws::codec`), and written in
//! exact bytes so a client in any language reads it whole. Conformance is by
//! decoded exact values, never golden bytes (Concordance 1.4): a client
//! conforms where every MUST act of the profiles it declares decodes at the
//! hypha to the values it meant, and every reply decodes to the fields named
//! here.

use super::{ACT_DOMAIN, ACT_LAWS, ACT_OBSERVE, ACT_PEERS, ACT_PROBE, ACT_QUOTE, ACT_RECEIPTS, ACT_ROWS, ACT_SEAT};
use super::{ACT_SOLVE, ACT_SPEECH, ACT_SYLLABUS, ACT_TASK, ACT_WALLET, ACT_WHY, WIRE_VERSION};
use crate::physics::laws::codec::{MODE_BATCH, MODE_CLOCK, MODE_CONCORDANCE, MODE_LEXICON, MODE_PARAMETERS, MODE_REGISTER};
use crate::physics::laws::codec::{MODE_SIGNATURE, MODE_STATEMENT, MODE_VERIFY};
use crate::physics::laws::concordance::CONCORDANCE_HASH;
use crate::physics::laws::LAW_VERSION;
use crate::transducer::address::{put_key_bytes, put_key_count, take_key_bytes, take_key_count};

/// WHAT KIND OF CLIENT a profile of the statement is for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Profile {
    /// A mind: the client loop of task, solve and the law register.
    Mind,
    /// A kernel's uplink (its house): the law register and its signature,
    /// verified transitions, a domain founded, its universe read and
    /// endowed. It speaks task and solve only for the minds it seats.
    Kernel,
}

impl Profile {
    /// Every profile, in the statement's order.
    pub const ALL: [Profile; 2] = [Profile::Mind, Profile::Kernel];

    /// The profile's byte on the wire.
    pub const fn byte(self) -> u8 {
        match self {
            Profile::Mind => 0,
            Profile::Kernel => 1,
        }
    }

    /// The profile a byte names, or none.
    pub const fn of(b: u8) -> Option<Profile> {
        match b {
            0 => Some(Profile::Mind),
            1 => Some(Profile::Kernel),
            _ => None,
        }
    }

    /// The profile's name, as a report prints it.
    pub const fn name(self) -> &'static str {
        match self {
            Profile::Mind => "mind",
            Profile::Kernel => "kernel",
        }
    }
}

/// WHETHER A CLIENT MUST SPEAK AN ACT, or may.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Need {
    /// Needed for the profile's loop: for a mind, take a task, solve, read
    /// the verdict, read the law version; for a kernel, the acts of its
    /// uplink.
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

/// ONE PROFILE OF THE STATEMENT: the kind of client, and every act it
/// speaks, each named once.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProfileStatement {
    pub profile: Profile,
    pub acts: &'static [ActRequirement],
}

/// THE STATEMENT: the wire version, the law version and the Concordance's
/// hash a client pins to, and each profile's acts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Statement {
    pub wire_version: u16,
    pub law_version: u8,
    pub concordance_hash: [u8; 32],
    pub profiles: &'static [ProfileStatement],
}

impl Statement {
    /// The acts of one profile, in the statement's order (none where the
    /// statement carries no such profile).
    pub fn acts(&self, profile: Profile) -> &'static [ActRequirement] {
        match self.profiles.iter().find(|p| p.profile == profile) {
            Some(p) => p.acts,
            None => &[],
        }
    }

    /// The acts of one need in one profile, in the statement's order.
    pub fn of_need(&self, profile: Profile, need: Need) -> impl Iterator<Item = &'static ActRequirement> {
        self.acts(profile).iter().filter(move |a| a.need == need)
    }

    /// The act a name names in one profile, or none.
    pub fn act(&self, profile: Profile, name: &str) -> Option<&'static ActRequirement> {
        self.acts(profile).iter().find(|a| a.name == name)
    }

    /// THE ACTS OF A CLIENT DECLARING `profiles`: every act any of them
    /// names, once, in the order first named — MUST where any declared
    /// profile makes it MUST, MAY otherwise.
    pub fn union(&self, profiles: &[Profile]) -> Vec<ActRequirement> {
        let mut out: Vec<ActRequirement> = Vec::new();
        for a in profiles.iter().flat_map(|p| self.acts(*p)) {
            match out.iter_mut().find(|held| held.name == a.name) {
                Some(held) if a.need == Need::Must => held.need = Need::Must,
                Some(_) => {}
                None => out.push(*a),
            }
        }
        out
    }
}

const fn must(act: u8, mode: Option<u8>, name: &'static str, reply: &'static str, fields: &'static [&'static str]) -> ActRequirement {
    ActRequirement { act, mode, name, need: Need::Must, reply, fields }
}

const fn may(act: u8, mode: Option<u8>, name: &'static str, reply: &'static str, fields: &'static [&'static str]) -> ActRequirement {
    ActRequirement { act, mode, name, need: Need::May, reply, fields }
}

const TASK: &[&str] = &["citation", "train", "tests"];
const SOLVE: &[&str] = &["code", "paid S:G:T:H", "refusal", "modes"];
const REGISTER: &[&str] = &["law version", "laws"];
const LEXICON: &[&str] = &["lexicon version", "terms"];
const CLOCK: &[&str] = &["core stamp"];
const PARAMETERS: &[&str] = &["law version", "parameters"];
const STATED: &[&str] = &["wire version", "law version", "concordance hash", "profiles", "acts"];
const HASH: &[&str] = &["concordance hash"];
const SIGNATURE: &[&str] = &["accepted, or the refusal by name"];
const RESERVE: &[&str] = &["current tier", "pending tier", "balance S:G:T:H"];
const MAIL: &[&str] = &["epoch", "sequence", "mail"];
const WALLET: &[&str] = &["court", "balance S:G:T:H", "bond S:G:T:H", "held S:G:T:H", "stamp"];
const VERDICT: &[&str] = &["breaches by law", "exchanged torques", "modes", "similarity", "expansion", "unmeasured laws"];
const FOUNDED: &[&str] = &["sealed stamp and receipt, or the refusal by name"];

/// THE MIND PROFILE: the MUST acts of the client loop first, then every MAY
/// act the reference mind and the SDK's readings speak.
const MIND: &[ActRequirement] = &[
    must(ACT_TASK, None, "task", "decode_task_envelope", TASK),
    must(ACT_SOLVE, None, "solve", "decode_solve_verdict", SOLVE),
    must(ACT_LAWS, Some(MODE_REGISTER), "laws register", "decode_register_reply", REGISTER),
    may(ACT_TASK, None, "task by citation", "decode_task_envelope", TASK),
    may(ACT_LAWS, Some(MODE_LEXICON), "laws lexicon", "decode_lexicon_reply", LEXICON),
    may(ACT_LAWS, Some(MODE_CLOCK), "laws clock", "decode_clock_reply", CLOCK),
    may(ACT_LAWS, Some(MODE_PARAMETERS), "laws parameters", "decode_parameters_reply", PARAMETERS),
    may(ACT_LAWS, Some(MODE_STATEMENT), "laws statement", "decode_statement_reply", STATED),
    may(ACT_LAWS, Some(MODE_CONCORDANCE), "laws concordance", "decode_concordance_reply", HASH),
    may(ACT_LAWS, Some(MODE_SIGNATURE), "laws signature", "decode_signature_reply", SIGNATURE),
    may(ACT_QUOTE, None, "quote", "decode_quote_reply", &["height", "surface S:G:T:H", "k", "work items"]),
    may(ACT_RECEIPTS, None, "receipts", "decode_receipts_reply", &["receipts"]),
    may(ACT_ROWS, None, "rows", "decode_rows_reply", &["height", "entries"]),
    may(ACT_OBSERVE, Some(crate::observe::MODE_QUERY), "observe query", "decode_reply", &["count", "held readings"]),
    may(ACT_WHY, None, "why", "decode_why_reply", &["stamp", "kind", "mu S:G:T:H", "named", "unheld", "refused"]),
    may(ACT_WALLET, None, "wallet", "decode_wallet_reply", WALLET),
    may(ACT_PROBE, None, "probe", "decode_probe_reply", &["settled S:G:T:H", "closure", "remaining S:G:T:H"]),
    may(ACT_PEERS, None, "peers", "decode_peers_reply", &["endpoints"]),
    may(ACT_SYLLABUS, None, "syllabus", "decode_syllabus_reply", &["taught"]),
    may(ACT_SPEECH, None, "speech", "decode_speech_reply", &["adjudications"]),
    may(ACT_DOMAIN, None, "domain reserve", "decode_reserve_reading", RESERVE),
    may(ACT_SEAT, None, "seat mail", "decode_mail_reply_standing", MAIL),
];

/// THE KERNEL PROFILE: the acts of a kernel's uplink — the register read and
/// signed, transitions verified, the clock, a domain founded, its universe
/// endowed and read — then every MAY act a house speaks, task and solve
/// among them for the minds it seats.
const KERNEL: &[ActRequirement] = &[
    must(ACT_LAWS, Some(MODE_REGISTER), "laws register", "decode_register_reply", REGISTER),
    must(ACT_LAWS, Some(MODE_STATEMENT), "laws statement", "decode_statement_reply", STATED),
    must(ACT_LAWS, Some(MODE_CONCORDANCE), "laws concordance", "decode_concordance_reply", HASH),
    must(ACT_LAWS, Some(MODE_SIGNATURE), "laws signature", "decode_signature_reply", SIGNATURE),
    must(ACT_LAWS, Some(MODE_CLOCK), "laws clock", "decode_clock_reply", CLOCK),
    must(ACT_LAWS, Some(MODE_VERIFY), "laws verify", "decode_verify_reply", VERDICT),
    must(ACT_DOMAIN, None, "domain founding", "decode_domain_reply", FOUNDED),
    must(ACT_DOMAIN, None, "domain manifest", "decode_grant_reply", &["granted μ S:G:T:H, founding, stamp and receipt, or the refusal by name"]),
    must(ACT_DOMAIN, None, "domain universe", "decode_universe_reading", &["originated", "geometry", "parent", "basket"]),
    may(ACT_TASK, None, "task", "decode_task_envelope", TASK),
    may(ACT_SOLVE, None, "solve", "decode_solve_verdict", SOLVE),
    may(ACT_TASK, None, "task by citation", "decode_task_envelope", TASK),
    may(ACT_LAWS, Some(MODE_BATCH), "laws batch", "decode_batch_reply", &["verdicts, one per transition in order"]),
    may(ACT_LAWS, Some(MODE_LEXICON), "laws lexicon", "decode_lexicon_reply", LEXICON),
    may(ACT_LAWS, Some(MODE_PARAMETERS), "laws parameters", "decode_parameters_reply", PARAMETERS),
    may(ACT_DOMAIN, None, "domain reserve", "decode_reserve_reading", RESERVE),
    may(ACT_DOMAIN, None, "domain designation", "decode_domain_reply", FOUNDED),
    may(ACT_DOMAIN, None, "domain origin", "decode_domain_reply", FOUNDED),
    may(ACT_DOMAIN, None, "domain origin reading", "decode_origin_reading", &["kind", "version", "commitment", "stamp", "receipt"]),
    may(ACT_DOMAIN, None, "domain statement", "decode_domain_reply", FOUNDED),
    may(ACT_DOMAIN, None, "domain endowment", "decode_endowment_reading", &["earned S:G:T:H", "founding S:G:T:H", "growth S:G:T:H", "grants", "nuclei"]),
    may(ACT_SEAT, None, "seat post", "decode_post_reply", &["sequence, or the refusal by name"]),
    may(ACT_SEAT, None, "seat listing", "decode_listing_reply", &["the latest seats post, or none"]),
    may(ACT_SEAT, None, "seat mail", "decode_mail_reply_standing", MAIL),
    may(ACT_WALLET, None, "wallet", "decode_wallet_reply", WALLET),
];

/// THE CLIENT CONFORMANCE STATEMENT at this wire and law version: the mind
/// profile, then the kernel profile.
pub const STATEMENT: Statement = Statement {
    wire_version: WIRE_VERSION,
    law_version: LAW_VERSION,
    concordance_hash: CONCORDANCE_HASH,
    profiles: &[ProfileStatement { profile: Profile::Mind, acts: MIND }, ProfileStatement { profile: Profile::Kernel, acts: KERNEL }],
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

/// ONE PROFILE AS A SERVED STATEMENT CARRIES IT, owned.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileRead {
    pub profile: Profile,
    pub acts: Vec<ActRead>,
}

/// A STATEMENT READ OFF THE WIRE, owned.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatementRead {
    pub wire_version: u16,
    pub law_version: u8,
    pub concordance_hash: [u8; 32],
    pub profiles: Vec<ProfileRead>,
}

impl StatementRead {
    /// The statement as a read of it would carry it: the comparison a served
    /// statement is held to.
    pub fn of(s: &Statement) -> StatementRead {
        let read = |a: &ActRequirement| ActRead {
            act: a.act,
            mode: a.mode,
            name: a.name.to_string(),
            need: a.need,
            reply: a.reply.to_string(),
            fields: a.fields.iter().map(|f| f.to_string()).collect(),
        };
        let profiles = s.profiles.iter().map(|p| ProfileRead { profile: p.profile, acts: p.acts.iter().map(read).collect() }).collect();
        StatementRead { wire_version: s.wire_version, law_version: s.law_version, concordance_hash: s.concordance_hash, profiles }
    }

    /// The acts of one profile as read (none where the read carries no such
    /// profile).
    pub fn acts(&self, profile: Profile) -> &[ActRead] {
        match self.profiles.iter().find(|p| p.profile == profile) {
            Some(p) => &p.acts,
            None => &[],
        }
    }
}

/// THE PROFILED FORM'S MARK, where the single-profile form of v0.3.0-rc.1
/// wrote its act count: a zero count, then this byte. That form wrote a zero
/// count only as its last byte (a statement of no acts), so the two forms
/// never read alike, and an rc.1 reader refuses a profiled statement whole.
pub const FORM_PROFILES: u8 = 2;

/// Every act of one profile: `[count]` then per act `[act][0 | 1 mode][need]
/// [name][reply][field count][field …]`.
pub(crate) fn put_acts(out: &mut Vec<u8>, acts: &[ActRequirement]) {
    put_key_count(out, acts.len());
    for a in acts {
        out.push(a.act);
        match a.mode {
            None => out.push(0),
            Some(m) => out.extend_from_slice(&[1, m]),
        }
        out.push(a.need.byte());
        put_key_bytes(out, a.name.as_bytes());
        put_key_bytes(out, a.reply.as_bytes());
        put_key_count(out, a.fields.len());
        a.fields.iter().for_each(|f| put_key_bytes(out, f.as_bytes()));
    }
}

/// THE STATEMENT'S BYTES: `[wire version u16 LE][law version][concordance
/// hash 32][0][FORM_PROFILES][profile count]` then per profile `[profile]
/// [act count]` and its acts as [`put_acts`] writes them, every text
/// length-prefixed by its own coordinate.
pub fn encode_statement(s: &Statement) -> Vec<u8> {
    let mut out = s.wire_version.to_le_bytes().to_vec();
    out.push(s.law_version);
    out.extend_from_slice(&s.concordance_hash);
    out.extend_from_slice(&[0, FORM_PROFILES]);
    put_key_count(&mut out, s.profiles.len());
    for p in s.profiles {
        out.push(p.profile.byte());
        put_acts(&mut out, p.acts);
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

fn take_acts(b: &[u8], p: &mut usize) -> Option<Vec<ActRead>> {
    let n = take_key_count(b, p)?;
    (0..n)
        .map(|_| {
            let act = take_u8(b, p)?;
            let mode = match take_u8(b, p)? {
                0 => None,
                1 => Some(take_u8(b, p)?),
                _ => return None,
            };
            let need = Need::of(take_u8(b, p)?)?;
            let name = take_text(b, p)?;
            let reply = take_text(b, p)?;
            let m = take_key_count(b, p)?;
            let fields = (0..m).map(|_| take_text(b, p)).collect::<Option<Vec<_>>>()?;
            Some(ActRead { act, mode, name, need, reply, fields })
        })
        .collect()
}

/// A statement read whole: a byte left over, a profile or need no byte
/// names, a form mark other than [`FORM_PROFILES`], or a text that is not
/// UTF-8 is no statement. The single-profile form v0.3.0-rc.1 served (its act
/// count where the mark stands) reads as the mind profile alone.
pub fn decode_statement_reply(b: &[u8]) -> Option<StatementRead> {
    let wire_version = u16::from_le_bytes(b.get(0..2)?.try_into().ok()?);
    let mut p = 2;
    let law_version = take_u8(b, &mut p)?;
    let concordance_hash: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
    p += 32;
    let profiles = match b.get(p..) {
        Some([0, form, ..]) => {
            if *form != FORM_PROFILES {
                return None;
            }
            p += 2;
            let n = take_key_count(b, &mut p)?;
            (0..n)
                .map(|_| {
                    let profile = Profile::of(take_u8(b, &mut p)?)?;
                    Some(ProfileRead { profile, acts: take_acts(b, &mut p)? })
                })
                .collect::<Option<Vec<_>>>()?
        }
        _ => vec![ProfileRead { profile: Profile::Mind, acts: take_acts(b, &mut p)? }],
    };
    (p == b.len()).then_some(StatementRead { wire_version, law_version, concordance_hash, profiles })
}
