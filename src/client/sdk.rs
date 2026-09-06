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
    /// Last aperture of RTTs per address; the floor is the min of this window.
    rtt_window: RefCell<HashMap<String, Vec<Duration>>>,
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
            rtt_window: RefCell::new(HashMap::new()),
        }
    }

    /// DISCOVER the nerve: ask every learned hypha for the addresses it
    /// stands among, then take a cover digest from every learned address
    /// so the first quote already has the trie.
    pub fn discover(&mut self) {
        self.ask_peers(&self.addr.clone());
        let known = self.peers();
        for a in known {
            self.ask_peers(&a);
        }
        self.refresh_covers();
    }

    /// One peer per Budget spend so a stroke cannot stall on the whole nerve.
    pub fn discover_budgeted(&mut self, budget: &mut crate::physics::Budget) {
        if !budget.spend() {
            return;
        }
        self.ask_peers(&self.addr.clone());
        let known = self.peers();
        if known.is_empty() {
            return;
        }
        let i = self.peer_cursor() % known.len();
        self.bump_cursor();
        if budget.spend() {
            self.ask_peers(&known[i]);
        }
        if budget.spend() {
            let voice = self.kernel.voice;
            if let Ok(body) = self.exchange_at(&known[i], dialogue::encode_digests_request(&voice)) {
                self.absorb_digest(&body);
            }
        }
    }

    fn ask_peers(&mut self, addr: &str) {
        let Ok(body) = self.exchange_at(addr, dialogue::encode_peers_request(&self.kernel.voice)) else {
            return;
        };
        for a in dialogue::decode_peers_reply(&body).unwrap_or_default() {
            let k = *blake3::hash(a.as_bytes()).as_bytes();
            if !self.peers.contains(&k) {
                self.router.insert(k, a.clone());
                self.peers.insert(k, a);
            }
        }
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
            // Empty cover: XOR-closest known hypha to the chart key, not the seed.
            self.router
                .closest(&inv.key())
                .map(|(_, a)| a)
                .unwrap_or_else(|| self.addr.clone())
        } else {
            dest
        }
    }

    fn peer_cursor(&self) -> usize {
        self.strokes.get() as usize
    }

    fn bump_cursor(&self) {
        let n = self.strokes.get();
        self.strokes.set(n.wrapping_add(1));
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
        let n = crate::physics::aperture_cycle().max(1) as usize;
        let sample = dt.max(Duration::from_micros(1));
        let mut windows = self.rtt_window.borrow_mut();
        let w = windows.entry(addr.to_string()).or_default();
        w.push(sample);
        if w.len() > n {
            w.remove(0);
        }
        let floor = w.iter().copied().min().unwrap_or(sample);
        let cap = crate::physics::persist_dwell(floor);
        self.rtt.borrow_mut().insert(addr.to_string(), sample.min(cap));
    }

    fn rtt_timeout(&self, addr: &str) {
        let ratio = crate::physics::aperture_cycle().max(2);
        let floor = self
            .rtt_window
            .borrow()
            .get(addr)
            .and_then(|w| w.iter().copied().min())
            .unwrap_or(Duration::from_millis(1));
        let cap = crate::physics::persist_dwell(floor);
        let mut m = self.rtt.borrow_mut();
        let cur = m.get(addr).copied().unwrap_or(Duration::from_millis(1));
        let next = cur.saturating_mul(ratio).min(cap);
        m.insert(addr.to_string(), next);
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
        match self.speak_nerve(dialogue::encode_quote_request_charts(&self.kernel.voice, charts)) {
            Ok(body) => {
                let (height, surface, k, items) = dialogue::decode_quote_reply(&body)
                    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "quote decode"))?;
                Ok(YieldQuote {
                    height,
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
                })
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(YieldQuote {
                height: 0,
                surface: [0; 4],
                k: 0,
                clusters: Vec::new(),
            }),
            Err(e) => Err(e),
        }
    }

    /// Speak an unkeyed act on the nerve: every learned hypha, starting at
    /// the stroke. A full reflex pool is impedance, not a failed quote.
    pub fn speak_nerve(&self, act_body: Vec<u8>) -> io::Result<Vec<u8>> {
        let n = self.strokes.get();
        self.strokes.set(n.wrapping_add(1));
        let mut last_err = io::Error::new(io::ErrorKind::NotConnected, "no hypha");
        let peer_list = self.peers();
        let start = (n as usize) % peer_list.len().max(1);
        let npeers = peer_list.len().max(1);
        let any_open = (0..npeers).any(|i| !self.occluded_at(&peer_list[(start + i) % npeers]));
        let mut pass = 0u8;
        while pass < 2 {
            for i in 0..npeers {
                let addr = peer_list[(start + i) % npeers].clone();
                if any_open && self.occluded_at(&addr) {
                    continue;
                }
                match self.exchange_at(&addr, act_body.clone()) {
                    Ok(body) => return Ok(body),
                    Err(e) => last_err = e,
                }
            }
            if last_err.kind() != io::ErrorKind::WouldBlock {
                break;
            }
            pass += 1;
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
            match (&out, deferred) {
                (Ok(_), false) => {
                    // A success anywhere means the aperture is open: every
                    // hypha's Z decays, and the working path is clear.
                    for z in r.values_mut() {
                        *z = z.saturating_sub(1);
                    }
                    r.insert(addr.to_string(), 0);
                }
                _ => {
                    *r.entry(addr.to_string()).or_insert(0) += 1;
                }
            }
        }
        if deferred {
            return Err(io::Error::new(io::ErrorKind::WouldBlock, "deferred by the judge"));
        }
        out
    }

    fn note_slow(tag: &str, act: u8, addr: &str, t0: Instant, deadline: Duration) {
        let dt = t0.elapsed();
        if dt > deadline {
            eprintln!(
                "[{tag}] exchange {act} to {addr} took {} ms deadline {} ms",
                dt.as_millis(),
                deadline.as_millis()
            );
        }
    }

    fn exchange_inner(&self, addr: &str, act_body: Vec<u8>) -> io::Result<Vec<u8>> {
        let act = act_body.first().copied().unwrap_or(0);
        let tag = self.kernel.name();
        let frame = Frame { lane: Stratum::Spatial, body: act_body };
        let bytes = frame.encode().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "frame encode"))?;
        let deadline = crate::physics::transport_deadline(self.rtt_of(addr));
        let t0 = Instant::now();
        let mut pool = self.streams.borrow_mut();
        if let Some(s) = pool.get_mut(addr) {
            match Self::roundtrip(s, &bytes) {
                Ok(body) => {
                    Self::note_slow(tag, act, addr, t0, deadline);
                    self.rtt_ok(addr, t0.elapsed());
                    return Ok(body);
                }
                Err(e) => {
                    pool.remove(addr);
                    if e.kind() == io::ErrorKind::TimedOut || e.kind() == io::ErrorKind::WouldBlock {
                        Self::note_slow(tag, act, addr, t0, deadline);
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
                Self::note_slow(tag, act, addr, t0, deadline);
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
                Self::note_slow(tag, act, addr, t0, deadline);
                self.rtt_timeout(addr);
                return Err(e);
            }
        };
        Self::note_slow(tag, act, addr, t0, deadline);
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
#[derive(Clone, Debug)]
pub enum Comprehension {
    /// The utterance decomposed into held words — address and standing
    /// from the working atlas. Never a sentinel index.
    Understood(Vec<([u8; 32], Standing)>),
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
    /// A sentence cites forms this mind does not hold.
    Withheld { missing: Vec<[u8; 32]> },
}

impl PartialEq for Comprehension {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Comprehension::Understood(a), Comprehension::Understood(b)) => a == b,
            (Comprehension::Novel { residual_cells: a }, Comprehension::Novel { residual_cells: b }) => a == b,
            (Comprehension::Held, Comprehension::Held) | (Comprehension::Admitted, Comprehension::Admitted) => true,
            (Comprehension::Dialect(a), Comprehension::Dialect(b)) => a.same_vertex(b),
            (Comprehension::Withheld { missing: a }, Comprehension::Withheld { missing: b }) => a == b,
            _ => false,
        }
    }
}
impl Eq for Comprehension {}

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
    /// TUniversal of last working-atlas read or stand.
    pub last_used_tu: u64,
    pub uses: u64,
    shape: Option<crate::transducer::blade::Shape>,
    /// A withheld sentence: the cited addresses this mind does not hold.
    pub withheld: bool,
    pub missing: Vec<[u8; 32]>,
}

/// Standing of a fiber member on the working atlas.
pub use crate::transducer::stalk::Standing;

/// One working atlas, three strata. A fiber read is `(address, Standing)`,
/// never a boolean. Genome-only `atlas()` remains for persistence.
pub struct WorkingAtlas<'a> {
    genome: &'a crate::transducer::stalk::Stalks,
    dialect: &'a crate::transducer::stalk::Stalks,
    ideas: Option<&'a crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>>,
}

impl<'a> WorkingAtlas<'a> {
    /// Addresses on this chart, each carrying the stratum it came from.
    /// Ruled first, then Stood, then Held; an address appears once, at its
    /// strongest standing.
    pub fn fiber(&self, inv: &crate::transducer::stalk::Invariant) -> Vec<([u8; 32], Standing)> {
        let mut out: Vec<([u8; 32], Standing)> = Vec::new();
        let mut have: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for a in self.genome.fiber(inv) {
            have.insert(a, ());
            out.push((a, Standing::Ruled));
        }
        for a in self.dialect.fiber(inv) {
            if have.contains(&a) {
                continue;
            }
            have.insert(a, ());
            out.push((a, Standing::Stood));
        }
        if let Some(ideas) = self.ideas {
            if let Some(m) = ideas.get(&inv.key()) {
                for (a, _) in m.fiber(&crate::network::nerve::ROOT) {
                    if have.contains(a) {
                        continue;
                    }
                    have.insert(*a, ());
                    out.push((*a, Standing::Held));
                }
            }
        }
        out
    }

    pub fn standing_of(&self, addr: &[u8; 32], inv: &crate::transducer::stalk::Invariant) -> Option<Standing> {
        self.fiber(inv).into_iter().find(|(a, _)| a == addr).map(|(_, s)| s)
    }

    pub fn standing_addr(&self, addr: &[u8; 32]) -> Option<Standing> {
        if self.genome.contains(addr) {
            return Some(Standing::Ruled);
        }
        if self.dialect.contains(addr) {
            return Some(Standing::Stood);
        }
        if let Some(ideas) = self.ideas {
            if ideas.fiber(&crate::network::nerve::ROOT).any(|(_, m)| m.contains(addr)) {
                return Some(Standing::Held);
            }
        }
        None
    }
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
    /// Charts of stood sentences, admitted in stand_dialect_at, retracted in shed.
    dialect_atlas: crate::transducer::stalk::Stalks,
    /// Addresses promoted from dialect into the genome this learn.
    promoted: Vec<[u8; 32]>,
    /// Dialects dropped this process.
    shed_n: u64,
    /// TUniversal at each of the last aperture_cycle strokes.
    stroke_tu: Vec<u64>,
    /// Held words keyed by (chart, axis). Filled at atlas/dialect admit,
    /// emptied at remove/shed/retire. `on_axis` is one fiber read.
    axis_index: crate::transducer::sentence::AxisIndex,
    /// Missing cited address → citing dialect sentences. Filled when
    /// Withheld is banked; resolve_withheld reads this, never the dialect.
    waiting_on: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
}

