// 23.8, 26 — THE DOOR: what one hypha holds for the act it is serving, and
// the control surface a client or an operator speaks to.
//
//! `Door` is the whole of a server's state as one act sees it.
//! `serve_control` answers the acts that are about the mesh rather than
//! about the language: peers, roots, the syllabus, task listing and intake.

use crate::exchange::{note_root, speak_root_locked, Mesh, RootHear};
use crate::syllabus::{syllabus_admit, syllabus_window, Syllabus};
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::cognition::Kernel;
use mycelium::network::nerve::{home, Keyed, Router};
use mycelium::network::wire::Frame;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};


#[derive(Clone)]
pub(crate) struct Door {
    /// 78.146 — the seat relay on its own lock (`judge::seat::serve_exempt`).
    pub(crate) seat_relay: mycelium::cognition::seat::SharedRelay,
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
            // A further solver of a consilient root is spoken on, never
            // re-dispatched as a first nor re-wound as a second.
            RootHear::Joined => {
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
                let Ok(mut k) = crate::held::held!(door.kernel) else { return true };
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
            let Ok(k) = crate::held::held!(door.kernel) else { return true };
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
        // An envelope that does not fit the lengths it writes is not sent
        // short: the voice would study a family this registry did not
        // dispatch. Nor is it answered empty — an empty body is "no work",
        // and a book with open tasks answered that while its cursor sat on
        // a task it could not frame (barren-realms read 8 such pulls as
        // tasks with no family; the task's long address passed the u16
        // wall, which the v1.2 citation removes). A plain request steps on
        // to the next open task, once round the book at most.
        let envelope = |tid: &mycelium::transducer::address::Address, task: &mycelium::arc::Task| {
            let tests: Vec<_> = task.test.iter().map(|(x, _)| x.clone()).collect();
            let b = dialogue::encode_task_envelope(&mycelium::registry::citation(tid),&task.train, &tests);
            if b.is_none() {
                eprintln!("  task passed over: its envelope does not fit its lengths");
            }
            b
        };
        // A plain request is dispatched the next open task; 78.146, a request
        // naming a task's citation (wire v1.2) is answered that task, open or
        // settled, or nothing where this registry holds no such task.
        let body = match (dialogue::decode_task_request(&frame.body), dialogue::decode_task_by_id(&frame.body)) {
            (Some(_), _) => {
                let Ok(mut reg) = door.registry.lock() else { return true };
                let mut body = Vec::new();
                for _ in 0..reg.len() {
                    let Some((tid, task)) = reg.dispatch() else { break };
                    if let Some(b) = envelope(&tid, task) {
                        body = b;
                        break;
                    }
                }
                body
            }
            (None, Some((_, cite))) => {
                let Ok(reg) = door.registry.lock() else { return true };
                let found = reg.by_citation(&cite).and_then(|tid| reg.task(tid.as_bytes()).and_then(|task| envelope(&tid, task)));
                match found {
                    Some(b) => b,
                    None => Vec::new(),
                }
            }
            (None, None) => return true,
        };
        let reply = Frame { lane: frame.lane, body };
        answer(s, reply);
        return true;
    }
    if kind == dialogue::ACT_SOLVE {
        let Some(dialogue::SolveAct { voice: svoice, task: cite, rule: rule_bytes, signed, .. }) =
            dialogue::decode_solve(&frame.body)
        else {
            return true;
        };
        let rule = rule_bytes.as_ref().and_then(|b| {
            let mut p = 0usize;
            mycelium::transducer::element::Element::from_bytes(b, &mut p)
        });
        // Wire v1.2: the solve names its task by citation; the court reads
        // the task's content address off this book. A citation this book
        // holds no task for is refused by name below ("no such task").
        let (tid, withheld, open) = {
            let Ok(reg) = door.registry.lock() else { return true };
            match reg.by_citation(&cite) {
                Some(tid) => {
                    let w = reg.withheld(&tid);
                    let o = reg.open(&tid);
                    (tid, w, o)
                }
                None => (mycelium::transducer::address::Address::new(cite.to_vec()), Vec::new(), None),
            }
        };
        // THE SOLVE, JUDGED BY ITS ACTION AND PAID (the actions' physics): the
        // court replays the rule on the withheld pairs and prices what it
        // discharges; the registry records the closure only after payment.
        // The kind as its place in the reader's own order (78.9,
        // `ObstructionKind::ALL`): every kind has one, a byte.
        let refused = |reason: &str, kind: mycelium::transducer::sentence::ObstructionKind| {
            use mycelium::transducer::sentence::ObstructionKind as K;
            let kind = match kind {
                K::P => 0,
                K::D => 1,
                K::C => 2,
                K::I => 3,
                K::N => 4,
                K::Unclassified => 5,
            };
            dialogue::SolveRefusal { reason: reason.as_bytes().to_vec(), kind }
        };
        // The operator (2026-10-08): the hypha serves anyone; a universe founded
        // here speaks only through its ancients — an inner tier of a founded
        // domain that is not its sitting reserve is refused (`may_solve`).
        let ancient = crate::held::held!(door.kernel).ok().is_some_and(|k| k.may_solve(&svoice));
        let (code, mu, refusal, stamp) = match (open, rule.as_ref()) {
            // 78.141 — the court pays the voice a solve names: unsigned, it
            // names no payee, and nothing is judged or paid.
            _ if !signed => (
                dialogue::SOLVE_NOTHING,
                [0; 4],
                Some(refused("unsigned: a solve names its payee by its voice's signature", mycelium::transducer::sentence::ObstructionKind::I)),
                None,
            ),
            _ if !ancient => (
                dialogue::SOLVE_NOTHING,
                [0; 4],
                Some(refused("an inner seat of a founded universe: it speaks to the hypha only through its ancients' house", mycelium::transducer::sentence::ObstructionKind::I)),
                None,
            ),
            (None, _) => (dialogue::SOLVE_NOTHING, [0; 4], Some(refused("no such task on this book", mycelium::transducer::sentence::ObstructionKind::I)), None),
            (Some(_), None) => (
                dialogue::SOLVE_NOTHING,
                [0; 4],
                Some(refused("a solve carries its rule: a raster is no proof of a motion", mycelium::transducer::sentence::ObstructionKind::I)),
                None,
            ),
            (Some(first), Some(rule)) => {
                let (verdict, offers) = {
                    let Ok(mut k) = crate::held::held!(door.kernel) else { return true };
                    let verdict = match k.judge_solve(rule, &withheld) {
                        mycelium::cognition::SolveVerdict::Solved { pay } => {
                            // The court names a task by its citation (wire v1.2).
                            let paid = k.pay_solve(&cite, svoice, rule, pay, first);
                            let code = if first { dialogue::SOLVE_BOUNTY } else { dialogue::SOLVE_FLOOR };
                            (code, paid, None, Some(k.height()))
                        }
                        mycelium::cognition::SolveVerdict::Refused(r) => {
                            (dialogue::SOLVE_NOTHING, [0; 4], Some(refused(r.reason, r.obstruction.kind())), None)
                        }
                        mycelium::cognition::SolveVerdict::NotARule => (
                            dialogue::SOLVE_NOTHING,
                            [0; 4],
                            Some(refused("the rule is no sentence", mycelium::transducer::sentence::ObstructionKind::I)),
                            None,
                        ),
                        mycelium::cognition::SolveVerdict::NoWork => (
                            dialogue::SOLVE_NOTHING,
                            [0; 4],
                            Some(refused("the rule discharges no work on the withheld pairs: nothing is closed or paid", mycelium::transducer::sentence::ObstructionKind::N)),
                            None,
                        ),
                        mycelium::cognition::SolveVerdict::Unverifiable => (
                            dialogue::SOLVE_NOTHING,
                            [0; 4],
                            Some(refused("the task withholds no answer to replay against", mycelium::transducer::sentence::ObstructionKind::N)),
                            None,
                        ),
                    };
                    (verdict, k.take_offers())
                };
                // The solving rule docketed is offered by the act that docketed it.
                crate::offer::offer(&door.kernel, &door.mesh, offers);
                verdict
            }
        };
        if let Some(at) = stamp {
            // PAID, THEN RECORDED: the closure enters the registry after its
            // payment, so none is lost between the two.
            if let Ok(mut reg) = door.registry.lock() {
                if reg.close(&tid) == Some(mycelium::registry::Closure::First) {
                        // A state path with no parent writes beside the
                        // working directory, which is where it already is.
                        let dir = match std::path::Path::new(&door.state_path).parent() {
                            Some(p) => p.join("corpora"),
                            None => std::path::PathBuf::from("corpora"),
                        };
                        if let Some(snap) = reg.snapshot() {
                            let _ = std::fs::create_dir_all(&dir);
                            let tmp = dir.join("registry.tmp");
                            if std::fs::write(&tmp, &snap).is_ok() {
                                let _ = std::fs::rename(&tmp, dir.join("registry"));
                            }
                        }
                }
            }
            // THE ROOT, AT THE HEIGHT THE SOLVE WAS PAID: a new solver is
            // spoken onward; the second makes the root consilient and rewinds
            // the syllabus; a later one is consilient already.
            let hear = {
                let Ok(mut roots) = door.roots.lock() else { return true };
                note_root(&mut roots, &tid, svoice, at)
            };
            match hear {
                RootHear::Known => {}
                RootHear::First | RootHear::Joined => {
                    speak_root_locked(&door.router, &door.nerve_tx, &door.id, &tid, &svoice, None, at);
                }
                RootHear::Second => {
                    rewind_syllabus(door);
                    speak_root_locked(&door.router, &door.nerve_tx, &door.id, &tid, &svoice, None, at);
                }
            }
        }
        let reply = Frame { lane: frame.lane, body: dialogue::encode_solve_verdict(code, &mu, refusal.as_ref()) };
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
            match dialogue::encode_peers_reply(&nerve_addrs) {
                Some(body) => answer(s, Frame { lane: frame.lane, body }),
                None => eprintln!("  peers reply refused: an address or the count passes its u16"),
            }
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
            let charts = match crate::held::held!(door.kernel) {
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
    if kind == dialogue::ACT_SYLLABUS {
        let Some(svoice) = dialogue::decode_syllabus_request(&frame.body) else { return true };
        // A PAGE IS A WINDOW, NOT A COPY OF THE BOOK: a slice from the voice's
        // cursor under the syllabus lock (checklist (j)).
        let seed = match crate::held::held!(door.kernel) {
            Ok(k) => k.syllabus_seed(),
            Err(_) => return true,
        };
        let (cursor, window) = {
            let Ok(mut sy) = door.syllabus.lock() else { return true };
            syllabus_admit(&mut sy, seed);
            syllabus_window(&mut sy, &svoice)
        };
        // THE PAGE IS WHAT THE COURT'S RESERVE AFFORDS (the actions'
        // metabolism): each word served is paid by its magnitude, drawn per
        // lane; the page ends at the frame, the wire's count, or the first
        // word a lane cannot fund. 45.1, 78.24 (2b): each word served is
        // sealed as a Syllabus act and carries that stamp. Nothing is sealed
        // for a word that is not sent.
        let (page, classes) = {
            let Ok(mut k) = crate::held::held!(door.kernel) else { return true };
            let page = k.serve_syllabus(&window, svoice, mycelium::network::wire::frame_budget());
            let classes = k.class_records(&mycelium::transducer::stalk::Invariant::default());
            drop(k);
            if let Ok(mut sy) = door.syllabus.lock() {
                sy.served.insert(svoice, cursor + page.consumed);
            }
            (page, classes)
        };
        // A word this court's reserve could not fund yet waits behind the
        // cursor with the words it missed.
        let words = page.words;
        let mut missed = page.missed;
        missed.extend(page.deferred.into_iter().map(|(addr, _)| addr));
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
            let Ok(mut k) = crate::held::held!(door.kernel) else { return true };
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
            let Ok(mut k) = crate::held::held!(door.kernel) else { return true };
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
    let room = match crate::held::held!(kernel) {
        Ok(k) => match k.lane_room() {
            Some(r) => r,
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
    // Wire v1.2 — the court names a task by its CITATION, the name voices
    // speak it by (a solve, a colour word): its family and its listings are
    // bound under those 32 bytes, once.
    let cite = mycelium::registry::citation(&mycelium::transducer::address::Address::new(tid.as_ref().to_vec()));
    let tid = &cite[..];
    if let Ok(mut k) = crate::held::held!(kernel) {
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
        // The pieces and their charts are read here, with no kernel held;
        // the lock below only commits them (`Kernel::intake_prepared`).
        let prepared = match here {
            true => mycelium::cognition::prepare_intake(&chain, &frame),
            false => Vec::new(),
        };
        // THE INTAKE LETS THE QUEUE IN, as the court loop does: it took the
        // kernel once per listing for minutes after every restart and headed
        // the lock on all three hyphae (2026-10-09), deferring seats and
        // wallets behind it.
        crate::held::yield_to_waiters();
        let mut k = match crate::held::held!(kernel) {
            Ok(g) => g,
            Err(_) => continue,
        };
        match here {
            true => {
                if k.intake_prepared(prepared) {
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

