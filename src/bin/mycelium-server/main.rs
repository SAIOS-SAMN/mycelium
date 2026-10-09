// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 23.8, 26 — THE HYPHA, a court on the mesh: the arguments it is started
//! with, the order its organs come up in, and the accept loop that hands
//! each act to the judge.

#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

mod admission;
mod backfill;
mod console;
mod door;
mod exchange;
mod fold;
mod genesis;
mod held;
mod judge;
mod ledger;
mod offer;
mod persist;
mod published;
mod syllabus;
mod transport;
mod tshelf;
mod wallet;
mod why;
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
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].as_str())
}

fn home_dir(args: &[String]) -> Result<PathBuf, String> {
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--home"
            || args[i] == "--at"
            || args[i] == "--voice"
            || args[i] == "--court"
            || args[i] == "--reading"
            || args[i] == "--why"
            || args[i] == "--wallet"
        {
            i += 2;
            continue;
        }
        // Valueless flags, checked here so an unknown one is still refused
        // rather than silently ignored (rule 11).
        if args[i] == "--mint-genesis" || args[i] == "--recut" || args[i] == "--check" || args[i] == "--console" || args[i] == "--ledger" {
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

/// WHAT THIS HOME WILL DO AT THE NEXT BOOT, read and not started.
///
/// A layout change is a recut (§10) and the reader for the retired layout
/// is deleted with it, so a build can arrive at a fleet whose carriers it
/// refuses — which is exactly where this one stands: `MYC7` writes every key
/// with a `u64` length and every ledger on disk is `MYC6`, which used the
/// wire's `u16` citation prefix and could not write a large form's address at
/// all. Before this, the only way to
/// learn that was to stop a unit and start the new binary on it, and the
/// answer came back as a court that would not come up. A procedure that
/// cannot be surveyed before it is run is a leap.
///
/// IT CHANGES NOTHING. `load_or_mint` would write a seed into a home that
/// has none and `load_or_migrate_hypha` re-saves a record it read under
/// the old magic (X7) -- both are right at a boot and both would make
/// this reading alter what it reports, so neither is used. The record is
/// opened directly and the ledger is restored into a scratch kernel that
/// is dropped.
///
/// Exit 0 where this home will boot, 3 where it will refuse -- distinct
/// from `refuse_boot`'s 2, so a caller can tell "this home needs the
/// recut" from "the check itself could not run".
fn check_home(home: &Path) -> ! {
    let name = home.display();
    let Some(key) = mycelium::court::identity::HyphaKey::load_only(&home.join("hypha.key").to_string_lossy())
    else {
        println!("{name}: no key — a home that has never booted; it will mint one and start fresh");
        std::process::exit(0)
    };
    let vk = key.vk_bytes();
    let tag = key.prefix();
    println!("{name}: key {tag}… vk {}", mycelium::court::identity::hex32(&vk));

    let mut refuses = false;
    let mesh_path = home.join("mesh");
    match std::fs::read(&mesh_path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("  mesh: none — it will be written from the old text files, or fresh");
        }
        Err(e) => {
            println!("  mesh: UNREADABLE ({e}) — the boot refuses");
            refuses = true;
        }
        Ok(b) => match mesh::MeshRecord::open_layout(&b, &vk) {
            None => {
                println!("  mesh: does not open under this home's key — the boot refuses");
                refuses = true;
            }
            Some((rec, layout)) => {
                let (magic, next) = match layout {
                    mesh::Layout::Current => ("MSH2", "already current"),
                    mesh::Layout::Legacy => ("MSH1", "the next boot writes it back as MSH2 (X7)"),
                };
                println!(
                    "  mesh: {magic}, {next}; bind {:?}, peers {}, genesis {}, epoch {}",
                    rec.bind,
                    rec.peers.len(),
                    match rec.genesis {
                        Some(g) => format!("{:02x}{:02x}…", g[0], g[1]),
                        None => "none".into(),
                    },
                    rec.epoch
                );
            }
        },
    }

    let ledger_path = home.join("ledger");
    match std::fs::read(&ledger_path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("  ledger: none — a fresh court, which boots");
        }
        Err(e) => {
            println!("  ledger: UNREADABLE ({e}) — the boot refuses");
            refuses = true;
        }
        Ok(b) => {
            // The line and the eras are held first, as the boot holds them;
            // nothing here writes to the home.
            let mut scratch = Kernel::new();
            let bound = scratch.bind_line(home.join("line")).and_then(|()| scratch.bind_eras(home.join("eras")));
            if let Err(why) = &bound {
                println!("  line or eras: {why}");
            }
            if bound.is_ok() && scratch.restore(&b) {
                println!(
                    "  ledger: {} bytes, restores — {} ruled sentences, K {}, {} nodes held hot ({} this court's own), {} eras",
                    b.len(),
                    scratch.basis_len(),
                    scratch.chain().head_k(),
                    scratch.chain().nodes().count(),
                    scratch.chain().own_len(),
                    scratch.archive().len()
                );
                // THE FOLD AS THIS HOME STANDS: where its line would cut, and
                // whether the ledger and the nets below the cut read exact.
                match scratch.fold_head() {
                    Some(head) => {
                        let rows = scratch.treasury().rows_below(head);
                        println!(
                            "  fold: head {} over {} rows — audit {:?}, nets {:?}",
                            head.print(),
                            rows.len(),
                            scratch.treasury().audit().map(|_| "exact"),
                            mycelium::economy::nets_read(&rows).map(|n| n.rows.len())
                        );
                    }
                    None => println!("  fold: no close to cut at"),
                }
                // What this build writes of that state must read back: the
                // restore reshapes the line (a peer's folded line leaves),
                // so the next persist is not what was just read.
                let (again, line) = (scratch.snapshot(), scratch.line_image());
                drop(scratch);
                let mut back = Kernel::new();
                let held = line.as_deref().map_or(Ok(()), |l| back.hold_line(l));
                if held.is_ok() && back.restore(&again) {
                    println!("  ledger: as this build writes it, {} bytes beside a line of {}, restores", again.len(), line.map_or(0, |l| l.len()));
                } else {
                    println!("  ledger: as this build writes it, {} bytes, DOES NOT RESTORE", again.len());
                    refuses = true;
                }
            } else {
                println!(
                    "  ledger: {} bytes, DOES NOT RESTORE — the refusal above names the magic and \
                     the byte. A ledger written by an older build is expected to refuse; §10's \
                     road is `scripts/recut`, which deletes it.",
                    b.len()
                );
                refuses = true;
            }
        }
    }
    match refuses {
        true => {
            println!("{name}: THIS HOME WILL NOT BOOT on this build");
            std::process::exit(3)
        }
        false => {
            println!("{name}: this home will boot");
            std::process::exit(0)
        }
    }
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
    // THE PREFLIGHT READS AND EXITS, before anything is created: a check
    // that makes the directories it was asked to report on has changed the
    // answer it gives.
    if args.iter().any(|a| a == "--check") {
        check_home(&home);
    }
    // §8.1 — THE CONSOLE ASKS AND EXITS, before anything is created, as the
    // check does: it reads a hypha (this home's own bind, or `--at`) and
    // starts nothing here.
    if args.iter().any(|a| a == "--console") {
        // 78.92 — why one act on the hypha's own line stands.
        if let Some(asked) = flag(&args, "--why") {
            why::run(&home, flag(&args, "--at"), asked);
        }
        // 78.94 — the hypha's whole line by action, archive included.
        if args.iter().any(|a| a == "--ledger") {
            ledger::run(&home, flag(&args, "--at"));
        }
        // 78.93 B1 — one party's wallet at the hypha.
        if let Some(asked) = flag(&args, "--wallet") {
            wallet::run(&home, flag(&args, "--at"), asked);
        }
        let ask = console::Ask {
            voice: flag(&args, "--voice"),
            court: flag(&args, "--court"),
            reading: flag(&args, "--reading"),
        };
        console::run(&home, flag(&args, "--at"), &ask);
    }
    let _ = std::fs::create_dir_all(&home);
    let _ = std::fs::create_dir_all(home.join("corpora"));
    let state_path = home.join("ledger").to_string_lossy().into_owned();
    // The key lives under the home the operator named; a home that is not
    // UTF-8 is spelled lossily, never swapped for a relative default.
    let hypha_key = HyphaKey::load_or_mint(&home.join("hypha.key").to_string_lossy());
    let vk = hypha_key.vk_bytes();
    let id = vk;
    let tag = hypha_key.prefix();
    let mut mesh_rec = match mesh::load_or_migrate_hypha(&home, &hypha_key) {
        Ok(rec) => rec,
        Err(why) => refuse_boot(&tag, &why),
    };
    let mut join_peers: Vec<String> = mesh_rec.peers.clone();

    let height = mycelium::chronometry::Height::default();
    // I38: the hypha boots unfunded and unbonded; the corpus that registers
    // measures its cover and its own key is what stakes (rule 8). The
    // fixture kernel funded `[0x11; 32]`, and every live ledger carried a
    // stake for a key the mesh record never named.
    let kernel = Arc::new(Mutex::new(Kernel::boot()));
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
        // SIGINT, SIGTERM and SIGHUP alike (ctrlc's `termination`): systemd
        // stops and restarts a unit with SIGTERM, and a court stopped
        // without persisting boots with what it last marked.
        let _ = ctrlc::set_handler(move || {
            persist(&k, &path);
            std::process::exit(0);
        });
    }
    // X8 — A COURT THAT CANNOT READ ITS LEDGER DOES NOT BOOT. Starting
    // clean on a ledger that refused is not a recovery: it is a whole
    // court's basis, chain and treasury dropped, and the next persist
    // writes the empty one over it. Hypha b ran for two epochs that way
    // and reported `basis=0 K=0` as if it were a fresh node. A ledger
    // that will not read is the operator's to look at -- the refusal
    // already names the section and the byte -- and the process exits
    // non-zero rather than continuing as an amnesiac.
    //
    // A ledger that is NOT THERE is a fresh court, which is a reading and
    // not a refusal; every other read error is a refusal, because a state
    // file this process cannot read is one it must not overwrite.
    //
    // 45.8: the line and the eras are on disk beside the snapshot, each in
    // its own order, and are held before the snapshot is restored against
    // them.
    {
        let Ok(mut k) = crate::held::held!(kernel) else { refuse_boot(&tag, "kernel lock poisoned at the line") };
        if let Err(why) = k.bind_line(home.join("line")).and_then(|()| k.bind_eras(home.join("eras"))) {
            refuse_boot(&tag, &format!("{why}. `--check` on this home reports the same reading."));
        }
    }
    match std::fs::read(&state_path) {
        Ok(bytes) => {
            let Ok(mut k) = crate::held::held!(kernel) else { refuse_boot(&tag, "kernel lock poisoned at restore") };
            if !k.restore(&bytes) {
                // A REFUSAL THAT NAMES THE ROAD. This named the fault and
                // stopped, leaving the operator to work out whether a
                // ledger written by an older build is expected to refuse
                // (it is -- §10 deletes the reader for a retired layout)
                // or whether the file is damaged. The reading above says
                // which, and the remedy for the first is a procedure this
                // repository owns.
                refuse_boot(
                    &tag,
                    &format!(
                        "the ledger at {state_path} did not restore -- the magic, section and \
                         byte are named above. This court's memory is NOT dropped and nothing is \
                         written over it. This build reads MYC7; a ledger written by an older \
                         build is EXPECTED to refuse, and §10's road for that is `scripts/recut`, \
                         which the operator runs. `--check` on this home reports the same reading \
                         without starting anything."
                    ),
                );
            }
            eprintln!(
                "[{tag}] ledger MYC7: {} ruled sentences, treasury entries {}, book derived at intake",
                k.basis_len(),
                k.treasury_entry_count()
            );
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("[{tag}] no ledger at {state_path}: a fresh court");
        }
        Err(e) => refuse_boot(&tag, &format!("the ledger at {state_path} is unreadable: {e}")),
    }
    eprintln!("[{tag}] hypha identity vk {}…", mycelium::court::identity::hex32(&vk));
    let genesis = Genesis {
        hypha: mesh_rec.hypha.clone(),
        voice: mesh_rec.voice.clone(),
    };
    {
        let mut live: Vec<[u8; 32]> = genesis.hypha.iter().map(|(who, _)| *who).collect();
        live.extend(genesis.voice.iter().map(|(who, _)| *who));
        let Ok(mut k) = crate::held::held!(kernel) else { refuse_boot(&tag, "kernel lock poisoned at genesis") };
        for (who, mu) in k.bind_live(&live) {
            eprintln!(
                "[{tag}] bond outside the live genesis burned {:02x}{:02x}… {mu:?}",
                who[0], who[1]
            );
        }
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

    let (early_listener, addr) = if let Some(bound) = mesh_rec.bind.clone() {
        (None, bound)
    } else {
        let listener = match TcpListener::bind("0.0.0.0:0") {
            Ok(l) => l,
            Err(e) => refuse_boot(&tag, &format!("bind 0.0.0.0:0: {e}")),
        };
        let resolved = match listener.local_addr() {
            Ok(local) => resolved_bind(local),
            Err(e) => refuse_boot(&tag, &format!("local address unread: {e}")),
        };
        mesh_rec.bind = Some(resolved.clone());
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
        asked_at: Arc::new(Mutex::new(Keyed::new())),
        circuits: Arc::new(Mutex::new(Keyed::new())),
        deficit: Arc::new(Mutex::new([0; 4])),
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
                // 23.8, I36: a peer whose reflex pool is full at boot
                // deferred; it is impedance, not a digest refused.
                if crate::transport::is_deferred(&reply) {
                    eprintln!("  court exchange deferred by {p}");
                    continue;
                }
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
        let seed = match crate::held::held!(kernel) {
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
    #[expect(clippy::cast_possible_truncation, reason = "site 4 — reserve_max, a u64 → usize narrowing on a 32-bit target; its opposite operation is asked at site 4")]
    let (nerve_tx, nerve_rx) = std::sync::mpsc::sync_channel::<(String, Vec<u8>)>(mycelium::physics::reserve_max() as usize);
    exchange::spawn_nerve(mesh.clone(), nerve_rx);
    // 78.152 TU-7: the lock watchdog, paced by the final point's advances.
    held::spawn_watchdog();
    // The roots this book already holds settled are announced to the nerve
    // at boot, so a family proven here is re-spoken on every peer's book.
    // A poisoned registry lock is no reading of the book; no root is
    // announced from one.
    let settled = registry.lock().map_or_else(|_| Vec::new(), |r| r.settled_ids());
    let boot_stamp = 0u64;
    for t in &settled {
        let hear = {
            let Ok(mut g) = roots.lock() else { refuse_boot(&tag, "roots lock poisoned at boot") };
            note_root(&mut g, t, id, boot_stamp)
        };
        match hear {
            RootHear::Known => {}
            RootHear::First | RootHear::Second | RootHear::Joined => {
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
        // A host that does not say how many cores it has is one core.
        std::thread::available_parallelism().map_or(1, |n| n.get()),
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
                        // A poisoned kernel lock is no reading of the
                        // volume; the two readings are taken or the line
                        // is not written.
                        let (Ok(before), Ok(())) =
                            (crate::held::held!(kernel).map(|k| k.volume()), Ok::<(), ()>(()))
                        else {
                            continue;
                        };
                        let _ = mycelium::synthesis::adopt(&kernel, ratifier, &word);
                        let Ok(after) = crate::held::held!(kernel).map(|k| k.volume()) else {
                            continue;
                        };
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
    // A host that does not say how many cores it has is one core.
    let cores = std::thread::available_parallelism().map_or(1, |n| n.get());
    let (reflex_tx, reflex_rx) = std::sync::mpsc::sync_channel::<ReflexJob>(cores);
    let reflex_rx = Arc::new(Mutex::new(reflex_rx));
    // 78.146 — the seat relay on its own lock, taken once the kernel is
    // restored: a mind's exempt read is served without the kernel's lock.
    // 78.151 — and the court's first published reading, which the door's
    // reads answer off until the court stroke publishes the next.
    let Ok((seat_relay, first)) = crate::held::held!(kernel).map(|mut k| (k.seat_relay(), k.publish(None))) else {
        eprintln!("the kernel's lock was poisoned before the door opened");
        return;
    };
    let Some(first) = first else {
        eprintln!("the court's first reading did not publish");
        return;
    };
    published::note_cost(&first);
    let shelf = published::Shelf::new(first);
    let door = Door {
        seat_relay,
        published: shelf.clone(),
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
        published: shelf,
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
    // THE DOOR READS, THEN JUDGES (Phase 1 item 2, 2026-10-01). One pool
    // read each frame and judged it, so a query queued behind acts whose
    // workers waited on the kernel: the console gave up at 0.6 s on a court
    // whose stroke held the lock (c at 00:35:42 on 98fc7f8f, and a and c
    // through epoch 115). READERS take each connection off the listener's
    // queue and read its one frame — they never touch the kernel — serving
    // an observation there and handing every other act, read, to the
    // WORKERS that judge it. A worker queue that is full defers the act (the
    // honest "come back"), as a full reflex queue did.
    let (work_tx, work_rx) = std::sync::mpsc::sync_channel::<(TcpStream, Frame)>(cores);
    let work_rx = Arc::new(Mutex::new(work_rx));
    for _ in 0..(cores / 2).max(1) {
        let rx = Arc::clone(&reflex_rx);
        let door = door.clone();
        let work_tx = work_tx.clone();
        std::thread::spawn(move || loop {
            let job = match rx.lock() {
                Ok(r) => r.recv(),
                Err(_) => break,
            };
            let Ok(ReflexJob { mut stream }) = job else { break };
            // One act, then the reader returns — keep-alive must not hold a
            // slot for the client's stroke. The transport deadline bounds
            // the wait for that one frame.
            arm_deadline(&stream);
            let Some(frame) = read_frame(&mut stream) else { continue };
            if mycelium::anastome::dialogue::act_tag(&frame.body) == Some(mycelium::anastome::dialogue::ACT_OBSERVE) {
                judge::observe::judge(&mut stream, &frame, &door);
                mycelium::observe::note_served(mycelium::anastome::dialogue::ACT_OBSERVE);
                continue;
            }
            // 78.151 — a read that takes no kernel is answered here.
            if judge::serve_published(&mut stream, &frame, &door) {
                continue;
            }
            if let Err(e) = work_tx.try_send((stream, frame)) {
                let (std::sync::mpsc::TrySendError::Full((mut stream, frame)) | std::sync::mpsc::TrySendError::Disconnected((mut stream, frame))) = e;
                // 78.146 — a mind's own read is served, once per window.
                if judge::seat::serve_exempt(&mut stream, &frame, &door) {
                    continue;
                }
                let Ok(deficit) = door.mesh.deficit.lock().map(|d| *d) else { continue };
                let body = dialogue::encode_deferral(outcome_code(&Outcome::GermPending), &deficit);
                answer(&mut stream, Frame { lane: frame.lane, body });
            }
        });
    }
    for _ in 0..(cores / 2).max(1) {
        let rx = Arc::clone(&work_rx);
        let door = door.clone();
        let served = Arc::clone(&served);
        std::thread::spawn(move || loop {
            let job = match rx.lock() {
                Ok(r) => r.recv(),
                Err(_) => break,
            };
            let Ok((mut stream, frame)) = job else { break };
            // An act with no kind byte is judged (and refused there) but
            // counted under no kind: 0xff was a sentinel standing for a byte
            // that was not read.
            let kind = mycelium::anastome::dialogue::act_tag(&frame.body);
            judge::judge(&mut stream, frame, &door);
            if let Some(kind) = kind {
                mycelium::observe::note_served(kind);
                let n = served[kind as usize].fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                if n.is_power_of_two() {
                    eprintln!("  reflex act {kind:02x} served {n}");
                }
            }
        });
    }

    // THE OBSERVER NEVER WAITS ON COGNITION — AT THE DOOR TOO. The observe
    // judge reads the kernel only by `try_lock`, but a query never reached
    // it: while every reflex worker waited on the kernel behind a court
    // stroke, the listener deferred each connection before reading which
    // act it carried, and the console read "no reading" off hyphae b and c
    // for whole epochs (the 43-byte deferral, epoch 113). A full pool now
    // hands the connection to ONE overflow worker, which reads the frame:
    // an observation is served, every other act gets the deferral it got
    // before. It takes no kernel lock, so it never queues behind cognition;
    // it is one thread, so a flood spawns nothing. Full too, the listener
    // defers unread, as it did.
    let (overflow_tx, overflow_rx) = std::sync::mpsc::sync_channel::<ReflexJob>(cores);
    {
        let door = door.clone();
        std::thread::spawn(move || {
            for ReflexJob { mut stream } in overflow_rx {
                arm_deadline(&stream);
                let Some(frame) = read_frame(&mut stream) else { continue };
                if mycelium::anastome::dialogue::act_tag(&frame.body) == Some(mycelium::anastome::dialogue::ACT_OBSERVE) {
                    judge::observe::judge(&mut stream, &frame, &door);
                    mycelium::observe::note_served(mycelium::anastome::dialogue::ACT_OBSERVE);
                    continue;
                }
                // 78.151 — a read that takes no kernel is answered here.
                if judge::serve_published(&mut stream, &frame, &door) {
                    continue;
                }
                // 78.146 — a mind's own read is served, once per window.
                if judge::seat::serve_exempt(&mut stream, &frame, &door) {
                    continue;
                }
                let Ok(deficit) = door.mesh.deficit.lock().map(|d| *d) else { continue };
                let body = dialogue::encode_deferral(outcome_code(&Outcome::GermPending), &deficit);
                answer(&mut stream, Frame { lane: frame.lane, body });
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
            let job = match overflow_tx.try_send(job) {
                Ok(()) => continue,
                Err(std::sync::mpsc::TrySendError::Full(j) | std::sync::mpsc::TrySendError::Disconnected(j)) => j,
            };
            let mut s = job.stream;
            // 23.8 (S13.4): the deferral carries the door's per-lane reserve
            // deficit, snapshotted on every court stroke, so the asker dwells
            // on the lanes it names; a deficit that cannot be read is
            // silence, which the asker already counts.
            let Ok(deficit) = mesh.deficit.lock().map(|d| *d) else {
                continue;
            };
            let body = dialogue::encode_deferral(outcome_code(&Outcome::GermPending), &deficit);
            answer(&mut s, Frame { lane: mycelium::network::wire::Stratum::Spatial, body });
        }
    }
}
