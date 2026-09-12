// ACT_PEERS, ACT_ROOT, ACT_DIGESTS, ACT_COVER — how two courts find each
// other and read what the other holds.
//
//! The cover digest is the signed statement of where a court stands, and
//! its frame is parsed WHOLE: `whole_cover_digest` accepts the one reading
//! that consumes the buffer exactly and refuses on two or zero.

use super::*;

pub fn encode_peers_request(voice: &[u8; 32]) -> Vec<u8> {
    let mut out = vec![ACT_PEERS];
    out.extend_from_slice(voice);
    out
}

pub fn decode_peers_request(body: &[u8]) -> Option<[u8; 32]> {
    if !act_is(body, ACT_PEERS) || body.len() != 33 {
        return None;
    }
    body[1..33].try_into().ok()
}

pub fn encode_peers_reply(addrs: &[String]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&(addrs.len().min(u16::MAX as usize) as u16).to_le_bytes());
    for a in addrs.iter().take(u16::MAX as usize) {
        let b = a.as_bytes();
        out.extend_from_slice(&(b.len().min(u16::MAX as usize) as u16).to_le_bytes());
        out.extend_from_slice(&b[..b.len().min(u16::MAX as usize)]);
    }
    out
}

pub fn decode_peers_reply(body: &[u8]) -> Option<Vec<String>> {
    let mut p = 0usize;
    let n = u16::from_le_bytes(body.get(p..p + 2)?.try_into().ok()?) as usize;
    p += 2;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let len = u16::from_le_bytes(body.get(p..p + 2)?.try_into().ok()?) as usize;
        p += 2;
        out.push(String::from_utf8(body.get(p..p + len)?.to_vec()).ok()?);
        p += len;
    }
    Some(out)
}

pub fn encode_root(voice: &[u8; 32], task: impl AsRef<[u8]>, solver: &[u8; 32], ticks: u64) -> Vec<u8> {
    let mut out = vec![ACT_ROOT];
    out.extend_from_slice(voice);
    let _ = put_addr(&mut out, task);
    out.extend_from_slice(solver);
    out.extend_from_slice(&ticks.to_le_bytes());
    out
}

pub fn decode_root(body: &[u8]) -> Option<([u8; 32], Address, [u8; 32], u64)> {
    if !act_is(body, ACT_ROOT) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let mut p = 33usize;
    let task = take_addr(body, &mut p)?;
    let solver: [u8; 32] = body.get(p..p + 32)?.try_into().ok()?;
    p += 32;
    let ticks = u64::from_le_bytes(body.get(p..p + 8)?.try_into().ok()?);
    Some((voice, task, solver, ticks))
}

pub fn encode_root_reply(solver: Option<[u8; 32]>, ticks: u64) -> Vec<u8> {
    let mut out = Vec::new();
    match solver {
        None => out.push(0),
        Some(s) => {
            out.push(1);
            out.extend_from_slice(&s);
        }
    }
    out.extend_from_slice(&ticks.to_le_bytes());
    out
}

pub fn decode_root_reply(body: &[u8]) -> Option<(Option<[u8; 32]>, u64)> {
    let tag = act_tag(body)?;
    let mut p = 1usize;
    let solver = match tag {
        0 => None,
        1 => {
            let s: [u8; 32] = body.get(p..p + 32)?.try_into().ok()?;
            p += 32;
            Some(s)
        }
        _ => return None,
    };
    let ticks = u64::from_le_bytes(body.get(p..p + 8)?.try_into().ok()?);
    Some((solver, ticks))
}

pub fn encode_digests_request(voice: &[u8; 32]) -> Vec<u8> {
    let mut out = vec![ACT_DIGESTS];
    out.extend_from_slice(voice);
    out
}

pub fn decode_digests_request(body: &[u8]) -> Option<[u8; 32]> {
    if !act_is(body, ACT_DIGESTS) || body.len() != 33 {
        return None;
    }
    body.get(1..33)?.try_into().ok()
}

