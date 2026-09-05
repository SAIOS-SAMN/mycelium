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
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

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
    /// Seat a kernel on verifying-key bytes (home/voice.key).
    pub fn from_voice(voice: [u8; 32]) -> Self {
        Kernel {
            name: format!("{:02x}{:02x}", voice[0], voice[1]),
            voice,
        }
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
    /// A section of a held word. Not an install.
    Sectioned,
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
            Verdict::Sectioned => "section",
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

/// A kernel's session with a hypha. Identity is stable; sockets are pooled
/// keep-alives per destination so a quote/probe/attest does not open a new
/// TCP handshake for every act.
pub struct Session {
    addr: String,
    kernel: Kernel,
    /// The voice this session is the mouth of.
    voice: crate::voice::Voice,
    /// Consecutive failed exchanges per hypha address.
    resistance: RefCell<std::collections::HashMap<String, u64>>,
    /// THE NERVE, as this entity holds it: every hypha it has learned of,
    /// placed by the same XOR router the hyphas use, so an act carrying a
    /// key (a listing, a task, a word) is spoken to that key's home and a
    /// stranger there is the second voice. The dial address is the seed.
    router: crate::network::nerve::Router,
    /// Peer addresses keyed by identity (blake3 of the contact string).
    peers: crate::network::nerve::Keyed<String>,
    /// The quote's stroke: quotes walk the nerve in turn, one hypha per
    /// stroke, so every book is thought about and none is a monologue.
    strokes: std::cell::Cell<u64>,
    /// Keep-alive streams keyed by destination address.
    streams: RefCell<HashMap<String, TcpStream>>,
    /// Cover trie learned from digest replies. Same read as the hypha.
    cover: RefCell<crate::network::nerve::Cover>,
    rooms: RefCell<crate::network::nerve::Keyed<[u64; 4]>>,
    addrs: RefCell<crate::network::nerve::Keyed<String>>,
    /// Last measured round-trip per hypha address; a timeout raises it by
    /// the aperture ratio, a success sets it to the measured trip.
    rtt: RefCell<HashMap<String, Duration>>,
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
    /// Point a kernel at a hypha address (e.g. `"127.0.0.1:7500"`).
    pub fn open(addr: &str, kernel: Kernel) -> Self {
        let voice = crate::voice::Voice::from_identity(kernel.voice());
        let mut router = crate::network::nerve::Router::new();
        router.insert(*blake3::hash(addr.as_bytes()).as_bytes(), addr.to_string());
        Session {
            addr: addr.to_string(),
            kernel,
            voice,
            resistance: RefCell::new(std::collections::HashMap::new()),
            router,
            peers: {
                let mut p = crate::network::nerve::Keyed::new();
                p.insert(*blake3::hash(addr.as_bytes()).as_bytes(), addr.to_string());
                p
            },
            strokes: std::cell::Cell::new(0),
            streams: RefCell::new(HashMap::new()),
            cover: RefCell::new(crate::network::nerve::Cover::new()),
            rooms: RefCell::new(crate::network::nerve::Keyed::new()),
            addrs: RefCell::new(crate::network::nerve::Keyed::new()),
            rtt: RefCell::new(HashMap::new()),
        }
    }

    /// DISCOVER the nerve: ask the dial hypha for the addresses it stands
    /// among, then take a cover digest from every learned address so the
    /// first quote already has the trie.
    pub fn discover(&mut self) {
        let Ok(body) = self.exchange_at(&self.addr.clone(), dialogue::encode_peers_request(&self.kernel.voice)) else { return };
        for a in dialogue::decode_peers_reply(&body).unwrap_or_default() {
            let k = *blake3::hash(a.as_bytes()).as_bytes();
            if !self.peers.contains(&k) {
                self.router.insert(k, a.clone());
                self.peers.insert(k, a);
            }
        }
        self.refresh_covers();
    }

    /// Digest every learned hypha and absorb its cover. A moved root replaces.
    pub fn refresh_covers(&mut self) {
        let addrs = self.peers();
        let voice = self.kernel.voice;
        for addr in addrs {
            let Ok(body) = self.exchange_at(&addr, dialogue::encode_digests_request(&voice)) else {
                continue;
            };
            self.absorb_digest(&body);
        }
    }

    /// The hypha a key lives on: longest-common-prefix on the cover trie,
    /// the seed when nothing else is known. Same read as the hypha.
    pub fn route(&self, key: &[u8; 32]) -> String {
        self.router.closest(key).map(|(_, a)| a).unwrap_or_else(|| self.addr.clone())
    }

    /// Home a word by its chart and magnitude. Occluded edges drop out of
    /// the fiber so incidence falls.
    pub fn route_chart(&self, inv: &crate::transducer::stalk::Invariant, mag: &[u64; 4]) -> String {
        use crate::network::nerve::home;
        let cover = self.cover.borrow();
        let rooms = self.rooms.borrow();
        let addrs = self.addrs.borrow();
        // Impedance is a coordinate on the address entry; occluded hyphas
        // drop out of rooms rather than rebuilding a live trie.
        let mut open_rooms = crate::network::nerve::Keyed::new();
        for (id, room) in rooms.fiber(&crate::network::nerve::ROOT) {
            if addrs.get(id).is_some_and(|a| self.occluded_at(a)) {
                continue;
            }
            open_rooms.insert(*id, *room);
        }
        let (_, dest) = home(&cover, inv, mag, &open_rooms, &addrs, None);
        if dest.is_empty() {
            self.addr.clone()
        } else {
            dest
        }
    }

    fn absorb_digest(&self, body: &[u8]) {
        let Some((charts, room, cover_roots, ruled_root, vk, addr, sig)) =
            crate::anastome::dialogue::decode_cover_digest(body)
        else {
            return;
        };
        let prefix = crate::anastome::dialogue::cover_signed_prefix(&cover_roots, &ruled_root, &room);
        if !crate::court::identity::verify(&vk, &prefix, &sig) {
            return;
        }
        self.cover.borrow_mut().retract_hypha(&vk);
        for (lane, inv) in &charts {
            self.cover.borrow_mut().admit(*lane, inv, vk);
        }
        self.rooms.borrow_mut().insert(vk, room);
        self.addrs.borrow_mut().insert(vk, addr);
    }

    /// The hyphas this entity knows.
    pub fn peers(&self) -> Vec<String> {
        self.peers.fiber(&crate::network::nerve::ROOT).map(|(_, a)| a.clone()).collect()
    }

    pub fn add_peer(&mut self, addr: &str) {
        let k = *blake3::hash(addr.as_bytes()).as_bytes();
        if self.peers.contains(&k) {
            return;
        }
        self.router.insert(k, addr.to_string());
        self.peers.insert(k, addr.to_string());
    }

    /// Speak an act carrying a key to that key's home.
    pub fn speak_keyed(&self, key: &[u8; 32], act_body: Vec<u8>) -> io::Result<Vec<u8>> {
        let addr = self.route(key);
        self.exchange_at(&addr, act_body)
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
        self.resistance.borrow().values().copied().max().unwrap_or(0)
    }

    pub fn rtt(&self) -> Duration {
        self.rtt_of(&self.addr)
    }

    fn rtt_of(&self, addr: &str) -> Duration {
        self.rtt.borrow().get(addr).copied().unwrap_or(Duration::from_millis(1))
    }

    fn rtt_ok(&self, addr: &str, dt: Duration) {
        self.rtt.borrow_mut().insert(addr.to_string(), dt.max(Duration::from_micros(1)));
    }

    fn rtt_timeout(&self, addr: &str) {
        let ratio = crate::physics::aperture_cycle().max(2);
        let mut m = self.rtt.borrow_mut();
        let cur = m.get(addr).copied().unwrap_or(Duration::from_millis(1));
        m.insert(addr.to_string(), cur.saturating_mul(ratio));
    }

    fn z_at(&self, addr: &str) -> u64 {
        self.resistance.borrow().get(addr).copied().unwrap_or(0)
    }

    fn occluded_at(&self, addr: &str) -> bool {
        let cycle = crate::physics::aperture_cycle() as u64;
        self.z_at(addr) >= cycle
    }

    /// The perceived transport aperture, derived from the impedance at the
    /// ratio-law boundary: a full stroke-cycle of consecutive failures
    /// (reserve ÷ spend strokes) reads as occlusion.
    pub fn transport(&self) -> TransportAperture {
        let z = self.impedance();
        let cycle = crate::physics::aperture_cycle() as u64;
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
        let key = crate::transducer::element::listing_key(chain, &crate::transducer::element::IDENTITY_FRAME);
        let inv = crate::transducer::stalk::invariant(chain);
        let mag = crate::transducer::element::measure_lanes(chain);
        let dest = self.route_chart(&inv, &mag);
        let body = self.exchange_at(&dest, dialogue::encode_demand(&self.kernel.voice, chain, cites))?;
        let _ = key;
        Ok(decode_reply(&body))
    }

    /// PROBE an open listing with a tentative sentence; the hypha replays and
    /// settles graded descent or closure. Returns (code, paid, remaining).
    pub fn probe(&self, target: &[u8; 32], cites: &[dialogue::CiteWire]) -> io::Result<(u8, [u64; 4], [u64; 4])> {
        let body = self.speak_keyed(target, dialogue::encode_probe(&self.kernel.voice, target, cites))?;
        dialogue::decode_probe_reply(&body)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "probe decode"))
    }

    /// ASK the quote — "what is the highest-yield work?" The hypha answers with
    /// the yield surface (per-lane, priced off genuine metabolic tension) and the
    /// open-demand market's top clusters, each carrying its chain so this kernel
    /// can actually attempt the reduction.
    pub fn quote(&self, charts: &[crate::transducer::stalk::Invariant]) -> io::Result<YieldQuote> {
        let n = self.strokes.get();
        self.strokes.set(n.wrapping_add(1));
        let mut last_err = io::Error::new(io::ErrorKind::NotConnected, "no hypha");
        let peer_list = self.peers();
        let start = (n as usize) % peer_list.len().max(1);
        let npeers = peer_list.len().max(1);
        let any_open = (0..npeers).any(|i| !self.occluded_at(&peer_list[(start + i) % npeers]));
        for i in 0..npeers {
            let addr = peer_list[(start + i) % npeers].clone();
            if any_open && self.occluded_at(&addr) {
                continue;
            }
            match self.exchange_at(
                &addr,
                dialogue::encode_quote_request_charts(&self.kernel.voice, charts),
            ) {
                Ok(body) => {
                    let (surface, k, items) = dialogue::decode_quote_reply(&body)
                        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "quote decode"))?;
                    return Ok(YieldQuote {
                        surface,
                        k,
                        clusters: items
                            .into_iter()
                            .map(|(address, mu, chain, open)| Cluster {
                                address,
                                mu,
                                chain,
                                open,
                            })
                            .collect(),
                    });
                }
                Err(e) => last_err = e,
            }
        }
        Err(last_err)
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
        self.exchange_at(&self.addr.clone(), act_body)
    }

    /// One exchange with a named hypha on the nerve; the impedance is
    /// measured on the wire as a whole.
    fn exchange_at(&self, addr: &str, act_body: Vec<u8>) -> io::Result<Vec<u8>> {
        let act = act_body.first().copied();
        let out = self.exchange_inner(addr, act_body);
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
        {
            let mut r = self.resistance.borrow_mut();
            let z = r.entry(addr.to_string()).or_insert(0);
            match (&out, deferred) {
                (Ok(_), false) => *z = z.saturating_sub(1),
                _ => *z = z.saturating_add(1),
            }
        }
        if deferred {
            return Err(io::Error::new(io::ErrorKind::WouldBlock, "deferred by the judge"));
        }
        out
    }

    fn exchange_inner(&self, addr: &str, act_body: Vec<u8>) -> io::Result<Vec<u8>> {
        let frame = Frame { lane: Stratum::Spatial, body: act_body };
        let bytes = frame.encode().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "frame encode"))?;
        let deadline = crate::physics::transport_deadline(self.rtt_of(addr));
        let t0 = Instant::now();
        let mut pool = self.streams.borrow_mut();
        if let Some(s) = pool.get_mut(addr) {
            match Self::roundtrip(s, &bytes) {
                Ok(body) => {
                    self.rtt_ok(addr, t0.elapsed());
                    return Ok(body);
                }
                Err(e) => {
                    pool.remove(addr);
                    if e.kind() == io::ErrorKind::TimedOut || e.kind() == io::ErrorKind::WouldBlock {
                        self.rtt_timeout(addr);
                    }
                }
            }
        }
        let deadline = crate::physics::transport_deadline(self.rtt_of(addr));
        let sockaddr = std::net::ToSocketAddrs::to_socket_addrs(&addr)?
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "unresolvable address"))?;
        let mut s = match TcpStream::connect_timeout(&sockaddr, deadline) {
            Ok(s) => s,
            Err(e) => {
                self.rtt_timeout(addr);
                return Err(e);
            }
        };
        s.set_read_timeout(Some(deadline))?;
        s.set_write_timeout(Some(deadline))?;
        let _ = s.set_nodelay(true);
        let body = match Self::roundtrip(&mut s, &bytes) {
            Ok(b) => b,
            Err(e) => {
                self.rtt_timeout(addr);
                return Err(e);
            }
        };
        self.rtt_ok(addr, t0.elapsed());
        Ok(body)
    }

    fn roundtrip(s: &mut TcpStream, bytes: &[u8]) -> io::Result<Vec<u8>> {
        s.write_all(bytes)?;
        let mut head = [0u8; 5];
        s.read_exact(&mut head)?;
        let len = u32::from_le_bytes([head[1], head[2], head[3], head[4]]) as usize;
        if len > crate::network::wire::BODY_BUDGET {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "frame too large"));
        }
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
    /// A CONNECTION heard by holding it: the transition to the held word on
    /// its fibre vanished — the same word, glued.
    Held,
    /// A connection heard whose transition to the fibre's representative
    /// did not vanish: the obstruction element is the dialect, carried.
    Dialect(Element),
    /// A connection on a fibre this genome holds no representative on:
    /// admitted as new geometry.
    Admitted,
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

