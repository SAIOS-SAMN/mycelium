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
/// Step 5 court acts: the docket pull ("what awaits finality?") and the signed
/// verdict. The court speaks the same wire as everyone — its authority is the
/// replay, not the socket.
pub const ACT_DOCKET: u8 = 0x06;
pub const ACT_VERDICT: u8 = 0x07;
/// Step 7c: the court pulls the selection fiber's accumulated birth evidence.
pub const ACT_EVIDENCE: u8 = 0x08;
/// PoUW dispatch: a client pulls a SIGNED WORK ENVELOPE — the hypha's
/// authoritative registry chooses; clients never arbitrate raw corpus.
pub const ACT_TASK: u8 = 0x0B;
/// PoUW settlement: a client submits predicted outputs for a dispatched
/// task_id; the hypha verifies against its held answers — first verified
/// solve takes the bounty, replays collapse to the verification floor.
pub const ACT_SOLVE: u8 = 0x0C;
/// REFINEMENT BID (compression IS learning): a client claims basis word W
/// is derivable — the rest of the vocabulary already speaks it (S' ≺_DM S,
/// holonomy preserved because the replacement IS a decomposition of W's own
/// substance). The hypha verifies by deterministic replay; on success the
/// word retires, the vocabulary compresses, and the bidder earns the
/// compression yield. `[ACT_REFINE][voice 32][word address 32]`.
/// Receipt: solve-receipt shape — `[REFINE_ACCEPTED|REFINE_NOTHING][μ⃗ 32]`.
pub const ACT_REFINE: u8 = 0x09;
pub const REFINE_ACCEPTED: u8 = 1;
pub const REFINE_NOTHING: u8 = 0;

/// `[ACT_REFINE][voice 32][word 32][cites]` — the bidder's sentence for the
/// word (spoken by other words) rides as citations; the hypha replays.
pub fn encode_refine(voice: &[u8; 32], word: &[u8; 32], cites: &[CiteWire]) -> Vec<u8> {
    let mut out = vec![ACT_REFINE];
    out.extend_from_slice(voice);
    out.extend_from_slice(word);
    push_cites(&mut out, cites);
    out
}

pub fn decode_refine(body: &[u8]) -> Option<([u8; 32], [u8; 32], Vec<CiteWire>)> {
    if body.first() != Some(&ACT_REFINE) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let word: [u8; 32] = body.get(33..65)?.try_into().ok()?;
    let mut p = 65usize;
    let cites = read_cites(body, &mut p).unwrap_or_default();
    Some((voice, word, cites))
}

/// ADOPTION (genome synchronization): the court propagates an upheld word
/// to every server with ITS OWN VOICE as provenance — the ratifier is the
/// discoverer of record until origin voices ride the wire (freeze v2).
/// `[ACT_ADOPT][voice 32][element bytes]`. The reply acknowledges queueing
/// only; verification and installation run on the server's deliberation
/// worker (never on the request path).
/// Word-tape tag for a translation (gauge between two sections). No new
/// act: translations and formed thoughts travel as words. Same value as
/// `element::TRANSLATION_TAG`.
pub const TRANSLATION_TAG: u8 = 0x13;
/// Genome-file section holding persisted thought bubbles.
pub const THT1: &[u8; 4] = b"THT1";
/// Snapshot section holding Čech complexes (addresses and rotors only).
pub const COC1: &[u8; 4] = b"COC1";

pub const ACT_ADOPT: u8 = 0x0E;

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

/// GENESIS AUDIT (federation seeding): the court pages through a server's
/// held basis, blind-replays each word, and seals the verified core into
/// its federation — then propagates adoptions across the nerve so the
/// server plane converges without waiting for incidental consensus.
/// Request: `[ACT_GENESIS][voice 32][cursor u32]`. Reply shares the
/// syllabus page shape (`encode_syllabus_reply`); an empty page ends the
/// walk.
pub const ACT_GENESIS: u8 = 0x0F;

/// THE SHEAF SYNC (Q3): a holder asks for the other's section digests over
/// the atlas; a discordant stratum is then fetched by chart, with the
/// asker's have-list, so only the lacking words travel (sections and
/// connection riding with each). Cost scales with divergence.
/// Digests request: `[ACT_DIGESTS][voice 32]`. Reply: `[n u32]` then per
/// stratum `[len u8][inv u64·len][count u32][root 32]`.
pub const ACT_DIGESTS: u8 = 0x10;
/// Fetch request: `[ACT_FETCH][voice 32][len u8][inv u64·len][n u32][addr 32·n]`.
/// Reply: the syllabus page shape.
pub const ACT_FETCH: u8 = 0x11;
/// Hypha-to-hypha court organ: cover, offer, installed. Clients never send these.
pub const ACT_COVER: u8 = 0x20;
pub const ACT_OFFER: u8 = 0x21;
pub const ACT_INSTALLED: u8 = 0x22;

/// THE PROBE (interrogative grammar): a tentative sentence spoken against an
/// open listing — citations (word address, transport g, gauge coordinate)
/// the hypha replays linearly. Yield settles on graded descent of the
/// residual without germination; closure clears the listing.
/// Request: `[ACT_PROBE][voice 32][target 32][n u32][(addr 32)(g 17)(asg u32)]·n`.
/// Reply: `[code][paid μ⃗ 4×u64][remaining μ⃗ 4×u64]`.
pub const ACT_PROBE: u8 = 0x12;
pub const PROBE_NOTHING: u8 = 0;
pub const PROBE_DESCENT: u8 = 1;
pub const PROBE_CLOSED: u8 = 2;
/// The listing is no longer on the book in the state it was quoted:
/// another voice settled or descended it first.
pub const PROBE_SETTLED: u8 = 3;

/// A PROVEN ROOT — a family a voice solved and a registry verified —
/// gossiped across the nerve: `[ACT_ROOT][voice 32][task 32][solver 32]`.
/// Every hypha holding the task re-dispatches it first; a voice that
/// reproduces the solve from the taught language proves it shares the root.
pub const ACT_ROOT: u8 = 0x13;

