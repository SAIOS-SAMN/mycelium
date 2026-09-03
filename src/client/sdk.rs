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
use crate::arc::Grid;
use crate::cognition::{encode_percept, perceive};
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
    /// The voice this session is the mouth of.
    voice: crate::voice::Voice,
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
        let voice = crate::voice::Voice::named(kernel.name());
        Session { addr: addr.to_string(), kernel, voice, resistance: std::cell::Cell::new(0) }
    }

    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    /// The voice this session speaks with.
    pub fn voice(&self) -> &crate::voice::Voice {
        &self.voice
    }

    /// The voice read the judge's clock.
    pub fn at(&mut self, k: u64) {
        self.voice.at(k);
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
    /// SPEAK a percept with its PROOF: the bodies, the measured frame, and
    /// per germ the citations this kernel reduced it to over its genome.
    pub fn submit(
        &self,
        a: &[(i64, i64, u8)],
        b: &[(i64, i64, u8)],
        frame: &crate::arc::Frame,
        proof: &[Vec<dialogue::CiteWire>],
    ) -> io::Result<Reply> {
        let (a_bodies, b_bodies) = perceive(a, b);
        let mut pbody = vec![dialogue::ACT_PERCEPT];
        pbody.extend_from_slice(&encode_percept(&a_bodies, &b_bodies, &self.kernel.voice, frame, proof));
        let body = self.exchange(pbody)?;
        Ok(decode_reply(&body))
    }

    /// SPEAK a `demand` — this kernel's INDEPENDENT derivation of an open
    /// residual with its citations (empty: irreducible for this voice).
    pub fn attest(&self, chain: &[(i64, i64, u8, u8)], cites: &[dialogue::CiteWire]) -> io::Result<Reply> {
        let body = self.exchange(dialogue::encode_demand(&self.kernel.voice, chain, cites))?;
        Ok(decode_reply(&body))
    }

    /// PROBE an open listing with a tentative sentence; the hypha replays and
    /// settles graded descent or closure. Returns (code, paid, remaining).
    pub fn probe(&self, target: &[u8; 32], cites: &[dialogue::CiteWire]) -> io::Result<(u8, [u64; 4], [u64; 4])> {
        let body = self.exchange(dialogue::encode_probe(&self.kernel.voice, target, cites))?;
        dialogue::decode_probe_reply(&body)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "probe decode"))
    }

    /// ASK the quote — "what is the highest-yield work?" The hypha answers with
    /// the yield surface (per-lane, priced off genuine metabolic tension) and the
    /// open-demand market's top clusters, each carrying its chain so this kernel
    /// can actually attempt the reduction.
    pub fn quote(&self) -> io::Result<YieldQuote> {
        let body = self.exchange(dialogue::encode_quote_request(&self.kernel.voice))?;
        let (surface, k, items) = dialogue::decode_quote_reply(&body)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "quote decode"))?;
        Ok(YieldQuote {
            surface,
            k,
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
        let act = act_body.first().copied();
        let out = self.exchange_inner(act_body);
        // A full reflex pool answers every act with the deferral shape (the
        // pending outcome and four zero lanes). For a percept or a demand
        // that is a verdict; for any other act it is the judge's measured
        // impedance, and the aperture reads it as resistance.
        let deferred = matches!(&out, Ok(b)
            if dialogue::is_deferral(b, crate::cognition::outcome_code(&crate::cognition::Outcome::GermPending))
                && !matches!(act, Some(dialogue::ACT_PERCEPT) | Some(dialogue::ACT_DEMAND)));
        // Impedance is a running measurement: a clear exchange relaxes it
        // by one stroke, a refused or timed-out one raises it by one, so a
        // judge deferring most of the time reads as sustained resistance
        // even when some acts pass.
        match (&out, deferred) {
            (Ok(_), false) => self.resistance.set(self.resistance.get().saturating_sub(1)),
            _ => self.resistance.set(self.resistance.get().saturating_add(1)),
        }
        if deferred {
            return Err(io::Error::new(io::ErrorKind::WouldBlock, "deferred by the judge"));
        }
        out
    }

    fn exchange_inner(&self, act_body: Vec<u8>) -> io::Result<Vec<u8>> {
        let frame = Frame { lane: Stratum::Spatial, body: act_body };
        let bytes = frame.encode().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "frame encode"))?;
        // THE TRANSPORT DEADLINE: no exchange may hang forever — a silent
        // wire must become MEASURED impedance (a hung read is invisible to
        // the aperture; a timed-out one raises Z and the metabolism adapts).
        // The deadline is one full metabolic cycle in dwell-milliseconds
        // (strokes-per-reserve × the base dwell) — hardware-calibrated, no
        // free constant.
        let deadline = std::time::Duration::from_millis(
            crate::physics::reserve_max()
                * (crate::physics::reserve_max() / crate::physics::spend_unit()).max(1),
        );
        let sockaddr = std::net::ToSocketAddrs::to_socket_addrs(&self.addr.as_str())?
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "unresolvable address"))?;
        let mut s = TcpStream::connect_timeout(&sockaddr, deadline)?;
        s.set_read_timeout(Some(deadline))?;
        s.set_write_timeout(Some(deadline))?;
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
    /// THE ATLAS of the genome — the similarity charts of its words, the
    /// precision index thinking locates base points through. Measured from
    /// the genome, rebuilt when the genome's size moves; never persisted.
    atlas: crate::transducer::stalk::Stalks,
    atlas_len: usize,
    /// Each word's shape, lifted once on first use (parallel to the genome).
    shapes: Vec<std::sync::OnceLock<Option<crate::transducer::blade::Shape>>>,
    /// Taught words refused because they did not verify (a transition
    /// that is not a versor, a section that does not span the support) —
    /// the entity's measured discordance with what it was taught.
    refused: u64,
    /// THE EXPRESSED WORDS (ephemeral, never persisted): the residual
    /// bodies this entity germinated itself, before any court ratified
    /// them. They let the entity recognise a residual it has already
    /// claimed, so a claim is spoken once; a ratified word arrives by
    /// teaching and enters the genome.
    expressed: Vec<Element>,
    expressed_atlas: crate::transducer::stalk::Stalks,
    expressed_shapes: Vec<std::sync::OnceLock<Option<crate::transducer::blade::Shape>>>,
    /// Every genome word reached by its address (index-aligned with
    /// `genome`): a keyed relation, never a walk of the genome.
    index: crate::network::nerve::Keyed<usize>,
}

