// 47.3, 45.8 — THE FOLD ROUND: the home carries its proposal to a bonded
// peer and seals on the co-signature.
//
//! One proposal at a time, one witness per spend of the court's own
//! stroke: the peers with room on some lane, in key order, until one
//! co-signs. A refusal names its 34.1 constant and is written; an
//! unmeasured answer is silence and the proposal stands for the next
//! orbit.

use crate::exchange::Mesh;
use crate::transport::{note_rtt_ok, note_rtt_timeout, rtt_floor, tell_peer_within};
use mycelium::anastome::dialogue;
use mycelium::cognition::Kernel;
use std::sync::Mutex;

pub(crate) fn fold_round(kernel: &Mutex<Kernel>, mesh: &Mesh) {
    let Some(p) = kernel.lock().ok().and_then(|k| k.fold_proposal().cloned()) else { return };
    let Some(section) = crate::admission::sign_position(kernel, mesh, &[], &p.root.hash) else { return };
    let proposal = dialogue::FoldProposal { k0: p.k0, root: p.root, epochs: p.epochs.clone(), section: section.clone() };
    let Some(body) = dialogue::encode_fold(&proposal) else { return };
    // 47.3: a witness holds bond — room on some lane — and is not this
    // court. The rooms fiber is the peers this court has heard from, in key
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
    let Some(mut budget) = kernel.lock().ok().map(|mut k| k.stroke()) else { return };
    for (vk, dest) in witnesses {
        if !budget.spend() {
            break;
        }
        let hint = match rtt_floor(mesh, &vk) {
            Some(r) => (r * 8).max(32),
            None => (mycelium::physics::aperture_cycle() as u64 * 4).max(32),
        };
        let Some((reply, dt)) = tell_peer_within(&dest, body.clone(), hint) else {
            note_rtt_timeout(mesh, vk);
            continue;
        };
        note_rtt_ok(mesh, vk, dt);
        if crate::transport::is_deferred(&reply) {
            continue;
        }
        match dialogue::decode_fold_reply(&reply) {
            Some(dialogue::FoldWitness::CoSigned { section: w }) => {
                let Ok(mut k) = kernel.lock() else { return };
                match k.commit_fold(section.sig, &w) {
                    Ok(_) => return,
                    Err(why) => {
                        eprintln!("  court fold refused {:02x}{:02x}…: {why}", vk[0], vk[1]);
                        return;
                    }
                }
            }
            Some(dialogue::FoldWitness::Refused { vk, why }) => {
                eprintln!(
                    "  court fold refused {:02x}{:02x}…: {}",
                    vk[0],
                    vk[1],
                    String::from_utf8_lossy(&why)
                );
            }
            Some(dialogue::FoldWitness::Unmeasured { .. }) => {
                eprintln!("  court fold unmeasured by witness {:02x}{:02x}…", vk[0], vk[1]);
            }
            None => {
                eprintln!("  court fold unmeasured: reply undecodable from {:02x}{:02x}…", vk[0], vk[1]);
            }
        }
    }
}
