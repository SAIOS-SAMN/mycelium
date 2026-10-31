// 47.3, 45.8 — THE FOLD ROUND: the home carries its proposal to a bonded
// peer and seals on the co-signature.
//
//! One proposal at a time, one witness per spend of the court's own
//! stroke: the peers with room on some lane, in key order, until one
//! co-signs. A refusal names its 34.1 constant and is written; an
//! unmeasured answer is silence and the proposal stands, carried to that
//! witness again once this court's K has moved (`Mesh::fold_unmeasured_at`).

use crate::exchange::Mesh;
use crate::transport::{note_rtt_ok, note_rtt_timeout, rtt_floor, tell_peer, tell_peer_within};
use mycelium::anastome::dialogue;
use mycelium::cognition::Court;
use mycelium::observe::{self, FoldOutcome};
use std::sync::Mutex;

pub(crate) fn fold_round(kernel: &Mutex<Court>, mesh: &Mesh) {
    // A proposal whose rows moved beneath it is dropped before it is
    // carried again (`pending_fold_stands`).
    let Some(p) = crate::held::held!(kernel).ok().and_then(|mut h| h.run(|k, _| k.pending_fold_stands().then(|| k.fold_proposal().cloned()).flatten())) else {
        return;
    };
    let Some(section) = crate::admission::sign_position(kernel, mesh, &mycelium::transducer::stalk::Invariant::default(), &p.root.hash) else { return };
    let proposal = dialogue::FoldProposal { after: p.after, root: p.root, epochs: p.epochs.clone(), section: section.clone() };
    let Some(body) = dialogue::encode_fold(&proposal) else { return };
    // 47.3: a witness holds bond — room on some lane — and is not this
    // court. The rooms fibre is the peers this court has heard from, in key
    // order; each is asked once per round, one per spend.
    let witnesses: Vec<([u8; 32], String)> = match (mesh.rooms.lock(), mesh.addrs.lock()) {
        (Ok(rooms), Ok(addrs)) => rooms
            .scoped()
            .filter(|(vk, room)| vk.as_slice() != mesh.vk.as_slice() && room.iter().any(|&r| r > 0))
            .filter_map(|(vk, _)| {
                let key = <[u8; 32]>::try_from(vk.as_slice()).ok()?;
                Some((key, addrs.get(&vk)?.clone()))
            })
            .collect(),
        _ => return,
    };
    // A round with no peer holding room carries the proposal nowhere:
    // counted, never silent (LD0).
    if witnesses.is_empty() {
        observe::note_fold(FoldOutcome::NoWitness);
        return;
    }
    // A witness that left this court's fold unmeasured at this K is not
    // carried it again until K moves: its answer is instant, and re-asked
    // every cycle it was a storm that starved the doors (2026-10-22).
    let Some(k_now) = crate::held::held!(kernel).ok().map(|mut h| h.run(|k, _| k.k_step())) else { return };
    let paced = |witness: &[u8; 32]| mesh.fold_unmeasured_at.lock().is_ok_and(|m| m.get(witness) == Some(&k_now));
    let unmeasured = |witness: &[u8; 32]| {
        if let Ok(mut m) = mesh.fold_unmeasured_at.lock() {
            m.insert(witness, k_now);
        }
    };
    // The round's stroke: a hold's, handed on whole (78.153b).
    let Some(mut budget) = crate::held::held!(kernel).ok().map(|h| h.into_stroke()) else { return };
    for (vk, dest) in witnesses {
        if paced(&vk) {
            continue;
        }
        if !budget.spend() {
            break;
        }
        // The witness is asked within an aperture of its measured round trip
        // (`transport_deadline`), and an unmeasured one as every wire is
        // first asked, its answer measuring it — no patience chosen here
        // (PH1).
        let asked = match rtt_floor(mesh, &vk) {
            Some(r) => tell_peer_within(&dest, body.clone(), r),
            None => tell_peer(&dest, body.clone()),
        };
        // Every answer is counted by outcome on the court's work reading;
        // silence, a deferral and an undecodable reply measured nothing.
        let Some((reply, dt)) = asked else {
            note_rtt_timeout(mesh, vk);
            observe::note_fold(FoldOutcome::Unmeasured);
            unmeasured(&vk);
            continue;
        };
        note_rtt_ok(mesh, vk, dt);
        if crate::transport::is_deferred(&reply) {
            observe::note_fold(FoldOutcome::Unmeasured);
            unmeasured(&vk);
            continue;
        }
        match dialogue::decode_fold_reply(&reply) {
            Some(dialogue::FoldWitness::CoSigned { section: w }) => {
                let Ok(mut h) = crate::held::held!(kernel) else { return };
                // 78.147: the root is kept with both signed positions, what a
                // catch-up below it is answered with.
                match h.run(|k, _| k.commit_fold_proved(&section, &w)) {
                    Ok(_) => {
                        observe::note_fold(FoldOutcome::Folded);
                        return;
                    }
                    Err(why) => {
                        observe::note_fold(FoldOutcome::Refused);
                        eprintln!("  court fold refused {:02x}{:02x}…: {why}", vk[0], vk[1]);
                        return;
                    }
                }
            }
            Some(dialogue::FoldWitness::Refused { vk, why }) => {
                observe::note_fold(FoldOutcome::Refused);
                eprintln!(
                    "  court fold refused {:02x}{:02x}…: {}",
                    vk[0],
                    vk[1],
                    String::from_utf8_lossy(&why)
                );
                // A witness that re-derived other nets: the proposal is not
                // carried again as it stood.
                if why == mycelium::verdict::REFUSE_FOLD_INEXACT.as_bytes() {
                    if let Ok(mut h) = crate::held::held!(kernel) {
                        h.run(|k, _| k.drop_fold_proposal());
                    }
                    return;
                }
            }
            Some(dialogue::FoldWitness::Unmeasured { .. }) => {
                observe::note_fold(FoldOutcome::Unmeasured);
                unmeasured(&vk);
                eprintln!("  court fold unmeasured by witness {:02x}{:02x}…", vk[0], vk[1]);
            }
            None => {
                observe::note_fold(FoldOutcome::Unmeasured);
                unmeasured(&vk);
                eprintln!("  court fold unmeasured: reply undecodable from {:02x}{:02x}…", vk[0], vk[1]);
            }
        }
    }
}