/// `[ACT_ROOT][voice 32][task 32][solver 32][ticks u64]` — the root's stamp
/// on the speaker's clock. The reply is the receiver's record for that
/// task: `[solver 32 (zeros: none)][ticks u64]`, its own clock, so the
/// speaker knows the root was delivered later than it was spoken.
/// THE PEERS an entity routes over: `[ACT_PEERS][voice 32]` asks a hypha for
/// the addresses of the nerve it stands in (itself and its configured
/// peers); the reply is `[n u16] ([len u16][addr bytes])…`. The entity
/// builds the same XOR router the hyphas use, so every act it speaks goes
/// to the key's home.
pub const ACT_PEERS: u8 = 0x15;

pub fn encode_peers_request(voice: &[u8; 32]) -> Vec<u8> {
    let mut out = vec![ACT_PEERS];
    out.extend_from_slice(voice);
    out
}

pub fn decode_peers_request(body: &[u8]) -> Option<[u8; 32]> {
    if body.first() != Some(&ACT_PEERS) || body.len() != 33 {
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

/// A DEMAND ROUTED ON THE NERVE to the listing key's home hypha — so the
/// second voice on a listing can be a stranger's, not a local twin's:
/// `[ACT_DEMAND_ROUTED][origin hypha 32][the demand body as spoken]`.
/// Heard once as a demand at the home; never routed again.
pub const ACT_DEMAND_ROUTED: u8 = 0x14;

pub fn encode_routed_demand(origin: &[u8; 32], demand: &[u8]) -> Vec<u8> {
    let mut out = vec![ACT_DEMAND_ROUTED];
    out.extend_from_slice(origin);
    out.extend_from_slice(demand);
    out
}

pub fn decode_routed_demand(body: &[u8]) -> Option<([u8; 32], &[u8])> {
    if body.first() != Some(&ACT_DEMAND_ROUTED) || body.len() < 34 {
        return None;
    }
    let origin: [u8; 32] = body[1..33].try_into().ok()?;
    Some((origin, &body[33..]))
}

pub fn encode_root(voice: &[u8; 32], task: &[u8; 32], solver: &[u8; 32], ticks: u64) -> Vec<u8> {
    let mut out = vec![ACT_ROOT];
    out.extend_from_slice(voice);
    out.extend_from_slice(task);
    out.extend_from_slice(solver);
    out.extend_from_slice(&ticks.to_le_bytes());
    out
}

pub fn decode_root(body: &[u8]) -> Option<([u8; 32], [u8; 32], [u8; 32], u64)> {
    if body.first() != Some(&ACT_ROOT) || body.len() < 97 {
        return None;
    }
    let ticks = body.get(97..105).map(|b| u64::from_le_bytes(b.try_into().unwrap())).unwrap_or(0);
    Some((body[1..33].try_into().ok()?, body[33..65].try_into().ok()?, body[65..97].try_into().ok()?, ticks))
}

pub fn encode_root_reply(solver: Option<[u8; 32]>, ticks: u64) -> Vec<u8> {
    let mut out = solver.unwrap_or([0; 32]).to_vec();
    out.extend_from_slice(&ticks.to_le_bytes());
    out
}

pub fn decode_root_reply(body: &[u8]) -> Option<(Option<[u8; 32]>, u64)> {
    let solver: [u8; 32] = body.get(0..32)?.try_into().ok()?;
    let ticks = u64::from_le_bytes(body.get(32..40)?.try_into().ok()?);
    Some(((solver != [0; 32]).then_some(solver), ticks))
}

/// A citation on the wire: (word address, transport, gauge coordinate).
/// A CITATION ON THE WIRE carries its operator: the word's address as a
/// hint for the hearer's lexicon, the versor, the gauge coordinate as the
/// speaker held it, and the body itself — the word's cells under that
/// gauge, (row, col, before, after) in the word's own frame. The hearer
/// verifies the operator it received; it never fetches a copy of the word
/// to verify against.
pub type CiteWire = ([u8; 32], crate::transducer::element::Versor, u32, Vec<Cell>);

pub fn push_cites(out: &mut Vec<u8>, cites: &[CiteWire]) {
    out.extend_from_slice(&(cites.len() as u32).to_le_bytes());
    for (addr, g, asg, body) in cites {
        out.extend_from_slice(addr);
        g.to_bytes(out);
        out.extend_from_slice(&asg.to_le_bytes());
        push_chain(out, body);
    }
}

pub fn read_cites(b: &[u8], p: &mut usize) -> Option<Vec<CiteWire>> {
    let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
    *p += 4;
    if n > b.len().saturating_sub(*p) / 40 {
        return None; // untrusted count: address, versor, coordinate, body per citation
    }
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let addr: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
        *p += 32;
        let g = crate::transducer::element::Versor::from_bytes(b, p)?;
        let asg = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
        *p += 4;
        let body = read_chain(b, p)?;
        out.push((addr, g, asg, body));
    }
    Some(out)
}

pub fn encode_probe(voice: &[u8; 32], target: &[u8; 32], cites: &[CiteWire]) -> Vec<u8> {
    let mut out = vec![ACT_PROBE];
    out.extend_from_slice(voice);
    out.extend_from_slice(target);
    push_cites(&mut out, cites);
    out
}

pub fn decode_probe(body: &[u8]) -> Option<([u8; 32], [u8; 32], Vec<CiteWire>)> {
    if body.first() != Some(&ACT_PROBE) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let target: [u8; 32] = body.get(33..65)?.try_into().ok()?;
    let mut p = 65usize;
    let cites = read_cites(body, &mut p)?;
    Some((voice, target, cites))
}

pub fn encode_probe_reply(code: u8, paid: &[u64; 4], remaining: &[u64; 4]) -> Vec<u8> {
    let mut out = vec![code];
    push_mu(&mut out, paid);
    push_mu(&mut out, remaining);
    out
}

pub fn decode_probe_reply(body: &[u8]) -> Option<(u8, [u64; 4], [u64; 4])> {
    let code = *body.first()?;
    let mut p = 1usize;
    let paid = read_mu(body, &mut p)?;
    let remaining = read_mu(body, &mut p)?;
    Some((code, paid, remaining))
}

pub fn encode_digests_request(voice: &[u8; 32]) -> Vec<u8> {
    let mut out = vec![ACT_DIGESTS];
    out.extend_from_slice(voice);
    out
}

pub fn decode_digests_request(body: &[u8]) -> Option<[u8; 32]> {
    if body.first() != Some(&ACT_DIGESTS) || body.len() != 33 {
        return None;
    }
    body.get(1..33)?.try_into().ok()
}

fn push_invariant(out: &mut Vec<u8>, inv: &crate::transducer::stalk::Invariant) {
    out.push(inv.0.len().min(255) as u8);
    for v in inv.0.iter().take(255) {
        out.extend_from_slice(&v.to_le_bytes());
    }
}

fn read_invariant(b: &[u8], p: &mut usize) -> Option<crate::transducer::stalk::Invariant> {
    let len = *b.get(*p)? as usize;
    *p += 1;
    let mut v = Vec::with_capacity(len);
    for _ in 0..len {
        v.push(u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?));
        *p += 8;
    }
    Some(crate::transducer::stalk::Invariant(v))
}

