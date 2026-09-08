// 23.8, 26 — THE DOOR: what one hypha holds for the act it is serving, and
// the control surface a client or an operator speaks to.
//
//! `Door` is the whole of a server's state as one act sees it.
//! `serve_control` answers the acts that are about the mesh rather than
//! about the language: peers, roots, the syllabus, task listing and intake.

use crate::admission::room64;
use crate::exchange::{note_root, speak_root_locked, Mesh, RootHear};
use crate::syllabus::{syllabus_admit, syllabus_page, Syllabus};
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::cognition::Kernel;
use mycelium::network::nerve::{home, Keyed, Router};
use mycelium::network::wire::Frame;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};


#[derive(Clone)]
pub(crate) struct Door {
    pub(crate) kernel: Arc<Mutex<Kernel>>,
    pub(crate) mesh: Mesh,
    pub(crate) router: Arc<Mutex<Router>>,
    pub(crate) id: [u8; 32],
    pub(crate) bind: String,
    pub(crate) join_peers: Arc<Vec<String>>,
    pub(crate) roots: Arc<Mutex<Keyed<Keyed<u64>>>>,
    pub(crate) registry: Arc<Mutex<mycelium::registry::TaskRegistry>>,
    pub(crate) syllabus: Arc<Mutex<Syllabus>>,
    pub(crate) nerve_tx: std::sync::mpsc::SyncSender<(String, Vec<u8>)>,
    pub(crate) synth_tx: std::sync::mpsc::SyncSender<mycelium::synthesis::Deliberation>,
    pub(crate) state_path: String,
}

/// Control acts that used to run on the accept thread. The listener never
/// holds the kernel lock; these run on a reflex worker.
pub(crate) fn serve_control(s: &mut TcpStream, frame: &Frame, door: &Door) -> bool {
    let Some((&kind, _rest)) = frame.body.split_first() else { return false };
    if kind == dialogue::ACT_ROOT {
        let Some((from, task, solver, spoken_at)) = dialogue::decode_root(&frame.body) else { return true };
        if !door.router.lock().map(|r| r.knows(&from)).unwrap_or(false) {
            answer(s, Frame { lane: frame.lane, body: Vec::new() });
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
        answer(s, Frame { lane: frame.lane, body: reply });
        return true;
    }
    if kind == dialogue::ACT_EVIDENCE {
        if let Some((_dvoice, items)) = dialogue::decode_evidence_deposit(&frame.body) {
            let accepted = {
                let mut k = door.kernel.lock().unwrap();
                items
                    .into_iter()
                    .filter(|e| {
                        k.deposit_evidence(e.addr.clone(), e.footprint.clone(), e.scene.clone())
                    })
                    .count()
            };
            let reply = Frame { lane: frame.lane, body: vec![accepted.min(255) as u8] };
            answer(s, reply);
            return true;
        }
        let Some(_evoice) = dialogue::decode_evidence_request(&frame.body) else { return true };
        let items: Vec<_> = {
            let k = door.kernel.lock().unwrap();
            k.anchor_evidence().iter().take(32).cloned().collect()
        };
        let reply = Frame { lane: frame.lane, body: dialogue::encode_evidence_reply(&items) };
        answer(s, reply);
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
        answer(s, reply);
        return true;
    }
    if kind == dialogue::ACT_SOLVE {
        let Some(dialogue::SolveAct { voice: svoice, task: tid, outputs, rule: rule_bytes }) =
            dialogue::decode_solve(&frame.body)
        else {
            return true;
        };
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
                answer(s, reply);
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
        answer(s, reply);
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
                for (_, a) in addrs.fiber(mycelium::network::nerve::ROOT) {
                    if !nerve_addrs.iter().any(|x| x == a) {
                        nerve_addrs.push(a.clone());
                    }
                }
            }
            let reply = Frame { lane: frame.lane, body: dialogue::encode_peers_reply(&nerve_addrs) };
            answer(s, reply);
        }
        return true;
    }
    if kind == dialogue::ACT_OFFER {
        crate::judge::offer::judge(s, frame, door);
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
        answer(s, Frame { lane: frame.lane, body: vec![queued as u8] });
        return true;
    }
    if kind == dialogue::ACT_GENESIS {
        let Some((_gvoice, cursor)) = dialogue::decode_genesis_request(&frame.body) else { return true };
        // A genesis page is a stroke's worth of basis from the cursor, read
        // as a slice and not walked to it. The size was a sum of the four
        // reserve lanes clamped at eight (law 5, rule 5); `skip(cursor)`
        // walked every word before the cursor to reach it.
        let words: Vec<Vec<u8>> = {
            let Ok(k) = door.kernel.lock() else { return true };
            let mut budget = mycelium::physics::Physics::default().stroke();
            let mut out = Vec::new();
            if let Some(rest) = k.basis().get(cursor as usize..) {
                for w in rest {
                    if !budget.spend() {
                        break;
                    }
                    out.push(w.to_bytes());
                }
            }
            out
        };
        let reply = Frame { lane: frame.lane, body: dialogue::encode_syllabus_reply(&words) };
        answer(s, reply);
        return true;
    }
    if kind == dialogue::ACT_SYLLABUS {
        let Some(svoice) = dialogue::decode_syllabus_request(&frame.body) else { return true };
        let mut missed: Vec<Vec<u8>> = Vec::new();
        // A PAGE IS A WINDOW, NOT A COPY OF THE BOOK. This cloned
        // `sy.order` — every address this court has ever ruled — out of the
        // lock on every request, so that a window of at most `depth`
        // entries could be read from it. Checklist (j) refuses "any
        // `clone()` of a carrier into two places"; at ten thousand peers
        // polling a court that holds a large order, the cost is O(ruled
        // words) per request per peer, allocated and copied while the
        // syllabus lock is held. The window is taken under the lock, from
        // the cursor, by a slice read that is O(depth) and not O(cursor).
        // THE PAGE IS WHAT A STROKE AFFORDS. Its size was
        // `(1 + (free * 7) / (4 * reserve_max)).min(8)` over
        // `free = reserve.iter().sum()`: law 5's "μ⃗ is never summed … or
        // reduced to a scalar" in one expression, and rule 5's own named
        // example, `depth.min(8)`, on the result.
        let seed = match door.kernel.lock() {
            Ok(k) => k.syllabus_seed(),
            Err(_) => return true,
        };
        let mut budget = mycelium::physics::Physics::default().stroke();
        let (cursor, page) = {
            let Ok(mut sy) = door.syllabus.lock() else { return true };
            syllabus_admit(&mut sy, seed);
            syllabus_page(&mut sy, &svoice, &mut budget)
        };
        let (words, classes, served_addrs) = {
            let Ok(k) = door.kernel.lock() else { return true };
            let mut out = Vec::new();
            let mut served_addrs = Vec::new();
            for addr in page {
                match k.word_at(&addr) {
                    Some(w) => {
                        served_addrs.push(addr);
                        out.push(w.to_bytes());
                    }
                    None => missed.push(addr),
                }
            }
            let classes = k.class_records(&mycelium::transducer::stalk::Invariant::default());
            let next = cursor + out.len() + missed.len();
            drop(k);
            if let Ok(mut sy) = door.syllabus.lock() {
                sy.served.insert(svoice, next);
            }
            (out, classes, served_addrs)
        };
        for addr in &served_addrs {
            let (k_at, h_at) = match door.kernel.lock() {
                Ok(mut k) => {
                    k.note_served(addr, svoice);
                    (k.k_step(), k.book_height())
                }
                Err(_) => continue,
            };
            eprintln!(
                "  court syllabus {:02x}{:02x}… to {:02x}{:02x}… receipt K {k_at} h {h_at}",
                addr[0], addr[1], svoice[0], svoice[1]
            );
        }
        // An address whose word this court no longer holds is re-queued
        // behind the cursor, once. This rebuilt the whole tail index INSIDE
        // the loop, one full walk of the order per missed address —
        // O(missed × order) where O(order) already stands (rule 10). The
        // index is built once, and the `.unwrap()`s with it (§2.4).
        if !missed.is_empty() {
            if let Ok(mut sy) = door.syllabus.lock() {
                let ahead = match sy.served.get(&svoice) {
                    Some(&n) => n,
                    None => 0,
                };
                let mut tail: Keyed<()> = Keyed::new();
                if let Some(rest) = sy.order.get(ahead..) {
                    for a in rest {
                        tail.insert(a, ());
                    }
                }
                for addr in missed {
                    if !tail.contains(&addr) {
                        tail.insert(&addr, ());
                        sy.order.push(addr);
                    }
                }
            }
        }
        let mut body = dialogue::encode_syllabus_reply(&words);
        dialogue::append_class_records(&mut body, &classes);
        let reply = Frame { lane: frame.lane, body };
        answer(s, reply);
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
        answer(s, reply);
        return true;
    }
    if kind == dialogue::ACT_VERDICT {
        crate::judge::verdict::judge(s, frame, door);
        return true;
    }
    false
}

