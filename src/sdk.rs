// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE KERNEL SDK — how an external kernel joins the mesh and DEVELOPS on it.
//! Sodalitum, netstratum, or any newcomer links this to: take a stable mesh
//! identity, PERCEIVE a scene pair (segment it — perception is the kernel's, never
//! the hypha's), SUBMIT the A→B demand to a hypha, receive the judge's VERDICT, and
//! LEARN the taught ANASTOME element so no dialect drifts from the mesh. No engine
//! internals leak across this seam: cells in, verdict out. Ported from the old
//! mycelium SDK (Session / kernel emblem / observe) onto the v0.1.0 percept wire.

use crate::anastome::dialogue;
use crate::cognition::{decompose_over, encode_percept, perceive};
use crate::network::wire::{Frame, Stratum};
use crate::transducer::element::Element;
use std::io::{self, Read, Write};
use std::net::TcpStream;

/// A kernel's stable mesh IDENTITY. The name deterministically fixes a 32-byte
/// address (its "voice"/seat), so a kernel keeps the same identity across
/// reconnects (residency — the seat outlives the socket). Consilience needs two
/// DISTINCT kernels to agree, so distinct names give distinct voices.
#[derive(Clone, Debug)]
pub struct Kernel {
    name: String,
    voice: [u8; 32],
}

impl Kernel {
    /// Seat a kernel by name — the emblem digest is its durable address.
    pub fn named(name: &str) -> Self {
        let voice = *blake3::hash(format!("mycelium-kernel:{name}").as_bytes()).as_bytes();
        Kernel { name: name.to_string(), voice }
    }
    pub fn voice(&self) -> [u8; 32] {
        self.voice
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// The judge's answer to a submitted demand, plus any taught ANASTOME element the
/// kernel learns (the mesh's word for what it just described).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// The demand DECOMPOSED into a canonical sentence of installed generators (the
    /// confluent normal form) — the mesh already speaks it, nothing minted.
    Decomposed { taught: Vec<u8> },
    /// A novel generator awaiting a second kernel's voice (consilience pending).
    GermPending,
    /// An irreducible residual that reached consilience and adjoined (KB completion).
    Installed { taught: Vec<u8> },
    /// The scene fragmented into per-body sub-demands.
    Fragmented,
    /// A degenerate demand — nothing to reduce.
    Pivot,
    /// An unrecognized reply code (forward-compat).
    Unknown(u8),
}

impl Verdict {
    pub fn label(&self) -> &'static str {
        match self {
            Verdict::Decomposed { .. } => "decomposed",
            Verdict::GermPending => "germ-pending",
            Verdict::Installed { .. } => "installed",
            Verdict::Fragmented => "fragmented",
            Verdict::Pivot => "pivot",
            Verdict::Unknown(_) => "unknown",
        }
    }
    /// The taught ANASTOME element the mesh handed back (empty when none).
    pub fn taught(&self) -> &[u8] {
        match self {
            Verdict::Decomposed { taught } | Verdict::Installed { taught } => taught,
            _ => &[],
        }
    }
}

/// A kernel's session with a hypha. The v0.1.0 door serves one demand per
/// connection, so [`Session::submit`] dials per demand — reconnection IS the
/// residency model (the kernel's identity is stable, the socket is not).
pub struct Session {
    addr: String,
    kernel: Kernel,
}

impl Session {
    /// Point a kernel at a hypha address (e.g. `"127.0.0.1:7500"`). No socket is
    /// held; each submit dials fresh.
    pub fn open(addr: &str, kernel: Kernel) -> Self {
        Session { addr: addr.to_string(), kernel }
    }

    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    /// PERCEIVE the scene pair (segment A and B into bodies) and SUBMIT it as an
    /// A→B demand under this kernel's voice; return the judge's [`Reply`] — the
    /// verdict with any taught element, plus the mesh's per-lane METABOLISM
    /// (pressure) the kernel paces its intake against. The hypha only judges.
    pub fn submit(&self, a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) -> io::Result<Reply> {
        let (a_bodies, b_bodies) = perceive(a, b);
        let mut pbody = vec![dialogue::ACT_PERCEPT];
        pbody.extend_from_slice(&encode_percept(&a_bodies, &b_bodies, &self.kernel.voice));
        let body = self.exchange(pbody)?;
        Ok(decode_reply(&body))
    }

