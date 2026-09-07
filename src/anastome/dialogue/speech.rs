// ACT_SPEECH, ACT_ADOPT, ACT_REFINE — a voice standing words at a court,
// a court adopting another's ruling, and a bid to refine a ruled word.

use super::*;

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
    let mut out = vec![ACT_SPEECH];
    out.extend_from_slice(voice);
    let _ = put_addr(&mut out, task);
    out.extend_from_slice(&tu.to_le_bytes());
    out.extend_from_slice(&(sentences.len() as u32).to_le_bytes());
    for (i, s) in sentences.iter().enumerate() {
        out.push(standing.get(i).copied().unwrap_or(crate::transducer::stalk::Standing::Stood.as_u8()));
        out.extend_from_slice(&(s.len() as u32).to_le_bytes());
        out.extend_from_slice(s);
    }
    out
}

pub fn decode_speech(body: &[u8]) -> Option<Speech> {
    let (voice, task, tu, sentences, _) = decode_speech_standing(body)?;
    Some((voice, task, tu, sentences))
}

pub fn decode_speech_standing(body: &[u8]) -> Option<SpeechStanding> {
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
    for _ in 0..n {
        let st = *body.get(pos)?;
        pos += 1;
        let len = u32::from_le_bytes(body.get(pos..pos + 4)?.try_into().ok()?) as usize;
        pos += 4;
        let s = body.get(pos..pos + len)?.to_vec();
        pos += len;
        standing.push(st);
        sentences.push(s);
    }
    Some((voice, task, tu, sentences, standing))
}


pub fn encode_speech_reply(codes: &[u8]) -> Vec<u8> {
    let mut out = vec![ACT_SPEECH, codes.len() as u8];
    out.extend_from_slice(codes);
    out
}

pub fn decode_speech_reply(body: &[u8]) -> Option<Vec<u8>> {
    if body.first() != Some(&ACT_SPEECH) {
        return None;
    }
    let n = *body.get(1)? as usize;
    Some(body.get(2..2 + n)?.to_vec())
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
