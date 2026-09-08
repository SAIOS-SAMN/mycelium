// A19 — ACT_SYLLABUS: every court-ratified word in ruling order, and the
// class records that travel as coordinates beside them.

use super::*;

pub fn encode_syllabus_request(voice: &[u8; 32]) -> Vec<u8> {
    let mut out = vec![ACT_SYLLABUS];
    out.extend_from_slice(voice);
    out
}

pub fn decode_syllabus_request(body: &[u8]) -> Option<[u8; 32]> {
    if body.first() != Some(&ACT_SYLLABUS) || body.len() != 33 {
        return None;
    }
    body.get(1..33)?.try_into().ok()
}

pub fn encode_syllabus_reply(words: &[Vec<u8>]) -> Vec<u8> {
    let mut out = vec![words.len().min(255) as u8];
    for w in words.iter().take(255) {
        out.extend_from_slice(&(w.len() as u32).to_le_bytes());
        out.extend_from_slice(w);
    }
    out
}

pub fn decode_syllabus_reply(body: &[u8]) -> Option<Vec<Vec<u8>>> {
    let n = *body.first()? as usize;
    let mut p = 1usize;
    let mut words = Vec::with_capacity(n.min(64));
    for _ in 0..n {
        let len = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
        if len > body.len().saturating_sub(p) {
            return None; // untrusted length
        }
        words.push(body.get(p..p + len)?.to_vec());
        p += len;
    }
    Some(words)
}

/// One class record on a syllabus page: key, order (0 = infinite), rotor bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassPage {
    pub key: crate::transducer::address::Address,
    pub order: u32,
    pub rotor: Vec<u8>,
}

pub fn append_class_records(out: &mut Vec<u8>, records: &[ClassPage]) {
    out.extend_from_slice(&(records.len().min(u16::MAX as usize) as u16).to_le_bytes());
    for r in records {
        let _ = crate::transducer::address::put_class_record(out, &r.key, r.order);
        out.extend_from_slice(&(r.rotor.len() as u32).to_le_bytes());
        out.extend_from_slice(&r.rotor);
    }
}

pub fn decode_class_records(body: &[u8]) -> Vec<ClassPage> {
    let Some(words) = decode_syllabus_reply(body) else {
        return Vec::new();
    };
    let mut p = 1usize;
    for w in &words {
        p = p.saturating_add(4).saturating_add(w.len());
    }
    if p + 2 > body.len() {
        return Vec::new();
    }
    let n = u16::from_le_bytes(body[p..p + 2].try_into().unwrap_or([0, 0])) as usize;
    p += 2;
    let mut out = Vec::new();
    for _ in 0..n.min(1024) {
        let Some((key, order)) = crate::transducer::address::take_class_record(body, &mut p) else {
            break;
        };
        let Some(rlen) = body.get(p..p + 4).and_then(|s| s.try_into().ok()).map(u32::from_le_bytes) else {
            break;
        };
        p += 4;
        let rlen = rlen as usize;
        if p + rlen > body.len() {
            break;
        }
        let rotor = body[p..p + rlen].to_vec();
        p += rlen;
        out.push(ClassPage { key, order, rotor });
    }
    out
}
