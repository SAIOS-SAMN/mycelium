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
use std::io::{self, Read, Write};
use std::net::TcpStream;

mod entity;
mod epigenome;
#[cfg(test)]
mod tests;

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
    /// Seat a kernel by name. The seed is a KDF over the name; the identity
    /// carried everywhere is the verifying key.
    pub fn named(name: &str) -> Self {
        let voice = crate::court::identity::HyphaKey::vk_from_name(name);
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
    resistance: RefCell<crate::network::nerve::Keyed<u64>>,
    /// THE NERVE, as this entity holds it: every hypha it has learned of,
    /// placed by the same XOR router the hyphas use, so an act carrying a
    /// key (a listing, a task, a word) is spoken to that key's home and a
    /// stranger there is the second voice. The dial address is the seed.
    router: RefCell<crate::network::nerve::Router>,
    /// Peer addresses keyed by the contact string's bytes.
    peers: crate::network::nerve::Keyed<String>,
    /// The quote's stroke: quotes walk the nerve in turn, one hypha per
    /// stroke, so every book is thought about and none is a monologue.
    strokes: std::cell::Cell<u64>,
    /// Keep-alive streams keyed by destination address.
    streams: RefCell<crate::network::nerve::Keyed<TcpStream>>,
    /// Cover trie learned from digest replies. Same read as the hypha.
    cover: RefCell<crate::network::nerve::Cover>,
    rooms: RefCell<crate::network::nerve::Keyed<[u64; 4]>>,
    addrs: RefCell<crate::network::nerve::Keyed<String>>,
    /// Last measured round-trip per hypha address; a timeout raises it by
    /// the aperture ratio, a success sets it to the measured trip.
    rtt: RefCell<crate::network::nerve::Keyed<u64>>,
    /// Last aperture of RTTs per address, in strokes; the floor is the min of this window.
    rtt_window: RefCell<crate::network::nerve::Keyed<Vec<u64>>>,
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
        router.insert(kernel.voice(), addr.to_string());
        Session {
            addr: addr.to_string(),
            kernel,
            voice,
            resistance: RefCell::new(crate::network::nerve::Keyed::new()),
            router: RefCell::new(router),
            peers: {
                let mut p = crate::network::nerve::Keyed::new();
                p.insert(addr.as_bytes(), addr.to_string());
                p
            },
            strokes: std::cell::Cell::new(0),
            streams: RefCell::new(crate::network::nerve::Keyed::new()),
            cover: RefCell::new(crate::network::nerve::Cover::new()),
            rooms: RefCell::new(crate::network::nerve::Keyed::new()),
            addrs: RefCell::new(crate::network::nerve::Keyed::new()),
            rtt: RefCell::new(crate::network::nerve::Keyed::new()),
            rtt_window: RefCell::new(crate::network::nerve::Keyed::new()),
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
            if !self.peers.contains(a.as_bytes()) {
                self.peers.insert(a.as_bytes().to_vec(), a);
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

    /// The hypha a key lives on: XOR-closest known contact to the key's
    /// bits. None when the nerve holds no contact (unmeasured).
    pub fn route(&self, key: &[u8]) -> Option<String> {
        self.router.borrow().closest(key).map(|(_, a)| a)
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
        for (id, room) in rooms.fiber(crate::network::nerve::ROOT) {
            if addrs.get(&id).is_some_and(|a| self.occluded_at(a)) {
                continue;
            }
            open_rooms.insert(&id, *room);
        }
        let (_, dest) = home(&cover, inv, mag, &open_rooms, &addrs, None);
        if dest.is_empty() {
            // Empty cover: XOR-closest known hypha to the chart key, not the seed.
            self.route(inv.key().as_bytes()).unwrap_or_default()
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
        self.addrs.borrow_mut().insert(vk, addr.clone());
        self.router.borrow_mut().insert(vk, addr);
    }

    /// The hyphas this entity knows.
    pub fn peers(&self) -> Vec<String> {
        self.peers.fiber(crate::network::nerve::ROOT).map(|(_, a)| a.clone()).collect()
    }

    pub fn add_peer(&mut self, addr: &str) {
        if self.peers.contains(addr.as_bytes()) {
            return;
        }
        self.peers.insert(addr.as_bytes(), addr.to_string());
    }

    /// A known hypha: contact string on the socket table, verifying key on
    /// the XOR nerve. Empty-cover routing is closest(chart key) among vks.
    pub fn know_hypha(&mut self, addr: &str, vk: [u8; 32]) {
        self.add_peer(addr);
        self.router.borrow_mut().insert(vk, addr.to_string());
    }

    /// Speak an act carrying a key to that key's home.
    pub fn speak_keyed(&self, key: &[u8], act_body: Vec<u8>) -> io::Result<Vec<u8>> {
        let Some(addr) = self.route(key) else {
            return Err(io::Error::new(io::ErrorKind::NotConnected, "unmeasured route"));
        };
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
        // W8: a max over wires collapses them to one impedance. Left as
        // the reading it was, keyed now; the per-wire read is `z_at`.
        let r = self.resistance.borrow();
        let mut z = 0u64;
        for (_, v) in r.fiber(crate::network::nerve::ROOT) {
            z = z.max(*v);
        }
        z
    }

    pub fn rtt(&self) -> Option<u64> {
        self.rtt_of(&self.addr)
    }

    fn rtt_of(&self, addr: &str) -> Option<u64> {
        self.rtt.borrow().get(addr.as_bytes()).copied()
    }

    fn rtt_ok(&self, addr: &str, sample: u64) {
        let n = crate::physics::aperture_cycle().max(1) as usize;
        let mut windows = self.rtt_window.borrow_mut();
        let mut w = windows.get(addr.as_bytes()).cloned().unwrap_or_default();
        w.push(sample);
        if w.len() > n {
            w.remove(0);
        }
        let floor = w.iter().copied().min().unwrap_or(sample);
        windows.insert(addr.as_bytes(), w);
        let cap = crate::physics::persist_dwell(floor);
        self.rtt.borrow_mut().insert(addr.as_bytes(), sample.min(cap));
    }

    fn rtt_timeout(&self, addr: &str) {
        let Some(floor) = self
            .rtt_window
            .borrow()
            .get(addr.as_bytes())
            .and_then(|w| w.iter().copied().min())
        else {
            return;
        };
        let ratio = crate::physics::aperture_cycle().max(2) as u64;
        let cap = crate::physics::persist_dwell(floor);
        let mut m = self.rtt.borrow_mut();
        if let Some(cur) = m.get(addr.as_bytes()).copied() {
            m.insert(addr.as_bytes(), cur.saturating_mul(ratio).min(cap));
        }
    }

    fn z_at(&self, addr: &str) -> u64 {
        self.resistance.borrow().get(addr.as_bytes()).copied().unwrap_or(0)
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
        let body = self.exchange_at(
            &dest,
            dialogue::encode_demand_at(&self.kernel.voice, self.voice.k(), chain, cites),
        )?;
        let _ = key;
        Ok(decode_reply(&body))
    }

    /// PROBE an open listing with a tentative sentence; the hypha replays and
    /// settles graded descent or closure. Returns (code, paid, remaining).
    pub fn probe(&self, target: &[u8], cites: &[dialogue::CiteWire]) -> io::Result<(u8, [u64; 4], [u64; 4])> {
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
                    let all: Vec<Vec<u8>> =
                        r.fiber(crate::network::nerve::ROOT).map(|(k, _)| k).collect();
                    for k in all {
                        if let Some(z) = r.get_mut(&k) {
                            *z = z.saturating_sub(1);
                        }
                    }
                    r.insert(addr.as_bytes(), 0);
                }
                _ => {
                    let cur = r.get(addr.as_bytes()).copied().unwrap_or(0);
                    r.insert(addr.as_bytes(), cur + 1);
                }
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
        let mut pool = self.streams.borrow_mut();
        if let Some(s) = pool.get_mut(addr.as_bytes()) {
            match Self::roundtrip(s, &bytes) {
                Ok(body) => {
                    self.rtt_ok(addr, 1);
                    return Ok(body);
                }
                Err(e) => {
                    pool.remove(addr.as_bytes());
                    if e.kind() == io::ErrorKind::TimedOut || e.kind() == io::ErrorKind::WouldBlock {
                        self.rtt_timeout(addr);
                    }
                }
            }
        }
        let mut s = match TcpStream::connect(addr) {
            Ok(s) => s,
            Err(e) => {
                self.rtt_timeout(addr);
                return Err(e);
            }
        };
        let _ = s.set_nodelay(true);
        let body = match Self::roundtrip(&mut s, &bytes) {
            Ok(b) => b,
            Err(e) => {
                self.rtt_timeout(addr);
                return Err(e);
            }
        };
        self.rtt_ok(addr, 1);
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
    Understood(Vec<(Vec<u8>, Standing)>),
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
    Withheld { missing: Vec<Vec<u8>> },
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
    pub missing: Vec<Vec<u8>>,
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
    pub fn fiber(&self, inv: &crate::transducer::stalk::Invariant) -> Vec<(Vec<u8>, Standing)> {
        let mut out: Vec<(Vec<u8>, Standing)> = Vec::new();
        let mut have: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for a in self.genome.fiber(inv) {
            have.insert(&a, ());
            out.push((a, Standing::Ruled));
        }
        for a in self.dialect.fiber(inv) {
            if have.contains(&a) {
                continue;
            }
            have.insert(&a, ());
            out.push((a, Standing::Stood));
        }
        if let Some(ideas) = self.ideas {
            if let Some(m) = ideas.get(inv.key().as_bytes()) {
                for (a, _) in m.fiber(crate::network::nerve::ROOT) {
                    if have.contains(&a) {
                        continue;
                    }
                    have.insert(&a, ());
                    out.push((a, Standing::Held));
                }
            }
        }
        out
    }

    pub fn standing_of(&self, addr: &[u8], inv: &crate::transducer::stalk::Invariant) -> Option<Standing> {
        if self.genome.chart_of(addr) == Some(inv) {
            return Some(Standing::Ruled);
        }
        if self.dialect.chart_of(addr) == Some(inv) {
            return Some(Standing::Stood);
        }
        if self
            .ideas
            .and_then(|m| m.get(inv.key().as_bytes()))
            .is_some_and(|inner| inner.contains(addr))
        {
            return Some(Standing::Held);
        }
        None
    }

    pub fn standing_addr(&self, addr: &[u8]) -> Option<Standing> {
        if self.genome.contains(addr) {
            return Some(Standing::Ruled);
        }
        if self.dialect.contains(addr) {
            return Some(Standing::Stood);
        }
        if let Some(ideas) = self.ideas {
            if ideas.fiber(crate::network::nerve::ROOT).any(|(_, m)| m.contains(addr)) {
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
    recollections: crate::network::nerve::Keyed<(Vec<Vec<u8>>, u64)>,
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
    promoted: Vec<Vec<u8>>,
    /// Dialects dropped this process.
    shed_n: u64,
    /// TUniversal at each of the last aperture_cycle strokes.
    stroke_k: Vec<u64>,
    /// Held words keyed by (chart, axis). Filled at atlas/dialect admit,
    /// emptied at remove/shed/retire. `on_axis` is one fiber read.
    axis_index: crate::transducer::sentence::AxisIndex,
    /// Missing cited address → citing dialect sentences. Filled when
    /// Withheld is banked; resolve_withheld reads this, never the dialect.
    waiting_on: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
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
    /// Ideas keyed by address: a cite is a read, never a walk of bubbles.
    ideas: crate::network::nerve::Keyed<Element>,
    physics: crate::physics::Physics,
    height: crate::chronometry::Height,
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
    pub address: crate::transducer::a21::Address,
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
    pub germ: crate::transducer::a21::Address,
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
