// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! mycelium-server — a HYPHA in the Čech-nerve mesh (never a star). Each hypha
//! runs its OWN Kernel (one eye per node). Identity is `home/hypha.key`; bind,
//! peers, genesis, and corpora live under `--home`. Usage:
//!   mycelium-server --home DIR
use mycelium::anastome::dialogue;
use mycelium::cognition::{decode_percept, outcome_code, Kernel, Outcome};
use mycelium::court::identity::{parse_hex32, parse_hex64, HyphaKey};
use mycelium::network::mesh::{self};
use mycelium::network::nerve::{home, Cover, Keyed, Router};
use mycelium::network::wire::Frame;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

fn arm_deadline(s: &TcpStream) {
    let _ = s.set_read_timeout(None);
    let _ = s.set_write_timeout(None);
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
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    Ok(PathBuf::from(home).join(".mycelium"))
}

struct Genesis {
    hypha: Vec<([u8; 32], [u64; 4])>,
    voice: Vec<([u8; 32], [u64; 4])>,
}

fn corpus_hash(dir: &Path) -> Option<[u8; 32]> {
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .ok()?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    files.sort();
    let mut chunks: Vec<Vec<u8>> = Vec::new();
    for f in &files {
        let rel = f.strip_prefix(dir).unwrap_or(f.as_path());
        chunks.push(rel.to_string_lossy().as_bytes().to_vec());
        chunks.push(std::fs::read(f).ok()?);
    }
    Some(mycelium::network::wire::integrity_hash(chunks.iter().map(|c| c.as_slice())))
}

fn parse_manifest(path: &Path) -> Option<(PathBuf, [u8; 32], [u8; 64])> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut lines = text
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'));
    let dir = PathBuf::from(lines.next()?);
    let hash = parse_hex32(lines.next()?)?;
    let sig = parse_hex64(lines.next()?)?;
    Some((dir, hash, sig))
}

fn resolved_bind(local: std::net::SocketAddr) -> String {
    if local.ip().is_unspecified() {
        format!("127.0.0.1:{}", local.port())
    } else {
        local.to_string()
    }
}

