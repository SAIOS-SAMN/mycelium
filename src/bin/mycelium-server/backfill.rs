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
    // A BACK-FILL CONVERGES ACROSS EXCHANGES; IT DOES NOT FINISH IN ONE.
    //
    // The original fault was that `pages` returned false whenever its
    // budget ran out, and the caller then never re-admitted the frame that
    // reported the gap — so receipts arrived, were kept, and the citation
    // they were fetched FOR stayed open. My fix for that was to make the
    // pass a MUST-FINISH one and hand it an aperture of strokes: 192 pages
    // of some 150 receipts each, per exchange cycle, per court, chasing a
    // chain that grows faster than the chase. The fleet's load average
    // went to 29.
    //
    // The budget was never the fault. What was wrong is the RETURN: every
    // page admitted is progress that is KEPT — `admit_backfill` writes it
    // into the known set and `known_head` moves — so the next exchange
    // starts further along and the gap closes incrementally. The caller
    // only needs to know that the chain ADVANCED, and re-admit on that; if
    // it is still short, the answer is another `Gap` and another page, at
    // one stroke a time.
    let Some(mut budget) = kernel.lock().ok().map(|mut k| k.stroke()) else {
        return false;
    };
    // Whether this pass moved the chain at all. A pass that moved nothing
    // has nothing for the caller to re-admit against.
    let mut advanced = false;
    // THE FETCH FRONTIER: the signer this gap was reported for, and every
    // signer a fetched receipt turns out to cite and this node does not
    // hold. A back-fill of one linear chain cannot close on a mesh whose
    // receipts cite each other — the causal order is a poset — so a hole
    // across signers is pushed here and chased, rather than ending the
    // page (47.2 step 3).
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
        // The budget ending is not the chase failing: what this pass
        // admitted is kept, and the caller re-admits against it.
        if !budget.spend() {
            return advanced;
        }
        // ASK THE SIGNER THAT OWNS THE CHAIN, not the one that mentioned
        // it. Every frontier entry went to `dest` — the peer whose digest
        // reported the gap — so when a receipt of a cited a receipt of b,
        // c asked A for B'S chain, and a serves another signer's receipts
        // only as far as it happens to have admitted them itself. Measured
        // on the fleet at epoch 11: `backfill ce02… 76..76 admitted 0
        // cites 244b… K 75`, repeating, the chase never closing because
        // the question was put to a node that could not answer it.
        //
        // The mesh already knows where each signer lives (`addrs`, written
        // by `admit_peer_digest`), so the fetch is routed there. `dest`
        // remains the fallback for a signer this node has no address for —
        // a proxy hop is worse than the direct ask and better than none.
        let at = match mesh.addrs.lock() {
            Ok(a) => a.get(&who).cloned(),
            Err(_) => None,
        };
        let to_ask: &str = match &at {
            Some(a) => a.as_str(),
            None => dest,
        };
        let req = dialogue::encode_receipts_request(&mesh.vk, &who, lo, hi);
        let Some((reply, _)) = tell_peer(to_ask, req) else { return false };
        if crate::transport::is_deferred(&reply) {
            return false;
        }
        let Some(receipts) = dialogue::decode_receipts_reply(&reply) else { return false };
        let (n, missing) = match kernel.lock() {
            Ok(mut k) => k.admit_backfill(&receipts),
            Err(_) => return false,
        };
        // A page that admitted nothing spans no range; the line names the
        // step it started at.
        let last = match n.checked_sub(1) {
            Some(span) => lo + span as u64,
            None => lo,
        };
        eprintln!(
            "  court receipt backfill {:02x}{:02x}… {lo}..{last} admitted {n}{}",
            who[0],
            who[1],
            match &missing {
                Some(p) => format!(" cites {:02x}{:02x}… K {}", p.vk[0], p.vk[1], p.k),
                None => String::new(),
            }
        );
        // Anything admitted is progress the known set KEEPS, so the next
        // exchange resumes past it whatever this pass does next.
        if n > 0 {
            advanced = true;
        }
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
            None if n == 0 => return advanced,
            None => frontier.push((who, hi)),
        }
    }
    true
}
