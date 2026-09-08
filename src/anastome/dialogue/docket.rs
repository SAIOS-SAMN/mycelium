// ACT_DOCKET, ACT_EVIDENCE, ACT_VERDICT, ACT_TEACH, ACT_SETTLE — the
// court's own acts: what is open, what was deposited against it, how it was
// ruled, what is taught from it, and the consilience receipt.

use super::*;

/// `[ACT_TEACH][len u32][element bytes]`
/// docket request: `[ACT_DOCKET][voice 32]`.
/// docket request: `[ACT_DOCKET][voice 32][since K u64][ticks u64]` — the
/// court states where it stands on this hypha's chain (the last k-step
/// it heard) and its own TUniversal clock; the hypha serves what lies
/// past that point and prunes what lies before it.
pub fn encode_docket_request(voice: &[u8; 32], since_k: u64, ticks: u64) -> Vec<u8> {
    let mut out = vec![ACT_DOCKET];
    out.extend_from_slice(voice);
    out.extend_from_slice(&since_k.to_le_bytes());
    out.extend_from_slice(&ticks.to_le_bytes());
    out
}

pub fn decode_docket_request(body: &[u8]) -> Option<([u8; 32], u64, u64)> {
    if body.first() != Some(&ACT_DOCKET) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let since = u64::from_le_bytes(body.get(33..41)?.try_into().ok()?);
    let ticks = u64::from_le_bytes(body.get(41..49)?.try_into().ok()?);
    Some((voice, since, ticks))
}

pub fn encode_docket_reply(dockets: &[Docket]) -> Vec<u8> {
    encode_docket_reply_with(dockets, &[])
}

/// `[dockets…][ns u32] ( [listing 32][chain][voice 32][cites][μ⃗ before 32][μ⃗ after 32] )…`
pub fn encode_docket_reply_with(dockets: &[Docket], settlements: &[Settlement]) -> Vec<u8> {
    encode_docket_reply_paged(dockets, settlements, &[], usize::MAX).0
}

/// THE DOCKET PAGE: words first, then settlements, then cavities, each
/// taken in order until the next would not fit the budget. Returns the
/// body and how many words, settlements, and cavities it carries, so the
/// server drains exactly what the court will hear and serves the rest
/// on the next pull.
pub fn encode_docket_reply_paged(
    dockets: &[Docket],
    settlements: &[Settlement],
    cavities: &[Vec<u8>],
    budget: usize,
) -> (Vec<u8>, usize, usize, usize) {
    // Settlements are the proof of convergence and bounded by the reserve;
    // they are measured first, and the words take what remains, so a long
    // provisional list never starves the court of what actually settled.
    let mut settled: Vec<Vec<u8>> = Vec::new();
    let mut settled_len = 8usize;
    for s in settlements {
        let mut item = Vec::new();
        let _ = put_addr(&mut item, &s.listing);
        push_chain(&mut item, &s.chain);
        item.extend_from_slice(&s.voice);
        push_cites(&mut item, &s.cites);
        push_mu(&mut item, &s.before);
        push_mu(&mut item, &s.after);
        item.extend_from_slice(&s.k.to_le_bytes());
        item.extend_from_slice(&s.t_universal.to_le_bytes());
        if settled_len + item.len() > budget {
            break;
        }
        settled_len += item.len();
        settled.push(item);
    }
    let mut out = Vec::new();
    out.extend_from_slice(&0u32.to_le_bytes());
    let mut words = 0usize;
    for d in dockets {
        let mut item = Vec::with_capacity(112 + d.elem.len() + 32 * d.voices.len());
        let _ = put_addr(&mut item, &d.addr);
        item.extend_from_slice(&(d.elem.len() as u32).to_le_bytes());
        item.extend_from_slice(&d.elem);
        item.extend_from_slice(&(d.voices.len() as u32).to_le_bytes());
        for v in &d.voices {
            item.extend_from_slice(v);
        }
        push_mu(&mut item, &d.escrowed);
        push_mu(&mut item, &d.deflation);
        item.extend_from_slice(&d.k.to_le_bytes());
        if out.len() + item.len() + settled_len > budget {
            break;
        }
        out.extend_from_slice(&item);
        words += 1;
    }
    out[..4].copy_from_slice(&(words as u32).to_le_bytes());
    out.extend_from_slice(&(settled.len() as u32).to_le_bytes());
    let served = settled.len();
    for item in settled {
        out.extend_from_slice(&item);
    }
    // Cavities: evicted unreachable listings, each the element itself.
    let cav_at = out.len();
    out.extend_from_slice(&0u32.to_le_bytes());
    let mut ncav = 0u32;
    for elem in cavities {
        if out.len() + 4 + elem.len() > budget {
            break;
        }
        out.extend_from_slice(&(elem.len() as u32).to_le_bytes());
        out.extend_from_slice(elem);
        ncav += 1;
    }
    out[cav_at..cav_at + 4].copy_from_slice(&ncav.to_le_bytes());
    (out, words, served, ncav as usize)
}