pub fn encode_digests_reply(digests: &[crate::transducer::sheaf::StratumDigest]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&(digests.len() as u32).to_le_bytes());
    for d in digests {
        push_invariant(&mut out, &d.inv);
        out.extend_from_slice(&d.count.to_le_bytes());
        out.extend_from_slice(&d.root);
    }
    out
}

pub fn decode_digests_reply(body: &[u8]) -> Option<Vec<crate::transducer::sheaf::StratumDigest>> {
    let n = u32::from_le_bytes(body.get(0..4)?.try_into().ok()?) as usize;
    if n > body.len().saturating_sub(4) / 37 {
        return None; // untrusted count: at least len + count + root per stratum
    }
    let mut p = 4usize;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let inv = read_invariant(body, &mut p)?;
        let count = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?);
        p += 4;
        let root: [u8; 32] = body.get(p..p + 32)?.try_into().ok()?;
        p += 32;
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

pub fn decode_cover_digest(
    body: &[u8],
) -> Option<(
    Vec<(u8, crate::transducer::stalk::Invariant)>,
    [u64; 4],
    [[u8; 32]; 4],
    [u8; 32],
    [u8; 32],
    String,
    [u8; 64],
)> {
    // Skip the stratum-digest prefix if present: n then n records.
    let mut p = 0usize;
    if let Some(digests) = decode_digests_reply(body) {
        p = 4;
        for d in &digests {
            p += 1 + 8 * d.inv.0.len() + 4 + 32;
        }
        if p >= body.len() {
            return None;
        }
    }
    let n = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    let mut charts = Vec::with_capacity(n.min(1024));
    for _ in 0..n.min(1024) {
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
    Some((charts, room, cover_roots, ruled_root, vk, addr, sig))
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

pub fn decode_cover(body: &[u8]) -> Option<([u8; 32], String, Vec<(u8, crate::transducer::stalk::Invariant)>)> {
    if body.first() != Some(&ACT_COVER) {
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
    Some((vk, addr, charts))
}

/// Offer: `[ACT_OFFER][addr 32][elen u32][elem][nv u32][voices][deflation 32][k 8][sig 64][inst_sig 64]`
/// `sig` covers the Offer prefix; `inst_sig` is the home organ's signature
/// over the Installed prefix, so a second vantage can emit Installed without
/// the home key.
pub fn encode_offer(
    addr: &[u8; 32],
    elem: &[u8],
    voices: &[[u8; 32]],
    deflation: &[u64; 4],
    k: u64,
    sig: &[u8; 64],
    inst_sig: &[u8; 64],
) -> Vec<u8> {
    let mut out = vec![ACT_OFFER];
    out.extend_from_slice(addr);
    out.extend_from_slice(&(elem.len() as u32).to_le_bytes());
    out.extend_from_slice(elem);
    out.extend_from_slice(&(voices.len() as u32).to_le_bytes());
    for v in voices {
        out.extend_from_slice(v);
    }
    push_mu(&mut out, deflation);
    out.extend_from_slice(&k.to_le_bytes());
    out.extend_from_slice(sig);
    out.extend_from_slice(inst_sig);
    out
}

pub fn decode_offer(body: &[u8]) -> Option<([u8; 32], Vec<u8>, Vec<[u8; 32]>, [u64; 4], u64, [u8; 64], [u8; 64])> {
    if body.first() != Some(&ACT_OFFER) {
        return None;
    }
    let addr: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let elen = u32::from_le_bytes(body.get(33..37)?.try_into().ok()?) as usize;
    let mut p = 37usize;
    let elem = body.get(p..p + elen)?.to_vec();
    p += elen;
    let nv = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    let mut voices = Vec::with_capacity(nv.min(16));
    for _ in 0..nv {
        voices.push(body.get(p..p + 32)?.try_into().ok()?);
        p += 32;
    }
    let deflation = read_mu(body, &mut p)?;
    let k = u64::from_le_bytes(body.get(p..p + 8)?.try_into().ok()?);
    p += 8;
    let sig: [u8; 64] = body.get(p..p + 64)?.try_into().ok()?;
    p += 64;
    let inst_sig: [u8; 64] = body.get(p..p + 64)?.try_into().ok()?;
    Some((addr, elem, voices, deflation, k, sig, inst_sig))
}

/// Installed: `[ACT_INSTALLED][addr 32][elen u32][elem][sig_home 64][sig_peer 64]`
pub fn encode_installed(addr: &[u8; 32], elem: &[u8], sig_home: &[u8; 64], sig_peer: &[u8; 64]) -> Vec<u8> {
    let mut out = vec![ACT_INSTALLED];
    out.extend_from_slice(addr);
    out.extend_from_slice(&(elem.len() as u32).to_le_bytes());
    out.extend_from_slice(elem);
    out.extend_from_slice(sig_home);
    out.extend_from_slice(sig_peer);
    out
}

pub fn decode_installed(body: &[u8]) -> Option<([u8; 32], Vec<u8>, [u8; 64], [u8; 64])> {
    if body.first() != Some(&ACT_INSTALLED) {
        return None;
    }
    let addr: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let elen = u32::from_le_bytes(body.get(33..37)?.try_into().ok()?) as usize;
    let p = 37usize;
    let elem = body.get(p..p + elen)?.to_vec();
    let p = p + elen;
    let sig_home: [u8; 64] = body.get(p..p + 64)?.try_into().ok()?;
    let sig_peer: [u8; 64] = body.get(p + 64..p + 128)?.try_into().ok()?;
    Some((addr, elem, sig_home, sig_peer))
}

/// Bytes of an Offer/Installed body that the signature covers (everything after the act except the trailing sigs).
pub fn offer_signed_prefix(addr: &[u8; 32], elem: &[u8], voices: &[[u8; 32]], deflation: &[u64; 4], k: u64) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(addr);
    out.extend_from_slice(&(elem.len() as u32).to_le_bytes());
    out.extend_from_slice(elem);
    out.extend_from_slice(&(voices.len() as u32).to_le_bytes());
    for v in voices {
        out.extend_from_slice(v);
    }
    push_mu(&mut out, deflation);
    out.extend_from_slice(&k.to_le_bytes());
    out
}

pub fn installed_signed_prefix(addr: &[u8; 32], elem: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(addr);
    out.extend_from_slice(&(elem.len() as u32).to_le_bytes());
    out.extend_from_slice(elem);
    out
}

/// A fetch names every discordant chart with the asker's have-list in each;
/// the request itself is filled within the frame budget.
/// `[ACT_FETCH][voice 32][k u32]{[len u8][inv u64·len][n u32][addr 32·n]}·k`.
pub type Chart = (crate::transducer::stalk::Invariant, Vec<[u8; 32]>);

pub fn encode_fetch(voice: &[u8; 32], charts: &[Chart], budget: usize) -> Vec<u8> {
    let mut out = vec![ACT_FETCH];
    out.extend_from_slice(voice);
    let count_at = out.len();
    out.extend_from_slice(&0u32.to_le_bytes());
    let mut k = 0u32;
    for (inv, have) in charts {
        let width = 1 + 8 * inv.0.len() + 4 + 32 * have.len();
        if out.len() + width > budget {
            break;
        }
        push_invariant(&mut out, inv);
        out.extend_from_slice(&(have.len() as u32).to_le_bytes());
        for a in have {
            out.extend_from_slice(a);
        }
        k += 1;
    }
    out[count_at..count_at + 4].copy_from_slice(&k.to_le_bytes());
    out
}

pub fn decode_fetch(body: &[u8]) -> Option<([u8; 32], Vec<Chart>)> {
    if body.first() != Some(&ACT_FETCH) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let mut p = 33usize;
    let k = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    if k > body.len().saturating_sub(p) / 5 {
        return None;
    }
    let mut charts = Vec::with_capacity(k);
    for _ in 0..k {
        let inv = read_invariant(body, &mut p)?;
        let n = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
        if n > body.len().saturating_sub(p) / 32 {
            return None;
        }
        let mut have = Vec::with_capacity(n);
        for _ in 0..n {
            have.push(body.get(p..p + 32)?.try_into().ok()?);
            p += 32;
        }
        charts.push((inv, have));
    }
    Some((voice, charts))
}

pub fn encode_genesis_request(voice: &[u8; 32], cursor: u32) -> Vec<u8> {
    let mut out = vec![ACT_GENESIS];
    out.extend_from_slice(voice);
    out.extend_from_slice(&cursor.to_le_bytes());
    out
}

pub fn decode_genesis_request(body: &[u8]) -> Option<([u8; 32], u32)> {
    if body.first() != Some(&ACT_GENESIS) || body.len() != 37 {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let cursor = u32::from_le_bytes(body.get(33..37)?.try_into().ok()?);
    Some((voice, cursor))
}

// The genesis reply is the syllabus page shape (`decode_syllabus_reply`)
// with no wrapper type: the carrier itself decides — a non-empty page is
// work, the empty page is the walk's end, a malformed body is None, and
// transport failure is the exchange's io::Result. Everything the walk
// needs is a measurable property of the data.

/// THE SYLLABUS (distributed cognition, operator-ruled): court-ratified
/// vocabulary PUSHED to every enrolled voice on its next contact — teaching
/// as an act of the network, not a passive download. The client asks with
/// its voice; the hypha serves the ratified stream from that voice's cursor
/// (each ratification reaches each voice exactly once, in ruling order).
/// Request: `[ACT_SYLLABUS][voice 32]`. Reply: `[n u8][(len u32)(word
/// bytes)·n]` — full lossless element forms (Epigenome::learn refreshes by
/// address, so rebinds propagate too).
pub const ACT_SYLLABUS: u8 = 0x0D;

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

fn push_grid(out: &mut Vec<u8>, g: &[Vec<u8>]) {
    out.extend_from_slice(&(g.len() as u16).to_le_bytes());
    out.extend_from_slice(&(g.first().map_or(0, |r| r.len()) as u16).to_le_bytes());
    for row in g {
        out.extend_from_slice(row);
    }
}

fn read_grid(b: &[u8], p: &mut usize) -> Option<Vec<Vec<u8>>> {
    let h = u16::from_le_bytes(b.get(*p..*p + 2)?.try_into().ok()?) as usize;
    let w = u16::from_le_bytes(b.get(*p + 2..*p + 4)?.try_into().ok()?) as usize;
    *p += 4;
    if h * w > b.len().saturating_sub(*p) || h > 1024 || w > 1024 {
        return None; // untrusted dims
    }
    let mut g = Vec::with_capacity(h);
    for _ in 0..h {
        g.push(b.get(*p..*p + w)?.to_vec());
        *p += w;
    }
    Some(g)
}

/// task request: `[ACT_TASK][voice 32]`.
pub fn encode_task_request(voice: &[u8; 32]) -> Vec<u8> {
    let mut out = vec![ACT_TASK];
    out.extend_from_slice(voice);
    out
}

pub fn decode_task_request(body: &[u8]) -> Option<[u8; 32]> {
    if body.first() != Some(&ACT_TASK) || body.len() != 33 {
        return None;
    }
    body.get(1..33)?.try_into().ok()
}

/// work envelope: `[task_id 32][nt u8][(input grid, output grid)·nt]
/// [nx u8][test-input grid·nx]` — train pairs to STUDY, test inputs to
/// SOLVE; the held answers never ride the wire. An empty body = no work.
pub fn encode_task_envelope(
    id: &[u8; 32],
    train: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)],
    tests: &[Vec<Vec<u8>>],
) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(id);
    out.push(train.len().min(255) as u8);
    for (a, b) in train.iter().take(255) {
        push_grid(&mut out, a);
        push_grid(&mut out, b);
    }
    out.push(tests.len().min(255) as u8);
    for x in tests.iter().take(255) {
        push_grid(&mut out, x);
    }
    out
}