pub fn encode_digests_reply(digests: &[crate::transducer::sheaf::StratumDigest]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&(digests.len() as u32).to_le_bytes());
    for d in digests {
        push_invariant(&mut out, &d.inv);
        out.extend_from_slice(&d.count.to_le_bytes());
        crate::transducer::address::put_bytes(&mut out, &d.root);
    }
    out
}

pub fn decode_digests_reply(body: &[u8]) -> Option<Vec<crate::transducer::sheaf::StratumDigest>> {
    let n = u32::from_le_bytes(body.get(0..4)?.try_into().ok()?) as usize;
    if n > body.len().checked_sub(4)? / 9 {
        return None; // untrusted count: at least invariant + count + root per stratum
    }
    let mut p = 4usize;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let inv = read_invariant(body, &mut p)?;
        let count = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?);
        p += 4;
        let root = crate::transducer::address::take_bytes(body, &mut p)?;
        out.push(crate::transducer::sheaf::StratumDigest { inv, count, root });
    }
    Some(out)
}

/// Bytes the cover/ruled-root signature covers: per-lane cover roots, the
/// ruled root, and the room vector. Never an individual word.
pub fn cover_signed_prefix(cover_roots: &[[u8; 32]; 4], ruled_root: &[u8; 32], room: &[u64; 4]) -> Vec<u8> {
    let mut out = Vec::new();
    for r in cover_roots {
        out.extend_from_slice(r);
    }
    out.extend_from_slice(ruled_root);
    for x in room {
        out.extend_from_slice(&x.to_le_bytes());
    }
    out
}

/// Tail of a digest reply: charts this hypha covers, room, roots, vk, bind, sig.
/// `[n u32]([lane u8][inv])* [room 4·u64][cover_roots 4·32][ruled_root 32][vk 32][alen u8][addr][sig 64]`
pub fn encode_cover_digest(
    charts: &[(u8, crate::transducer::stalk::Invariant)],
    room: &[u64; 4],
    cover_roots: &[[u8; 32]; 4],
    ruled_root: &[u8; 32],
    vk: &[u8; 32],
    addr: &str,
    sig: &[u8; 64],
) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&(charts.len() as u32).to_le_bytes());
    for (lane, inv) in charts {
        out.push(*lane);
        push_invariant(&mut out, inv);
    }
    for x in room {
        out.extend_from_slice(&x.to_le_bytes());
    }
    for r in cover_roots {
        out.extend_from_slice(r);
    }
    out.extend_from_slice(ruled_root);
    out.extend_from_slice(vk);
    let ab = addr.as_bytes();
    out.push(ab.len().min(255) as u8);
    out.extend_from_slice(&ab[..ab.len().min(255)]);
    out.extend_from_slice(sig);
    out
}

/// One cover digest read from `at`, with the offset one past its
/// signature. Reads the fields once; every caller that needs the end
/// offset gets it from the same read, so the two cannot drift apart on a
/// length one of them guessed.
pub(crate) fn cover_digest_at(body: &[u8], at: usize) -> Option<(CoverDigest, usize)> {
    let mut p = at;
    let n = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    if n > 1024 {
        return None;
    }
    let mut charts = Vec::with_capacity(n);
    for _ in 0..n {
        let lane = *body.get(p)?;
        p += 1;
        let inv = read_invariant(body, &mut p)?;
        charts.push((lane, inv));
    }
    let mut room = [0u64; 4];
    for r in &mut room {
        *r = u64::from_le_bytes(body.get(p..p + 8)?.try_into().ok()?);
        p += 8;
    }
    let mut cover_roots = [[0u8; 32]; 4];
    for r in &mut cover_roots {
        *r = body.get(p..p + 32)?.try_into().ok()?;
        p += 32;
    }
    let ruled_root: [u8; 32] = body.get(p..p + 32)?.try_into().ok()?;
    p += 32;
    let vk: [u8; 32] = body.get(p..p + 32)?.try_into().ok()?;
    p += 32;
    let alen = *body.get(p)? as usize;
    p += 1;
    let addr = String::from_utf8(body.get(p..p + alen)?.to_vec()).ok()?;
    p += alen;
    let sig: [u8; 64] = body.get(p..p + 64)?.try_into().ok()?;
    p += 64;
    Some((CoverDigest { charts, room, cover_roots, ruled_root, vk, bind: addr, sig }, p))
}

