// 47.2 step 3 — THE BACK-FILL: a gapped signer's receipts, fetched by K
// range from the peer that reported the gap and admitted by their chain,
// page after page until the chain reaches the reported K.
//
//! A gap is not a refusal: the frame that reported it is admitted again
//! once the chain reaches it, and nothing back-filled is cited as a
//! parent. The pages of one gap are paced by the exchange's own stroke —
//! one spend per page — so a peer whose K outruns one page per ask still
//! closes (5.2g, S13.10a, R4).

use crate::exchange::Mesh;
use crate::transport::tell_peer;
use mycelium::anastome::dialogue;
use mycelium::cognition::Kernel;
use std::sync::Mutex;

/// Fetch and admit pages of `vk`'s receipts below `to`, the K of the
/// frame that reported the gap — from the head this court holds for the
/// signer, or from the signer's genesis when it holds none — until the
/// chain reaches `to - 1` or the exchange's stroke budget is spent.
/// Answers whether the chain reached it, so the caller admits the
/// reporting frame again.
pub(crate) fn pages(kernel: &Mutex<Kernel>, mesh: &Mesh, dest: &str, vk: [u8; 32], to: u64) -> bool {
    let Some(hi) = to.checked_sub(1) else { return true };
    let Some(mut budget) = kernel.lock().ok().map(|mut k| k.stroke()) else { return false };
    loop {
        let held = match kernel.lock() {
            Ok(k) => k.known_head(&vk),
            Err(_) => return false,
        };
        let lo = held.map_or(0, |(head_k, _)| head_k + 1);
        if lo > hi {
            return true;
        }
        if !budget.spend() {
            return false;
        }
        let req = dialogue::encode_receipts_request(&mesh.vk, &vk, lo, hi);
        let Some((reply, _)) = tell_peer(dest, req) else { return false };
        if crate::transport::is_deferred(&reply) {
            return false;
        }
        let Some(receipts) = dialogue::decode_receipts_reply(&reply) else { return false };
        let n = match kernel.lock() {
            Ok(mut k) => k.admit_backfill(&receipts),
            Err(_) => return false,
        };
        let last = lo + n.saturating_sub(1) as u64;
        eprintln!(
            "  court receipt backfill {:02x}{:02x}… {lo}..{last} admitted {n}",
            vk[0], vk[1]
        );
        if n == 0 {
            return false;
        }
    }
}