/// The settlements on a docket reply, after the dockets (absent in an
/// older reply: none).
pub fn decode_settlements(body: &[u8]) -> Vec<Settlement> {
    let Some((_, mut p)) = decode_dockets_at(body) else { return Vec::new() };
    let Some(n) = body.get(p..p + 4).and_then(|b| b.try_into().ok()).map(|b| u32::from_le_bytes(b) as usize) else { return Vec::new() };
    p += 4;
    let mut out = Vec::new();
    for _ in 0..n {
        let Some(listing) = take_addr(body, &mut p) else { break };
        let Some(chain) = read_chain(body, &mut p) else { break };
        let Some(voice) = body.get(p..p + 32).and_then(|b| b.try_into().ok()) else { break };
        p += 32;
        let Some(cites) = read_cites(body, &mut p) else { break };
        let Some(before) = read_mu(body, &mut p) else { break };
        let Some(after) = read_mu(body, &mut p) else { break };
        let Some(k) = body.get(p..p + 8).and_then(|b| b.try_into().ok()).map(u64::from_le_bytes) else { break };
        p += 8;
        let Some(tu) = body.get(p..p + 8).and_then(|b| b.try_into().ok()).map(u64::from_le_bytes) else { break };
        p += 8;
        out.push(Settlement { listing, chain, voice, cites, before, after, k, t_universal: tu });
    }
    out
}

/// Cavities riding after the settlements: the evicted element's bytes.
/// Absent on an older reply.
pub fn decode_cavities(body: &[u8]) -> Vec<Vec<u8>> {
    let Some((_, mut p)) = decode_dockets_at(body) else { return Vec::new() };
    let Some(ns) = body.get(p..p + 4).and_then(|b| b.try_into().ok()).map(|b| u32::from_le_bytes(b) as usize) else {
        return Vec::new();
    };
    p += 4;
    for _ in 0..ns {
        if take_addr(body, &mut p).is_none() {
            return Vec::new();
        }
        let Some(_) = read_chain(body, &mut p) else { return Vec::new() };
        p += 32;
        let Some(_) = read_cites(body, &mut p) else { return Vec::new() };
        let Some(_) = read_mu(body, &mut p) else { return Vec::new() };
        let Some(_) = read_mu(body, &mut p) else { return Vec::new() };
        p += 8;
        if p + 8 <= body.len() {
            p += 8;
        }
    }
    let Some(n) = body.get(p..p + 4).and_then(|b| b.try_into().ok()).map(|b| u32::from_le_bytes(b) as usize) else {
        return Vec::new();
    };
    p += 4;
    let mut out = Vec::with_capacity(n.min(64));
    for _ in 0..n {
        let Some(el) = body.get(p..p + 4).and_then(|b| b.try_into().ok()).map(|b| u32::from_le_bytes(b) as usize) else {
            break;
        };
        p += 4;
        let Some(bytes) = body.get(p..p + el) else { break };
        p += el;
        out.push(bytes.to_vec());
    }
    out
}

