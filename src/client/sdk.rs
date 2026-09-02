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
use crate::arc::{apply_bound_chains, chain_footprint, readback, refine_delta, Grid};
use crate::cognition::{decompose_over, encode_percept, perceive};
use crate::network::wire::{Frame, Stratum};
use crate::transducer::anchor::{derive_carrying_all, Sel};
use crate::transducer::spectral::collapse_beam;
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
    /// Consecutive failed exchanges — the measured transport impedance Z.
    /// Interior mutability: impedance is perception, updated by every
    /// exchange regardless of the calling method's own mutability.
    resistance: std::cell::Cell<u64>,
}

/// The transport aperture — the perceived state of the wire to the server
/// (distinct from the epigenome's chromatic `Aperture`). A timeout or
/// refusal is not an anonymous error: it is a measured physical condition
/// (an impedance mismatch — a closed door or a saturated manifold). The
/// entity perceives it, folds it into its metabolic pacing, and adapts its
/// speech instead of redialing blind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportAperture {
    /// The last exchange passed — the wire is clear.
    Open,
    /// Resistance measured: `z` consecutive failed exchanges. Speech
    /// continues, paced by the impedance (the dwell scales by 1 + z).
    Impeded { z: u64 },
    /// Sustained saturation — a full stroke-cycle of consecutive failures
    /// (the strokes-per-reserve ratio). The entity turns inward this
    /// stroke: internal study and dialect work instead of speech.
    Occluded { z: u64 },
}

impl Session {
    /// Point a kernel at a hypha address (e.g. `"127.0.0.1:7500"`). No socket is
    /// held; each submit dials fresh.
    pub fn open(addr: &str, kernel: Kernel) -> Self {
        Session { addr: addr.to_string(), kernel, resistance: std::cell::Cell::new(0) }
    }

    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    /// The measured impedance Z: consecutive failed exchanges.
    pub fn impedance(&self) -> u64 {
        self.resistance.get()
    }

    /// The perceived transport aperture, derived from the impedance at the
    /// ratio-law boundary: a full stroke-cycle of consecutive failures
    /// (reserve ÷ spend strokes) reads as occlusion.
    pub fn transport(&self) -> TransportAperture {
        let z = self.resistance.get();
        let cycle = (crate::physics::reserve_max() / crate::physics::spend_unit()).max(1);
        if z == 0 {
            TransportAperture::Open
        } else if z < cycle {
            TransportAperture::Impeded { z }
        } else {
            TransportAperture::Occluded { z }
        }
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
        let out = self.exchange_inner(act_body);
        match &out {
            Ok(_) => self.resistance.set(0),
            Err(_) => self.resistance.set(self.resistance.get().saturating_add(1)),
        }
        out
    }

