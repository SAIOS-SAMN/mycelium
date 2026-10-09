// 45.2, 47.2 — THE EXCHANGE: the nerve this hypha speaks through, and the
// cycle in which two courts read each other.
//
//! `Mesh` is what one hypha holds about the others: the cover, the rooms,
//! the addresses, the apertures, the origin it adopted. `Exchange` is the
//! cycle: a court stroke, the intake, then one digest round-trip per peer.

use crate::admission::{admit_peer_digest, admit_peer_receipt, rebuild_own_cover, AdmissionTally};
use crate::door::{intake_corpus, relocate_corpus};
use crate::persist::persist_if_due;
use crate::syllabus::{syllabus_admit, Syllabus};
use crate::transport::{note_rtt_ok, note_rtt_timeout, tell_peer};
use mycelium::anastome::dialogue;
use mycelium::cognition::Kernel;
use mycelium::court::identity::HyphaKey;
use mycelium::network::nerve::{Cover, Keyed, Router};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};


/// THE FOUR STRATUM ROOTS a peer last showed, with the cover root over
/// them — the dialogue's own carrier for the pair, not a second shape of
/// it (law 12): this binary compares what it recorded against what a
/// signer wrote, so the two cannot be separate types.
pub(crate) use mycelium::anastome::dialogue::Roots;

/// A NODE'S ORIGIN: the genesis hash it adopted and the epoch it adopted it
/// at — the pair 47.2 step 1 tests every signed frame against.
pub(crate) type Origin = ([u8; 32], u32);

/// THE LOCK ORDER of this binary (Stage 11 I26), read from every site that
/// holds one lock while taking another. The kernel is taken first, and
/// under it, in this order and never the reverse: `cover`, then `rooms`,
/// then `addrs` (`homes_here`, the glue in the exchange and the offer
/// judge), then, those three released, `rtt_window`. The router is never taken under the kernel. No path holds a
/// mesh lock, the router, the registry, the syllabus, the roots or the
/// origin while it takes the kernel: `sign_position`, `admit_peer_receipt`
/// and the two judges read `origin` and drop it before the kernel; the
/// door drops the registry (`drop(reg)`) and the kernel before the
/// syllabus and the roots; the exchange drops the kernel (`drop(k)`)
/// before the syllabus; the transport takes `rtt_window` and `rtt` one at
/// a time. A new site that nests two of these locks keeps this order or is
/// returned.
pub(crate) struct Mesh {
    pub(crate) cover: Arc<Mutex<Cover>>,
    pub(crate) rooms: Arc<Mutex<Keyed<[u64; 4]>>>,
    pub(crate) addrs: Arc<Mutex<Keyed<String>>>,
    pub(crate) vk: [u8; 32],
    pub(crate) sk: ed25519_dalek::SigningKey,
    pub(crate) bind: String,
    pub(crate) rtt: Arc<Mutex<Keyed<u64>>>,
    /// The last aperture of measured round trips per peer, in strokes.
    pub(crate) rtt_window: Arc<Mutex<Keyed<Vec<u64>>>>,
    pub(crate) last_roots: Arc<Mutex<Keyed<Roots>>>,
    /// This court's K at the last digest ask per peer (45.7, S13.1): a peer
    /// is asked again only after its Λ elapsed on this court's own clock or
    /// this court's own roots moved. Taken one at a time, like `rtt_window`.
    pub(crate) asked_at: Arc<Mutex<Keyed<u64>>>,
    /// The exchange to each peer as a carrier (45.7, S13.3): every cycle
    /// strokes it, each ask is one circuit, and the period is this court's
    /// cycles between asks, written once per ask as `court circuit exchange`.
    /// Taken one at a time, like `rtt_window`.
    pub(crate) circuits: Arc<Mutex<Keyed<mycelium::physics::Circuits>>>,
    /// The door's per-lane reserve deficit, snapshotted from the kernel on
    /// every court stroke so the listener can say it without the kernel
    /// (23.8, S13.4): the deferral carries it and the asker dwells on it.
    /// Taken one at a time, like `rtt_window`.
    pub(crate) deficit: Arc<Mutex<[u64; 4]>>,
    /// This node's own `(H_genesis, E_origin)`, the pair 47.2 step 1 tests
    /// every signed frame against. `None` is UNMEASURED, not zero: a node
    /// that has not adopted a genesis admits no receipt at all rather than
    /// admitting everything under a zero sentinel.
    pub(crate) origin: Arc<Mutex<Option<Origin>>>,
}

impl Clone for Mesh {
    fn clone(&self) -> Self {
        Mesh {
            cover: Arc::clone(&self.cover),
            rooms: Arc::clone(&self.rooms),
            addrs: Arc::clone(&self.addrs),
            vk: self.vk,
            sk: self.sk.clone(),
            bind: self.bind.clone(),
            rtt: Arc::clone(&self.rtt),
            rtt_window: Arc::clone(&self.rtt_window),
            last_roots: Arc::clone(&self.last_roots),
            asked_at: Arc::clone(&self.asked_at),
            circuits: Arc::clone(&self.circuits),
            deficit: Arc::clone(&self.deficit),
            origin: Arc::clone(&self.origin),
        }
    }
}

/// SPEAK A ROOT ON THE NERVE: to the one peer the router places closest to
/// the family's key, other than this node and the voice it was heard from.
/// Every node walks the same key to the same next hop, so a root crosses
/// the nerve along one deterministic path and never as a broadcast.
pub(crate) enum RootHear {
    Known,
    /// The task's first solver here.
    First,
    /// The second: the root is consilient from here.
    Second,
    /// A third or later solver of a root already consilient.
    Joined,
}

