// 78.62 step 2, 78.64 H4 — THE WIRE CONTRACT'S FIRST SLICE: the law
// register's own objects in exact bytes, so what a kernel sends and what a
// court answers are the same objects on both ends. Every rational and every
// multivector through the one codec of each (`put_q`, `Mv::to_bytes`: the
// length intrinsic to the precision, LEB128, never a widened width); every
// count by its own coordinate (`put_key_count`). A decoder that does not
// find what it needs answers `None`, never a default.
//
//! Laid in the crate first so the crate extraction (H4) moves it whole; the
//! law's coordinate is its address (`Law::address`), read back by
//! [`law_at`].

use super::{BodyState, Breach, Flow, Flux, Law, Transition, Verdict, LAWS, LAW_VERSION};
use crate::physics::tensegrity::Body;
use crate::transducer::address::{put_key_count, take_key_count, Address, TAG_LAW};
use crate::transducer::alphabet::{put_q, take_q};
use crate::transducer::cga::{Mv, Undecomposed};
use num_rational::BigRational;

type Q = BigRational;

/// THE LAW AT A COORDINATE: `TAG_LAW ‖ index ‖ version`, at this register's
/// version. `None` for another tag, an index the register does not hold, or
/// another version — a law read at a version is that version's law.
pub fn law_at(a: &Address) -> Option<Law> {
    match a.as_bytes() {
        [TAG_LAW, index, version] if *version == LAW_VERSION => LAWS.get(usize::from(*index)).copied(),
        _ => None,
    }
}

fn put_u64(out: &mut Vec<u8>, v: u64) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn take_u64(b: &[u8], p: &mut usize) -> Option<u64> {
    let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    Some(v)
}

fn take_u8(b: &[u8], p: &mut usize) -> Option<u8> {
    let v = *b.get(*p)?;
    *p += 1;
    Some(v)
}

fn take_mv(b: &[u8], p: &mut usize) -> Option<Mv> {
    Mv::from_bytes(b, p)
}

fn put_qs(out: &mut Vec<u8>, qs: &[Q]) {
    put_key_count(out, qs.len());
    qs.iter().for_each(|q| put_q(out, q));
}

fn take_qs(b: &[u8], p: &mut usize) -> Option<Vec<Q>> {
    let n = take_key_count(b, p)?;
    (0..n).map(|_| take_q(b, p)).collect()
}

/// A BODY AS A.13 READS IT: its centre, its mass, its second moments.
pub fn put_body(out: &mut Vec<u8>, body: &Body) {
    put_qs(out, &body.centre);
    put_q(out, &body.mass);
    put_key_count(out, body.moments.len());
    body.moments.iter().for_each(|row| put_qs(out, row));
}

pub fn take_body(b: &[u8], p: &mut usize) -> Option<Body> {
    let centre = take_qs(b, p)?;
    let mass = take_q(b, p)?;
    let n = take_key_count(b, p)?;
    let moments = (0..n).map(|_| take_qs(b, p)).collect::<Option<Vec<_>>>()?;
    Some(Body { centre, mass, moments })
}

fn put_state(out: &mut Vec<u8>, s: &BodyState) {
    s.nucleus.to_bytes(out);
    put_body(out, &s.body);
}

fn take_state(b: &[u8], p: &mut usize) -> Option<BodyState> {
    Some(BodyState { nucleus: take_mv(b, p)?, body: take_body(b, p)? })
}

/// ONE TRANSITION, as an uplinked kernel sends it: the motion, the heights,
/// every body before and after, the hue relation.
pub fn put_transition(out: &mut Vec<u8>, t: &Transition) {
    t.motion.to_bytes(out);
    put_u64(out, t.heights.0);
    put_u64(out, t.heights.1);
    put_key_count(out, t.bodies.len());
    for (before, after) in &t.bodies {
        put_state(out, before);
        put_state(out, after);
    }
    put_key_count(out, t.hues.len());
    for (f, h) in &t.hues {
        out.push(*f);
        out.push(*h);
    }
    put_key_count(out, t.fluxes.len());
    for f in &t.fluxes {
        out.push(match f.flow {
            Flow::Source => 0,
            Flow::Sink => 1,
        });
        f.at.to_bytes(out);
        put_q(out, &f.mass);
    }
}