/// A sentence this mind spoke that the hypha sectioned or settled.
/// Memory only; never serialized.
#[derive(Debug, Clone)]
pub struct Dialect {
    pub sentence: Element,
    pub last_used_k: u64,
    pub uses: u64,
}

/// The crystallized language: words on disk. The epigenome is the rest.
pub struct GenomeView<'a> {
    pub words: &'a [Element],
}

#[derive(Default)]
pub struct Epigenome {
    genome: Vec<Element>,
    /// Expression weighting per genome word — how often this entity speaks/hears
    /// it (the recollection efficiency; parallel to `genome`).
    expression: Vec<u64>,
    /// Locally comprehended sentences, keyed by the sentence's word addresses
    /// (not by basis indices that shift on retire).
    recollections: crate::network::nerve::Keyed<(Vec<[u8; 32]>, u64)>,
    /// The entity's chromatic aperture — its perceptual bias (harmonic import).
    pub aperture: Aperture,
    /// THE ATLAS of the genome — admitted in place as words are learned
    /// or restored; rebuilt only on retire. Never persisted.
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
    expressed_index: crate::network::nerve::Keyed<usize>,
    /// Every genome word reached by its address (index-aligned with
    /// `genome`): a keyed relation, never a walk of the genome.
    index: crate::network::nerve::Keyed<usize>,
    /// Sentences this mind spoke that stood. Memory only.
    dialect: crate::network::nerve::Keyed<Dialect>,
    /// Dialects dropped this process.
    shed_n: u64,
}

