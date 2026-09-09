// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 23.8, 26 — THE HYPHA, a court on the mesh: the arguments it is started
//! with, the order its organs come up in, and the accept loop that hands
//! each act to the judge.

mod admission;
mod door;
mod exchange;
mod genesis;
mod judge;
mod offer;
mod persist;
mod syllabus;
mod transport;
#[cfg(test)]
mod tests;

use crate::admission::{admit_peer_digest, rebuild_own_cover};
use mycelium::anastome::dialogue;
use crate::door::Door;
use crate::exchange::{note_root, speak_root_locked, Exchange, Mesh, RootHear};
use crate::genesis::Genesis;
use crate::persist::{persist, persist_if_due};
use crate::syllabus::{syllabus_admit, Syllabus};
use crate::transport::{answer, arm_deadline, note_rtt_ok, note_rtt_timeout, read_frame, tell_peer};
use mycelium::cognition::{outcome_code, Kernel, Outcome};
use mycelium::court::identity::HyphaKey;
use mycelium::network::mesh::{self};
use mycelium::network::nerve::{Cover, Keyed, Router};
use mycelium::network::wire::Frame;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].as_str())
}

fn home_dir(args: &[String]) -> Result<PathBuf, String> {
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--home" {
            i += 2;
            continue;
        }
        // Valueless flags, checked here so an unknown one is still refused
        // rather than silently ignored (rule 11).
        if args[i] == "--mint-genesis" || args[i] == "--recut" {
            i += 1;
            continue;
        }
        if args[i].starts_with("--") {
            return Err(format!("unknown flag {}", args[i]));
        }
        return Err(format!("unexpected argument {}", args[i]));
    }
    if let Some(p) = flag(args, "--home") {
        return Ok(PathBuf::from(p));
    }
    if let Ok(p) = std::env::var("MYCELIUM_HOME") {
        return Ok(PathBuf::from(p));
    }
    // The fleet's homes are `var/<unit>` under the repository the binary
    // runs from (the units' working directory), never the operator's home
    // directory (moved 2026-09-09). A hypha started with no `--home` is the
    // one hypha of a single-node run.
    Ok(PathBuf::from("var").join("hypha"))
}

/// A boot that cannot come up says why and exits: a socket that will not
/// bind, a lock poisoned before any thread ran. It panicked at each of
/// these (§2.4); the operator reads one line either way, and a unit
/// restarts a clean exit as it restarts a panic, without the backtrace.
fn refuse_boot(tag: &str, what: &str) -> ! {
    eprintln!("[{tag}] boot refused: {what}");
    std::process::exit(2)
}

fn resolved_bind(local: std::net::SocketAddr) -> String {
    if local.ip().is_unspecified() {
        format!("127.0.0.1:{}", local.port())
    } else {
        local.to_string()
    }
}