pub fn decode_docket_reply(body: &[u8]) -> Option<Vec<Docket>> {
    decode_dockets_at(body).map(|(d, _)| d)
}

pub(crate) fn decode_dockets_at(body: &[u8]) -> Option<(Vec<Docket>, usize)> {
    let mut p = 0usize;
    let n = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    if n > body.len().saturating_sub(p) / 74 {
        return None; // untrusted count: address + element + voices + μ⃗ pair + k
    }
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let addr = take_addr(body, &mut p)?;
        let el = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
        if el > body.len().saturating_sub(p) {
            return None;
        }
        let elem = body.get(p..p + el)?.to_vec();
        p += el;
        let nv = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
        if nv > body.len().saturating_sub(p) / 32 {
            return None;
        }
        let mut voices = Vec::with_capacity(nv);
        for _ in 0..nv {
            voices.push(body.get(p..p + 32)?.try_into().ok()?);
            p += 32;
        }
        let mu = read_mu(body, &mut p)?;
        let deflation = read_mu(body, &mut p)?;
        let k = u64::from_le_bytes(body.get(p..p + 8)?.try_into().ok()?);
        p += 8;
        out.push(Docket { addr, elem, voices, escrowed: mu, deflation, k });
    }
    Some((out, p))
}

/// evidence request: `[ACT_EVIDENCE][voice 32]`.
pub fn encode_evidence_request(voice: &[u8; 32]) -> Vec<u8> {
    let mut out = vec![ACT_EVIDENCE];
    out.extend_from_slice(voice);
    out
}

pub fn decode_evidence_request(body: &[u8]) -> Option<[u8; 32]> {
    if body.first() != Some(&ACT_EVIDENCE) || body.len() != 33 {
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
) -> Vec<u8> {
    let mut out = vec![ACT_EVIDENCE];
    out.extend_from_slice(voice);
    out.extend_from_slice(&encode_evidence_reply(items));
    out
}

pub fn decode_evidence_deposit(body: &[u8]) -> Option<EvidenceDeposit> {
    if body.first() != Some(&ACT_EVIDENCE) || body.len() <= 33 {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let items = decode_evidence_reply(&body[33..])?;
    Some((voice, items))
}

/// evidence reply: `[n u32] ( [addr][nf u32][fp (r i64,c i64)·nf]
/// [ns u32][scene (r i64,c i64,h u8)·ns] )…` — the selection fiber's birth
/// evidence, count-guarded at every level.
pub fn encode_evidence_reply<A: AsRef<[u8]>>(
    items: &[EvidenceOf<A>],
) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&(items.len() as u32).to_le_bytes());
    for EvidenceOf { addr, footprint, scene } in items {
        let _ = put_addr(&mut out, addr);
        out.extend_from_slice(&(footprint.len() as u32).to_le_bytes());
        for (r, c) in footprint {
            out.extend_from_slice(&r.to_le_bytes());
            out.extend_from_slice(&c.to_le_bytes());
        }
        out.extend_from_slice(&(scene.len() as u32).to_le_bytes());
        for (r, c, h) in scene {
            out.extend_from_slice(&r.to_le_bytes());
            out.extend_from_slice(&c.to_le_bytes());
            out.push(*h);
        }
    }
    out
}

pub fn decode_evidence_reply(body: &[u8]) -> Option<Vec<EvidenceItem>> {
    let mut p = 0usize;
    let n = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    if n > body.len().saturating_sub(p) / 10 {
        return None; // untrusted count
    }
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let addr = take_addr(body, &mut p)?;
        let nf = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
        if nf > body.len().saturating_sub(p) / 16 {
            return None;
        }
        let mut fp = Vec::with_capacity(nf);
        for _ in 0..nf {
            let r = i64::from_le_bytes(body.get(p..p + 8)?.try_into().ok()?);
            let c = i64::from_le_bytes(body.get(p + 8..p + 16)?.try_into().ok()?);
            p += 16;
            fp.push((r, c));
        }
        let ns = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
        if ns > body.len().saturating_sub(p) / 17 {
            return None;
        }
        let mut scene = Vec::with_capacity(ns);
        for _ in 0..ns {
            let r = i64::from_le_bytes(body.get(p..p + 8)?.try_into().ok()?);
            let c = i64::from_le_bytes(body.get(p + 8..p + 16)?.try_into().ok()?);
            let h = *body.get(p + 16)?;
            p += 17;
            scene.push((r, c, h));
        }
        out.push(EvidenceOf { addr, footprint: fp, scene });
    }
    Some(out)
}