impl Epigenome {
    /// The held word at an address, read from the key.
    pub fn word_at(&self, addr: &[u8; 32]) -> Option<&Element> {
        self.index.get(addr).and_then(|&i| self.genome.get(i))
    }

    /// The genome index of an address, read from the key.
    pub fn index_of(&self, addr: &[u8; 32]) -> Option<usize> {
        self.index.get(addr).copied()
    }

    /// Re-key the genome after a removal shifted positions.
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

    pub fn dialect_len(&self) -> usize {
        self.dialect.len()
    }

    pub fn shed_n(&self) -> u64 {
        self.shed_n
    }

    /// Drop dialect unused for an aperture of K, and expression/taught/glued
    /// for words no longer in the genome.
    pub fn shed(&mut self, k: u64) -> usize {
        let aperture = crate::physics::aperture_cycle() as u64;
        let drop: Vec<[u8; 32]> = self
            .dialect
            .fiber(&crate::network::nerve::ROOT)
            .filter(|(_, d)| k.saturating_sub(d.last_used_k) > aperture)
            .map(|(a, _)| *a)
            .collect();
        let n = drop.len();
        for a in &drop {
            self.dialect.remove(a);
        }
        self.shed_n = self.shed_n.saturating_add(n as u64);
        n
    }

    pub fn stand_dialect(&mut self, sentence: Element, k: u64) {
        let addr = sentence.address();
        match self.dialect.get_mut(&addr) {
            Some(d) => {
                d.last_used_k = k;
                d.uses = d.uses.saturating_add(1);
            }
            None => {
                self.dialect.insert(addr, Dialect { sentence, last_used_k: k, uses: 1 });
            }
        }
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
            let index = &self.expressed_index;
            let shape_at = |i: usize| {
                shapes.get(i).and_then(|c| c.get_or_init(|| crate::transducer::element::shape_of_chain(expressed[i].sub())).as_ref())
            };
            crate::transducer::sentence::deconstruct(expressed, &self.expressed_atlas, &|a| index.get(a).copied(), &shape_at, residual, None)
        };
        if let ([], []) = (located.residual.as_slice(), located.punctures.as_slice()) {
            return false; // already spoken: the same body under a transport
        }
        let word = crate::transducer::element::from_residual(&residual.to_vec());
        self.expressed_atlas.admit(&crate::transducer::stalk::invariant_of(&word), word.address());
        self.expressed_index.insert(word.address(), self.expressed.len());
        self.expressed_shapes.push(std::sync::OnceLock::new());
        self.stand_dialect(word.clone(), 0);
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
    pub fn recollections(&self) -> Vec<(Vec<[u8; 32]>, u64)> {
        self.recollections
            .fiber(&crate::network::nerve::ROOT)
            .map(|(_, (a, n))| (a.clone(), *n))
            .collect()
    }

    /// RETIRE a word whose meaning the rest of the genome already carries
    /// (the accepted compression bid). Expression rides along (parallel
    /// vector); recollections citing the retired index dissolve — they were
    /// spellings in the old vocabulary — and surviving citations reindex.
    /// The genome's atlas, admitted in place. Never rebuilt here.
    pub fn atlas(&self) -> &crate::transducer::stalk::Stalks {
        &self.atlas
    }

    pub fn atlas_is_empty(&self) -> bool {
        self.atlas_len == 0
    }

    /// The shape of word `i`, lifted on first use.
    fn shape(&self, i: usize) -> Option<&crate::transducer::blade::Shape> {
        let cell = self.shapes.get(i)?;
        cell.get_or_init(|| crate::transducer::element::shape_of_chain(self.genome[i].sub())).as_ref()
    }

