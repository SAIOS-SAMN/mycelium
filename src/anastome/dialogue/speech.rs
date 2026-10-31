// ACT_SPEECH, ACT_ADOPT — a voice standing words at a court, and a court
// adopting another's ruling.
//
//! The speech frame carries, per sentence, its standing, the contraction
//! certificate it cites (50.9), the agreement of another mind where it is
//! agreed language (MU-CLIENT §5.5) and its tape; the reply carries one
//! `Adjudication` per sentence sent, in the order sent (26).

use super::*;
use crate::transducer::sentence::Contraction;

/// ONE SENTENCE AS SPOKEN: its tape, the standing the speaker claims, and
/// the contraction certificate it cites, if any.
pub struct Spoken {
    pub tape: Vec<u8>,
    pub standing: u8,
    pub contraction: Option<Contraction>,
    /// MU-CLIENT §5.5 — AGREED LANGUAGE: the signed HOLD frame of another
    /// mind that re-derived this sentence and holds one loop with the
    /// speaker on every word (78.161, 78.163). The court verifies the
    /// signature and seals the other mind's attestation (`Attest`).
    pub agreement: Option<Vec<u8>>,
    // 50.2 step 2 as ruled 2026-09-10: `paths` was here. The generators a
    // voice reached each word vertex through are ON the tape at v7, in
    // the vertex's own derivation, because they are what the vertex is.
    // The contraction certificate stays beside the tape: it is a claim
    // about which utterance this is (50.9).
}

/// `None` where the task's address cannot be framed (LD0): not spoken.
pub fn encode_speech(voice: &[u8; 32], task: impl AsRef<[u8]>, tu: u64, sentences: &[Vec<u8>]) -> Option<Vec<u8>> {
    encode_speech_standing(voice, task, tu, sentences, &[])
}

/// Each sentence carries the standing of the word it rests on (one byte
/// beside the record). Empty `standing` is Stood for every sentence.
pub fn encode_speech_standing(
    voice: &[u8; 32],
    task: impl AsRef<[u8]>,
    tu: u64,
    sentences: &[Vec<u8>],
    standing: &[u8],
) -> Option<Vec<u8>> {
    let spoken: Vec<Spoken> = sentences
        .iter()
        .enumerate()
        .map(|(i, s)| Spoken {
            tape: s.clone(),
            // A sentence the caller named no standing for is spoken as
            // Stood, which is what speaking it claims.
            standing: match standing.get(i) {
                Some(s) => *s,
                None => crate::transducer::stalk::Standing::Stood.byte(),
            },
            contraction: None,
            agreement: None,
        })
        .collect();
    encode_speech_spoken(voice, task, tu, &spoken)
}

/// `[ACT_SPEECH][voice 32][task address][K u64][n u32]` then per sentence
/// `[standing u8][carries u8][contraction?][agreement?][len u32][tape]`:
/// `carries` bit 0 a contraction certificate follows, bit 1 an agreement
/// (`[len u32][the other mind's signed HOLD frame]`); the derivation
/// of every word vertex is inside the tape at v7 (50.2 step 2). `None`
/// where the task's address cannot be framed (LD0): the speech is not
/// spoken, never sent with the task dropped.
pub fn encode_speech_spoken(
    voice: &[u8; 32],
    task: impl AsRef<[u8]>,
    tu: u64,
    sentences: &[Spoken],
) -> Option<Vec<u8>> {
    let mut out = vec![ACT_SPEECH];
    out.extend_from_slice(voice);
    put_addr(&mut out, task)?;
    out.extend_from_slice(&tu.to_le_bytes());
    crate::transducer::address::put_u32_count(&mut out, sentences.len())?;
    for s in sentences {
        out.push(s.standing);
        out.push(u8::from(s.contraction.is_some()) | (u8::from(s.agreement.is_some()) << 1));
        if let Some(c) = &s.contraction {
            c.to_bytes(&mut out)?;
        }
        if let Some(a) = &s.agreement {
            crate::transducer::address::put_bytes(&mut out, a)?;
        }
        crate::transducer::address::put_bytes(&mut out, &s.tape)?;
    }
    Some(out)
}