impl Epigenome {
    /// The held word at an address, read from the key.
    pub fn word_at(&self, addr: &[u8; 32]) -> Option<&Element> {
        self.index.get(addr).and_then(|&i| self.genome.get(i))
    }

    /// Re-key the genome after a removal or a restore shifted positions.
    fn reindex(&mut self) {
        self.index = crate::network::nerve::Keyed::new();
        for (i, w) in self.genome.iter().enumerate() {
            self.index.insert(w.address(), i);
        }
    }

    /// Deepen a held word's fiber with a section this genome verified.
    pub fn puncture(&mut self, i: usize, asg: Vec<(u8, u8)>) -> bool {
        self.genome.get_mut(i).is_some_and(|w| w.puncture(asg))
    }

    /// The count of taught words refused for not verifying.
    pub fn refused(&self) -> u64 {
        self.refused
    }

    /// The words this entity has expressed and not yet had ratified.
    pub fn expressed_len(&self) -> usize {
        self.expressed.len()
    }

    /// EXPRESS a residual: germinate it as this entity's own word. Returns
    /// false when the expressed vocabulary already speaks it (the same body
    /// under a transport) — a claim already made, not to be made again.
    pub fn express(&mut self, residual: &[(i64, i64, u8, u8)]) -> bool {
        if let [] = residual {
            return false;
        }
        let located = {
            let expressed = &self.expressed;
            let shapes = &self.expressed_shapes;
            let shape_at = |i: usize| {
                shapes.get(i).and_then(|c| c.get_or_init(|| crate::transducer::element::shape_of_chain(expressed[i].sub())).as_ref())
            };
            crate::transducer::sentence::deconstruct(expressed, &self.expressed_atlas, &shape_at, residual, None)
        };
        if let ([], []) = (located.residual.as_slice(), located.punctures.as_slice()) {
            return false; // already spoken: the same body under a transport
        }
        let word = crate::transducer::element::from_residual(&residual.to_vec());
        self.expressed_atlas.admit(&crate::transducer::stalk::invariant(word.sub()), self.expressed.len());
        self.expressed_shapes.push(std::sync::OnceLock::new());
        self.expressed.push(word);
        true
    }

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
    /// The genome's atlas, current with the genome.
    pub fn atlas(&mut self) -> &crate::transducer::stalk::Stalks {
        if self.atlas_len != self.genome.len() {
            self.atlas = crate::transducer::stalk::Stalks::over(self.genome.iter().map(|w| w.sub()));
            self.shapes = (0..self.genome.len()).map(|_| std::sync::OnceLock::new()).collect();
            self.atlas_len = self.genome.len();
        }
        &self.atlas
    }