    /// Deconstruct a chain over this genome: the sentence it composes and
    /// the residual no word speaks.
    fn locate(&mut self, chain: &[(i64, i64, u8, u8)], exclude: Option<usize>) -> crate::transducer::sentence::Sentence {
        crate::transducer::sentence::deconstruct(&self.genome, &self.atlas, &|a| self.index.get(a).copied(), &|i| self.shape(i), chain, exclude)
    }

    pub fn retire(&mut self, addr: &[u8; 32]) -> bool {
        let Some(i) = self.index.get(addr).copied() else {
            return false;
        };
        let gone = self.genome[i].address();
        self.genome.remove(i);
        if i < self.expression.len() {
            self.expression.remove(i);
        }
        let drop: Vec<[u8; 32]> = self
            .recollections
            .fiber(&crate::network::nerve::ROOT)
            .filter(|(_, (addrs, _))| addrs.iter().any(|a| a == &gone))
            .map(|(k, _)| *k)
            .collect();
        for k in drop {
            self.recollections.remove(&k);
        }
        self.atlas.remove(&gone);
        if i < self.shapes.len() {
            self.shapes.remove(i);
        }
        self.atlas_len = self.genome.len();
        self.reindex();
        true
    }

    /// COMPREHEND a received utterance: decompose it over the genome (the shared
    /// projective mathematics, `cognition::decompose_over`) and fold the
    /// understanding into the epigenome — expression weights rise on the cited
    /// words, the sentence joins the recollections. The genome is untouched.
    pub fn comprehend(&mut self, utterance: &Element) -> Comprehension {
        // A taught sentence is re-derived as a nerve: germs, faces, cycles.
        // Understanding is re-deriving its cochain, not matching its bytes.
        if utterance.is_sentence() {
            let Some(n) = crate::transducer::sentence::from_element(utterance) else {
                return Comprehension::Novel { residual_cells: utterance.sub().len() };
            };
            if let Some(_reason) = crate::transducer::sentence::replay_nerve(utterance) {
                return Comprehension::Novel { residual_cells: n.remainder.len().max(1) };
            }
            let chart = n.germs.first().and_then(|g| g.target.clone().or(g.source.clone()));
            let mut understood = Vec::new();
            if let Some(inv) = chart {
                let fibre = self.atlas.fiber(&inv);
                if let Some(a) = fibre.first() {
                    if let Some(&i) = self.index.get(a) {
                        if let Some(x) = self.expression.get_mut(i) {
                            *x += 1;
                        }
                        understood.push(i);
                    }
                }
            }
            if n.is_closed() && !understood.is_empty() {
                return Comprehension::Understood(understood);
            }
            if !n.cycles.iter().all(|c| c.holonomy.abelian_identity()) {
                return Comprehension::Dialect(utterance.clone());
            }
            return Comprehension::Admitted;
        }
        // HEAR g BY HOLDING IT: a connection has no chain to cover. It is
        // glued against the representative on its fibre — the transition
        // vanishes (held), does not (dialect, the element carried), or there
        // is no representative (admitted). Never an empty covering.
        if utterance.is_translation() {
            return Comprehension::Admitted;
        }
        if utterance.is_connection() && utterance.sub().is_empty() {
            let inv = crate::transducer::stalk::invariant_of(utterance);
            let representative = self
                .atlas
                .fiber(&inv)
                .iter()
                .filter_map(|a| self.word_at(a))
                .find(|held| held.address() != utterance.address() && !held.axes().is_empty());
            return match representative.and_then(|held| crate::transducer::cech::transition(held, utterance)) {
                None => Comprehension::Admitted,
                Some(g) => match (g.axes().as_slice(), g.sub()) {
                    ([], []) => Comprehension::Held,
                    _ => Comprehension::Dialect(g),
                },
            };
        }
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
                    let mut addrs: Vec<[u8; 32]> = sentence
                        .iter()
                        .filter_map(|&i| self.genome.get(i).map(|w| w.address()))
                        .collect();
                    addrs.sort_unstable();
                    let mut h = blake3::Hasher::new();
                    for a in &addrs {
                        h.update(a);
                    }
                    let key = *h.finalize().as_bytes();
                    match self.recollections.get_mut(&key) {
                        Some((_, n)) => *n += 1,
                        None => {
                            self.recollections.insert(key, (addrs, 1));
                        }
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
        // The syllabus may carry several words back to back (every anchored
        // citation of a sentence): each is learned in turn; the first
        // comprehension is returned.
        let mut pos = 0usize;
        let mut first: Option<Comprehension> = None;
        while pos < taught.len() {
            let before = pos;
            let heard = self.learn_one(taught, &mut pos);
            if pos == before {
                break;
            }
            match heard {
                Some(h) => {
                    if first.is_none() {
                        first = Some(h);
                    }
                }
                None => {
                    if first.is_none() {
                        return None;
                    }
                }
            }
        }
        first
    }

    fn learn_one(&mut self, taught: &[u8], pos: &mut usize) -> Option<Comprehension> {
        let e = Element::from_bytes(taught, pos)?;
        let spelled = !e.is_connection() || e.letters_rederive() == Some(true);
        if !spelled {
            self.refused += 1;
            return None;
        }
        // The APERTURE tunes on what is inherited: each transition refracts a
        // value signal into h[0]; each torsion period a spatial signal into h[1]
        // — the entity's optics drift with its diet (dialect-legitimate).
        for &(_, _, before, after) in e.sub() {
            self.aperture.absorb_value_signal(before, after);
        }
        for (_, l, _) in e.torsion() {
            self.aperture.absorb_spatial_signal(*l);
        }
        let heard = self.comprehend(&e);
        let addr = e.address();
        match self.index.get(&addr).copied() {
            Some(i) => self.genome[i] = e, // the taught form may carry new sections — refresh
            None => {
                self.atlas.admit(&crate::transducer::stalk::invariant_of(&e), e.address());
                self.shapes.push(std::sync::OnceLock::new());
                self.atlas_len = self.atlas_len.saturating_add(1);
                self.index.insert(e.address(), self.genome.len());
                self.genome.push(e);
                self.expression.push(0);
            }
        }
        Some(heard)
    }

    /// CRYSTALLIZE the genome for persistence — the permanent language only.
    /// The epigenome is memory-only. Format: `GEN2 | n u32 | (len u32 | element)…`.
    pub fn crystallize(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + self.genome.len() * 64);
        out.extend_from_slice(b"GEN2");
        out.extend_from_slice(&(self.genome.len() as u32).to_le_bytes());
        for g in &self.genome {
            let b = g.to_bytes();
            out.extend_from_slice(&(b.len() as u32).to_le_bytes());
            out.extend_from_slice(&b);
        }
        out
    }