pub fn take_transition(b: &[u8], p: &mut usize) -> Option<Transition> {
    let motion = take_mv(b, p)?;
    let heights = (take_u64(b, p)?, take_u64(b, p)?);
    let n = take_key_count(b, p)?;
    let bodies = (0..n).map(|_| Some((take_state(b, p)?, take_state(b, p)?))).collect::<Option<Vec<_>>>()?;
    let m = take_key_count(b, p)?;
    let hues = (0..m).map(|_| Some((take_u8(b, p)?, take_u8(b, p)?))).collect::<Option<Vec<_>>>()?;
    let f = take_key_count(b, p)?;
    let fluxes = (0..f)
        .map(|_| {
            let flow = match take_u8(b, p)? {
                0 => Flow::Source,
                1 => Flow::Sink,
                _ => return None,
            };
            Some(Flux { flow, at: take_mv(b, p)?, mass: take_q(b, p)? })
        })
        .collect::<Option<Vec<_>>>()?;
    Some(Transition { motion, heights, bodies, hues, fluxes })
}

fn put_breach(out: &mut Vec<u8>, breach: &Breach) {
    match breach {
        Breach::NotAVersor => out.push(0),
        Breach::Undecomposed(why) => {
            out.push(1);
            match why {
                Undecomposed::Irrational(d) => {
                    out.push(0);
                    put_key_count(out, *d);
                }
                Undecomposed::Defective => out.push(1),
                Undecomposed::Unbounded(d) => {
                    out.push(2);
                    put_key_count(out, *d);
                }
            }
        }
        Breach::NotCarried { body } => {
            out.push(2);
            put_key_count(out, *body);
        }
        Breach::MomentumUnread { body } => {
            out.push(3);
            put_key_count(out, *body);
        }
        Breach::MomentumNotConserved { net } => {
            out.push(4);
            net.to_bytes(out);
        }
        Breach::NonRigid { i, j, delta } => {
            out.push(5);
            put_key_count(out, *i);
            put_key_count(out, *j);
            put_q(out, delta);
        }
        Breach::RigidityUnmeasured { i, j } => {
            out.push(6);
            put_key_count(out, *i);
            put_key_count(out, *j);
        }
        Breach::HeightNotAdvanced { before, after } => {
            out.push(7);
            put_u64(out, *before);
            put_u64(out, *after);
        }
        Breach::NotUnitary => out.push(8),
        Breach::MassNotConserved { made } => {
            out.push(9);
            put_q(out, made);
        }
        Breach::FluxUnplaced { index } => {
            out.push(10);
            put_key_count(out, *index);
        }
    }
}

fn take_breach(b: &[u8], p: &mut usize) -> Option<Breach> {
    Some(match take_u8(b, p)? {
        0 => Breach::NotAVersor,
        1 => Breach::Undecomposed(match take_u8(b, p)? {
            0 => Undecomposed::Irrational(take_key_count(b, p)?),
            1 => Undecomposed::Defective,
            2 => Undecomposed::Unbounded(take_key_count(b, p)?),
            _ => return None,
        }),
        2 => Breach::NotCarried { body: take_key_count(b, p)? },
        3 => Breach::MomentumUnread { body: take_key_count(b, p)? },
        4 => Breach::MomentumNotConserved { net: take_mv(b, p)? },
        5 => Breach::NonRigid { i: take_key_count(b, p)?, j: take_key_count(b, p)?, delta: take_q(b, p)? },
        6 => Breach::RigidityUnmeasured { i: take_key_count(b, p)?, j: take_key_count(b, p)? },
        7 => Breach::HeightNotAdvanced { before: take_u64(b, p)?, after: take_u64(b, p)? },
        8 => Breach::NotUnitary,
        9 => Breach::MassNotConserved { made: take_q(b, p)? },
        10 => Breach::FluxUnplaced { index: take_key_count(b, p)? },
        _ => return None,
    })
}