/// A percept, gossip, or demand frame handed from the listener to the reflex
/// pool with its open socket: the reply is written by the judging worker.
struct ReflexJob {
    pub(crate) stream: TcpStream,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // 45.5: H_genesis is written ONCE, by the first hypha started on a fresh
    // state, and PROVISIONED to every other node the way the bootstrap peer
    // list is. Self-adoption from a node's own first receipt is not that:
    // three hyphae started together each wrote their own, and by 45.5 that
    // is three meshes -- measured on this fleet as `refused-origin 2` on
    // every cycle, every node refusing both its peers. A node mints only
    // when told to; every other node adopts from a peer that has one.
    let mint_genesis = args.iter().any(|a| a == "--mint-genesis");
    // A RECUT drops this node's origin so it takes the mesh's again (45.5).
    // It is an operation, not a file edit: the mesh record is signed, so a
    // genesis cannot be cleared from outside the process that holds the key,
    // and a fleet that has diverged has no other way back to one mesh.
    //
    // It DROPS AND EXITS. As a serving mode it would re-cut on every restart
    // -- which is what a unit drop-in carrying it did to this fleet, walking
    // it to epoch 3 one restart at a time. Exiting makes that impossible: a
    // unit that carries the flag flaps visibly instead of quietly
    // re-originating the mesh under everyone. `scripts/recut` runs it once.
    let recut = args.iter().any(|a| a == "--recut");
    let home = match home_dir(&args) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("mycelium-server --home DIR: {e}");
            std::process::exit(2);
        }
    };
    let _ = std::fs::create_dir_all(&home);
    let _ = std::fs::create_dir_all(home.join("corpora"));
    let state_path = home.join("ledger").to_string_lossy().into_owned();
    // The key lives under the home the operator named; a home that is not
    // UTF-8 is spelled lossily, never swapped for a relative default.
    let hypha_key = HyphaKey::load_or_mint(&home.join("hypha.key").to_string_lossy());
    let vk = hypha_key.vk_bytes();
    let id = vk;
    let tag = hypha_key.prefix();
    let mut mesh_rec = mesh::load_or_migrate_hypha(&home, &hypha_key);
    let mut join_peers: Vec<String> = mesh_rec.peers.clone();

    let height = mycelium::chronometry::Height::default();
    let kernel = Arc::new(Mutex::new(Kernel::new()));
    // C6. `genesis` was written as [0u8; 32] and only ever printed, so the
    // boot line read `genesis 0000… epoch 2` on every node and said nothing.
    // 45.5: H_genesis is the CHAIN HASH OF THE MESH'S FIRST RECEIPT, written
    // once by the first hypha started on a fresh state. It is adopted here
    // from this kernel's own chain the first time one exists, so the line is
    // a reading rather than a placeholder; a node that has admitted nothing
    // still has no genesis and says so. 47.2's REFUSE_ORIGIN check on it is
    // task 5.2 and is not claimed here.
    if recut {
        genesis::recut(&mut mesh_rec, &home, &hypha_key, &tag);
        return;
    }
    genesis::announce(&mesh_rec, &tag);
    {
        let k = Arc::clone(&kernel);
        let path = state_path.clone();
        let _ = ctrlc::set_handler(move || {
            persist(&k, &path);
            std::process::exit(0);
        });
    }
    if let Ok(bytes) = std::fs::read(&state_path) {
        let Ok(mut k) = kernel.lock() else { refuse_boot(&tag, "kernel lock poisoned at restore") };
        if k.restore(&bytes) {
            eprintln!(
                "[{tag}] ledger MYC3: {} ruled sentences, treasury entries {}, book derived at intake",
                k.basis_len(),
                k.treasury_entry_count()
            );
        }
    }
    eprintln!("[{tag}] hypha identity vk {}…", mycelium::court::identity::hex32(&vk));
    let genesis = Genesis {
        hypha: mesh_rec.hypha.clone(),
        voice: mesh_rec.voice.clone(),
    };
    {
        let mut live: Vec<[u8; 32]> = genesis.hypha.iter().map(|(who, _)| *who).collect();
        live.extend(genesis.voice.iter().map(|(who, _)| *who));
        let Ok(mut k) = kernel.lock() else { refuse_boot(&tag, "kernel lock poisoned at genesis") };
        k.bind_live(&live);
        k.bind_identity(vk);
        if let Some(held) = k.treasury().bond_of(&vk) {
            eprintln!("[{tag}] bond from ledger {held:?}");
        }
        for (who, mu) in &genesis.voice {
            let paid = k.allocate(*who, *mu);
            if paid.iter().any(|&x| x > 0) {
                eprintln!("[{tag}] genesis voice {:02x}{:02x}… allocated {paid:?}", who[0], who[1]);
            }
        }
    }

    let (early_listener, addr) = if !mesh_rec.bind.is_empty() {
        (None, mesh_rec.bind.clone())
    } else {
        let listener = match TcpListener::bind("0.0.0.0:0") {
            Ok(l) => l,
            Err(e) => refuse_boot(&tag, &format!("bind 0.0.0.0:0: {e}")),
        };
        let resolved = match listener.local_addr() {
            Ok(local) => resolved_bind(local),
            Err(e) => refuse_boot(&tag, &format!("local address unread: {e}")),
        };
        mesh_rec.bind = resolved.clone();
        let _ = mesh_rec.save(&home, &hypha_key);
        eprintln!("[{tag}] wrote bind {resolved}");
        (Some(listener), resolved)
    };
    join_peers.retain(|p| p != &addr);

    let mesh = Mesh {
        cover: Arc::new(Mutex::new(Cover::new())),
        rooms: Arc::new(Mutex::new(Keyed::new())),
        addrs: Arc::new(Mutex::new(Keyed::new())),
        vk,
        sk: hypha_key.sk.clone(),
        bind: addr.clone(),
        rtt: Arc::new(Mutex::new(Keyed::new())),
        rtt_window: Arc::new(Mutex::new(Keyed::new())),
        last_roots: Arc::new(Mutex::new(Keyed::new())),
        // 45.5: a genesis of all zeros is not a genesis, it is the absence
        // of one. `None` here refuses every admission (REFUSE_ORIGIN is not
        // even reached -- there is nothing to compare against), which is the
        // correct answer for a node that has adopted no origin.
        origin: Arc::new(Mutex::new(
            mesh_rec.genesis.map(|g| (g, mesh_rec.epoch)),
        )),
    };
    let router = Arc::new(Mutex::new(Router::new()));
    if let Ok(mut r) = router.lock() {
        r.insert(vk, addr.clone());
    }
    rebuild_own_cover(&kernel, &mesh);
    for p in &join_peers {
        if p == &addr {
            continue;
        }
        match tell_peer(p, dialogue::encode_digests_request(&mesh.vk)) {
            Some((reply, dt)) => {
                note_rtt_ok(&mesh, p.as_bytes(), dt);
                let _ = admit_peer_digest(&mesh, &reply, &router);
            }
            None => note_rtt_timeout(&mesh, p.as_bytes()),
        }
    }

    let registry = Arc::new(Mutex::new(mycelium::registry::TaskRegistry::new()));
    genesis::register_corpora(&home, &state_path, &kernel, &registry, &genesis, vk, &tag);
    let listener = match early_listener {
        Some(l) => l,
        None => match TcpListener::bind(&addr) {
            Ok(l) => l,
            Err(e) => refuse_boot(&tag, &format!("bind {addr}: {e}")),
        },
    };
    eprintln!(
        "[{tag}] mycelium-hypha bound {addr}; join {:?}; vk {tag}…",
        join_peers,
    );

    // THE SYLLABUS (distributed cognition): every court-ratified word, in
    // ruling order; each enrolled voice carries a cursor and receives the
    // stream exactly once on contact (a rebind re-enters the stream — learn
    // refreshes by address, so refinements propagate). Vec-of-relations.
    let syllabus = Arc::new(Mutex::new(Syllabus {
        order: Vec::new(),
        at: Keyed::new(),
        served: Keyed::new(),
    }));
    {
        let seed = match kernel.lock() {
            Ok(k) => k.syllabus_seed(),
            Err(_) => refuse_boot(&tag, "kernel lock poisoned at syllabus seed"),
        };
        let Ok(mut sy) = syllabus.lock() else { refuse_boot(&tag, "syllabus lock poisoned at seed") };
        // index: ruled strata, coarsest first
        syllabus_admit(&mut sy, seed);
        if !sy.order.is_empty() {
            eprintln!("  syllabus seeded: {} shallowest-first ruled words", sy.order.len());
        }
    }
    // PROVEN ROOTS heard on the nerve: (task, solver). A root is gossiped
    // once and re-dispatched here first; a voice that reproduces the solve
    // proves it shares the root.
    let roots: Arc<Mutex<Keyed<Keyed<u64>>>> = Arc::new(Mutex::new(Keyed::new()));
    // THE NERVE THREAD: every act this node speaks to a peer leaves through
    // a bounded channel drained here, so the door never dials. The bound is
    // the reserve ceiling; a full nerve drops the act, as a full reflex
    // pool defers a demand.
    let (nerve_tx, nerve_rx) = std::sync::mpsc::sync_channel::<(String, Vec<u8>)>(mycelium::physics::reserve_max() as usize);
    exchange::spawn_nerve(mesh.clone(), nerve_rx);
    // The roots this book already holds settled are announced to the nerve
    // at boot, so a family proven here is re-spoken on every peer's book.
    let settled = registry.lock().map(|r| r.settled_ids()).unwrap_or_default();
    let boot_stamp = 0u64;
    for t in &settled {
        let hear = {
            let Ok(mut g) = roots.lock() else { refuse_boot(&tag, "roots lock poisoned at boot") };
            note_root(&mut g, t, id, boot_stamp)
        };
        match hear {
            RootHear::Known => {}
            RootHear::First | RootHear::Second => {
                speak_root_locked(&router, &nerve_tx, &id, t, &id, None, boot_stamp);
            }
        }
    }
    if !settled.is_empty() {
        eprintln!("  roots announced: {} settled families, each to its key's next hop", settled.len());
    }

    // THE ASYNC DISPATCHER (contemplation queue): dispatched tasks flow to a
    // worker thread over a bounded channel, so the accept loop never blocks
    // behind deep projection. The bound is the machine's parallelism; a full
    // queue drops the send — speculation is optional, service is not.
    let (synth_tx, synth_rx) = std::sync::mpsc::sync_channel::<mycelium::synthesis::Deliberation>(
        std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
    );
    {
        let kernel = Arc::clone(&kernel);
        let tu = height;
        let state_path = state_path.clone();
        std::thread::spawn(move || loop {
            let work = match synth_rx.recv() {
                Ok(w) => w,
                Err(_) => break,
            };
            {
                match work {
                    mycelium::synthesis::Deliberation::Adopt(ratifier, word) => {
                        let before = kernel.lock().map(|k| k.volume()).unwrap_or_default();
                        let _ = mycelium::synthesis::adopt(&kernel, ratifier, &word);
                        let after = kernel.lock().map(|k| k.volume()).unwrap_or_default();
                        if after != before {
                            eprintln!(
                                "  [{}] adopt {:02x}{:02x}… volume {before:?} → {after:?} (court {:02x})",
                                tu.stamp(),
                                word.address()[0],
                                word.address()[1],
                                ratifier[0],
                            );
                        }
                        persist_if_due(&kernel, &state_path);
                    }
                }
            }
        });
    }

    // THE REFLEX POOL: percepts and demands are judged on worker threads —
    // the reflex opening and the commit are brief locks, the reduction runs
    // over the language snapshot with no lock held — so the listener only
    // accepts and dispatches. Sized by the machine's parallelism; a full
    // pool defers the demand (the honest "come back", which clients retry).
    let cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    let (reflex_tx, reflex_rx) = std::sync::mpsc::sync_channel::<ReflexJob>(cores);
    let reflex_rx = Arc::new(Mutex::new(reflex_rx));
    let door = Door {
        kernel: Arc::clone(&kernel),
        mesh: mesh.clone(),
        router: Arc::clone(&router),
        id,
        bind: addr.clone(),
        join_peers: Arc::new(join_peers.clone()),
        roots: Arc::clone(&roots),
        registry: Arc::clone(&registry),
        syllabus: Arc::clone(&syllabus),
        nerve_tx: nerve_tx.clone(),
        synth_tx: synth_tx.clone(),
        state_path: state_path.clone(),
    };
    Exchange {
        kernel: Arc::clone(&kernel),
        mesh: mesh.clone(),
        join: join_peers.clone(),
        gen_home: home.clone(),
        epoch: mesh_rec.epoch,
        mint_genesis,
        registry: Arc::clone(&registry),
        router: Arc::clone(&router),
        state_path: state_path.clone(),
        syllabus: Arc::clone(&syllabus),
        genesis_held: mesh_rec.genesis.is_some(),
    }
    .spawn();
    // ACTS SERVED, counted per kind. This wrote one line per act and one
    // wall-clock duration with it: 63,331 lines in five minutes on one
    // hypha, which is not a trace, it is a screen that hides every other
    // line the fleet writes -- the exchange's own lines were read as absent
    // when they were merely outnumbered a thousand to one. It also read
    // `Instant::now()`, and a kernel reads no clock (law 9).
    //
    // The count speaks when it crosses a power of two: about twenty lines
    // per kind for the life of the process, dense where the behaviour is
    // new and silent where it is steady.
    let served: Arc<[std::sync::atomic::AtomicU64; 256]> =
        Arc::new(std::array::from_fn(|_| std::sync::atomic::AtomicU64::new(0)));
    for _ in 0..(cores / 2).max(1) {
        let rx = Arc::clone(&reflex_rx);
        let door = door.clone();
        let served = Arc::clone(&served);
        std::thread::spawn(move || loop {
            let job = match rx.lock() {
                Ok(r) => r.recv(),
                Err(_) => break,
            };
            match job {
                Ok(job) => {
                    let ReflexJob { mut stream } = job;
                    // One act, then the worker returns — keep-alive must not
                    // hold a reflex slot for the client's stroke. The
                    // transport deadline bounds the wait for that one frame.
                    arm_deadline(&stream);
                    if let Some(frame) = read_frame(&mut stream) {
                        // An act with no kind byte is judged (and refused
                        // there) but counted under no kind: 0xff was a
                        // sentinel standing for a byte that was not read.
                        let kind = frame.body.first().copied();
                        judge::judge(&mut stream, frame, &door);
                        if let Some(kind) = kind {
                            let n = served[kind as usize]
                                .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                                + 1;
                            if n.is_power_of_two() {
                                eprintln!("  reflex act {kind:02x} served {n}");
                            }
                        }
                    }
                }
                Err(_) => break,
            }
        });
    }

    for stream in listener.incoming() {
        let Ok(s) = stream else { continue };
        let _ = s.set_nodelay(true);
        arm_deadline(&s);
        if let Err(e) = reflex_tx.try_send(ReflexJob { stream: s }) {
            let job = match e {
                std::sync::mpsc::TrySendError::Full(j) | std::sync::mpsc::TrySendError::Disconnected(j) => j,
            };
            let mut s = job.stream;
            let body = dialogue::encode_deferral(outcome_code(&Outcome::GermPending));
            answer(&mut s, Frame { lane: mycelium::network::wire::Stratum::Spatial, body });
        }
    }
}