    /// Restore GEN2, else migrate GEN1 in place. Expression starts at zero.
    pub fn uncrystallize(&mut self, bytes: &[u8]) -> bool {
        if bytes.len() < 8 {
            return false;
        }
        let tag = &bytes[..4];
        if tag != b"GEN2" && tag != b"GEN1" {
            return false;
        }
        let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
        if n > bytes.len().saturating_sub(8) / 5 {
            return false; // untrusted count vs remaining bytes
        }
        let mut genome = Vec::with_capacity(n);
        let mut pos = 8usize;
        let mut skipped = 0usize;
        for _ in 0..n {
            let Some(l4) = bytes.get(pos..pos + 4) else { break };
            let len = u32::from_le_bytes(l4.try_into().unwrap()) as usize;
            pos += 4;
            if len > bytes.len().saturating_sub(pos) {
                break;
            }
            let slice = &bytes[pos..pos + len];
            pos += len;
            let mut ep = 0usize;
            match Element::from_bytes(slice, &mut ep) {
                Some(e) => genome.push(e),
                None => {
                    // Previous GEN1 layouts (pre-fibre-index tails, a
                    // word that will not uncrystallize) must not sink the
                    // rest: residuals are never lost.
                    skipped += 1;
                }
            }
        }
        if genome.is_empty() {
            return false;
        }
        if skipped > 0 {
            eprintln!("genome migrated: {} words, {skipped} skipped", genome.len());
        }
        if tag == b"GEN1" {
            eprintln!("genome migrated GEN1 → GEN2: {} words", genome.len());
        }
        self.expression = vec![0; genome.len()];
        self.genome = genome;
        self.atlas = crate::transducer::stalk::Stalks::default();
        self.shapes.clear();
        for w in &self.genome {
            self.atlas.admit(&crate::transducer::stalk::invariant_of(w), w.address());
            self.shapes.push(std::sync::OnceLock::new());
        }
        self.atlas_len = self.genome.len();
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
    taught_up: crate::network::nerve::Keyed<()>,
    /// The discordance measured at the last sync (strata whose roots differ
    /// from the hypha's); zero is a glued genome — the condition under which
    /// this entity speaks proofs. Unmeasured: the open thought's boundary
    /// cochain norm, never usize::MAX.
    discordance: usize,
    discordance_measured: bool,
    /// ČECH AFTER FETCH: the obstructions measured on the fibre when a
    /// fetched word met the words already held there — (the fetched
    /// word's address, the transition that did not vanish). The digest
    /// only indexed the wire; this is the glue, on the elements.
    obstructions: crate::network::nerve::Keyed<Element>,
    /// Strata the fibre Čech has already glued (a fetched word's transition
    /// to the held representative vanished): the outer index is not walked
    /// there again. The digest names what differs by address; the geometry
    /// decides what is fetched.
    glued: crate::network::nerve::Keyed<()>,
    /// OPEN THOUGHTS: the contemplation over bubbles. Remainders persist;
    /// residuals are never lost on restart.
    contemplation: crate::transducer::bubble::Contemplation,
    /// Genome length last seen by the pivot; words added since dirty their charts.
    thought_gen: usize,
    /// Listings this mind has opened an axis on (Vec-of-relations, keyed by
    /// listing address on read): an axis is opened once per listing.
    opened: crate::network::nerve::Keyed<()>,
    physics: crate::physics::Physics,
    tu: crate::chronometry::TUniversal,
}

impl Entity {
    pub fn add_hypha(&mut self, addr: &str) {
        self.session.add_peer(addr);
    }

    /// Ask the bootstrap hypha for the rest of the nerve.
    pub fn discover(&mut self) {
        self.session.discover();
    }

    pub fn open(addr: &str, kernel: Kernel) -> Self {
        let mut session = Session::open(addr, kernel);
        session.discover();
        Entity {
            session,
            epigenome: Epigenome::new(),
            refine_cursor: 0,
            taught_up: crate::network::nerve::Keyed::new(),
            discordance: 0,
            discordance_measured: false,
            obstructions: crate::network::nerve::Keyed::new(),
            glued: crate::network::nerve::Keyed::new(),
            contemplation: crate::transducer::bubble::Contemplation::new(),
            thought_gen: 0,
            opened: crate::network::nerve::Keyed::new(),
            physics: crate::physics::Physics::default(),
            tu: crate::chronometry::TUniversal::default(),
        }
    }

    pub fn set_tuniversal(&mut self, tu: crate::chronometry::TUniversal) {
        self.tu = tu;
    }

    fn now_tu(&self) -> u64 {
        self.tu.ticks()
    }

    /// Open with CONTINUITY: restore the crystallized genome from `genome_path`
    /// if present (GEN2, migrating GEN1 in place). The epigenome starts empty.
    pub fn open_persistent(addr: &str, kernel: Kernel, genome_path: &str, _thoughts_path: &str) -> Self {
        let mut e = Entity::open(addr, kernel);
        if let Ok(bytes) = std::fs::read(genome_path) {
            if e.epigenome.uncrystallize(&bytes) {
                let n = e.epigenome.genome_len();
                let tag = if bytes.get(..4) == Some(b"GEN2") { "GEN2" } else { "GEN1" };
                if tag == "GEN1" {
                    let _ = e.save_genome(genome_path);
                }
                let kb = std::fs::metadata(genome_path).map(|m| m.len() / 1024).unwrap_or(0);
                eprintln!("genome {n} words ({tag}, {kb} KB), epigenome empty");
            }
        } else {
            eprintln!("genome 0 words, epigenome empty");
        }
        e.thought_gen = e.epigenome.genome_len();
        e
    }

    /// Persist the genome atomically (write-then-rename). Call from the
    /// syntropic phase when the genome has grown.
    pub fn save_genome(&self, genome_path: &str) -> io::Result<()> {
        let tmp = format!("{genome_path}.tmp");
        let bytes = self.epigenome.crystallize();
        std::fs::write(&tmp, bytes)?;
        std::fs::rename(&tmp, genome_path)
    }

    /// Persist open thoughts atomically.
    pub fn save_thoughts(&self, thoughts_path: &str) -> io::Result<()> {
        let tmp = format!("{thoughts_path}.tmp");
        let mut bytes = Vec::new();
        self.contemplation.to_bytes(&mut bytes);
        std::fs::write(&tmp, bytes)?;
        std::fs::rename(&tmp, thoughts_path)
    }