/// THE COURT'S ANSWER: every breach with its law (by the law's coordinate in
/// the register), the torques exchanged, the modes, the similarity.
pub fn put_verdict(out: &mut Vec<u8>, v: &Verdict) {
    put_key_count(out, v.breaches.len());
    for (law, breach) in &v.breaches {
        out.push(law.index());
        put_breach(out, breach);
    }
    put_key_count(out, v.exchanged.len());
    for (i, tau) in &v.exchanged {
        put_key_count(out, *i);
        tau.to_bytes(out);
    }
    put_key_count(out, v.modes);
    for ratio in [&v.similarity, &v.expansion] {
        match ratio {
            None => out.push(0),
            Some(r) => {
                out.push(1);
                put_q(out, r);
            }
        }
    }
    // 78.121 — every law unread, with the stroke it needs.
    put_key_count(out, v.unmeasured.len());
    for u in &v.unmeasured {
        out.push(u.law.index());
        match u.needed_stroke {
            None => out.push(0),
            Some(n) => {
                out.push(1);
                out.extend_from_slice(&n.to_le_bytes());
            }
        }
    }
}

fn take_ratio(b: &[u8], p: &mut usize) -> Option<Option<Q>> {
    match take_u8(b, p)? {
        0 => Some(None),
        1 => Some(Some(take_q(b, p)?)),
        _ => None,
    }
}

/// WHAT A KERNEL ASKS THE REGISTER (`ACT_LAWS`): the laws, or a verdict on a
/// transition it composed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LawsRequest {
    Register,
    Verify(Transition),
    /// 78.118 — many transitions in one frame, a verdict for each in order.
    Batch(Vec<Transition>),
    /// 78.122 — the shared lexicon's register, at its version.
    Lexicon,
    /// 78.124 — the core clock: the court's head stamp.
    Clock,
    /// 78.125 — the register's parameters, at its version.
    Parameters,
}

const MODE_LEXICON: u8 = 3;
const MODE_CLOCK: u8 = 4;
const MODE_PARAMETERS: u8 = 5;

/// 78.125 — `[ACT_LAWS][5]`: the register's parameters, asked by address.
pub fn encode_parameters_request() -> Vec<u8> {
    vec![crate::anastome::dialogue::ACT_LAWS, MODE_PARAMETERS]
}

/// The parameters' reply: `[law version][count][(name, value u64 LE) …]`.
pub fn encode_parameters_reply() -> Vec<u8> {
    let mut out = vec![LAW_VERSION];
    put_key_count(&mut out, super::PARAMETERS.len());
    for (name, value) in super::PARAMETERS {
        crate::transducer::address::put_key_bytes(&mut out, name.as_bytes());
        out.extend_from_slice(&value.to_le_bytes());
    }
    out
}

/// The parameters a reply carries — the law version and each by name — read
/// whole.
pub fn decode_parameters_reply(b: &[u8]) -> Option<(u8, Vec<(String, u64)>)> {
    let mut p = 0;
    let version = take_u8(b, &mut p)?;
    let n = take_key_count(b, &mut p)?;
    let params = (0..n)
        .map(|_| {
            let name = String::from_utf8(crate::transducer::address::take_key_bytes(b, &mut p)?).ok()?;
            let value = u64::from_le_bytes(b.get(p..p + 8)?.try_into().ok()?);
            p += 8;
            Some((name, value))
        })
        .collect::<Option<Vec<_>>>()?;
    (p == b.len()).then_some((version, params))
}

/// 78.124 — `[ACT_LAWS][4]`: the core clock, asked by address.
pub fn encode_clock_request() -> Vec<u8> {
    vec![crate::anastome::dialogue::ACT_LAWS, MODE_CLOCK]
}

/// The clock's reply: `[0]` — no act yet, no head — or `[1]` and the head's
/// stamp, seven `u64`s.
pub fn encode_clock_reply(at: Option<&crate::chronometry::CoreStamp>) -> Vec<u8> {
    match at {
        None => vec![0],
        Some(s) => {
            let mut out = vec![1];
            s.put(&mut out);
            out
        }
    }
}

/// The clock a reply carries, read whole: `Some(None)` a court with no head
/// yet, `None` bytes that do not read.
pub fn decode_clock_reply(b: &[u8]) -> Option<Option<crate::chronometry::CoreStamp>> {
    let mut p = 1;
    let read = match b {
        [0] => return Some(None),
        [1, ..] => crate::chronometry::CoreStamp::take(b, &mut p)?,
        _ => return None,
    };
    (p == b.len()).then_some(Some(read))
}

