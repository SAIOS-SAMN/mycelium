// 23.8, 26 — THE DOOR: what one hypha holds for the act it is serving, and
// the control surface a client or an operator speaks to.
//
//! `Door` is the whole of a server's state as one act sees it.
//! `serve_control` answers the acts that are about the mesh rather than
//! about the language: peers, roots, the syllabus, task listing and intake.

use crate::admission::room64;
use crate::exchange::{note_root, speak_root_locked, Mesh, RootHear};
use crate::syllabus::{syllabus_admit, Syllabus};
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::cognition::Kernel;
use mycelium::network::nerve::{home, Keyed, Router};
use mycelium::network::wire::Frame;
use std::io::Write;
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
        let Some(dialogue::Offer { elem: elem_bytes, .. }) = dialogue::decode_offer(&frame.body)
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
        // will never come. Refuse the PAYLOAD, never the reply.
        //
        // What is not done here is invent one. My first cut ended
        // `.unwrap_or_else(|| vec![frame.lane.as_u8(), 0, 0, 0, 0])`: a
        // hand-built frame for the case where encoding a five-byte frame
        // fails, which is exactly the shape rule 11 refuses and which I have
        // been deleting from other people's code all session. A one-byte
        // body is inside every budget the wire has, so the fallback cannot
        // fail; if it somehow does, the act is REFUSED and the connection
        // closes, which the caller already reads as silence.
        // The witness's form may not fit the wire budget. A reply that
        // cannot carry it still ANSWERS, with the verdict bit alone, which
        // the home reads as "no form travelled" and holds as unmeasured.
        // Refuse the PAYLOAD, never the reply -- and never invent one: the
        // first cut of this ended `.unwrap_or_else(|| vec![lane, 0,0,0,0])`,
        // a hand-built frame for the case where a five-byte frame fails to
        // encode, which is the shape rule 11 refuses.
        let full = Frame { lane: frame.lane, body: rb };
        match full.encode() {
            Some(bytes) => {
                let _ = s.write_all(&bytes);
            }
            None => answer(s, Frame { lane: frame.lane, body: vec![glued as u8] }),
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
        let words: Vec<Vec<u8>> = {
            let k = door.kernel.lock().unwrap();
            let free: u64 = k.physics().reserve.iter().sum();
            let depth = (1 + (free as usize * 7) / (4 * mycelium::physics::reserve_max() as usize)).min(8);
            k.basis().iter().skip(cursor as usize).take(depth).map(|w| w.to_bytes()).collect()
        };
        let reply = Frame { lane: frame.lane, body: dialogue::encode_syllabus_reply(&words) };
        answer(s, reply);
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
        let (words, classes, k_now, height, served_addrs) = {
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
        let Some(dialogue::Verdict { voice: cvoice, addr, ruling, receipt, pi, attach }) =
            dialogue::decode_verdict(&frame.body)
        else {
            return true;
        };
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
        answer(s, reply);
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