/// Offset one past a stratum-digest reply at the head of `body`, if the
/// head parses as one. A `Some` here is a CANDIDATE start for the cover
/// digest, never a decision: the counts and lengths of a cover digest can
/// read as a digests reply, and the reply that carries both is the shape
/// `judge` sends, so the two cannot be told apart by looking at the head.
pub(crate) fn digests_reply_end(body: &[u8]) -> Option<usize> {
    let digests = decode_digests_reply(body)?;
    let mut p = 4usize;
    for d in &digests {
        p += 1 + 8 * d.inv.0.len() + 4 + 4 + d.root.len();
    }
    (p < body.len()).then_some(p)
}

/// The whole frame, parsed WHOLE. A body either is a cover digest, or a
/// stratum-digest reply followed by one, and in both cases the parse must
/// consume every byte -- with or without a receipt section on the end.
///
/// The old reader guessed: "skip the stratum-digest prefix if present",
/// deciding by whether the head happened to parse as one. A bare cover
/// digest's own chart count and invariant bytes parse as a digests reply,
/// so the guess mis-read the frame and produced charts that were never
/// sent, silently. Rule 11: the parse that consumes the buffer exactly is
/// the reading, two of them are an ambiguity and refuse, and none refuses.
pub(crate) fn whole_cover_digest(body: &[u8]) -> Option<(CoverDigest, usize)> {
    let mut hit = None;
    let starts = [Some(0usize), digests_reply_end(body)];
    for at in starts.into_iter().flatten() {
        let Some((fields, end)) = cover_digest_at(body, at) else {
            continue;
        };
        let whole = end == body.len() || receipt_section_end(body, end) == Some(body.len());
        if whole {
            if hit.is_some() {
                return None; // two readings consume the buffer: ambiguous
            }
            hit = Some((fields, end));
        }
    }
    hit
}

pub fn decode_cover_digest(body: &[u8]) -> Option<CoverDigest> {
    whole_cover_digest(body).map(|(f, _)| f)
}

/// Offset one past a decoded digest's signature: where a receipt section
/// begins, if one was sent. Comes from the same single reading the fields
/// come from, so no length is computed twice.
pub fn cover_digest_end(body: &[u8]) -> Option<usize> {
    whole_cover_digest(body).map(|(_, e)| e)
}

/// Cover: `[ACT_COVER][vk 32][alen u8][addr][n u32]([lane u8][invariant])*`
pub fn encode_cover(vk: &[u8; 32], addr: &str, charts: &[(u8, crate::transducer::stalk::Invariant)]) -> Vec<u8> {
    let mut out = vec![ACT_COVER];
    out.extend_from_slice(vk);
    let ab = addr.as_bytes();
    out.push(ab.len().min(255) as u8);
    out.extend_from_slice(&ab[..ab.len().min(255)]);
    out.extend_from_slice(&(charts.len() as u32).to_le_bytes());
    for (lane, inv) in charts {
        out.push(*lane);
        push_invariant(&mut out, inv);
    }
    out
}

pub fn decode_cover(body: &[u8]) -> Option<Cover> {
    if !act_is(body, ACT_COVER) {
        return None;
    }
    let vk: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let alen = *body.get(33)? as usize;
    let addr = String::from_utf8(body.get(34..34 + alen)?.to_vec()).ok()?;
    let mut p = 34 + alen;
    let n = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    let mut charts = Vec::with_capacity(n.min(1024));
    for _ in 0..n {
        let lane = *body.get(p)?;
        p += 1;
        let inv = read_invariant(body, &mut p)?;
        charts.push((lane, inv));
    }
    Some(Cover { vk, bind: addr, charts })
}