    fn exchange_inner(&self, act_body: Vec<u8>) -> io::Result<Vec<u8>> {
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
    /// THE TASK DIALECT (ephemeral, never persisted) — the bound chains the
    /// last study() crystallized: γ_local = (π*, Δ) formed from the immediate
    /// family's evidence and verified by virtual rollout before anything is
    /// spoken. Projection consults the dialect FIRST (π_dialect ⊓ π_genome):
    /// the entity reasons with its highest-precision local knowledge without
    /// waiting on the court's slower global cycle.
    dialect: Vec<(Vec<(i64, i64, u8, u8)>, Option<Sel>)>,
    /// The beam's retained ALTERNATIVE READING (joint consensus): when the
    /// head dialect goes silent on a sibling test input, projection falls
    /// to this reading — one reading must serve every test jointly.
    dialect_alt: Vec<(Vec<(i64, i64, u8, u8)>, Option<Sel>)>,
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

    /// RETIRE a word whose meaning the rest of the genome already carries
    /// (the accepted compression bid). Expression rides along (parallel
    /// vector); recollections citing the retired index dissolve — they were
    /// spellings in the old vocabulary — and surviving citations reindex.
    pub fn retire(&mut self, addr: &[u8; 32]) -> bool {
        let Some(i) = self.genome.iter().position(|w| w.address() == *addr) else {
            return false;
        };
        self.genome.remove(i);
        if i < self.expression.len() {
            self.expression.remove(i);
        }
        self.recollections.retain(|(s, _)| !s.contains(&i));
        for (s, _) in self.recollections.iter_mut() {
            for j in s.iter_mut() {
                if *j > i {
                    *j -= 1;
                }
            }
        }
        true
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
    /// Verified dialect findings awaiting the wire (bounded queue) — flushed
    /// as anchor-evidence deposits so local thought feeds the court.
    evidence_out: Vec<([u8; 32], Vec<(i64, i64)>, Vec<(i64, i64, u8)>)>,
    /// Rotating cursor for the refinement probe — one genome word examined
    /// per stroke, so the compression search is metabolic, never a sweep.
    refine_cursor: usize,
}

impl Entity {
    pub fn open(addr: &str, kernel: Kernel) -> Self {
        Entity {
            session: Session::open(addr, kernel),
            epigenome: Epigenome::new(),
            evidence_out: Vec::new(),
            refine_cursor: 0,
        }
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

    /// STUDY — the entity THINKS THROUGH THOUGHT over a family of observed
    /// transitions (the epigenetic dialect engine):
    ///   1. LIFT: each pair's coboundary (refined to the common complex when
    ///      frames differ) decomposes over the GENOME into cited words — the
    ///      superposition of candidate readings.
    ///   2. BEAM: per (word, variant), the witnessed placements across the
    ///      family derive a BEAM of candidate anchor terms (all common
    ///      carriers, tightest-first — optionality retained, never a boolean
    ///      filter; a variant with no common term stays ambient and lets the
    ///      rollout judge it).
    ///   3. ROLLOUT: the crystallization candidate replays VIRTUALLY against
    ///      every pair; a nonzero residual on ANY pair is the one unambiguous
    ///      terminal obstruction — the dialect is pruned whole (curvature,
    ///      not preference).
    ///   4. CRYSTALLIZE + SEED: the surviving dialect γ_local = (π*, Δ) lives
    ///      in the epigenome (ephemeral, by ruling) and every VERIFIED
    ///      binding queues as anchor evidence for the wire — local thought
    ///      seeding the court's global refinement.
    /// Returns the crystallized dialect size (0 = the family is beyond the
    /// entity's current language — honest silence).
    pub fn study(&mut self, family: &[(Grid, Grid)]) -> usize {
        self.epigenome.dialect.clear();
        self.epigenome.dialect_alt.clear();
        if family.is_empty() {
            return 0;
        }
        // 1. LIFT — decompose every pair over the genome.
        let mut deltas: Vec<Vec<(i64, i64, u8, u8)>> = Vec::new();
        let mut scenes: Vec<Grid> = Vec::new();
        let mut cited: Vec<usize> = Vec::new();
        for (a, b) in family {
            let Some((mut d, refined_in, _frame)) = refine_delta(a, b) else { return 0 };
            d.sort_unstable();
            let min_r = d.iter().map(|(r, _, _, _)| *r).min().unwrap_or(0);
            let min_c = d.iter().map(|(_, c, _, _)| *c).min().unwrap_or(0);
            let mut centered: Vec<_> =
                d.iter().map(|(r, c, x, y)| (r - min_r, c - min_c, *x, *y)).collect();
            centered.sort_unstable();
            match decompose_over(self.epigenome.genome(), &centered, None, 4, 16) {
                Ok(sentence) => {
                    for wi in sentence {
                        if !cited.contains(&wi) {
                            cited.push(wi);
                        }
                    }
                }
                Err(_) => return 0, // a mute pair: the family is unspeakable today
            }
            deltas.push(d);
            scenes.push(refined_in);
        }
        // 2. BEAM — per witnessed variant, the common-carrier beam.
        let cells_of = |g: &Grid| -> Vec<(i64, i64, u8)> {
            g.iter()
                .enumerate()
                .flat_map(|(r, row)| {
                    row.iter().enumerate().map(move |(c, &h)| (r as i64, c as i64, h))
                })
                .collect()
        };
        let mut spoken: Vec<(Vec<(i64, i64, u8, u8)>, Option<Sel>)> = Vec::new();
        let mut spoken_alt: Vec<(Vec<(i64, i64, u8, u8)>, Option<Sel>)> = Vec::new();
        let mut findings: Vec<([u8; 32], Vec<(i64, i64)>, Vec<(i64, i64, u8)>)> = Vec::new();
        for &wi in &cited {
            let word_addr = self.epigenome.genome()[wi].address();
            for chain in self.epigenome.genome()[wi].variant_chains() {
                let mut sets: Vec<Vec<Sel>> = Vec::new();
                let mut witnesses: Vec<(Vec<(i64, i64)>, Vec<(i64, i64, u8)>)> = Vec::new();
                for (pi, d) in deltas.iter().enumerate() {
                    let fp = chain_footprint(&chain, d);
                    if fp.is_empty() {
                        continue;
                    }
                    let sc = cells_of(&scenes[pi]);
                    sets.push(derive_carrying_all(&fp, &sc));
                    witnesses.push((fp, sc));
                }
                if witnesses.is_empty() {
                    continue;
                }
                // The beam: every first-set term common to all witnessed pairs
                // is a PROJECTED FUTURE STATE; the SPECTRAL COLLAPSE (DC3)
                // takes the one whose selection resonates with the footprint
                // across every witness — exact per-axis invariants, worst
                // pair governing — with the tightest-first order as tiebreak.
                // The uncollapsed tail is retained optionality for re-study.
                let commons: Vec<Sel> = match sets.split_first() {
                    Some((first, rest)) => first
                        .iter()
                        .filter(|t| rest.iter().all(|s| s.contains(*t)))
                        .cloned()
                        .collect(),
                    None => Vec::new(),
                };
                let binding = collapse_beam(&commons, &witnesses);
                if binding.is_some() {
                    findings.push((word_addr, witnesses[0].0.clone(), witnesses[0].1.clone()));
                }
                // JOINT CONSENSUS (the gate's missing thought-form): the
                // beam's runner-up is retained as a full ALTERNATIVE READING
                // — projection later demands one reading serve EVERY test
                // input jointly, and falls to the alternative when the head
                // reading goes silent on a sibling.
                let alt = match &binding {
                    Some(head) => {
                        let rest: Vec<Sel> =
                            commons.iter().filter(|t| *t != head).cloned().collect();
                        collapse_beam(&rest, &witnesses).or_else(|| binding.clone())
                    }
                    None => None,
                };
                spoken_alt.push((chain.clone(), alt));
                spoken.push((chain, binding));
            }
        }
        // 3. ROLLOUT — each candidate dialect must re-derive its own
        // evidence exactly. The primary is terminal (δσ̂ ≠ 0 prunes the
        // study whole); a failing alternative collapses back to the primary.
        let closes = |dialect: &[(Vec<(i64, i64, u8, u8)>, Option<Sel>)]| {
            family.iter().zip(&scenes).all(|((_, b), refined_in)| {
                let refined_got = apply_bound_chains(refined_in, dialect);
                readback(&refined_got, b.len(), b.first().map_or(0, |r| r.len()))
                    .is_some_and(|g| g == *b)
            })
        };
        if !closes(&spoken) {
            return 0; // δσ̂ ≠ 0 on a pair: terminal curvature, pruned whole
        }
        if spoken_alt != spoken && !closes(&spoken_alt) {
            spoken_alt = spoken.clone();
        }
        // 4. CRYSTALLIZE + SEED.
        let size = spoken.len();
        self.epigenome.dialect = spoken;
        self.epigenome.dialect_alt = spoken_alt;
        self.evidence_out.extend(findings);
        self.evidence_out.truncate(64);
        size
    }

    /// PULL a signed work envelope from the hypha's authoritative registry —
    /// the client never arbitrates raw corpus; the server decides what is
    /// useful work. None = the book is settled (or no registry).
    pub fn pull_task(&self) -> io::Result<Option<dialogue::TaskEnvelope>> {
        let voice = self.session.kernel().voice();
        let body = self.session.speak(dialogue::encode_task_request(&voice))?;
        if body.is_empty() {
            return Ok(None);
        }
        Ok(dialogue::decode_task_envelope(&body))
    }

    /// SUBMIT predicted outputs for a dispatched task. The hypha verifies
    /// against its held answers: (SOLVE_BOUNTY, μ⃗) on the first verified
    /// solve, (SOLVE_FLOOR, μ⃗) on a replay, (SOLVE_NOTHING, 0⃗) otherwise —
    /// the anti-replay collapse is the server's ruling, never the client's.
    pub fn submit_solve(
        &self,
        id: &[u8; 32],
        outputs: &[Grid],
    ) -> io::Result<(u8, [u64; 4])> {
        let voice = self.session.kernel().voice();
        let body = self.session.speak(dialogue::encode_solve(&voice, id, outputs))?;
        Ok(dialogue::decode_solve_receipt(&body).unwrap_or((dialogue::SOLVE_NOTHING, [0; 4])))
    }

    /// The perceived transport aperture (see [`TransportAperture`]) — the
    /// entity's answer to "why can I not speak to the server, and what can
    /// I do?": measured impedance, adapted pacing, and an inward turn under
    /// occlusion.
    pub fn transport(&self) -> TransportAperture {
        self.session.transport()
    }

    /// The measured transport impedance Z (consecutive failed exchanges).
    pub fn impedance(&self) -> u64 {
        self.session.impedance()
    }

    /// SEEK REFINEMENT — the compression probe (0x09, one word per stroke):
    /// advance the cursor to the next genome word and ask, in the entity's
    /// OWN language, whether the rest of the genome already speaks it
    /// (decompose its substance with itself excluded). A derivable word is a
    /// redundancy the vocabulary is carrying — a compression bid. The probe
    /// is a virtual rollout (zero mutation); the hypha's deterministic
    /// replay, not this discovery, decides retirement.
    pub fn seek_refinement(&mut self) -> Option<[u8; 32]> {
        let g = self.epigenome.genome();
        if g.len() < 2 {
            return None;
        }
        self.refine_cursor = (self.refine_cursor + 1) % g.len();
        let i = self.refine_cursor;
        let sub = g[i].substance();
        if sub.sub().is_empty() {
            return None;
        }
        match decompose_over(g, sub.sub(), Some(i), 4, 16) {
            Ok(sentence) if !sentence.is_empty() => Some(g[i].address()),
            _ => None,
        }
    }

    /// BID the refinement on the wire: `(REFINE_ACCEPTED, μ⃗ compression
    /// yield)` if the hypha's replay confirms and the word retires;
    /// `(REFINE_NOTHING, 0⃗)` otherwise. On acceptance the entity drops its
    /// own copy too — the compression is global, and its next decomposition
    /// of the retired meaning re-derives the sentence form.
    pub fn bid_refinement(&mut self, word: &[u8; 32]) -> io::Result<(u8, [u64; 4])> {
        let voice = self.session.kernel().voice();
        let body = self.session.speak(dialogue::encode_refine(&voice, word))?;
        let (code, mu) =
            dialogue::decode_solve_receipt(&body).unwrap_or((dialogue::REFINE_NOTHING, [0; 4]));
        if code == dialogue::REFINE_ACCEPTED {
            self.epigenome.retire(word);
        }
        Ok((code, mu))
    }

    /// PULL THE SYLLABUS — the court-ratified teaching stream, served from
    /// this voice's cursor on the hypha (push-on-contact). Every received
    /// word folds into the genome by address (Epigenome::learn), so both
    /// new vocabulary and rebound anchors propagate; the dialect then
    /// re-resolves against the higher-order language on the next study.
    /// Returns how many words were learned this contact.
    pub fn pull_syllabus(&mut self) -> io::Result<usize> {
        let voice = self.session.kernel().voice();
        let body = self.session.speak(dialogue::encode_syllabus_request(&voice))?;
        let words = dialogue::decode_syllabus_reply(&body).unwrap_or_default();
        let mut learned = 0usize;
        for w in &words {
            if self.epigenome.learn(w).is_some() {
                learned += 1;
            }
        }
        Ok(learned)
    }

    /// FLUSH verified dialect findings to the hypha as anchor evidence — the
    /// entity's local reasoning seeding the court's global refinement.
    pub fn flush_evidence(&mut self) -> io::Result<usize> {
        if self.evidence_out.is_empty() {
            return Ok(0);
        }
        let voice = self.session.kernel().voice();
        let items: Vec<_> = self.evidence_out.drain(..).collect();
        self.session.speak(dialogue::encode_evidence_deposit(&voice, &items))?;
        Ok(items.len())
    }

    /// PROJECT — the entity THINKS with its language, DIALECT FIRST: the
    /// task-local γ_local = (π*, Δ) crystallized by study() is its highest-
    /// precision knowledge and speaks before the genome's global anchors
    /// (π_dialect ⊓ π_genome); with no dialect at hand, the court-refined
    /// genome speaks alone. Only bound speech acts; the unrefined stay
    /// silent — a young mind projects humbly.
    pub fn project(&self, grid: &crate::arc::Grid) -> (crate::arc::Grid, Vec<usize>) {
        self.project_variant(grid, 0)
    }

    /// The number of candidate readings the last study retained (1 when the
    /// beam had no distinct runner-up). Joint consensus iterates these: one
    /// reading must serve every test input of a family.
    pub fn dialect_variants(&self) -> usize {
        if !self.epigenome.dialect_alt.is_empty()
            && self.epigenome.dialect_alt != self.epigenome.dialect
        {
            2
        } else {
            1
        }
    }

    /// Project with a specific retained reading: 0 = the spectral head,
    /// 1 = the beam's alternative (rollout-verified or collapsed to the
    /// head at study time).
    pub fn project_variant(
        &self,
        grid: &crate::arc::Grid,
        variant: usize,
    ) -> (crate::arc::Grid, Vec<usize>) {
        let dialect = match variant {
            0 => &self.epigenome.dialect,
            _ => &self.epigenome.dialect_alt,
        };
        if !dialect.is_empty() {
            let out = apply_bound_chains(grid, dialect);
            let spoke: Vec<usize> =
                if out != *grid { (0..dialect.len()).collect() } else { Vec::new() };
            return (out, spoke);
        }
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

    /// Transport impedance is perceived, not thrown: failed exchanges raise
    /// Z monotonically, the aperture derives Impeded then Occluded at the
    /// ratio-law boundary, and one successful exchange restores Open.
    #[test]
    fn impedance_is_measured_and_the_aperture_derives() {
        // A session pointed at a port nothing listens on: every exchange
        // fails, so resistance accumulates deterministically.
        let s = Session::open("127.0.0.1:1", Kernel::named("impedance-test"));
        assert_eq!(s.transport(), TransportAperture::Open, "no speech yet — presumed clear");
        let cycle = (crate::physics::reserve_max() / crate::physics::spend_unit()).max(1);
        for i in 1..=cycle {
            let _ = s.speak(vec![0xFF]);
            assert_eq!(s.impedance(), i, "each failure is one unit of measured Z");
        }
        assert!(
            matches!(s.transport(), TransportAperture::Occluded { z } if z == cycle),
            "a full stroke-cycle of failures reads as occlusion"
        );
        // The dwell factor the client applies is 1 + Z — derived, monotone.
        assert_eq!(1 + s.impedance(), 1 + cycle);
    }

    /// THE THOUGHT ORGAN: study lifts a family into the epigenetic
    /// superposition, crystallizes a verified task dialect γ_local = (π*, Δ),
    /// queues its findings as court evidence, and project — dialect-first —
    /// solves a fresh instance the global genome alone could not.
    #[test]
    fn study_crystallizes_a_dialect_and_project_solves_with_it() {
        use crate::transducer::element::from_residual;
        // The entity holds ONE inherited word: the 1-cell recolour 1→2 (⊤ —
        // the court has not bound it; alone it projects nothing).
        let mut e = Entity::open("127.0.0.1:1", Kernel::named("study-test"));
        let w = from_residual(&vec![(0, 0, 1, 2)]);
        assert!(e.epigenome.learn(&w.to_bytes()).is_some());
        assert!(e.epigenome.genome()[0].anchor().is_none(), "inherited ⊤");
        // A family: hue-1 cells beside a hue-3 body recolour to 2 — twice,
        // in different arrangements (the family, not one pair, decides).
        let a1 = vec![vec![1, 1, 0], vec![3, 3, 0], vec![0, 0, 0]];
        let b1 = vec![vec![2, 2, 0], vec![3, 3, 0], vec![0, 0, 0]];
        let a2 = vec![vec![0, 3, 1], vec![0, 3, 1], vec![0, 0, 0]];
        let b2 = vec![vec![0, 3, 2], vec![0, 3, 2], vec![0, 0, 0]];
        let family = vec![(a1, b1), (a2, b2)];
        let size = e.study(&family);
        assert!(size > 0, "the dialect crystallized (rollout closed on both pairs)");
        assert!(!e.evidence_out.is_empty(), "verified findings queued for the court");
        // A FRESH instance: same relation, new geometry — the dialect thinks it
        // through; the projected next state is exact.
        let x = vec![vec![0, 0, 0], vec![1, 3, 0], vec![1, 3, 0]];
        let y = vec![vec![0, 0, 0], vec![2, 3, 0], vec![2, 3, 0]];
        let (guess, spoke) = e.project(&x);
        assert!(!spoke.is_empty(), "the dialect spoke");
        assert_eq!(guess, y, "the entity's own thought solved the fresh instance");
        // A new study clears the old dialect — task thought is ephemeral.
        assert_eq!(e.study(&[]), 0);
        let (silent, said) = e.project(&x);
        assert_eq!(silent, x, "no dialect, ⊤ genome: humble silence");
        assert!(said.is_empty());
    }

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
