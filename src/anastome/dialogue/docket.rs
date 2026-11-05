// ACT_EVIDENCE — what was deposited against the court's open listings, and
// the receiver's answer to a ruling. The docket, teach and settle codecs
// retired with their acts (26, law 8): the door refuses those bytes by kind.

use super::*;

/// evidence request: `[ACT_EVIDENCE][voice 32]`.
pub fn encode_evidence_request(voice: &[u8; 32]) -> Vec<u8> {
    let mut out = vec![ACT_EVIDENCE];
    out.extend_from_slice(voice);
    out
}

pub fn decode_evidence_request(body: &[u8]) -> Option<[u8; 32]> {
    if !act_is(body, ACT_EVIDENCE) || body.len() != 33 {
        return None; // exactly [act][voice] — longer bodies are DEPOSITS
    }
    body.get(1..33)?.try_into().ok()
}

/// evidence DEPOSIT (an entity's verified dialect finding → the court's
/// docket): `[ACT_EVIDENCE][voice 32][evidence-reply payload]`. A 33-byte
/// body is a pull; anything longer is a deposit — one act, two directions.
pub fn encode_evidence_deposit<A: AsRef<[u8]>>(
    voice: &[u8; 32],
    items: &[EvidenceOf<A>],
) -> Option<Vec<u8>> {
    let mut out = vec![ACT_EVIDENCE];
    out.extend_from_slice(voice);
    out.extend_from_slice(&encode_evidence_reply(items)?);
    Some(out)
}

pub fn decode_evidence_deposit(body: &[u8]) -> Option<EvidenceDeposit> {
    if !act_is(body, ACT_EVIDENCE) || body.len() <= 33 {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let items = decode_evidence_reply(&body[33..])?;
    Some((voice, items))
}

/// THE EVIDENCE REPLY'S HEADER: `[n u32]` ahead of the items.
/// FORMAT: `encode_evidence_reply`'s count field, 4 bytes.
pub const EVIDENCE_REPLY_HEADER: usize = 4;

/// THE BYTES ONE ITEM TAKES ON THE REPLY, `σ(e)`, exact from the format:
/// `2 + |address| + 4 + 16·|footprint| + 4 + 17·|scene|` (D4.2). `None` past
/// the address's `u16` wall or the carrier.
pub fn evidence_width<A: AsRef<[u8]>>(e: &EvidenceOf<A>) -> Option<usize> {
    u16::try_from(e.addr.as_ref().len()).ok()?;
    let fp = e.footprint.len().checked_mul(16)?;
    let scene = e.scene.len().checked_mul(17)?;
    (2 + e.addr.as_ref().len() + 4).checked_add(fp)?.checked_add(4)?.checked_add(scene)
}

/// WHERE AN EVIDENCE PAGE ENDED (D4.2): the store read to its end, the next
/// item past the page's room, an item no frame can carry (refused by name,
/// with its width — the page ends before it and it is reported, never
/// skipped), or the stroke spent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidencePageEnd {
    Store,
    Full,
    /// Its width, `None` past the address's wall or the carrier.
    Unframable { width: Option<usize> },
    Spent,
}

/// THE EVIDENCE PAGE A REPLY CARRIES: the longest prefix of `store` whose
/// items, behind the reply's header, fit `budget`, one read of the stroke an
/// item; and where it ended. A filled page, never a count chosen beside it.
pub fn evidence_page<A: AsRef<[u8]>>(store: &[EvidenceOf<A>], budget: usize, reads: &mut crate::physics::Budget) -> (usize, EvidencePageEnd) {
    let mut used = EVIDENCE_REPLY_HEADER;
    for (m, e) in store.iter().enumerate() {
        if !reads.spend() {
            return (m, EvidencePageEnd::Spent);
        }
        let width = match evidence_width(e) {
            Some(w) if EVIDENCE_REPLY_HEADER.checked_add(w).is_some_and(|whole| whole <= budget) => w,
            width => return (m, EvidencePageEnd::Unframable { width }),
        };
        match used.checked_add(width) {
            Some(next) if next <= budget => used = next,
            _ => return (m, EvidencePageEnd::Full),
        }
    }
    (store.len(), EvidencePageEnd::Store)
}

