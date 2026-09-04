// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! mycelium-server — a HYPHA in the Čech-nerve mesh (never a star). Each hypha
//! runs its OWN Kernel (one eye per node), carries an id + economic tier, and
//! registers its peers into its nerve so the mesh is a non-ring cover of hyphas.
//! Clients dial a hypha (server/client split preserved); peers fuse bases via
//! sun_recon (P6). Usage:
//!   mycelium-server [bind] [--id N] [--tier founder|validator|participant] [--peer ADDR]...
use mycelium::anastome::dialogue;
use mycelium::cognition::{decode_percept, outcome_code, Kernel, Outcome};
use mycelium::economy::Tier;
use mycelium::network::nerve::Router;
use mycelium::network::wire::Frame;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn read_frame(s: &mut TcpStream) -> Option<Frame> {
    let mut head = [0u8; 5];
    s.read_exact(&mut head).ok()?;
    let len = u32::from_le_bytes([head[1], head[2], head[3], head[4]]) as usize;
    if len > mycelium::network::wire::BODY_BUDGET {
        return None;
    }
    let mut buf = vec![0u8; 5 + len];
    buf[..5].copy_from_slice(&head);
    s.read_exact(&mut buf[5..]).ok()?;
    Frame::decode(&buf).map(|(f, _)| f)
}

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].as_str())
}
fn flags(args: &[String], name: &str) -> Vec<String> {
    args.windows(2).filter(|w| w[0] == name).map(|w| w[1].clone()).collect()
}

/// Durable state: the kernel snapshot lands atomically (to .tmp, then
/// rename) so a crash mid-write never corrupts the ledger. Every path that
/// changes the held language — germination, a verdict, an adoption — writes
/// through here; a restart then resumes exactly what the node held.
fn persist(kernel: &Mutex<Kernel>, state_path: &str) {
    let snap = match kernel.lock() {
        Ok(k) => k.snapshot(),
        Err(_) => return,
    };
    let tmp = format!("{state_path}.tmp");
    if std::fs::write(&tmp, &snap).is_ok() {
        let _ = std::fs::rename(&tmp, state_path);
    }
}