pub type TaskEnvelope = ([u8; 32], Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)>, Vec<Vec<Vec<u8>>>);

pub fn decode_task_envelope(body: &[u8]) -> Option<TaskEnvelope> {
    let id: [u8; 32] = body.get(0..32)?.try_into().ok()?;
    let mut p = 32usize;
    let nt = *body.get(p)? as usize;
    p += 1;
    let mut train = Vec::with_capacity(nt.min(64));
    for _ in 0..nt {
        let a = read_grid(body, &mut p)?;
        let b = read_grid(body, &mut p)?;
        train.push((a, b));
    }
    let nx = *body.get(p)? as usize;
    p += 1;
    let mut tests = Vec::with_capacity(nx.min(64));
    for _ in 0..nx {
        tests.push(read_grid(body, &mut p)?);
    }
    Some((id, train, tests))
}

/// solve submission: `[ACT_SOLVE][voice 32][task_id 32][n u8][grid·n]`.
pub fn encode_solve(voice: &[u8; 32], id: &[u8; 32], outputs: &[Vec<Vec<u8>>]) -> Vec<u8> {
    let mut out = vec![ACT_SOLVE];
    out.extend_from_slice(voice);
    out.extend_from_slice(id);
    out.push(outputs.len().min(255) as u8);
    for g in outputs.iter().take(255) {
        push_grid(&mut out, g);
    }
    out
}