    /// PULL a signed work envelope from the hypha's authoritative registry —
    /// the client never arbitrates raw corpus; the server decides what is
    /// useful work. None = the book is settled (or no registry).
    pub fn pull_task(&self) -> io::Result<Option<dialogue::TaskEnvelope>> {
        let voice = self.session.kernel().voice();
        let body = self.session.speak_keyed(&voice, dialogue::encode_task_request(&voice))?;
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
        self.submit_solve_rule(id, outputs, None)
    }

    /// Submit predicted outputs together with the sentence that spoke them.
    pub fn submit_solve_rule(
        &self,
        id: &[u8; 32],
        outputs: &[Grid],
        rule: Option<&Element>,
    ) -> io::Result<(u8, [u64; 4])> {
        let voice = self.session.kernel().voice();
        let bytes = rule.map(|e| e.to_bytes());
        let body = self.session.speak_keyed(
            id,
            dialogue::encode_solve_with_rule(&voice, id, outputs, bytes.as_deref()),
        )?;
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

    pub fn transport_rtt(&self) -> std::time::Duration {
        self.session.rtt()
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
        let body = self.session.speak_keyed(word, dialogue::encode_refine(&voice, word, cites))?;
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
        let body = self.session.speak_keyed(&voice, dialogue::encode_digests_request(&voice))?;
        self.session.absorb_digest(&body);
        let remote = dialogue::decode_digests_reply(&body).unwrap_or_default();
        // The genome's own atlas (current, admitted in place), never rebuilt here.
        let atlas = self.epigenome.atlas.clone();
        let local = sheaf::digests(&atlas);
        // The outer index names what differs by address; a stratum the
        // fibre Čech already glued is not walked again.
        let strata: Vec<_> = sheaf::discordant(&remote, &local)
            .into_iter()
            .filter(|d| !self.glued.contains(&d.inv.key()))
            .cloned()
            .collect();
        self.discordance = strata.len();
        self.discordance_measured = true;
        // One exchange per contact: every discordant chart with this
        // genome's have-list, the request and the reply each filled to the
        // carrier. A wide discordance converges over contacts.
        let charts: Vec<dialogue::Chart> = strata
            .iter()
            .map(|d| (d.inv.clone(), sheaf::have(&atlas, &d.inv)))
            .collect();
        let request = dialogue::encode_fetch(&voice, &charts, crate::network::wire::frame_budget());
        let mut learned = 0usize;
        let body = self.session.speak_keyed(&voice, request)?;
        for w in dialogue::decode_syllabus_reply(&body).unwrap_or_default() {
            let mut pos = 0usize;
            let fetched = Element::from_bytes(&w, &mut pos);
            if self.absorb_taught(&w).is_some() {
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
            let inv = crate::transducer::stalk::invariant_of(&fetched);
            let fibre: Vec<[u8; 32]> = self.epigenome.atlas.fiber(&inv).to_vec();
            // One transition, to the stratum's representative: the first
            // held word on the fibre with a group view that is not the
            // fetched word itself. Siblings are interchangeable under the
            // class's symmetry; the transition to one of them is the glue.
            let representative = fibre
                .iter()
                .filter_map(|a| self.epigenome.word_at(a))
                .find(|held| held.address() != fetched.address() && !held.axes().is_empty());
            let Some(held) = representative else { continue };
            let Some(g) = crate::transducer::cech::transition(held, &fetched) else { continue };
            match (g.axes().as_slice(), g.sub()) {
                ([], []) => {
                    self.glued.insert(inv.key(), ());
                }
                _ => {
                    if !self.obstructions.contains(&fetched.address()) {
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
                        self.obstructions.insert(fetched.address(), g);
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
                if used + bytes.len() > crate::network::wire::frame_budget() {
                    break 'strata;
                }
                used += bytes.len();
                if self.session.speak_keyed(&addr, dialogue::encode_adopt(&voice, &bytes)).is_ok() {
                    self.taught_up.insert(addr, ());
                    taught += 1;
                }
            }
        }
        Ok(learned + taught)
    }

    pub fn pull_syllabus(&mut self) -> io::Result<usize> {
        let voice = self.session.kernel().voice();
        let body = self.session.speak_keyed(&voice, dialogue::encode_syllabus_request(&voice))?;
        let words = dialogue::decode_syllabus_reply(&body).unwrap_or_default();
        let mut learned = 0usize;
        for w in &words {
            if self.absorb_taught(w).is_some() {
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
        // carries the word's cells under the body's own transitions. The
        // wire carries it; this genome deepens only after the hypha verifies
        // (probe closed/descent or attest), never before the utterance.
        for (i, g, body) in &located.punctures {
            cites.push((genome[*i].address(), g.clone(), u32::MAX, body.clone()));
        }
        (cites, located.residual)
    }

    /// Deepen fibres from puncture cites the hypha has verified. Linear in
    /// the proof; each address is a keyed read.
    pub fn puncture_verified(&mut self, cites: &[dialogue::CiteWire]) -> usize {
        let mut deepened = 0usize;
        for (addr, _, asg, body) in cites {
            if *asg != u32::MAX {
                continue;
            }
            let Some(i) = self.epigenome.index_of(addr) else { continue };
            if self.epigenome.puncture(i, crate::transducer::element::assignment(body)) {
                deepened += 1;
            }
        }
        deepened
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
            t => self.absorb_taught(t),
        };
        Ok((reply, heard))
    }
    pub fn quote(&mut self) -> io::Result<YieldQuote> {
        let thought_charts: Vec<crate::transducer::stalk::Invariant> = {
            let mut charts = Vec::new();
            // index: held bubbles; each cover is a peel of its invariant
            for (_, b) in self.contemplation.bubbles.fiber(&crate::network::nerve::ROOT) {
                let inv = crate::transducer::stalk::invariant(&b.remainder);
                let mut cur = inv;
                loop {
                    if b.cover.contains(&cur.key()) {
                        charts.push(cur.clone());
                    }
                    if cur.0.is_empty() {
                        break;
                    }
                    cur = cur.peel();
                }
            }
            charts
        };
        let charts = thought_charts;
        let mut dests: crate::network::nerve::Keyed<Vec<crate::transducer::stalk::Invariant>> =
            crate::network::nerve::Keyed::new();
        let mut dest_order: Vec<String> = Vec::new();
        let bubbles: Vec<(Vec<(i64, i64, u8, u8)>, [u64; 4])> = self
            .contemplation
            .bubbles
            .fiber(&crate::network::nerve::ROOT)
            .map(|(_, b)| (b.remainder.clone(), b.volume))
            .collect();
        for (remainder, mag) in &bubbles {
            let inv = crate::transducer::stalk::invariant(remainder);
            let dest = self.session.route_chart(&inv, mag);
            let dk = *blake3::hash(dest.as_bytes()).as_bytes();
            match dests.get_mut(&dk) {
                Some(held) => {
                    if !held.iter().any(|h| h == &inv) {
                        held.push(inv);
                    }
                }
                None => {
                    dest_order.push(dest);
                    dests.insert(dk, vec![inv]);
                }
            }
        }
        if dests.is_empty() || self.epigenome.atlas_is_empty() {
            // An empty atlas is incident to the root prefix: the first
            // quote reads the book at depth zero, so a mind need not
            // already hold a word to learn one.
            let root = [crate::transducer::stalk::Invariant(Vec::new())];
            let asked = if charts.is_empty() || self.epigenome.atlas_is_empty() {
                root.as_slice()
            } else {
                charts.as_slice()
            };
            return self.session.quote(asked);
        }
        let mut last_err = io::Error::new(io::ErrorKind::NotConnected, "no hypha");
        let mut out: Option<YieldQuote> = None;
        for dest in dest_order {
            let dk = *blake3::hash(dest.as_bytes()).as_bytes();
            let invs = dests.get(&dk).cloned().unwrap_or_default();
            let asked = if invs.is_empty() { charts.clone() } else { invs };
            match self.session.exchange_at(
                &dest,
                dialogue::encode_quote_request_charts(&self.session.kernel().voice(), &asked),
            ) {
                Ok(body) => {
                    let (surface, k, items) = dialogue::decode_quote_reply(&body)
                        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "quote decode"))?;
                    let clusters: Vec<Cluster> = items
                        .into_iter()
                        .map(|(address, mu, chain, open)| Cluster { address, mu, chain, open })
                        .collect();
                    match &mut out {
                        None => {
                            out = Some(YieldQuote { surface, k, clusters });
                        }
                        Some(q) => {
                            let mut have: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
                            for h in &q.clusters {
                                have.insert(h.address, ());
                            }
                            for c in clusters {
                                if have.contains(&c.address) {
                                    continue;
                                }
                                have.insert(c.address, ());
                                q.clusters.push(c);
                            }
                        }
                    }
                }
                Err(e) => last_err = e,
            }
        }
        out.ok_or(last_err)
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
    pub fn obstructions(&self) -> Vec<([u8; 32], Element)> {
        self.obstructions
            .fiber(&crate::network::nerve::ROOT)
            .map(|(a, g)| (*a, g.clone()))
            .collect()
    }

    /// HOLD A THOUGHT: a remainder this mind has not collapsed, as a bubble
    /// whose cover is a peel of its invariant. Connections are the link of
    /// the new bubble through by_chart.
    pub fn hold_thought(&mut self, remainder: Vec<(i64, i64, u8, u8)>) {
        self.contemplation.hold(remainder);
    }

    /// Hold a withheld sentence as a bubble: germs, nerve, remainder, cover.
    pub fn hold_sentence(&mut self, word: &Element) {
        if let Some(n) = crate::transducer::sentence::from_element(word) {
            self.contemplation.hold_nerve(n);
        } else {
            self.hold_thought(word.sub().to_vec());
        }
    }

    /// The open thoughts held here: remainders not yet closed into words.
    pub fn thoughts(&self) -> Vec<&[(i64, i64, u8, u8)]> {
        self.contemplation.remainders()
    }

    pub fn bubbles(&self) -> usize {
        self.contemplation.len()
    }

    pub fn components(&self) -> usize {
        self.contemplation.component_count()
    }

    pub fn contemplation_fills(&self) -> u64 {
        self.contemplation.fills
    }

    pub fn contemplation_classes(&self) -> u64 {
        self.contemplation.classes
    }

    pub fn thoughts_formed(&self) -> u64 {
        self.contemplation.formed
    }

    pub fn horizon_cells(&self) -> usize {
        self.contemplation.horizon_cells()
    }

    pub fn thought_potential(&self) -> [u64; 4] {
        self.contemplation.potential()
    }

    pub fn torsion_merges(&self) -> u64 {
        self.contemplation.torsion_merges
    }

    /// OPEN AN AXIS ON MULTIPLICITY — the convergence engine's `expand_m`
    /// on this manifold. The sentence's H² class is the coordinate the
    /// current axes could not express; it is seeded first, never from zero.
    /// A listing that closes on several charts without a class still seeds
    /// from the transition on the overlap. Once per listing.
    pub fn open_axis(
        &mut self,
        listing: &[u8; 32],
        chain: &[(i64, i64, u8, u8)],
        charts: &[crate::transducer::stalk::Invariant],
    ) -> Option<[u8; 32]> {
        use crate::transducer::element::{measure_lanes, FIBER_GERMINATED};
        if let Some(a) = self.open_contemplation_axis() {
            return Some(a);
        }
        if charts.len() < 2 || self.opened.contains(listing) {
            return None;
        }
        let representatives: Vec<Element> = {
            let atlas = self.epigenome.atlas();
            let heads: Vec<[u8; 32]> = charts.iter().filter_map(|c| atlas.fiber(c).first().copied()).collect();
            heads.iter().filter_map(|a| self.epigenome.word_at(a).cloned()).collect()
        };
        let [base, other, ..] = representatives.as_slice() else { return None };
        // The glue between the two charts on this listing.
        let glue = crate::transducer::cech::transition(base, other)?;
        let block = glue.fiber(crate::transducer::element::FIBER_SPATIAL)?.clone();
        if !base.is_connection() {
            return None; // the axis stands on a spoken motion, never on a bare residual
        }
        let axis = base.max_axis().map_or(FIBER_GERMINATED, |m| m.max(FIBER_GERMINATED - 1) + 1);
        let mut germ = base.clone();
        germ.set_fiber(axis, block);
        // The opened axis carries the listing's own measure on the lanes it
        // moved, and the charts it joined as fibre volume.
        let mut mag = measure_lanes(chain);
        mag[3] = mag[3].saturating_add(charts.len() as u64);
        germ.set_magnitude(mag);
        self.opened.insert(*listing, ());
        match self.speak_word(&germ) {
            Ok(Utterance::Spoken) => Some(germ.address()),
            _ => None,
        }
    }

    /// Seed from a class at any degree on the contemplation — the coordinate
    /// the current axes could not express. A class is an ideology; the gauge
    /// that would translate one frame into the other is a translation word.
    fn open_contemplation_axis(&mut self) -> Option<[u8; 32]> {
        let classes = self.contemplation.classes_held();
        for (class, _degree) in classes {
            let a = [0u8; 32];
            let b = class.key();
            let Some(t) = crate::transducer::element::from_translation(&class, a, b) else {
                continue;
            };
            match self.speak_word(&t) {
                Ok(Utterance::Spoken) => return Some(t.address()),
                _ => {}
            }
        }
        None
    }

    /// Listings this mind opened an axis on.
    pub fn axes_opened(&self) -> usize {
        self.opened.len()
    }

    /// Expand dirty ideas once per syntropic phase. The atlas is borrowed.
    pub fn expand_ideas(&mut self) {
        let mut budget = self.physics.stroke();
        self.contemplation.set_now(self.now_tu());
        let rotors = crate::network::nerve::Keyed::new();
        let _ = self.contemplation.expand_all(self.epigenome.atlas(), &rotors, &mut budget);
        self.contemplation.retire_on_t(self.physics.reserve[2] as usize);
    }

    /// PIVOT ON A WORD: read by_chart at the word's chart and peels, fill
    /// those ideas under one stroke Budget. The atlas is borrowed, never cloned.
    pub fn pivot_on(&mut self, word: &Element) -> usize {
        let mut budget = self.physics.stroke();
        self.contemplation.set_now(self.now_tu());
        let (_fills, _formed, _classes, spoken) = self.contemplation.pivot_on(word, &mut budget);
        let mut n = 0usize;
        for e in spoken {
            match self.speak_word(&e) {
                Ok(Utterance::Spoken) => n += 1,
                _ => {}
            }
        }
        n
    }

    /// Pivot every genome word added since last stroke.
    pub fn pivot_thoughts(&mut self) -> usize {
        let n = self.epigenome.genome_len();
        if n <= self.thought_gen {
            return 0;
        }
        let added: Vec<Element> = self.epigenome.genome()[self.thought_gen..].to_vec();
        self.thought_gen = n;
        let mut spoken = 0usize;
        for w in &added {
            spoken += self.pivot_on(w);
        }
        spoken
    }

    /// Hear a taught word by pivoting the contemplation before banking it.
    fn absorb_taught(&mut self, taught: &[u8]) -> Option<Comprehension> {
        let mut pos = 0usize;
        while pos < taught.len() {
            let before = pos;
            let Some(e) = Element::from_bytes(taught, &mut pos) else {
                break;
            };
            if pos == before {
                break;
            }
            let _ = self.pivot_on(&e);
        }
        self.epigenome.learn(taught)
    }

    /// SPEAK a family's cohering sentences as one act. Refuses if two
    /// sentences share a germ with a non-identity transition. Each uttered
    /// word enters the dialect (epigenome handles); the genome is frozen.
    pub fn speak(&mut self, sentences: &[Element]) -> io::Result<Vec<u8>> {
        if sentences.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "no sentence"));
        }
        let voice = self.session.kernel().voice();
        let k = self.session.voice().k();
        let bytes: Vec<Vec<u8>> = sentences.iter().map(|e| e.to_bytes()).collect();
        let body = dialogue::encode_speech(&voice, &[0u8; 32], self.now_tu(), &bytes);
        let reply = self.session.speak(body)?;
        let codes = dialogue::decode_speech_reply(&reply).unwrap_or_default();
        for e in sentences {
            self.epigenome.stand_dialect(e.clone(), k);
        }
        Ok(codes)
    }

    /// SPEAK A WORD as a one-sentence family. The word enters the dialect
    /// when it stands; the genome receives it only from the syllabus.
    pub fn speak_word(&mut self, word: &Element) -> io::Result<Utterance> {
        // A thought that still has a remainder is work, not a message: the
        // wire never carries ∂g. A word with a chain is held, not uttered.
        if let [_, ..] = word.sub() {
            self.hold_thought(word.sub().to_vec());
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "a remainder is thought, not speech"));
        }
        // Speech is a connection or a connection with an opened axis; a
        // residual germ or the unit is not a message.
        if word.is_identity() || !(word.is_connection() || !word.opened_axes().is_empty()) {
            return Ok(Utterance::Silent);
        }
        let voice = self.session.kernel().voice();
        let addr = word.address();
        if self.taught_up.contains(&addr) {
            return Ok(Utterance::Already);
        }
        let bytes = word.to_bytes();
        let codes = self.speak(std::slice::from_ref(word))?;
        self.taught_up.insert(addr, ());
        let _ = (voice, bytes);
        match codes.first().copied() {
            Some(dialogue::SPEECH_STOOD) | None => Ok(Utterance::Spoken),
            Some(dialogue::SPEECH_REFUSED) => Ok(Utterance::Silent),
            _ => Ok(Utterance::Spoken),
        }
    }

