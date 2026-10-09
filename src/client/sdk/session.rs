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
    /// The quote's stroke: quotes run round the nerve in turn, one hypha per
    /// stroke, so every book is thought about and none is a monologue.
    pub(crate) strokes: std::cell::Cell<u64>,
    /// Keep-alive streams keyed by destination address.
    pub(crate) streams: RefCell<crate::network::nerve::Keyed<TcpStream>>,
    /// Cover trie learned by page from `ACT_COVER`. Same read as the hypha.
    pub(crate) cover: RefCell<crate::network::nerve::Cover>,
    /// The four cover roots and the ruled root last read from each hypha:
    /// the cover is fetched when they MOVE, and not on every digest.
    pub(crate) last_roots: RefCell<crate::network::nerve::Keyed<crate::anastome::dialogue::Roots>>,
    pub(crate) rooms: RefCell<crate::network::nerve::Keyed<[u64; 4]>>,
    pub(crate) addrs: RefCell<crate::network::nerve::Keyed<String>>,
    /// Last measured round-trip per hypha address; a timeout raises it by
    /// the aperture ratio, a success sets it to the measured trip.
    pub(crate) rtt: RefCell<crate::network::nerve::Keyed<u64>>,
    /// Last aperture of RTTs per address, in strokes; the floor is the min of this window.
    pub(crate) rtt_window: RefCell<crate::network::nerve::Keyed<Vec<u64>>>,
    /// 78.152 TU-7 — THE LAST FINAL POINT this session read: carried behind
    /// every act it speaks, raised by every door reply that carries a later
    /// one. `None` before the first.
    pub(crate) final_point: std::cell::Cell<Option<crate::chronometry::tuniversal::TPoint>>,
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
            last_roots: RefCell::new(crate::network::nerve::Keyed::new()),
            rooms: RefCell::new(crate::network::nerve::Keyed::new()),
            addrs: RefCell::new(crate::network::nerve::Keyed::new()),
            rtt: RefCell::new(crate::network::nerve::Keyed::new()),
            rtt_window: RefCell::new(crate::network::nerve::Keyed::new()),
            final_point: std::cell::Cell::new(None),
        }
    }

    /// 78.152 TU-7 — THE TUNIVERSAL READ: the last FINAL point this session
    /// read off a hypha's door — `origin/h:S:G:T:H:K`, final on two keepers'
    /// verified readings (TU-3) — or `None` before the first. No ask is
    /// spoken: every door reply carries the door's final point, and this is
    /// the greatest read. A kernel drives its clock by it and stamps what it
    /// publishes with it (`TPoint::print`); its own step clock stays its own.
    pub fn tuniversal(&self) -> Option<crate::chronometry::tuniversal::TPoint> {
        self.final_point.get()
    }

    /// A point read off a door reply: held where it advances on the one held.
    pub(crate) fn hear_point(&self, point: Option<crate::chronometry::tuniversal::TPoint>) {
        if let Some(p) = point {
            if p.advances(self.final_point.get().as_ref()) {
                self.final_point.set(Some(p));
            }
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
        let Some(i) = self.peer_start(known.len()) else { return };
        self.bump_cursor();
        if budget.spend() {
            self.ask_peers(&known[i]);
        }
        if budget.spend() {
            let voice = self.kernel.voice;
            if let Ok(body) = self.exchange_at(&known[i], dialogue::encode_digests_request(&voice)) {
                if self.absorb_digest(&body) {
                    self.fetch_cover(&known[i]);
                }
            }
        }
    }

    pub(crate) fn ask_peers(&mut self, addr: &str) {
        let Ok(body) = self.exchange_at(addr, dialogue::encode_peers_request(&self.kernel.voice)) else {
            return;
        };
        // A reply that does not decode names no peers, and none are
        // admitted from it (rule 11).
        let Some(peers) = dialogue::decode_peers_reply(&body) else {
            return;
        };
        for a in peers {
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
            if self.absorb_digest(&body) {
                self.fetch_cover(&addr);
            }
        }
    }

    /// The hypha a key lives on: XOR-closest known contact to the key's
    /// bits. None when the nerve holds no contact (unmeasured).
    pub fn route(&self, key: &[u8]) -> Option<String> {
        self.router.borrow().closest(key).map(|(_, a)| a)
    }

    /// Home a word by its chart and magnitude. Occluded edges drop out of
    /// the fiber so incidence falls.
    /// `None` where neither the cover nor the router names a hypha: 0.11c's
    /// own ruling on this reader — a destination that is not known is
    /// absent, never the empty address, which every caller would then have
    /// to tell apart from a hypha bound at one.
    pub fn route_chart(
        &self,
        inv: &crate::transducer::stalk::Invariant,
        mag: &[u64; 4],
    ) -> Option<String> {
        use crate::network::nerve::home;
        let cover = self.cover.borrow();
        let rooms = self.rooms.borrow();
        let addrs = self.addrs.borrow();
        // Impedance is a coordinate on the address entry; occluded hyphas
        // drop out of rooms rather than rebuilding a live trie.
        // The rooms this voice has learned, one entry per hypha: bounded by
        // the mesh, and the occluded ones are dropped from the copy rather
        // than a second live trie being kept beside it.
        let mut open_rooms = crate::network::nerve::Keyed::new();
        for (id, room) in rooms.scoped() {
            if addrs.get(&id).is_some_and(|a| self.occluded_at(a)) {
                continue;
            }
            open_rooms.insert(&id, *room);
        }
        let (_, dest) = home(&cover, inv, mag, &open_rooms, &addrs, None);
        if dest.is_empty() {
            // Empty cover: XOR-closest known hypha to the chart key, not
            // the seed. A router that knows no hypha names none.
            self.route(inv.key().as_bytes())
        } else {
            Some(dest)
        }
    }

    /// WHERE A ROUND OF `n` PEERS STARTS: the session's strokes modulo
    /// `n`, reduced in the count's own `u64` and only then narrowed — a
    /// value below `n` fits `usize` by construction. It narrowed the count
    /// first, which on a narrower target wrapped before it rotated. `None`
    /// where there is no peer to start at.
    pub(crate) fn peer_start(&self, n: usize) -> Option<usize> {
        let at = self.strokes.get().checked_rem(u64::try_from(n).ok()?)?;
        usize::try_from(at).ok()
    }

    pub(crate) fn bump_cursor(&self) {
        let n = self.strokes.get();
        self.strokes.set(n.wrapping_add(1));
    }

    /// Absorb a digest reply and say whether that hypha's ROOTS MOVED. A
    /// caller holding the address it dialled fetches the cover with
    /// [`Session::fetch_cover`] when they did; a caller that does not
    /// learns the room and the route, which is what the digest carries.
    pub(crate) fn absorb_digest(&self, body: &[u8]) -> bool {
        let Some(crate::anastome::dialogue::CoverDigest {
            room,
            cover_roots,
            ruled_root,
            vk,
            bind: addr,
            sig,
            // The keeper ring is the courts' (78.152): a voice reads the
            // final point off its door's replies, never off one keeper.
            ring: _,
        }) = crate::anastome::dialogue::decode_cover_digest(body)
        else {
            return false;
        };
        let prefix = crate::anastome::dialogue::cover_signed_prefix(&cover_roots, &ruled_root, &room);
        if !crate::court::identity::verify(&vk, &prefix, &sig) {
            return false;
        }
        // A4: the digest is the ROOTS. A voice whose record of this hypha's
        // roots moved asks for its cover by page (`fetch_cover`), and
        // nothing here retracts a cover this reading cannot replace.
        let now = (cover_roots, ruled_root);
        let moved = self.last_roots.borrow().get(&vk) != Some(&now);
        self.last_roots.borrow_mut().insert(vk, now);
        self.rooms.borrow_mut().insert(vk, room);
        self.addrs.borrow_mut().insert(vk, addr.clone());
        self.router.borrow_mut().insert(vk, addr);
        moved
    }

    /// A hypha's COVER, by page, from the last cursor read: the digest
    /// carries the four roots and no charts (A4), so this is the only way a
    /// voice's trie is filled. The first page retracts what was held for
    /// that hypha, since the page is its own statement of what it holds.
    pub(crate) fn fetch_cover(&self, addr: &str) {
        let mut from: Vec<u8> = Vec::new();
        // Rule 10: the bound is the transport aperture in strokes, the
        // measure this session paces its other acts by -- never "until the
        // peer stops saying more", which is the peer's loop and not this
        // one's. A cover wider than the aperture converges over contacts,
        // since the cursor is the last chart held and nothing is re-read.
        let mut pages = crate::physics::aperture_cycle() as usize;
        while pages > 0 {
            pages -= 1;
            // A cursor past its four bytes asks nothing (STD1 3c).
            let Some(ask) = crate::anastome::dialogue::encode_cover_request(&self.kernel.voice, &from) else {
                return;
            };
            let Ok(page) = self.exchange_at(addr, ask) else {
                return;
            };
            let Some((cover, more)) = crate::anastome::dialogue::decode_cover_page(&page) else {
                return;
            };
            if from.is_empty() {
                self.cover.borrow_mut().retract_hypha(&cover.vk);
            }
            let read = cover.charts.len();
            for (lane, inv) in &cover.charts {
                self.cover.borrow_mut().admit(*lane, inv, cover.vk);
                from = crate::anastome::dialogue::cover_cursor(*lane, inv);
            }
            if !more || read == 0 {
                return;
            }
        }
    }

    /// The hyphas this entity knows.
    pub fn peers(&self) -> Vec<String> {
        self.peers.scoped().map(|(_, a)| a.clone()).collect()
    }

    /// WHAT THIS SESSION HOLDS OF THE MESH, as counts (§8.1, measurement
    /// only): the peers, the pooled streams, the cover's charts, the roots
    /// last read per hypha, the rooms, the addresses, the resistance
    /// entries, the round trips and the round-trip windows. Counts and
    /// never bytes, one per carrier, so the stroke line says WHICH of them
    /// grows while every entity carrier is flat (L33); nothing branches on
    /// it.
    pub fn sizes(&self) -> [usize; 9] {
        [
            self.peers.len(),
            self.streams.borrow().len(),
            self.cover.borrow().all_charts().len(),
            self.last_roots.borrow().len(),
            self.rooms.borrow().len(),
            self.addrs.borrow().len(),
            self.resistance.borrow().len(),
            self.rtt.borrow().len(),
            self.rtt_window.borrow().len(),
        ]
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

    /// The voice this session speaks with: an identity that speaks (78.49).
    pub fn voice(&self) -> &crate::voice::Voice {
        &self.voice
    }

    /// The measured impedance Z on THIS SESSION'S OWN WIRE: consecutive
    /// failed exchanges with the peer it is bound to.
    ///
    /// It was a `max` over every wire the session had ever touched, which
    /// is rule 4's "any `.max()` on two measurements" and collapses N
    /// independent readings to one: a mind with one occluded peer and nine
    /// open ones read itself as occluded, and `transport()` -- which paces
    /// the whole stroke -- read it that way too. The wires are not one
    /// wire, and 42.6 prices the mesh cover by the hyphae covering a
    /// stratum, not by the worst of them. `z_at` reads any other wire, and
    /// `occluded_at` decides one.
    pub fn impedance(&self) -> u64 {
        self.z_at(&self.addr)
    }

    pub fn rtt(&self) -> Option<u64> {
        self.rtt_of(&self.addr)
    }

    pub(crate) fn rtt_of(&self, addr: &str) -> Option<u64> {
        self.rtt.borrow().get(addr.as_bytes()).copied()
    }

    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — the floor of the RTT window is that window's extent, a measured reading of the samples it holds"
    )]
    pub(crate) fn rtt_ok(&self, addr: &str, sample: u64) {
        let n = crate::physics::aperture_cycle() as usize;
        let mut windows = self.rtt_window.borrow_mut();
        let mut w = windows.get(addr.as_bytes()).cloned().unwrap_or_default();
        w.push(sample);
        if w.len() > n {
            w.remove(0);
        }
        // The window holds this sample, so it has a floor; a window that
        // somehow holds none, or a dwell that does not fit, records no
        // reading rather than one it did not take (rule 11).
        let (Some(floor), _) = (w.iter().copied().min(), ()) else {
            return;
        };
        windows.insert(addr.as_bytes(), w);
        // A dwell the ledger cannot count caps nothing, and the sample
        // stands as measured.
        let cap = match crate::physics::persist_dwell(floor) {
            crate::physics::Lambda::Strokes(n) => n,
            crate::physics::Lambda::BeyondTheLedger => u64::MAX,
            crate::physics::Lambda::Unmeasured => return,
        };
        self.rtt.borrow_mut().insert(addr.as_bytes(), sample.min(cap));
    }

    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — the floor of the RTT window is that window's extent; a peer with no window has no floor and refuses"
    )]
    pub(crate) fn rtt_timeout(&self, addr: &str) {
        let Some(floor) = self
            .rtt_window
            .borrow()
            .get(addr.as_bytes())
            .and_then(|w| w.iter().copied().min())
        else {
            return;
        };
        let ratio = u64::from(crate::physics::aperture_cycle());
        // As above: a dwell beyond the ledger caps nothing.
        let cap = match crate::physics::persist_dwell(floor) {
            crate::physics::Lambda::Strokes(n) => n,
            crate::physics::Lambda::BeyondTheLedger => u64::MAX,
            crate::physics::Lambda::Unmeasured => return,
        };
        let mut m = self.rtt.borrow_mut();
        // A backed-off dwell that does not fit is not a dwell this peer
        // was measured at; the standing reading holds (rule 11).
        if let Some(cur) = m.get(addr.as_bytes()).copied() {
            if let Some(backed) = cur.checked_mul(ratio) {
                m.insert(addr.as_bytes(), backed.min(cap));
            }
        }
    }

    pub(crate) fn z_at(&self, addr: &str) -> u64 {
        // A peer the carrier has no resistance for has met none: the
        // reading of absence (0.11c).
        match self.resistance.borrow().get(addr.as_bytes()) {
            Some(z) => *z,
            None => 0,
        }
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

    /// PROBE an open listing with a tentative sentence; the hypha replays and
    /// settles a graded trajectory or closure. Returns its reading (PR1).
    pub fn probe(&self, target: &[u8], cites: &[dialogue::CiteWire]) -> io::Result<dialogue::ProbeReading> {
        // LD0: a probe whose address cannot be framed is not spoken.
        let Some(frame) = dialogue::encode_probe(&self.kernel.voice, target, cites) else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "probe refused: an address cannot be framed"));
        };
        let body = self.speak_keyed(target, frame)?;
        dialogue::decode_probe_reply(&body)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "probe decode"))
    }

    /// ASK the quote — "what is the highest-yield work?" The hypha answers with
    /// the yield surface (per-lane, priced off genuine metabolic tension) and the
    /// open-demand market's top clusters, each carrying its chain so this kernel
    /// can actually attempt the reduction.
    pub fn quote(&self, charts: &[crate::transducer::stalk::Invariant]) -> io::Result<YieldQuote> {
        let Some(req) = dialogue::encode_quote_request_charts(&self.kernel.voice, charts) else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "the ask does not fit the lengths its frame writes",
            ));
        };
        match self.speak_nerve(req) {
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
                            element: i.element,
                            open: i.open,
                        })
                        .collect(),
                })
            }
            // A deferred quote is no quote: no height, no K, no clusters —
            // and the door's deficit as the surface, so the arm dwells on
            // the lanes the judge named (23.8, S13.4).
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(YieldQuote {
                height: 0,
                // A would-block that carries no deferral names no lanes,
                // and no surface is read off one.
                surface: match e
                    .get_ref()
                    .and_then(|inner| inner.downcast_ref::<Deferral>())
                {
                    Some(d) => d.lanes,
                    None => [0; 4],
                },
                k: 0,
                clusters: Vec::new(),
            }),
            Err(e) => Err(e),
        }
    }

    /// Speak an unkeyed act on the nerve: every learned hypha, starting at
    /// the stroke. A full reflex pool is impedance, not a failed quote.
    pub fn speak_nerve(&self, act_body: Vec<u8>) -> io::Result<Vec<u8>> {
        let peer_list = self.peers();
        let npeers = peer_list.len().max(1);
        let Some(start) = self.peer_start(npeers) else {
            return Err(io::Error::new(io::ErrorKind::NotConnected, "no hypha"));
        };
        self.bump_cursor();
        let mut last_err = io::Error::new(io::ErrorKind::NotConnected, "no hypha");
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
        let out = self.exchange_inner(addr, act_body);
        // A full reflex pool answers every act with the deferral shape (the
        // pending outcome and four zero lanes): the judge's measured
        // impedance, which the aperture reads as resistance (23.8, S13.4).
        let deferred = matches!(&out, Ok(b)
            if dialogue::is_deferral(b, crate::cognition::outcome_code(&crate::cognition::Outcome::GermPending)));
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
                    // EVERY hypha's Z decays on a success anywhere, which
                    // is the claim this reading makes: the resistance
                    // carrier holds one entry per address dialled.
                    let all: Vec<Vec<u8>> = r.scoped().map(|(k, _)| k).collect();
                    for k in all {
                        if let Some(z) = r.get_mut(&k) {
                            // A hypha at no resistance decays to none: the
                            // reading of absence, not a floor.
                            if let Some(next) = z.checked_sub(1) {
                                *z = next;
                            }
                        }
                    }
                    r.insert(addr.as_bytes(), 0);
                }
                _ => {
                    // A peer with no entry in this carrier has met no
                    // resistance: the reading of absence, zero by the
                    // carrier's own definition and not a floor.
                    let cur = match r.get(addr.as_bytes()) {
                        Some(z) => *z,
                        None => 0,
                    };
                    r.insert(addr.as_bytes(), cur + 1);
                }
            }
        }
        if deferred {
            // 23.8 (S13.4): the deferral carries the door's deficit; the
            // refusal carries it up so the arm dwells on those lanes.
            let code = crate::cognition::outcome_code(&crate::cognition::Outcome::GermPending);
            // A DEFERRAL THAT NAMES NO LANE IS NOT A DEFERRAL. Four zero
            // lanes tell the arm the door deferred and is short on nothing,
            // which is a deficit reading this reply never made; unmeasured
            // is what it is, and the arm is told that instead.
            let Some(lanes) = out.as_ref().ok().and_then(|b| dialogue::deferral_lanes(b, code))
            else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "deferral names no lane",
                ));
            };
            return Err(io::Error::new(io::ErrorKind::WouldBlock, Deferral { lanes }));
        }
        out
    }

    pub(crate) fn exchange_inner(&self, addr: &str, act_body: Vec<u8>) -> io::Result<Vec<u8>> {
        let frame = Frame { lane: Stratum::Spatial, body: act_body };
        // A body past the wire's one budget is refused here, by name, before
        // a byte is sent (78.125): a large publication is streamed in parts.
        let len = frame.body.len();
        // 78.152 TU-7: every act carries the last final point this session
        // read — and before it has read one, the zero point, which says only
        // "this speaker reads points": the door answers a pointed speaker with
        // its own point and an unpointed one (a kernel before 78.152) as it
        // always did, so a session that never pointed would never learn one.
        let carried = match self.final_point.get() {
            Some(p) => p,
            None => crate::chronometry::tuniversal::TPoint::default(),
        };
        let bytes = frame.encode_at(Some(&carried)).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, format!("act body {len} bytes is over the wire budget of {} bytes", crate::network::wire::BODY_BUDGET))
        })?;
        // A SEALING ACT IS NEVER SENT TWICE (78.125, the coordinator's
        // ruling): a pooled stream is retried on a fresh one only where the
        // WRITE failed — the court cannot have read the act. Once the act is
        // written, a lost reply is an error to the caller, never a resend:
        // the first may have sealed, and a resend would seal it again.
        let mut pool = self.streams.borrow_mut();
        if let Some(s) = pool.get_mut(addr.as_bytes()) {
            match s.write_all(&bytes) {
                Ok(()) => {
                    let read = self.read_reply(s);
                    if read.is_err() {
                        pool.remove(addr.as_bytes());
                    }
                    match &read {
                        Ok(_) => self.rtt_ok(addr, 1),
                        Err(e) if e.kind() == io::ErrorKind::TimedOut || e.kind() == io::ErrorKind::WouldBlock => self.rtt_timeout(addr),
                        Err(_) => {}
                    }
                    return read;
                }
                Err(_) => {
                    pool.remove(addr.as_bytes());
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
        let body = match self.roundtrip(&mut s, &bytes) {
            Ok(b) => b,
            Err(e) => {
                self.rtt_timeout(addr);
                return Err(e);
            }
        };
        self.rtt_ok(addr, 1);
        Ok(body)
    }

    pub(crate) fn roundtrip(&self, s: &mut TcpStream, bytes: &[u8]) -> io::Result<Vec<u8>> {
        s.write_all(bytes)?;
        self.read_reply(s)
    }

    /// One reply frame's body, read whole; the door's final point it carries
    /// (78.152 TU-7) is heard and leaves the body.
    fn read_reply(&self, s: &mut TcpStream) -> io::Result<Vec<u8>> {
        let mut head = [0u8; 5];
        s.read_exact(&mut head)?;
        let Some((_, pointed)) = crate::network::wire::read_lane(head[0]) else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "a reply frame names no lane"));
        };
        let len = u32::from_le_bytes([head[1], head[2], head[3], head[4]]) as usize;
        if len > crate::network::wire::BODY_BUDGET + crate::chronometry::tuniversal::TPoint::LEN {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "frame too large"));
        }
        let mut body = vec![0u8; len];
        s.read_exact(&mut body)?;
        let Some((body, point)) = crate::network::wire::split_point(body, pointed) else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "a pointed reply is shorter than its point"));
        };
        self.hear_point(point);
        Ok(body)
    }
}