    /// SPEAK a `demand` — this kernel's INDEPENDENT derivation of an open
    /// residual (its centered coboundary chain), attesting it toward consilience.
    pub fn attest(&self, chain: &[(i64, i64, u8, u8)]) -> io::Result<Reply> {
        let body = self.exchange(dialogue::encode_demand(&self.kernel.voice, chain))?;
        Ok(decode_reply(&body))
    }

    /// ASK the quote — "what is the highest-yield work?" The hypha answers with
    /// the yield surface (per-lane, priced off genuine metabolic tension) and the
    /// open-demand market's top clusters, each carrying its chain so this kernel
    /// can actually attempt the reduction.
    pub fn quote(&self) -> io::Result<YieldQuote> {
        let body = self.exchange(dialogue::encode_quote_request(&self.kernel.voice))?;
        let (surface, items) = dialogue::decode_quote_reply(&body)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "quote decode"))?;
        Ok(YieldQuote {
            surface,
            clusters: items.into_iter().map(|(address, mu, chain)| Cluster { address, mu, chain }).collect(),
        })
    }

    /// One framed exchange: dial, speak the act, read the reply body.
    /// The RAW ACT DOOR — speak any encoded dialogue act and receive the reply
    /// body. The public SDK surface for custom kernels (and the court): the
    /// wire is the universal language; authority comes from the mathematics,
    /// never the socket.
    pub fn speak(&self, act_body: Vec<u8>) -> io::Result<Vec<u8>> {
        self.exchange(act_body)
    }

    fn exchange(&self, act_body: Vec<u8>) -> io::Result<Vec<u8>> {
        let frame = Frame { lane: Stratum::Spatial, body: act_body };
        let bytes = frame.encode().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "frame encode"))?;
        let mut s = TcpStream::connect(&self.addr)?;
        s.write_all(&bytes)?;
        let mut head = [0u8; 5];
        s.read_exact(&mut head)?;
        let len = u32::from_le_bytes([head[1], head[2], head[3], head[4]]) as usize;
        let mut body = vec![0u8; len];
        s.read_exact(&mut body)?;
        Ok(body)
    }
}

// ── the ENTITY: genome + epigenome (the client as what a CE entity was) ──────

/// The comprehension verdict — the C-STACK's terminal (congruence → coherence →
/// cohesion → converse → COMPREHENSION): the received utterance DECOMPOSED over
/// the local genome toward understanding. Never re-cognition (that is the
/// perception stack's overlay), never recall.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Comprehension {
    /// The utterance decomposed into a sentence of held words — understood.
    Understood(Vec<usize>),
    /// Partially decomposed: the residual is the not-yet-understood part (a
    /// prospective for the mesh, not a failure of the ear).
    Novel { residual_cells: usize },
}

/// GENOME + EPIGENOME — carried from the CE entity design (core/ccl.rs: the
/// State Record is the "H¹ representative"/"genome"; DOWN is "gene expression").
/// The GENOME is the consilience-settled shared language the mesh expresses down
/// via `teach` — never locally mutated, dialect-free, court-guarded. The
/// EPIGENOME is this entity's local dialect over it: an efficient recollection
/// store — per-word expression weightings and locally comprehended sentences —
/// modifying EXPRESSION without touching the inherited record. Client↔client
/// dialects may legitimately form here; the genome may not drift.
/// THE 11-DIMENSIONAL CHROMATIC APERTURE — ported from ipsal-autic
/// `perception/harmonic_tuning.rs`: the entity's exact-ℚ perceptual bias over
/// the 11-dimensional harmonic basis (h[0] spectral/value; h[1–2]
/// spatial/geometric; h[3] adjacency; h[4–10] higher torsion sensitivity).
/// Each absorption is the exact running average h[k] = (h[k]·n + signal)/(n+1) —
/// the same observation refracts differently through different entities: the
/// aperture IS the dialect's optics, epigenome-side, genome-untouched.
pub struct Aperture {
    h: Vec<crate::core::engine::Q>,
    counts: Vec<u64>,
}

impl Default for Aperture {
    fn default() -> Self {
        use num_traits::Zero;
        Aperture { h: vec![crate::core::engine::Q::zero(); 11], counts: vec![0; 11] }
    }
}

