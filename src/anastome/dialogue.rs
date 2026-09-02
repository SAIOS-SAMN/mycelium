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

pub fn encode_refine(voice: &[u8; 32], word: &[u8; 32]) -> Vec<u8> {
    let mut out = vec![ACT_REFINE];
    out.extend_from_slice(voice);
    out.extend_from_slice(word);
    out
}

pub fn decode_refine(body: &[u8]) -> Option<([u8; 32], [u8; 32])> {
    if body.first() != Some(&ACT_REFINE) || body.len() != 65 {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let word: [u8; 32] = body.get(33..65)?.try_into().ok()?;
    Some((voice, word))
}

/// ADOPTION (genome synchronization): the court propagates an upheld word
/// to every server with ITS OWN VOICE as provenance — the ratifier is the
/// discoverer of record until origin voices ride the wire (freeze v2).
/// `[ACT_ADOPT][voice 32][element bytes]`. The reply acknowledges queueing
/// only; verification and installation run on the server's deliberation
/// worker (never on the request path).
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
    let n = *body.get(p)? as usize; // u8 — bounded by construction
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
/// docket request: `[ACT_DOCKET][voice 32]`.
pub fn encode_docket_request(voice: &[u8; 32]) -> Vec<u8> {
    let mut out = vec![ACT_DOCKET];
    out.extend_from_slice(voice);
    out
}

pub fn decode_docket_request(body: &[u8]) -> Option<[u8; 32]> {
    if body.first() != Some(&ACT_DOCKET) {
        return None;
    }
    body.get(1..33)?.try_into().ok()
}

/// docket reply: `[n u32] ( [addr 32][elen u32][element][nv u32][voices 32·nv][μ⃗ 32] )…`
pub fn encode_docket_reply(dockets: &[([u8; 32], Vec<u8>, Vec<[u8; 32]>, [u64; 4])]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&(dockets.len() as u32).to_le_bytes());
    for (addr, elem, voices, mu) in dockets {
        out.extend_from_slice(addr);
        out.extend_from_slice(&(elem.len() as u32).to_le_bytes());
        out.extend_from_slice(elem);
        out.extend_from_slice(&(voices.len() as u32).to_le_bytes());
        for v in voices {
            out.extend_from_slice(v);
        }
        push_mu(&mut out, mu);
    }
    out
}

pub fn decode_docket_reply(body: &[u8]) -> Option<Vec<([u8; 32], Vec<u8>, Vec<[u8; 32]>, [u64; 4])>> {
    let mut p = 0usize;
    let n = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    if n > body.len().saturating_sub(p) / 72 {
        return None; // untrusted count: ≥72 bytes per docket
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
        out.push((addr, elem, voices, mu));
    }
    Some(out)
}

/// Verdict rulings — the court's closed vocabulary.
pub const RULING_UPHELD: u8 = 1;
pub const RULING_DEMOTED: u8 = 2;
pub const RULING_SLASHED: u8 = 0;
/// Step 7c: rebind a word's anchor fiber — the verdict carries π* (a Sel term)
/// derived from family-wide evidence; the kernel upgrades γ = (⊤, Δ) → (π*, Δ)
/// in place (canonical substance identity untouched).
pub const RULING_REBIND: u8 = 3;

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

pub fn decode_verdict(
    body: &[u8],
) -> Option<([u8; 32], [u8; 32], u8, [u8; 32], Option<crate::transducer::anchor::Sel>)> {
    if body.first() != Some(&ACT_VERDICT) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let addr: [u8; 32] = body.get(33..65)?.try_into().ok()?;
    let ruling = *body.get(65)?;
    let receipt: [u8; 32] = body.get(66..98)?.try_into().ok()?;
    let pi = match body.get(98) {
        Some(1) => {
            let mut p = 99usize;
            Some(crate::transducer::anchor::Sel::from_bytes(body, &mut p)?)
        }
        _ => None,
    };
    Some((voice, addr, ruling, receipt, pi))
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