impl Epigenome {
    /// The held word at an address, read from the key.
    pub fn word_at(&self, addr: &[u8; 32]) -> Option<&Element> {
        self.index.get(addr).and_then(|&i| self.genome.get(i))
    }

    /// Working-atlas root: blake3 of sorted stratum roots of genome ∪ dialect.
    pub fn atlas_root(&self) -> [u8; 32] {
        let mut rows = crate::transducer::sheaf::digests(&self.atlas);
        rows.extend(crate::transducer::sheaf::digests(&self.dialect_atlas));
        rows.sort_by(|a, b| a.root.cmp(&b.root).then_with(|| a.inv.key().cmp(&b.inv.key())));
        let mut h = blake3::Hasher::new();
        for r in &rows {
            h.update(&r.root);
        }
        *h.finalize().as_bytes()
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

    /// True when a dialect sentence lives on this chart (exact invariant).
    pub fn dialect_holds(&self, c: &crate::transducer::stalk::Invariant) -> bool {
        !self.dialect_atlas.fiber(c).is_empty()
    }

    pub fn shed_n(&self) -> u64 {
        self.shed_n
    }

    /// A working-atlas read of a stood word is a use.
    pub fn touch(&mut self, addr: &[u8; 32], now_tu: u64) {
        if let Some(d) = self.dialect.get_mut(addr) {
            d.last_used_tu = now_tu;
            d.uses = d.uses.saturating_add(1);
        }
    }

    /// Drop dialect unused for an aperture of TUniversal. A dialect entry a live
    /// idea glues is not shed. The genome never sheds.
    pub fn shed(&mut self, now_tu: u64) -> usize {
        self.shed_except(now_tu, &|_| false)
    }

    pub fn note_stroke(&mut self, now_tu: u64) {
        let n = crate::physics::aperture_cycle().max(1) as usize;
        self.stroke_tu.push(now_tu);
        if self.stroke_tu.len() > n {
            self.stroke_tu.remove(0);
        }
    }

    /// TUniversal span of the last aperture of strokes. One unit, measured.
    /// `None` until a whole aperture of strokes has been noted: an
    /// unmeasured aperture is held, never guessed (rule 15), so nothing
    /// sheds in the first aperture after boot.
    fn aperture_tu(&self, now_tu: u64) -> Option<u64> {
        let n = crate::physics::aperture_cycle().max(1) as usize;
        if self.stroke_tu.len() < n {
            return None;
        }
        let first = *self.stroke_tu.first()?;
        Some(now_tu.saturating_sub(first))
    }

    pub fn shed_except(&mut self, now_tu: u64, glued: &impl Fn(&[u8; 32]) -> bool) -> usize {
        let Some(aperture_tu) = self.aperture_tu(now_tu) else {
            return 0;
        };
        let drop: Vec<[u8; 32]> = self
            .dialect
            .fiber(&crate::network::nerve::ROOT)
            .filter(|(a, d)| now_tu.saturating_sub(d.last_used_tu) > aperture_tu && !glued(a))
            .map(|(a, _)| *a)
            .collect();
        let n = drop.len();
        for a in &drop {
            self.drop_dialect(a);
        }
        self.shed_n = self.shed_n.saturating_add(n as u64);
        n
    }

    fn wait_on(&mut self, missing: &[u8; 32], citer: [u8; 32]) {
        if let Some(citers) = self.waiting_on.get_mut(missing) {
            citers.insert(citer, ());
            return;
        }
        let mut citers = crate::network::nerve::Keyed::new();
        citers.insert(citer, ());
        self.waiting_on.insert(*missing, citers);
    }

    fn retract_waiter(&mut self, missing: &[u8; 32], citer: &[u8; 32]) {
        let empty = match self.waiting_on.get_mut(missing) {
            Some(citers) => {
                citers.remove(citer);
                citers.is_empty()
            }
            None => return,
        };
        if empty {
            self.waiting_on.remove(missing);
        }
    }

    fn drop_dialect(&mut self, da: &[u8; 32]) {
        let (inv, words, missing) = match self.dialect.get(da) {
            Some(d) => (
                crate::transducer::stalk::invariant_of(&d.sentence),
                crate::transducer::sentence::words_of_element(&d.sentence),
                if d.withheld {
                    d.missing.clone()
                } else {
                    Vec::new()
                },
            ),
            None => {
                self.dialect_atlas.remove(da);
                self.dialect.remove(da);
                return;
            }
        };
        crate::transducer::sentence::retract_words(&mut self.axis_index, &inv, &words);
        for m in &missing {
            self.retract_waiter(m, da);
        }
        self.dialect_atlas.remove(da);
        self.dialect.remove(da);
    }

    pub fn stand_dialect_at(&mut self, sentence: Element, k: u64, now_tu: u64) -> Option<[u8; 32]> {
        if let Some(i) = self.same_vertex_in_genome(&sentence) {
            if let Some(x) = self.expression.get_mut(i) {
                *x = x.saturating_add(1);
            }
            return None;
        }
        let addr = sentence.address();
        if let Some(d) = self.dialect.get_mut(&addr) {
            d.last_used_k = k;
            d.last_used_tu = now_tu;
            d.uses = d.uses.saturating_add(1);
            return Some(addr);
        }
        if let Some(existing) = self.same_vertex_in_dialect(&sentence) {
            if let Some(d) = self.dialect.get_mut(&existing) {
                d.last_used_k = k;
                d.last_used_tu = now_tu;
                d.uses = d.uses.saturating_add(1);
            }
            return Some(existing);
        }
        let inv = crate::transducer::stalk::invariant_of(&sentence);
        self.dialect_atlas.admit(&inv, addr);
        self.axis_admit(&sentence);
        let shape = crate::transducer::element::shape_of_chain(sentence.sub());
        self.dialect.insert(
            addr,
            Dialect {
                sentence,
                last_used_k: k,
                last_used_tu: now_tu,
                uses: 1,
                shape,
                withheld: false,
                missing: Vec::new(),
            },
        );
        eprintln!(
            "  dialect stood {:02x}{:02x}… k {k} tu {now_tu}",
            addr[0], addr[1]
        );
        Some(addr)
    }

    /// Stand a withheld form in the dialect (24.2, 24.3): `withheld: true`,
    /// never the genome.
    pub fn stand_dialect_withheld(
        &mut self,
        sentence: Element,
        k: u64,
        now_tu: u64,
    ) -> Option<[u8; 32]> {
        let addr = self.stand_dialect_at(sentence, k, now_tu)?;
        if let Some(d) = self.dialect.get_mut(&addr) {
            d.withheld = true;
        }
        Some(addr)
    }

    /// The words this entity has expressed and not yet had ratified.
    pub fn expressed_len(&self) -> usize {
        self.expressed.len()
    }

    /// EXPRESS a residual: germinate it as this entity's own word. Returns
    /// false when the expressed vocabulary already speaks it (the same body
    /// under a transport) — a claim already made, not to be made again.
    /// `k` is the voice's K (law 9); `now_tu` is TUniversal.
    pub fn express(&mut self, residual: &[(i64, i64, u8, u8)], k: u64, now_tu: u64) -> bool {
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
        self.stand_dialect_at(word.clone(), k, now_tu);
        self.expressed.push(word);
        true
    }

    pub fn new() -> Self {
        Epigenome::default()
    }

    fn axis_admit(&mut self, e: &Element) {
        let inv = crate::transducer::stalk::invariant_of(e);
        crate::transducer::sentence::admit_words(
            &mut self.axis_index,
            &inv,
            &crate::transducer::sentence::words_of_element(e),
        );
    }

    fn axis_retract(&mut self, e: &Element) {
        let inv = crate::transducer::stalk::invariant_of(e);
        crate::transducer::sentence::retract_words(
            &mut self.axis_index,
            &inv,
            &crate::transducer::sentence::words_of_element(e),
        );
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
    /// The genome's atlas, admitted in place. Persistence and crystallize
    /// only. Cognitive reads use [`working_atlas`].
    pub fn atlas(&self) -> &crate::transducer::stalk::Stalks {
        &self.atlas
    }

    /// Empty when the working language has no genome word and no dialect word.
    pub fn atlas_is_empty(&self) -> bool {
        self.atlas_len == 0 && self.dialect.is_empty()
    }

    /// Working atlas of genome + dialect. Ideas join at Entity::working_atlas.
    pub fn working_atlas(&self) -> WorkingAtlas<'_> {
        WorkingAtlas {
            genome: &self.atlas,
            dialect: &self.dialect_atlas,
            ideas: None,
        }
    }

    pub fn working_atlas_with<'a>(
        &'a self,
        ideas: &'a crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    ) -> WorkingAtlas<'a> {
        WorkingAtlas {
            genome: &self.atlas,
            dialect: &self.dialect_atlas,
            ideas: Some(ideas),
        }
    }

    /// A word on the working language: genome first, then dialect.
    pub fn working_word(&self, addr: &[u8; 32]) -> Option<&Element> {
        self.word_at(addr)
            .or_else(|| self.dialect.get(addr).map(|d| &d.sentence))
    }

    pub fn take_promoted(&mut self) -> Vec<[u8; 32]> {
        std::mem::take(&mut self.promoted)
    }

    fn same_vertex_in_dialect(&self, e: &Element) -> Option<[u8; 32]> {
        let addr = e.address();
        let d = self.dialect.get(&addr)?;
        debug_assert!(d.sentence.same_vertex(e));
        Some(addr)
    }

    fn same_vertex_in_genome(&self, e: &Element) -> Option<usize> {
        let i = self.index.get(&e.address()).copied()?;
        debug_assert!(self.genome.get(i).is_some_and(|w| w.same_vertex(e)));
        Some(i)
    }

    /// Deconstruct a chain over the working language: genome, dialect, and
    /// extra words (ideas). A word this mind stood closes a remainder
    /// without waiting for the court.
    fn locate(
        &mut self,
        chain: &[(i64, i64, u8, u8)],
        exclude: Option<usize>,
    ) -> crate::transducer::sentence::Sentence {
        self.locate_on(chain, exclude, &[])
    }

    fn locate_on(
        &mut self,
        chain: &[(i64, i64, u8, u8)],
        exclude: Option<usize>,
        extra: &[Element],
    ) -> crate::transducer::sentence::Sentence {
        let n = self.genome.len();
        let dialect_addrs: Vec<[u8; 32]> = self
            .dialect
            .fiber(&crate::network::nerve::ROOT)
            .map(|(a, _)| *a)
            .collect();
        let n_d = dialect_addrs.len();
        let extra_shapes: Vec<Option<crate::transducer::blade::Shape>> = extra
            .iter()
            .map(|w| crate::transducer::element::shape_of_chain(w.sub()))
            .collect();
        let mut extra_atlas = crate::transducer::stalk::Stalks::default();
        for w in extra {
            extra_atlas.admit(&crate::transducer::stalk::invariant_of(w), w.address());
        }
        let genome = &self.genome;
        let dialect = &self.dialect;
        let atlas = &self.atlas;
        let dialect_atlas = &self.dialect_atlas;
        let index = &self.index;
        let shapes = &self.shapes;
        let word_at = |i: usize| -> Option<&Element> {
            if i < n {
                genome.get(i)
            } else if i < n + n_d {
                dialect_addrs.get(i - n).and_then(|a| dialect.get(a).map(|d| &d.sentence))
            } else {
                extra.get(i - n - n_d)
            }
        };
        let index_of = |a: &[u8; 32]| -> Option<usize> {
            index.get(a).copied().or_else(|| {
                dialect_addrs
                    .iter()
                    .position(|x| x == a)
                    .map(|j| n + j)
                    .or_else(|| extra.iter().position(|w| &w.address() == a).map(|j| n + n_d + j))
            })
        };
        let shape_at = |i: usize| -> Option<&crate::transducer::blade::Shape> {
            if i < n {
                shapes.get(i).and_then(|c| c.get_or_init(|| crate::transducer::element::shape_of_chain(genome[i].sub())).as_ref())
            } else if i < n + n_d {
                dialect_addrs.get(i - n).and_then(|a| dialect.get(a).and_then(|d| d.shape.as_ref()))
            } else {
                extra_shapes.get(i - n - n_d).and_then(|s| s.as_ref())
            }
        };
        let fiber_of = |inv: &crate::transducer::stalk::Invariant| {
            let mut v = atlas.fiber(inv);
            for a in dialect_atlas.fiber(inv) {
                if !v.iter().any(|b| b == &a) {
                    v.push(a);
                }
            }
            for a in extra_atlas.fiber(inv) {
                if !v.iter().any(|b| b == &a) {
                    v.push(a);
                }
            }
            v
        };
        crate::transducer::sentence::deconstruct_on(&word_at, &fiber_of, &index_of, &shape_at, chain, exclude)
    }

    pub fn retire(&mut self, addr: &[u8; 32]) -> bool {
        let Some(i) = self.index.get(addr).copied() else {
            return false;
        };
        let gone = self.genome[i].address();
        let held = self.genome[i].clone();
        self.axis_retract(&held);
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
        // A taught sentence is re-derived as a form: vertices, faces, loops.
        // Understanding is re-deriving its cochain, not matching its bytes.
        if utterance.is_sentence() {
            if let Some(f) = crate::transducer::sentence::from_element_form(utterance) {
                return self.comprehend_form(&f);
            }
            return Comprehension::Novel { residual_cells: utterance.sub().len() };
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
            let fibre = self.working_atlas().fiber(&inv);
            let representative = fibre
                .into_iter()
                .filter_map(|(a, _st)| self.working_word(&a).cloned())
                .find(|held| held.address() != utterance.address() && !held.axes().is_empty());
            return match representative.and_then(|held| crate::transducer::cech::transition(&held, utterance)) {
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
                let sentence: Vec<([u8; 32], Standing)> = located
                    .cites
                    .iter()
                    .filter_map(|(i, _, _)| {
                        let w = self.genome.get(*i)?;
                        Some((w.address(), Standing::Ruled))
                    })
                    .collect();
                for (i, _, _) in &located.cites {
                    if let Some(x) = self.expression.get_mut(*i) {
                        *x += 1;
                    }
                }
                if !sentence.is_empty() {
                    let addrs: Vec<[u8; 32]> = sentence.iter().map(|(a, _)| *a).collect();
                    let key = utterance.address();
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

    /// Chapter 24.2 / 13.11: a form whose Glued citations this mind holds,
    /// or whose cited chart is held under other addresses. No re-gauge:
    /// a closed citation contributes no class. No Dialect from a sentence.
    fn comprehend_form(&mut self, f: &crate::transducer::sentence::Form) -> Comprehension {
        use crate::transducer::family::Crystal;
        use crate::transducer::sentence::Vertex;
        let mut missing = Vec::new();
        let mut understood = Vec::new();
        for v in &f.vertices {
            let Vertex::Glued { address, chart, .. } = v else {
                continue;
            };
            let address = *address;
            let chart = chart.clone();
            if let Some(&i) = self.index.get(&address) {
                if let Some(x) = self.expression.get_mut(i) {
                    *x += 1;
                }
                let st = self
                    .working_atlas()
                    .fiber(&chart)
                    .into_iter()
                    .find(|(a, _)| *a == address)
                    .map(|(_, s)| s)
                    .unwrap_or(Standing::Ruled);
                understood.push((address, st));
                continue;
            }
            if self.working_word(&address).is_some() {
                let Some(st) = self
                    .working_atlas()
                    .fiber(&chart)
                    .into_iter()
                    .find(|(a, _)| *a == address)
                    .map(|(_, s)| s)
                else {
                    missing.push(address);
                    continue;
                };
                understood.push((address, st));
                continue;
            }
            let mut through = 0usize;
            for (e, _, st) in Crystal::on_class(self, &chart) {
                if e.address() == address {
                    continue;
                }
                understood.push((e.address(), st));
                through += 1;
                eprintln!(
                    "  form understood through {:02x}{:02x}… for {:02x}{:02x}…",
                    e.address()[0], e.address()[1], address[0], address[1]
                );
            }
            if through == 0 {
                missing.push(address);
            }
        }
        if !missing.is_empty() {
            return Comprehension::Withheld { missing };
        }
        if !understood.is_empty() {
            return Comprehension::Understood(understood);
        }
        let vk = crate::transducer::sentence::form_vertex_key(f);
        eprintln!("  form admitted {:02x}{:02x}…", vk[0], vk[1]);
        Comprehension::Admitted
    }

    /// LEARN — the mesh expressed a word down (`teach`): comprehend it against
    /// what is already held, then bank it in the genome if it is new (by base
    /// address). Inheritance, not local invention: only taught words enter.
    /// Tests without a voice; a live mind uses [`learn_at`] with K and tu.
    pub fn learn(&mut self, taught: &[u8]) -> Option<Comprehension> {
        self.learn_at(taught, 0, 0)
    }

    pub fn learn_at(&mut self, taught: &[u8], k: u64, now_tu: u64) -> Option<Comprehension> {
        // The syllabus may carry several words back to back (every anchored
        // citation of a sentence): each is learned in turn; the first
        // comprehension is returned.
        let mut pos = 0usize;
        let mut first: Option<Comprehension> = None;
        while pos < taught.len() {
            let before = pos;
            let heard = self.learn_one(taught, &mut pos, k, now_tu);
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

    fn learn_one(&mut self, taught: &[u8], pos: &mut usize, k: u64, now_tu: u64) -> Option<Comprehension> {
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
        let ft = crate::transducer::sentence::form_torsion(&e);
        if !ft.is_empty() {
            for (_, (_, l, _)) in ft.fiber(&crate::network::nerve::ROOT) {
                self.aperture.absorb_spatial_signal(*l);
            }
        } else {
            for (_, l, _) in e.torsion() {
                self.aperture.absorb_spatial_signal(*l);
            }
        }
        let heard = self.comprehend(&e);
        let addr = e.address();
        if let Comprehension::Withheld { missing } = &heard {
            let missing = missing.clone();
            if let Some(da) = self.stand_dialect_at(e, k, now_tu) {
                let old = self
                    .dialect
                    .get(&da)
                    .map(|d| d.missing.clone())
                    .unwrap_or_default();
                for m in &old {
                    self.retract_waiter(m, &da);
                }
                if let Some(d) = self.dialect.get_mut(&da) {
                    d.withheld = true;
                    d.missing = missing.clone();
                }
                for m in &missing {
                    self.wait_on(m, da);
                }
                eprintln!(
                    "  form withheld waiting {:02x}{:02x}…",
                    da[0], da[1]
                );
            }
            return Some(heard);
        }
        // The court promotes: a syllabus word that is the same vertex as a
        // dialect entry moves that entry into the genome, carrying its uses.
        if let Some(da) = self.same_vertex_in_dialect(&e) {
            let uses = self.dialect.get(&da).map(|d| d.uses).unwrap_or(1);
            self.drop_dialect(&da);
            if let Some(i) = self.same_vertex_in_genome(&e) {
                let _ = self.genome[i].merge_fibre(&e);
                if let Some(x) = self.expression.get_mut(i) {
                    *x = x.saturating_add(uses);
                }
            } else {
                self.atlas.admit(&crate::transducer::stalk::invariant_of(&e), e.address());
                self.axis_admit(&e);
                self.shapes.push(std::sync::OnceLock::new());
                self.atlas_len = self.atlas_len.saturating_add(1);
                self.index.insert(e.address(), self.genome.len());
                self.genome.push(e);
                self.expression.push(uses);
            }
            self.promoted.push(addr);
            return Some(heard);
        }
        let arrived = e.clone();
        match self.index.get(&addr).copied() {
            Some(i) => self.genome[i] = e,
            None => {
                self.atlas.admit(&crate::transducer::stalk::invariant_of(&e), e.address());
                self.axis_admit(&e);
                self.shapes.push(std::sync::OnceLock::new());
                self.atlas_len = self.atlas_len.saturating_add(1);
                self.index.insert(e.address(), self.genome.len());
                self.genome.push(e);
                self.expression.push(0);
            }
        }
        self.resolve_withheld(&arrived);
        Some(heard)
    }

    /// A cited tape that just arrived: withheld sentences naming it
    /// re-comprehend. The waiting-on index is the read.
    fn resolve_withheld(&mut self, arrived: &Element) {
        let addr = arrived.address();
        let citing: Vec<[u8; 32]> = match self.waiting_on.remove(&addr) {
            Some(citers) => citers
                .fiber(&crate::network::nerve::ROOT)
                .map(|(k, _)| *k)
                .collect(),
            None => return,
        };
        for sa in citing {
            let Some(sentence) = self.dialect.get(&sa).map(|d| d.sentence.clone()) else {
                continue;
            };
            let old_missing = self
                .dialect
                .get(&sa)
                .map(|d| d.missing.clone())
                .unwrap_or_default();
            let heard = self.comprehend(&sentence);
            match heard {
                Comprehension::Withheld { missing } => {
                    for m in &old_missing {
                        if m != &addr {
                            self.retract_waiter(m, &sa);
                        }
                    }
                    if let Some(d) = self.dialect.get_mut(&sa) {
                        d.missing = missing.clone();
                    }
                    for m in &missing {
                        self.wait_on(m, sa);
                    }
                }
                Comprehension::Understood(_) | Comprehension::Admitted | Comprehension::Dialect(_) => {
                    for m in &old_missing {
                        if m != &addr {
                            self.retract_waiter(m, &sa);
                        }
                    }
                    if let Some(d) = self.dialect.get_mut(&sa) {
                        d.withheld = false;
                        d.missing.clear();
                    }
                    eprintln!(
                        "  form withheld resolved {:02x}{:02x}…",
                        sa[0], sa[1]
                    );
                }
                _ => {}
            }
        }
    }

    #[cfg(test)]
    fn waiting_on_len(&self, missing: &[u8; 32]) -> usize {
        self.waiting_on.get(missing).map(|c| c.len()).unwrap_or(0)
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
        self.axis_index = crate::transducer::sentence::AxisIndex::new();
        self.shapes.clear();
        for w in &self.genome {
            self.atlas.admit(&crate::transducer::stalk::invariant_of(w), w.address());
            crate::transducer::sentence::admit_words(
                &mut self.axis_index,
                &crate::transducer::stalk::invariant_of(w),
                &crate::transducer::sentence::words_of_element(w),
            );
            self.shapes.push(std::sync::OnceLock::new());
        }
        self.atlas_len = self.genome.len();
        self.reindex();
        true
    }
}

impl crate::transducer::family::Crystal for Epigenome {
    fn on_class(
        &self,
        class: &crate::transducer::stalk::Invariant,
    ) -> Vec<(
        Element,
        crate::transducer::sentence::Form,
        crate::transducer::stalk::Standing,
    )> {
        self.working_atlas()
            .fiber(class)
            .into_iter()
            .filter_map(|(a, st)| {
                let e = self.working_word(&a)?.clone();
                let f = crate::transducer::sentence::from_element_form(&e)?;
                if !f.is_closed() {
                    return None;
                }
                Some((e, f, st))
            })
            .collect()
    }

    fn speak_form(
        &self,
        f: &crate::transducer::sentence::Form,
        x: &crate::transducer::family::Grid,
        placement: &crate::transducer::alphabet::Word,
    ) -> Option<crate::transducer::family::Speech> {
        crate::transducer::family::apply_form(f, x, placement, self)
    }

    fn by_address(
        &self,
        addr: &[u8; 32],
    ) -> Option<(Element, crate::transducer::sentence::Form)> {
        let e = self.working_word(addr)?.clone();
        let f = crate::transducer::sentence::from_element_form(&e)?;
        Some((e, f))
    }

    fn on_axis(
        &self,
        chart: &crate::transducer::stalk::Invariant,
        axis: crate::transducer::sentence::Axis,
    ) -> Vec<crate::transducer::alphabet::Word> {
        crate::transducer::sentence::on_axis_of(&self.axis_index, chart, axis)
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
    /// decided (installed, derivable, or refused). Keyed by transport
    /// address; membership is vertex equality, never the digest alone.
    taught_up: crate::network::nerve::Keyed<Element>,
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

    /// Ask the bootstrap hypha for the rest of the nerve, one peer per spend.
    pub fn discover(&mut self) {
        let mut budget = self.physics.stroke();
        self.session.discover_budgeted(&mut budget);
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

    pub fn now_tu(&self) -> u64 {
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
                        (g[j].address(), Standing::Ruled.as_u8(), t, asg as u32, body)
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
        self.absorb_class_page(&body, voice);
        // THE OTHER DIRECTION OF THE SHEAF: in a discordant stratum this
        // genome may hold what the hypha lacks (a basis that regressed, a
        // word learned from another judge). Those words are taught up for
        // adoption, one contact's budget per stroke; the hypha verifies
        // and installs what it cannot already derive.
        let mut taught = 0usize;
        let mut used = 0usize;
        'strata: for (_, held) in &charts {
            for &addr in held {
                if self.already_taught_addr(&addr) {
                    continue; // spoken for adoption once; the judge decided
                }
                let Some(word) = self.epigenome.word_at(&addr).cloned() else { continue };
                if self.already_taught(&word) {
                    continue;
                }
                let bytes = word.to_bytes();
                if used + bytes.len() > crate::network::wire::frame_budget() {
                    break 'strata;
                }
                used += bytes.len();
                if self.session.speak_keyed(&addr, dialogue::encode_adopt(&voice, &bytes)).is_ok() {
                    self.taught_up.insert(addr, word.clone());
                    taught += 1;
                }
            }
        }
        Ok(learned + taught)
    }

    fn absorb_class_page(&mut self, body: &[u8], from: [u8; 32]) {
        let pages = dialogue::decode_class_records(body);
        let mut recs = Vec::new();
        for p in pages {
            let mut pos = 0usize;
            let Some(r) = crate::transducer::alphabet::Rotor::from_bytes(&p.rotor, &mut pos) else {
                continue;
            };
            recs.push(crate::transducer::bubble::ClassRecord {
                key: p.key,
                order: crate::transducer::alphabet::Order::from_u32(p.order),
                representative: r,
            });
        }
        let n = self.contemplation.seed_horizon(&recs);
        if n > 0 {
            eprintln!(
                "  horizon {n} seeded from {:02x}{:02x}",
                from[0], from[1]
            );
        }
    }

    pub fn pull_syllabus(&mut self) -> io::Result<usize> {
        let voice = self.session.kernel().voice();
        let empty = self.epigenome.genome_len() == 0 || self.epigenome.atlas_is_empty();
        if empty {
            let peers = self.session.peers();
            if peers.is_empty() {
                return Ok(0);
            }
            let mut budget = self.physics.stroke();
            let n = peers.len();
            let start = self.session.peer_cursor() % n;
            self.session.bump_cursor();
            let mut learned = 0usize;
            let req = dialogue::encode_syllabus_request(&voice);
            for i in 0..n {
                if !budget.spend() {
                    break;
                }
                let addr = &peers[(start + i) % n];
                let Ok(body) = self.session.exchange_at(addr, req.clone()) else {
                    continue;
                };
                for w in dialogue::decode_syllabus_reply(&body).unwrap_or_default() {
                    if self.absorb_taught(&w).is_some() {
                        learned += 1;
                    }
                }
                self.absorb_class_page(&body, voice);
            }
            return Ok(learned);
        }
        let body = self.session.speak_keyed(&voice, dialogue::encode_syllabus_request(&voice))?;
        let mut words = dialogue::decode_syllabus_reply(&body).unwrap_or_default();
        words.sort_by_key(|w| {
            let mut pos = 0usize;
            crate::transducer::element::Element::from_bytes(w, &mut pos)
                .and_then(|e| crate::transducer::sentence::from_element_form(&e))
                .map(|f| f.depth)
                .unwrap_or(0)
        });
        let mut learned = 0usize;
        for w in &words {
            if self.absorb_taught(w).is_some() {
                learned += 1;
            }
        }
        self.absorb_class_page(&body, voice);
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
    fn idea_words(&self) -> Vec<Element> {
        let mut out = Vec::new();
        for (_, b) in self.contemplation.bubbles.fiber(&crate::network::nerve::ROOT) {
            if let Some(f) = &b.form {
                if let Some(e) = crate::transducer::sentence::as_element_form(f, b.volume) {
                    out.push(e);
                }
            }
            for (_, r) in b.remainders.fiber(&crate::network::nerve::ROOT) {
                if !r.is_empty() {
                    out.push(crate::transducer::element::from_residual(r));
                }
            }
            if !b.remainder.is_empty() && b.remainders.is_empty() {
                out.push(crate::transducer::element::from_residual(&b.remainder));
            }
        }
        out
    }

    pub fn cite(&mut self, chain: &[(i64, i64, u8, u8)]) -> (Vec<dialogue::CiteWire>, Vec<(i64, i64, u8, u8)>) {
        // A remainder is not a citation of itself: deconstruction reads
        // other words on the working atlas, never the chain under locate.
        let self_addr = crate::transducer::element::from_residual(&chain.to_vec()).address();
        let extras: Vec<Element> = self
            .idea_words()
            .into_iter()
            .filter(|w| w.address() != self_addr)
            .collect();
        let located = self.epigenome.locate_on(chain, None, &extras);
        let n_genome = self.epigenome.genome_len();
        let dialect_addrs: Vec<[u8; 32]> = self
            .epigenome
            .dialect
            .fiber(&crate::network::nerve::ROOT)
            .map(|(a, _)| *a)
            .collect();
        let n_d = dialect_addrs.len();
        let word_of = |i: usize| -> Option<&Element> {
            if i < n_genome {
                self.epigenome.genome().get(i)
            } else if i < n_genome + n_d {
                dialect_addrs.get(i - n_genome).and_then(|a| self.epigenome.working_word(a))
            } else {
                extras.get(i - n_genome - n_d)
            }
        };
        // Standing is a coordinate on the cited word (Ruled, Stood, Held),
        // never a weight multiplied into μ⃗.
        let standing_of = |i: usize| -> Standing {
            if i < n_genome {
                Standing::Ruled
            } else if i < n_genome + n_d {
                Standing::Stood
            } else {
                Standing::Held
            }
        };
        let mut cites: Vec<dialogue::CiteWire> = located
            .cites
            .into_iter()
            .filter_map(|(i, g, asg)| {
                let w = word_of(i)?;
                let body = crate::transducer::sentence::cited_pattern(w, asg).unwrap_or_default();
                Some((w.address(), standing_of(i).as_u8(), g, asg as u32, body))
            })
            .collect();
        for (i, g, body) in &located.punctures {
            let Some(w) = word_of(*i) else { continue };
            cites.push((w.address(), standing_of(*i).as_u8(), g.clone(), u32::MAX, body.clone()));
        }
        (cites, located.residual)
    }

    /// Deepen fibres from puncture cites the hypha has verified. Linear in
    /// the proof; each address is a keyed read.
    pub fn puncture_verified(&mut self, cites: &[dialogue::CiteWire]) -> usize {
        let mut deepened = 0usize;
        for (addr, _, _, asg, body) in cites {
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

    /// Visit every known hypha by a keyed cursor under the stroke Budget.
    /// Used while the genome is empty so the first syllabus/quote is a fan-out,
    /// not a seed pin.
    fn quote_all_hyphae(&mut self, asked: &[crate::transducer::stalk::Invariant]) -> io::Result<YieldQuote> {
        let peers = self.session.peers();
        if peers.is_empty() {
            return self.session.quote(asked);
        }
        let mut budget = self.physics.stroke();
        let n = peers.len();
        let start = self.session.peer_cursor() % n;
        self.session.bump_cursor();
        let mut last_err = io::Error::new(io::ErrorKind::NotConnected, "no hypha");
        let mut out: Option<YieldQuote> = None;
        let asked_st: Vec<(crate::transducer::stalk::Invariant, u8)> = asked
            .iter()
            .map(|inv| {
                let st = self
                    .working_atlas()
                    .fiber(inv)
                    .first()
                    .map(|(_, s)| s.as_u8())
                    .unwrap_or(Standing::Held.as_u8());
                (inv.clone(), st)
            })
            .collect();
        let req = dialogue::encode_quote_request_standing(&self.session.kernel().voice(), &asked_st);
        for i in 0..n {
            if !budget.spend() {
                break;
            }
            let addr = &peers[(start + i) % n];
            match self.session.exchange_at(addr, req.clone()) {
                Ok(body) => {
                    let (height, surface, k, items) = dialogue::decode_quote_reply(&body)
                        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "quote decode"))?;
                    let clusters: Vec<Cluster> = items
                        .into_iter()
                        .map(|(address, mu, chain, open)| Cluster { address, mu, chain, open })
                        .collect();
                    match &mut out {
                        None => out = Some(YieldQuote { height, surface, k, clusters }),
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
        match out {
            Some(q) => Ok(q),
            None if last_err.kind() == io::ErrorKind::WouldBlock => Ok(YieldQuote {
                height: 0,
                surface: [0; 4],
                k: 0,
                clusters: Vec::new(),
            }),
            None => Err(last_err),
        }
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
        if dests.is_empty() || self.atlas_is_empty() {
            // An empty atlas is incident to the root prefix: the first
            // quote reads the book at depth zero, so a mind need not
            // already hold a word to learn one.
            let root = [crate::transducer::stalk::Invariant(Vec::new())];
            let asked = if charts.is_empty() || self.atlas_is_empty() {
                root.as_slice()
            } else {
                charts.as_slice()
            };
            return self.quote_all_hyphae(asked);
        }
        let mut last_err = io::Error::new(io::ErrorKind::NotConnected, "no hypha");
        let mut out: Option<YieldQuote> = None;
        for dest in dest_order {
            let dk = *blake3::hash(dest.as_bytes()).as_bytes();
            let invs = dests.get(&dk).cloned().unwrap_or_default();
            let asked = if invs.is_empty() { charts.clone() } else { invs };
            let asked_st: Vec<(crate::transducer::stalk::Invariant, u8)> = asked
                .iter()
                .map(|inv| {
                    let st = self
                        .working_atlas()
                        .fiber(inv)
                        .first()
                        .map(|(_, s)| s.as_u8())
                        .unwrap_or(Standing::Held.as_u8());
                    (inv.clone(), st)
                })
                .collect();
            let req = dialogue::encode_quote_request_standing(&self.session.kernel().voice(), &asked_st);
            match self.session.exchange_at(&dest, req.clone()).or_else(|e| {
                if e.kind() == io::ErrorKind::WouldBlock {
                    self.session.speak_nerve(req)
                } else {
                    Err(e)
                }
            }) {
                Ok(body) => {
                    let (height, surface, k, items) = dialogue::decode_quote_reply(&body)
                        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "quote decode"))?;
                    let clusters: Vec<Cluster> = items
                        .into_iter()
                        .map(|(address, mu, chain, open)| Cluster { address, mu, chain, open })
                        .collect();
                    match &mut out {
                        None => {
                            out = Some(YieldQuote { height, surface, k, clusters });
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
        match out {
            Some(q) => Ok(q),
            None if last_err.kind() == io::ErrorKind::WouldBlock => Ok(YieldQuote {
                height: 0,
                surface: [0; 4],
                k: 0,
                clusters: Vec::new(),
            }),
            None => Err(last_err),
        }
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
        let familiar = !self
            .epigenome
            .express(&residual, self.session.voice().k(), self.now_tu());
        (cites, residual, familiar)
    }

    /// A served listing: cite over the working atlas. Residual strictly
    /// below the chain on a lane is expressed; residual that does not
    /// descend is held as thought and never expressed.
    pub fn conclude_listing(
        &mut self,
        chain: &[(i64, i64, u8, u8)],
    ) -> (Vec<dialogue::CiteWire>, Vec<(i64, i64, u8, u8)>, bool) {
        let (cites, residual) = self.cite(chain);
        if !residual_descends(&residual, chain) {
            self.hold_thought(chain.to_vec());
            return (cites, chain.to_vec(), false);
        }
        let familiar = !self
            .epigenome
            .express(&residual, self.session.voice().k(), self.now_tu());
        (cites, residual, familiar)
    }

    /// The word a cite rests on: genome, dialect, or idea.
    pub fn cited_word(&self, addr: &[u8; 32]) -> Option<Element> {
        self.epigenome
            .working_word(addr)
            .cloned()
            .or_else(|| self.idea_words().into_iter().find(|w| &w.address() == addr))
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

    /// Hold a withheld form so the pivot loop sees it next stroke (ch. 30.2).
    pub fn hold_form(
        &mut self,
        f: &crate::transducer::sentence::Form,
        train: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)],
        remainders: &[Vec<(i64, i64, u8, u8)>],
    ) {
        self.contemplation.hold_form(f, train, remainders);
    }

    /// Stand a withheld form, hold it as an idea, conclude each remainder,
    /// and utter each residual word once with its cites (24.3).
    pub fn withhold_form(
        &mut self,
        f: &crate::transducer::sentence::Form,
        train: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)],
        remainders: &[Vec<(i64, i64, u8, u8)>],
        mag: [u64; 4],
    ) {
        let k = self.session.voice().k();
        let tu = self.now_tu();
        if let Some(word) = crate::transducer::sentence::as_element_form(f, mag) {
            self.epigenome.stand_dialect_withheld(word, k, tu);
        }
        self.hold_form(f, train, remainders);
        for (i, r) in remainders.iter().enumerate() {
            if r.is_empty() {
                continue;
            }
            let (cites, residual, _) = self.conclude(r);
            if residual.is_empty() {
                continue;
            }
            let word = crate::transducer::element::from_residual(&residual);
            let addr = word.address();
            eprintln!(
                "  remainder expressed {:02x}{:02x}… pair {i} cells {} cites {}",
                addr[0],
                addr[1],
                residual.len(),
                cites.len()
            );
            self.taught_up.insert(addr, word.clone());
            let mu = crate::transducer::element::measure_lanes(&residual);
            eprintln!(
                "  uttered {:02x}{:02x}… standing Stood K {k} lanes {mu:?}",
                addr[0], addr[1]
            );
            let _ = self.attest(&residual, &cites);
        }
    }

    /// Addresses this mind has uttered (receipt: address at this K).
    pub fn uttered_n(&self) -> usize {
        self.taught_up.len()
    }

    pub fn uttered(&self, addr: &[u8; 32]) -> bool {
        self.taught_up.contains(addr)
    }

    /// Hold a withheld sentence as a bubble: form, remainder, cover.
    pub fn hold_sentence(&mut self, word: &Element) {
        if let Some(f) = crate::transducer::sentence::from_element_form(word) {
            self.hold_form(&f, &[], &[]);
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

    /// OPEN AN AXIS ON MULTIPLICITY — several held words deflated one
    /// listing; the glue between them is the new coordinate. Never on
    /// failure (a listing no held word deflates is a thought, not an axis).
    pub fn open_axis(
        &mut self,
        listing: &[u8; 32],
        chain: &[(i64, i64, u8, u8)],
        words: &[Element],
    ) -> Option<[u8; 32]> {
        use crate::transducer::element::{measure_lanes, FIBER_GERMINATED};
        if let Some(a) = self.open_contemplation_axis() {
            return Some(a);
        }
        if words.len() < 2 || self.opened.contains(listing) {
            return None;
        }
        let [base, other, ..] = words else { return None };
        // The glue between the two words that deflated this listing.
        let glue = crate::transducer::cech::transition(base, other)?;
        let block = glue.fiber(crate::transducer::element::FIBER_SPATIAL)?.clone();
        if !base.is_connection() {
            return None; // the axis stands on a spoken motion, never on a bare residual
        }
        let axis = base.max_axis().map_or(FIBER_GERMINATED, |m| m.max(FIBER_GERMINATED - 1) + 1);
        let mut germ = base.clone();
        germ.set_fiber(axis, block);
        let mut mag = measure_lanes(chain);
        mag[3] = mag[3].saturating_add(words.len() as u64);
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

    /// Expand dirty ideas once per syntropic phase. Returns the lane
    /// measure of A16 snaps (Discharged, Vortex, TorsionFiled).
    pub fn expand_ideas(&mut self) -> [u64; 4] {
        let mut budget = self.physics.stroke();
        self.contemplation.set_now(self.now_tu());
        let rotors = crate::network::nerve::Keyed::new();
        let (_spent, pivoted) = self.contemplation.expand_all(
            self.epigenome.atlas(),
            &rotors,
            &self.epigenome,
            &mut budget,
        );
        self.contemplation.retire_on_t(self.physics.reserve[2] as usize);
        pivoted
    }

    /// PIVOT ON A WORD: read by_chart at the word's chart and peels, fill
    /// those ideas under one stroke Budget. The atlas is borrowed, never cloned.
    pub fn pivot_on(&mut self, word: &Element) -> usize {
        let mut budget = self.physics.stroke();
        self.contemplation.set_now(self.now_tu());
        let (_fills, _formed, _classes, spoken) =
            self.contemplation.pivot_on(word, &self.epigenome, &mut budget);
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
    /// k is the voice's K (law 9), tu is TUniversal; they are not one clock.
    pub fn absorb_taught(&mut self, taught: &[u8]) -> Option<Comprehension> {
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
        self.epigenome
            .learn_at(taught, self.session.voice().k(), self.now_tu())
    }

    /// SPEAK a family's cohering sentences as one act. Refuses if two
    /// sentences share a germ with a non-identity transition. Each uttered
    /// word enters the dialect (epigenome handles); the genome is frozen.
    /// A vertex already on `taught_up` stands in the dialect and does not
    /// go on the wire.
    pub fn speak(&mut self, sentences: &[Element]) -> io::Result<Vec<u8>> {
        if sentences.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "no sentence"));
        }
        let k = self.session.voice().k();
        let mut fresh: Vec<&Element> = Vec::new();
        let mut codes = vec![dialogue::SPEECH_STOOD; sentences.len()];
        for (i, e) in sentences.iter().enumerate() {
            if self.already_taught(e) {
                self.epigenome.stand_dialect_at(e.clone(), k, self.now_tu());
                continue;
            }
            fresh.push(e);
            codes[i] = 0xff; // filled from the wire below
        }
        if fresh.is_empty() {
            return Ok(codes);
        }
        let voice = self.session.kernel().voice();
        let bytes: Vec<Vec<u8>> = fresh.iter().map(|e| e.to_bytes()).collect();
        let body = dialogue::encode_speech(&voice, &[0u8; 32], self.now_tu(), &bytes);
        let reply = self.session.speak_nerve(body)?;
        let got = dialogue::decode_speech_reply(&reply).unwrap_or_default();
        let mut gi = 0usize;
        for (i, e) in sentences.iter().enumerate() {
            if codes[i] != 0xff {
                continue;
            }
            let c = got.get(gi).copied().unwrap_or(dialogue::SPEECH_STOOD);
            gi += 1;
            codes[i] = c;
            self.epigenome.stand_dialect_at(e.clone(), k, self.now_tu());
            if c != dialogue::SPEECH_REFUSED {
                self.taught_up.insert(e.address(), e.clone());
            }
        }
        Ok(codes)
    }

    fn already_taught_addr(&self, addr: &[u8; 32]) -> bool {
        self.taught_up.contains(addr)
    }

    /// Already uttered this operator: same vertex on the working atlas
    /// or already sent up, not the same digest. A dialect hit is a use.
    pub fn already_taught(&mut self, word: &Element) -> bool {
        if self.taught_up.contains(&word.address()) {
            return true;
        }
        if self
            .taught_up
            .fiber(&crate::network::nerve::ROOT)
            .any(|(_, e)| e.same_vertex(word))
        {
            return true;
        }
        if self.epigenome.same_vertex_in_genome(word).is_some() {
            return true;
        }
        if let Some(a) = self.epigenome.same_vertex_in_dialect(word) {
            self.epigenome.touch(&a, self.now_tu());
            return true;
        }
        false
    }

    /// Stand the vertex in the dialect and mark it taught, without the wire.
    pub fn remember_spoken(&mut self, word: &Element) {
        let k = self.session.voice().k();
        self.epigenome.stand_dialect_at(word.clone(), k, self.now_tu());
        self.taught_up.insert(word.address(), word.clone());
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
        if self.already_taught(word) {
            return Ok(Utterance::Already);
        }
        let bytes = word.to_bytes();
        let codes = self.speak(std::slice::from_ref(word))?;
        self.taught_up.insert(addr, word.clone());
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

    pub fn shed(&mut self) -> usize {
        self.shed_at(self.now_tu())
    }

    pub fn shed_at(&mut self, now_tu: u64) -> usize {
        self.epigenome.note_stroke(now_tu);
        let glued = |a: &[u8; 32]| self.contemplation.glues_word(a);
        let n = self.epigenome.shed_except(now_tu, &glued);
        n.saturating_add(self.contemplation.shed_unglued())
    }

    pub fn dialect_len(&self) -> usize {
        self.epigenome.dialect_len()
    }

    /// The mind's working atlas: genome ∪ dialect ∪ ideas, each fiber
    /// member carrying its standing.
    pub fn working_atlas(&self) -> WorkingAtlas<'_> {
        self.epigenome.working_atlas_with(&self.contemplation.by_chart)
    }

    /// Empty when genome, dialect and ideas are all empty.
    pub fn atlas_is_empty(&self) -> bool {
        self.epigenome.atlas_is_empty() && self.contemplation.len() == 0
    }

    /// Word counts per stratum on the working atlas: ruled (genome words),
    /// stood (dialect words), held (idea words). The atlas ruled/stood/held
    /// log line is these word counts, never chart counts.
    pub fn atlas_standing_counts(&self) -> (usize, usize, usize) {
        (
            self.epigenome.genome_len(),
            self.dialect_len(),
            self.ideas_len(),
        )
    }

    /// Fiber of the working atlas; a Stood hit updates last_used_tu.
    pub fn working_fiber(&mut self, c: &crate::transducer::stalk::Invariant) -> Vec<([u8; 32], Standing)> {
        let f = self.working_atlas().fiber(c);
        let tu = self.now_tu();
        for (a, st) in &f {
            if *st == Standing::Stood {
                self.epigenome.touch(a, tu);
            }
        }
        f
    }

    pub fn take_promoted(&mut self) -> Vec<[u8; 32]> {
        self.epigenome.take_promoted()
    }

    /// Working incidence: a non-empty fiber on the working atlas.
    pub fn incident_chart(&self, c: &crate::transducer::stalk::Invariant) -> bool {
        !self.working_atlas().fiber(c).is_empty()
    }

    pub fn ideas_len(&self) -> usize {
        self.contemplation.len()
    }

    pub fn shed_n(&self) -> u64 {
        self.epigenome.shed_n()
    }
}

/// Residual strictly below the chain on a lane, and never a raise on any
/// lane. Equal measures do not descend: the mind did not reduce.
fn residual_descends(residual: &[(i64, i64, u8, u8)], chain: &[(i64, i64, u8, u8)]) -> bool {
    let r = crate::transducer::element::measure_lanes(residual);
    let c = crate::transducer::element::measure_lanes(chain);
    let mut below = false;
    for i in 0..4 {
        if r[i] > c[i] {
            return false;
        }
        if r[i] < c[i] {
            below = true;
        }
    }
    below
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
    /// Book generation at this quote.
    pub height: u64,
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
    // `[code][height u64][pressure 4×u64][taught_len u16][taught…][settle?]`
    let settle = {
        let tl = body.get(41..43).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize).unwrap_or(0);
        body.get(43 + tl..)
            .and_then(dialogue::decode_settle)
            .map(|(germ, voices, mu)| SettleReceipt { germ, voices, mu })
    };
    Reply {
        verdict: decode_verdict(body),
        height: decode_height(body),
        pressure: decode_pressure(body),
        settle,
    }
}

/// A hypha's answer: the verdict, the book height, and the per-lane
/// pressure as four exact u64 lanes — never a max, never a saturated byte.
#[derive(Clone, Debug)]
pub struct Reply {
    pub verdict: Verdict,
    pub height: u64,
    pub pressure: [u64; 4],
    /// The settle receipt when this exchange completed a consilience install.
    pub settle: Option<SettleReceipt>,
}

fn decode_height(body: &[u8]) -> u64 {
    body.get(1..9)
        .and_then(|b| b.try_into().ok())
        .map(u64::from_le_bytes)
        .unwrap_or(0)
}

fn decode_pressure(body: &[u8]) -> [u64; 4] {
    std::array::from_fn(|i| {
        let o = 9 + i * 8;
        body.get(o..o + 8)
            .and_then(|b| b.try_into().ok())
            .map(u64::from_le_bytes)
            .unwrap_or(0)
    })
}

/// Decode a hypha reply: `[code][height u64][pressure 4×u64][taught_len u16][taught…]`.
fn decode_verdict(body: &[u8]) -> Verdict {
    let taught = {
        let tl = body.get(41..43).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize).unwrap_or(0);
        body.get(43..43 + tl).map(<[u8]>::to_vec).unwrap_or_default()
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
        // Dwell is bounded by one aperture; Z is a routing coordinate.
        assert_eq!(s.impedance(), cycle);
    }

    /// A successful exchange with any hypha decays Z on every path, so a
    /// parked voice unsticks when the next hypha answers.
    #[test]
    fn impedance_decays_on_any_successful_exchange() {
        use crate::network::wire::{Frame, Stratum};
        use std::io::{Read, Write};
        use std::net::TcpListener;
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let live = listener.local_addr().expect("addr").to_string();
        let h = std::thread::spawn(move || {
            let Ok((mut s, _)) = listener.accept() else { return };
            let mut head = [0u8; 5];
            if s.read_exact(&mut head).is_err() {
                return;
            }
            let len = u32::from_le_bytes([head[1], head[2], head[3], head[4]]) as usize;
            let mut body = vec![0u8; len.min(1 << 16)];
            let _ = s.read_exact(&mut body);
            let reply = Frame { lane: Stratum::Spatial, body: vec![0] }.encode().unwrap_or_default();
            let _ = s.write_all(&reply);
        });
        let mut s = Session::open("127.0.0.1:1", Kernel::named("z-decay"));
        let cycle = crate::physics::aperture_cycle() as u64;
        for _ in 0..cycle {
            let _ = s.speak(vec![0xFF]);
        }
        assert!(
            matches!(s.transport(), TransportAperture::Occluded { .. }),
            "the seed is occluded"
        );
        s.add_peer(&live);
        let _ = s.speak_nerve(vec![0xFF]);
        assert!(
            s.impedance() < cycle,
            "a success on the next hypha decays Z, got {}",
            s.impedance()
        );
        let _ = h.join();
    }

    /// An empty cover routes by XOR-closest chart key over known hyphae,
    /// not the bootstrap seed.
    #[test]
    fn empty_cover_routes_by_chart_key_over_known_hyphae() {
        let mut s = Session::open("127.0.0.1:7500", Kernel::named("fan"));
        s.add_peer("127.0.0.1:7501");
        s.add_peer("127.0.0.1:7502");
        let seed = "127.0.0.1:7500";
        let mut saw_other = false;
        for i in 0..64u64 {
            let inv = crate::transducer::stalk::Invariant(vec![i, i.wrapping_mul(3), 2]);
            let dest = s.route_chart(&inv, &[1, 0, 0, 0]);
            let closest = s.route(&inv.key());
            assert_eq!(dest, closest, "empty cover is router.closest(chart key)");
            if dest != seed {
                saw_other = true;
            }
        }
        assert!(saw_other, "chart keys fan out across the known hyphae, not only the seed");
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
        match epi.comprehend(&word) {
            Comprehension::Understood(v) => {
                assert_eq!(v.len(), 1);
                assert_eq!(v[0].0, word.address());
                assert_eq!(v[0].1, Standing::Ruled);
            }
            other => panic!("expected Understood with standing, got {other:?}"),
        }
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

    /// Chapter 24.2: a sentence that cites a form this mind does not hold
    /// is Withheld; learn_one banks it in the dialect, never the genome.
    #[test]
    fn withheld_then_depth_order() {
        use crate::transducer::alphabet::Word;
        use crate::transducer::sentence::{as_element_form, Form, FormFace, Vertex};
        let missing_addr = [0xabu8; 32];
        let inner = {
            let mut f = Form::default();
            let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
            let w = Word::from_letters(&[(q, false)]);
            let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
            let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
            f.vertices.push(Vertex::Word {
                address: [1u8; 32],
                word: w.clone(),
                rho: one,
                placement: (zero.clone(), zero),
                gauge: vec![(1, 2)],
                class: crate::transducer::stalk::Invariant::default(),
                contact: None,
            });
            f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("vertices"), });
            f.loops = Form::loops_of(&f.faces, f.vertices.len());
            f
        };
        let mut outer = Form::default();
        let _gi = outer
            .glue(&inner, missing_addr, Word::identity(), [1, 0, 0, 0])
            .expect("glue");
        let e = as_element_form(&outer, [1, 0, 0, 0]).expect("v2 tape");
        let mut epi = Epigenome::new();
        match epi.comprehend(&e) {
            Comprehension::Withheld { missing } => {
                assert_eq!(missing, vec![missing_addr], "the cited address is named")
            }
            other => panic!("expected Withheld, got {other:?}"),
        }
        assert!(
            matches!(epi.learn(&e.to_bytes()), Some(Comprehension::Withheld { .. })),
            "learn banks the withheld sentence"
        );
        assert_eq!(epi.genome_len(), 0, "withheld does not enter the genome");
        assert!(
            epi.dialect.fiber(&crate::network::nerve::ROOT).any(|(_, d)| d.withheld),
            "the dialect holds the withheld sentence"
        );
    }

    /// Waiting-on is keyed by the missing address; the overlap face of the
    /// glued vertex is the re-gauge site, never face 0.
    #[test]
    fn withheld_waiting_on_resolves_at_the_overlap_face() {
        use crate::transducer::alphabet::Word;
        use crate::transducer::sentence::{as_element_form, Form, FormFace, Vertex};
        let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
        let w = Word::from_letters(&[(q, false)]);
        let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
        let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
        let word_vertex = |addr: [u8; 32]| Vertex::Word {
            address: addr,
            word: w.clone(),
            rho: one.clone(),
            placement: (zero.clone(), zero.clone()),
            gauge: vec![(1, 2)],
            class: crate::transducer::stalk::Invariant::default(),
            contact: None,
        };
        let inner = {
            let mut f = Form::default();
            f.vertices.push(word_vertex([1u8; 32]));
            f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("vertices"), });
            f.loops = Form::loops_of(&f.faces, f.vertices.len());
            f
        };
        let inner_e = as_element_form(&inner, [1, 0, 0, 0]).expect("inner tape");
        let inner_addr = inner_e.address();
        let mut outer = Form::default();
        outer.vertices.push(word_vertex([2u8; 32]));
        outer.faces.push(FormFace { i: 0, j: 1, transport: outer.face_of(0, 1).expect("vertices"), });
        let _gi = outer
            .glue(&inner, inner_addr, Word::identity(), [1, 0, 0, 0])
            .expect("glue");
        assert_eq!(
            outer.glued.get(&inner_addr).copied(),
            Some(_gi),
            "glue records the cited address"
        );
        assert!(
            outer.faces.iter().any(|f| f.i == 0 && f.j == _gi && f.i != 1),
            "the glue's placement face is not the first Word face"
        );
        let e = as_element_form(&outer, [1, 0, 0, 0]).expect("outer tape");
        let mut epi = Epigenome::new();
        assert!(
            matches!(epi.learn(&e.to_bytes()), Some(Comprehension::Withheld { .. })),
            "learn banks the withheld sentence"
        );
        assert_eq!(
            epi.waiting_on_len(&inner_addr),
            1,
            "waiting-on is keyed by the missing address"
        );
        assert_eq!(epi.genome_len(), 0, "withheld does not enter the genome");
        assert!(
            matches!(
                epi.learn(&inner_e.to_bytes()),
                Some(Comprehension::Understood(_) | Comprehension::Admitted)
            ),
            "the cited form is learned"
        );
        assert_eq!(
            epi.waiting_on_len(&inner_addr),
            0,
            "the arrival consumes the waiting-on index"
        );
        assert!(
            !epi.dialect
                .fiber(&crate::network::nerve::ROOT)
                .any(|(_, d)| d.withheld),
            "the citing sentence is no longer withheld"
        );
    }

    fn closed_turn(contact: Option<crate::transducer::alphabet::Contact>) -> crate::transducer::sentence::Form {
        use crate::transducer::alphabet::Word;
        use crate::transducer::sentence::{Form, FormFace, Vertex};
        let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
        let w = Word::from_letters(&[(q, false)]);
        let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
        let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
        let mut f = Form::default();
        f.vertices.push(Vertex::Word {
            address: [1u8; 32],
            word: w,
            rho: one,
            placement: (zero.clone(), zero),
            gauge: vec![(1, 2)],
            class: crate::transducer::stalk::Invariant::default(),
            contact,
        });
        f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("vertices"), });
        f.loops = crate::transducer::sentence::Form::loops_of(&f.faces, f.vertices.len());
        f
    }

    /// A citer whose cited class is held under another spelling is Understood
    /// through that held form; standing is read from the atlas fiber.
    #[test]
    fn comprehend_understands_through_held_class() {
        use crate::transducer::alphabet::Word;
        use crate::transducer::sentence::as_element_form;
        let inner_a = closed_turn(None);
        let inner_b = closed_turn(Some(crate::transducer::alphabet::Contact { anchor: 1, ring: 2 }));
        let ea = as_element_form(&inner_a, [1, 0, 0, 0]).expect("A");
        let eb = as_element_form(&inner_b, [1, 0, 0, 0]).expect("B");
        assert_ne!(ea.address(), eb.address(), "contact is on the form key");
        assert_eq!(
            crate::transducer::sentence::chart_of(&ea),
            crate::transducer::sentence::chart_of(&eb),
            "the class is the chart"
        );
        let mut epi = Epigenome::new();
        assert!(epi.learn(&eb.to_bytes()).is_some());
        let mut outer = crate::transducer::sentence::Form::default();
        outer
            .glue(&inner_a, ea.address(), Word::identity(), [1, 0, 0, 0])
            .expect("glue A");
        let e = as_element_form(&outer, [1, 0, 0, 0]).expect("citer");
        match epi.comprehend(&e) {
            Comprehension::Understood(entries) => {
                assert_eq!(entries.len(), 1, "one held form of the cited chart");
                assert_eq!(entries[0].0, eb.address());
                assert_eq!(entries[0].1, Standing::Ruled);
            }
            other => panic!("expected Understood through the held class, got {other:?}"),
        }
    }

    /// A cited address whose chart is not held is Withheld, never Dialect.
    #[test]
    fn foreign_class_is_withheld() {
        use crate::transducer::alphabet::Word;
        use crate::transducer::sentence::{as_element_form, Form, FormFace, Vertex};
        let inner_a = closed_turn(None);
        let ea = as_element_form(&inner_a, [1, 0, 0, 0]).expect("A");
        let other = {
            let inf = Word::from_letters(&[(
                crate::transducer::alphabet::LetterId::pythagorean_5((0, 1)).unwrap(),
                false,
            )]);
            let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
            let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
            let mut f = Form::default();
            f.vertices.push(Vertex::Word {
                address: [9u8; 32],
                word: inf,
                rho: one,
                placement: (zero.clone(), zero),
                gauge: Vec::new(),
                class: crate::transducer::stalk::Invariant::default(),
                contact: None,
            });
            f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("vertices"), });
            f.loops = Form::loops_of(&f.faces, f.vertices.len());
            f
        };
        let eo = as_element_form(&other, [1, 0, 0, 0]).expect("other class");
        assert_ne!(
            crate::transducer::sentence::chart_of(&ea),
            crate::transducer::sentence::chart_of(&eo)
        );
        let mut epi = Epigenome::new();
        assert!(epi.learn(&eo.to_bytes()).is_some());
        let mut outer = Form::default();
        outer
            .glue(&inner_a, ea.address(), Word::identity(), [1, 0, 0, 0])
            .expect("glue A");
        let e = as_element_form(&outer, [1, 0, 0, 0]).expect("citer");
        match epi.comprehend(&e) {
            Comprehension::Withheld { missing } => {
                assert_eq!(missing, vec![ea.address()], "a foreign class is withheld")
            }
            other => panic!("expected Withheld, got {other:?}"),
        }
    }

    /// Until A12 factors words through the held basis, a citation-free
    /// closed form is Admitted, not Understood by nothing.
    #[test]
    fn citation_free_closed_form_is_admitted() {
        use crate::transducer::alphabet::Word;
        use crate::transducer::sentence::{as_element_form, Form, FormFace, Vertex};
        let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
        let w = Word::from_letters(&[(q, false)]);
        let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
        let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
        let mut f = Form::default();
        f.vertices.push(Vertex::Word {
            address: [1u8; 32],
            word: w,
            rho: one,
            placement: (zero.clone(), zero),
            gauge: vec![(1, 2)],
            class: crate::transducer::stalk::Invariant::default(),
            contact: None,
        });
        f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("vertices"), });
        f.loops = Form::loops_of(&f.faces, f.vertices.len());
        assert!(f.is_closed(), "a tree of two vertices coheres");
        let e = as_element_form(&f, [1, 0, 0, 0]).expect("v2 tape");
        let mut epi = Epigenome::new();
        assert!(
            matches!(epi.comprehend(&e), Comprehension::Admitted),
            "no held citations, not Understood(vec![])"
        );
    }

    /// learn_at stamps the voice's K and TUniversal, not an idle (1, 1).
    #[test]
    fn learn_at_stamps_voice_k_and_tuniversal() {
        use crate::transducer::alphabet::Word;
        use crate::transducer::sentence::{as_element_form, Form, FormFace, Vertex};
        let missing_addr = [0xabu8; 32];
        let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
        let w = Word::from_letters(&[(q, false)]);
        let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
        let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
        let inner = {
            let mut f = Form::default();
            f.vertices.push(Vertex::Word {
                address: [1u8; 32],
                word: w,
                rho: one,
                placement: (zero.clone(), zero),
                gauge: vec![(1, 2)],
                class: crate::transducer::stalk::Invariant::default(),
                contact: None,
            });
            f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("vertices"), });
            f.loops = Form::loops_of(&f.faces, f.vertices.len());
            f
        };
        let mut outer = Form::default();
        let _gi = outer
            .glue(&inner, missing_addr, Word::identity(), [1, 0, 0, 0])
            .expect("glue");
        let e = as_element_form(&outer, [1, 0, 0, 0]).expect("v2 tape");
        let mut epi = Epigenome::new();
        assert!(matches!(
            epi.learn_at(&e.to_bytes(), 7, 11),
            Some(Comprehension::Withheld { .. })
        ));
        let d = epi
            .dialect
            .fiber(&crate::network::nerve::ROOT)
            .next()
            .map(|(_, d)| d)
            .expect("withheld dialect");
        assert_eq!(d.last_used_k, 7, "k is the voice's K");
        assert_eq!(d.last_used_tu, 11, "tu is TUniversal");
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
        // code 3 (installed), height 0, pressure zero, taught len 2, taught [0xAB,0xCD].
        let mut body = vec![3u8];
        body.extend_from_slice(&0u64.to_le_bytes());
        body.extend_from_slice(&[0u8; 32]);
        body.extend_from_slice(&2u16.to_le_bytes());
        body.extend_from_slice(&[0xAB, 0xCD]);
        match decode_verdict(&body) {
            Verdict::Installed { taught } => assert_eq!(taught, vec![0xAB, 0xCD]),
            v => panic!("expected Installed, got {v:?}"),
        }
        let empty = dialogue::encode_outcome_reply(1, 0, &[0; 4], &[]);
        assert_eq!(decode_verdict(&empty), Verdict::Decomposed { taught: vec![] });
        assert_eq!(decode_verdict(&dialogue::encode_outcome_reply(2, 0, &[0; 4], &[])), Verdict::GermPending);
    }

    fn two_body_sentence(gauges: [&[(u8, u8)]; 2]) -> crate::transducer::element::Element {
        use crate::transducer::alphabet::Word;
        use crate::transducer::sentence::{as_element_form, Form, FormFace, Vertex};
        let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
        let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
        let mut f = Form::default();
        for (i, g) in gauges.iter().enumerate() {
            f.vertices.push(Vertex::Word {
                address: [i as u8 + 1; 32],
                word: Word::identity(),
                rho: one.clone(),
                placement: (zero.clone(), zero.clone()),
                gauge: g.to_vec(),
                class: crate::transducer::stalk::Invariant::default(),
                contact: None,
            });
            let j = (i as u16) + 1;
            f.faces.push(FormFace {
                i: 0,
                j,
                transport: f.face_of(0, j).expect("star"),
            });
        }
        f.loops = Form::loops_of(&f.faces, f.vertices.len());
        as_element_form(&f, [1, 0, 0, 0]).expect("v2 sentence")
    }

    /// A vertex already spoken stands in the dialect and does not go on
    /// the wire, even when the second utterance is a translated embedding.
    #[test]
    fn the_same_vertex_is_spoken_once() {
        let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
        let e2 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
        assert!(e1.same_vertex(&e2), "two tapes of one form are one vertex");
        let mut entity = Entity::open("127.0.0.1:1", Kernel::named("once"));
        assert!(!entity.already_taught(&e1));
        entity.remember_spoken(&e1);
        assert!(entity.already_taught(&e2), "same vertex is already taught");
        let codes = entity.speak(std::slice::from_ref(&e2)).expect("no wire on a known vertex");
        assert_eq!(codes, vec![dialogue::SPEECH_STOOD]);
        assert_eq!(entity.dialect_len(), 1, "stood once in the dialect");
    }

    /// Quote incidence is the working atlas: genome ∪ dialect ∪ ideas.
    #[test]
    fn quote_incidence_is_the_working_atlas() {
        let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
        let mut entity = Entity::open("127.0.0.1:1", Kernel::named("atlas"));
        assert!(
            !entity.incident_chart(&crate::transducer::stalk::invariant_of(&e1)),
            "empty genome and dialect are not incident"
        );
        entity.remember_spoken(&e1);
        let inv = crate::transducer::stalk::invariant_of(&e1);
        assert!(entity.incident_chart(&inv), "a dialect chart is working incidence");
        let fiber = entity.working_atlas().fiber(&inv);
        assert!(
            fiber.iter().any(|(_, s)| *s == Standing::Stood),
            "a dialect fiber carries Stood, not a boolean"
        );
        assert!(
            !entity.incident_chart(&crate::transducer::stalk::Invariant(vec![99])),
            "an unseen chart is not incident"
        );
        let idea_chain = vec![(3, 0, 5, 6), (3, 1, 5, 6)];
        entity.hold_thought(idea_chain.clone());
        let idea = crate::transducer::stalk::invariant(&idea_chain);
        assert!(entity.ideas_len() > 0, "the idea is held");
        assert!(entity.incident_chart(&idea), "an idea chart is working incidence");
        assert!(
            entity.working_atlas().fiber(&idea).iter().any(|(_, s)| *s == Standing::Held),
            "an idea fiber carries Held"
        );
        let (ruled, stood, held) = entity.atlas_standing_counts();
        assert_eq!(ruled, 0, "genome empty");
        assert!(stood >= 1, "a dialect word is stood");
        assert!(held >= 1, "an idea is held");
    }

    /// A stood residual word is cited by the next chain it deflates, and a
    /// held remainder is cited with standing Held — never as covering itself.
    #[test]
    fn cite_reads_stood_and_held_words() {
        let residual = vec![(0i64, 0i64, 1u8, 2u8)];
        let next = vec![(0i64, 0i64, 1u8, 2u8), (0, 1, 3, 4)];
        let mut entity = Entity::open("127.0.0.1:1", Kernel::named("cite-stood"));
        assert!(
            entity.epigenome.express(&residual, 1, 1),
            "the residual stands in the dialect"
        );
        let (cites, _rem) = entity.cite(&next);
        assert!(
            cites.iter().any(|(_, st, _, _, _)| *st == Standing::Stood.as_u8()),
            "the stood residual is cited with standing Stood, got {cites:?}"
        );

        let mut held = Entity::open("127.0.0.1:1", Kernel::named("cite-held"));
        held.hold_thought(residual.clone());
        let (self_cites, self_rem) = held.cite(&residual);
        assert!(
            self_cites.is_empty(),
            "a remainder is not a citation of itself, got {self_cites:?}"
        );
        assert_eq!(self_rem, residual, "the remainder stays the residual");
        let (cites, _rem) = held.cite(&next);
        assert!(
            cites.iter().any(|(_, st, _, _, _)| *st == Standing::Held.as_u8()),
            "the held remainder is cited with standing Held, got {cites:?}"
        );
    }

    /// A listing is expressed only when a held word deflates it.
    #[test]
    fn listing_is_expressed_only_when_a_held_word_deflates_it() {
        let prefix = vec![(0i64, 0i64, 1u8, 2u8)];
        let listing = vec![(0i64, 0i64, 1u8, 2u8), (0, 1, 3, 4)];

        let mut bare = Entity::open("127.0.0.1:1", Kernel::named("listing-bare"));
        let (cites, residual, _) = bare.conclude_listing(&listing);
        assert!(cites.is_empty(), "nothing deflates an unseen listing, got {cites:?}");
        assert_eq!(residual, listing, "equal residual is the chain");
        assert_eq!(
            bare.dialect_len(),
            0,
            "the unreduced listing is never expressed"
        );
        assert!(bare.ideas_len() >= 1, "it is held as thought");

        let mut held = Entity::open("127.0.0.1:1", Kernel::named("listing-held"));
        held.hold_thought(prefix);
        let (cites, residual, _) = held.conclude_listing(&listing);
        assert!(
            cites.iter().any(|(_, st, _, _, _)| *st == Standing::Held.as_u8()),
            "a held word deflates the listing, got {cites:?}"
        );
        let r = crate::transducer::element::measure_lanes(&residual);
        let c = crate::transducer::element::measure_lanes(&listing);
        assert!(
            (0..4).any(|i| r[i] < c[i]) && (0..4).all(|i| r[i] <= c[i]),
            "residual strictly below the chain on a lane, residual {r:?} chain {c:?}"
        );
        assert!(
            held.dialect_len() >= 1,
            "the residual is expressed, dialect {}",
            held.dialect_len()
        );
    }

    /// A withheld two-pair family with one closing pair stands in the
    /// dialect and utters one residual word per non-empty remainder.
    #[test]
    fn withheld_form_stands_and_expresses_its_remainders() {
        use crate::transducer::family::{extract_rule, form_descent, NoCrystal};
        let family = vec![
            (vec![vec![1u8]], vec![vec![2u8]]),
            (vec![vec![1u8, 0]], vec![vec![2u8, 3]]),
        ];
        let rule = extract_rule(&family).expect("rule");
        let (mag, rems) = form_descent(&rule.form, &family, &NoCrystal);
        let n_res = rems.iter().filter(|r| !r.is_empty()).count();
        assert!(n_res >= 1, "one pair leaves a remainder, got {rems:?}");
        let mut entity = Entity::open("127.0.0.1:1", Kernel::named("withhold"));
        entity.withhold_form(&rule.form, &family, &rems, mag);
        assert!(
            entity.dialect_len() >= 1,
            "the withheld form or its residual stands, dialect {}",
            entity.dialect_len()
        );
        assert_eq!(
            entity.uttered_n(),
            n_res,
            "one utterance per residual, taught_up {}",
            entity.uttered_n()
        );
    }

    /// The court promotes a dialect vertex into the genome, carrying uses,
    /// and never admits a second copy.
    #[test]
    fn learn_one_promotes_dialect_same_vertex() {
        use crate::transducer::family::Crystal;
        let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
        let mut epi = Epigenome::new();
        epi.stand_dialect_at(e1.clone(), 1, 1);
        epi.stand_dialect_at(e1.clone(), 2, 2);
        assert_eq!(epi.dialect_len(), 1);
        assert_eq!(epi.genome_len(), 0);
        let uses = 2u64;
        let _ = uses;
        let heard = epi.learn(&e1.to_bytes()).expect("promoted");
        let _ = heard;
        assert_eq!(epi.dialect_len(), 0, "dialect moves, it is not copied");
        assert_eq!(epi.genome_len(), 1, "genome is the promoted vertex");
        let promoted = epi.take_promoted();
        assert_eq!(promoted.len(), 1);
        assert_eq!(epi.expression[0], 2, "uses ride with the promotion");
        let inv = crate::transducer::stalk::invariant_of(&e1);
        let on = epi.on_class(&inv);
        assert!(
            on.iter().any(|(_, _, st)| *st == Standing::Ruled),
            "Crystal carries standing as a coordinate after promotion"
        );
    }

    /// A stood dialect word closes a remainder without waiting for the court.
    #[test]
    fn a_stood_word_closes_todays_remainder() {
        use crate::transducer::element::from_residual;
        use crate::transducer::germinate::germinate;
        let mut epi = Epigenome::new();
        let word = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
        epi.stand_dialect_at(word.clone(), 1, 1);
        assert_eq!(epi.genome_len(), 0);
        let chain = word.sub().to_vec();
        let located = {
            // locate is private; comprehend over the same chain is the public read.
            epi.comprehend(&from_residual(&chain))
        };
        match located {
            Comprehension::Understood(_) | Comprehension::Held | Comprehension::Admitted => {}
            other => {
                // residual words locate as the chain of the stood word
                let again = epi.comprehend(&word);
                assert!(
                    matches!(again, Comprehension::Understood(_) | Comprehension::Admitted | Comprehension::Held),
                    "a stood word is on the working language, got {other:?} then {again:?}"
                );
            }
        }
    }

    /// Shed never removes a dialect entry a live idea glued, and never the genome.
    #[test]
    fn shed_spares_glued_dialect_and_the_genome() {
        let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
        let e2 = two_body_sentence([&[(1, 3), (1, 3)], &[(3, 5), (3, 5)]]);
        let mut entity = Entity::open("127.0.0.1:1", Kernel::named("shed"));
        entity.epigenome.stand_dialect_at(e1.clone(), 0, 0);
        entity.epigenome.stand_dialect_at(e2.clone(), 0, 0);
        assert_eq!(entity.dialect_len(), 2);
        let addr = e1.address();
        entity.hold_thought(vec![(9, 0, 1, 2)]);
        let idea = entity
            .contemplation
            .bubbles
            .fiber(&crate::network::nerve::ROOT)
            .next()
            .map(|(k, _)| *k)
            .expect("idea");
        entity.contemplation.mark_glue(&idea, addr);
        let aperture = crate::physics::aperture_cycle() as u64;
        // The first aperture after boot is unmeasured: nothing sheds while
        // the stroke ring fills, however idle an entry is.
        for tu in 1..aperture {
            entity.shed_at(tu);
            assert_eq!(entity.dialect_len(), 2, "an unmeasured aperture sheds nothing");
        }
        // The ring is full at this stroke: the aperture is measured, the
        // idle unglued entry sheds, the glued one stands.
        entity.shed_at(aperture + 1);
        assert_eq!(entity.dialect_len(), 1, "the idle unglued entry sheds; the glued one stands");
        assert!(entity.epigenome.working_word(&addr).is_some(), "the glued entry stands");
        assert!(entity.epigenome.working_word(&e2.address()).is_none(), "the idle entry is gone");
        assert_eq!(entity.epigenome.genome_len(), 0, "genome is untouched by shed");
    }

    /// Equal rotor, different ρ / σ / shift: three genome entries. Two
    /// spellings of one vertex still merge.
    #[test]
    fn rho_sigma_shift_are_three_vertices_two_spellings_merge() {
        use crate::transducer::element::{affine_of, from_connection, identity_versor};
        use num_bigint::BigInt;
        let qn = |n: i64| crate::core::engine::Q::from(BigInt::from(n));
        let id = affine_of(&identity_versor()).expect("affine");
        let swap = from_connection(&id, &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0]).expect("swap");
        let cycle = from_connection(&id, &[(1, 2), (2, 3), (3, 1)], &Vec::new(), [1, 0, 0, 0]).expect("cycle");
        let shifted = vec![
            vec![qn(1), qn(0), qn(5)],
            vec![qn(0), qn(1), qn(0)],
            vec![qn(0), qn(0), qn(1)],
        ];
        let place = from_connection(&shifted, &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0]).expect("shift");
        let mut epi = Epigenome::new();
        assert!(epi.learn(&swap.to_bytes()).is_some());
        assert!(epi.learn(&cycle.to_bytes()).is_some());
        assert!(epi.learn(&place.to_bytes()).is_some());
        assert_eq!(epi.genome_len(), 3, "ρ/σ/shift are three vertices");
        let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).unwrap();
        let long = crate::transducer::alphabet::Word::from_letters(&[
            (q, false),
            (q, false),
            (q, false),
            (q, false),
        ]);
        let mut alt = swap.clone();
        let letters = alt.letters().expect("connection");
        let spelling = crate::transducer::alphabet::Spelling {
            word: long,
            rho: letters.spelling.rho,
        };
        let rem = alt.sub().to_vec();
        alt.set_description(crate::transducer::element::connection_word(
            &spelling,
            &letters.shift,
            &letters.sigma,
            &rem,
        ));
        assert_eq!(alt.address(), swap.address(), "two spellings, one address");
        assert!(epi.learn(&alt.to_bytes()).is_some());
        assert_eq!(epi.genome_len(), 3, "a second tape of the same address does not mint");
    }

    /// An expressed word stands at the voice's K and TUniversal, so a shed
    /// inside the aperture does not drop the mind's own speech.
    #[test]
    fn expressed_word_survives_shed_within_the_aperture() {
        use crate::transducer::element::from_residual;
        let mut epi = Epigenome::new();
        let residual = [(0i64, 0i64, 1u8, 2u8)];
        assert!(epi.express(&residual, 7, 10), "the residual is claimed");
        let addr = from_residual(&residual.to_vec()).address();
        let d = epi.dialect.get(&addr).expect("stood");
        assert_eq!(d.last_used_k, 7, "k is the voice's K");
        assert_eq!(d.last_used_tu, 10, "tu is TUniversal");
        let aperture = crate::physics::aperture_cycle() as u64;
        let now = 10 + aperture - 1;
        for tu in 10..=now {
            epi.note_stroke(tu);
        }
        let n = epi.shed(now);
        assert_eq!(n, 0, "expressed within the aperture stands");
        assert_eq!(epi.dialect_len(), 1);
        assert!(epi.working_word(&addr).is_some());
    }
}