    /// The shape of word `i`, lifted on first use.
    fn shape(&self, i: usize) -> Option<&crate::transducer::blade::Shape> {
        let cell = self.shapes.get(i)?;
        cell.get_or_init(|| crate::transducer::element::shape_of_chain(self.genome[i].sub())).as_ref()
    }

    /// Deconstruct a chain over this genome: the sentence it composes and
    /// the residual no word speaks.
    fn locate(&mut self, chain: &[(i64, i64, u8, u8)], exclude: Option<usize>) -> crate::transducer::sentence::Sentence {
        self.atlas();
        crate::transducer::sentence::deconstruct(&self.genome, &self.atlas, &|i| self.shape(i), chain, exclude)
    }

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
        self.reindex();
        true
    }

    /// COMPREHEND a received utterance: decompose it over the genome (the shared
    /// projective mathematics, `cognition::decompose_over`) and fold the
    /// understanding into the epigenome — expression weights rise on the cited
    /// words, the sentence joins the recollections. The genome is untouched.
    pub fn comprehend(&mut self, utterance: &Element) -> Comprehension {
        // Located through the atlas and the versor, never searched. A taught
        // word is one piece: one stroke locates it.
        let located = self.locate(utterance.sub(), None);
        // The remainder decides: no residual and no fiber to deepen is the
        // understood arm; anything left over is the novelty, measured.
        match (located.residual.len(), located.punctures.len()) {
            (0, 0) => {
                let sentence: Vec<usize> = located.cites.into_iter().map(|(i, _, _)| i).collect();
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
            _ => Comprehension::Novel { residual_cells: located.open(self.genome(), utterance.sub()).len() },
        }
    }

    /// LEARN — the mesh expressed a word down (`teach`): comprehend it against
    /// what is already held, then bank it in the genome if it is new (by base
    /// address). Inheritance, not local invention: only taught words enter.
    pub fn learn(&mut self, taught: &[u8]) -> Option<Comprehension> {
        let mut pos = 0usize;
        let e = Element::from_bytes(taught, &mut pos)?;
        // A taught word is comprehended before it is held: every transport
        // in its connection must be a versor and every section must span
        // its support. A word that does not verify is refused and counted.
        let n = e.sub().len();
        let coherent = e.transitions().iter().all(|(a, g)| a.len() == n && crate::transducer::cga::is_versor(g).is_some())
            && e.sections().iter().all(|s| s.len() == n);
        if !coherent {
            self.refused += 1;
            return None;
        }
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
                // The atlas admits the new word in place when it is current;
                // otherwise it rebuilds on the next lookup.
                if self.atlas_len == self.genome.len() {
                    self.atlas.admit(&crate::transducer::stalk::invariant(e.sub()), self.genome.len());
                    self.shapes.push(std::sync::OnceLock::new());
                    self.atlas_len += 1;
                }
                self.index.insert(e.address(), self.genome.len());
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
        self.reindex();
        true
    }
}

/// THE ENTITY — a kernel identity with its genome/epigenome, conversing through
/// a session: it submits percepts, learns what the mesh teaches (comprehending,
/// not storing), asks quotes, and attests demands.
pub struct Entity {
    session: Session,
    pub epigenome: Epigenome,
    /// Rotating cursor for the refinement probe — one genome word examined
    /// per stroke, so the compression search is metabolic, never a sweep.
    refine_cursor: usize,
    /// Words this voice has spoken up for adoption: said once, the judge
    /// decided (installed, derivable, or refused).
    taught_up: Vec<[u8; 32]>,
    /// The discordance measured at the last sync (strata whose roots differ
    /// from the hypha's); zero is a glued genome — the condition under which
    /// this entity speaks proofs.
    discordance: usize,
    /// ČECH AFTER FETCH: the obstructions measured on the fibre when a
    /// fetched word met the words already held there — (the fetched
    /// word's address, the transition that did not vanish). The digest
    /// only indexed the wire; this is the glue, on the elements.
    obstructions: Vec<([u8; 32], Element)>,
    /// Strata the fibre Čech has already glued (a fetched word's transition
    /// to the held representative vanished): the outer index is not walked
    /// there again. The digest names what differs by address; the geometry
    /// decides what is fetched.
    glued: Vec<crate::transducer::stalk::Invariant>,
}

