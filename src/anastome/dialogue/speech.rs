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
    let cites = read_cites(body, &mut p).unwrap_or_default();
    Some((voice, word, cites))
}

/// ONE SENTENCE AS SPOKEN: its tape, the standing the speaker claims, and
/// the contraction certificate it cites, if any.
pub struct Spoken {
    pub tape: Vec<u8>,
    pub standing: u8,
    pub contraction: Option<Contraction>,
    /// 50.2 step 2: the generators this voice reached each word vertex
    /// through. Beside the tape and never in it — the geodesic quotients
    /// the path away (1.5), so it is not a coordinate of the form (law
    /// 12) and carrying it forces no tape change.
    pub paths: Vec<crate::transducer::sentence::Path>,
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
            standing: standing.get(i).copied().unwrap_or(crate::transducer::stalk::Standing::Stood.as_u8()),
            contraction: None,
            paths: Vec::new(),
        })
        .collect();
    encode_speech_spoken(voice, task, tu, &spoken)
}

/// `[ACT_SPEECH][voice 32][task address][K u64][n u32]` then per sentence
/// `[standing u8][cert u8][contraction?][paths u32][path…][len u32][tape]`.
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
        out.extend_from_slice(&(s.paths.len() as u32).to_le_bytes());
        for p in &s.paths {
            p.to_bytes(&mut out);
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
    let mut paths: Vec<Vec<crate::transducer::sentence::Path>> = Vec::with_capacity(n.min(64));
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
        let np = u32::from_le_bytes(body.get(pos..pos + 4)?.try_into().ok()?) as usize;
        pos += 4;
        let mut ps = Vec::with_capacity(np.min(64));
        for _ in 0..np {
            ps.push(crate::transducer::sentence::Path::from_bytes(body, &mut pos)?);
        }
        let len = u32::from_le_bytes(body.get(pos..pos + 4)?.try_into().ok()?) as usize;
        pos += 4;
        let s = body.get(pos..pos + len)?.to_vec();
        pos += len;
        standing.push(st);
        contractions.push(cited);
        paths.push(ps);
        sentences.push(s);
    }
    Some(SpeechAct { voice, task, k: tu, sentences, standing, contractions, paths })
}

/// `[ACT_SPEECH][n u8]` then per sentence `[code u8][receipt 32]` and, for
/// a refusal, `[kind u8][at]`.
pub fn encode_speech_reply(adjudications: &[Adjudication]) -> Vec<u8> {
    let mut out = vec![ACT_SPEECH, adjudications.len().min(255) as u8];
    for a in adjudications.iter().take(255) {
        out.push(a.code());
        out.extend_from_slice(&a.receipt());
        if let Adjudication::Refused { kind, at, .. } = a {
            out.push(kind.to_byte());
            at.to_bytes(&mut out);
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