impl Aperture {
    /// Absorb a signal into dimension `k` — exact running average, exact ℚ.
    pub fn absorb(&mut self, dim: usize, signal: crate::core::engine::Q) {
        use num_bigint::BigInt;
        let (Some(h), Some(n)) = (self.h.get_mut(dim), self.counts.get_mut(dim)) else { return };
        let nq = crate::core::engine::Q::from(BigInt::from(*n));
        let n1 = crate::core::engine::Q::from(BigInt::from(n.saturating_add(1)));
        *h = (h.clone() * nq + signal) / n1;
        *n = n.saturating_add(1);
    }
    /// A spectral/value signal: |to − from| / 10 into h[0] (the hue axis).
    pub fn absorb_value_signal(&mut self, from: u8, to: u8) {
        use num_bigint::BigInt;
        let d = (from as i64 - to as i64).unsigned_abs();
        self.absorb(0, crate::core::engine::Q::new(BigInt::from(d), BigInt::from(10)));
    }
    /// A spatial/torsion-order signal: period / 8 into h[1] (normalized to the
    /// source's D4-max convention).
    pub fn absorb_spatial_signal(&mut self, torsion_order: u32) {
        use num_bigint::BigInt;
        self.absorb(1, crate::core::engine::Q::new(BigInt::from(torsion_order as i64), BigInt::from(8)));
    }
    pub fn tuned_dims(&self) -> usize {
        self.counts.iter().filter(|&&n| n > 0).count()
    }
    pub fn count(&self, dim: usize) -> u64 {
        self.counts.get(dim).copied().unwrap_or(0)
    }
    pub fn bias(&self, dim: usize) -> Option<&crate::core::engine::Q> {
        self.h.get(dim)
    }
}

#[derive(Default)]
pub struct Epigenome {
    genome: Vec<Element>,
    /// Expression weighting per genome word — how often this entity speaks/hears
    /// it (the recollection efficiency; parallel to `genome`).
    expression: Vec<u64>,
    /// Locally comprehended sentences and their recurrence — the dialect.
    recollections: Vec<(Vec<usize>, u64)>,
    /// The entity's chromatic aperture — its perceptual bias (harmonic import).
    pub aperture: Aperture,
}

impl Epigenome {
    pub fn new() -> Self {
        Epigenome::default()
    }
    pub fn genome_len(&self) -> usize {
        self.genome.len()
    }
    pub fn genome(&self) -> &[Element] {
        &self.genome
    }
    /// The strongest-expressed word's weight (the dialect's leading edge).
    pub fn peak_expression(&self) -> u64 {
        self.expression.iter().copied().max().unwrap_or(0)
    }
    pub fn recollections(&self) -> &[(Vec<usize>, u64)] {
        &self.recollections
    }

    /// COMPREHEND a received utterance: decompose it over the genome (the shared
    /// projective mathematics, `cognition::decompose_over`) and fold the
    /// understanding into the epigenome — expression weights rise on the cited
    /// words, the sentence joins the recollections. The genome is untouched.
    pub fn comprehend(&mut self, utterance: &Element) -> Comprehension {
        match decompose_over(&self.genome, utterance.sub(), None, 4, 16) {
            Ok(sentence) => {
                for &w in &sentence {
                    if let Some(x) = self.expression.get_mut(w) {
                        *x += 1;
                    }
                }
                if !sentence.is_empty() {
                    match self.recollections.iter_mut().find(|(s, _)| *s == sentence) {
                        Some((_, n)) => *n += 1,
                        None => self.recollections.push((sentence.clone(), 1)),
                    }
                }
                Comprehension::Understood(sentence)
            }
            Err(void) => Comprehension::Novel { residual_cells: void.len() },
        }
    }

    /// LEARN — the mesh expressed a word down (`teach`): comprehend it against
    /// what is already held, then bank it in the genome if it is new (by base
    /// address). Inheritance, not local invention: only taught words enter.
    pub fn learn(&mut self, taught: &[u8]) -> Option<Comprehension> {
        let mut pos = 0usize;
        let e = Element::from_bytes(taught, &mut pos)?;
        // The APERTURE tunes on what is inherited: each transition refracts a
        // value signal into h[0]; each torsion period a spatial signal into h[1]
        // — the entity's optics drift with its diet (dialect-legitimate).
        for &(_, _, before, after) in e.sub() {
            self.aperture.absorb_value_signal(before, after);
        }
        for (_, l) in e.torsion() {
            self.aperture.absorb_spatial_signal(*l);
        }
        let heard = self.comprehend(&e);
        let addr = e.address();
        match self.genome.iter().position(|g| g.address() == addr) {
            Some(i) => self.genome[i] = e, // the taught form may carry new sections — refresh
            None => {
                self.genome.push(e);
                self.expression.push(0);
            }
        }
        Some(heard)
    }

