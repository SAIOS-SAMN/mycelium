// ACT_SPEECH, ACT_ADOPT, ACT_REFINE — a voice standing words at a court,
// a court adopting another's ruling, and a bid to refine a ruled word.
//
//! The speech frame carries, per sentence, its standing, the contraction
//! certificate it cites (50.9) and its tape; the reply carries one
//! `Adjudication` per sentence sent, in the order sent (26).

use super::*;
use crate::transducer::sentence::Contraction;

/// `[ACT_REFINE][voice 32][word address][cites]` — the bidder's sentence for the
/// word (spoken by other words) rides as citations; the hypha replays.
pub fn encode_refine(voice: &[u8; 32], word: impl AsRef<[u8]>, cites: &[CiteWire]) -> Vec<u8> {
    let mut out = vec![ACT_REFINE];
    out.extend_from_slice(voice);
    let _ = put_addr(&mut out, word);
    push_cites(&mut out, cites);
    out
}

pub fn decode_refine(body: &[u8]) -> Option<([u8; 32], Address, Vec<CiteWire>)> {
    if body.first() != Some(&ACT_REFINE) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let mut p = 33usize;
    let word = take_addr(body, &mut p)?;
    // A frame whose cite section does not parse is MALFORMED, not a frame
    // that cites nothing: the two are different claims about the refinement
    // and only the sender knows which it made.
    let cites = read_cites(body, &mut p)?;
    Some((voice, word, cites))
}

/// ONE SENTENCE AS SPOKEN: its tape, the standing the speaker claims, and
/// the contraction certificate it cites, if any.
pub struct Spoken {
    pub tape: Vec<u8>,
    pub standing: u8,
    pub contraction: Option<Contraction>,
    // 50.2 step 2 as ruled 2026-09-10: `paths` was here. The generators a
    // voice reached each word vertex through are ON the tape at v7, in
    // the vertex's own derivation, because they are what the vertex is.
    // The contraction certificate stays beside the tape: it is a claim
    // about which utterance this is (50.9).
}

pub fn encode_speech(voice: &[u8; 32], task: impl AsRef<[u8]>, tu: u64, sentences: &[Vec<u8>]) -> Vec<u8> {
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
) -> Vec<u8> {
    let spoken: Vec<Spoken> = sentences
        .iter()
        .enumerate()
        .map(|(i, s)| Spoken {
            tape: s.clone(),
            // A sentence the caller named no standing for is spoken as
            // Stood, which is what speaking it claims.
            standing: match standing.get(i) {
                Some(s) => *s,
                None => crate::transducer::stalk::Standing::Stood.as_u8(),
            },
            contraction: None,
        })
        .collect();
    encode_speech_spoken(voice, task, tu, &spoken)
}

/// `[ACT_SPEECH][voice 32][task address][K u64][n u32]` then per sentence
/// `[standing u8][cert u8][contraction?][len u32][tape]`; the derivation
/// of every word vertex is inside the tape at v7 (50.2 step 2).
pub fn encode_speech_spoken(voice: &[u8; 32], task: impl AsRef<[u8]>, tu: u64, sentences: &[Spoken]) -> Vec<u8> {
    let mut out = vec![ACT_SPEECH];
    out.extend_from_slice(voice);
    let _ = put_addr(&mut out, task);
    out.extend_from_slice(&tu.to_le_bytes());
    out.extend_from_slice(&(sentences.len() as u32).to_le_bytes());
    for s in sentences {
        out.push(s.standing);
        match &s.contraction {
            Some(c) => {
                out.push(1);
                c.to_bytes(&mut out);
            }
            None => out.push(0),
        }
        out.extend_from_slice(&(s.tape.len() as u32).to_le_bytes());
        out.extend_from_slice(&s.tape);
    }
    out
}

pub fn decode_speech(body: &[u8]) -> Option<SpeechAct> {
    if body.first() != Some(&ACT_SPEECH) || body.len() < 47 {
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
    for _ in 0..n {
        let st = *body.get(pos)?;
        pos += 1;
        let cites = *body.get(pos)?;
        pos += 1;
        let cited = match cites {
            0 => None,
            1 => Some(Contraction::from_bytes(body, &mut pos)?),
            _ => return None,
        };
        let len = u32::from_le_bytes(body.get(pos..pos + 4)?.try_into().ok()?) as usize;
        pos += 4;
        let s = body.get(pos..pos + len)?.to_vec();
        pos += len;
        standing.push(st);
        contractions.push(cited);
        sentences.push(s);
    }
    Some(SpeechAct { voice, task, k: tu, sentences, standing, contractions })
}

/// `[ACT_SPEECH][n u8]` then per sentence `[code u8][receipt 32]` and, for
/// a refusal, `[kind u8][at]`.
/// `[ACT_SPEECH][n u8]` then per sentence `[code u8]` and what the code
/// carries: a 32-byte receipt for a verdict, the refusal's kind and
/// coordinate after it for `SPEECH_REFUSED`, and for `SPEECH_DIALECT` the
/// CITED ADDRESS length-prefixed in the receipt's place — a wait admitted
/// no act, so there is no chain position to write and none is invented
/// (rule 11).
pub fn encode_speech_reply(adjudications: &[Adjudication]) -> Vec<u8> {
    let mut out = vec![ACT_SPEECH, adjudications.len().min(255) as u8];
    for a in adjudications.iter().take(255) {
        out.push(a.code());
        match a {
            Adjudication::Awaiting { form } => {
                let _ = put_addr(&mut out, form);
            }
            _ => {
                if let Some(r) = a.receipt() {
                    out.extend_from_slice(&r);
                }
                if let Adjudication::Refused { kind, at, .. } = a {
                    out.push(kind.to_byte());
                    at.to_bytes(&mut out);
                }
            }
        }
    }
    out
}

pub fn decode_speech_reply(body: &[u8]) -> Option<Vec<Adjudication>> {
    if body.first() != Some(&ACT_SPEECH) {
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
        out.push(match code {
            SPEECH_STOOD => Adjudication::Stood { receipt },
            SPEECH_RATIFIED => Adjudication::Ratified { receipt },
            SPEECH_REFUSED => {
                let kind = crate::transducer::sentence::RefusalKind::from_byte(*body.get(pos)?)?;
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
    if body.first() != Some(&ACT_ADOPT) || body.len() <= 33 {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    Some((voice, body[33..].to_vec()))
}