pub fn decode_solve(body: &[u8]) -> Option<([u8; 32], [u8; 32], Vec<Vec<Vec<u8>>>)> {
    if body.first() != Some(&ACT_SOLVE) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let id: [u8; 32] = body.get(33..65)?.try_into().ok()?;
    let mut p = 65usize;
    let n = *body.get(p)? as usize;
    p += 1;
    let mut outputs = Vec::with_capacity(n.min(64));
    for _ in 0..n {
        outputs.push(read_grid(body, &mut p)?);
    }
    Some((voice, id, outputs))
}

/// solve receipt: `[code u8][μ⃗ 32]` — code 2 = BOUNTY (first verified solve),
/// 1 = verification floor (replay), 0 = nothing (wrong or unregistered).
pub const SOLVE_BOUNTY: u8 = 2;
pub const SOLVE_FLOOR: u8 = 1;
pub const SOLVE_NOTHING: u8 = 0;

pub fn encode_solve_receipt(code: u8, mu: &[u64; 4]) -> Vec<u8> {
    let mut out = vec![code];
    push_mu(&mut out, mu);
    out
}

pub fn decode_solve_receipt(body: &[u8]) -> Option<(u8, [u64; 4])> {
    let code = *body.first()?;
    let mut p = 1usize;
    let mu = read_mu(body, &mut p)?;
    Some((code, mu))
}

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

/// The wire width of one chain cell, measured from the encoder itself (the
/// carrier's own coordinate, never a number chosen elsewhere).
pub fn cell_wire_width() -> usize {
    let mut probe = Vec::new();
    push_chain(&mut probe, &[(0, 0, 0, 0)]);
    probe.len() - 4
}

