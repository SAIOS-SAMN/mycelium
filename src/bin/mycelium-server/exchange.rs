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


/// THE FOUR STRATUM ROOTS a peer last showed, with the cover root over them.
pub(crate) type Roots = ([[u8; 32]; 4], [u8; 32]);

/// A NODE'S ORIGIN: the genesis hash it adopted and the epoch it adopted it
/// at — the pair 47.2 step 1 tests every signed frame against.
pub(crate) type Origin = ([u8; 32], u32);

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
            origin: Arc::clone(&self.origin),
        }
    }
}

/// THE JUDGE on a reflex worker. The kernel lock is held only for the reflex
/// opening and the commit (synthesis::adjudicate / attest); the reduction
/// runs over the language snapshot with no lock held. The reply carries the
/// outcome, the per-lane metabolism, what was taught, and the settle
/// receipt; a percept still awaiting consilience routes one receipt to its
/// rendezvous.
/// SPEAK A ROOT ON THE NERVE: to the one peer the router places closest to
/// the family's key, other than this node and the voice it was heard from.
/// Every node walks the same key to the same next hop, so a root crosses
/// the nerve along one deterministic path and never as a broadcast.
pub(crate) enum RootHear {
    Known,
    First,
    Second,
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
                if v.len() == 2 {
                    RootHear::Second
                } else {
                    RootHear::First
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
        let _ = nerve.try_send((addr, dialogue::encode_root(self_id, task, solver, ticks)));
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
        for (addr, body) in nerve_rx {
            match tell_peer(&addr, body) {
                Some((reply, dt)) => {
                    note_rtt_ok(&mesh_nerve, addr.as_bytes(), dt);
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
                    eprintln!("  nerve → {addr}: no answer");
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
}

impl Exchange {
    pub(crate) fn spawn(self) {
        std::thread::spawn(move || self.run());
    }

    fn run(self) {
        let Exchange {
            kernel,
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
        } = self;
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
        let mut idle = 0u64;
        let mut last_tally = AdmissionTally::default();
        let mut last_cover: Option<[[u8; 32]; 4]> = None;
        loop {
            std::thread::yield_now();
            if let Ok(mut k) = kernel.lock() {
                k.court_stroke();
                let seed = k.syllabus_seed();
                drop(k);
                if let Ok(mut sy) = syllabus.lock() {
                    syllabus_admit(&mut sy, seed);
                }
            }
            persist_if_due(&kernel, &state_path);
            let budget = mycelium::physics::expansion_unit() as usize;
            intake_corpus(&kernel, &mesh, &registry, budget);
            rebuild_own_cover(&kernel, &mesh);
            let roots = mesh.cover.lock().ok().map(|c| c.roots());
            if roots != last_cover {
                relocate_corpus(&kernel, &mesh, &registry, budget);
                last_cover = roots;
            }
            let mut dests: mycelium::network::nerve::Keyed<String> = mycelium::network::nerve::Keyed::new();
            for p in &join {
                if p != &mesh.bind {
                    dests.insert(p.as_bytes(), p.clone());
                }
            }
            if let Ok(a) = mesh.addrs.lock() {
                for (_, addr) in a.fiber(mycelium::network::nerve::ROOT) {
                    if addr != &mesh.bind {
                        dests.insert(addr.as_bytes(), addr.clone());
                    }
                }
            }
            cycles += 1;
            if cycles.is_power_of_two() {
                eprintln!("  court exchange cycle {cycles} idle {idle}");
            }
            if dests.is_empty() {
                // NOT A SLEEP AND NOT A SPIN: with no destination there
                // is nothing to exchange, and `yield_now` alone burned a
                // core doing the whole preamble again. The stroke is the
                // pace (law 9), so the loop pays one before retrying.
                idle += 1;
                let mut b = mycelium::physics::Physics::default().stroke();
                while b.spend() {}
                continue;
            }
            let req = dialogue::encode_digests_request(&mesh.vk);
            let mut any_moved = false;
            let dest_list: Vec<(Vec<u8>, String)> = dests
                .fiber(mycelium::network::nerve::ROOT)
                .map(|(id, a)| (id, a.clone()))
                .collect();
            let mut organ_budget = mycelium::physics::expansion_unit() as usize;
            let (mut reached, mut silent) = (0usize, 0usize);
            let mut tally = AdmissionTally::default();
            let _ = &last_organs;
            for (pid, d) in dest_list {
                if organ_budget == 0 {
                    break;
                }
                organ_budget -= 1;
                match tell_peer(&d, req.clone()) {
                    Some((reply, dt)) => {
                        reached += 1;
                        note_rtt_ok(&mesh, &pid, dt);
                        if !admit_peer_digest(&mesh, &reply, &router) {
                            continue;
                        }
                        tally.note(admit_peer_receipt(&kernel, &mesh, &reply));
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
                        let remote = dialogue::decode_digests_reply(&reply).unwrap_or_default();
                        let charts = {
                            let mut k = match kernel.lock() {
                                Ok(g) => g,
                                Err(_) => continue,
                            };
                            k.confirm_peer_holds(&remote, vk_peer);
                            let local = k.ruled_digests();
                            let discord = mycelium::transducer::sheaf::discordant(&remote, &local);
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
                                let mut named: Vec<mycelium::transducer::address::Address> = Vec::new();
                                for w in dialogue::decode_syllabus_reply(&page).unwrap_or_default() {
                                    let mut p = 0usize;
                                    if let Some(spoken) = mycelium::transducer::element::Element::from_bytes(&w, &mut p)
                                    {
                                        named.push(spoken.address());
                                        if let Ok(mut k) = kernel.lock() {
                                            let g = match mesh.cover.lock() {
                                                Ok(mut c) => k.glue_fetched(spoken, vk_peer, mesh.vk, &mut c),
                                                Err(_) => continue,
                                            };
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
                                            }
                                        }
                                    }
                                }
                                if !named.is_empty() {
                                    if let Ok(mut k) = kernel.lock() {
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
                        .and_then(|k| k.chain().receipts().first().map(|r| r.chain_hash))
                        .map(|h| (h, epoch))
                } else {
                    None
                };
                if let Some((g, e)) = mine {
                    let key = HyphaKey::load_or_mint(
                        gen_home.join("hypha.key").to_str().unwrap_or("hypha.key"),
                    );
                    let mut rec = mycelium::network::mesh::load_or_migrate_hypha(&gen_home, &key);
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
            // OFFER what this court holds at first vantage. Without
            // this the exchange has nothing to move: a ruled root only
            // moves once a peer has glued a provisional, and a peer only
            // sees a provisional when it is offered. `moved false` on
            // every cycle was that deadlock.
            let offers = {
                let mut b = mycelium::physics::Physics::default().stroke();
                match kernel.lock() {
                    Ok(k) => k.offers_pending(&mut b),
                    Err(_) => Vec::new(),
                }
            };
            let mut offered_out = 0usize;
            for (oaddr, elem) in offers {
                for (pid, d) in dests.fiber(mycelium::network::nerve::ROOT) {
                    let body = dialogue::encode_offer(&dialogue::Offer {
                        addr: oaddr.clone(),
                        home: mesh.vk,
                        elem: elem.clone(),
                        voices: vec![mesh.vk],
                        deflation: [0; 4],
                        k: 0,
                        sig: [0u8; 64],
                        install_sig: [0u8; 64],
                    });
                    let Some((reply, _)) = tell_peer(d, body) else {
                        continue;
                    };
                    offered_out += 1;
                    // 47.3's SECOND VANTAGE. The peer replayed the offer
                    // on its own tapes and sent back the form it reached;
                    // that is a re-derivation, and the home converges on
                    // it by `same_vertex` (M1), never by comparing the
                    // bytes it happens to be spelled with. A reply with no
                    // form carries no vantage: the witness is UNMEASURED
                    // and the object keeps waiting, which is not the same
                    // as a witness that disagreed.
                    //
                    // THE VANTAGE IS THE KEY THE REPLY NAMES. This read the
                    // 32 bytes of the destination's BIND STRING as a
                    // verifying key: on any deployment whose addresses are
                    // not exactly 32 characters the `try_from` failed and
                    // every second vantage was dropped on the floor, and
                    // where it did not fail the witness was recorded under
                    // an ASCII address that no bond and no room answers to.
                    // Either way `converge_witness` never found an
                    // independent second organ and every germ stayed
                    // pending on one voice. An address is where a hypha
                    // answers; a vantage is a key.
                    let _ = &pid;
                    let Some(w) = dialogue::decode_offer_reply(&reply) else {
                        continue;
                    };
                    if !w.glued || w.reached.is_empty() || w.vk == mesh.vk {
                        continue;
                    }
                    let mut p = 0usize;
                    let Some(theirs) =
                        mycelium::transducer::element::Element::from_bytes(&w.reached, &mut p)
                    else {
                        continue;
                    };
                    if let Ok(mut k) = kernel.lock() {
                        k.witness_rederived(&oaddr, w.vk, theirs);
                    }
                }
            }
            if offered_out > 0 {
                eprintln!("  court offered {offered_out} germs to the overlap");
            }
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