pub(crate) fn homes_here(mesh: &Mesh, kernel: &Mutex<Kernel>, inv: &mycelium::transducer::stalk::Invariant, mag: &[u64; 4]) -> bool {
    let room = match kernel.lock() {
        Ok(k) => room64(k.lane_room()),
        Err(_) => return false,
    };
    let c = mesh.cover.lock().unwrap();
    let r = mesh.rooms.lock().unwrap();
    let a = mesh.addrs.lock().unwrap();
    home(&c, inv, mag, &r, &a, Some((&mesh.vk, &room, &mesh.bind))).0 == mesh.vk
}

pub(crate) fn list_task(
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
        let Some(mycelium::arc::Refined { delta: mut chain, frame, .. }) =
            mycelium::arc::refine_delta(a, b)
        else {
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
            false if rehome => if k.has_listing(&addr) { k.relocate_listing(&addr) },
            false => {}
        }
    }
}

/// First-time listing under a stroke budget until the manifest is exhausted.
pub(crate) fn intake_corpus(
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
pub(crate) fn relocate_corpus(
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

pub(crate) fn demand_cover_home(mesh: &Mesh, kernel: &Mutex<Kernel>, chain: &[(i64, i64, u8, u8)]) -> ([u8; 32], String) {
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

/// The RENDEZVOUS for a content key: XOR-closest known hypha to the key's
/// bits. A listing is not hashed; its Address walks the trie.
pub(crate) fn rendezvous(key: &[u8], self_id: &[u8; 32], router: &Router) -> Option<String> {
    let dist = |idb: &[u8]| -> [u8; 32] {
        std::array::from_fn(|i| idb.get(i).copied().unwrap_or(0) ^ key.get(i).copied().unwrap_or(0))
    };
    match router.closest(key) {
        Some((pid, addr)) if dist(&pid) < dist(self_id) => Some(addr),
        _ => None,
    }
}