/// verdict: `[ACT_VERDICT][court voice 32][addr][ruling u8][receipt 32]
/// [payload tag …][47.1 receipt section]`.
///
/// The `receipt` field is the head of the court's own chronometric chain,
/// which the ruling binds to; the trailing section is 47.1 whole — the
/// signed prefix with the ruled ADDRESS as its payload, the signature over
/// `prefix.bytes()` and nothing else, and the receipt the prefix names, so
/// the receiver can run all four steps of 47.2 before touching a carrier.
pub fn encode_verdict(
    voice: &[u8; 32],
    addr: impl AsRef<[u8]>,
    ruling: u8,
    receipt: &[u8; 32],
    section: &ReceiptSection,
) -> Option<Vec<u8>> {
    encode_verdict_payload(voice, addr, ruling, receipt, None, section)
}

/// With the optional π* payload (RULING_REBIND): `…[receipt 32][1][sel]` /
/// `…[receipt 32][0]`, then the receipt section. `None` when the section
/// cannot be framed (`encode_receipt_section`): a ruling without its
/// signed position is not spoken.
pub fn encode_verdict_payload(
    voice: &[u8; 32],
    addr: impl AsRef<[u8]>,
    ruling: u8,
    receipt: &[u8; 32],
    pi: Option<&crate::transducer::anchor::Sel>,
    section: &ReceiptSection,
) -> Option<Vec<u8>> {
    let mut out = vec![ACT_VERDICT];
    out.extend_from_slice(voice);
    let _ = put_addr(&mut out, addr);
    out.push(ruling);
    out.extend_from_slice(receipt);
    match pi {
        Some(t) => {
            out.push(1);
            t.to_bytes(&mut out);
        }
        None => out.push(0),
    }
    out.extend_from_slice(&super::encode_receipt_section(
        &section.prefix,
        &section.sig,
        &section.receipt,
    )?);
    Some(out)
}

/// The attach payload (RULING_ATTACHED): `…[receipt 32][2][base][g 17]` —
/// the base the word connects to and the transport g into its frame.
pub fn encode_verdict_attach(
    voice: &[u8; 32],
    addr: impl AsRef<[u8]>,
    receipt: &[u8; 32],
    base: impl AsRef<[u8]>,
    g: &crate::transducer::element::Versor,
    section: &ReceiptSection,
) -> Option<Vec<u8>> {
    let mut out = vec![ACT_VERDICT];
    out.extend_from_slice(voice);
    let _ = put_addr(&mut out, addr);
    out.push(RULING_ATTACHED);
    out.extend_from_slice(receipt);
    out.push(2);
    let _ = put_addr(&mut out, base);
    g.to_bytes(&mut out);
    out.extend_from_slice(&super::encode_receipt_section(
        &section.prefix,
        &section.sig,
        &section.receipt,
    )?);
    Some(out)
}