fn speak_root_locked(
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

fn persist_if_due(kernel: &Mutex<Kernel>, state_path: &str) {
    let due = kernel.lock().ok().is_some_and(|mut k| k.take_persist());
    if due {
        persist(kernel, state_path);
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
    order: Vec<Vec<u8>>,
    at: Keyed<()>,
    served: Keyed<usize>,
}

/// Push newly ruled addresses into the served stream. Court_stroke admits
/// onto the ruled cover; this is the teaching organ that drip feeds them.
fn syllabus_admit(sy: &mut Syllabus, addrs: impl IntoIterator<Item = impl AsRef<[u8]>>) {
    for addr in addrs {
        let addr = addr.as_ref();
        if !sy.at.contains(addr) {
            sy.at.insert(addr, ());
            sy.order.push(addr.to_vec());
        }
    }
}

#[derive(Clone)]
struct Door {
    kernel: Arc<Mutex<Kernel>>,
    mesh: Mesh,
    router: Arc<Mutex<Router>>,
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
fn note_root(roots: &mut Keyed<Keyed<u64>>, task: impl AsRef<[u8]>, solver: [u8; 32], stamp: u64) -> RootHear {
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

fn speak_root(
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

/// Speak one act to a peer on the nerve, bounded by the transport deadline;
/// the reply, if any, is not needed. Silence is the peer's impedance.
fn tell_peer(addr: &str, body: Vec<u8>) -> Option<(Vec<u8>, u64)> {
    let mut s = std::net::TcpStream::connect(addr).ok()?;
    let _ = s.set_nodelay(true);
    // A PEER THAT SAYS NOTHING IS SILENT, NOT AUTHORITATIVE. `read_exact`
    // with no timeout makes a peer that writes nothing an unbounded wait,
    // and one such peer stops this thread and every act it was going to
    // perform -- which is what a frame that failed to encode on the far
    // side produced: an empty write, and a caller waiting on five bytes
    // that never came. Silence is a refusal and is counted as one.
    let _ = s.set_read_timeout(Some(std::time::Duration::from_secs(20)));
    let _ = s.set_write_timeout(Some(std::time::Duration::from_secs(20)));
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
    Some((reply, 1))
}

struct Mesh {
    cover: Arc<Mutex<Cover>>,
    rooms: Arc<Mutex<Keyed<[u64; 4]>>>,
    addrs: Arc<Mutex<Keyed<String>>>,
    vk: [u8; 32],
    sk: ed25519_dalek::SigningKey,
    bind: String,
    rtt: Arc<Mutex<Keyed<u64>>>,
    /// The last aperture of measured round trips per peer, in strokes.
    rtt_window: Arc<Mutex<Keyed<Vec<u64>>>>,
    last_roots: Arc<Mutex<Keyed<([[u8; 32]; 4], [u8; 32])>>>,
    /// This node's own `(H_genesis, E_origin)`, the pair 47.2 step 1 tests
    /// every signed frame against. `None` is UNMEASURED, not zero: a node
    /// that has not adopted a genesis admits no receipt at all rather than
    /// admitting everything under a zero sentinel.
    origin: Arc<Mutex<Option<([u8; 32], u32)>>>,
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

/// The floor of the last aperture of measured round trips to a peer.
fn rtt_floor(mesh: &Mesh, id: &[u8]) -> Option<u64> {
    mesh.rtt_window
        .lock()
        .ok()
        .and_then(|w| w.get(id).and_then(|ring| ring.iter().copied().min()))
}

/// A measured round trip in strokes enters the peer's window.
fn note_rtt_ok(mesh: &Mesh, id: impl AsRef<[u8]>, sample: u64) {
    let id = id.as_ref();
    let n = mycelium::physics::aperture_cycle().max(1) as usize;
    if let Ok(mut w) = mesh.rtt_window.lock() {
        let mut ring = w.get(id).cloned().unwrap_or_default();
        ring.push(sample);
        if ring.len() > n {
            ring.remove(0);
        }
        w.insert(id, ring);
    }
    let cap = rtt_floor(mesh, id)
        .map(mycelium::physics::persist_dwell)
        .unwrap_or_else(|| mycelium::physics::persist_dwell(sample));
    if let Ok(mut m) = mesh.rtt.lock() {
        m.insert(id, sample.min(cap));
    }
}

/// A timeout raises the peer's RTT by the aperture ratio, capped at an
/// aperture of the measured floor: a dead peer's deadline is a ratio of
/// what its wire measured, never a wall clock and never unbounded.
fn note_rtt_timeout(mesh: &Mesh, id: impl AsRef<[u8]>) {
    let id = id.as_ref();
    let ratio = mycelium::physics::aperture_cycle().max(2);
    let Some(floor) = rtt_floor(mesh, id) else {
        return;
    };
    let cap = mycelium::physics::persist_dwell(floor);
    if let Ok(mut m) = mesh.rtt.lock() {
        let Some(cur) = m.get(id).copied() else {
            return;
        };
        m.insert(id, cur.saturating_mul(ratio as u64).min(cap));
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
    let mut out =
        dialogue::encode_cover_digest(&charts, &room, &cover_roots, &ruled_root, &mesh.vk, &mesh.bind, &sig);
    // 47.1: the signer's causal position travels with its signature. The
    // head receipt and the prefix it is signed over are appended, so the
    // peer can run 47.2 and cite this receipt as a parent of its next step.
    // A node with no genesis sends none: it has no origin to claim.
    let origin = mesh.origin.lock().ok().and_then(|o| *o);
    if let (Some((genesis, epoch)), Ok(k)) = (origin, kernel.lock()) {
        if let Some(head) = k.chain().receipts().last().cloned() {
            let prefix = mycelium::court::admission::SignedPrefix {
                genesis,
                epoch,
                prev_receipt_hash: head.prev_receipt_hash,
                k: head.step_index,
                height: head.height,
                chart: k.active_cover().iter().flat_map(|(_, inv)| inv.0.clone()).collect(),
                payload: ruled_root.to_vec(),
            };
            let sig = {
                use ed25519_dalek::Signer;
                mesh.sk.sign(&prefix.bytes()).to_bytes()
            };
            out.extend(dialogue::encode_receipt_section(&prefix, &sig, &head));
        }
    }
    out
}

/// Why one peer's digest carried no admission. A `None` return with no
/// reason is the rule-11 fault itself: an admission path that cannot say
/// whether it refused, or never ran, is UNMEASURED, and a fleet reading
/// zero admitted receipts cannot tell the two apart.
#[derive(Clone, Copy, PartialEq, Eq)]
enum NoAdmission {
    /// The peer's digest ends where a section would start: it sent none.
    NoSection,
    /// A section is there and does not parse whole (rule 11: refused).
    Malformed,
    /// This node has adopted no origin, so it has nothing to compare.
    NoOrigin,
}

/// 47.2 on the voice: run admission over the receipt section a peer
/// appended to its digest, and cite what admits.
///
/// The signature (step 2) is checked HERE, over exactly `prefix.bytes()`,
/// before the mind is asked: `Kernel::admit_receipt` is deliberately not
/// given a key, so no call site can satisfy admission by verifying bytes
/// and skipping the chain.
fn admit_peer_receipt(
    kernel: &Mutex<Kernel>,
    mesh: &Mesh,
    body: &[u8],
) -> Result<mycelium::court::admission::Admitted, NoAdmission> {
    use mycelium::court::admission::Admitted;
    let at = dialogue::cover_digest_end(body).ok_or(NoAdmission::Malformed)?;
    if at >= body.len() {
        return Err(NoAdmission::NoSection);
    }
    let (prefix, sig, receipt) =
        dialogue::decode_receipt_section(body, at).ok_or(NoAdmission::Malformed)?;
    if !mycelium::court::identity::verify(&receipt.vk, &prefix.bytes(), &sig) {
        return Ok(Admitted::Refused("bad signature"));
    }
    // 45.5: a node with NO origin adopts the one its bootstrap peer signs
    // for. That is the provisioning channel the chapter names -- the peer
    // list is how a node is told which mesh it belongs to, and the genesis
    // travels the same way. A node that already holds an origin never
    // adopts another: a peer claiming a different one is REFUSE_ORIGIN
    // below, which is the whole point of the check.
    {
        let mut o = mesh.origin.lock().map_err(|_| NoAdmission::Malformed)?;
        if o.is_none() {
            *o = Some((prefix.genesis, prefix.epoch));
            eprintln!(
                "  court genesis {:02x}{:02x}… epoch {} adopted from {:02x}{:02x}…",
                prefix.genesis[0], prefix.genesis[1], prefix.epoch, receipt.vk[0], receipt.vk[1]
            );
        }
    }
    let origin = mesh
        .origin
        .lock()
        .ok()
        .and_then(|o| *o)
        .ok_or(NoAdmission::NoOrigin)?;
    let mut k = kernel.lock().map_err(|_| NoAdmission::Malformed)?;
    Ok(k.admit_receipt(origin, &prefix, &receipt))
}

/// One exchange cycle's admission tally, written with the 34.2 organ line.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
struct AdmissionTally {
    parent: usize,
    gap: usize,
    stale: usize,
    origin: usize,
    signature: usize,
    receipt: usize,
    equivocation: usize,
    no_section: usize,
    malformed: usize,
    no_origin: usize,
}

impl AdmissionTally {
    fn note(&mut self, r: Result<mycelium::court::admission::Admitted, NoAdmission>) {
        use mycelium::court::admission::Admitted;
        match r {
            Ok(Admitted::Parent(_)) => self.parent += 1,
            Ok(Admitted::Gap { .. }) => self.gap += 1,
            Ok(Admitted::Stale) => self.stale += 1,
            Ok(Admitted::Equivocation { .. }) => self.equivocation += 1,
            Ok(Admitted::Refused(why)) => {
                if why == mycelium::court::admission::REFUSE_ORIGIN {
                    self.origin += 1;
                } else if why == mycelium::court::admission::REFUSE_RECEIPT {
                    self.receipt += 1;
                } else {
                    self.signature += 1;
                }
            }
            Err(NoAdmission::NoSection) => self.no_section += 1,
            Err(NoAdmission::Malformed) => self.malformed += 1,
            Err(NoAdmission::NoOrigin) => self.no_origin += 1,
        }
    }

    fn line(&self) -> String {
        format!(
            "  court receipts parent {} gap {} stale {} equivocation {} \
refused-origin {} refused-signature {} refused-receipt {} \
none {} malformed {} no-origin {}",
            self.parent,
            self.gap,
            self.stale,
            self.equivocation,
            self.origin,
            self.signature,
            self.receipt,
            self.no_section,
            self.malformed,
            self.no_origin
        )
    }
}

fn admit_peer_digest(mesh: &Mesh, body: &[u8], router: &Mutex<Router>) -> bool {
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
        a.insert(vk, addr.clone());
    }
    if let Ok(mut r) = router.lock() {
        r.insert(vk, addr);
    }
    true
}

/// Control acts that used to run on the accept thread. The listener never
/// holds the kernel lock; these run on a reflex worker.
fn serve_control(s: &mut TcpStream, frame: &Frame, door: &Door) -> bool {
    let Some((&kind, _rest)) = frame.body.split_first() else { return false };
    if kind == dialogue::ACT_ROOT {
        let Some((from, task, solver, spoken_at)) = dialogue::decode_root(&frame.body) else { return true };
        if !door.router.lock().map(|r| r.knows(&from)).unwrap_or(false) {
            let _ = s.write_all(&Frame { lane: frame.lane, body: Vec::new() }.encode().unwrap_or_default());
            return true;
        }
        let hear = {
            let mut roots = door.roots.lock().unwrap();
            note_root(&mut roots, &task, solver, spoken_at)
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
                speak_root_locked(&door.router, &door.nerve_tx, &door.id, &task, &solver, Some(from), spoken_at);
            }
            RootHear::Second => {
                let voices: Vec<Vec<u8>> = {
                    let sy = door.syllabus.lock().unwrap();
                    sy.served.fiber(mycelium::network::nerve::ROOT).map(|(v, _)| v).collect()
                };
                if let Ok(mut sy) = door.syllabus.lock() {
                    for v in voices {
                        sy.served.insert(v, 0);
                    }
                }
                eprintln!("  ROOT CONSILIENT task {:02x}{:02x}… two voices; syllabus rewound", task[0], task[1]);
                speak_root_locked(&door.router, &door.nerve_tx, &door.id, &task, &solver, Some(from), spoken_at);
            }
        }
        let reply = {
            let roots = door.roots.lock().unwrap();
            dialogue::encode_root_reply(
                roots.get(task.as_bytes()).and_then(|v| {
                    v.fiber(mycelium::network::nerve::ROOT)
                        .next()
                        .and_then(|(k, _)| <[u8; 32]>::try_from(k.as_slice()).ok())
                }),
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
                items.into_iter().filter(|(addr, fp, scene)| k.deposit_evidence(addr.clone(), fp.clone(), scene.clone())).count()
            };
            let reply = Frame { lane: frame.lane, body: vec![accepted.min(255) as u8] };
            let _ = s.write_all(&reply.encode().unwrap_or_default());
            return true;
        }
        let Some(_evoice) = dialogue::decode_evidence_request(&frame.body) else { return true };
        let items: Vec<_> = {
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
        let Some((svoice, tid, outputs, rule_bytes)) = dialogue::decode_solve(&frame.body) else { return true };
        let rule_elem = rule_bytes.as_ref().and_then(|b| {
            let mut p = 0usize;
            mycelium::transducer::element::Element::from_bytes(b, &mut p)
        });
        let withheld = {
            let reg = door.registry.lock().unwrap();
            reg.withheld(&tid)
        };
        if let Some(rule) = rule_elem.as_ref() {
            // The rule's cited forms resolve through this hypha's ruled cover.
            let matches = match door.kernel.lock() {
                Ok(k) => k.rule_matches_tests(rule, &withheld),
                Err(_) => false,
            };
            if !matches {
                let reply = Frame { lane: frame.lane, body: dialogue::encode_solve_receipt(dialogue::SOLVE_NOTHING, &[0; 4]) };
                let _ = s.write_all(&reply.encode().unwrap_or_default());
                return true;
            }
        }
        let (code, mu) = {
            let mut reg = door.registry.lock().unwrap();
            match reg.settle(&tid, &outputs) {
                mycelium::registry::SolveYield::Bounty(tension) => {
                    let snap = reg.snapshot();
                    let dir = std::path::Path::new(&door.state_path)
                        .parent()
                        .map(|p| p.join("corpora"))
                        .unwrap_or_else(|| std::path::PathBuf::from("corpora"));
                    let _ = std::fs::create_dir_all(&dir);
                    let tmp = dir.join("registry.tmp");
                    if std::fs::write(&tmp, &snap).is_ok() {
                        let _ = std::fs::rename(&tmp, dir.join("registry"));
                    }
                    drop(reg);
                    let paid = {
                        let mut k = door.kernel.lock().unwrap();
                        k.pay_solve(&tid, svoice, tension, rule_elem.as_ref(), &withheld)
                    };
                    let hear = {
                        let mut roots = door.roots.lock().unwrap();
                        note_root(&mut roots, &tid, svoice, 0)
                    };
                    match hear {
                        RootHear::Known => {}
                        RootHear::First | RootHear::Second => {
                            eprintln!("  ROOT task {:02x}{:02x}… proven by voice {:02x}; spoken to its key's next hop", tid[0], tid[1], svoice[0]);
                            speak_root_locked(&door.router, &door.nerve_tx, &door.id, &tid, &svoice, None, 0);
                        }
                    }
                    if matches!(hear, RootHear::Second) {
                        let voices: Vec<Vec<u8>> = door.syllabus.lock().unwrap().served.fiber(mycelium::network::nerve::ROOT).map(|(v, _)| v).collect();
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
                        k.pay_solve(&tid, svoice, [0, 0, 1, 0], rule_elem.as_ref(), &withheld)
                    };
                    let hear = {
                        let mut roots = door.roots.lock().unwrap();
                        note_root(&mut roots, &tid, svoice, 0)
                    };
                    if matches!(hear, RootHear::Second) {
                        let voices: Vec<Vec<u8>> = door.syllabus.lock().unwrap().served.fiber(mycelium::network::nerve::ROOT).map(|(v, _)| v).collect();
                        if let Ok(mut sy) = door.syllabus.lock() {
                            for v in voices { sy.served.insert(v, 0); }
                        }
                        speak_root_locked(&door.router, &door.nerve_tx, &door.id, &tid, &svoice, None, 0);
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
            for p in door.join_peers.iter() {
                if !nerve_addrs.iter().any(|a| a == p) {
                    nerve_addrs.push(p.clone());
                }
            }
            if let Ok(addrs) = door.mesh.addrs.lock() {
                for (_, a) in addrs.fiber(&mycelium::network::nerve::ROOT) {
                    if !nerve_addrs.iter().any(|x| x == a) {
                        nerve_addrs.push(a.clone());
                    }
                }
            }
            let reply = Frame { lane: frame.lane, body: dialogue::encode_peers_reply(&nerve_addrs) };
            let _ = s.write_all(&reply.encode().unwrap_or_default());
        }
        return true;
    }
    // THE SECOND ORGAN ARRIVES HERE. A home Offers a first-vantage germ;
    // the peer replays it and glues it into its own cover, and its ruled
    // root moves, which is the evidence the home installs on
    // (MU-ENGINEERING §3). ACT_OFFER had an encoder and a decoder and no
    // caller at either end: the home marked germs `offered` in a carrier
    // nothing read, so no peer ever saw a provisional, no root ever moved,
    // and every percept sat at `germ pending (one voice)` forever with
    // basis 0. The gate was never the germ quorum; it was that the offer
    // never left the process.
    if kind == dialogue::ACT_OFFER {
        let Some((_addr, elem_bytes, _voices, _defl, _k, _sig, _inst)) =
            dialogue::decode_offer(&frame.body)
        else {
            return true;
        };
        let mut pos = 0usize;
        // THE WITNESS RE-DERIVES. This hypha replays the offered form on its
        // OWN tapes and reaches a form of its own; that form is what 47.3's
        // second vantage is, and until now it was reduced to one bit on the
        // way back. The reply carries it, so the home compares two
        // re-derivations by `same_vertex` (M1) rather than assuming that a
        // peer which did not slash agreed.
        let (glued, reached) =
            match mycelium::transducer::element::Element::from_bytes(&elem_bytes, &mut pos) {
                Some(word) => {
                    let mut cover = mycelium::network::nerve::Cover::new();
                    match door.kernel.lock() {
                        Ok(mut k) => {
                            let vk = door.mesh.vk;
                            let g = k.glue_fetched(word.clone(), vk, vk, &mut cover);
                            let ok = !matches!(g, mycelium::cognition::Glue::Slashed(_));
                            // What this court holds at that address after
                            // replay, which is its own reading of the form.
                            let mine = k.word_at(&word.address()).cloned().unwrap_or(word);
                            (ok, ok.then_some(mine))
                        }
                        Err(_) => (false, None),
                    }
                }
                None => (false, None),
            };
        let mut rb = vec![glued as u8];
        if let Some(w) = reached.as_ref() {
            rb.extend_from_slice(&w.to_bytes());
        }
        // The witness's form may not fit the frame budget. A reply that
        // cannot carry it still ANSWERS -- with the verdict bit alone, which
        // the home reads as "no form travelled" and holds as unmeasured --
        // because writing nothing leaves the home waiting on a frame that
        // will never come. Rule 11: refuse the payload, never the reply.
        let reply = match (Frame { lane: frame.lane, body: rb }).encode() {
            Some(bytes) => bytes,
            None => Frame { lane: frame.lane, body: vec![glued as u8] }
                .encode()
                .unwrap_or_else(|| vec![frame.lane.as_u8(), 0, 0, 0, 0]),
        };
        let _ = s.write_all(&reply);
        return true;
    }
    if kind == dialogue::ACT_ADOPT {
        let Some((ratifier, elem_bytes)) = dialogue::decode_adopt(&frame.body) else { return true };
        let mut pos = 0usize;
        let queued = mycelium::transducer::element::Element::from_bytes(&elem_bytes, &mut pos)
            .map(|word| {
                let addr = word.address();
                if let Ok(mut sy) = door.syllabus.lock() {
                    syllabus_admit(&mut sy, [addr]);
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
        let mut missed: Vec<Vec<u8>> = Vec::new();
        let seed = door.kernel.lock().ok().map(|k| k.syllabus_seed()).unwrap_or_default();
        let (cursor, order) = {
            let mut sy = door.syllabus.lock().unwrap();
            syllabus_admit(&mut sy, seed);
            if sy.served.get(&svoice).is_none() {
                sy.served.insert(svoice, 0);
            }
            (*sy.served.get(&svoice).unwrap(), sy.order.clone())
        };
        let (words, classes, k_now, height, served_addrs): (Vec<Vec<u8>>, Vec<dialogue::ClassPage>, u64, u64, Vec<Vec<u8>>) = {
            let k = door.kernel.lock().unwrap();
            let free: u64 = k.physics().reserve.iter().sum();
            let depth = (1 + (free as usize * 7) / (4 * mycelium::physics::reserve_max() as usize)).min(8);
            let mut out = Vec::new();
            let mut served_addrs = Vec::new();
            let mut next = cursor;
            while next < order.len() && out.len() < depth {
                let addr = order[next].clone();
                match k.word_at(&addr) {
                    Some(w) => {
                        served_addrs.push(addr);
                        out.push(w.to_bytes());
                    }
                    None => missed.push(addr),
                }
                next += 1;
            }
            let classes = k.class_records(&mycelium::transducer::stalk::Invariant::default());
            let k_now = k.k_step();
            let height = k.book_height();
            drop(k);
            if let Ok(mut sy) = door.syllabus.lock() {
                sy.served.insert(svoice, next);
            }
            (out, classes, k_now, height, served_addrs)
        };
        for addr in &served_addrs {
            eprintln!(
                "  court syllabus {:02x}{:02x}… to {:02x}{:02x}… receipt K {k_now} h {height}",
                addr[0], addr[1], svoice[0], svoice[1]
            );
        }
        if !missed.is_empty() {
            let mut sy = door.syllabus.lock().unwrap();
            let ahead = *sy.served.get(&svoice).unwrap_or(&0);
            for addr in missed {
                let mut tail: Keyed<()> = Keyed::new();
                for a in sy.order.get(ahead..).unwrap_or(&[]) {
                    tail.insert(a, ());
                }
                if !tail.contains(&addr) {
                    sy.order.push(addr);
                }
            }
        }
        let mut body = dialogue::encode_syllabus_reply(&words);
        dialogue::append_class_records(&mut body, &classes);
        let reply = Frame { lane: frame.lane, body };
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
                syllabus_admit(&mut sy, [addr]);
            }
        }
        let reply = Frame { lane: frame.lane, body: vec![applied as u8] };
        let _ = s.write_all(&reply.encode().unwrap_or_default());
        return true;
    }
    false
}

fn homes_here(mesh: &Mesh, kernel: &Mutex<Kernel>, inv: &mycelium::transducer::stalk::Invariant, mag: &[u64; 4]) -> bool {
    let room = match kernel.lock() {
        Ok(k) => room64(k.lane_room()),
        Err(_) => return false,
    };
    let c = mesh.cover.lock().unwrap();
    let r = mesh.rooms.lock().unwrap();
    let a = mesh.addrs.lock().unwrap();
    home(&c, inv, mag, &r, &a, Some((&mesh.vk, &room, &mesh.bind))).0 == mesh.vk
}

fn list_task(
    kernel: &Mutex<Kernel>,
    mesh: &Mesh,
    tid: impl AsRef<[u8]>,
    task: &mycelium::arc::Task,
    rehome: bool,
) {
    let tid = tid.as_ref();
    if let Ok(mut k) = kernel.lock() {
        k.bind_task_family(tid, task.train.clone());
    }
    for (a, b) in &task.train {
        let Some((mut chain, _, frame)) = mycelium::arc::refine_delta(a, b) else {
            continue;
        };
        chain.sort_unstable();
        let residual = mycelium::transducer::element::from_residual_framed(&chain, &frame);
        let inv = mycelium::transducer::stalk::invariant_of(&residual);
        let addr = residual.address();
        let here = homes_here(mesh, kernel, &inv, &residual.magnitude());
        let mut k = match kernel.lock() {
            Ok(g) => g,
            Err(_) => continue,
        };
        match here {
            true => {
                if k.intake(&chain, &frame) {
                    k.bind_listing_task(addr, tid);
                }
            }
            false if rehome => match k.has_listing(&addr) {
                true => k.relocate_listing(&addr),
                false => {}
            },
            false => {}
        }
    }
}

/// First-time listing under a stroke budget until the manifest is exhausted.
fn intake_corpus(
    kernel: &Mutex<Kernel>,
    mesh: &Mesh,
    registry: &Mutex<mycelium::registry::TaskRegistry>,
    budget: usize,
) {
    let batch = match registry.lock() {
        Ok(mut r) => r.next_intake_batch(budget),
        Err(_) => return,
    };
    for (tid, task) in batch {
        list_task(kernel, mesh, tid, &task, false);
    }
}

/// Cover-change walk: re-home listings whose key no longer (or now) homes here.
fn relocate_corpus(
    kernel: &Mutex<Kernel>,
    mesh: &Mesh,
    registry: &Mutex<mycelium::registry::TaskRegistry>,
    budget: usize,
) {
    let batch = match registry.lock() {
        Ok(mut r) => r.next_cover_batch(budget),
        Err(_) => return,
    };
    for (tid, task) in batch {
        list_task(kernel, mesh, tid, &task, true);
    }
}

fn demand_cover_home(mesh: &Mesh, kernel: &Mutex<Kernel>, chain: &[(i64, i64, u8, u8)]) -> ([u8; 32], String) {
    let residual = mycelium::transducer::element::from_residual(&chain.to_vec());
    let inv = mycelium::transducer::stalk::invariant_of(&residual);
    let mag = residual.magnitude();
    let locked = kernel.lock();
    let room = match locked.as_ref() {
        Ok(k) => Some(room64(k.lane_room())),
        Err(_) => None,
    };
    let self_id = room.as_ref().map(|room| (&mesh.vk, room, mesh.bind.as_str()));
    let c = mesh.cover.lock().unwrap();
    let r = mesh.rooms.lock().unwrap();
    let a = mesh.addrs.lock().unwrap();
    home(&c, &inv, &mag, &r, &a, self_id)
}

fn judge(stream: &mut TcpStream, frame: Frame, door: &Door) {
    let kernel = &door.kernel;
    let id = &door.id;
    let mesh = &door.mesh;
    if serve_control(stream, &frame, door) {
        return;
    }
    // A demand the client spoke here is forwarded once, to the key's home
    // on the cover trie. A frame that arrived as ACT_DEMAND_ROUTED is
    // judged and never forwarded — the loop a→b→c→a was that unwrap
    // becoming ACT_DEMAND and closest_excluding sending it on again.
    let arrived_routed = frame.body.first() == Some(&dialogue::ACT_DEMAND_ROUTED);
    let frame = match frame.body.first() {
        Some(&k) if k == dialogue::ACT_DEMAND_ROUTED => match dialogue::decode_routed_demand(&frame.body) {
            Some((origin, demand))
                if origin == mesh.vk
                    || mesh.addrs.lock().map(|a| a.contains(&origin)).unwrap_or(false) =>
            {
                Frame { lane: frame.lane, body: demand.to_vec() }
            }
            _ => return,
        },
        _ => frame,
    };
    let Some((&kind, rest)) = frame.body.split_first() else { return };
    // ── quote: the market view leaves under a brief lock; pricing and the
    // cohesion witness compute over the snapshot.
    if kind == dialogue::ACT_SPEECH {
        let Some((svoice, task, _tu, blobs)) = dialogue::decode_speech(&frame.body) else { return };
        let mut sentences = Vec::new();
        for b in &blobs {
            let mut p = 0usize;
            if let Some(e) = mycelium::transducer::element::Element::from_bytes(b, &mut p) {
                sentences.push(e);
            }
        }
        let codes = match kernel.lock() {
            Ok(mut k) => k.perceive_speech(svoice, &task, &sentences),
            Err(_) => return,
        };
        for (i, e) in sentences.iter().enumerate() {
            if codes.get(i).copied() == Some(dialogue::SPEECH_STOOD) {
                eprintln!(
                    "  speech stood {:02x}{:02x}… sectioned at home",
                    e.address()[0],
                    e.address()[1]
                );
            }
        }
        let body = dialogue::encode_speech_reply(&codes);
        let reply = Frame { lane: frame.lane, body };
        let _ = stream.write_all(&reply.encode().unwrap_or_default());
        return;
    }
    if kind == dialogue::ACT_QUOTE {
        let Some((qvoice, charts)) = dialogue::decode_quote_request(&frame.body) else { return };
        let Some((surface, priced, k_step, cohesion, height)) = mycelium::synthesis::quote(kernel, qvoice, &charts) else {
            return;
        };
        let items: Vec<dialogue::WorkItem> = priced
            .iter()
            .map(|(e, mu, open)| (e.address(), *mu, e.sub().to_vec(), open.clone()))
            .collect();
        // The frame budget bounds what one contact carries: items ride in
        // rank order until the next would not fit.
        let body = dialogue::encode_quote_reply_within(
            height,
            &surface,
            k_step,
            &items,
            mycelium::network::wire::frame_budget(),
        );
        let served = body.get(dialogue::QUOTE_COUNT_AT).copied().unwrap_or(0);
        if served > 0 {
            if let Ok(mut k) = kernel.lock() {
                k.grant_t_lease(qvoice);
            }
        }
        eprintln!(
            "  court book height {height} frontier {served} for {:02x}{:02x}…",
            qvoice[0], qvoice[1]
        );
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
        let mut classes: Vec<dialogue::ClassPage> = Vec::new();
        if let Ok(mut k) = kernel.lock() {
            let mut have_all: Vec<mycelium::transducer::a21::Address> = Vec::new();
            for (inv, have) in &charts {
                have_all.extend(have.iter().cloned());
                let page = k.serve_ruled(inv, have, budget);
                budget = budget.saturating_sub(page.iter().map(|w| w.len() + 4).sum::<usize>());
                words.extend(page);
                classes.extend(k.class_records(inv));
                if budget == 0 {
                    break;
                }
            }
            k.confirm_peer_holds_addrs(&have_all, fvoice);
        }
        if !words.is_empty() {
            eprintln!(
                "  fetch voice {:02x} ← {} words across {} charts",
                fvoice[0],
                words.len(),
                charts.len()
            );
        }
        let mut body = dialogue::encode_syllabus_reply(&words);
        dialogue::append_class_records(&mut body, &classes);
        let reply = Frame { lane: frame.lane, body };
        let _ = stream.write_all(&reply.encode().unwrap_or_default());
        return;
    }
    // ── probe: a tentative sentence against an open listing — replayed,
    // settled on graded descent or closure, never germinated.
    if kind == dialogue::ACT_PROBE {
        let Some((pvoice, target, cites)) = dialogue::decode_probe(&frame.body) else { return };
        let (code, paid, remaining) = mycelium::synthesis::probe(kernel, pvoice, &target, &cites);
        if matches!(code, dialogue::PROBE_CLOSED | dialogue::PROBE_DESCENT | dialogue::PROBE_SETTLED) {
            if let Ok(mut k) = kernel.lock() {
                k.release_t_lease(pvoice);
            }
        }
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
    let pbytes = if kind == dialogue::ACT_GOSSIP {
        let mut p = 0usize;
        let _ = mycelium::transducer::a21::take_addr(rest, &mut p);
        rest.get(p..).unwrap_or(&[])
    } else {
        rest
    };
    if kind == dialogue::ACT_DEMAND && !arrived_routed {
        if let Some((_, chain, _)) = dialogue::decode_demand(&frame.body) {
            let (home_vk, home_addr) = demand_cover_home(mesh, kernel, &chain);
            if home_vk != mesh.vk && home_addr != mesh.bind && !home_addr.is_empty() {
                let _ = door.nerve_tx.try_send((home_addr, dialogue::encode_routed_demand(id, &frame.body)));
            }
        }
    }
    let (out, voice) = if kind == dialogue::ACT_DEMAND {
        // A spoken coboundary with the speaker's own reduction — replayed.
        let Some((dvoice, chain, cites, k_voice)) = dialogue::decode_demand_at(&frame.body) else { return };
        (mycelium::synthesis::attest(kernel, &chain, dvoice, &cites, k_voice), dvoice)
    } else {
        // The client PERCEIVED and REDUCED; the JUDGE replays the proof.
        let Some((pvoice, a_bodies, b_bodies, frame, proof)) = decode_percept(pbytes) else { return };
        (
            mycelium::synthesis::adjudicate(kernel, &a_bodies, &b_bodies, pvoice, &frame, &proof),
            pvoice,
        )
    };
    // TEACH by the docket's standing of the cited word (anchored → taught),
    // never by sentence length. SETTLE: the consilience receipt.
    let (basis, flux, reserve, taught, settle, height) = {
        let k = kernel.lock().unwrap();
        let (taught, settle) = match &out {
            Outcome::Installed(i) => match k.basis().get(*i) {
                Some(g) => {
                    let addr = g.address();
                    let mu = mycelium::economy::price(&g.magnitude(), &k.physics().pressure);
                    (g.to_bytes(), Some((addr.clone(), k.voices_of(addr.as_bytes()), mu)))
                }
                None => (Vec::new(), None),
            },
            Outcome::Decomposed(word) => (k.taught_anchored(word), None),
            _ => (Vec::new(), None),
        };
        (
            (k.basis_len(), k.sections_total()),
            k.physics().metabolism_rate(),
            k.physics().reserve,
            taught,
            settle,
            k.book_height(),
        )
    };
    let label = match &out {
        Outcome::Decomposed(word) => format!("valid: decomposed → sentence {word:?} ({} generators)", word.len()),
        Outcome::GermPending => "valid: docketed, awaiting witness".to_string(),
        Outcome::Installed(i) => format!("valid: GERM INSTALLED #{i}"),
        Outcome::Sectioned(i) => format!("valid: section of #{i}"),
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
    persist_if_due(kernel, &door.state_path);
    // Reply: outcome | height u64 | pressure 4×u64 | taught (len16 | bytes) | settle?
    let mut body = dialogue::encode_outcome_reply(outcome_code(&out), height, &flux, &taught);
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
        let Some((_, a_bodies, b_bodies, extent, _)) = decode_percept(pbytes) else {
            return;
        };
        let fa: Vec<(i64, i64, u8)> = a_bodies.into_iter().flatten().collect();
        let fb: Vec<(i64, i64, u8)> = b_bodies.into_iter().flatten().collect();
        let chain = mycelium::transducer::germinate::germinate(&fa, &fb).unwrap_or_default();
        let key = mycelium::transducer::element::listing_key(&chain, &extent);
        let dest = door.router.lock().ok().and_then(|r| rendezvous(key.as_bytes(), id, &r));
        if let Some(dest) = dest {
            let mut g = vec![0x01u8];
            let _ = mycelium::transducer::a21::put_addr(&mut g, &key);
            g.extend_from_slice(pbytes);
            let gframe = Frame { lane: frame.lane, body: g }.encode().unwrap_or_default();
            if let Ok(mut ps) = TcpStream::connect(&dest) {
                let _ = ps.write_all(&gframe);
            }
        }
    }
}

/// The RENDEZVOUS for a content key: XOR-closest known hypha to the key's
/// bits. A listing is not hashed; its Address walks the trie.
fn rendezvous(key: &[u8], self_id: &[u8; 32], router: &Router) -> Option<String> {
    let dist = |idb: &[u8]| -> [u8; 32] {
        std::array::from_fn(|i| idb.get(i).copied().unwrap_or(0) ^ key.get(i).copied().unwrap_or(0))
    };
    match router.closest(key) {
        Some((pid, addr)) if dist(&pid) < dist(self_id) => Some(addr),
        _ => None,
    }
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
    let hypha_key = HyphaKey::load_or_mint(home.join("hypha.key").to_str().unwrap_or("hypha.key"));
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
        match mesh_rec.genesis {
            Some(_) => {
                mesh_rec.genesis = None;
                mesh_rec.epoch += 1;
                if let Err(e) = mesh_rec.save(&home, &hypha_key) {
                    eprintln!("[{tag}] recut refused: {e}");
                    std::process::exit(2);
                }
                eprintln!("[{tag}] recut: origin dropped, epoch {}", mesh_rec.epoch);
            }
            None => eprintln!("[{tag}] recut: no origin held, epoch {}", mesh_rec.epoch),
        }
        return;
    }
    match mesh_rec.genesis {
        None => eprintln!("[{tag}] genesis unwritten epoch {}", mesh_rec.epoch),
        Some(g) => eprintln!("[{tag}] genesis {:02x}{:02x}… epoch {}", g[0], g[1], mesh_rec.epoch),
    }
    {
        let k = Arc::clone(&kernel);
        let path = state_path.clone();
        let _ = ctrlc::set_handler(move || {
            persist(&k, &path);
            std::process::exit(0);
        });
    }
    if let Ok(bytes) = std::fs::read(&state_path) {
        let mut k = kernel.lock().unwrap();
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
        let mut k = kernel.lock().unwrap();
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
        let listener = TcpListener::bind("0.0.0.0:0").expect("bind");
        let resolved = resolved_bind(listener.local_addr().expect("local_addr"));
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
    let genesis_vks: Vec<[u8; 32]> = genesis.hypha.iter().map(|(v, _)| *v).collect();
    let mut cover = [0u64; 4];
    let mut cover_measured = false;
    if let Ok(rd) = std::fs::read_dir(home.join("corpora")) {
        let mut manifests: Vec<_> = rd.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| {
            p.extension().is_some_and(|x| x == "manifest")
        }).collect();
        manifests.sort();
        for man in manifests {
            let Some((corpus_dir, want, sig)) = parse_manifest(&man) else {
                eprintln!("[{tag}] corpus manifest {} unreadable", man.display());
                continue;
            };
            let Some(got) = corpus_hash(&corpus_dir) else {
                eprintln!("[{tag}] corpus {} unreadable", corpus_dir.display());
                continue;
            };
            if got != want {
                eprintln!("[{tag}] corpus {} hash mismatch", corpus_dir.display());
                continue;
            }
            let signers: Vec<[u8; 32]> = if genesis_vks.is_empty() { vec![vk] } else { genesis_vks.clone() };
            let signed = signers.iter().any(|s| mycelium::court::identity::verify(s, &want, &sig));
            if !signed {
                eprintln!("[{tag}] corpus {} signature refused", corpus_dir.display());
                continue;
            }
            let mut files: Vec<_> = std::fs::read_dir(&corpus_dir)
                .map(|rd| {
                    rd.filter_map(|e| e.ok().map(|e| e.path()))
                        .filter(|p| p.extension().is_some_and(|x| x == "json"))
                        .collect()
                })
                .unwrap_or_default();
            files.sort();
            let mut tasked = 0usize;
            for f in &files {
                let Ok(text) = std::fs::read_to_string(f) else { continue };
                let Some(task) = mycelium::arc::parse_task(&text) else { continue };
                if let Some(fam) = mycelium::arc::family_cover(&task.train) {
                    cover_measured = true;
                    mycelium::arc::cover_join(&mut cover, &fam);
                }
                registry.lock().unwrap().register(task);
                tasked += 1;
            }
            let restored = std::fs::read(home.join("corpora").join("registry"))
                .or_else(|_| std::fs::read(format!("{state_path}.registry")))
                .map(|b| registry.lock().unwrap().overlay(&b))
                .unwrap_or(0);
            eprintln!(
                "[{tag}] corpus registered: {tasked} tasks from {} ({} unsolved, {restored} settled states restored; intake continues under stroke budget)",
                corpus_dir.display(),
                registry.lock().unwrap().unsolved(),
            );
        }
    }
    {
        let mut k = kernel.lock().unwrap();
        if cover_measured {
            k.set_genesis_cover(cover);
            if k.treasury().bond_of(&vk).is_none() {
                let staked = k.enroll(vk, cover);
                eprintln!("[{tag}] genesis bond {cover:?}; staked {staked:?}");
            }
            for (who, _) in &genesis.voice {
                if k.treasury().bond_of(who).is_none() {
                    k.enroll(*who, cover);
                }
            }
        } else if k.treasury().bond_of(&vk).is_none() {
            eprintln!("[{tag}] unbonded; the corpus did not measure a cover");
        }
        let room = k.lane_room();
        let bond = k.treasury().bond_of(&vk);
        eprintln!(
            "[{tag}] room {room:?} = bond {bond:?} − occupancy {:?}",
            k.occupancy()
        );
    }

    let listener = match early_listener {
        Some(l) => l,
        None => TcpListener::bind(&addr).expect("bind"),
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
        let seed = kernel.lock().unwrap().syllabus_seed();
        let mut sy = syllabus.lock().unwrap();
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
    {
        let mesh_nerve = mesh.clone();
        std::thread::spawn(move || {
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
    // The roots this book already holds settled are announced to the nerve
    // at boot, so a family proven here is re-spoken on every peer's book.
    let settled = registry.lock().map(|r| r.settled_ids()).unwrap_or_default();
    let boot_stamp = 0u64;
    for t in &settled {
        let hear = {
            let mut g = roots.lock().unwrap();
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
    {
        let kernel = Arc::clone(&kernel);
        let mesh = mesh.clone();
        let join = join_peers.clone();
        let gen_home = home.clone();
        let epoch = mesh_rec.epoch;
        let genesis_written = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(
            mesh_rec.genesis.is_some(),
        ));
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
        let registry = Arc::clone(&registry);
        let router = Arc::clone(&router);
        let state_path = state_path.clone();
        let syllabus = Arc::clone(&syllabus);
        std::thread::spawn(move || {
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
                                        .map(|s| (s.inv.clone(), k.fetch_have(&s.inv)))
                                        .collect::<Vec<_>>()
                                } else {
                                    discord
                                        .iter()
                                        .map(|s| (s.inv.clone(), k.fetch_have(&s.inv)))
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
                                    let mut named: Vec<mycelium::transducer::a21::Address> = Vec::new();
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
                        let body = dialogue::encode_offer(
                            &oaddr,
                            &elem,
                            &[mesh.vk],
                            &[0; 4],
                            0,
                            &[0u8; 64],
                            &[0u8; 64],
                        );
                        let Some((reply, _)) = tell_peer(d, body) else {
                            continue;
                        };
                        offered_out += 1;
                        // 47.3's SECOND VANTAGE. The peer replayed the offer
                        // on its own tapes and sent back the form it reached;
                        // that is a re-derivation, and the home converges on
                        // it by `same_vertex` (M1), never by comparing the
                        // bytes it happens to be spelled with. A reply of one
                        // byte carries no form: the witness is UNMEASURED and
                        // the object keeps waiting, which is not the same as
                        // a witness that disagreed.
                        if reply.first() != Some(&1) || reply.len() < 2 {
                            continue;
                        }
                        let Some(peer_vk) = <[u8; 32]>::try_from(pid.as_slice()).ok() else {
                            continue;
                        };
                        let mut p = 0usize;
                        let Some(theirs) =
                            mycelium::transducer::element::Element::from_bytes(&reply[1..], &mut p)
                        else {
                            continue;
                        };
                        if let Ok(mut k) = kernel.lock() {
                            k.witness_rederived(&oaddr, peer_vk, theirs);
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
        });
    }
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
                        let kind = frame.body.first().copied().unwrap_or(0xff);
                        judge(&mut stream, frame, &door);
                        let n = served[kind as usize]
                            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                            + 1;
                        if n.is_power_of_two() {
                            eprintln!("  reflex act {kind:02x} served {n}");
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
            let _ = s.write_all(&Frame { lane: mycelium::network::wire::Stratum::Spatial, body }.encode().unwrap_or_default());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn court_seed_enters_syllabus_order_once() {
        let mut sy = Syllabus {
            order: Vec::new(),
            at: Keyed::new(),
            served: Keyed::new(),
        };
        let a = vec![1u8];
        let b = vec![2u8];
        syllabus_admit(&mut sy, [a.clone(), b.clone()]);
        syllabus_admit(&mut sy, [a.clone()]);
        assert_eq!(sy.order, vec![a.clone(), b.clone()], "a ruled address is taught once, in order");
        assert!(sy.at.contains(&a) && sy.at.contains(&b));
    }
}