/// Record a root by (task, solver) with its stamp. A solver already held
/// for this task is silence; a new solver is spoken.
pub(crate) fn note_root(roots: &mut Keyed<Keyed<u64>>, task: impl AsRef<[u8]>, solver: [u8; 32], stamp: u64) -> RootHear {
    let task = task.as_ref();
    match roots.get_mut(task) {
        Some(v) => {
            if v.contains(&solver) {
                RootHear::Known
            } else {
                v.insert(solver, stamp);
                match v.len() {
                    2 => RootHear::Second,
                    _ => RootHear::Joined,
                }
            }
        }
        None => {
            let mut v = Keyed::new();
            v.insert(solver, stamp);
            roots.insert(task, v);
            RootHear::First
        }
    }
}

pub(crate) fn speak_root(
    router: &Router,
    nerve: &std::sync::mpsc::SyncSender<(String, Vec<u8>)>,
    self_id: &[u8; 32],
    task: &[u8],
    solver: &[u8; 32],
    heard_from: Option<[u8; 32]>,
    ticks: u64,
) {
    let mut exclude = vec![*self_id];
    exclude.extend(heard_from);
    let hop = router.closest_excluding(task, &exclude);
    if let Some((_, addr)) = hop {
        // Never dialed from the door: queued for the nerve thread, dropped
        // when the nerve is full — the mesh's own impedance.
        match dialogue::encode_root(self_id, task, solver, ticks) {
            Some(body) => {
                let _ = nerve.try_send((addr, body));
            }
            None => eprintln!("  root refused: the task's address cannot be framed"),
        }
    }
}

pub(crate) fn speak_root_locked(
    router: &Mutex<Router>,
    nerve: &std::sync::mpsc::SyncSender<(String, Vec<u8>)>,
    self_id: &[u8; 32],
    task: &[u8],
    solver: &[u8; 32],
    heard_from: Option<[u8; 32]>,
    ticks: u64,
) {
    if let Ok(r) = router.lock() {
        speak_root(&r, nerve, self_id, task, solver, heard_from, ticks);
    }
}

/// THE NERVE THREAD: every act this node speaks to a peer leaves through a
/// bounded channel drained here, so the door never dials. The bound is the
/// reserve ceiling; a full nerve drops the act, as a full reflex pool
/// defers a demand.
pub(crate) fn spawn_nerve(mesh: Mesh, nerve_rx: std::sync::mpsc::Receiver<(String, Vec<u8>)>) {
    std::thread::spawn(move || {
        let mesh_nerve = mesh;
        // R2 (34.2): a peer's deferrals of the acts routed to it — demands
        // routed to a listing's home above all — are read once per
        // aperture of acts per peer, never per act: per act it was 376,000
        // lines in one aperture on the fleet of 2026-09-09.
        let mut deferred: mycelium::network::nerve::Keyed<u64> = mycelium::network::nerve::Keyed::new();
        let aperture = u64::from(mycelium::physics::aperture_cycle());
        // 78.152b: pushed rings refused by a peer, counted, said at powers of
        // two; a heard ring is the steady state and says nothing.
        let mut ticks_refused = 0u64;
        for (addr, body) in nerve_rx {
            let tick = dialogue::act_is(&body, dialogue::ACT_TICK);
            match tell_peer(&addr, body) {
                Some((reply, dt)) => {
                    note_rtt_ok(&mesh_nerve, addr.as_bytes(), dt);
                    // 23.8, I36: a full reflex pool's deferral is impedance,
                    // not a root reply that failed to decode.
                    if crate::transport::is_deferred(&reply) {
                        // A peer that has deferred nothing yet has deferred
                        // none: the reading of absence on a keyed carrier.
                        let seen = match deferred.get(addr.as_bytes()) {
                            Some(n) => *n,
                            None => 0,
                        };
                        #[expect(clippy::disallowed_methods, reason = "rule 11 — one per deferral event in one process: a u64 cannot pass 2⁶⁴ events before the process ends")]
                        let n = seen.saturating_add(1);
                        deferred.insert(addr.as_bytes(), n);
                        if n % aperture == 0 {
                            eprintln!("  court nerve deferred by {addr} acts {n}");
                        }
                        continue;
                    }
                    if tick {
                        if reply.as_slice() != [dialogue::TICK_HEARD] {
                            ticks_refused = ticks_refused.checked_add(1).map_or(ticks_refused, |n| n);
                            if ticks_refused.is_power_of_two() {
                                eprintln!("  nerve → {addr}: tick refused ({ticks_refused})");
                            }
                        }
                        continue;
                    }
                    match dialogue::decode_root_reply(&reply) {
                        Some((held, at)) => eprintln!(
                            "  nerve → {addr}: root delivered at {} ({})",
                            mycelium::chronometry::TUniversal::stamp_of(at),
                            if held.is_some() { "held there" } else { "new there" }
                        ),
                        None => eprintln!("  nerve → {addr}: answered {} bytes", reply.len()),
                    }
                }
                None => {
                    note_rtt_timeout(&mesh_nerve, addr.as_bytes());
                    // A silent keeper is read on the residual and the final
                    // point, not one line per pushed ring.
                    if !tick {
                        eprintln!("  nerve → {addr}: no answer");
                    }
                }
            }
        }
    });
}