fn read_chain(b: &[u8], p: &mut usize) -> Option<Vec<Cell>> {
    let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
    *p += 4;
    if n > b.len().saturating_sub(*p) / 18 {
        return None; // untrusted count: 18 bytes per cell, or refuse before allocating
    }
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

/// `[ACT_DEMAND][voice 32][chain][cites]` — the speaker's own reduction of
/// the chain over its genome rides as citations; an empty citation list
/// attests the chain irreducible for this voice.
pub fn encode_demand(voice: &[u8; 32], chain: &[Cell], cites: &[CiteWire]) -> Vec<u8> {
    let mut out = vec![ACT_DEMAND];
    out.extend_from_slice(voice);
    push_chain(&mut out, chain);
    push_cites(&mut out, cites);
    out
}

pub fn decode_demand(body: &[u8]) -> Option<([u8; 32], Vec<Cell>, Vec<CiteWire>)> {
    if body.first() != Some(&ACT_DEMAND) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let mut p = 33usize;
    let chain = read_chain(body, &mut p)?;
    let cites = read_cites(body, &mut p).unwrap_or_default();
    Some((voice, chain, cites))
}

// ── quote: the yield question and the quoted surface ─────────────────────────

/// `[ACT_QUOTE][voice 32][n u8][invariants…]` — the question, carrying the
/// charts of the voice's own atlas so the hypha can answer by incidence.
pub fn encode_quote_request(voice: &[u8; 32]) -> Vec<u8> {
    encode_quote_request_charts(voice, &[])
}

pub fn encode_quote_request_charts(
    voice: &[u8; 32],
    charts: &[crate::transducer::stalk::Invariant],
) -> Vec<u8> {
    let mut out = vec![ACT_QUOTE];
    out.extend_from_slice(voice);
    out.push(charts.len().min(255) as u8);
    for inv in charts.iter().take(255) {
        push_invariant(&mut out, inv);
    }
    out
}

pub fn decode_quote_request(body: &[u8]) -> Option<([u8; 32], Vec<crate::transducer::stalk::Invariant>)> {
    if body.first() != Some(&ACT_QUOTE) {
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
    }
    Some((voice, charts))
}

/// One quoted work item: the residual's address, its priced μ⃗, the chain,
/// and the open (non-flat) charts of its coherence field.
pub type WorkItem = ([u8; 32], [u64; 4], Vec<Cell>, Vec<crate::transducer::stalk::Invariant>);

/// `[ACT_QUOTE][surface 4×u64][K u64][n u8][item: addr 32 | μ⃗ 4×u64 | chain]…`
/// — K is the judge's k-step at the quote: the temporal coordinate every
/// listing quoted here is read at.
/// The offset of the served count in a quote reply (act, surface, K).
pub const QUOTE_COUNT_AT: usize = 1 + 32 + 8;
pub fn encode_quote_reply(surface: &[u64; 4], k: u64, items: &[WorkItem]) -> Vec<u8> {
    encode_quote_reply_within(surface, k, items, usize::MAX)
}

/// The deferral a full judge answers every act with: the pending outcome
/// and four zero lanes, nothing taught. For a percept or a demand it is a
/// verdict; for any other act it is impedance.
pub fn encode_deferral(code: u8) -> Vec<u8> {
    vec![code, 0, 0, 0, 0, 0, 0]
}

pub fn is_deferral(body: &[u8], code: u8) -> bool {
    body.len() == 7 && body[0] == code && body[1..].iter().all(|&x| x == 0)
}

/// The quote reply filled up to `budget` bytes: items ride in rank order
/// until the next would not fit. The frame budget decides what a contact
/// can carry; a wide book is served across contacts, never as a frame the
/// wire refuses.
pub fn encode_quote_reply_within(surface: &[u64; 4], k: u64, items: &[WorkItem], budget: usize) -> Vec<u8> {
    let mut out = vec![ACT_QUOTE];
    push_mu(&mut out, surface);
    out.extend_from_slice(&k.to_le_bytes());
    let count_at = out.len();
    out.push(0);
    let mut served = 0u8;
    for (addr, mu, chain, open) in items.iter().take(255) {
        let open_len: usize = 1 + open.iter().map(|i| 1 + 8 * i.0.len().min(255)).sum::<usize>();
        let item_len = 32 + 32 + 4 + chain.len() * 18 + open_len;
        if out.len() + item_len > budget {
            break;
        }
        out.extend_from_slice(addr);
        push_mu(&mut out, mu);
        push_chain(&mut out, chain);
        out.push(open.len().min(255) as u8);
        for inv in open.iter().take(255) {
            push_invariant(&mut out, inv);
        }
        served += 1;
    }
    out[count_at] = served;
    out
}

pub fn decode_quote_reply(body: &[u8]) -> Option<([u64; 4], u64, Vec<WorkItem>)> {
    if body.first() != Some(&ACT_QUOTE) {
        return None;
    }
    let mut p = 1usize;
    let surface = read_mu(body, &mut p)?;
    let k = u64::from_le_bytes(body.get(p..p + 8)?.try_into().ok()?);
    p += 8;
    let n = *body.get(p)? as usize; // u8 — bounded by construction
    p += 1;
    let mut items = Vec::with_capacity(n);
    for _ in 0..n {
        let addr: [u8; 32] = body.get(p..p + 32)?.try_into().ok()?;
        p += 32;
        let mu = read_mu(body, &mut p)?;
        let chain = read_chain(body, &mut p)?;
        let nopen = *body.get(p)? as usize;
        p += 1;
        let mut open = Vec::with_capacity(nopen);
        for _ in 0..nopen {
            open.push(read_invariant(body, &mut p)?);
        }
        items.push((addr, mu, chain, open));
    }
    Some((surface, k, items))
}

// ── teach: canonical element transmission ────────────────────────────────────

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
    let since = body.get(33..41).map(|b| u64::from_le_bytes(b.try_into().unwrap())).unwrap_or(0);
    let ticks = body.get(41..49).map(|b| u64::from_le_bytes(b.try_into().unwrap())).unwrap_or(0);
    Some((voice, since, ticks))
}

/// docket reply: `[n u32] ( [addr 32][elen u32][element][nv u32][voices 32·nv][μ⃗ 32][Δμ⃗ 32] )…`
/// — Δμ⃗ is the deflation the word's install measured on a listed demand
/// (zero when the germ settled nothing on the book): the court's proof of
/// convergence rides with the word.
/// The last field is the hypha's k-step at which the word was installed:
/// the coordinate a court's "since" is read against.
pub type Docket = ([u8; 32], Vec<u8>, Vec<[u8; 32]>, [u64; 4], [u64; 4], u64);