    pub fn discordance(&self) -> usize {
        if self.discordance_measured {
            self.discordance
        } else {
            self.contemplation.boundary_norm()
        }
    }

    pub fn shed(&mut self, k: u64) -> usize {
        self.epigenome.shed(k)
    }

    pub fn dialect_len(&self) -> usize {
        self.epigenome.dialect_len()
    }

    pub fn ideas_len(&self) -> usize {
        self.contemplation.len()
    }

    pub fn shed_n(&self) -> u64 {
        self.epigenome.shed_n()
    }
}

/// What `speak_word` did with a closed mint. Only [`Utterance::Spoken`]
/// is a fresh frame; the memo and the identity gate are not speech.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Utterance {
    /// Adopted up; the address is now on the memo.
    Spoken,
    /// This address was already uttered; the judge has the word.
    Already,
    /// Identity, or a mint with no group component: not a message.
    Silent,
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
    /// Open (non-flat) charts of the listing's coherence field. The entity
    /// reads these against its own atlas; several may close, none is ranked.
    pub open: Vec<crate::transducer::stalk::Invariant>,
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
        Some(6) => Verdict::Sectioned,
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
        let cycle = crate::physics::aperture_cycle() as u64;
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
        if let Ok(bytes) = std::fs::read("state/client-a.genome") {
            let mut old = Epigenome::new();
            if old.uncrystallize(&bytes) {
                assert!(old.genome_len() > 0, "a prior GEN1 is residuals, not lost");
            }
        }
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