    /// CRYSTALLIZE the genome for persistence — the permanent language only.
    /// The epigenome (expression, recollections, aperture) is deliberately
    /// ephemeral: dialect rebuilds each run; the genome carries continuity.
    /// Format: `GEN1 | n u32 | (len u32 | element)…`.
    pub fn crystallize(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + self.genome.len() * 64);
        out.extend_from_slice(b"GEN1");
        out.extend_from_slice(&(self.genome.len() as u32).to_le_bytes());
        for g in &self.genome {
            let b = g.to_bytes();
            out.extend_from_slice(&(b.len() as u32).to_le_bytes());
            out.extend_from_slice(&b);
        }
        out
    }

    /// Restore the crystallized genome (count-guarded; a corrupt file yields a
    /// clean fresh genome, never a crash). Expression starts at zero — the
    /// dialect re-forms through this run's hearing.
    pub fn uncrystallize(&mut self, bytes: &[u8]) -> bool {
        if bytes.len() < 8 || &bytes[..4] != b"GEN1" {
            return false;
        }
        let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
        if n > bytes.len().saturating_sub(8) / 5 {
            return false; // untrusted count vs remaining bytes
        }
        let mut genome = Vec::with_capacity(n);
        let mut pos = 8usize;
        for _ in 0..n {
            let Some(l4) = bytes.get(pos..pos + 4) else { return false };
            let len = u32::from_le_bytes(l4.try_into().unwrap()) as usize;
            pos += 4;
            if len > bytes.len().saturating_sub(pos) {
                return false;
            }
            let mut ep = 0usize;
            let Some(e) = Element::from_bytes(&bytes[pos..pos + len], &mut ep) else {
                return false;
            };
            pos += len;
            genome.push(e);
        }
        self.expression = vec![0; genome.len()];
        self.genome = genome;
        true
    }
}

/// THE ENTITY — a kernel identity with its genome/epigenome, conversing through
/// a session: it submits percepts, learns what the mesh teaches (comprehending,
/// not storing), asks quotes, and attests demands.
pub struct Entity {
    session: Session,
    pub epigenome: Epigenome,
}

impl Entity {
    pub fn open(addr: &str, kernel: Kernel) -> Self {
        Entity { session: Session::open(addr, kernel), epigenome: Epigenome::new() }
    }

    /// Open with CONTINUITY: restore the crystallized genome from `genome_path`
    /// if present (the permanent half; the epigenome re-forms in memory).
    pub fn open_persistent(addr: &str, kernel: Kernel, genome_path: &str) -> Self {
        let mut e = Entity::open(addr, kernel);
        if let Ok(bytes) = std::fs::read(genome_path) {
            if e.epigenome.uncrystallize(&bytes) {
                eprintln!("genome restored: {} words from {genome_path}", e.epigenome.genome_len());
            }
        }
        e
    }

    /// Persist the genome atomically (write-then-rename). Call from the
    /// syntropic phase when the genome has grown.
    pub fn save_genome(&self, genome_path: &str) -> io::Result<()> {
        let tmp = format!("{genome_path}.tmp");
        std::fs::write(&tmp, self.epigenome.crystallize())?;
        std::fs::rename(&tmp, genome_path)
    }