pub fn decode_verdict(body: &[u8]) -> Option<Verdict> {
    if body.first() != Some(&ACT_VERDICT) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let mut p = 33usize;
    let addr = take_addr(body, &mut p)?;
    let ruling = *body.get(p)?;
    p += 1;
    let receipt: [u8; 32] = body.get(p..p + 32)?.try_into().ok()?;
    p += 32;
    // THE PAYLOAD TAG IS ONE BYTE ON EVERY ARM, and every arm consumes it.
    // The no-payload arm did not: `encode_verdict_payload` writes `push(0)`
    // for it, so `p` was left pointing AT that zero and the receipt section
    // was then read from one byte early — its `plen` came out of
    // `[0x00, first-byte-of-plen]`. RULING_UPHELD and RULING_SLASHED carry
    // no payload, so the two ordinary rulings never decoded at all: the
    // door returned before answering and the ruling court blocked on a
    // reply that never came. REBIND and ATTACH parsed, which is why nothing
    // in the tree noticed. A tag that is written on three arms and read on
    // two is not a layout, and `0` is refused rather than ignored.
    let (pi, attach) = match body.get(p) {
        Some(0) => {
            p += 1;
            (None, None)
        }
        Some(1) => {
            p += 1;
            (Some(crate::transducer::anchor::Sel::from_bytes(body, &mut p)?), None)
        }
        Some(2) => {
            p += 1;
            let base = take_addr(body, &mut p)?;
            let g = crate::transducer::element::Versor::from_bytes(body, &mut p)?;
            (None, Some((base, g)))
        }
        _ => return None,
    };
    let section = super::decode_receipt_section(body, p)?;
    Some(Verdict { voice, addr, ruling, receipt, pi, attach, section })
}

/// The receiver's answer to a ruling: `[arm u8][payload]` — `0` applied,
/// `1` inert, `2` refused with `len16 + reason`, `3` gap with two `u64`.
/// It was one byte carrying a bool.
pub fn encode_ruled(r: &Ruled) -> Vec<u8> {
    let mut out = Vec::new();
    match r {
        Ruled::Applied => out.push(0),
        Ruled::Inert => out.push(1),
        Ruled::Refused(why) => {
            out.push(2);
            out.extend_from_slice(&(why.len() as u16).to_le_bytes());
            out.extend_from_slice(why);
        }
        Ruled::Gap { from, to } => {
            out.push(3);
            out.extend_from_slice(&from.to_le_bytes());
            out.extend_from_slice(&to.to_le_bytes());
        }
    }
    out
}

pub fn decode_ruled(body: &[u8]) -> Option<Ruled> {
    match *body.first()? {
        0 => Some(Ruled::Applied),
        1 => Some(Ruled::Inert),
        2 => {
            let n = u16::from_le_bytes(body.get(1..3)?.try_into().ok()?) as usize;
            Some(Ruled::Refused(body.get(3..3 + n)?.to_vec()))
        }
        3 => Some(Ruled::Gap {
            from: u64::from_le_bytes(body.get(1..9)?.try_into().ok()?),
            to: u64::from_le_bytes(body.get(9..17)?.try_into().ok()?),
        }),
        // An arm the layout does not define is refused, never guessed at.
        _ => None,
    }
}

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

/// `[ACT_SETTLE][germ address][nvoices u8][voices 32×n][μ⃗ 4×u64]`
pub fn encode_settle(germ: impl AsRef<[u8]>, voices: &[[u8; 32]], mu: &[u64; 4]) -> Vec<u8> {
    let mut out = vec![ACT_SETTLE];
    let _ = put_addr(&mut out, germ);
    out.push(voices.len().min(255) as u8);
    for v in voices.iter().take(255) {
        out.extend_from_slice(v);
    }
    push_mu(&mut out, mu);
    out
}

pub fn decode_settle(body: &[u8]) -> Option<(Address, Vec<[u8; 32]>, [u64; 4])> {
    if body.first() != Some(&ACT_SETTLE) {
        return None;
    }
    let mut p = 1usize;
    let germ = take_addr(body, &mut p)?;
    let n = *body.get(p)? as usize; // u8 — bounded by construction
    p += 1;
    let mut voices = Vec::with_capacity(n);
    for _ in 0..n {
        let v: [u8; 32] = body.get(p..p + 32)?.try_into().ok()?;
        p += 32;
        voices.push(v);
    }
    let mu = read_mu(body, &mut p)?;
    Some((germ, voices, mu))
}