impl Entity {
    pub fn open(addr: &str, kernel: Kernel) -> Self {
        Entity {
            session: Session::open(addr, kernel),
            epigenome: Epigenome::new(),
            refine_cursor: 0,
            taught_up: Vec::new(),
            discordance: usize::MAX,
            obstructions: Vec::new(),
            glued: Vec::new(),
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
    pub fn seek_refinement(&mut self) -> Option<([u8; 32], Vec<dialogue::CiteWire>)> {
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
        let located = self.epigenome.locate(sub.sub(), Some(i));
        let (cites, residual) = (located.cites, located.residual);
        let g = self.epigenome.genome();
        match (cites.len(), residual.as_slice()) {
            (1.., []) => Some((
                g[i].address(),
                cites
                    .into_iter()
                    .map(|(j, t, asg)| {
                        let body = crate::transducer::sentence::cited_pattern(&g[j], asg).unwrap_or_default();
                        (g[j].address(), t, asg as u32, body)
                    })
                    .collect(),
            )),
            _ => None,
        }
    }

    /// BID the refinement on the wire: `(REFINE_ACCEPTED, μ⃗ compression
    /// yield)` if the hypha's replay confirms and the word retires;
    /// `(REFINE_NOTHING, 0⃗)` otherwise. On acceptance the entity drops its
    /// own copy too — the compression is global, and its next decomposition
    /// of the retired meaning re-derives the sentence form.
    pub fn bid_refinement(&mut self, word: &[u8; 32], cites: &[dialogue::CiteWire]) -> io::Result<(u8, [u64; 4])> {
        let voice = self.session.kernel().voice();
        let body = self.session.speak(dialogue::encode_refine(&voice, word, cites))?;
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
    /// THE SHEAF SYNC (Q3): converge this genome on the hypha's language by
    /// stratum. The hypha's section digests over the atlas arrive; the
    /// genome measures its own over the same cover; only the discordant
    /// strata are fetched, each with this genome's have-list, so only the
    /// lacking words travel (their sections and connection riding along).
    /// Strata per contact follow the two-stroke ratio; a wide stratum
    /// converges over successive contacts. Returns the words learned.
    pub fn sync_genome(&mut self) -> io::Result<usize> {
        use crate::transducer::sheaf;
        let voice = self.session.kernel().voice();
        let body = self.session.speak(dialogue::encode_digests_request(&voice))?;
        let remote = dialogue::decode_digests_reply(&body).unwrap_or_default();
        // The genome's own atlas (current, admitted in place), never rebuilt here.
        self.epigenome.atlas();
        let atlas = self.epigenome.atlas.clone();
        let local = sheaf::digests(&atlas, self.epigenome.genome());
        // The outer index names what differs by address; a stratum the
        // fibre Čech already glued is not walked again.
        let strata: Vec<_> = sheaf::discordant(&remote, &local)
            .into_iter()
            .filter(|d| !self.glued.contains(&d.inv))
            .cloned()
            .collect();
        self.discordance = strata.len();
        // One exchange per contact: every discordant chart with this
        // genome's have-list, the request and the reply each filled to the
        // carrier. A wide discordance converges over contacts.
        let charts: Vec<dialogue::Chart> = strata
            .iter()
            .map(|d| (d.inv.clone(), sheaf::have(&atlas, self.epigenome.genome(), &d.inv)))
            .collect();
        let request = dialogue::encode_fetch(&voice, &charts, crate::network::wire::BODY_BUDGET);
        let mut learned = 0usize;
        let body = self.session.speak(request)?;
        for w in dialogue::decode_syllabus_reply(&body).unwrap_or_default() {
            let mut pos = 0usize;
            let fetched = Element::from_bytes(&w, &mut pos);
            if self.epigenome.learn(&w).is_some() {
                learned += 1;
            }
            // ČECH AFTER FETCH: the word is glued on the elements, not on
            // the root that indexed it. Every held word on the same fibre
            // with a group view is a chart overlapping this one; the
            // transition between them is the obstruction, and only a
            // transition with no axis and no chain glues. What does not
            // vanish is kept as the dialect measured, never discarded.
            let Some(fetched) = fetched else { continue };
            if let [] = fetched.axes().as_slice() {
                continue; // no group view: the additive coboundary has no transition
            }
            self.epigenome.atlas();
            let inv = crate::transducer::stalk::invariant(fetched.sub());
            let fibre: Vec<usize> = self.epigenome.atlas.fiber(&inv).to_vec();
            let genome = self.epigenome.genome();
            // One transition, to the stratum's representative: the first
            // held word on the fibre with a group view that is not the
            // fetched word itself. Siblings are interchangeable under the
            // class's symmetry; the transition to one of them is the glue.
            let representative = fibre
                .iter()
                .filter_map(|&i| genome.get(i))
                .find(|held| held.address() != fetched.address() && !held.axes().is_empty());
            let Some(held) = representative else { continue };
            let Some(g) = crate::transducer::cech::transition(held, &fetched) else { continue };
            match (g.axes().as_slice(), g.sub()) {
                ([], []) => {
                    if !self.glued.contains(&inv) {
                        self.glued.push(inv);
                    }
                }
                _ => {
                    if !self.obstructions.iter().any(|(a, _)| *a == fetched.address()) {
                        // THE OBSTRUCTION RIDES UP: the two views of one
                        // object are spoken to the hypha as a percept pair
                        // (held view → fetched view), a demand on the book
                        // for the word that carries one onto the other. The
                        // docket takes it to the court, where it lands on
                        // the Dialect arm if the federation cannot glue it.
                        let scene = |w: &Element| -> Vec<(i64, i64, u8)> { w.sub().iter().map(|&(r, c, _, h)| (r, c, h)).collect() };
                        let (a, b) = (scene(held), scene(&fetched));
                        let extent = |s: &[(i64, i64, u8)]| -> (usize, usize) {
                            let h = s.iter().map(|c| c.0 + 1).max().unwrap_or(0).max(1) as usize;
                            let w = s.iter().map(|c| c.1 + 1).max().unwrap_or(0).max(1) as usize;
                            (h, w)
                        };
                        let ((ha, wa), (hb, wb)) = (extent(&a), extent(&b));
                        let _ = self.session.submit(&a, &b, &(ha, wa, hb, wb), &[]);
                        self.obstructions.push((fetched.address(), g));
                    }
                }
            }
        }
        // THE OTHER DIRECTION OF THE SHEAF: in a discordant stratum this
        // genome may hold what the hypha lacks (a basis that regressed, a
        // word learned from another judge). Those words are taught up for
        // adoption, one contact's budget per stroke; the hypha verifies
        // and installs what it cannot already derive.
        let mut taught = 0usize;
        let mut used = 0usize;
        'strata: for (_, held) in &charts {
            for &addr in held {
                if self.taught_up.contains(&addr) {
                    continue; // spoken for adoption once; the judge decided
                }
                let Some(word) = self.epigenome.word_at(&addr) else { continue };
                let bytes = word.to_bytes();
                if used + bytes.len() > crate::network::wire::BODY_BUDGET {
                    break 'strata;
                }
                used += bytes.len();
                if self.session.speak(dialogue::encode_adopt(&voice, &bytes)).is_ok() {
                    self.taught_up.push(addr);
                    taught += 1;
                }
            }
        }
        Ok(learned + taught)
    }

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




    pub fn kernel(&self) -> &Kernel {
        self.session.kernel()
    }
    /// Submit a percept; COMPREHEND anything taught back (the C-stack, not a
    /// byte-store). Returns the reply and the comprehension of the teaching.
    /// CITE: this kernel's reduction of a chain over its own genome — the
    /// citations (address, transport, gauge coordinate) and the residual it
    /// could not reduce. Cognition happens here; the hypha only replays.
    pub fn cite(&mut self, chain: &[(i64, i64, u8, u8)]) -> (Vec<dialogue::CiteWire>, Vec<(i64, i64, u8, u8)>) {
        let located = self.epigenome.locate(chain, None);
        let genome = self.epigenome.genome();
        // Each citation carries its operator: the word's body under the
        // gauge it was read at, so the hearer verifies what was said.
        let mut cites: Vec<dialogue::CiteWire> = located
            .cites
            .into_iter()
            .map(|(i, g, asg)| {
                let body = crate::transducer::sentence::cited_pattern(&genome[i], asg).unwrap_or_default();
                (genome[i].address(), g, asg as u32, body)
            })
            .collect();
        // A puncture is spoken as a section of the held word: the operator
        // carries the word's cells under the body's own transitions. This
        // genome deepens its own copy at once; the hearer deepens on
        // verification.
        let punctures = located.punctures;
        for (i, g, body) in &punctures {
            cites.push((genome[*i].address(), g.clone(), u32::MAX, body.clone()));
        }
        let residual = located.residual;
        for (i, _, body) in punctures {
            self.epigenome.puncture(i, crate::transducer::element::assignment(&body));
        }
        (cites, residual)
    }

    /// SPEAK a percept as a proof: perceive, germinate, reduce every germ
    /// over the genome, and submit the citations with the bodies.
    pub fn submit(
        &mut self,
        a: &[(i64, i64, u8)],
        b: &[(i64, i64, u8)],
        frame: &crate::arc::Frame,
    ) -> io::Result<(Reply, Option<Comprehension>)> {
        let (a_bodies, b_bodies) = perceive(a, b);
        let proof: Vec<Vec<dialogue::CiteWire>> = crate::cognition::germinate_bodies(&a_bodies, &b_bodies, frame)
            .map(|(germs, _)| {
                germs.iter().map(|(g, _)| self.cite(g.substance().sub()).0).collect()
            })
            .unwrap_or_default();
        let reply = self.session.submit(a, b, frame, &proof)?;
        let heard = match reply.verdict.taught() {
            t if t.is_empty() => None,
            t => self.epigenome.learn(t),
        };
        Ok((reply, heard))
    }
    pub fn quote(&self) -> io::Result<YieldQuote> {
        self.session.quote()
    }
    /// This entity's voice read the judge's clock.
    pub fn at(&mut self, k: u64) {
        self.session.at(k);
    }
    pub fn attest(&self, chain: &[(i64, i64, u8, u8)], cites: &[dialogue::CiteWire]) -> io::Result<Reply> {
        self.session.attest(chain, cites)
    }
    /// CONCLUDE a listing in this genome's own algebra: the chain is
    /// deconstructed into the words that compose it; the residual is
    /// expressed as this entity's own word. Returns the citations, the
    /// residual, and whether the residual was already claimed (familiar):
    /// a familiar residual is not spoken again.
    pub fn conclude(
        &mut self,
        chain: &[(i64, i64, u8, u8)],
    ) -> (Vec<dialogue::CiteWire>, Vec<(i64, i64, u8, u8)>, bool) {
        let (cites, residual) = self.cite(chain);
        let familiar = !self.epigenome.express(&residual);
        (cites, residual, familiar)
    }
    pub fn probe(&self, target: &[u8; 32], cites: &[dialogue::CiteWire]) -> io::Result<(u8, [u64; 4], [u64; 4])> {
        self.session.probe(target, cites)
    }
    /// The discordance measured at the last sync: strata whose roots differ
    /// from the hypha's. Zero is a glued genome.
    /// The Čech obstructions measured after fetch: fetched words whose
    /// transition to a held word on the same fibre did not vanish.
    pub fn obstructions(&self) -> &[([u8; 32], Element)] {
        &self.obstructions
    }

    /// SPEAK A WORD this entity minted — a connection with its remainder —
    /// up to the hypha for adoption. The word enters the entity's own
    /// genome as its expressed thought and travels as itself on the wire.
    pub fn speak_word(&mut self, word: &Element) -> io::Result<()> {
        let voice = self.session.kernel().voice();
        let addr = word.address();
        if self.taught_up.contains(&addr) {
            return Ok(()); // spoken once; the judge decided
        }
        self.session.speak(dialogue::encode_adopt(&voice, &word.to_bytes()))?;
        self.taught_up.push(addr);
        Ok(())
    }

    pub fn discordance(&self) -> usize {
        self.discordance
    }
}

/// The quoted yield: the per-lane surface and the open-demand market's top
/// clusters (address, priced μ⃗, and the chain itself — real, attemptable work).
#[derive(Clone, Debug)]
pub struct YieldQuote {
    pub surface: [u64; 4],
    /// The judge's k-step at the quote — the temporal coordinate of the
    /// book this quote reads.
    pub k: u64,
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