/// 78.122 — `[ACT_LAWS][3]`: the shared lexicon, asked by address.
pub fn encode_lexicon_request() -> Vec<u8> {
    vec![crate::anastome::dialogue::ACT_LAWS, MODE_LEXICON]
}

/// The lexicon's reply: `[version][count][(vocabulary, name, meaning) …]`.
pub fn encode_lexicon_reply() -> Vec<u8> {
    use super::lexicon::{LEXICON_VERSION, TERMS};
    let mut out = vec![LEXICON_VERSION];
    put_key_count(&mut out, TERMS.len());
    for term in TERMS {
        out.push(term.vocabulary.byte());
        crate::transducer::address::put_key_bytes(&mut out, term.name.as_bytes());
        crate::transducer::address::put_key_bytes(&mut out, term.meaning.as_bytes());
    }
    out
}

/// ONE TERM AS THE REGISTER SERVES IT.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServedTerm {
    pub name: String,
    pub vocabulary: super::lexicon::Vocabulary,
    pub meaning: String,
}

/// The lexicon a reply carries — its version and every term — read whole.
/// A kernel compares the version with its own: another version is told so,
/// never read as this one.
pub fn decode_lexicon_reply(b: &[u8]) -> Option<(u8, Vec<ServedTerm>)> {
    let mut p = 0;
    let version = take_u8(b, &mut p)?;
    let n = take_key_count(b, &mut p)?;
    let terms = (0..n)
        .map(|_| {
            let vocabulary = super::lexicon::Vocabulary::of(take_u8(b, &mut p)?)?;
            let name = String::from_utf8(crate::transducer::address::take_key_bytes(b, &mut p)?).ok()?;
            let meaning = String::from_utf8(crate::transducer::address::take_key_bytes(b, &mut p)?).ok()?;
            Some(ServedTerm { name, vocabulary, meaning })
        })
        .collect::<Option<Vec<_>>>()?;
    (p == b.len()).then_some((version, terms))
}

const MODE_REGISTER: u8 = 0;
const MODE_VERIFY: u8 = 1;
const MODE_BATCH: u8 = 2;

/// 78.118 — `[ACT_LAWS][2][count][transition …]`: many transitions put to the
/// laws in one frame (a domain's tiers batching up), each answered in the
/// order it was sent.
pub fn encode_batch_request(ts: &[Transition]) -> Vec<u8> {
    let mut out = vec![crate::anastome::dialogue::ACT_LAWS, MODE_BATCH];
    put_key_count(&mut out, ts.len());
    for t in ts {
        put_transition(&mut out, t);
    }
    out
}

/// The batch's reply: `[count][verdict …]`, one verdict per transition in the
/// order they were sent.
pub fn encode_batch_reply(verdicts: &[Verdict]) -> Vec<u8> {
    let mut out = Vec::new();
    put_key_count(&mut out, verdicts.len());
    for v in verdicts {
        put_verdict(&mut out, v);
    }
    out
}

/// The verdicts a batch reply carries, read whole: bytes left over, or a
/// verdict that does not read, is no reply.
pub fn decode_batch_reply(b: &[u8]) -> Option<Vec<Verdict>> {
    let mut p = 0;
    let n = take_key_count(b, &mut p)?;
    let verdicts = (0..n).map(|_| take_verdict(b, &mut p)).collect::<Option<Vec<_>>>()?;
    (p == b.len()).then_some(verdicts)
}

/// `[ACT_LAWS][0]`.
pub fn encode_register_request() -> Vec<u8> {
    vec![crate::anastome::dialogue::ACT_LAWS, MODE_REGISTER]
}

/// `[ACT_LAWS][1][transition]`.
pub fn encode_verify_request(t: &Transition) -> Vec<u8> {
    let mut out = vec![crate::anastome::dialogue::ACT_LAWS, MODE_VERIFY];
    put_transition(&mut out, t);
    out
}