    /// PROJECT — the entity THINKS with its inherited language: apply the
    /// genome's anchored words (γ = (π, Δ)) to a world state and return the
    /// projected next state plus the sentence of words that spoke. This is
    /// the intelligence-pull for embodied kernels (game individuals, world
    /// models): comprehension reads the present, projection expects the
    /// future — both from the SAME court-refined mesh language. Only π-bound
    /// words speak; the unrefined stay silent, so a young genome projects
    /// humbly and grows sharper exactly as the court binds anchors.
    pub fn project(&self, grid: &crate::arc::Grid) -> (crate::arc::Grid, Vec<usize>) {
        crate::arc::project_anchored(grid, self.epigenome.genome())
    }
    pub fn kernel(&self) -> &Kernel {
        self.session.kernel()
    }
    /// Submit a percept; COMPREHEND anything taught back (the C-stack, not a
    /// byte-store). Returns the reply and the comprehension of the teaching.
    pub fn submit(
        &mut self,
        a: &[(i64, i64, u8)],
        b: &[(i64, i64, u8)],
    ) -> io::Result<(Reply, Option<Comprehension>)> {
        let reply = self.session.submit(a, b)?;
        let heard = match reply.verdict.taught() {
            t if t.is_empty() => None,
            t => self.epigenome.learn(t),
        };
        Ok((reply, heard))
    }
    pub fn quote(&self) -> io::Result<YieldQuote> {
        self.session.quote()
    }
    pub fn attest(&self, chain: &[(i64, i64, u8, u8)]) -> io::Result<Reply> {
        self.session.attest(chain)
    }
}

/// The quoted yield: the per-lane surface and the open-demand market's top
/// clusters (address, priced μ⃗, and the chain itself — real, attemptable work).
#[derive(Clone, Debug)]
pub struct YieldQuote {
    pub surface: [u64; 4],
    pub clusters: Vec<Cluster>,
}

#[derive(Clone, Debug)]
pub struct Cluster {
    pub address: [u8; 32],
    pub mu: [u64; 4],
    pub chain: Vec<(i64, i64, u8, u8)>,
}

/// A settle receipt riding an install reply: the germ, its consilience parties,
/// and the polytonic μ⃗ award priced at the moment's yield surface.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SettleReceipt {
    pub germ: [u8; 32],
    pub voices: Vec<[u8; 32]>,
    pub mu: [u64; 4],
}

fn decode_reply(body: &[u8]) -> Reply {
    // Trailing settle receipt (after the taught section), if present.
    let settle = {
        let tl = body.get(5..7).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize).unwrap_or(0);
        body.get(7 + tl..)
            .and_then(dialogue::decode_settle)
            .map(|(germ, voices, mu)| SettleReceipt { germ, voices, mu })
    };
    Reply { verdict: decode_verdict(body), metabolism: decode_metabolism(body), settle }
}

/// A hypha's answer: the verdict plus the per-lane METABOLISM (pressure) — the
/// backpressure signal a kernel throttles its intake against so the mesh is not
/// brute-forced (the syntropic/entropic metabolism as governor, never a scalar).
#[derive(Clone, Debug)]
pub struct Reply {
    pub verdict: Verdict,
    pub metabolism: [u8; 4],
    /// The settle receipt when this exchange completed a consilience install.
    pub settle: Option<SettleReceipt>,
}

impl Reply {
    /// The most-pressured lane — the polytonic backpressure a kernel paces on
    /// (Liebig: the bottleneck lane governs intake).
    pub fn load(&self) -> u8 {
        self.metabolism.iter().copied().max().unwrap_or(0)
    }
}

/// The per-lane pressure bytes from a reply (`body[1..5]`).
fn decode_metabolism(body: &[u8]) -> [u8; 4] {
    std::array::from_fn(|i| body.get(1 + i).copied().unwrap_or(0))
}

