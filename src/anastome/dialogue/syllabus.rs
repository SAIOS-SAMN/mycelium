// A19 — ACT_SYLLABUS: every court-ratified word in ruling order, and the
// class records that travel as coordinates beside them.

use super::*;

pub fn encode_syllabus_request(voice: &[u8; 32]) -> Vec<u8> {
    let mut out = vec![ACT_SYLLABUS];
    out.extend_from_slice(voice);
    out
}

pub fn decode_syllabus_request(body: &[u8]) -> Option<[u8; 32]> {
    if !act_is(body, ACT_SYLLABUS) || body.len() != 33 {
        return None;
    }
    body.get(1..33)?.try_into().ok()
}

/// ONE TAUGHT WORD: the stamp that locates it (78.24 (2b)) — the serving
/// court's Syllabus act on the stream, the word's admitting stamp on a
/// fetch — and the word's tape.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Taught {
    pub at: crate::chronometry::Stamp,
    pub word: Vec<u8>,
}

/// `[n u8]` then per word `[stamp 48][len u32][tape]`. `None` where the
/// page holds more words than the count can name: a reply that carries 255
/// of them under a count of 255 is a shorter page than the one served, and
/// the fetch that asked cannot tell.
pub fn encode_syllabus_reply(words: &[Taught]) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    crate::transducer::address::put_u8_count(&mut out, words.len())?;
    for t in words {
        t.at.put_canonical(&mut out);
        crate::transducer::address::put_bytes(&mut out, &t.word)?;
    }
    Some(out)
}

pub fn decode_syllabus_reply(body: &[u8]) -> Option<Vec<Taught>> {
    let n = act_tag(body)? as usize;
    let mut p = 1usize;
    let mut words = Vec::with_capacity(n.min(64));
    for _ in 0..n {
        let at = crate::chronometry::Stamp::take_canonical(body, &mut p)?;
        let len = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
        if len > body.len().checked_sub(p)? {
            return None; // untrusted length
        }
        words.push(Taught { at, word: body.get(p..p + len)?.to_vec() });
        p += len;
    }
    Some(words)
}

/// One class record on a syllabus page: key, order (0 = infinite), and the
/// representative rotor the reader seeds its horizon with.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassPage {
    pub key: crate::transducer::address::Address,
    pub order: u32,
    pub rotor: crate::transducer::alphabet::Rotor,
}

/// The records framed whole, or `None` with nothing written: a count past
/// its u16 or a class key that reaches its wall is refused, never cut.
///
/// LD0 step 5: the representative is written inline as its canonical ray
/// (A1), its sixteen coordinates, never a length-prefixed blob of raw
/// coefficients beside the class record.
pub fn append_class_records(out: &mut Vec<u8>, records: &[ClassPage]) -> Option<()> {
    let mut rec = Vec::new();
    crate::transducer::address::put_u16_count(&mut rec, records.len())?;
    for r in records {
        crate::transducer::address::put_class_record(&mut rec, &r.key, r.order)?;
        r.rotor.put_canonical(&mut rec);
    }
    out.append(&mut rec);
    Some(())
}

/// The class records after the taught words, or none: a record that does
/// not read refuses the page whole and says so, never a page cut short at
/// the record that failed.
pub fn decode_class_records(body: &[u8]) -> Vec<ClassPage> {
    let Some(words) = decode_syllabus_reply(body) else {
        return Vec::new();
    };
    let mut p = 1usize;
    for t in &words {
        // STD1 — an offset past its carrier is past the body, and a page
        // whose words run past its body names no classes, as below.
        let Some(next) = p
            .checked_add(crate::chronometry::Stamp::LEN)
            .and_then(|q| q.checked_add(4))
            .and_then(|q| q.checked_add(t.word.len()))
        else {
            return Vec::new();
        };
        p = next;
    }
    if p + 2 > body.len() {
        return Vec::new();
    }
    // The two bytes are in range — the bound above says so — so the count
    // is read exactly; a slice that does not convert is a malformed frame
    // and names no classes.
    let Ok(raw) = <[u8; 2]>::try_from(&body[p..p + 2]) else {
        return Vec::new();
    };
    let n = u16::from_le_bytes(raw) as usize;
    p += 2;
    // No cap on `n`: each record reads at least its class record's bytes,
    // so a count past the body refuses at the first record it cannot read.
    let mut out = Vec::new();
    for i in 0..n {
        let read = |p: &mut usize| -> Option<ClassPage> {
            let (key, order) = crate::transducer::address::take_class_record(body, p)?;
            let rotor = crate::transducer::alphabet::Rotor::from_bytes(body, p)?;
            Some(ClassPage { key, order, rotor })
        };
        match read(&mut p) {
            Some(page) => out.push(page),
            None => {
                eprintln!("  syllabus class page refused: record {i} of {n} does not read at byte {p}");
                return Vec::new();
            }
        }
    }
    out
}
