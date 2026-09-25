// ACT_QUOTE — "what is the highest-yield work?", and the outcome reply
// every judged act answers with.
//
//! The reply is bounded by the frame budget: items ride in rank order until
//! the next would not fit, and the count is written where the reader can
//! find it without parsing the whole body.

use super::*;

/// `[ACT_QUOTE][voice 32][n u8][invariants…]` — the question, carrying the
/// charts of the voice's own atlas so the hypha can answer by incidence.
pub fn encode_quote_request(voice: &[u8; 32]) -> Option<Vec<u8>> {
    encode_quote_request_charts(voice, &[])
}

pub fn encode_quote_request_charts(
    voice: &[u8; 32],
    charts: &[crate::transducer::stalk::Invariant],
) -> Option<Vec<u8>> {
    let mut out = vec![ACT_QUOTE];
    out.extend_from_slice(voice);
    crate::transducer::address::put_u8_count(&mut out, charts.len())?;
    for inv in charts {
        push_invariant(&mut out, inv)?;
        out.push(0); // Ruled unless the caller uses encode_quote_request_standing
    }
    Some(out)
}

/// Quote request with the standing of each chart the mind rests on.
pub fn encode_quote_request_standing(
    voice: &[u8; 32],
    charts: &[(crate::transducer::stalk::Invariant, u8)],
) -> Option<Vec<u8>> {
    let mut out = vec![ACT_QUOTE];
    out.extend_from_slice(voice);
    crate::transducer::address::put_u8_count(&mut out, charts.len())?;
    for (inv, st) in charts {
        push_invariant(&mut out, inv)?;
        out.push(*st);
    }
    Some(out)
}

pub fn decode_quote_request(body: &[u8]) -> Option<([u8; 32], Vec<crate::transducer::stalk::Invariant>)> {
    if !act_is(body, ACT_QUOTE) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    if body.len() == 33 {
        return Some((voice, Vec::new()));
    }
    let n = *body.get(33)? as usize;
    let mut p = 34usize;
    let mut charts = Vec::with_capacity(n);
    for _ in 0..n {
        charts.push(read_invariant(body, &mut p)?);
        let _standing = *body.get(p)?;
        p += 1;
    }
    Some((voice, charts))
}


pub fn encode_quote_reply(
    height: u64,
    surface: &[u64; 4],
    k: u64,
    items: &[WorkItem],
) -> Option<Vec<u8>> {
    encode_quote_reply_within(height, surface, k, items, usize::MAX)
}

/// Demand/percept reply: `[code][height u64][pressure 4×u64][taught_len u16][taught…]`.
/// `None` where the taught bytes pass their two-byte length (STD1 3c).
pub fn encode_outcome_reply(code: u8, height: u64, pressure: &[u64; 4], taught: &[u8]) -> Option<Vec<u8>> {
    let mut body = outcome_head(code, height, pressure);
    crate::transducer::address::put_u16_count(&mut body, taught.len())?;
    body.extend_from_slice(taught);
    Some(body)
}

/// The same reply with nothing taught: total, since an empty field fits.
pub fn encode_outcome(code: u8, height: u64, pressure: &[u64; 4]) -> Vec<u8> {
    let mut body = outcome_head(code, height, pressure);
    body.extend_from_slice(&0u16.to_le_bytes());
    body
}

fn outcome_head(code: u8, height: u64, pressure: &[u64; 4]) -> Vec<u8> {
    let mut body = vec![code];
    body.extend_from_slice(&height.to_le_bytes());
    push_mu(&mut body, pressure);
    body
}

/// The deferral a full judge answers every act with (23.8, ruled
/// 2026-09-09): the pending outcome, no height, the door's per-lane reserve
/// deficit as its four lanes, nothing taught. For a percept or a demand it
/// is a verdict; for any other act it is impedance; either way the asker
/// dwells on the lanes it carries and never counts deferrals.
pub fn encode_deferral(code: u8, deficit: &[u64; 4]) -> Vec<u8> {
    encode_outcome(code, 0, deficit)
}

/// THE OUTCOME REPLY READ BACK — the one reader of the layout
/// `encode_outcome_reply` writes: the code, the height, the four lanes,
/// what was taught, and whatever the sender appended after it (a settle
/// receipt). A frame shorter than it names refuses; nothing is defaulted.
pub struct OutcomeReply<'a> {
    pub code: u8,
    pub height: u64,
    pub pressure: [u64; 4],
    pub taught: &'a [u8],
    pub rest: &'a [u8],
}