/// A SETTLEMENT on the docket: the listing, its chain, the voice, the
/// utterance that settled it, and the magnitudes before and after as the
/// hypha measured them. The court replays the utterance and measures for
/// itself: two measurements of one deflation are the proof of convergence.
/// The last field is the hypha's k-step of the settlement.
pub type Settlement = ([u8; 32], Vec<Cell>, [u8; 32], Vec<CiteWire>, [u64; 4], [u64; 4], u64);

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
    for (listing, chain, voice, cites, before, after, k) in settlements {
        let mut item = Vec::new();
        item.extend_from_slice(listing);
        push_chain(&mut item, chain);
        item.extend_from_slice(voice);
        push_cites(&mut item, cites);
        push_mu(&mut item, before);
        push_mu(&mut item, after);
        item.extend_from_slice(&k.to_le_bytes());
        if settled_len + item.len() > budget {
            break;
        }
        settled_len += item.len();
        settled.push(item);
    }
    let mut out = Vec::new();
    out.extend_from_slice(&0u32.to_le_bytes());
    let mut words = 0usize;
    for (addr, elem, voices, mu, deflation, k) in dockets {
        let mut item = Vec::with_capacity(112 + elem.len() + 32 * voices.len());
        item.extend_from_slice(addr);
        item.extend_from_slice(&(elem.len() as u32).to_le_bytes());
        item.extend_from_slice(elem);
        item.extend_from_slice(&(voices.len() as u32).to_le_bytes());
        for v in voices {
            item.extend_from_slice(v);
        }
        push_mu(&mut item, mu);
        push_mu(&mut item, deflation);
        item.extend_from_slice(&k.to_le_bytes());
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
    let Some(n) = body.get(p..p + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap()) as usize) else { return Vec::new() };
    p += 4;
    let mut out = Vec::new();
    for _ in 0..n {
        let Some(listing) = body.get(p..p + 32).and_then(|b| b.try_into().ok()) else { break };
        p += 32;
        let Some(chain) = read_chain(body, &mut p) else { break };
        let Some(voice) = body.get(p..p + 32).and_then(|b| b.try_into().ok()) else { break };
        p += 32;
        let Some(cites) = read_cites(body, &mut p) else { break };
        let Some(before) = read_mu(body, &mut p) else { break };
        let Some(after) = read_mu(body, &mut p) else { break };
        let Some(k) = body.get(p..p + 8).map(|b| u64::from_le_bytes(b.try_into().unwrap())) else { break };
        p += 8;
        out.push((listing, chain, voice, cites, before, after, k));
    }
    out
}

/// Cavities riding after the settlements: the evicted element's bytes.
/// Absent on an older reply.
pub fn decode_cavities(body: &[u8]) -> Vec<Vec<u8>> {
    let Some((_, mut p)) = decode_dockets_at(body) else { return Vec::new() };
    let Some(ns) = body.get(p..p + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap()) as usize) else {
        return Vec::new();
    };
    p += 4;
    for _ in 0..ns {
        p += 32;
        let Some(_) = read_chain(body, &mut p) else { return Vec::new() };
        p += 32;
        let Some(_) = read_cites(body, &mut p) else { return Vec::new() };
        let Some(_) = read_mu(body, &mut p) else { return Vec::new() };
        let Some(_) = read_mu(body, &mut p) else { return Vec::new() };
        p += 8;
    }
    let Some(n) = body.get(p..p + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap()) as usize) else {
        return Vec::new();
    };
    p += 4;
    let mut out = Vec::with_capacity(n.min(64));
    for _ in 0..n {
        let Some(el) = body.get(p..p + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap()) as usize) else {
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

fn decode_dockets_at(body: &[u8]) -> Option<(Vec<Docket>, usize)> {
    let mut p = 0usize;
    let n = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    if n > body.len().saturating_sub(p) / 104 {
        return None; // untrusted count: ≥104 bytes per docket
    }
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let addr: [u8; 32] = body.get(p..p + 32)?.try_into().ok()?;
        p += 32;
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
        out.push((addr, elem, voices, mu, deflation, k));
    }
    Some((out, p))
}

/// Verdict rulings — the court's closed vocabulary.
pub const RULING_UPHELD: u8 = 1;
pub const RULING_DEMOTED: u8 = 2;
pub const RULING_SLASHED: u8 = 0;
/// Step 7c: rebind a word's anchor fiber — the verdict carries π* (a Sel term)
/// derived from family-wide evidence; the kernel upgrades γ = (⊤, Δ) → (π*, Δ)
/// in place (canonical substance identity untouched).
pub const RULING_REBIND: u8 = 3;
/// The bundle connects: the ruled word is a placement of a federated base
/// geometry — the verdict carries the base address and the group element g
/// that transports the word into the base frame. A hypha holding both
/// attaches the word as a section of the base and retires its card.
pub const RULING_ATTACHED: u8 = 4;
/// Two independent voices verified the same task: a proven root.
pub const RULING_ROOT: u8 = 5;

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
pub fn encode_evidence_deposit(
    voice: &[u8; 32],
    items: &[([u8; 32], Vec<(i64, i64)>, Vec<(i64, i64, u8)>)],
) -> Vec<u8> {
    let mut out = vec![ACT_EVIDENCE];
    out.extend_from_slice(voice);
    out.extend_from_slice(&encode_evidence_reply(items));
    out
}