/// The request a frame carries, read whole: a verify request with bytes left
/// over is no request.
pub fn decode_laws_request(body: &[u8]) -> Option<LawsRequest> {
    match body {
        [act, MODE_REGISTER] if *act == crate::anastome::dialogue::ACT_LAWS => Some(LawsRequest::Register),
        [act, MODE_LEXICON] if *act == crate::anastome::dialogue::ACT_LAWS => Some(LawsRequest::Lexicon),
        [act, MODE_CLOCK] if *act == crate::anastome::dialogue::ACT_LAWS => Some(LawsRequest::Clock),
        [act, MODE_PARAMETERS] if *act == crate::anastome::dialogue::ACT_LAWS => Some(LawsRequest::Parameters),
        [act, MODE_VERIFY, rest @ ..] if *act == crate::anastome::dialogue::ACT_LAWS => {
            let mut p = 0;
            let t = take_transition(rest, &mut p)?;
            (p == rest.len()).then_some(LawsRequest::Verify(t))
        }
        [act, MODE_BATCH, rest @ ..] if *act == crate::anastome::dialogue::ACT_LAWS => {
            let mut p = 0;
            let n = take_key_count(rest, &mut p)?;
            // Read one by one, never reserved by the count: a count the bytes
            // do not carry fails at the first transition that does not read.
            let ts = (0..n).map(|_| take_transition(rest, &mut p)).collect::<Option<Vec<_>>>()?;
            (p == rest.len()).then_some(LawsRequest::Batch(ts))
        }
        _ => None,
    }
}

/// ONE LAW AS THE REGISTER SERVES IT: its coordinate and its statement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Served {
    pub law: Law,
    pub statement: String,
}

/// THE REGISTER'S REPLY: its version, then every law by its coordinate and
/// its statement, in the register's order.
pub fn encode_register_reply() -> Vec<u8> {
    let mut out = vec![LAW_VERSION];
    put_key_count(&mut out, LAWS.len());
    for law in LAWS {
        crate::transducer::address::put_key_bytes(&mut out, law.address().as_bytes());
        crate::transducer::address::put_key_bytes(&mut out, law.statement().as_bytes());
    }
    out
}

/// The register a reply carries: every law read back from its coordinate,
/// at the version the reply names. `None` where a coordinate names no law
/// of this register's version, or a statement is not text.
pub fn decode_register_reply(b: &[u8]) -> Option<Vec<Served>> {
    let mut p = 0;
    if take_u8(b, &mut p)? != LAW_VERSION {
        return None;
    }
    let n = take_key_count(b, &mut p)?;
    let laws = (0..n)
        .map(|_| {
            let a = crate::transducer::address::take_key_bytes(b, &mut p)?;
            let s = crate::transducer::address::take_key_bytes(b, &mut p)?;
            Some(Served { law: law_at(&Address::new(a))?, statement: String::from_utf8(s).ok()? })
        })
        .collect::<Option<Vec<_>>>()?;
    (p == b.len()).then_some(laws)
}

pub fn take_verdict(b: &[u8], p: &mut usize) -> Option<Verdict> {
    let n = take_key_count(b, p)?;
    let breaches = (0..n)
        .map(|_| Some((*LAWS.get(usize::from(take_u8(b, p)?))?, take_breach(b, p)?)))
        .collect::<Option<Vec<_>>>()?;
    let m = take_key_count(b, p)?;
    let exchanged = (0..m).map(|_| Some((take_key_count(b, p)?, take_mv(b, p)?))).collect::<Option<Vec<_>>>()?;
    let modes = take_key_count(b, p)?;
    let similarity = take_ratio(b, p)?;
    let expansion = take_ratio(b, p)?;
    let u = take_key_count(b, p)?;
    let unmeasured = (0..u)
        .map(|_| {
            let law = *LAWS.get(usize::from(take_u8(b, p)?))?;
            let needed_stroke = match take_u8(b, p)? {
                0 => None,
                1 => Some(take_u64(b, p)?),
                _ => return None,
            };
            Some(crate::physics::laws::Unmeasured { law, needed_stroke })
        })
        .collect::<Option<Vec<_>>>()?;
    Some(Verdict { breaches, exchanged, modes, similarity, expansion, unmeasured })
}
