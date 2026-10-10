// 78.160 P1 — THE PRESENTATION ASKED BY A RESIDUE: a sub-kind of ACT_FETCH.
// A mind sends a motion it could not cite; its court answers with the words
// that deconstruct it and the deconstruction itself.
//
//! Request: `[ACT_FETCH][voice 32][0xFFFF_FFFF][motion]` — the marker stands
//! where a fetch writes its chart count, a count no fetch can carry (a fetch
//! decoder refuses it as more charts than the body holds), so the two never
//! read as one another. The motion is `Mv::to_bytes`, read whole.
//!
//! Reply: the syllabus page shape (`encode_syllabus_reply`: the words the mind
//! lacked, each at the stamp that admitted it), then `[read u8][n u16]
//! {[form addr][vertex addr][e i64]}·n [composes u8][residue u64][holes u64]`.

use super::*;
use crate::transducer::cga::Mv;

/// The marker a presentation ask writes where a fetch writes its count.
const ASK: u32 = u32::MAX;

/// WHAT THE COURT READ OF THE MOTION: whether its coordinates read at all;
/// the generators cited, each `(form, vertex, exponent)`; whether their
/// product is the motion exactly; the holes no held generator reached; and
/// the holes the motion has.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PresentationAnswer {
    pub read: bool,
    pub cites: Vec<(Address, Address, i64)>,
    pub composes: bool,
    pub residue: u64,
    pub holes: u64,
}

pub fn encode_presentation_request(voice: &[u8; 32], motion: &Mv) -> Vec<u8> {
    let mut out = vec![ACT_FETCH];
    out.extend_from_slice(voice);
    out.extend_from_slice(&ASK.to_le_bytes());
    motion.to_bytes(&mut out);
    out
}

/// The asker and the motion, read whole; `None` for any other body,
/// a fetch included.
pub fn decode_presentation_request(body: &[u8]) -> Option<([u8; 32], Mv)> {
    if !act_is(body, ACT_FETCH) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    if body.get(33..37)? != ASK.to_le_bytes().as_slice() {
        return None;
    }
    let mut p = 37usize;
    let motion = Mv::from_bytes(body, &mut p)?;
    (p == body.len()).then_some((voice, motion))
}

/// `None` where a count or an address passes the field that carries it
/// (STD1 3c): the reply is refused whole, never framed short.
pub fn encode_presentation_reply(words: &[Taught], a: &PresentationAnswer) -> Option<Vec<u8>> {
    let mut out = encode_syllabus_reply(words)?;
    out.push(u8::from(a.read));
    crate::transducer::address::put_u16_count(&mut out, a.cites.len())?;
    for (form, vertex, e) in &a.cites {
        put_addr(&mut out, form)?;
        put_addr(&mut out, vertex)?;
        out.extend_from_slice(&e.to_le_bytes());
    }
    out.push(u8::from(a.composes));
    out.extend_from_slice(&a.residue.to_le_bytes());
    out.extend_from_slice(&a.holes.to_le_bytes());
    Some(out)
}

/// The words and the answer, read whole, or refused: a byte left over is no
/// answer (rule 11).
pub fn decode_presentation_reply(body: &[u8]) -> Option<(Vec<Taught>, PresentationAnswer)> {
    let words = decode_syllabus_reply(body)?;
    let mut p = 1usize;
    for t in &words {
        p = p.checked_add(crate::chronometry::Stamp::LEN)?.checked_add(4)?.checked_add(t.word.len())?;
    }
    let flag = |b: u8| match b {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    };
    let read = flag(*body.get(p)?)?;
    p += 1;
    let n = crate::transducer::address::take_u16_count(body, &mut p)?;
    let mut cites = Vec::with_capacity(n.min(64));
    for _ in 0..n {
        let form = take_addr(body, &mut p)?;
        let vertex = take_addr(body, &mut p)?;
        let e = i64::from_le_bytes(body.get(p..p + 8)?.try_into().ok()?);
        p += 8;
        cites.push((form, vertex, e));
    }
    let composes = flag(*body.get(p)?)?;
    p += 1;
    let residue = read_u64(body, &mut p)?;
    let holes = read_u64(body, &mut p)?;
    (p == body.len()).then_some((words, PresentationAnswer { read, cites, composes, residue, holes }))
}