pub fn decode_outcome_reply(body: &[u8]) -> Option<OutcomeReply<'_>> {
    let code = act_tag(body)?;
    let mut p = 1usize;
    let height = read_u64(body, &mut p)?;
    let pressure = read_mu(body, &mut p)?;
    let n = read_u16(body, &mut p)? as usize;
    let taught = body.get(p..p + n)?;
    let rest = body.get(p + n..)?;
    Some(OutcomeReply { code, height, pressure, taught, rest })
}

/// The deferral's shape, read through the codec: the pending code, no
/// height, four lanes of any value, nothing taught, nothing after. A
/// pending outcome that carries a height is an answer.
pub fn deferral_lanes(body: &[u8], code: u8) -> Option<[u64; 4]> {
    let r = decode_outcome_reply(body)?;
    (r.code == code && r.height == 0 && r.taught.is_empty() && r.rest.is_empty()).then_some(r.pressure)
}

pub fn is_deferral(body: &[u8], code: u8) -> bool {
    deferral_lanes(body, code).is_some()
}

/// The quote reply filled up to `budget` bytes: items ride in rank order
/// until the next would not fit. The frame budget decides what a contact
/// can carry; a wide book is served across contacts, never as a frame the
/// wire refuses.
pub fn encode_quote_reply_within(height: u64, surface: &[u64; 4], k: u64, items: &[WorkItem], budget: usize) -> Option<Vec<u8>> {
    let mut out = vec![ACT_QUOTE];
    out.extend_from_slice(&height.to_le_bytes());
    push_mu(&mut out, surface);
    out.extend_from_slice(&k.to_le_bytes());
    let count_at = out.len();
    out.push(0);
    // THE COUNT IS WHAT WAS SERVED, and the budget below is what stops the
    // page — so the served count is bounded by the frame, not by the
    // field. A page whose own count overflows the field is refused with
    // the rest of the reply rather than sent naming fewer items than it
    // carries.
    let mut served = 0u8;
    for WorkItem { addr, mu, element, open } in items {
        let open_len: usize = 1 + open.iter().map(|i| 1 + 8 * i.0.len()).sum::<usize>();
        let item_len = 2 + addr.len() + 32 + 4 + element.len() + open_len;
        if out.len() + item_len > budget {
            break;
        }
        put_addr(&mut out, addr)?;
        push_mu(&mut out, mu);
        // The listing whole, length-prefixed as a chain's links are.
        crate::transducer::address::put_bytes(&mut out, element)?;
        crate::transducer::address::put_u8_count(&mut out, open.len())?;
        for inv in open {
            push_invariant(&mut out, inv)?;
        }
        served = served.checked_add(1)?;
    }
    *out.get_mut(count_at)? = served;
    Some(out)
}

pub fn decode_quote_reply(body: &[u8]) -> Option<(u64, [u64; 4], u64, Vec<WorkItem>)> {
    if !act_is(body, ACT_QUOTE) {
        return None;
    }
    let mut p = 1usize;
    let height = u64::from_le_bytes(body.get(p..p + 8)?.try_into().ok()?);
    p += 8;
    let surface = read_mu(body, &mut p)?;
    let k = u64::from_le_bytes(body.get(p..p + 8)?.try_into().ok()?);
    p += 8;
    let n = *body.get(p)? as usize; // u8 — bounded by construction
    p += 1;
    let mut items = Vec::with_capacity(n);
    for _ in 0..n {
        let addr = take_addr(body, &mut p)?;
        let mu = read_mu(body, &mut p)?;
        // The listing whole, refused before it is allocated if the count
        // does not fit what is left (an untrusted length, as everywhere).
        let tlen = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
        let element = body.get(p..p + tlen)?.to_vec();
        p += tlen;
        let nopen = *body.get(p)? as usize;
        p += 1;
        let mut open = Vec::with_capacity(nopen);
        for _ in 0..nopen {
            open.push(read_invariant(body, &mut p)?);
        }
        items.push(WorkItem { addr, mu, element, open });
    }
    Some((height, surface, k, items))
}
