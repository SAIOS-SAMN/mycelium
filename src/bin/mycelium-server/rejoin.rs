// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// #273, MU-ENGINEERING §8 — THE BOOT'S REJOIN: before this court signs
// anything, each peer is asked for the receipts of this court's own line
// above the head it restored, and what continues the head is held.
//
//! A court's own receipts reach its disk only at intervals, so a hard stop
//! loses the newest while its peers already hold them; signing those places
//! again equivocates against itself and burns its own bond. The ask is the
//! back-fill's own (`ACT_RECEIPTS`, this court's key as the signer), page by
//! page, each page asked above the head the last one left. A peer that does
//! not answer is passed over and said in the log: when every peer is down
//! at once nothing can be rejoined (#273's open question), and the court
//! signs on the head it restored.

use crate::exchange::Mesh;
use mycelium::anastome::dialogue;
use mycelium::chronometry::{Place, RejoinRefusal};
use mycelium::cognition::Court;
use std::sync::{Arc, Mutex};

/// The span's top: every place of the line above the head.
const TOP: Place = Place { k: u64::MAX, i: u64::MAX };

/// THE REJOIN, run once at boot before the first ask that can sign. Answers
/// the own receipts held and the peers that answered.
pub(crate) fn rejoin(mesh: &Mesh, court: &Arc<Mutex<Court>>, peers: &[String], tag: &str) -> (usize, usize) {
    let mut held = 0usize;
    let mut answered = 0usize;
    for p in peers.iter().filter(|p| **p != mesh.bind) {
        let mut reached = false;
        // The parent last back-filled: one that is still missing after its
        // back-fill advanced ends the rejoin from this peer (no cycle).
        let mut chased: Option<([u8; 32], u64, u64)> = None;
        loop {
            let Ok(mut h) = crate::court_lock::held!(court) else { return (held, answered) };
            let after = h.run(|k, _| k.own_head_place());
            drop(h);
            let req = dialogue::encode_receipts_request(&mesh.vk, &mesh.vk, after, TOP);
            let Some((reply, _)) = crate::transport::tell_peer_measured(mesh, p, req) else { break };
            if crate::transport::is_deferred(&reply) {
                break;
            }
            let page = match dialogue::decode_receipts_answer(&reply) {
                Some(dialogue::ReceiptsAnswer::Page(page)) => page,
                _ => break,
            };
            reached = true;
            if page.is_empty() {
                break;
            }
            let Ok(mut h) = crate::court_lock::held!(court) else { return (held, answered) };
            let done = h.run(|k, _| k.rejoin_own(&page));
            drop(h);
            held += done.held;
            match &done.stopped {
                // A rejoined receipt cites a peer's receipt this court does
                // not hold: that line is back-filled up to it (47.2 step 3,
                // the exchange's own chase) and the page asked again.
                Some((_, RejoinRefusal::MissingParent { vk, k, i })) if chased != Some((*vk, *k, *i)) => {
                    chased = Some((*vk, *k, *i));
                    let origin = mesh.origin.lock().ok().and_then(|o| *o);
                    let Some(mut budget) = crate::court_lock::held!(court).ok().map(|h| h.into_stroke()) else { break };
                    let pass = mycelium::server::backfill::chase(court, origin, *vk, Place { k: *k, i: *i }, &mut budget, crate::backfill::fetch(mesh, p));
                    if !pass.advanced {
                        eprintln!("[{tag}] rejoin from {p}: the parent at K {k}.{i} did not back-fill");
                        break;
                    }
                    continue;
                }
                Some((at, why)) => {
                    eprintln!("[{tag}] rejoin from {p}: stopped at K {}.{}: {why:?}", at.k, at.i);
                    break;
                }
                None => {}
            }
            if done.held == 0 {
                break;
            }
        }
        match reached {
            true => answered += 1,
            false => eprintln!("[{tag}] rejoin: {p} did not answer"),
        }
    }
    let head = crate::court_lock::held!(court).ok().and_then(|mut h| h.run(|k, _| k.own_head_place()));
    match head {
        Some(at) => eprintln!("[{tag}] rejoin: {held} own receipts taken back from {answered} peers; signs above K {}.{}", at.k, at.i),
        None => eprintln!("[{tag}] rejoin: {held} own receipts taken back from {answered} peers; no own line yet"),
    }
    (held, answered)
}