/// The organ that runs one court against the others. Its fields are what
/// the cycle needs and nothing else; `spawn` hands the whole of it to a
/// thread, which is what the boot order used to do with eleven separate
/// clones at the call site.
pub(crate) struct Exchange {
    pub kernel: Arc<Mutex<Kernel>>,
    /// 78.151 — the shelf the court stroke publishes the door's reading to.
    pub published: crate::published::Shelf,
    pub mesh: Mesh,
    pub join: Vec<String>,
    pub gen_home: PathBuf,
    pub epoch: u32,
    pub mint_genesis: bool,
    pub registry: Arc<Mutex<mycelium::registry::TaskRegistry>>,
    pub router: Arc<Mutex<Router>>,
    pub state_path: String,
    pub syllabus: Arc<Mutex<Syllabus>>,
    pub genesis_held: bool,
    /// 78.152b — the nerve this court's keeper pushes its ring on.
    pub nerve: std::sync::mpsc::SyncSender<(String, Vec<u8>)>,
}

/// THE PEER COURTS A CYCLE SPEAKS TO: the configured peers and every door a
/// signed digest named, this court's own excepted — one entry per hypha.
fn peer_doors(mesh: &Mesh, join: &[String]) -> Keyed<String> {
    let mut dests: Keyed<String> = Keyed::new();
    for p in join {
        if p != &mesh.bind {
            dests.insert(p.as_bytes(), p.clone());
        }
    }
    if let Ok(a) = mesh.addrs.lock() {
        // The peers this court knows, which is what a cycle's
        // destinations are: one entry per hypha, bounded by the
        // mesh and not by anything that grows with use.
        for (_, addr) in a.scoped() {
            if addr != &mesh.bind {
                dests.insert(addr.as_bytes(), addr.clone());
            }
        }
    }
    dests
}

/// 78.152b — THE KEEPER'S RING PUSHED: where the court loop signed a reading,
/// its ring is queued as `ACT_TICK` to every peer court on the nerve, never
/// blocking (`try_send`: a full nerve drops it, and the next reading's ring
/// carries it again, as the digest still does). Called with no kernel held.
fn push_tick(mesh: &Mesh, join: &[String], nerve: &std::sync::mpsc::SyncSender<(String, Vec<u8>)>, ring: &[mycelium::chronometry::tuniversal::TickReading]) {
    let Some(body) = dialogue::encode_tick(&mesh.vk, ring) else {
        eprintln!("  court tuniversal tick refused: a reading cannot be written");
        return;
    };
    for (_, addr) in peer_doors(mesh, join).scoped() {
        let _ = nerve.try_send((addr.clone(), body.clone()));
    }
}

impl Exchange {
    pub(crate) fn spawn(self) {
        // THE CORPUS INTAKE IS PERCEPTION, AND THE CYCLE DOES NOT WAIT ON IT.
        // It ran inside the exchange loop: every boot re-perceives the
        // corpus's charts, and for the first minutes the one thread that
        // strokes the court, replicates and folds spent eight samples in ten
        // in `chart::invariant_of` (gdb on hypha a, 2026-09-30) — the window
        // in which a restarted court most needs to catch up. The intake takes
        // the kernel only per chain, so it runs on its own thread until the
        // registry has taken the whole corpus in, and then ends.
        {
            let (kernel, mesh, registry) = (self.kernel.clone(), self.mesh.clone(), self.registry.clone());
            std::thread::spawn(move || {
                #[expect(clippy::cast_possible_truncation, reason = "site 4 — expansion_unit, a u64 → usize narrowing on a 32-bit target; its opposite operation is asked at site 4")]
                let budget = mycelium::physics::expansion_unit() as usize;
                while registry.lock().is_ok_and(|r| !r.intake_done()) {
                    intake_corpus(&kernel, &mesh, &registry, budget);
                    std::thread::yield_now();
                }
            });
        }
        std::thread::spawn(move || self.run());
    }

