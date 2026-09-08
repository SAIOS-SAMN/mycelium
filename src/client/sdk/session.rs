// 24.3 — THE SESSION: one client's contact with the mesh.
//
//! Discovery, the peer cursor, the cover it absorbs, the route it takes for
//! a chart, and every act it speaks. Each exchange is bounded by the
//! transport aperture, which is denominated in strokes (law 9).

use super::*;

/// A kernel's session with a hypha. Identity is stable; sockets are pooled
/// keep-alives per destination so a quote/probe/attest does not open a new
/// TCP handshake for every act.
pub struct Session {
    pub(crate) addr: String,
    pub(crate) kernel: Kernel,
    /// The voice this session is the mouth of.
    pub(crate) voice: crate::voice::Voice,
    /// Consecutive failed exchanges per hypha address.
    pub(crate) resistance: RefCell<crate::network::nerve::Keyed<u64>>,
    /// THE NERVE, as this entity holds it: every hypha it has learned of,
    /// placed by the same XOR router the hyphas use, so an act carrying a
    /// key (a listing, a task, a word) is spoken to that key's home and a
    /// stranger there is the second voice. The dial address is the seed.
    pub(crate) router: RefCell<crate::network::nerve::Router>,
    /// Peer addresses keyed by the contact string's bytes.
    pub(crate) peers: crate::network::nerve::Keyed<String>,
    /// The quote's stroke: quotes walk the nerve in turn, one hypha per
    /// stroke, so every book is thought about and none is a monologue.
    pub(crate) strokes: std::cell::Cell<u64>,
    /// Keep-alive streams keyed by destination address.
    pub(crate) streams: RefCell<crate::network::nerve::Keyed<TcpStream>>,
    /// Cover trie learned from digest replies. Same read as the hypha.
    pub(crate) cover: RefCell<crate::network::nerve::Cover>,
    pub(crate) rooms: RefCell<crate::network::nerve::Keyed<[u64; 4]>>,
    pub(crate) addrs: RefCell<crate::network::nerve::Keyed<String>>,
    /// Last measured round-trip per hypha address; a timeout raises it by
    /// the aperture ratio, a success sets it to the measured trip.
    pub(crate) rtt: RefCell<crate::network::nerve::Keyed<u64>>,
    /// Last aperture of RTTs per address, in strokes; the floor is the min of this window.
    pub(crate) rtt_window: RefCell<crate::network::nerve::Keyed<Vec<u64>>>,
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

    pub(crate) fn ask_peers(&mut self, addr: &str) {
        let Ok(body) = self.exchange_at(addr, dialogue::encode_peers_request(&self.kernel.voice)) else {
            return;
        };
        for a in dialogue::decode_peers_reply(&body).unwrap_or_default() {
            if !self.peers.contains(a.as_bytes()) {
                self.peers.insert(a.as_bytes(), a.clone());
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

    pub(crate) fn peer_cursor(&self) -> usize {
        self.strokes.get() as usize
    }

    pub(crate) fn bump_cursor(&self) {
        let n = self.strokes.get();
        self.strokes.set(n.wrapping_add(1));
    }

    pub(crate) fn absorb_digest(&self, body: &[u8]) {
        let Some(crate::anastome::dialogue::CoverDigest {
            charts,
            room,
            cover_roots,
            ruled_root,
            vk,
            bind: addr,
            sig,
        }) = crate::anastome::dialogue::decode_cover_digest(body)
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

    /// An act of this voice was admitted; its own clock steps (45.4).
    pub fn advance(&mut self) -> u64 {
        self.voice.advance()
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

    pub(crate) fn rtt_of(&self, addr: &str) -> Option<u64> {
        self.rtt.borrow().get(addr.as_bytes()).copied()
    }

    pub(crate) fn rtt_ok(&self, addr: &str, sample: u64) {
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

    pub(crate) fn rtt_timeout(&self, addr: &str) {
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

    pub(crate) fn z_at(&self, addr: &str) -> u64 {
        self.resistance.borrow().get(addr.as_bytes()).copied().unwrap_or(0)
    }

    pub(crate) fn occluded_at(&self, addr: &str) -> bool {
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
                        .map(|i| Cluster {
                            address: i.addr,
                            mu: i.mu,
                            chain: i.chain,
                            open: i.open,
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

    pub(crate) fn exchange(&self, act_body: Vec<u8>) -> io::Result<Vec<u8>> {
        self.exchange_at(&self.addr.clone(), act_body)
    }

    /// One exchange with a named hypha on the nerve; the impedance is
    /// measured on the wire as a whole.
    pub(crate) fn exchange_at(&self, addr: &str, act_body: Vec<u8>) -> io::Result<Vec<u8>> {
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

    pub(crate) fn exchange_inner(&self, addr: &str, act_body: Vec<u8>) -> io::Result<Vec<u8>> {
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

    pub(crate) fn roundtrip(s: &mut TcpStream, bytes: &[u8]) -> io::Result<Vec<u8>> {
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