pub fn decode_evidence_deposit(
    body: &[u8],
) -> Option<([u8; 32], Vec<([u8; 32], Vec<(i64, i64)>, Vec<(i64, i64, u8)>)>)> {
    if body.first() != Some(&ACT_EVIDENCE) || body.len() <= 33 {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let items = decode_evidence_reply(&body[33..])?;
    Some((voice, items))
}

/// evidence reply: `[n u32] ( [addr 32][nf u32][fp (r i64,c i64)·nf]
/// [ns u32][scene (r i64,c i64,h u8)·ns] )…` — the selection fiber's birth
/// evidence, count-guarded at every level.
pub fn encode_evidence_reply(
    items: &[([u8; 32], Vec<(i64, i64)>, Vec<(i64, i64, u8)>)],
) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&(items.len() as u32).to_le_bytes());
    for (addr, fp, scene) in items {
        out.extend_from_slice(addr);
        out.extend_from_slice(&(fp.len() as u32).to_le_bytes());
        for (r, c) in fp {
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

pub fn decode_evidence_reply(
    body: &[u8],
) -> Option<Vec<([u8; 32], Vec<(i64, i64)>, Vec<(i64, i64, u8)>)>> {
    let mut p = 0usize;
    let n = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    if n > body.len().saturating_sub(p) / 40 {
        return None; // untrusted count
    }
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let addr: [u8; 32] = body.get(p..p + 32)?.try_into().ok()?;
        p += 32;
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
        out.push((addr, fp, scene));
    }
    Some(out)
}

/// verdict: `[ACT_VERDICT][court voice 32][addr 32][ruling u8][receipt 32]` —
/// the receipt is the head of the court's own chronometric chain (the signature
/// binding the ruling to the court's tamper-evident history).
pub fn encode_verdict(voice: &[u8; 32], addr: &[u8; 32], ruling: u8, receipt: &[u8; 32]) -> Vec<u8> {
    encode_verdict_payload(voice, addr, ruling, receipt, None)
}

/// With the optional π* payload (RULING_REBIND): `…[receipt 32][1][sel]` /
/// `…[receipt 32][0]`.
pub fn encode_verdict_payload(
    voice: &[u8; 32],
    addr: &[u8; 32],
    ruling: u8,
    receipt: &[u8; 32],
    pi: Option<&crate::transducer::anchor::Sel>,
) -> Vec<u8> {
    let mut out = vec![ACT_VERDICT];
    out.extend_from_slice(voice);
    out.extend_from_slice(addr);
    out.push(ruling);
    out.extend_from_slice(receipt);
    match pi {
        Some(t) => {
            out.push(1);
            t.to_bytes(&mut out);
        }
        None => out.push(0),
    }
    out
}

/// The attach payload (RULING_ATTACHED): `…[receipt 32][2][base 32][g 17]` —
/// the base the word connects to and the transport g into its frame.
pub fn encode_verdict_attach(
    voice: &[u8; 32],
    addr: &[u8; 32],
    receipt: &[u8; 32],
    base: &[u8; 32],
    g: &crate::transducer::element::Versor,
) -> Vec<u8> {
    let mut out = vec![ACT_VERDICT];
    out.extend_from_slice(voice);
    out.extend_from_slice(addr);
    out.push(RULING_ATTACHED);
    out.extend_from_slice(receipt);
    out.push(2);
    out.extend_from_slice(base);
    g.to_bytes(&mut out);
    out
}

/// A decoded verdict: (court voice, word address, ruling, receipt, π* for a
/// rebind, (base, g) for an attach).
pub type Verdict = (
    [u8; 32],
    [u8; 32],
    u8,
    [u8; 32],
    Option<crate::transducer::anchor::Sel>,
    Option<([u8; 32], crate::transducer::element::Versor)>,
);

pub fn decode_verdict(body: &[u8]) -> Option<Verdict> {
    if body.first() != Some(&ACT_VERDICT) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let addr: [u8; 32] = body.get(33..65)?.try_into().ok()?;
    let ruling = *body.get(65)?;
    let receipt: [u8; 32] = body.get(66..98)?.try_into().ok()?;
    let (pi, attach) = match body.get(98) {
        Some(1) => {
            let mut p = 99usize;
            (Some(crate::transducer::anchor::Sel::from_bytes(body, &mut p)?), None)
        }
        Some(2) => {
            let base: [u8; 32] = body.get(99..131)?.try_into().ok()?;
            let mut p = 131usize;
            let g = crate::transducer::element::Versor::from_bytes(body, &mut p)?;
            (None, Some((base, g)))
        }
        _ => (None, None),
    };
    Some((voice, addr, ruling, receipt, pi, attach))
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
    let n = *body.get(33)? as usize; // u8 — bounded by construction
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
        let cites = vec![([3u8; 32], crate::transducer::element::translator_versor(1, -1), 1u32, chain.clone())];
        let (v, c, k) = decode_demand(&encode_demand(&voice, &chain, &cites)).unwrap();
        assert_eq!(k, cites, "the citations ride the demand");
        assert_eq!((v, c), (voice, chain.clone()));
        assert_eq!(decode_quote_request(&encode_quote_request(&voice)).unwrap(), (voice, Vec::new()));
        let items = vec![(germ, [1u64, 2, 30, 4], chain.clone(), Vec::new())];
        let (s, k, it) = decode_quote_reply(&encode_quote_reply(&[5, 0, 9, 0], 4208, &items)).unwrap();
        assert_eq!(s, [5, 0, 9, 0]);
        assert_eq!(k, 4208, "the judge's k-step rides the quote");
        assert_eq!(it, items);
        // A settlement rides the docket after the dockets, with its utterance.
        let docket: Docket = (germ, vec![1, 2, 3], vec![voice], [1, 2, 3, 4], [0, 0, 7, 0], 41);
        let settlement: Settlement = (germ, chain.clone(), voice, cites.clone(), [2, 0, 2, 0], [0, 0, 1, 0], 42);
        let body = encode_docket_reply_with(std::slice::from_ref(&docket), std::slice::from_ref(&settlement));
        assert_eq!(decode_docket_reply(&body).unwrap(), vec![docket.clone()]);
        assert_eq!(decode_settlements(&body), vec![settlement.clone()]);
        let cavity = vec![0xCAu8, 0xFE, 0xBA, 0xBE];
        let (paged, w, s, n) = encode_docket_reply_paged(
            std::slice::from_ref(&docket),
            std::slice::from_ref(&settlement),
            std::slice::from_ref(&cavity),
            usize::MAX,
        );
        assert_eq!((w, s, n), (1, 1, 1));
        assert_eq!(decode_cavities(&paged), vec![cavity]);
        assert_eq!(decode_teach(&encode_teach(&[0xAB, 0xCD])).unwrap(), vec![0xAB, 0xCD]);
        let (g2, vs, mu) = decode_settle(&encode_settle(&germ, &[voice, germ], &[0, 0, 12, 0])).unwrap();
        assert_eq!((g2, vs, mu), (germ, vec![voice, germ], [0, 0, 12, 0]));
    }
}