/// A percept, gossip, or demand frame handed from the listener to the reflex
/// pool with its open socket: the reply is written by the judging worker.
struct ReflexJob {
    stream: TcpStream,
    frame: Frame,
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
fn speak_root(
    router: &Router,
    nerve: &std::sync::mpsc::SyncSender<(String, Vec<u8>)>,
    self_id: &[u8; 32],
    task: &[u8; 32],
    solver: &[u8; 32],
    heard_from: Option<[u8; 32]>,
    ticks: u64,
) {
    let mut exclude = vec![*self_id];
    exclude.extend(heard_from);
    if let Some((_, addr)) = router.closest_excluding(task, &exclude) {
        // Never dialed from the door: queued for the nerve thread, dropped
        // when the nerve is full — the mesh's own impedance.
        let _ = nerve.try_send((addr, dialogue::encode_root(self_id, task, solver, ticks)));
    }
}

/// Speak one act to a peer on the nerve, bounded by the transport deadline;
/// the reply, if any, is not needed. Silence is the peer's impedance.
fn tell_peer(addr: &str, body: Vec<u8>) -> Option<Vec<u8>> {
    let deadline = std::time::Duration::from_millis(
        mycelium::physics::reserve_max() * (mycelium::physics::reserve_max() / mycelium::physics::spend_unit()).max(1),
    );
    let sock = std::net::ToSocketAddrs::to_socket_addrs(&addr).ok().and_then(|mut a| a.next())?;
    let mut s = std::net::TcpStream::connect_timeout(&sock, deadline).ok()?;
    let _ = s.set_write_timeout(Some(deadline));
    let _ = s.set_read_timeout(Some(deadline));
    let frame = Frame { lane: mycelium::network::wire::Stratum::Spatial, body };
    s.write_all(&frame.encode()?).ok()?;
    let mut head = [0u8; 5];
    std::io::Read::read_exact(&mut s, &mut head).ok()?;
    let len = u32::from_le_bytes([head[1], head[2], head[3], head[4]]) as usize;
    if len > mycelium::network::wire::BODY_BUDGET {
        return None;
    }
    let mut reply = vec![0u8; len];
    std::io::Read::read_exact(&mut s, &mut reply).ok()?;
    Some(reply)
}

fn judge(stream: &mut TcpStream, frame: Frame, kernel: &Mutex<Kernel>, router: &Router, id: &[u8; 32]) {
    // A demand routed on the nerve is heard as the demand it carries; only
    // a peer the router knows is heard, and it is never routed on.
    let frame = match frame.body.first() {
        Some(&k) if k == dialogue::ACT_DEMAND_ROUTED => match dialogue::decode_routed_demand(&frame.body) {
            Some((origin, demand)) if router.knows(&origin) => Frame { lane: frame.lane, body: demand.to_vec() },
            _ => return,
        },
        _ => frame,
    };
    let Some((&kind, rest)) = frame.body.split_first() else { return };
    // ── quote: the market view leaves under a brief lock; pricing and the
    // cohesion witness compute over the snapshot.
    if kind == dialogue::ACT_QUOTE {
        let Some((qvoice, charts)) = dialogue::decode_quote_request(&frame.body) else { return };
        let Some((surface, priced, k_step, cohesion)) = mycelium::synthesis::quote(kernel, qvoice, &charts) else {
            return;
        };
        let items: Vec<dialogue::WorkItem> = priced
            .iter()
            .map(|(e, mu, open)| (e.address(), *mu, e.sub().to_vec(), open.clone()))
            .collect();
        // The frame budget bounds what one contact carries: items ride in
        // rank order until the next would not fit.
        let body = dialogue::encode_quote_reply_within(
            &surface,
            k_step,
            &items,
            mycelium::network::wire::BODY_BUDGET,
        );
        let served = body.get(dialogue::QUOTE_COUNT_AT).copied().unwrap_or(0);
        eprintln!(
            "  quote voice {:02x} → surface {surface:?}; {served}/{} clusters ({} bytes); K={k_step}; cohesion {cohesion:?}",
            qvoice[0],
            items.len(),
            body.len()
        );
        let reply = Frame { lane: frame.lane, body };
        let _ = stream.write_all(&reply.encode().unwrap_or_default());
        return;
    }
    // ── sheaf sync: section digests over the atlas, and a stratum fetch —
    // both reads over the language snapshot, no lock held.
    if kind == dialogue::ACT_DIGESTS {
        let Some(dvoice) = dialogue::decode_digests_request(&frame.body) else { return };
        // The court is answered with the RULED cover (what it finalized
        // here); an entity's sync is answered with the whole language.
        let (ruled, language) = match kernel.lock() {
            Ok(mut k) => (k.ruled_digests_for(&dvoice), k.language()),
            Err(_) => return,
        };
        let digests = ruled.unwrap_or_else(|| mycelium::transducer::sheaf::digests(&language.stalks));
        let reply = Frame { lane: frame.lane, body: dialogue::encode_digests_reply(&digests) };
        let _ = stream.write_all(&reply.encode().unwrap_or_default());
        return;
    }
    if kind == dialogue::ACT_FETCH {
        let Some((fvoice, charts)) = dialogue::decode_fetch(&frame.body) else { return };
        let language = match kernel.lock() {
            Ok(mut k) => k.language(),
            Err(_) => return,
        };
        // The reply fills the carrier across the asked charts, in order.
        let mut budget = mycelium::network::wire::BODY_BUDGET - 64;
        let mut words: Vec<Vec<u8>> = Vec::new();
        for (inv, have) in &charts {
            let page = mycelium::transducer::sheaf::serve(
                &language.stalks,
                |a| language.index.get(a).and_then(|&i| language.words.get(i)),
                inv,
                have,
                budget,
            );
            budget = budget.saturating_sub(page.iter().map(|w| w.len() + 4).sum::<usize>());
            words.extend(page);
            if budget == 0 {
                break;
            }
        }
        if !words.is_empty() {
            eprintln!(
                "  fetch voice {:02x} ← {} words across {} charts",
                fvoice[0],
                words.len(),
                charts.len()
            );
        }
        let reply = Frame { lane: frame.lane, body: dialogue::encode_syllabus_reply(&words) };
        let _ = stream.write_all(&reply.encode().unwrap_or_default());
        return;
    }
    // ── probe: a tentative sentence against an open listing — replayed,
    // settled on graded descent or closure, never germinated.
    if kind == dialogue::ACT_PROBE {
        let Some((pvoice, target, cites)) = dialogue::decode_probe(&frame.body) else { return };
        let (code, paid, remaining) = mycelium::synthesis::probe(kernel, pvoice, &target, &cites);
        if code != dialogue::PROBE_NOTHING {
            eprintln!(
                "  probe voice {:02x} on {:02x}{:02x}… code {code} paid {paid:?} remaining {remaining:?}",
                pvoice[0], target[0], target[1]
            );
        }
        let reply = Frame { lane: frame.lane, body: dialogue::encode_probe_reply(code, &paid, &remaining) };
        let _ = stream.write_all(&reply.encode().unwrap_or_default());
        return;
    }
    let pbytes = if kind == dialogue::ACT_GOSSIP && rest.len() >= 32 { &rest[32..] } else { rest };
    let (out, voice) = if kind == dialogue::ACT_DEMAND {
        // A spoken coboundary with the speaker's own reduction — replayed.
        let Some((dvoice, chain, cites)) = dialogue::decode_demand(&frame.body) else { return };
        (mycelium::synthesis::attest(kernel, &chain, dvoice, &cites), dvoice)
    } else {
        // The client PERCEIVED and REDUCED; the JUDGE replays the proof.
        let Some((pvoice, a_bodies, b_bodies, frame, proof)) = decode_percept(pbytes) else { return };
        (
            mycelium::synthesis::adjudicate(kernel, &a_bodies, &b_bodies, pvoice, &frame, &proof),
            pvoice,
        )
    };
    // TEACH: on install AND on a single-word sentence (the demand IS a held
    // word — express it down so the entity's genome inherits it); SETTLE:
    // the consilience receipt (parties + the μ⃗ award priced at the surface).
    // A brief lock: reads only.
    let (basis, flux, reserve, taught, settle) = {
        let k = kernel.lock().unwrap();
        let (taught, settle) = match &out {
            Outcome::Installed(i) => match k.basis().get(*i) {
                Some(g) => {
                    let addr = g.address();
                    let mu = mycelium::economy::price(
                        &g.magnitude(),
                        &k.physics().pressure,
                        &k.treasury().pool(),
                    );
                    // Teach the FULL lossless form (to_bytes) — the entity's
                    // genome must reconstruct the exact element, sections and all.
                    (g.to_bytes(), Some((addr, k.voices_of(&addr), mu)))
                }
                None => (Vec::new(), None), // retired while the reply formed
            },
            Outcome::Decomposed(word) if word.len() == 1 => {
                (k.basis().get(word[0]).map(|g| g.to_bytes()).unwrap_or_default(), None)
            }
            // RE-TEACH ON REBIND: any sentence citing a court-ANCHORED word
            // teaches those words' full γ = (π, Δ) forms — Epigenome::learn
            // refreshes by address, so the refined anchors propagate to every
            // speaking genome. Every anchored citation is taught, back to
            // back; none is ranked above another.
            Outcome::Decomposed(word) => {
                let mut taught = Vec::new();
                for g in word.iter().filter_map(|&i| k.basis().get(i)).filter(|g| g.anchor().is_some()) {
                    taught.extend_from_slice(&g.to_bytes());
                }
                (taught, None)
            }
            _ => (Vec::new(), None),
        };
        (
            (k.basis_len(), k.sections_total()),
            k.physics().metabolism_rate(),
            k.physics().reserve,
            taught,
            settle,
        )
    };
    let label = match &out {
        Outcome::Decomposed(word) => format!("valid: decomposed → sentence {word:?} ({} generators)", word.len()),
        Outcome::GermPending => "valid: germ pending (one voice)".to_string(),
        Outcome::Installed(i) => format!("valid: GERM INSTALLED #{i}"),
        Outcome::Fragmented(parts) => format!("valid: fragmented ({} bodies)", parts.len()),
        Outcome::Pivot => "invalid: pivot (degenerate — nothing to reduce)".to_string(),
    };
    let src = match kind {
        k if k == dialogue::ACT_GOSSIP => "gossip",
        k if k == dialogue::ACT_DEMAND => "demand",
        _ => "client",
    };
    eprintln!(
        "  {src} percept voice {:02x} → {label}; basis={} vol={}; metabolism {flux:?}; reserve {reserve:?}",
        voice[0], basis.0, basis.1
    );
    // Reply: outcome | metabolism/pressure(4) | taught (len16 | bytes) | settle?
    let mut body = vec![outcome_code(&out)];
    for f in flux {
        body.push(f.min(255) as u8); // per-lane pressure, saturated to a byte
    }
    body.extend_from_slice(&(taught.len() as u16).to_le_bytes());
    body.extend_from_slice(&taught);
    if let Some((germ, voices, mu)) = &settle {
        body.extend_from_slice(&dialogue::encode_settle(germ, voices, mu));
    }
    let reply = Frame { lane: frame.lane, body };
    let _ = stream.write_all(&reply.encode().unwrap_or_default());

    // P6/R5 · sun_recon: a CLIENT percept still awaiting consilience routes ONE
    // receipt to its RENDEZVOUS — the node XOR-closest to the demand's
    // content-address (voice EXCLUDED so all voices of a germ converge there).
    // The rendezvous accumulates the distinct voice + verifies by recomputation;
    // quorum → install + teach outward. No fanout; a down rendezvous fails
    // silently (resilience). Gossip (kind 1) is never re-routed here.
    if kind == dialogue::ACT_PERCEPT
        && pbytes.len() > 32
        && matches!(out, Outcome::GermPending | Outcome::Fragmented(_))
    {
        let key = *blake3::hash(&pbytes[32..]).as_bytes(); // demand address, voice-free
        if let Some(dest) = rendezvous(&key, id, router) {
            let mut g = vec![0x01u8];
            g.extend_from_slice(&key);
            g.extend_from_slice(pbytes);
            let gframe = Frame { lane: frame.lane, body: g }.encode().unwrap_or_default();
            if let Ok(mut ps) = TcpStream::connect(&dest) {
                let _ = ps.write_all(&gframe);
            }
        }
    }
}

/// A stable node id from a string (address) — a wire identity, not cognition.
fn id_of(s: &str) -> [u8; 32] {
    *blake3::hash(s.as_bytes()).as_bytes()
}

/// The RENDEZVOUS for a content key: the XOR-closest peer (found by trie descent,
/// O(bits), never a peer scan), IF closer than this node — else `None` (self IS
/// the rendezvous). One destination, content-addressed; no fanout.
fn rendezvous(key: &[u8; 32], self_id: &[u8; 32], router: &Router) -> Option<String> {
    let dist = |idb: &[u8; 32]| -> [u8; 32] { std::array::from_fn(|i| idb[i] ^ key[i]) };
    match router.closest(key) {
        Some((pid, addr)) if dist(&pid) < dist(self_id) => Some(addr),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let addr = args.get(1).filter(|a| !a.starts_with("--")).cloned().unwrap_or_else(|| "127.0.0.1:7500".to_string());
    let tier = Tier::from_str(flag(&args, "--tier").unwrap_or("participant"));
    let id = flag(&args, "--id").map(id_of).unwrap_or_else(|| id_of(&addr));
    // THE NERVE IDENTITY: what peers key this node by — the digest of the
    // address they were configured with, the same function of the same
    // string on both sides. Speech on the nerve is signed with it and
    // accepted only from identities the router holds.
    let nerve_id = id_of(&addr);
    let join_peers = flags(&args, "--peer");

    // P7 — durable state: the germinated basis + earned ledger persist across
    // restart. Default path is per-node; override with --state PATH.
    let id_tag = flag(&args, "--id").map(String::from).unwrap_or_else(|| addr.replace(':', "_"));
    let state_path = flag(&args, "--state").map(String::from).unwrap_or_else(|| format!("/opt/mycelium-v0.1.0/state/hypha-{id_tag}.ledger"));
    if let Some(dir) = std::path::Path::new(&state_path).parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    // TUNIVERSAL (operator-ruled): the universe's ONE wall clock — genesis
    // origin persisted on the shared state, never a local timezone. Every
    // event line stamps with it so the universe replays on exactness.
    let tuniversal = {
        let dir = std::path::Path::new(&state_path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".into());
        mycelium::chronometry::TUniversal::load_or_genesis(&format!("{dir}/tuniversal"))
    };
    eprintln!("TUniversal {} (origin {})", tuniversal.stamp(), tuniversal.origin_unix);

    let kernel = Arc::new(Mutex::new(Kernel::new()));
    {
        let k = Arc::clone(&kernel);
        let path = state_path.clone();
        let _ = ctrlc::set_handler(move || {
            persist(&k, &path);
            std::process::exit(0);
        });
    }
    // Resume the learned language from disk (a corrupt ledger boots clean, no crash).
    if let Ok(bytes) = std::fs::read(&state_path) {
        let mut k = kernel.lock().unwrap();
        if k.restore(&bytes) {
            eprintln!("restored basis={} from {state_path}", k.basis_len());
        }
    }
    let hypha_key = mycelium::court::identity::HyphaKey::load_or_mint(&format!("{state_path}.key"));
    eprintln!("hypha identity vk {:02x}{:02x}…", hypha_key.vk_bytes()[0], hypha_key.vk_bytes()[1]);
    if !kernel.lock().unwrap().crt_imported() {
        let fed_path = std::path::Path::new(&state_path)
            .parent()
            .map(|p| p.join("court.federation"))
            .unwrap_or_else(|| std::path::PathBuf::from("state/court.federation"));
        if let Some(s) = mycelium::court::ledger::CourtState::load(fed_path.to_str().unwrap_or("state/court.federation")) {
            let n = s.federation.len();
            kernel.lock().unwrap().import_crt(s);
            eprintln!("imported court federation {n} words into CRT1");
        }
    }
    // THE NODE'S OWN STAKE (P5): the hypha enrolls itself at its tier — the
    // genesis capitalization of its escrow pool. Every yield it ever quotes or
    // settles is paid FROM this pool; nothing is minted by a formula.
    {
        let mut k = kernel.lock().unwrap();
        let staked = k.enroll(id, tier);
        eprintln!("enrolled self at {tier:?}; staked mu {staked:?}; pool {:?}", k.treasury().pool());
    }

    // THE AUTHORITATIVE CORPUS REGISTRY (PoUW integrity): the hypha — never a
    // client — decides what counts as work. --corpus DIR ingests each task,
    // registers its content-addressed identity (anti-replay: the FIRST
    // verified solve takes the bounty, replays collapse to the verification
    // floor), and INTAKES its train coboundaries onto the open market so the
    // language grows from the same authority that dispatches the work.
    let mut router = Router::new();
    for pr in &join_peers {
        router.insert(id_of(pr), pr.clone());
    }
    let registry = Arc::new(Mutex::new(mycelium::registry::TaskRegistry::new()));
    if let Some(corpus_dir) = flag(&args, "--corpus") {
        let mut files: Vec<_> = std::fs::read_dir(corpus_dir)
            .map(|rd| {
                rd.filter_map(|e| e.ok().map(|e| e.path()))
                    .filter(|p| p.extension().is_some_and(|x| x == "json"))
                    .collect()
            })
            .unwrap_or_default();
        files.sort();
        let mut reg = registry.lock().unwrap();
        let mut k = kernel.lock().unwrap();
        let mut listed = 0usize;
        let mut tasked = 0usize;
        for f in &files {
            let Ok(text) = std::fs::read_to_string(f) else { continue };
            let Some(task) = mycelium::arc::parse_task(&text) else { continue };
            let mut home = false;
            for (a, b) in &task.train {
                if let Some((mut chain, _, frame)) = mycelium::arc::refine_delta(a, b) {
                    chain.sort_unstable();
                    let key = mycelium::transducer::element::listing_key(&chain, &frame);
                    if rendezvous(&key, &nerve_id, &router).is_some() {
                        continue;
                    }
                    home = true;
                    if k.intake(&chain, &frame) {
                        listed += 1;
                    }
                }
            }
            if home {
                reg.register(task);
                tasked += 1;
            }
        }
        k.unlist_foreign(|addr| rendezvous(addr, &nerve_id, &router).is_none());
        let restored = std::fs::read(format!("{state_path}.registry"))
            .map(|b| reg.overlay(&b))
            .unwrap_or(0);
        eprintln!(
            "corpus registered: {tasked} home tasks from {corpus_dir} ({} unsolved, {listed} demands listed, {restored} settled states restored)",
            reg.unsolved(),
        );
    }

    let listener = TcpListener::bind(&addr).expect("bind");
    eprintln!(
        "mycelium-hypha {:02x} tier {:?} bound {addr}; join {:?} vk {:02x}{:02x}…",
        id[0],
        tier,
        join_peers,
        hypha_key.vk_bytes()[0],
        hypha_key.vk_bytes()[1],
    );

    // THE SYLLABUS (distributed cognition): every court-ratified word, in
    // ruling order; each enrolled voice carries a cursor and receives the
    // stream exactly once on contact (a rebind re-enters the stream — learn
    // refreshes by address, so refinements propagate). Vec-of-relations.
    let mut syllabus: Vec<[u8; 32]> = Vec::new();
    // PROVEN ROOTS heard on the nerve: (task, solver). A root is gossiped
    // once and re-dispatched here first; a voice that reproduces the solve
    // proves it shares the root.
    let mut roots: mycelium::network::nerve::Keyed<Vec<[u8; 32]>> = mycelium::network::nerve::Keyed::new();
    // THE NERVE THREAD: every act this node speaks to a peer leaves through
    // a bounded channel drained here, so the door never dials. The bound is
    // the reserve ceiling; a full nerve drops the act, as a full reflex
    // pool defers a demand.
    let (nerve_tx, nerve_rx) = std::sync::mpsc::sync_channel::<(String, Vec<u8>)>(mycelium::physics::reserve_max() as usize);
    std::thread::spawn(move || {
        for (addr, body) in nerve_rx {
            // Each dial's outcome is the peer's measured impedance: a root
            // is delivered when the peer answers with its own clock.
            match tell_peer(&addr, body) {
                Some(reply) => match dialogue::decode_root_reply(&reply) {
                    Some((held, at)) => eprintln!(
                        "  nerve → {addr}: root delivered at {} ({})",
                        mycelium::chronometry::TUniversal::stamp_of(at),
                        if held.is_some() { "held there" } else { "new there" }
                    ),
                    None => eprintln!("  nerve → {addr}: answered {} bytes", reply.len()),
                },
                None => eprintln!("  nerve → {addr}: no answer"),
            }
        }
    });
    // The roots this book already holds settled are announced to the nerve
    // at boot, so a family proven here is re-spoken on every peer's book.
    let settled = registry.lock().map(|r| r.settled_ids()).unwrap_or_default();
    for t in &settled {
        roots.insert(*t, vec![id]);
        speak_root(&router, &nerve_tx, &nerve_id, t, &id, None, tuniversal.ticks());
    }
    if !settled.is_empty() {
        eprintln!("  roots announced: {} settled families, each to its key's next hop", settled.len());
    }
    let mut syllabus_served: Vec<([u8; 32], usize)> = Vec::new();

    // THE ASYNC DISPATCHER (contemplation queue): dispatched tasks flow to a
    // worker thread over a bounded channel, so the accept loop never blocks
    // behind deep projection. The bound is the machine's parallelism; a full
    // queue drops the send — speculation is optional, service is not.
    let (synth_tx, synth_rx) = std::sync::mpsc::sync_channel::<mycelium::synthesis::Deliberation>(
        std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
    );
    {
        let kernel = Arc::clone(&kernel);
        let tu = tuniversal;
        let state_path = state_path.clone();
        // Persistence is PACED: topological volume (cards, fibre, listings)
        // and an absolute dwell. K is not a persist event.
        let mut persisted = kernel.lock().map(|k| k.volume()).unwrap_or_default();
        let mut last_persist = Instant::now();
        std::thread::spawn(move || loop {
            let work = match synth_rx.recv_timeout(std::time::Duration::from_millis(
                mycelium::physics::reserve_max(),
            )) {
                Ok(w) => w,
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    mycelium::synthesis::consolidate(&kernel);
                    let live = kernel.lock().map(|k| k.volume()).unwrap_or_default();
                    if live != persisted && last_persist.elapsed() >= mycelium::physics::PERSIST_DWELL {
                        persist(&kernel, &state_path);
                        persisted = live;
                        last_persist = Instant::now();
                    }
                    continue;
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
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
                        if after != persisted && last_persist.elapsed() >= mycelium::physics::PERSIST_DWELL {
                            persist(&kernel, &state_path);
                            persisted = after;
                            last_persist = Instant::now();
                        }
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
    let router = Arc::new(router);
    let overlap: Arc<Mutex<Vec<([u8; 32], String, [u8; 32], Vec<(u8, mycelium::transducer::stalk::Invariant)>)>>> =
        Arc::new(Mutex::new(Vec::new()));
    {
        let kernel = Arc::clone(&kernel);
        let nerve_tx = nerve_tx.clone();
        let router = Arc::clone(&router);
        let overlap = Arc::clone(&overlap);
        let join = join_peers.clone();
        let vk = hypha_key.vk_bytes();
        let sk_bytes = hypha_key.sk.to_bytes();
        let nerve_id = nerve_id;
        let bind = addr.clone();
        std::thread::spawn(move || {
            let sk = ed25519_dalek::SigningKey::from_bytes(&sk_bytes);
            loop {
                std::thread::sleep(std::time::Duration::from_secs(2));
                // Cover first: the peer must hold this vk before an Offer
                // can verify. The nerve thread is FIFO, one dial at a time.
                let charts = kernel.lock().map(|k| k.active_cover()).unwrap_or_default();
                let cover = dialogue::encode_cover(&vk, &bind, &charts);
                let mut cover_dests = join.clone();
                if let Ok(ov) = overlap.lock() {
                    for (_, a, _, _) in ov.iter() {
                        if !cover_dests.contains(a) {
                            cover_dests.push(a.clone());
                        }
                    }
                }
                for d in cover_dests {
                    let _ = nerve_tx.try_send((d, cover.clone()));
                }
                let offers = match kernel.lock() {
                    Ok(mut k) => k.court_stroke(),
                    Err(_) => continue,
                };
                for (addr, spoken, voices, deflation, kstep) in offers {
                    let elem = spoken.to_bytes();
                    let prefix = dialogue::offer_signed_prefix(&addr, &elem, &voices, &deflation, kstep);
                    let sig = {
                        use ed25519_dalek::Signer;
                        sk.sign(&prefix).to_bytes()
                    };
                    let body = dialogue::encode_offer(&addr, &elem, &voices, &deflation, kstep, &sig);
                    let chart = mycelium::transducer::stalk::invariant_of(&spoken);
                    let dests = {
                        let ov = overlap.lock().unwrap();
                        let hits: Vec<String> = ov
                            .iter()
                            .filter(|(_, _, _, chs)| chs.iter().any(|(_, inv)| inv == &chart))
                            .map(|(_, a, _, _)| a.clone())
                            .collect();
                        if !hits.is_empty() {
                            hits
                        } else if let Some(d) = rendezvous(
                            &*blake3::hash(
                                &chart.0.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<_>>(),
                            )
                            .as_bytes(),
                            &nerve_id,
                            &router,
                        ) {
                            vec![d]
                        } else {
                            join.clone()
                        }
                    };
                    for d in dests {
                        let _ = nerve_tx.try_send((d, body.clone()));
                    }
                    eprintln!(
                        "  court Offer {:02x}{:02x}… first vantage",
                        addr[0],
                        addr[1]
                    );
                }
            }
        });
    }
    for _ in 0..(cores / 2).max(1) {
        let rx = Arc::clone(&reflex_rx);
        let kernel = Arc::clone(&kernel);
        let router = Arc::clone(&router);
        std::thread::spawn(move || loop {
            let job = match rx.lock() {
                Ok(r) => r.recv(),
                Err(_) => break,
            };
            match job {
                Ok(job) => {
                    let ReflexJob { mut stream, mut frame } = job;
                    loop {
                        let kind = frame.body.first().copied().unwrap_or(0xff);
                        let t0 = std::time::Instant::now();
                        judge(&mut stream, frame, &kernel, &router, &id);
                        eprintln!("  reflex act {kind:02x} {} ms", t0.elapsed().as_millis());
                        match read_frame(&mut stream) {
                            Some(next) => frame = next,
                            None => break,
                        }
                    }
                }
                Err(_) => break,
            }
        });
    }

    for stream in listener.incoming() {
        let Ok(mut s) = stream else { continue };
        loop {
        let Some(frame) = read_frame(&mut s) else { break };
        // Body: [act][..] — the ANASTOME dialogue (anastome::dialogue). Percept and
        // routed-gossip percepts keep their historical tags; quote and demand are
        // the spoken economy; teach rides the reply; settle is appended on install.
        let Some((&kind, rest)) = frame.body.split_first() else { break };

        // ── quote: "what is the highest-yield work?" ─────────────────────────
        // (ACT_QUOTE is judged on the reflex pool — see `judge`.)

        // ── root: a proven family heard on the nerve ─────────────────────────
        if kind == dialogue::ACT_ROOT {
            let Some((from, task, solver, spoken_at)) = dialogue::decode_root(&frame.body) else { continue };
            // Only a peer the nerve was configured with is heard.
            if !router.knows(&from) {
                let _ = s.write_all(&Frame { lane: frame.lane, body: Vec::new() }.encode().unwrap_or_default());
                continue;
            }
            let _ = spoken_at;
            let n = {
                if let Some(v) = roots.get_mut(&task) {
                    if !v.contains(&solver) {
                        v.push(solver);
                    }
                    v.len()
                } else {
                    roots.insert(task, vec![solver]);
                    1
                }
            };
            if n == 1 {
                let prioritized = registry.lock().map(|mut r| r.prioritize(&task)).unwrap_or(false);
                eprintln!(
                    "  ROOT heard from {:02x}: task {:02x}{:02x}… solved by voice {:02x}; {}",
                    from[0],
                    task[0],
                    task[1],
                    solver[0],
                    if prioritized { "re-dispatched first here" } else { "not open on this book" }
                );
                speak_root(&router, &nerve_tx, &nerve_id, &task, &solver, Some(from), tuniversal.ticks());
            } else if n == 2 {
                for (_, c) in syllabus_served.iter_mut() {
                    *c = 0;
                }
                eprintln!(
                    "  ROOT CONSILIENT task {:02x}{:02x}… two voices; syllabus rewound",
                    task[0],
                    task[1],
                );
                speak_root(&router, &nerve_tx, &nerve_id, &task, &solver, Some(from), tuniversal.ticks());
            }
            // The reply is this node's record for the task and its clock.
            let reply = dialogue::encode_root_reply(roots.get(&task).and_then(|v| v.first().copied()), tuniversal.ticks());
            let _ = s.write_all(&Frame { lane: frame.lane, body: reply }.encode().unwrap_or_default());
            continue;
        }

        if kind == dialogue::ACT_COVER {
            if let Some((vk, from, charts)) = dialogue::decode_cover(&frame.body) {
                let pid = id_of(&from);
                let mut forward: Vec<String> = Vec::new();
                if let Ok(mut ov) = overlap.lock() {
                    let fresh = ov.iter().all(|(id, _, _, _)| *id != pid);
                    ov.retain(|(id, _, _, _)| *id != pid);
                    ov.push((pid, from.clone(), vk, charts));
                    if fresh {
                        eprintln!(
                            "  cover from {:02x}{:02x}… {from} vk {:02x}{:02x}…",
                            pid[0],
                            pid[1],
                            vk[0],
                            vk[1]
                        );
                        for (_, a, _, _) in ov.iter() {
                            if a != &from && !forward.contains(a) {
                                forward.push(a.clone());
                            }
                        }
                        for j in &join_peers {
                            if j != &from && !forward.contains(j) {
                                forward.push(j.clone());
                            }
                        }
                    }
                }
                let body = frame.body.clone();
                for d in forward {
                    let _ = nerve_tx.try_send((d, body.clone()));
                }
                let _ = s.write_all(&Frame { lane: frame.lane, body: Vec::new() }.encode().unwrap_or_default());
            }
            continue;
        }
        if kind == dialogue::ACT_OFFER {
            if let Some((addr, bytes, voices, deflation, kstep, sig)) = dialogue::decode_offer(&frame.body) {
                let prefix = dialogue::offer_signed_prefix(&addr, &bytes, &voices, &deflation, kstep);
                let known = overlap.lock().map(|ov| ov.clone()).unwrap_or_default();
                // Offer is the other organ's first vantage — never self.
                let ok = known.iter().any(|(_, _, vk, _)| mycelium::court::identity::verify(vk, &prefix, &sig));
                let mut installed_body = None;
                if ok {
                    let mut pos = 0usize;
                    if let Some(spoken) = mycelium::transducer::element::Element::from_bytes(&bytes, &mut pos) {
                        if let Ok(mut k) = kernel.lock() {
                            if let Some(elem) = k.accept_offer(&addr, spoken, deflation) {
                                let eb = elem.to_bytes();
                                let pre = dialogue::installed_signed_prefix(&addr, &eb);
                                use ed25519_dalek::Signer;
                                let sig_peer = hypha_key.sk.sign(&pre).to_bytes();
                                installed_body = Some(dialogue::encode_installed(&addr, &eb, &sig, &sig_peer));
                                eprintln!("  court Installed {:02x}{:02x}… second vantage", addr[0], addr[1]);
                            }
                        }
                    }
                }
                if let Some(body) = installed_body {
                    let mut dests: Vec<String> = overlap
                        .lock()
                        .map(|ov| ov.iter().map(|(_, a, _, _)| a.clone()).collect())
                        .unwrap_or_default();
                    if dests.is_empty() {
                        dests = join_peers.clone();
                    }
                    for d in dests {
                        let _ = nerve_tx.try_send((d, body.clone()));
                    }
                }
                let _ = s.write_all(&Frame { lane: frame.lane, body: Vec::new() }.encode().unwrap_or_default());
            }
            continue;
        }
        if kind == dialogue::ACT_INSTALLED {
            if let Some((addr, bytes, sig_home, sig_peer)) = dialogue::decode_installed(&frame.body) {
                let prefix = dialogue::installed_signed_prefix(&addr, &bytes);
                let mut vks = vec![hypha_key.vk_bytes()];
                if let Ok(ov) = overlap.lock() {
                    for (_, _, vk, _) in ov.iter() {
                        if !vks.contains(vk) {
                            vks.push(*vk);
                        }
                    }
                }
                let home_vk = vks.iter().find(|vk| mycelium::court::identity::verify(vk, &prefix, &sig_home)).copied();
                let peer_vk = vks.iter().find(|vk| mycelium::court::identity::verify(vk, &prefix, &sig_peer)).copied();
                if home_vk.is_some() && peer_vk.is_some() && home_vk != peer_vk {
                    let mut pos = 0usize;
                    if let Some(spoken) = mycelium::transducer::element::Element::from_bytes(&bytes, &mut pos) {
                        if let Ok(mut k) = kernel.lock() {
                            if k.accept_installed(&addr, spoken) {
                                eprintln!("  court admitted Installed {:02x}{:02x}…", addr[0], addr[1]);
                            }
                        }
                    }
                }
                let _ = s.write_all(&Frame { lane: frame.lane, body: Vec::new() }.encode().unwrap_or_default());
            }
            continue;
        }

        // ── evidence: the court PULLS the fiber's evidence; entities DEPOSIT
        // their verified dialect findings — local thought seeding the global
        // refinement (one act, two directions, told apart by body length).
        if kind == dialogue::ACT_EVIDENCE {
            if let Some((_dvoice, items)) = dialogue::decode_evidence_deposit(&frame.body) {
                let accepted = {
                    let mut k = kernel.lock().unwrap();
                    items
                        .into_iter()
                        .filter(|(addr, fp, scene)| {
                            k.deposit_evidence(*addr, fp.clone(), scene.clone())
                        })
                        .count()
                };
                let reply = Frame { lane: frame.lane, body: vec![accepted.min(255) as u8] };
                let _ = s.write_all(&reply.encode().unwrap_or_default());
                continue;
            }
            let Some(_evoice) = dialogue::decode_evidence_request(&frame.body) else { continue };
            let items: Vec<([u8; 32], Vec<(i64, i64)>, Vec<(i64, i64, u8)>)> = {
                let k = kernel.lock().unwrap();
                k.anchor_evidence().iter().take(32).cloned().collect()
            };
            let reply = Frame { lane: frame.lane, body: dialogue::encode_evidence_reply(&items) };
            let _ = s.write_all(&reply.encode().unwrap_or_default());
            continue;
        }

        // ── task: a client pulls a signed work envelope from the registry ────
        if kind == dialogue::ACT_TASK {
            let Some(_tvoice) = dialogue::decode_task_request(&frame.body) else { continue };
            let dispatched = {
                let mut reg = registry.lock().unwrap();
                reg.dispatch().map(|(tid, task)| (tid, task.clone()))
            };
            let body = match &dispatched {
                Some((tid, task)) => {
                    let tests: Vec<_> = task.test.iter().map(|(x, _)| x.clone()).collect();
                    dialogue::encode_task_envelope(tid, &task.train, &tests)
                }
                None => Vec::new(), // an empty envelope: the book is settled
            };
            let reply = Frame { lane: frame.lane, body };
            let _ = s.write_all(&reply.encode().unwrap_or_default());
            // The entity comprehends the family it is dispatched; the hypha
            // only dispatches and verifies.
            let _ = dispatched;
            continue;
        }

        // ── solve: verify against held answers; ANTI-REPLAY yield collapse ───
        if kind == dialogue::ACT_SOLVE {
            let Some((svoice, tid, outputs)) = dialogue::decode_solve(&frame.body) else { continue };
            let (code, mu) = {
                let mut reg = registry.lock().unwrap();
                match reg.settle(&tid, &outputs) {
                    mycelium::registry::SolveYield::Bounty(tension) => {
                        let snap = reg.snapshot();
                        let tmp = format!("{state_path}.registry.tmp");
                        if std::fs::write(&tmp, &snap).is_ok() {
                            let _ = std::fs::rename(&tmp, format!("{state_path}.registry"));
                        }
                        let mut k = kernel.lock().unwrap();
                        let anchor = k.anchor_pub();
                        let paid = k.treasury_settle_solve(svoice, tension, anchor);
                        drop(k);
                        let n = {
                            if let Some(v) = roots.get_mut(&tid) {
                                if !v.contains(&svoice) {
                                    v.push(svoice);
                                }
                                v.len()
                            } else {
                                roots.insert(tid, vec![svoice]);
                                1
                            }
                        };
                        eprintln!("  ROOT task {:02x}{:02x}… proven by voice {:02x}; spoken to its key's next hop", tid[0], tid[1], svoice[0]);
                        speak_root(&router, &nerve_tx, &nerve_id, &tid, &svoice, None, tuniversal.ticks());
                        if n >= 2 {
                            for (_, c) in syllabus_served.iter_mut() {
                                *c = 0;
                            }
                            eprintln!("  ROOT CONSILIENT task {:02x}{:02x}… {n} voices; syllabus rewound", tid[0], tid[1]);
                        }
                        (dialogue::SOLVE_BOUNTY, paid)
                    }
                    mycelium::registry::SolveYield::VerifyFloor => {
                        let mut k = kernel.lock().unwrap();
                        let anchor = k.anchor_pub();
                        let paid = k.treasury_settle_solve(svoice, [0, 0, 1, 0], anchor);
                        drop(k);
                        let n = {
                            if let Some(v) = roots.get_mut(&tid) {
                                if !v.contains(&svoice) {
                                    v.push(svoice);
                                }
                                v.len()
                            } else {
                                roots.insert(tid, vec![svoice]);
                                1
                            }
                        };
                        if n == 2 {
                            for (_, c) in syllabus_served.iter_mut() {
                                *c = 0;
                            }
                            speak_root(&router, &nerve_tx, &nerve_id, &tid, &svoice, None, tuniversal.ticks());
                            eprintln!("  ROOT CONSILIENT task {:02x}{:02x}… two voices; syllabus rewound", tid[0], tid[1]);
                        }
                        (dialogue::SOLVE_FLOOR, paid)
                    }
                    mycelium::registry::SolveYield::Nothing => (dialogue::SOLVE_NOTHING, [0; 4]),
                }
            };
            eprintln!(
                "  [{}] solve voice {:02x} task {:02x}{:02x}… → {} μ⃗ {mu:?}",
                tuniversal.stamp(),
                svoice[0],
                tid[0],
                tid[1],
                match code {
                    dialogue::SOLVE_BOUNTY => "BOUNTY (first verified solve)",
                    dialogue::SOLVE_FLOOR => "floor (replay)",
                    _ => "nothing",
                },
            );
            let reply =
                Frame { lane: frame.lane, body: dialogue::encode_solve_receipt(code, &mu) };
            let _ = s.write_all(&reply.encode().unwrap_or_default());
            continue;
        }

        // ── adopt (genome synchronization): a court-ratified word arriving
        // with the ratifier's voice. The request path only parses and
        // queues; verification and installation run on the deliberation
        // worker (replay pure, derivability on a snapshot, brief install) —
        // no long lock on the reflex path, no boolean ladder. The reply
        // acknowledges queueing.
        // ── peers: the nerve this hypha stands in, for an entity's router.
        if kind == dialogue::ACT_PEERS {
            if dialogue::decode_peers_request(&frame.body).is_some() {
                let mut nerve_addrs = vec![addr.clone()];
                nerve_addrs.extend(join_peers.iter().cloned());
                let reply = Frame { lane: frame.lane, body: dialogue::encode_peers_reply(&nerve_addrs) };
                let _ = s.write_all(&reply.encode().unwrap_or_default());
            }
            continue;
        }
        if kind == dialogue::ACT_ADOPT {
            let Some((ratifier, elem_bytes)) = dialogue::decode_adopt(&frame.body) else {
                continue;
            };
            let mut pos = 0usize;
            let queued = mycelium::transducer::element::Element::from_bytes(&elem_bytes, &mut pos)
                .map(|word| {
                    // A court-ratified word arriving for adoption is ratified
                    // language wherever it was first spoken: it enters the
                    // teaching stream here.
                    let addr = word.address();
                    if !syllabus.contains(&addr) {
                        syllabus.push(addr);
                    }
                    synth_tx
                        .try_send(mycelium::synthesis::Deliberation::Adopt(ratifier, word))
                        .is_ok()
                })
                .unwrap_or(false);
            let reply = Frame { lane: frame.lane, body: vec![queued as u8] };
            let _ = s.write_all(&reply.encode().unwrap_or_default());
            continue;
        }

        // ── genesis: the court pages through the held basis for its audit —
        // a brief serialization under the lock, depth paced by the
        // metabolism like the syllabus. An empty page ends the walk.
        if kind == dialogue::ACT_GENESIS {
            let Some((_gvoice, cursor)) = dialogue::decode_genesis_request(&frame.body) else {
                continue;
            };
            let words: Vec<Vec<u8>> = {
                let k = kernel.lock().unwrap();
                let free: u64 = k.physics().reserve.iter().sum();
                let depth = (1
                    + (free as usize * 7) / (4 * mycelium::physics::reserve_max() as usize))
                    .min(8);
                k.basis()
                    .iter()
                    .skip(cursor as usize)
                    .take(depth)
                    .map(|w| w.to_bytes())
                    .collect()
            };
            let reply =
                Frame { lane: frame.lane, body: dialogue::encode_syllabus_reply(&words) };
            let _ = s.write_all(&reply.encode().unwrap_or_default());
            continue;
        }

        // ── syllabus: the ratified stream served from this voice's cursor ────
        if kind == dialogue::ACT_SYLLABUS {
            let Some(svoice) = dialogue::decode_syllabus_request(&frame.body) else { continue };
            if !syllabus_served.iter().any(|(v, _)| *v == svoice) {
                syllabus_served.push((svoice, 0));
            }
            let cursor =
                &mut syllabus_served.iter_mut().find(|(v, _)| *v == svoice).unwrap().1;
            let mut missed: Vec<[u8; 32]> = Vec::new();
            let words: Vec<Vec<u8>> = {
                let k = kernel.lock().unwrap();
                // Depth follows the metabolism like the quote book: a calm
                // judge teaches more per contact, a strained one fewer.
                let free: u64 = k.physics().reserve.iter().sum();
                let depth =
                    (1 + (free as usize * 7) / (4 * mycelium::physics::reserve_max() as usize)).min(8);
                let mut out = Vec::new();
                while *cursor < syllabus.len() && out.len() < depth {
                    let addr = syllabus[*cursor];
                    // Serve the CURRENT form — a rebind since ruling teaches
                    // the refined γ. A word not yet held (adoption in flight)
                    // holds the cursor until it lands; a retired word is
                    // passed over.
                    match k.word_at(&addr) {
                        Some(w) => out.push(w.to_bytes()),
                        None => missed.push(addr), // adoption in flight: taught once it lands
                    }
                    *cursor += 1;
                }
                out
            };
            let served = words.len();
            // A word not yet held returns to the tail of the stream once, so
            // an adoption in flight is taught when it lands and a retired
            // word falls out.
            let ahead = *cursor;
            for addr in missed {
                if !syllabus[ahead..].contains(&addr) {
                    syllabus.push(addr);
                }
            }
            let reply =
                Frame { lane: frame.lane, body: dialogue::encode_syllabus_reply(&words) };
            let _ = s.write_all(&reply.encode().unwrap_or_default());
            if served > 0 {
                eprintln!(
                    "  [{}] syllabus voice {:02x} ← {served} ratified words",
                    tuniversal.stamp(),
                    svoice[0],
                );
            }
            continue;
        }

        // ── refine: a compression bid — verified by deterministic replay ─────
        if kind == dialogue::ACT_REFINE {
            let Some((rvoice, word, cites)) = dialogue::decode_refine(&frame.body) else { continue };
            let (code, mu) = {
                let mut k = kernel.lock().unwrap();
                match k.try_refine(&word, rvoice, &cites) {
                    Some(paid) => (dialogue::REFINE_ACCEPTED, paid),
                    None => (dialogue::REFINE_NOTHING, [0; 4]),
                }
            };
            if code == dialogue::REFINE_ACCEPTED {
                eprintln!(
                    "  [{}] refine voice {:02x} word {:02x}{:02x}… RETIRED (vocabulary compressed) μ⃗ {mu:?}",
                    tuniversal.stamp(),
                    rvoice[0],
                    word[0],
                    word[1],
                );
            }
            let reply =
                Frame { lane: frame.lane, body: dialogue::encode_solve_receipt(code, &mu) };
            let _ = s.write_all(&reply.encode().unwrap_or_default());
            continue;
        }

        // ── verdict: the court's signed ruling lands ─────────────────────────
        if kind == dialogue::ACT_VERDICT {
            let Some((cvoice, addr, ruling, receipt, pi, attach)) =
                dialogue::decode_verdict(&frame.body)
            else {
                continue;
            };
            let applied = {
                let mut k = kernel.lock().unwrap();
                k.set_court_head(receipt); // the court's head is this hypha's temporal root
                k.set_court_voice(cvoice); // the court's voice, for the ruled cover it reconciles against
                let ok = match ruling {
                    // The court's family-derived π* rebinds the word's anchor
                    // fiber — the dual condition activates for this word.
                    dialogue::RULING_REBIND => match pi {
                        Some(term) => k.apply_rebind(&addr, term),
                        None => false, // a rebind with no term rules nothing
                    },
                    // The bundle connects: the word transports into its base
                    // here by the court's g; an attach with no base rules nothing.
                    dialogue::RULING_ATTACHED => match attach {
                        Some((base, g)) => k.apply_attach(&addr, &base, &g) > 0,
                        None => false,
                    },
                    _ => k.apply_verdict(&addr, ruling),
                };
                ok
            };
            // RATIFICATION → SYLLABUS: a ratified word this hypha holds enters
            // the teaching stream for every enrolled voice, whether the
            // ruling arrived while it was provisional or after it installed.
            let held = kernel.lock().map(|k| k.basis().iter().any(|w| w.address() == addr)).unwrap_or(false);
            if held
                && matches!(ruling, dialogue::RULING_UPHELD | dialogue::RULING_REBIND | dialogue::RULING_ATTACHED)
                && !syllabus.contains(&addr)
            {
                syllabus.push(addr);
            }
            eprintln!(
                "  [{}] verdict court {:02x} on {:02x}{:02x}… ruling {ruling} receipt {:02x}{:02x}… applied={applied}",
                tuniversal.stamp(),
                cvoice[0], addr[0], addr[1], receipt[0], receipt[1]
            );
            let reply = Frame { lane: frame.lane, body: vec![applied as u8] };
            let _ = s.write_all(&reply.encode().unwrap_or_default());
            continue;
        }

        // ── demand → also spoken on the nerve, to the one peer the router
        // places closest to the listing's key (its home in the mesh), so the
        // second voice on a listing can be a stranger's. One hop, keyed,
        // never a broadcast; a routed demand is heard once and not routed on.
        if kind == dialogue::ACT_DEMAND {
            if let Some((_, chain, _)) = dialogue::decode_demand(&frame.body) {
                let key = mycelium::transducer::element::from_residual(&chain).address();
                if let Some((_, addr)) = router.closest_excluding(&key, &[nerve_id]) {
                    let _ = nerve_tx.try_send((addr, dialogue::encode_routed_demand(&nerve_id, &frame.body)));
                }
            }
        }
        // ── percept / gossip / demand → the reflex pool. The listener never
        // reduces; a full pool defers the demand in the reply's own shape
        // (outcome, four zero lanes, nothing taught).
        let lane = frame.lane;
        let job = ReflexJob { stream: s, frame };
        if let Err(e) = reflex_tx.try_send(job) {
            let job = match e {
                std::sync::mpsc::TrySendError::Full(j) | std::sync::mpsc::TrySendError::Disconnected(j) => j,
            };
            let mut s = job.stream;
            let body = dialogue::encode_deferral(outcome_code(&Outcome::GermPending));
            let _ = s.write_all(&Frame { lane, body }.encode().unwrap_or_default());
        }
        break;
        }
    }
}
