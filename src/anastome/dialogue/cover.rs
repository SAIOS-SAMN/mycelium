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

/// Tail of a digest reply: THE ROOTS, room, vk, bind, sig — and no charts.
/// `[room 4·u64][cover_roots 4·32][ruled_root 32][vk 32][alen u8][addr][sig 64]`
///
/// A DIGEST IS ROOTS, AND A COVER IS A PAGE (A4, owed since Stage 8c). The
/// digest carried the whole chart list, so its size was the cover's size:
/// on the fleet at 3d2987e a hypha holding 65 charts could not write one —
/// `charts 65 of which 12 wider than the axis count` — and 438 digests were
/// refused on hypha a in five minutes while no cover propagated. Widening
/// the field would be the same fault one order up (I33). The roots are four
/// hashes and the ruled root a fifth; a peer whose roots MOVED asks for the
/// cover with `ACT_COVER` and reads it a page at a time.
///
/// `None` only where the bind does not fit its length, which is the one
/// remaining variable field and is a configuration refusal (47.1).
pub fn encode_cover_digest(
    room: &[u64; 4],
    cover_roots: &[[u8; 32]; 4],
    ruled_root: &[u8; 32],
    vk: &[u8; 32],
    addr: &str,
    sig: &[u8; 64],
) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    for x in room {
        out.extend_from_slice(&x.to_le_bytes());
    }
    for r in cover_roots {
        out.extend_from_slice(r);
    }
    out.extend_from_slice(ruled_root);
    out.extend_from_slice(vk);
    // A bind address longer than its length field is not shortened to fit:
    // a digest signed over a truncated address names a door that is not
    // this one, and every peer that dialled it would be dialling nothing.
    crate::transducer::address::put_u8_bytes(&mut out, addr.as_bytes())?;
    out.extend_from_slice(sig);
    Some(out)
}