    fn run(self) {
        let Exchange {
            kernel,
            published,
            mesh,
            join,
            gen_home,
            epoch,
            mint_genesis,
            registry,
            router,
            state_path,
            syllabus,
            genesis_held,
            nerve,
        } = self;
        // 78.152b: the tick the court loop last handed the keepers, at the
        // origin then held — a stroke where neither moved asks them nothing.
        let mut last_tick: (Option<mycelium::chronometry::tuniversal::Tick>, Option<Origin>) = (None, None);
        let genesis_written =
            std::sync::Arc::new(std::sync::atomic::AtomicBool::new(genesis_held));
        let mut last_organs = (usize::MAX, usize::MAX);
        // CYCLES RUN, counted. A change-gated line says nothing while the
        // reading is steady, which is right -- and says nothing at all when
        // the loop is not running, which is indistinguishable from steady.
        // The two were read as one for most of a session: an exchange that
        // had stopped and an exchange with nothing new to report wrote the
        // same empty journal. The count separates them, on powers of two, so
        // a live loop is visibly alive without becoming the noise it is here
        // to cut through.
        let mut cycles = 0u64;
        // The cycle each peer was last asked at: the quiet-digest ceiling's
        // reading, one entry per peer this court knows.
        let mut asked_cycle: mycelium::network::nerve::Keyed<u64> = mycelium::network::nerve::Keyed::new();
        // The orbits this court sealed since the ledger was last persisted.
        let mut sealed_since_persist = 0u64;
        let mut idle = 0u64;
        // 78.152: what the door's shelf had moved when this cycle last
        // parked or worked — a park waits for motion past it.
        let mut park_seen = crate::tshelf::shelf().seen();
        let mut last_tally = AdmissionTally::default();
        let mut last_cover: Option<[[u8; 32]; 4]> = None;
        loop {
            std::thread::yield_now();
            crate::held::yield_to_waiters();
            if let Ok(mut k) = crate::held::held!(kernel) {
                crate::held::step("court stroke");
                k.court_stroke();
                // A question the observer left while this stroke held the
                // kernel is answered under the lock already held.
                crate::held::step("answer standing");
                crate::judge::observe::answer_standing(&k, &mesh, mesh.vk);
                // 78.151 — the door's reading, published whole under the
                // lock the stroke already holds.
                crate::held::step("publish");
                published.publish(&mut k);
                crate::held::step("offers");
                // The words the stroke moved onto the docket — a rewrite, the
                // citers a slash re-docketed — are offered by it.
                let offers = k.take_offers();
                let seed = k.syllabus_seed();
                // 23.8, S13.4: the door's deficit, read under the kernel and
                // handed to the listener's deferral without it.
                let deficit = k.physics().reserve_deficit();
                // 78.152b TU-1: the keeper's tick — head h, lanes, K — read
                // once per stroke under the lock already held.
                let tick = k.keeper_tick();
                drop(k);
                if let Ok(mut d) = mesh.deficit.lock() {
                    *d = deficit;
                }
                // 78.152b: SIGNED IN THE COURT LOOP, the kernel dropped, so a
                // reading is signed whenever it is due — no digest ask needed
                // — and its ring pushed to every peer court on the nerve.
                // Asked again where the tick moved or an origin was adopted
                // since: a reading verifies only at an origin.
                let at = (tick, mesh.origin.lock().ok().and_then(|o| *o));
                if at.0.is_some() && at != last_tick {
                    last_tick = at;
                    if let Some(ring) = crate::tshelf::sign_tick(&mesh, tick) {
                        push_tick(&mesh, &join, &nerve, &ring);
                    }
                }
                if let Ok(mut sy) = syllabus.lock() {
                    syllabus_admit(&mut sy, seed);
                }
                crate::offer::offer(&kernel, &mesh, offers);
            }
            // THE LEDGER IS PERSISTED AT THE FEDERATION FLUSH, not on every
            // marked change: hyphae b and c wrote their whole 27–33 MB
            // ledger 223 and 347 times in six minutes, one per witnessed
            // fold, each snapshot taken under the kernel (2026-09-30). A
            // marked change waits an aperture of this court's own sealed
            // orbits — its clock, never a count of loop cycles (PH1); a stop
            // persists whatever is still marked (SIGTERM).
            if sealed_since_persist >= u64::from(mycelium::physics::aperture_cycle()) && persist_if_due(&kernel, &state_path) {
                sealed_since_persist = 0;
                // 78.67 — THE COURT'S METABOLISM AND ITS FOLDS, written at the
                // persist, its own pace: the reserve off its own books (every
                // thread that drew on it), the eras and hot nodes it holds,
                // and the fold's round by outcome. None of this reached the
                // journal, so epoch 117's fold storm and its reserve were
                // read only by hand.
                if let Ok(k) = crate::held::held!(kernel) {
                    let folds = mycelium::observe::folds();
                    // 78.68: the winding carrier — first closes, carried
                    // traversals, returns sealed, unbound closes — and the
                    // rank steps the charts' holonomy algebras took.
                    let w = k.winding_tally();
                    // 78.71: the court's field — its tubes, net and transport
                    // apart, the flux left open, helicity, reconnections.
                    let (tubes, net, transport, open, helicity) = k.tube_reading();
                    // 78.74: the nerve's elastic vent — cells released as
                    // words left — and the seals the ledger now books.
                    let vent = k.released_cells();
                    // 78.75: the nerve by degree — vertices, edges, faces,
                    // volumes — and how much of each carries content.
                    let nerve: Vec<String> = k
                        .nerve_census()
                        .iter()
                        .enumerate()
                        .map(|(d, c)| {
                            format!(
                                "d{d} {} (open {} filled {} congruent {} class {} unspellable {} unread {} | flat {} turn {} boost {} null {} 180° {} modes unread {})",
                                c.cells,
                                c.open,
                                c.filled,
                                c.congruent,
                                c.class,
                                c.unspellable,
                                c.unread,
                                c.flat,
                                c.turn,
                                c.boost,
                                c.null,
                                c.half,
                                c.modes_unread
                            )
                        })
                        .collect();
                    eprintln!("  court nerve {} | complexes {}", nerve.join(" | "), k.complex_count());
                    // 78.100 — where this court's letter offer breaks.
                    eprintln!(
                        "  court shortcuts offered {} spelled {} peel unread {} no discoverer {} rotor unread {} | heard: offered {} spelled {} peel unread {} rotor unread {} spanned {} rings unread {} | heard sentences {} rings {} taught nothing {}",
                        w.shortcuts_offered,
                        w.shortcuts_spelled,
                        w.shortcuts_peel_unread,
                        w.shortcuts_no_discoverer,
                        w.shortcuts_rotor_unread,
                        w.heard_offered,
                        w.heard_spelled,
                        w.heard_peel_unread,
                        w.heard_rotor_unread,
                        w.heard_spanned,
                        w.heard_ring_unread,
                        w.heard_sentences,
                        w.heard_rings,
                        w.heard_taught_nothing
                    );
                    // 78.100 — why this court's climb stops, by why.
                    let climb = mycelium::transducer::cech::construction_tally();
                    eprintln!(
                        "  court climb {}",
                        mycelium::transducer::cech::CONSTRUCTION
                            .iter()
                            .zip(climb)
                            .filter(|(name, _)| name.starts_with("climb"))
                            .map(|(name, n)| format!("{} {n}", &name[7..]))
                            .collect::<Vec<_>>()
                            .join(" · ")
                    );
                    eprintln!(
                        "  court metabolism K {} | {} | syntropy {:?} pressure {:?} | {} | eras {} hot {} | rings first {} carried {} returns {} unbound {} repriced {} rank steps {} (by outflow {}) held {} ringless {} (acyclic {} vacuous {} circuits unread {}) formless {} unread {} | sentences {} theorems checked {} refused {} shortcuts offered {} nerve closed {} | outflow {} | tubes {tubes} net {net} transport {transport} open {open} helicity {helicity} flux {} reconnections {} | vent released {} unsealed {} vacuum {} cauterized {} pinned {} | sealed {} | multiplicity condensed {} | folds {}",
                        k.k_step(),
                        k.physics().reserve.line(),
                        k.physics().syntropic,
                        k.physics().pressure,
                        k.malleability_line(),
                        k.archive().len(),
                        k.chain().len(),
                        w.first,
                        w.carried,
                        w.returns,
                        w.unbound,
                        w.repriced,
                        w.rank_steps,
                        w.outflow_rank_steps,
                        w.rank_held,
                        w.rank_ringless,
                        w.rank_acyclic,
                        w.rank_vacuous,
                        w.rank_circuits_unread,
                        w.rank_formless,
                        w.rank_unread,
                        w.sentences,
                        w.theorems_checked,
                        w.theorems_refused,
                        w.shortcuts_offered,
                        w.nerve_closed,
                        w.outflow,
                        w.flux_transport,
                        w.reconnections,
                        vent.released,
                        vent.unsealed,
                        vent.vacuum,
                        vent.cauterized,
                        vent.pinned,
                        k.cells_sealed(),
                        k.multiplicity_condensed(),
                        mycelium::observe::FOLD_OUTCOMES
                            .iter()
                            .zip(folds)
                            .map(|(axis, n)| format!("{axis} {n}"))
                            .collect::<Vec<_>>()
                            .join(" ")
                    );
                }
            }
            #[expect(clippy::cast_possible_truncation, reason = "site 4 — expansion_unit, a u64 → usize narrowing on a 32-bit target; its opposite operation is asked at site 4")]
            let budget = mycelium::physics::expansion_unit() as usize;
            rebuild_own_cover(&kernel, &mesh);
            let roots = mesh.cover.lock().ok().map(|mut c| c.roots());
            // 45.7, S13.1: this court's own roots moving is one of the two
            // readings that ask a peer again; the other is the peer's Λ.
            let own_moved = roots != last_cover;
            if own_moved {
                relocate_corpus(&kernel, &mesh, &registry, budget);
                last_cover = roots;
            }
            let k_now = match crate::held::held!(kernel) {
                Ok(k) => k.k_step(),
                Err(_) => continue,
            };
            let dests = peer_doors(&mesh, &join);
            cycles += 1;
            if cycles.is_power_of_two() {
                eprintln!("  court exchange cycle {cycles} idle {idle}");
            }
            // A COURT WITH NO PEER STILL CLOSES ITS ORBIT. With no
            // destination the peers' loop below asks nobody and the cycle
            // parks as idle at its foot (the stroke-duration ruling); it
            // `continue`d here instead, past the genesis, the fold and
            // `orbit()` — so a lone court told to mint never wrote its
            // origin and its `K` never moved while it served its voices
            // (the live wire, 2026-09-29: 524 288 cycles, genesis unwritten,
            // `K=0` on every quote).
            let req = dialogue::encode_digests_request(&mesh.vk);
            let mut any_moved = false;
            let dest_list: Vec<(Vec<u8>, String)> = dests
                .scoped()
                .map(|(id, a)| (id, a.clone()))
                .collect();
            #[expect(clippy::cast_possible_truncation, reason = "site 4 — expansion_unit, a u64 → usize narrowing on a 32-bit target; its opposite operation is asked at site 4")]
            let mut organ_budget = mycelium::physics::expansion_unit() as usize;
            let (mut reached, mut silent) = (0usize, 0usize);
            let mut asked_any = false;
            let mut tally = AdmissionTally::default();
            let _ = &last_organs;
            for (pid, d) in dest_list {
                if organ_budget == 0 {
                    break;
                }
                // 45.7, S13.1 — THE EXCHANGE IS PACED PER PEER. A peer is
                // asked for its digest again only when its Λ, read from the
                // round trips this court measured to it, has elapsed since
                // the last ask on this court's own K, or this court's own
                // roots moved. Asked on every circuit, one hypha ran 131,072
                // circuits and 100,000 deferrals in one reading (2026-09-09):
                // the poll filled the peers' reflex pools and nothing was
                // admitted. A peer silent since boot has no Λ and is asked
                // again when these roots move.
                // The exchange is due against a horizon the ledger counted:
                // a peer with no measured round trip has none, and one
                // beyond the ledger means no ask has waited that long.
                let lambda = match crate::transport::rtt_floor(&mesh, &pid) {
                    None => None,
                    Some(floor) => {
                        match mycelium::physics::from_transport(
                            floor,
                            mycelium::physics::aperture_cycle() as u64,
                        ) {
                            mycelium::physics::Lambda::Strokes(n) => Some(n),
                            mycelium::physics::Lambda::BeyondTheLedger
                            | mycelium::physics::Lambda::Unmeasured => None,
                        }
                    }
                };
                let last = mesh.asked_at.lock().ok().and_then(|a| a.get(&pid).copied());
                // A QUIET COURT STILL ASKS: Λ is read on this court's K, which
                // moves only when the court acts, so hypha a — spinning 340
                // idle cycles a second with its K standing — asked each peer
                // once in six minutes (2026-09-30). The quiet-digest ceiling
                // bounds the wait in this loop's own strokes: a full reserve
                // of the peer's measured transports.
                let quiet = {
                    // An unmeasured wire is read at the instrument's own
                    // resolution: one poll, as a first-poll reply measures.
                    let ceiling = match mycelium::physics::digest_ceiling(crate::transport::rtt_floor(&mesh, &pid).map_or(1, |f| f)) {
                        mycelium::physics::Lambda::Strokes(n) => Some(n),
                        mycelium::physics::Lambda::BeyondTheLedger | mycelium::physics::Lambda::Unmeasured => None,
                    };
                    let since = asked_cycle.get(&pid).copied();
                    match (since, ceiling) {
                        (Some(at), Some(c)) => cycles.checked_sub(at).is_some_and(|gap| gap >= c),
                        (None, _) => true,
                        (Some(_), None) => false,
                    }
                };
                let due = crate::transport::exchange_due(last, k_now, lambda, own_moved) || quiet;
                // 45.7, S13.3: the exchange to one peer is a carrier whose
                // circuit is one ask; every cycle strokes it, so its period
                // is read in this court's cycles between asks — velocity in
                // kernel coordinates, beside K on the ask line.
                let circuit = mesh.circuits.lock().ok().and_then(|mut o| {
                    if o.get(&pid).is_none() {
                        o.insert(&pid, mycelium::physics::Circuits::default());
                    }
                    o.get_mut(&pid).and_then(|c| c.stroke(u64::from(due)))
                });
                if !due {
                    continue;
                }
                if let Ok(mut a) = mesh.asked_at.lock() {
                    a.insert(&pid, k_now);
                }
                asked_cycle.insert(&pid, cycles);
                asked_any = true;
                eprintln!("  court exchange asked {d} K {k_now}");
                if let Some((n, period)) = circuit {
                    eprintln!("  court circuit exchange {d} {n} period {period}");
                }
                organ_budget -= 1;
                match tell_peer(&d, req.clone()) {
                    Some((reply, dt)) => {
                        reached += 1;
                        note_rtt_ok(&mesh, &pid, dt);
                        // 23.8, I36: a full reflex pool's deferral is
                        // impedance, not a digest that failed admission.
                        if crate::transport::is_deferred(&reply) {
                            eprintln!("  court exchange deferred by {d}");
                            continue;
                        }
                        // The digest is admitted BEFORE its receipt section
                        // is read, and its refusal is tallied: a `continue`
                        // here used to leave the exchange line saying
                        // nothing at all, so three courts read zero
                        // admitted receipts with no way to tell a refused
                        // digest from a path that never ran (rule 11).
                        let digest = admit_peer_digest(&mesh, &reply, &router);
                        tally.note_digest(digest);
                        if digest.is_err() {
                            continue;
                        }
                        // 47.2 step 3, S13.10a, R4, 45.2: a gap fetches pages
                        // of the missing receipts from the signer that owns
                        // them until the chain reaches the reported K, within
                        // this exchange's stroke budget, and the reporting
                        // frame is then admitted again — the one citation.
                        // The chase and its readings (ACT1-hold-b) are the
                        // lib's; the wire is `backfill::fetch`.
                        // Each page spends one stroke; a stroke is taken
                        // again only while the lag the chase closes shrinks.
                        // 78.147: a fold root answered for a catch-up is
                        // verified at this node's origin.
                        let origin = mesh.origin.lock().ok().and_then(|o| *o);
                        let admitted = mycelium::server::backfill::hear(
                            &kernel,
                            origin,
                            || admit_peer_receipt(&kernel, &mesh, &reply),
                            || crate::held::held!(kernel).ok().map(|mut k| k.stroke()),
                            crate::backfill::fetch(&mesh, &d),
                        );
                        // The signer is the key the digest names; a digest
                        // that admitted but does not decode names none.
                        let signer = dialogue::decode_cover_digest(&reply).map(|c| c.vk);
                        tally.note(signer, admitted);
                        let Some(dialogue::CoverDigest {
                            cover_roots, ruled_root, vk: vk_peer, ..
                        }) = dialogue::decode_cover_digest(&reply)
                        else {
                            continue;
                        };
                        let now = (cover_roots, ruled_root);
                        let unchanged = mesh
                            .last_roots
                            .lock()
                            .ok()
                            .and_then(|g| g.get(&vk_peer).copied())
                            == Some(now);
                        if let Ok(mut g) = mesh.last_roots.lock() {
                            g.insert(vk_peer, now);
                        }
                        // A reply that does not decode names no strata,
                        // and nothing is discordant against it.
                        let Some(remote) = dialogue::decode_digests_reply(&reply) else {
                            continue;
                        };
                        let charts = {
                            let mut k = match crate::held::held!(kernel) {
                                Ok(g) => g,
                                Err(_) => continue,
                            };
                            k.confirm_peer_holds(&remote, vk_peer);
                            // AGREEMENT IS ON INSTALLED WORDS (ruled 2026-09-30,
                            // the operator's word). The peer answers a court's
                            // digests request with its language atlas — the
                            // words it installed — and this compared them with
                            // its own RULED cover, pending and attached words
                            // included, which no fetch can ever carry: one
                            // stratum stood discordant between every pair for
                            // good. Both sides are now the installed language.
                            let local = mycelium::transducer::sheaf::digests(&k.language().stalks);
                            let discord = mycelium::transducer::sheaf::discordant(&remote, &local);
                            // PHASE 1 ITEM 1 — AGREEMENT WITH THIS PEER, read
                            // at its digest: the strata it named, and how many
                            // of them differ from this court's ruled sections.
                            mycelium::observe::set_replication(vk_peer, mycelium::observe::rep::STRATA, remote.len() as u64);
                            mycelium::observe::set_replication(vk_peer, mycelium::observe::rep::DISCORDANT, discord.len() as u64);
                            // Which stratum, and each side's count (Phase 1
                            // diagnostic, on the journal at powers of two).
                            {
                                static SEEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
                                for s in &discord {
                                    let n = SEEN.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                                    if n.is_power_of_two() {
                                        let here = local.iter().find(|l| l.inv.key() == s.inv.key()).map(|l| l.count);
                                        eprintln!(
                                            "  discordant ({n}) with {:02x}{:02x}…: stratum {:02x?}… theirs {} ours {here:?}",
                                            vk_peer[0],
                                            vk_peer[1],
                                            s.inv.key().iter().take(6).collect::<Vec<_>>(),
                                            s.count
                                        );
                                    }
                                }
                            }
                            if discord.is_empty() {
                                local
                                    .iter()
                                    .map(|s| dialogue::ChartOf {
                                        inv: s.inv.clone(),
                                        have: k.fetch_have(&s.inv),
                                    })
                                    .collect::<Vec<_>>()
                            } else {
                                discord
                                    .iter()
                                    .map(|s| dialogue::ChartOf {
                                        inv: s.inv.clone(),
                                        have: k.fetch_have(&s.inv),
                                    })
                                    .collect::<Vec<_>>()
                            }
                        };
                        if !unchanged {
                            any_moved = true;
                            // A4, owed since Stage 8c: THE COVER IS FETCHED
                            // WHEN THE ROOTS MOVE. The digest carries the
                            // four roots and no charts, so this ask is the
                            // only way a peer's cover arrives -- read from
                            // the last cursor held, page after page, while
                            // the peer says more stands and this organ's
                            // budget affords the ask.
                            let mut from: Vec<u8> = Vec::new();
                            while organ_budget > 0 {
                                organ_budget -= 1;
                                // A cursor past its four bytes asks
                                // nothing (STD1 3c).
                                let Some(ask) = dialogue::encode_cover_request(&mesh.vk, &from) else {
                                    eprintln!("  cover ask refused: the cursor passes its length field");
                                    break;
                                };
                                let Some((page, dt3)) = tell_peer(&d, ask) else {
                                    note_rtt_timeout(&mesh, &pid);
                                    break;
                                };
                                note_rtt_ok(&mesh, &pid, dt3);
                                if crate::transport::is_deferred(&page) {
                                    eprintln!("  court exchange deferred by {d}");
                                    break;
                                }
                                let Some((cover, more)) = dialogue::decode_cover_page(&page) else {
                                    break;
                                };
                                // The page is the peer's own statement of
                                // its charts under the roots it just
                                // signed, so the first page retracts what
                                // this court held for it and the rest add.
                                let Ok(mut c) = mesh.cover.lock() else { break };
                                if from.is_empty() {
                                    c.retract_hypha(&cover.vk);
                                }
                                let read = cover.charts.len();
                                for (lane, inv) in &cover.charts {
                                    c.admit(*lane, inv, cover.vk);
                                    from = dialogue::cover_cursor(*lane, inv);
                                }
                                drop(c);
                                // A PEER ARRIVING ON A CHART is a transition for
                                // every word in flight under it: each is carried
                                // to the peer that now covers it, and the
                                // carriage draws on its escrow (47.3, 78.42).
                                let offers = match crate::held::held!(kernel) {
                                    Ok(mut k) => {
                                        for (_, inv) in &cover.charts {
                                            k.carry_chart(inv);
                                        }
                                        k.take_offers()
                                    }
                                    Err(_) => Vec::new(),
                                };
                                crate::offer::offer(&kernel, &mesh, offers);
                                eprintln!("  court cover read {read} charts from {d} more {more}");
                                if !more || read == 0 {
                                    break;
                                }
                            }
                        }
                        if charts.is_empty() {
                            continue;
                        }
                        let fetch = dialogue::encode_fetch(
                            &mesh.vk,
                            &charts,
                            mycelium::network::wire::frame_budget(),
                        );
                        match tell_peer(&d, fetch) {
                            Some((page, dt2)) => {
                                note_rtt_ok(&mesh, &pid, dt2);
                                if crate::transport::is_deferred(&page) {
                                    eprintln!("  court exchange deferred by {d}");
                                    continue;
                                }
                                let mut named: Vec<mycelium::transducer::address::Address> = Vec::new();
                                // A page that does not decode names nothing.
                                for t in dialogue::decode_syllabus_reply(&page).into_iter().flatten() {
                                    let mut p = 0usize;
                                    if let Some(spoken) = mycelium::transducer::element::Element::from_bytes(&t.word, &mut p)
                                    {
                                        let addr = spoken.address();
                                        named.push(addr.clone());
                                        if let Ok(mut k) = crate::held::held!(kernel) {
                                            // 78.24 (2b): a word learned from a peer is
                                            // located by the stamp that admitted it there,
                                            // the peer's key its coordinate; every act
                                            // here cites that stamp.
                                            // A slash withdraws only the stamp
                                            // this filing placed, never this
                                            // court's own on a word it holds.
                                            // THE ACT'S ROOT (65.5): one fetched
                                            // word glued is one act, and its one
                                            // stroke is the filing's and the
                                            // glue's draw, threaded to both.
                                            let mut stroke = k.stroke();
                                            let filed = k.file_admitted(&addr, t.at, &mut stroke);
                                            let g = match mesh.cover.lock() {
                                                Ok(mut c) => k.glue_fetched(spoken, vk_peer, mesh.vk, &mut c, &mut stroke),
                                                Err(_) if filed => {
                                                    k.drop_admitted(&addr);
                                                    continue;
                                                }
                                                Err(_) => continue,
                                            };
                                            if filed && matches!(g, mycelium::cognition::Glue::Slashed(_)) {
                                                k.drop_admitted(&addr);
                                            }
                                            match g {
                                                mycelium::cognition::Glue::Admitted => {
                                                    eprintln!("  court glued admitted from {d}");
                                                }
                                                mycelium::cognition::Glue::Attached => {
                                                    eprintln!("  court glued attached from {d}");
                                                }
                                                mycelium::cognition::Glue::Dialect => {
                                                    eprintln!("  court glued dialect from {d}");
                                                }
                                                mycelium::cognition::Glue::Slashed(r) => {
                                                    eprintln!("  court glued slash from {d}: {r}");
                                                }
                                                // Held already from this peer: nothing
                                                // written, nothing said (78.71).
                                                mycelium::cognition::Glue::Held => {}
                                            }
                                        }
                                    }
                                }
                                if !named.is_empty() {
                                    if let Ok(mut k) = crate::held::held!(kernel) {
                                        k.confirm_peer_holds_addrs(&named, vk_peer);
                                    }
                                }
                            }
                            None => note_rtt_timeout(&mesh, pid),
                        }
                    }
                    None => {
                        silent += 1;
                        note_rtt_timeout(&mesh, pid)
                    }
                }
            }
            // 34.2's missing trace. The second organ arrives only through
            // this exchange, so a fleet where it never lands looks exactly
            // like a fleet where the germ gate is wrong -- 379 `germ
            // pending (one voice)` against `basis=0` reads as a quorum
            // fault and is a reachability fault. Silence here is the
            // difference and nothing said it. Written once per cycle,
            // not per peer, and only when something changed or nothing
            // was reached, so an exchange that is working is quiet.
            // C6. H_genesis is the chain hash of the mesh's FIRST
            // RECEIPT (45.5). It cannot be read at boot: a fresh kernel
            // has an empty chain, which is why the adoption I put there
            // never fired and every node still printed `genesis
            // unwritten`. It is adopted here, the first cycle after a
            // receipt exists, and written once.
            if !genesis_written.load(std::sync::atomic::Ordering::Relaxed) {
                // Adopted from a peer above, or minted here when this
                // node is the one told to mint. Either way it is written
                // to the mesh record once, so it survives a restart and
                // is what 47.2 step 1 tests against from then on.
                let held = mesh.origin.lock().ok().and_then(|o| *o);
                let mine = if held.is_some() {
                    held
                } else if mint_genesis {
                    kernel
                        .lock()
                        .ok()
                        .and_then(|k| k.chain().opening().map(|r| r.chain_hash))
                        .map(|h| (h, epoch))
                } else {
                    None
                };
                'genesis: {
                    let Some((g, e)) = mine else { break 'genesis };
                    // A home whose path is not UTF-8 names no key file, and
                    // minting under the bare name would put this hypha's key
                    // in the working directory — a different key from the
                    // one that home holds (rule 11).
                    let key_path = gen_home.join("hypha.key");
                    let Some(key_name) = key_path.to_str() else {
                        eprintln!("  court genesis refused: home path is not UTF-8");
                        break 'genesis;
                    };
                    let key = HyphaKey::load_or_mint(key_name);
                    let rec = mycelium::network::mesh::load_or_migrate_hypha(&gen_home, &key);
                    let mut rec = match rec {
                        Ok(rec) => rec,
                        Err(why) => {
                            eprintln!("  court genesis refused: {why}");
                            break 'genesis;
                        }
                    };
                    if rec.genesis != Some(g) {
                        rec.genesis = Some(g);
                        rec.epoch = e;
                        let _ = rec.save(&gen_home, &key);
                        eprintln!("  court genesis {:02x}{:02x}… epoch {e} written", g[0], g[1]);
                    }
                    if let Ok(mut o) = mesh.origin.lock() {
                        if o.is_none() {
                            *o = Some((g, e));
                        }
                    }
                    genesis_written.store(true, std::sync::atomic::Ordering::Relaxed);
                }
            }
            // 47.3, S13.10: a fold proposed on the docket circuit is carried
            // to a witness here and sealed on its co-signature.
            crate::fold::fold_round(&kernel, &mesh);
            // OK1: the court's cycle — exchange, deliberation, lapse,
            // persist — closes its orbit by name. `K` moves here and nowhere
            // else; a cycle that sealed no act closes nothing. A withheld
            // close is counted on the clock, never printed beside it.
            if let Ok(mut k) = crate::held::held!(kernel) {
                if matches!(k.orbit(), mycelium::chronometry::Close::Sealed(_)) {
                    sealed_since_persist += 1;
                }
            }
            // 45.7, S13.1: a circuit that asked no peer spends the tick, as
            // the circuit with no destination does, so the loop paces on the
            // court's own stroke; `idle` counts those circuits on the cycle
            // line.
            if !asked_any {
                // The same tick, and the same ruling: no peer was due, so
                // this cycle did nothing and must cost the processor
                // something rather than a decrement.
                // 78.152: THE PARK ENDS ON THE FIRST MOTION — an advance of
                // the final point, or an act read at the door — and is
                // bounded by the one scheduler floor (`stroke_floor`), told
                // to the scheduler and read by no one.
                idle += 1;
                let shelf = crate::tshelf::shelf();
                let _ = shelf.park(park_seen, mycelium::physics::stroke_floor());
            }
            park_seen = crate::tshelf::shelf().seen();
            // Written when the READING CHANGES, not every cycle. As
            // first shipped this fired on any silence and logged 17,588
            // lines in 35 minutes on one hypha: it found the deadlock
            // and then became the noise it was added to cut through.
            if (reached, silent) != last_organs {
                last_organs = (reached, silent);
                eprintln!(
                    "  court organs reached {reached} silent {silent} moved {any_moved}"
                );
            }
            // 34.2: what the exchange ADMITTED, beside what it reached.
            // Change-gated like the organ line -- a cycle that admits
            // the same nothing twice says it once.
            if tally != last_tally {
                last_tally = tally;
                eprintln!("{}", tally.line());
            }
        }
    }
}
