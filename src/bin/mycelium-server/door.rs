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
        // A poisoned router lock knows no peer; the root is not heard.
        if !door.router.lock().is_ok_and(|r| r.knows(&from)) {
            answer(s, Frame { lane: frame.lane, body: Vec::new() });
            return true;
        }
        let hear = {
            let Ok(mut roots) = door.roots.lock() else { return true };
            note_root(&mut roots, &task, solver, spoken_at)
        };
        match hear {
            RootHear::Known => {}
            RootHear::First => {
                // A poisoned registry lock prioritises nothing.
                let prioritized = door.registry.lock().is_ok_and(|mut r| r.prioritize(&task));
                eprintln!(
                    "  ROOT heard from {:02x}: task {:02x}{:02x}… solved by voice {:02x}; {}",
                    from[0], task[0], task[1], solver[0],
                    if prioritized { "re-dispatched first here" } else { "not open on this book" }
                );
                speak_root_locked(&door.router, &door.nerve_tx, &door.id, &task, &solver, Some(from), spoken_at);
            }
            RootHear::Second => {
                rewind_syllabus(door);
                eprintln!("  ROOT CONSILIENT task {:02x}{:02x}… two voices; syllabus rewound", task[0], task[1]);
                speak_root_locked(&door.router, &door.nerve_tx, &door.id, &task, &solver, Some(from), spoken_at);
            }
        }
        let reply = {
            let Ok(roots) = door.roots.lock() else { return true };
            dialogue::encode_root_reply(
                roots.get(task.as_bytes()).and_then(|v| {
                    // `get(task)` follows the task's prefix: the solvers standing on
                    // THIS task, and the first in key order is the reply.
                    v.scoped()
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
                let Ok(mut k) = door.kernel.lock() else { return true };
                items
                    .into_iter()
                    .filter(|e| {
                        k.deposit_evidence(e.addr.clone(), e.footprint.clone(), e.scene.clone())
                    })
                    .count()
            };
            // The reply is the count ACCEPTED, in one byte. A deposit of
            // more than 255 items cannot be answered in it, and answering
            // 255 would tell the peer a number this door did not measure.
            let Ok(accepted) = u8::try_from(accepted) else {
                eprintln!("  evidence refused: {accepted} accepted does not fit the reply");
                return true;
            };
            let reply = Frame { lane: frame.lane, body: vec![accepted] };
            answer(s, reply);
            return true;
        }
        let Some(_evoice) = dialogue::decode_evidence_request(&frame.body) else { return true };
        let items: Vec<_> = {
            let Ok(k) = door.kernel.lock() else { return true };
            k.anchor_evidence().iter().take(32).cloned().collect()
        };
        // LD0 — a reply whose address cannot be framed is refused whole,
        // by name, and nothing partial is answered.
        let Some(body) = dialogue::encode_evidence_reply(&items) else {
            eprintln!("  evidence reply refused: an address cannot be framed");
            return true;
        };
        answer(s, Frame { lane: frame.lane, body });
        return true;
    }
    if kind == dialogue::ACT_TASK {
        let Some(_tvoice) = dialogue::decode_task_request(&frame.body) else { return true };
        let dispatched = {
            let Ok(mut reg) = door.registry.lock() else { return true };
            reg.dispatch().map(|(tid, task)| (tid, task.clone()))
        };
        // An envelope that does not fit the lengths it writes is not sent
        // short: the voice would study a family this registry did not
        // dispatch. An empty body is "no work", which it already reads.
        let body = match &dispatched {
            Some((tid, task)) => {
                let tests: Vec<_> = task.test.iter().map(|(x, _)| x.clone()).collect();
                match dialogue::encode_task_envelope(tid, &task.train, &tests) {
                    Some(b) => b,
                    None => {
                        eprintln!("  task refused: the envelope does not fit its lengths");
                        Vec::new()
                    }
                }
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
            let Ok(reg) = door.registry.lock() else { return true };
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
            let Ok(mut reg) = door.registry.lock() else { return true };
            match reg.settle(&tid, &outputs) {
                mycelium::registry::SolveYield::Bounty(tension) => {
                    let snap = reg.snapshot();
                    // A state path with no parent writes beside the
                    // working directory, which is where it already is.
                    let dir = match std::path::Path::new(&door.state_path).parent() {
                        Some(p) => p.join("corpora"),
                        None => std::path::PathBuf::from("corpora"),
                    };
                    let _ = std::fs::create_dir_all(&dir);
                    let tmp = dir.join("registry.tmp");
                    if std::fs::write(&tmp, &snap).is_ok() {
                        let _ = std::fs::rename(&tmp, dir.join("registry"));
                    }
                    drop(reg);
                    let paid = {
                        let Ok(mut k) = door.kernel.lock() else { return true };
                        k.pay_solve(&tid, svoice, tension, rule_elem.as_ref(), &withheld)
                    };
                    let hear = {
                        let Ok(mut roots) = door.roots.lock() else { return true };
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
                        rewind_syllabus(door);
                        eprintln!("  ROOT CONSILIENT task {:02x}{:02x}… two voices; syllabus rewound", tid[0], tid[1]);
                    }
                    (dialogue::SOLVE_BOUNTY, paid)
                }
                mycelium::registry::SolveYield::VerifyFloor => {
                    drop(reg);
                    let paid = {
                        let Ok(mut k) = door.kernel.lock() else { return true };
                        k.pay_solve(&tid, svoice, [0, 0, 1, 0], rule_elem.as_ref(), &withheld)
                    };
                    let hear = {
                        let Ok(mut roots) = door.roots.lock() else { return true };
                        note_root(&mut roots, &tid, svoice, 0)
                    };
                    if matches!(hear, RootHear::Second) {
                        rewind_syllabus(door);
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
                for (_, a) in addrs.scoped() {
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
    // A4 — THE COVER, BY PAGE. The digest carries the four roots; a peer
    // whose roots moved asks here for the charts under them, from the last
    // chart it holds, and reads as many as one frame affords. The cover
    // used to ride the digest whole, which made the digest's size the
    // cover's and refused it outright past the axis count.
    if kind == dialogue::ACT_COVER {
        if let Some((_voice, from)) = dialogue::decode_cover_request(&frame.body) {
            let charts = match door.kernel.lock() {
                Ok(k) => k.own_cover_charts(),
                Err(_) => return true,
            };
            // A chart wider than the page's axis count is passed over by
            // the encoder, so the count of them is said here once, where
            // the charts are this court's own: on the fleet at 3d2987e
            // this width took the whole digest down with it and the line
            // that named it was the only reason the cause was findable.
            let wide = charts.iter().filter(|(_, inv)| inv.0.len() > 255).count();
            if wide > 0 {
                eprintln!("  cover page passed over {wide} of {} charts: wider than the axis count", charts.len());
            }
            let Some(body) = dialogue::encode_cover_page(
                &door.mesh.vk,
                &door.bind,
                &charts,
                &from,
                mycelium::network::wire::frame_budget(),
            ) else {
                eprintln!("  cover page refused: the bind does not fit its length");
                return true;
            };
            answer(s, Frame { lane: frame.lane, body });
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
            // A word whose tape does not parse queues nothing.
            .is_some_and(|q| q);
        answer(s, Frame { lane: frame.lane, body: vec![queued as u8] });
        return true;
    }
    if kind == dialogue::ACT_GENESIS {
        let Some((_gvoice, cursor)) = dialogue::decode_genesis_request(&frame.body) else { return true };
        // A genesis page is a stroke's worth of basis from the cursor, read
        // as a slice and not walked to it. The size was a sum of the four
        // reserve lanes clamped at eight (law 5, rule 5); `skip(cursor)`
        // walked every word before the cursor to reach it.
        let words: Vec<dialogue::Taught> = {
            let Ok(mut k) = door.kernel.lock() else { return true };
            // The page is what THIS court's stroke affords, read off its own
            // physics, never a default's (rule 5).
            let mut budget = k.stroke();
            let mut out = Vec::new();
            let rest: Vec<mycelium::transducer::address::Address> = match k.basis().get(cursor as usize..) {
                Some(rest) => rest.iter().map(|w| w.address()).collect(),
                None => Vec::new(),
            };
            for addr in rest {
                if !budget.spend() {
                    break;
                }
                // Each word with the stamp that admitted it (78.24 (2b)); a
                // word no act admitted is named and not taught.
                match k.taught(&addr) {
                    Some(t) => out.push(t),
                    None => eprintln!("  genesis page: a basis word has no admitting stamp and is not taught"),
                }
            }
            out
        };
        let Some(body) = dialogue::encode_syllabus_reply(&words) else {
            eprintln!("  syllabus refused: the page does not fit its count");
            return true;
        };
        let reply = Frame { lane: frame.lane, body };
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
        // The budget is this court's own stroke, taken under the same lock
        // as the seed: a default `Physics` is not the court's metabolism
        // (rule 5).
        let (seed, mut budget) = match door.kernel.lock() {
            Ok(mut k) => (k.syllabus_seed(), k.stroke()),
            Err(_) => return true,
        };
        let (cursor, page) = {
            let Ok(mut sy) = door.syllabus.lock() else { return true };
            syllabus_admit(&mut sy, seed);
            syllabus_page(&mut sy, &svoice, &mut budget)
        };
        // 45.1, 78.24 (2b): A WORD SERVED IS TAUGHT AT THE STAMP OF THE ACT
        // THAT SERVED IT. The Syllabus node is sealed first, and the page
        // carries its stamp beside the word, so the voice files the word's
        // locator from the act that taught it.
        let (words, classes) = {
            let Ok(mut k) = door.kernel.lock() else { return true };
            let mut out = Vec::new();
            for addr in page {
                let Some(w) = k.word_at(&addr).map(|w| w.to_bytes()) else {
                    missed.push(addr);
                    continue;
                };
                let Some(at) = k.note_served(&addr, svoice) else {
                    eprintln!(
                        "  court syllabus {:02x}{:02x}… to {:02x}{:02x}… refused: the act could not be sealed",
                        addr[0], addr[1], svoice[0], svoice[1]
                    );
                    missed.push(addr);
                    continue;
                };
                eprintln!(
                    "  court syllabus {:02x}{:02x}… to {:02x}{:02x}… receipt K {} h {}",
                    addr[0], addr[1], svoice[0], svoice[1], at.place().print(), at.h
                );
                out.push(dialogue::Taught { at, word: w });
            }
            let classes = k.class_records(&mycelium::transducer::stalk::Invariant::default());
            let next = cursor + out.len() + missed.len();
            drop(k);
            if let Ok(mut sy) = door.syllabus.lock() {
                sy.served.insert(svoice, next);
            }
            (out, classes)
        };
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
        let Some(mut body) = dialogue::encode_syllabus_reply(&words) else {
            eprintln!("  syllabus refused: the page does not fit its count");
            return true;
        };
        if dialogue::append_class_records(&mut body, &classes).is_none() {
            eprintln!("  syllabus refused: the class records cannot be framed");
            return true;
        }
        let reply = Frame { lane: frame.lane, body };
        answer(s, reply);
        return true;
    }
    if kind == dialogue::ACT_REFINE {
        let Some((rvoice, word, cites)) = dialogue::decode_refine(&frame.body) else { return true };
        let (code, mu) = {
            let Ok(mut k) = door.kernel.lock() else { return true };
            match k.try_refine(&word, rvoice, &cites) {
                Some(paid) => (dialogue::REFINE_ACCEPTED, paid),
                None => (dialogue::REFINE_NOTHING, [0; 4]),
            }
        };
        let reply = Frame { lane: frame.lane, body: dialogue::encode_solve_receipt(code, &mu) };
        answer(s, reply);
        return true;
    }
    // EXPANSION 2 — the algebraic bid is ruled on the court's own algebra,
    // never on a spatial replay it has no links for.
    if kind == dialogue::ACT_REFINE_ALGEBRAIC {
        let Some((rvoice, word, held, witness)) = dialogue::decode_refine_algebraic(&frame.body) else {
            return true;
        };
        let (code, mu) = {
            let Ok(mut k) = door.kernel.lock() else { return true };
            match k.try_refine_algebraic(&word, rvoice, &held, &witness) {
                Some(paid) => (dialogue::REFINE_ACCEPTED, paid),
                None => (dialogue::REFINE_NOTHING, [0; 4]),
            }
        };
        if code == dialogue::REFINE_ACCEPTED {
            eprintln!(
                "  court refined algebraically {:02x}{:02x}… against {:02x}{:02x}… ({}) for {:02x}{:02x}…",
                word[0],
                word[1],
                held[0],
                held[1],
                match witness {
                    dialogue::AlgebraicWitness::SameOperator => "same operator",
                    dialogue::AlgebraicWitness::Section(_) => "section",
                },
                rvoice[0],
                rvoice[1]
            );
        }
        let reply = Frame { lane: frame.lane, body: dialogue::encode_solve_receipt(code, &mu) };
        answer(s, reply);
        return true;
    }
    false
}

/// ROOT CONSILIENT: every voice's syllabus cursor returns to the start, so
/// the proven word reaches each of them on its next page (26). Read and
/// rewound under one lock; it read the fibre under one lock and wrote under
/// a second, and panicked on a poisoned first (§2.4).
fn rewind_syllabus(door: &Door) {
    let Ok(mut sy) = door.syllabus.lock() else { return };
    // EVERY voice's cursor returns to the start -- that is what a rewind
    // is -- and the carrier holds one entry per voice.
    let voices: Vec<Vec<u8>> = sy.served.scoped().map(|(v, _)| v).collect();
    for v in voices {
        sy.served.insert(v, 0);
    }
}

/// Does the listing home on this hypha. A poisoned mesh lock is no reading
/// of the cover: the listing is not taken here (§2.4, rule 11).
pub(crate) fn homes_here(mesh: &Mesh, kernel: &Mutex<Kernel>, inv: &mycelium::transducer::stalk::Invariant, mag: &[u64; 4]) -> bool {
    let room = match kernel.lock() {
        Ok(k) => match k.lane_room() {
            Some(r) => room64(r),
            None => return false,
        },
        Err(_) => return false,
    };
    let (Ok(c), Ok(r), Ok(a)) = (mesh.cover.lock(), mesh.rooms.lock(), mesh.addrs.lock()) else {
        return false;
    };
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
        // A chain with no A5 form cannot be addressed, so it is not routed
        // and not taken in (U14).
        let Some(residual) = mycelium::transducer::element::from_residual_framed(&chain, &frame)
        else {
            continue;
        };
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

