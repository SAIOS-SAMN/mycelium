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
use mycelium::network::nerve::{home, Cover, Keyed, Router};
use mycelium::network::wire::Frame;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn arm_deadline(s: &TcpStream, deadline: std::time::Duration) {
    let _ = s.set_read_timeout(Some(deadline));
    let _ = s.set_write_timeout(Some(deadline));
}

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
}

/// Shared door state: the listener only accepts and try_sends; every act
/// that needs the kernel runs on a reflex worker, snapshot / off-lock /
/// re-lock, never on the accept thread.
struct Syllabus {
    order: Vec<[u8; 32]>,
    at: Keyed<()>,
    served: Keyed<usize>,
}

#[derive(Clone)]
struct Door {
    kernel: Arc<Mutex<Kernel>>,
    mesh: Mesh,
    router: Arc<Router>,
    id: [u8; 32],
    bind: String,
    join_peers: Arc<Vec<String>>,
    roots: Arc<Mutex<Keyed<Keyed<u64>>>>,
    registry: Arc<Mutex<mycelium::registry::TaskRegistry>>,
    syllabus: Arc<Mutex<Syllabus>>,
    nerve_tx: std::sync::mpsc::SyncSender<(String, Vec<u8>)>,
    synth_tx: std::sync::mpsc::SyncSender<mycelium::synthesis::Deliberation>,
    state_path: String,
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
enum RootHear {
    Known,
    First,
    Second,
}

/// Record a root by (task, solver) with its stamp. A solver already held
/// for this task is silence; a new solver is spoken.
fn note_root(roots: &mut Keyed<Keyed<u64>>, task: [u8; 32], solver: [u8; 32], stamp: u64) -> RootHear {
    match roots.get_mut(&task) {
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
fn tell_peer(addr: &str, body: Vec<u8>, deadline: std::time::Duration) -> Option<(Vec<u8>, std::time::Duration)> {
    let t0 = Instant::now();
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
    Some((reply, t0.elapsed()))
}

struct Mesh {
    cover: Arc<Mutex<Cover>>,
    rooms: Arc<Mutex<Keyed<[u64; 4]>>>,
    addrs: Arc<Mutex<Keyed<String>>>,
    vk: [u8; 32],
    sk: ed25519_dalek::SigningKey,
    bind: String,
    rtt: Arc<Mutex<std::time::Duration>>,
    last_roots: Arc<Mutex<Keyed<([[u8; 32]; 4], [u8; 32])>>>,
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
            last_roots: Arc::clone(&self.last_roots),
        }
    }
}

fn mesh_deadline(mesh: &Mesh) -> std::time::Duration {
    let rtt = mesh.rtt.lock().map(|t| *t).unwrap_or_default();
    mycelium::physics::transport_deadline(rtt)
}

fn mesh_rtt(mesh: &Mesh) -> std::time::Duration {
    mesh.rtt.lock().map(|t| *t).unwrap_or_default()
}

fn note_rtt(mesh: &Mesh, dt: std::time::Duration) {
    if let Ok(mut t) = mesh.rtt.lock() {
        *t = if dt.is_zero() { std::time::Duration::from_micros(1) } else { dt };
    }
}

fn room64(r: [usize; 4]) -> [u64; 4] {
    std::array::from_fn(|i| r[i] as u64)
}

fn rebuild_own_cover(kernel: &Mutex<Kernel>, mesh: &Mesh) -> (Vec<(u8, mycelium::transducer::stalk::Invariant)>, [u64; 4]) {
    let (charts, room) = match kernel.lock() {
        Ok(k) => (k.own_cover_charts(), room64(k.lane_room())),
        Err(_) => return (Vec::new(), [0; 4]),
    };
    if let Ok(mut c) = mesh.cover.lock() {
        c.retract_hypha(&mesh.vk);
        for (lane, inv) in &charts {
            c.admit(*lane, inv, mesh.vk);
        }
    }
    if let Ok(mut r) = mesh.rooms.lock() {
        r.insert(mesh.vk, room);
    }
    if let Ok(mut a) = mesh.addrs.lock() {
        a.insert(mesh.vk, mesh.bind.clone());
    }
    (charts, room)
}

fn sign_cover_digest(kernel: &Mutex<Kernel>, mesh: &Mesh) -> Vec<u8> {
    let (charts, room) = rebuild_own_cover(kernel, mesh);
    let ruled_root = match kernel.lock() {
        Ok(k) => k.ruled_root(),
        Err(_) => return Vec::new(),
    };
    let cover_roots = mesh.cover.lock().map(|c| c.roots()).unwrap_or([[0u8; 32]; 4]);
    let prefix = dialogue::cover_signed_prefix(&cover_roots, &ruled_root, &room);
    let sig = {
        use ed25519_dalek::Signer;
        mesh.sk.sign(&prefix).to_bytes()
    };
    dialogue::encode_cover_digest(&charts, &room, &cover_roots, &ruled_root, &mesh.vk, &mesh.bind, &sig)
}

fn admit_peer_digest(mesh: &Mesh, body: &[u8]) -> bool {
    let Some((charts, room, cover_roots, ruled_root, vk, addr, sig)) = dialogue::decode_cover_digest(body) else {
        return false;
    };
    let prefix = dialogue::cover_signed_prefix(&cover_roots, &ruled_root, &room);
    if !mycelium::court::identity::verify(&vk, &prefix, &sig) {
        return false;
    }
    if let Ok(mut c) = mesh.cover.lock() {
        c.retract_hypha(&vk);
        for (lane, inv) in &charts {
            c.admit(*lane, inv, vk);
        }
    }
    if let Ok(mut r) = mesh.rooms.lock() {
        r.insert(vk, room);
    }
    if let Ok(mut a) = mesh.addrs.lock() {
        a.insert(vk, addr);
    }
    true
}

/// Control acts that used to run on the accept thread. The listener never
/// holds the kernel lock; these run on a reflex worker.
fn serve_control(s: &mut TcpStream, frame: &Frame, door: &Door) -> bool {
    let Some((&kind, _rest)) = frame.body.split_first() else { return false };
    if kind == dialogue::ACT_ROOT {
        let Some((from, task, solver, spoken_at)) = dialogue::decode_root(&frame.body) else { return true };
        if !door.router.knows(&from) {
            let _ = s.write_all(&Frame { lane: frame.lane, body: Vec::new() }.encode().unwrap_or_default());
            return true;
        }
        let hear = {
            let mut roots = door.roots.lock().unwrap();
            note_root(&mut roots, task, solver, spoken_at)
        };
        match hear {
            RootHear::Known => {}
            RootHear::First => {
                let prioritized = door.registry.lock().map(|mut r| r.prioritize(&task)).unwrap_or(false);
                eprintln!(
                    "  ROOT heard from {:02x}: task {:02x}{:02x}… solved by voice {:02x}; {}",
                    from[0], task[0], task[1], solver[0],
                    if prioritized { "re-dispatched first here" } else { "not open on this book" }
                );
                speak_root(&door.router, &door.nerve_tx, &door.id, &task, &solver, Some(from), spoken_at);
            }
            RootHear::Second => {
                let voices: Vec<[u8; 32]> = {
                    let sy = door.syllabus.lock().unwrap();
                    sy.served.fiber(&mycelium::network::nerve::ROOT).map(|(v, _)| *v).collect()
                };
                if let Ok(mut sy) = door.syllabus.lock() {
                    for v in voices {
                        sy.served.insert(v, 0);
                    }
                }
                eprintln!("  ROOT CONSILIENT task {:02x}{:02x}… two voices; syllabus rewound", task[0], task[1]);
                speak_root(&door.router, &door.nerve_tx, &door.id, &task, &solver, Some(from), spoken_at);
            }
        }
        let reply = {
            let roots = door.roots.lock().unwrap();
            dialogue::encode_root_reply(
                roots.get(&task).and_then(|v| v.fiber(&mycelium::network::nerve::ROOT).next().map(|(k, _)| *k)),
                spoken_at,
            )
        };
        let _ = s.write_all(&Frame { lane: frame.lane, body: reply }.encode().unwrap_or_default());
        return true;
    }
    if kind == dialogue::ACT_EVIDENCE {
        if let Some((_dvoice, items)) = dialogue::decode_evidence_deposit(&frame.body) {
            let accepted = {
                let mut k = door.kernel.lock().unwrap();
                items.into_iter().filter(|(addr, fp, scene)| k.deposit_evidence(*addr, fp.clone(), scene.clone())).count()
            };
            let reply = Frame { lane: frame.lane, body: vec![accepted.min(255) as u8] };
            let _ = s.write_all(&reply.encode().unwrap_or_default());
            return true;
        }
        let Some(_evoice) = dialogue::decode_evidence_request(&frame.body) else { return true };
        let items: Vec<([u8; 32], Vec<(i64, i64)>, Vec<(i64, i64, u8)>)> = {
            let k = door.kernel.lock().unwrap();
            k.anchor_evidence().iter().take(32).cloned().collect()
        };
        let reply = Frame { lane: frame.lane, body: dialogue::encode_evidence_reply(&items) };
        let _ = s.write_all(&reply.encode().unwrap_or_default());
        return true;
    }
    if kind == dialogue::ACT_TASK {
        let Some(_tvoice) = dialogue::decode_task_request(&frame.body) else { return true };
        let dispatched = {
            let mut reg = door.registry.lock().unwrap();
            reg.dispatch().map(|(tid, task)| (tid, task.clone()))
        };
        let body = match &dispatched {
            Some((tid, task)) => {
                let tests: Vec<_> = task.test.iter().map(|(x, _)| x.clone()).collect();
                dialogue::encode_task_envelope(tid, &task.train, &tests)
            }
            None => Vec::new(),
        };
        let reply = Frame { lane: frame.lane, body };
        let _ = s.write_all(&reply.encode().unwrap_or_default());
        return true;
    }
    if kind == dialogue::ACT_SOLVE {
        let Some((svoice, tid, outputs)) = dialogue::decode_solve(&frame.body) else { return true };
        let (code, mu) = {
            let mut reg = door.registry.lock().unwrap();
            match reg.settle(&tid, &outputs) {
                mycelium::registry::SolveYield::Bounty(tension) => {
                    let snap = reg.snapshot();
                    let tmp = format!("{}.registry.tmp", door.state_path);
                    if std::fs::write(&tmp, &snap).is_ok() {
                        let _ = std::fs::rename(&tmp, format!("{}.registry", door.state_path));
                    }
                    drop(reg);
                    let paid = {
                        let mut k = door.kernel.lock().unwrap();
                        k.pay_solve(&tid, svoice, tension)
                    };
                    let hear = {
                        let mut roots = door.roots.lock().unwrap();
                        note_root(&mut roots, tid, svoice, 0)
                    };
                    match hear {
                        RootHear::Known => {}
                        RootHear::First | RootHear::Second => {
                            eprintln!("  ROOT task {:02x}{:02x}… proven by voice {:02x}; spoken to its key's next hop", tid[0], tid[1], svoice[0]);
                            speak_root(&door.router, &door.nerve_tx, &door.id, &tid, &svoice, None, 0);
                        }
                    }
                    if matches!(hear, RootHear::Second) {
                        let voices: Vec<[u8; 32]> = door.syllabus.lock().unwrap().served.fiber(&mycelium::network::nerve::ROOT).map(|(v, _)| *v).collect();
                        if let Ok(mut sy) = door.syllabus.lock() {
                            for v in voices { sy.served.insert(v, 0); }
                        }
                        eprintln!("  ROOT CONSILIENT task {:02x}{:02x}… two voices; syllabus rewound", tid[0], tid[1]);
                    }
                    (dialogue::SOLVE_BOUNTY, paid)
                }
                mycelium::registry::SolveYield::VerifyFloor => {
                    drop(reg);
                    let paid = {
                        let mut k = door.kernel.lock().unwrap();
                        k.pay_solve(&tid, svoice, [0, 0, 1, 0])
                    };
                    let hear = {
                        let mut roots = door.roots.lock().unwrap();
                        note_root(&mut roots, tid, svoice, 0)
                    };
                    if matches!(hear, RootHear::Second) {
                        let voices: Vec<[u8; 32]> = door.syllabus.lock().unwrap().served.fiber(&mycelium::network::nerve::ROOT).map(|(v, _)| *v).collect();
                        if let Ok(mut sy) = door.syllabus.lock() {
                            for v in voices { sy.served.insert(v, 0); }
                        }
                        speak_root(&door.router, &door.nerve_tx, &door.id, &tid, &svoice, None, 0);
                        eprintln!("  ROOT CONSILIENT task {:02x}{:02x}… two voices; syllabus rewound", tid[0], tid[1]);
                    }
                    (dialogue::SOLVE_FLOOR, paid)
                }
                mycelium::registry::SolveYield::Nothing => (dialogue::SOLVE_NOTHING, [0; 4]),
            }
        };
        let reply = Frame { lane: frame.lane, body: dialogue::encode_solve_receipt(code, &mu) };
        let _ = s.write_all(&reply.encode().unwrap_or_default());
        return true;
    }
    if kind == dialogue::ACT_PEERS {
        if dialogue::decode_peers_request(&frame.body).is_some() {
            let mut nerve_addrs = vec![door.bind.clone()];
            nerve_addrs.extend(door.join_peers.iter().cloned());
            let reply = Frame { lane: frame.lane, body: dialogue::encode_peers_reply(&nerve_addrs) };
            let _ = s.write_all(&reply.encode().unwrap_or_default());
        }
        return true;
    }
    if kind == dialogue::ACT_ADOPT {
        let Some((ratifier, elem_bytes)) = dialogue::decode_adopt(&frame.body) else { return true };
        let mut pos = 0usize;
        let queued = mycelium::transducer::element::Element::from_bytes(&elem_bytes, &mut pos)
            .map(|word| {
                let addr = word.address();
                if let Ok(mut sy) = door.syllabus.lock() {
                    if !sy.at.contains(&addr) {
                        sy.at.insert(addr, ());
                        sy.order.push(addr);
                    }
                }
                door.synth_tx.try_send(mycelium::synthesis::Deliberation::Adopt(ratifier, word)).is_ok()
            })
            .unwrap_or(false);
        let reply = Frame { lane: frame.lane, body: vec![queued as u8] };
        let _ = s.write_all(&reply.encode().unwrap_or_default());
        return true;
    }
    if kind == dialogue::ACT_GENESIS {
        let Some((_gvoice, cursor)) = dialogue::decode_genesis_request(&frame.body) else { return true };
        let words: Vec<Vec<u8>> = {
            let k = door.kernel.lock().unwrap();
            let free: u64 = k.physics().reserve.iter().sum();
            let depth = (1 + (free as usize * 7) / (4 * mycelium::physics::reserve_max() as usize)).min(8);
            k.basis().iter().skip(cursor as usize).take(depth).map(|w| w.to_bytes()).collect()
        };
        let reply = Frame { lane: frame.lane, body: dialogue::encode_syllabus_reply(&words) };
        let _ = s.write_all(&reply.encode().unwrap_or_default());
        return true;
    }
    if kind == dialogue::ACT_SYLLABUS {
        let Some(svoice) = dialogue::decode_syllabus_request(&frame.body) else { return true };
        let mut missed: Vec<[u8; 32]> = Vec::new();
        let (cursor, order) = {
            let mut sy = door.syllabus.lock().unwrap();
            if sy.served.get(&svoice).is_none() {
                sy.served.insert(svoice, 0);
            }
            (*sy.served.get(&svoice).unwrap(), sy.order.clone())
        };
        let words: Vec<Vec<u8>> = {
            let k = door.kernel.lock().unwrap();
            let free: u64 = k.physics().reserve.iter().sum();
            let depth = (1 + (free as usize * 7) / (4 * mycelium::physics::reserve_max() as usize)).min(8);
            let mut out = Vec::new();
            let mut next = cursor;
            while next < order.len() && out.len() < depth {
                let addr = order[next];
                match k.word_at(&addr) {
                    Some(w) => out.push(w.to_bytes()),
                    None => missed.push(addr),
                }
                next += 1;
            }
            drop(k);
            if let Ok(mut sy) = door.syllabus.lock() {
                sy.served.insert(svoice, next);
            }
            out
        };
        if !missed.is_empty() {
            let mut sy = door.syllabus.lock().unwrap();
            let ahead = *sy.served.get(&svoice).unwrap_or(&0);
            for addr in missed {
                let mut tail: Keyed<()> = Keyed::new();
                for a in sy.order.get(ahead..).unwrap_or(&[]) {
                    tail.insert(*a, ());
                }
                if !tail.contains(&addr) {
                    sy.order.push(addr);
                }
            }
        }
        let reply = Frame { lane: frame.lane, body: dialogue::encode_syllabus_reply(&words) };
        let _ = s.write_all(&reply.encode().unwrap_or_default());
        return true;
    }
    if kind == dialogue::ACT_REFINE {
        let Some((rvoice, word, cites)) = dialogue::decode_refine(&frame.body) else { return true };
        let (code, mu) = {
            let mut k = door.kernel.lock().unwrap();
            match k.try_refine(&word, rvoice, &cites) {
                Some(paid) => (dialogue::REFINE_ACCEPTED, paid),
                None => (dialogue::REFINE_NOTHING, [0; 4]),
            }
        };
        let reply = Frame { lane: frame.lane, body: dialogue::encode_solve_receipt(code, &mu) };
        let _ = s.write_all(&reply.encode().unwrap_or_default());
        return true;
    }
    if kind == dialogue::ACT_VERDICT {
        let Some((cvoice, addr, ruling, receipt, pi, attach)) = dialogue::decode_verdict(&frame.body) else { return true };
        let applied = {
            let mut k = door.kernel.lock().unwrap();
            k.set_court_head(receipt);
            k.set_court_voice(cvoice);
            match ruling {
                dialogue::RULING_REBIND => match pi {
                    Some(term) => k.apply_rebind(&addr, term),
                    None => false,
                },
                dialogue::RULING_ATTACHED => match attach {
                    Some((base, g)) => k.apply_attach(&addr, &base, &g) > 0,
                    None => false,
                },
                _ => k.apply_verdict(&addr, ruling),
            }
        };
        let held = door.kernel.lock().map(|k| k.word_at(&addr).is_some()).unwrap_or(false);
        if held && matches!(ruling, dialogue::RULING_UPHELD | dialogue::RULING_REBIND | dialogue::RULING_ATTACHED) {
            if let Ok(mut sy) = door.syllabus.lock() {
                if !sy.at.contains(&addr) {
                    sy.at.insert(addr, ());
                    sy.order.push(addr);
                }
            }
        }
        let reply = Frame { lane: frame.lane, body: vec![applied as u8] };
        let _ = s.write_all(&reply.encode().unwrap_or_default());
        return true;
    }
    false
}

fn homes_here(mesh: &Mesh, kernel: &Mutex<Kernel>, inv: &mycelium::transducer::stalk::Invariant, mag: &[u64; 4]) -> bool {
    let room = kernel.lock().map(|k| room64(k.lane_room())).unwrap_or([0; 4]);
    let c = mesh.cover.lock().unwrap();
    let r = mesh.rooms.lock().unwrap();
    let a = mesh.addrs.lock().unwrap();
    home(&c, inv, mag, &r, &a, Some((&mesh.vk, &room, &mesh.bind))).0 == mesh.vk
}

fn judge(stream: &mut TcpStream, frame: Frame, door: &Door) {
    let kernel = &door.kernel;
    let router = door.router.as_ref();
    let id = &door.id;
    let mesh = &door.mesh;
    if serve_control(stream, &frame, door) {
        return;
    }
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
            mycelium::network::wire::frame_budget(),
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
        let (ruled, language) = match kernel.lock() {
            Ok(mut k) => (k.ruled_digests_for(&dvoice), k.language()),
            Err(_) => return,
        };
        let digests = ruled.unwrap_or_else(|| mycelium::transducer::sheaf::digests(&language.stalks));
        let mut body = dialogue::encode_digests_reply(&digests);
        body.extend(sign_cover_digest(kernel, mesh));
        let reply = Frame { lane: frame.lane, body };
        let _ = stream.write_all(&reply.encode().unwrap_or_default());
        return;
    }
    if kind == dialogue::ACT_FETCH {
        let Some((fvoice, charts)) = dialogue::decode_fetch(&frame.body) else { return };
        let mut budget = mycelium::network::wire::frame_budget().saturating_sub(64);
        let mut words: Vec<Vec<u8>> = Vec::new();
        if let Ok(mut k) = kernel.lock() {
            let mut have_all: Vec<[u8; 32]> = Vec::new();
            for (inv, have) in &charts {
                have_all.extend_from_slice(have);
                let page = k.serve_ruled(inv, have, budget);
                budget = budget.saturating_sub(page.iter().map(|w| w.len() + 4).sum::<usize>());
                words.extend(page);
                if budget == 0 {
                    break;
                }
            }
            k.confirm_peer_holds_addrs(&have_all);
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
    if kind == dialogue::ACT_DEMAND {
        if let Some((_, chain, _)) = dialogue::decode_demand(&frame.body) {
            let key = mycelium::transducer::element::from_residual(&chain).address();
            if let Some((_, addr)) = router.closest_excluding(&key, &[*id]) {
                let _ = door.nerve_tx.try_send((addr, dialogue::encode_routed_demand(id, &frame.body)));
            }
        }
    }
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
    let mut pruned = false;
    if let Ok(bytes) = std::fs::read(&state_path) {
        let mut k = kernel.lock().unwrap();
        if k.restore(&bytes) {
            eprintln!("restored basis={} from {state_path}", k.basis_len());
            pruned = true;
        }
    }
    if pruned {
        persist(&kernel, &state_path);
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

    let mesh = Mesh {
        cover: Arc::new(Mutex::new(Cover::new())),
        rooms: Arc::new(Mutex::new(Keyed::new())),
        addrs: Arc::new(Mutex::new(Keyed::new())),
        vk: hypha_key.vk_bytes(),
        sk: hypha_key.sk.clone(),
        bind: addr.clone(),
        rtt: Arc::new(Mutex::new(std::time::Duration::ZERO)),
        last_roots: Arc::new(Mutex::new(Keyed::new())),
    };
    rebuild_own_cover(&kernel, &mesh);
    let join_deadline = mesh_deadline(&mesh);
    for p in &join_peers {
        if let Some((reply, dt)) = tell_peer(p, dialogue::encode_digests_request(&mesh.vk), join_deadline) {
            note_rtt(&mesh, dt);
            let _ = admit_peer_digest(&mesh, &reply);
        }
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
        let mut listed = 0usize;
        let mut tasked = 0usize;
        for f in &files {
            let Ok(text) = std::fs::read_to_string(f) else { continue };
            let Some(task) = mycelium::arc::parse_task(&text) else { continue };
            let mut here = false;
            for (a, b) in &task.train {
                if let Some((mut chain, _, frame)) = mycelium::arc::refine_delta(a, b) {
                    chain.sort_unstable();
                    let _key = mycelium::transducer::element::listing_key(&chain, &frame);
                    let residual = mycelium::transducer::element::from_residual_framed(&chain, &frame);
                    let inv = mycelium::transducer::stalk::invariant_of(&residual);
                    if !homes_here(&mesh, &kernel, &inv, &residual.magnitude()) {
                        continue;
                    }
                    here = true;
                    let key = mycelium::transducer::element::listing_key(&chain, &frame);
                    let tid = mycelium::registry::task_id(&task);
                    if kernel.lock().unwrap().intake(&chain, &frame) {
                        listed += 1;
                        kernel.lock().unwrap().bind_listing_task(key, tid);
                    }
                }
            }
            if here {
                registry.lock().unwrap().register(task);
                tasked += 1;
            }
        }
        let self_room = room64(kernel.lock().unwrap().lane_room());
        kernel.lock().unwrap().unlist_foreign(|p| {
            let inv = mycelium::transducer::stalk::invariant_of(p);
            let c = mesh.cover.lock().unwrap();
            let r = mesh.rooms.lock().unwrap();
            let a = mesh.addrs.lock().unwrap();
            home(&c, &inv, &p.magnitude(), &r, &a, Some((&mesh.vk, &self_room, &mesh.bind))).0 == mesh.vk
        });
        let restored = std::fs::read(format!("{state_path}.registry"))
            .map(|b| registry.lock().unwrap().overlay(&b))
            .unwrap_or(0);
        eprintln!(
            "corpus registered: {tasked} home tasks from {corpus_dir} ({} unsolved, {listed} demands listed, {restored} settled states restored)",
            registry.lock().unwrap().unsolved(),
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
    let syllabus = Arc::new(Mutex::new(Syllabus {
        order: Vec::new(),
        at: Keyed::new(),
        served: Keyed::new(),
    }));
    {
        let seed = kernel.lock().unwrap().syllabus_seed();
        let mut sy = syllabus.lock().unwrap();
        // index: ruled strata, coarsest first
        for addr in seed {
            if !sy.at.contains(&addr) {
                sy.at.insert(addr, ());
                sy.order.push(addr);
            }
        }
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
    {
        let rtt = Arc::clone(&mesh.rtt);
        std::thread::spawn(move || {
            for (addr, body) in nerve_rx {
                let deadline = mycelium::physics::transport_deadline(
                    rtt.lock().map(|t| *t).unwrap_or_default(),
                );
                match tell_peer(&addr, body, deadline) {
                    Some((reply, dt)) => {
                        if let Ok(mut t) = rtt.lock() {
                            *t = if dt.is_zero() { std::time::Duration::from_micros(1) } else { dt };
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
                    None => eprintln!("  nerve → {addr}: no answer"),
                }
            }
        });
    }
    // The roots this book already holds settled are announced to the nerve
    // at boot, so a family proven here is re-spoken on every peer's book.
    let settled = registry.lock().map(|r| r.settled_ids()).unwrap_or_default();
    let boot_stamp = tuniversal.ticks();
    for t in &settled {
        let hear = {
            let mut g = roots.lock().unwrap();
            note_root(&mut g, *t, id, boot_stamp)
        };
        match hear {
            RootHear::Known => {}
            RootHear::First | RootHear::Second => {
                speak_root(&router, &nerve_tx, &nerve_id, t, &id, None, boot_stamp);
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
        let tu = tuniversal;
        let state_path = state_path.clone();
        let rtt = Arc::clone(&mesh.rtt);
        // Persistence is PACED: topological volume (cards, fibre, listings)
        // and a dwell of measured transports. K is not a persist event.
        let mut persisted = kernel.lock().map(|k| k.volume()).unwrap_or_default();
        let mut last_persist = Instant::now();
        std::thread::spawn(move || loop {
            let rtt_now = rtt.lock().map(|t| *t).unwrap_or_default();
            let work = match synth_rx.recv_timeout(mycelium::physics::transport_deadline(rtt_now)) {
                Ok(w) => w,
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    mycelium::synthesis::consolidate(&kernel);
                    let live = kernel.lock().map(|k| k.volume()).unwrap_or_default();
                    if live != persisted && last_persist.elapsed() >= mycelium::physics::persist_dwell(rtt_now) {
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
                        if after != persisted && last_persist.elapsed() >= mycelium::physics::persist_dwell(rtt_now) {
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
    {
        let kernel = Arc::clone(&kernel);
        let mesh = mesh.clone();
        let join = join_peers.clone();
        std::thread::spawn(move || {
            let ratio = mycelium::physics::aperture_cycle() as u64;
            let mut wait = mycelium::physics::from_transport(mesh_rtt(&mesh), 1);
            let mut last_moved = Instant::now();
            loop {
                std::thread::sleep(wait);
                if let Ok(mut k) = kernel.lock() {
                    k.court_stroke();
                }
                rebuild_own_cover(&kernel, &mesh);
                let mut dests: mycelium::network::nerve::Keyed<String> = mycelium::network::nerve::Keyed::new();
                for p in &join {
                    if p != &mesh.bind {
                        dests.insert(*blake3::hash(p.as_bytes()).as_bytes(), p.clone());
                    }
                }
                if let Ok(a) = mesh.addrs.lock() {
                    for (id, addr) in a.fiber(&mycelium::network::nerve::ROOT) {
                        if addr != &mesh.bind {
                            dests.insert(*id, addr.clone());
                        }
                    }
                }
                if dests.is_empty() {
                    wait = mycelium::physics::from_transport(mesh_rtt(&mesh), ratio);
                    continue;
                }
                let req = dialogue::encode_digests_request(&mesh.vk);
                let deadline = mesh_deadline(&mesh);
                let mut any_moved = false;
                let dest_list: Vec<String> = dests.fiber(&mycelium::network::nerve::ROOT).map(|(_, a)| a.clone()).collect();
                for d in dest_list {
                    match tell_peer(&d, req.clone(), deadline) {
                        Some((reply, dt)) => {
                            note_rtt(&mesh, dt);
                            if !admit_peer_digest(&mesh, &reply) {
                                continue;
                            }
                            let Some((_, _, cover_roots, ruled_root, vk_peer, _, _)) =
                                dialogue::decode_cover_digest(&reply)
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
                            let (local, charts) = {
                                let mut k = match kernel.lock() {
                                    Ok(g) => g,
                                    Err(_) => continue,
                                };
                                k.confirm_peer_holds(&remote);
                                if unchanged {
                                    continue;
                                }
                                let local = k.ruled_digests();
                                let discord = mycelium::transducer::sheaf::discordant(&remote, &local);
                                if discord.is_empty() {
                                    continue;
                                }
                                let charts: Vec<_> = discord
                                    .iter()
                                    .map(|s| (s.inv.clone(), k.fetch_have(&s.inv)))
                                    .collect();
                                (local, charts)
                            };
                            let _ = local;
                            any_moved = true;
                            let fetch = dialogue::encode_fetch(
                                &mesh.vk,
                                &charts,
                                mycelium::network::wire::frame_budget(),
                            );
                            if let Some((page, dt2)) = tell_peer(&d, fetch, mesh_deadline(&mesh)) {
                                note_rtt(&mesh, dt2);
                                for w in dialogue::decode_syllabus_reply(&page).unwrap_or_default() {
                                    let mut p = 0usize;
                                    if let Some(spoken) = mycelium::transducer::element::Element::from_bytes(&w, &mut p)
                                    {
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
                            }
                        }
                        None => {}
                    }
                }
                let rtt = mesh_rtt(&mesh);
                if any_moved {
                    last_moved = Instant::now();
                    wait = mycelium::physics::from_transport(rtt, 1);
                } else {
                    wait = last_moved
                        .elapsed()
                        .saturating_mul(ratio as u32)
                        .max(mycelium::physics::from_transport(rtt, 1));
                }
            }
        });
    }
    for _ in 0..(cores / 2).max(1) {
        let rx = Arc::clone(&reflex_rx);
        let door = door.clone();
        std::thread::spawn(move || loop {
            let job = match rx.lock() {
                Ok(r) => r.recv(),
                Err(_) => break,
            };
            match job {
                Ok(job) => {
                    let ReflexJob { mut stream } = job;
                    loop {
                        arm_deadline(&stream, mesh_deadline(&door.mesh));
                        let Some(frame) = read_frame(&mut stream) else { break };
                        let kind = frame.body.first().copied().unwrap_or(0xff);
                        let t0 = std::time::Instant::now();
                        judge(&mut stream, frame, &door);
                        eprintln!("  reflex act {kind:02x} {} ms", t0.elapsed().as_millis());
                    }
                }
                Err(_) => break,
            }
        });
    }

    for stream in listener.incoming() {
        let Ok(s) = stream else { continue };
        let _ = s.set_nodelay(true);
        arm_deadline(&s, mesh_deadline(&mesh));
        if let Err(e) = reflex_tx.try_send(ReflexJob { stream: s }) {
            let job = match e {
                std::sync::mpsc::TrySendError::Full(j) | std::sync::mpsc::TrySendError::Disconnected(j) => j,
            };
            let mut s = job.stream;
            let body = dialogue::encode_deferral(outcome_code(&Outcome::GermPending));
            let _ = s.write_all(&Frame { lane: mycelium::network::wire::Stratum::Spatial, body }.encode().unwrap_or_default());
        }
    }
}

