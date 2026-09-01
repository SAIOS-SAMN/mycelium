// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE 5-ACT DIALOGUE — the closed ANASTOME speech-act algebra:
//!   percept — a sensory cochain broadcast (∂₁Q: the framed scene bodies).
//!   demand  — an unsolved coboundary challenge (an irreducible centered chain
//!             seeking inverse reduction / projective decomposition).
//!   quote   — "what is the highest-yield work?" answered by the yield surface
//!             read from genuine metabolic tension, with the open-demand market
//!             priced per lane (μ⃗).
//!   teach   — canonical element/sentence transmission across the nerve.
//!   settle  — the two-voice consilience receipt (δσ = 0 witnessed) carrying the
//!             polytonic μ⃗ award.
//! Every payload is exact geometry/algebra — chains, addresses, μ⃗ vectors. No
//! English, no semantic tags, no solver hints. Routing state is Vec-only
//! (Law II). The percept codec itself lives in `cognition` (this module owns the
//! act tags and the non-percept payloads).

/// Act tags — frame `body[0]`. ACT_PERCEPT/ACT_GOSSIP keep their historical
/// values so existing kernels speak unchanged.
pub const ACT_PERCEPT: u8 = 0x00;
pub const ACT_GOSSIP: u8 = 0x01;
pub const ACT_DEMAND: u8 = 0x02;
pub const ACT_QUOTE: u8 = 0x03;
pub const ACT_TEACH: u8 = 0x04;
pub const ACT_SETTLE: u8 = 0x05;

/// One centered coboundary cell `(row, col, before, after)`.
pub type Cell = (i64, i64, u8, u8);

fn push_chain(out: &mut Vec<u8>, chain: &[Cell]) {
    out.extend_from_slice(&(chain.len() as u32).to_le_bytes());
    for (r, c, before, after) in chain {
        out.extend_from_slice(&r.to_le_bytes());
        out.extend_from_slice(&c.to_le_bytes());
        out.push(*before);
        out.push(*after);
    }
}

fn read_chain(b: &[u8], p: &mut usize) -> Option<Vec<Cell>> {
    let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
    *p += 4;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let r = i64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        let c = i64::from_le_bytes(b.get(*p + 8..*p + 16)?.try_into().ok()?);
        let before = *b.get(*p + 16)?;
        let after = *b.get(*p + 17)?;
        *p += 18;
        out.push((r, c, before, after));
    }
    Some(out)
}

fn push_mu(out: &mut Vec<u8>, mu: &[u64; 4]) {
    for v in mu {
        out.extend_from_slice(&v.to_le_bytes());
    }
}

fn read_mu(b: &[u8], p: &mut usize) -> Option<[u64; 4]> {
    let mut mu = [0u64; 4];
    for v in &mut mu {
        *v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        *p += 8;
    }
    Some(mu)
}

// ── demand: an unsolved coboundary challenge ─────────────────────────────────

/// `[ACT_DEMAND][voice 32][chain]`
pub fn encode_demand(voice: &[u8; 32], chain: &[Cell]) -> Vec<u8> {
    let mut out = vec![ACT_DEMAND];
    out.extend_from_slice(voice);
    push_chain(&mut out, chain);
    out
}

pub fn decode_demand(body: &[u8]) -> Option<([u8; 32], Vec<Cell>)> {
    if body.first() != Some(&ACT_DEMAND) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let mut p = 33usize;
    let chain = read_chain(body, &mut p)?;
    Some((voice, chain))
}

// ── quote: the yield question and the quoted surface ─────────────────────────

/// `[ACT_QUOTE][voice 32]` — the question ("what is the highest-yield work?").
pub fn encode_quote_request(voice: &[u8; 32]) -> Vec<u8> {
    let mut out = vec![ACT_QUOTE];
    out.extend_from_slice(voice);
    out
}

pub fn decode_quote_request(body: &[u8]) -> Option<[u8; 32]> {
    if body.first() != Some(&ACT_QUOTE) {
        return None;
    }
    body.get(1..33)?.try_into().ok()
}

/// One quoted work item: the residual's address, its priced μ⃗, and the chain
/// itself (so a kernel can actually attempt the reduction).
pub type WorkItem = ([u8; 32], [u64; 4], Vec<Cell>);