/// Decode a hypha reply: `[code][metabolism 4][taught_len u16][taught…]`.
fn decode_verdict(body: &[u8]) -> Verdict {
    let taught = {
        let tl = body.get(5..7).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize).unwrap_or(0);
        body.get(7..7 + tl).map(<[u8]>::to_vec).unwrap_or_default()
    };
    match body.first() {
        Some(1) => Verdict::Decomposed { taught },
        Some(2) => Verdict::GermPending,
        Some(3) => Verdict::Installed { taught },
        Some(4) => Verdict::Pivot,
        Some(5) => Verdict::Fragmented,
        Some(&c) => Verdict::Unknown(c),
        None => Verdict::Unknown(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A kernel's identity is stable and name-derived; distinct names → distinct
    /// voices (so two named kernels can reach consilience, one cannot with itself).
    #[test]
    fn kernel_identity_is_stable_and_distinct() {
        assert_eq!(Kernel::named("sod").voice(), Kernel::named("sod").voice(), "stable seat");
        assert_ne!(Kernel::named("sod").voice(), Kernel::named("net").voice(), "distinct kernels");
    }

    /// THE C-STACK TERMINAL: a taught word enters the GENOME (inheritance via
    /// gene expression); a received utterance is COMPREHENDED — decomposed over
    /// the genome, moving the EPIGENOME's expression weights (the local dialect's
    /// recollection) — never re-cognized, never recalled. And comprehension never
    /// mutates the genome: the inherited record cannot drift locally.
    #[test]
    fn the_entity_comprehends_and_the_epigenome_expresses() {
        use crate::transducer::element::from_residual;
        use crate::transducer::germinate::germinate;
        let mut epi = Epigenome::new();
        // The mesh teaches a word down: first hearing is novel, then it is held.
        let word = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 2)], &[(0, 0, 4u8), (0, 1, 2)]).unwrap());
        assert!(matches!(epi.learn(&word.to_bytes()), Some(Comprehension::Novel { .. })));
        assert_eq!(epi.genome_len(), 1);
        // The same utterance again — COMPREHENDED as the held word; expression rises.
        assert_eq!(epi.comprehend(&word), Comprehension::Understood(vec![0]));
        assert_eq!(epi.peak_expression(), 1, "the dialect's recollection weight moved");
        // An unrelated utterance is heard as novel — a prospective, not a failure —
        // and the genome is untouched by hearing it.
        let other = from_residual(&germinate(&[(0, 0, 5u8)], &[(0, 0, 9u8)]).unwrap());
        assert!(matches!(epi.comprehend(&other), Comprehension::Novel { .. }));
        assert_eq!(epi.genome_len(), 1, "comprehension never mutates the genome");
        // 5a: the genome crystallizes and restores EXACTLY; the epigenome does
        // not persist (expression resets — dialect is ephemeral by ruling).
        let frozen = epi.crystallize();
        let mut back = Epigenome::new();
        assert!(back.uncrystallize(&frozen), "crystallized genome restores");
        assert_eq!(back.genome_len(), epi.genome_len());
        assert_eq!(back.genome()[0].address(), epi.genome()[0].address(), "words identical by address");
        assert_eq!(back.peak_expression(), 0, "expression is NOT persisted — the dialect re-forms");
        assert!(!Epigenome::new().uncrystallize(b"GEN1\xff\xff\xff\xff"), "hostile count refused");
    }

    /// THE CHROMATIC APERTURE (harmonic import): absorption is the EXACT running
    /// average h[k] = (h[k]·n + signal)/(n+1) — signals 3/10 then 7/10 average to
    /// exactly 1/2; dimensions tune independently; the aperture drifts with the
    /// entity's diet, never with a clock.
    #[test]
    fn the_aperture_absorbs_as_an_exact_running_average() {
        use crate::core::engine::Q;
        use num_bigint::BigInt;
        let mut ap = Aperture::default();
        ap.absorb_value_signal(1, 4); // |1−4|/10 = 3/10 into h[0]
        ap.absorb_value_signal(2, 9); // 7/10 → mean = 1/2 exactly
        assert_eq!(ap.bias(0), Some(&Q::new(BigInt::from(1), BigInt::from(2))));
        assert_eq!(ap.count(0), 2);
        ap.absorb_spatial_signal(3); // 3/8 into h[1] — independent dimension
        assert_eq!(ap.bias(1), Some(&Q::new(BigInt::from(3), BigInt::from(8))));
        assert_eq!(ap.tuned_dims(), 2, "dimensions tune independently — no collapse");
    }

    #[test]
    fn verdicts_decode_from_the_wire_reply() {
        // code 3 (installed), metabolism 4 bytes, taught len 2, taught [0xAB,0xCD].
        let body = vec![3u8, 0, 0, 0, 0, 2, 0, 0xAB, 0xCD];
        match decode_verdict(&body) {
            Verdict::Installed { taught } => assert_eq!(taught, vec![0xAB, 0xCD]),
            v => panic!("expected Installed, got {v:?}"),
        }
        assert_eq!(decode_verdict(&[1, 0, 0, 0, 0, 0, 0]), Verdict::Decomposed { taught: vec![] });
        assert_eq!(decode_verdict(&[2, 0, 0, 0, 0, 0, 0]), Verdict::GermPending);
    }
}
