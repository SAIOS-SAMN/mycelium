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
    let Some(hi0) = to.checked_sub(1) else { return true };
    let Some(mut budget) = kernel.lock().ok().map(|mut k| k.stroke()) else { return false };
    // THE FETCH FRONTIER: the signer this gap was reported for, and every
    // signer a fetched receipt turns out to cite and this node does not
    // hold. A back-fill of one linear chain cannot close on a mesh whose
    // receipts cite each other — the causal order is a poset — so a hole
    // across signers is pushed here and chased, rather than ending the
    // page (47.2 step 3). The frontier is bounded by the exchange's own
    // stroke, which every fetch spends from.
    let mut frontier: Vec<([u8; 32], u64)> = vec![(vk, hi0)];
    let mut asked: mycelium::network::nerve::Keyed<()> = mycelium::network::nerve::Keyed::new();
    while let Some((who, hi)) = frontier.pop() {
        let held = match kernel.lock() {
            Ok(k) => k.known_head(&who),
            Err(_) => return false,
        };
        let lo = held.map_or(0, |(head_k, _)| head_k + 1);
        if lo > hi {
            continue;
        }
        // One range asked once per back-fill, so a citation two receipts
        // both name is fetched once and a cycle in the frontier ends.
        let mut key = who.to_vec();
        key.extend_from_slice(&lo.to_le_bytes());
        key.extend_from_slice(&hi.to_le_bytes());
        if asked.contains(&key) {
            continue;
        }
        asked.insert(&key, ());
        if !budget.spend() {
            return false;
        }
        let req = dialogue::encode_receipts_request(&mesh.vk, &who, lo, hi);
        let Some((reply, _)) = tell_peer(dest, req) else { return false };
        if crate::transport::is_deferred(&reply) {
            return false;
        }
        let Some(receipts) = dialogue::decode_receipts_reply(&reply) else { return false };
        let (n, missing) = match kernel.lock() {
            Ok(mut k) => k.admit_backfill(&receipts),
            Err(_) => return false,
        };
        let last = lo + n.saturating_sub(1) as u64;
        eprintln!(
            "  court receipt backfill {:02x}{:02x}… {lo}..{last} admitted {n}{}",
            who[0],
            who[1],
            match &missing {
                Some(p) => format!(" cites {:02x}{:02x}… K {}", p.vk[0], p.vk[1], p.k),
                None => String::new(),
            }
        );
        match missing {
            // The page stopped on a citation: fetch that signer up to the
            // K it was cited at, then come back to this range — the
            // receipts are ingested parents before children.
            Some(p) => {
                frontier.push((who, hi));
                frontier.push((p.vk, p.k));
            }
            // Nothing missing and nothing admitted: this peer cannot serve
            // the range, and the gap does not close here.
            None if n == 0 => return false,
            None => frontier.push((who, hi)),
        }
    }
    true
}