pub fn decode_speech(body: &[u8]) -> Option<SpeechAct> {
    if !act_is(body, ACT_SPEECH) || body.len() < 47 {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let mut pos = 33usize;
    let task = take_addr(body, &mut pos)?;
    let tu = u64::from_le_bytes(body.get(pos..pos + 8)?.try_into().ok()?);
    pos += 8;
    let n = u32::from_le_bytes(body.get(pos..pos + 4)?.try_into().ok()?) as usize;
    pos += 4;
    let mut sentences = Vec::with_capacity(n.min(64));
    let mut standing = Vec::with_capacity(n.min(64));
    let mut contractions = Vec::with_capacity(n.min(64));
    let mut agreements = Vec::with_capacity(n.min(64));
    for _ in 0..n {
        let st = *body.get(pos)?;
        pos += 1;
        let carries = *body.get(pos)?;
        pos += 1;
        if carries & !0b11 != 0 {
            return None;
        }
        let cited = match carries & 1 {
            0 => None,
            _ => Some(Contraction::from_bytes(body, &mut pos)?),
        };
        let agreed = match carries & 0b10 {
            0 => None,
            _ => Some(crate::transducer::address::take_bytes(body, &mut pos)?),
        };
        let s = crate::transducer::address::take_bytes(body, &mut pos)?;
        standing.push(st);
        contractions.push(cited);
        agreements.push(agreed);
        sentences.push(s);
    }
    Some(SpeechAct { voice, task, k: tu, sentences, standing, contractions, agreements })
}

/// `[ACT_SPEECH][n u8]` then per sentence `[code u8][receipt 32]` and, for
/// a refusal, `[kind u8][at]`.
/// `[ACT_SPEECH][n u8]` then per sentence `[code u8]` and what the code
/// carries: a 32-byte receipt for a verdict, the refusal's kind and
/// coordinate after it for `SPEECH_REFUSED`, and for `SPEECH_DIALECT` the
/// CITED ADDRESS length-prefixed in the receipt's place — a wait admitted
/// no act, so there is no chain position to write and none is invented
/// (rule 11).
/// `None` where more adjudications stand than the count can name: a reply
/// that answers 255 of 300 sentences is not this court's answer.
pub fn encode_speech_reply(adjudications: &[Adjudication]) -> Option<Vec<u8>> {
    let mut out = vec![ACT_SPEECH];
    crate::transducer::address::put_u8_count(&mut out, adjudications.len())?;
    for a in adjudications {
        out.push(a.code());
        match a {
            // A wait NAMES the form it waits on; one whose address does
            // not fit its prefix names nothing, and the reply is refused
            // rather than sent with the field dropped.
            Adjudication::Awaiting { form } => put_addr(&mut out, form)?,
            _ => {
                if let Some(r) = a.receipt() {
                    out.extend_from_slice(&r);
                }
                match a {
                    Adjudication::Refused { kind, at, .. } => {
                        out.push(kind.byte());
                        at.to_bytes(&mut out)?;
                    }
                    // 78.93 B0 — where the word stands, where an act
                    // admitted it.
                    _ => match a.at() {
                        Some(s) => {
                            out.push(1);
                            s.put_canonical(&mut out);
                        }
                        None => out.push(0),
                    },
                }
            }
        }
    }
    Some(out)
}

pub fn decode_speech_reply(body: &[u8]) -> Option<Vec<Adjudication>> {
    if !act_is(body, ACT_SPEECH) {
        return None;
    }
    let n = *body.get(1)? as usize;
    let mut pos = 2usize;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let code = *body.get(pos)?;
        pos += 1;
        // A wait carries the cited address where a verdict carries its
        // receipt, so the shape is read from the code and never assumed.
        if code == SPEECH_DIALECT {
            let form = take_addr(body, &mut pos)?;
            out.push(Adjudication::Awaiting { form });
            continue;
        }
        let receipt: [u8; 32] = body.get(pos..pos + 32)?.try_into().ok()?;
        pos += 32;
        let stood_at = |pos: &mut usize| -> Option<Option<crate::chronometry::Stamp>> {
            let flag = *body.get(*pos)?;
            *pos += 1;
            match flag {
                0 => Some(None),
                1 => Some(Some(crate::chronometry::Stamp::take_canonical(body, pos)?)),
                _ => None,
            }
        };
        out.push(match code {
            SPEECH_STOOD => Adjudication::Stood { receipt, at: stood_at(&mut pos)? },
            SPEECH_RATIFIED => Adjudication::Ratified { receipt, at: stood_at(&mut pos)? },
            SPEECH_REFUSED => {
                let kind = crate::transducer::sentence::RefusalKind::of(*body.get(pos)?)?;
                pos += 1;
                let at = crate::transducer::sentence::At::from_bytes(body, &mut pos)?;
                Adjudication::Refused { kind, at, receipt }
            }
            _ => return None,
        });
    }
    Some(out)
}

pub fn encode_adopt(voice: &[u8; 32], element_bytes: &[u8]) -> Vec<u8> {
    let mut out = vec![ACT_ADOPT];
    out.extend_from_slice(voice);
    out.extend_from_slice(element_bytes);
    out
}

pub fn decode_adopt(body: &[u8]) -> Option<([u8; 32], Vec<u8>)> {
    if !act_is(body, ACT_ADOPT) || body.len() <= 33 {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    Some((voice, body[33..].to_vec()))
}