/// `[ACT_QUOTE][surface 4×u64][n u8][item: addr 32 | μ⃗ 4×u64 | chain]…`
pub fn encode_quote_reply(surface: &[u64; 4], items: &[WorkItem]) -> Vec<u8> {
    let mut out = vec![ACT_QUOTE];
    push_mu(&mut out, surface);
    out.push(items.len().min(255) as u8);
    for (addr, mu, chain) in items.iter().take(255) {
        out.extend_from_slice(addr);
        push_mu(&mut out, mu);
        push_chain(&mut out, chain);
    }
    out
}

pub fn decode_quote_reply(body: &[u8]) -> Option<([u64; 4], Vec<WorkItem>)> {
    if body.first() != Some(&ACT_QUOTE) {
        return None;
    }
    let mut p = 1usize;
    let surface = read_mu(body, &mut p)?;
    let n = *body.get(p)? as usize;
    p += 1;
    let mut items = Vec::with_capacity(n);
    for _ in 0..n {
        let addr: [u8; 32] = body.get(p..p + 32)?.try_into().ok()?;
        p += 32;
        let mu = read_mu(body, &mut p)?;
        let chain = read_chain(body, &mut p)?;
        items.push((addr, mu, chain));
    }
    Some((surface, items))
}

// ── teach: canonical element transmission ────────────────────────────────────

/// `[ACT_TEACH][len u32][element bytes]`
pub fn encode_teach(element_bytes: &[u8]) -> Vec<u8> {
    let mut out = vec![ACT_TEACH];
    out.extend_from_slice(&(element_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(element_bytes);
    out
}

pub fn decode_teach(body: &[u8]) -> Option<Vec<u8>> {
    if body.first() != Some(&ACT_TEACH) {
        return None;
    }
    let len = u32::from_le_bytes(body.get(1..5)?.try_into().ok()?) as usize;
    Some(body.get(5..5 + len)?.to_vec())
}

// ── settle: the two-voice consilience receipt ────────────────────────────────

/// `[ACT_SETTLE][germ 32][nvoices u8][voices 32×n][μ⃗ 4×u64]`
pub fn encode_settle(germ: &[u8; 32], voices: &[[u8; 32]], mu: &[u64; 4]) -> Vec<u8> {
    let mut out = vec![ACT_SETTLE];
    out.extend_from_slice(germ);
    out.push(voices.len().min(255) as u8);
    for v in voices.iter().take(255) {
        out.extend_from_slice(v);
    }
    push_mu(&mut out, mu);
    out
}

pub fn decode_settle(body: &[u8]) -> Option<([u8; 32], Vec<[u8; 32]>, [u64; 4])> {
    if body.first() != Some(&ACT_SETTLE) {
        return None;
    }
    let germ: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let n = *body.get(33)? as usize;
    let mut p = 34usize;
    let mut voices = Vec::with_capacity(n);
    for _ in 0..n {
        let v: [u8; 32] = body.get(p..p + 32)?.try_into().ok()?;
        p += 32;
        voices.push(v);
    }
    let mu = read_mu(body, &mut p)?;
    Some((germ, voices, mu))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every act round-trips exactly — the calculus is closed and lossless.
    #[test]
    fn the_five_acts_round_trip() {
        let chain = vec![(0i64, 0i64, 1u8, 4u8), (2, 3, 0, 7)];
        let (voice, germ) = ([9u8; 32], [7u8; 32]);
        let (v, c) = decode_demand(&encode_demand(&voice, &chain)).unwrap();
        assert_eq!((v, c), (voice, chain.clone()));
        assert_eq!(decode_quote_request(&encode_quote_request(&voice)).unwrap(), voice);
        let items = vec![(germ, [1u64, 2, 30, 4], chain.clone())];
        let (s, it) = decode_quote_reply(&encode_quote_reply(&[5, 0, 9, 0], &items)).unwrap();
        assert_eq!(s, [5, 0, 9, 0]);
        assert_eq!(it, items);
        assert_eq!(decode_teach(&encode_teach(&[0xAB, 0xCD])).unwrap(), vec![0xAB, 0xCD]);
        let (g2, vs, mu) = decode_settle(&encode_settle(&germ, &[voice, germ], &[0, 0, 12, 0])).unwrap();
        assert_eq!((g2, vs, mu), (germ, vec![voice, germ], [0, 0, 12, 0]));
    }
}