/// evidence reply: `[n u32] ( [addr][nf u32][fp (site)·nf]
/// [ns u32][scene (site, h u8)·ns] )…`, each site a section site written
/// along the plane in the plane's order, `i64` per coordinate
/// (`SectionSite::put_canonical`) — the selection fibre's birth evidence,
/// count-guarded at every level.
/// `None` where an address cannot be framed: the whole reply is refused,
/// never a partial one (LD0, rule 11).
pub fn encode_evidence_reply<A: AsRef<[u8]>>(
    items: &[EvidenceOf<A>],
) -> Option<Vec<u8>> {
    use crate::transducer::address::put_u32_count;
    let mut out = Vec::new();
    put_u32_count(&mut out, items.len())?;
    for EvidenceOf { addr, footprint, scene } in items {
        put_address(&mut out, addr)?;
        put_u32_count(&mut out, footprint.len())?;
        for site in footprint {
            site.put_canonical(&mut out);
        }
        put_u32_count(&mut out, scene.len())?;
        for p in scene {
            p.site.put_canonical(&mut out);
            out.push(p.symbol);
        }
    }
    Some(out)
}

pub fn decode_evidence_reply(body: &[u8]) -> Option<Vec<EvidenceItem>> {
    let mut p = 0usize;
    let n = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    if n > body.len().checked_sub(p)? / 10 {
        return None; // untrusted count
    }
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let addr = take_address(body, &mut p)?;
        let nf = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
        if nf > body.len().checked_sub(p)? / 16 {
            return None;
        }
        let mut fp = Vec::with_capacity(nf);
        for _ in 0..nf {
            fp.push(crate::transducer::spelled::SectionSite::take_canonical(body, &mut p)?);
        }
        let ns = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
        if ns > body.len().checked_sub(p)? / 17 {
            return None;
        }
        let mut scene = Vec::with_capacity(ns);
        for _ in 0..ns {
            let site = crate::transducer::spelled::SectionSite::take_canonical(body, &mut p)?;
            let h = *body.get(p)?;
            p += 1;
            scene.push(crate::transducer::spelled::PaintedSite::new(site, h));
        }
        out.push(EvidenceOf { addr, footprint: fp, scene });
    }
    Some(out)
}

/// The receiver's answer to a ruling: `[arm u8][payload]` — `0` applied,
/// `1` inert, `2` refused with `len16 + reason`, `3` gap with `from` (a
/// tag byte, then its place where `1`) and `to`, each place `K‖i`.
/// `None` where the reason passes its u16: the answer is refused, never
/// wrapped (LD0).
pub fn encode_ruled(r: &Ruled) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    match r {
        Ruled::Applied => out.push(0),
        Ruled::Inert => out.push(1),
        Ruled::Refused(why) => {
            out.push(2);
            crate::transducer::address::put_u16_count(&mut out, why.len())?;
            out.extend_from_slice(why);
        }
        Ruled::Gap { from, to } => {
            out.push(3);
            match from {
                None => out.push(0),
                Some(f) => {
                    out.push(1);
                    f.put_canonical(&mut out);
                }
            }
            to.put_canonical(&mut out);
        }
    }
    Some(out)
}

pub fn decode_ruled(body: &[u8]) -> Option<Ruled> {
    match act_tag(body)? {
        0 => Some(Ruled::Applied),
        1 => Some(Ruled::Inert),
        2 => {
            let n = u16::from_le_bytes(body.get(1..3)?.try_into().ok()?) as usize;
            Some(Ruled::Refused(body.get(3..3 + n)?.to_vec()))
        }
        3 => {
            let mut p = 2usize;
            let from = match body.get(1)? {
                0 => None,
                1 => Some(crate::chronometry::Place::take_canonical(body, &mut p)?),
                _ => return None,
            };
            let to = crate::chronometry::Place::take_canonical(body, &mut p)?;
            (p == body.len()).then_some(Ruled::Gap { from, to })
        }
        // An arm the layout does not define is refused, never guessed at.
        _ => None,
    }
}
