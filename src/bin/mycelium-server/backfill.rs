// 47.2 step 3 — THE BACK-FILL: one page of a gapped signer's receipts,
// fetched by K range from the peer that reported the gap and admitted by
// their chain.
//
//! A gap is not a refusal: the frame that reported it is admitted again
//! once the chain reaches it, and nothing back-filled is cited as a
//! parent. One page per gap reported, on the exchange that reported it,
//! so the fetch is paced by the exchange's own orbit (5.2g, S13.10).

use crate::exchange::Mesh;
use crate::transport::tell_peer;
use mycelium::anastome::dialogue;
use mycelium::cognition::Kernel;
use std::sync::Mutex;

/// Fetch and admit one page of `vk`'s receipts below `to`, the K of the
/// frame that reported the gap: from the head this court holds for the
/// signer, or from the signer's genesis when it holds none.
pub(crate) fn page(kernel: &Mutex<Kernel>, mesh: &Mesh, dest: &str, vk: [u8; 32], to: u64) {
    let held = match kernel.lock() {
        Ok(k) => k.known_head(&vk),
        Err(_) => return,
    };
    let lo = held.map_or(0, |(head_k, _)| head_k + 1);
    let Some(hi) = to.checked_sub(1) else { return };
    if hi < lo {
        return;
    }
    let req = dialogue::encode_receipts_request(&mesh.vk, &vk, lo, hi);
    let Some((reply, _)) = tell_peer(dest, req) else { return };
    if crate::transport::is_deferred(&reply) {
        return;
    }
    let Some(receipts) = dialogue::decode_receipts_reply(&reply) else { return };
    let n = match kernel.lock() {
        Ok(mut k) => k.admit_backfill(&receipts),
        Err(_) => return,
    };
    let last = lo + n.saturating_sub(1) as u64;
    eprintln!(
        "  court receipt backfill {:02x}{:02x}… {lo}..{last} admitted {n}",
        vk[0], vk[1]
    );
}