/// One cover digest read from `at`, with the offset one past its
/// signature. Reads the fields once; every caller that needs the end
/// offset gets it from the same read, so the two cannot drift apart on a
/// length one of them guessed.
pub(crate) fn cover_digest_at(body: &[u8], at: usize) -> Option<(CoverDigest, usize)> {
    let mut p = at;
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
    Some((CoverDigest { room, cover_roots, ruled_root, vk, bind: addr, sig }, p))
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
        p += 1 + d.inv.key().as_bytes().len() + 4 + 4 + d.root.len();
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

/// THE ASK FOR A PEER'S COVER, from a cursor:
/// `[ACT_COVER][voice 32][flen u32][from]`.
///
/// The cursor is the lane byte and the axes of the last chart the asker
/// received, so a cover wider than one frame is read across asks and the
/// asker holds no position of its own — the coordinate it sends IS the
/// last chart it holds (rule 15, S13.3).
pub fn encode_cover_request(voice: &[u8; 32], from: &[u8]) -> Vec<u8> {
    let mut out = vec![ACT_COVER];
    out.extend_from_slice(voice);
    crate::transducer::address::put_bytes(&mut out, from);
    out
}

/// `None` when the body is not an ask — which is what a cover REPLY is,
/// since both ride `ACT_COVER` and are told apart by shape.
pub fn decode_cover_request(body: &[u8]) -> Option<([u8; 32], Vec<u8>)> {
    if !act_is(body, ACT_COVER) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let mut p = 33usize;
    let from = crate::transducer::address::take_bytes(body, &mut p)?;
    // An ask ends at its cursor; anything after it is a reply's chart list.
    (p == body.len()).then_some((voice, from))
}

/// Cover, ONE PAGE:
/// `[ACT_COVER][vk 32][alen u8][addr][n u32]([lane u8][invariant])* [more u8]`
///
/// The charts are served in CURSOR ORDER from `from` exclusive, as many as
/// `budget` affords, and `more` says whether the page stopped short of the
/// cover. A cover is not a digest: the digest is the four roots, and this
/// is what a peer reads when they move (A4).
///
/// The rows are written out of the sorted cursors themselves, not out of
/// the caller's list in the caller's order: the cursor a page ends on is
/// the position the next ask resumes from, so an order the server does not
/// control would skip whatever it happened to place before that cursor and
/// serve the rest forever. Canonical key order, rule 15's first entry.
pub fn encode_cover_page(
    vk: &[u8; 32],
    addr: &str,
    charts: &[(u8, crate::transducer::stalk::Invariant)],
    from: &[u8],
    budget: usize,
) -> Option<Vec<u8>> {
    let mut keys: Vec<Vec<u8>> = charts.iter().map(|(lane, inv)| cover_cursor(*lane, inv)).collect();
    keys.sort_unstable();
    keys.dedup();
    let mut out = vec![ACT_COVER];
    out.extend_from_slice(vk);
    crate::transducer::address::put_u8_bytes(&mut out, addr.as_bytes())?;
    let count_at = out.len();
    out.extend_from_slice(&0u32.to_le_bytes());
    let mut served = 0u32;
    let mut more = false;
    for key in &keys {
        if key.as_slice() <= from {
            continue;
        }
        let (lane, chart) = key.split_first()?;
        // A chart whose key does not fit the length byte is PASSED OVER,
        // not refused with the page: the asker resumes from the last cursor
        // it read, so a refusal here would meet the same chart as the head
        // of every later ask and the tail of the cover would never be
        // served. The field is not widened either (I33) -- the door names
        // the width off its own charts.
        //
        // The width is the KEY'S, in bytes. It was `axes.len() / 8`, a
        // count of `u64` limbs, which stopped being a count of coordinates
        // when a coordinate became whole (A21 as ruled 2026-09-18) and
        // passed over every chart rather than serving it.
        if chart.len() > u8::MAX as usize {
            continue;
        }
        // One chart's width on the wire, exact: the lane byte, the key's
        // length byte, and the key -- which is what `read_invariant` reads.
        if out.len() + 2 + chart.len() + 1 > budget {
            more = true;
            break;
        }
        out.push(*lane);
        out.push(chart.len() as u8);
        out.extend_from_slice(chart);
        served = served.checked_add(1)?;
    }
    out.get_mut(count_at..count_at + 4)?.copy_from_slice(&served.to_le_bytes());
    out.push(u8::from(more));
    Some(out)
}

/// The cursor a chart is read back from: its lane, then its axes. The
/// asker sends the last one it holds and the server continues past it, so
/// neither side keeps a position (S13.3).
pub fn cover_cursor(lane: u8, inv: &crate::transducer::stalk::Invariant) -> Vec<u8> {
    // The lane, then the chart's own key: a cursor is a coordinate, so it
    // is written the way every other chart coordinate is (law 2).
    let mut out = vec![lane];
    out.extend_from_slice(inv.key().as_bytes());
    out
}

/// One page of a peer's cover, and whether more of it stands. `None` for
/// an ask, which rides the same act.
pub fn decode_cover_page(body: &[u8]) -> Option<(Cover, bool)> {
    if !act_is(body, ACT_COVER) {
        return None;
    }
    let vk: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let alen = *body.get(33)? as usize;
    let addr = String::from_utf8(body.get(34..34 + alen)?.to_vec()).ok()?;
    let mut p = 34 + alen;
    let n = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    if n > body.len().checked_sub(p)? / 2 {
        return None; // untrusted count: a lane byte and an axis count at least
    }
    let mut charts = Vec::with_capacity(n);
    for _ in 0..n {
        let lane = *body.get(p)?;
        p += 1;
        let inv = read_invariant(body, &mut p)?;
        charts.push((lane, inv));
    }
    let more = match *body.get(p)? {
        0 => false,
        1 => true,
        _ => return None,
    };
    Some((Cover { vk, bind: addr, charts }, more))
}
