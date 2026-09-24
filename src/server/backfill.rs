// 47.2 step 3, 45.2 — THE BACK-FILL: a gapped signer's receipts, fetched by
// K range and admitted by their chain, page after page until the chain
// reaches the reported K; and the heard head admitted again once it does.
//
//! A gap is not a refusal: the frame that reported it is admitted again
//! once the chain reaches it, and nothing back-filled is cited as a
//! parent. Only the reported head, re-admitted once `to − 1` is held, is a
//! parent (45.2); back-fill never advances a head. The pages of one gap are
//! paced by a stroke — one spend per page.
//!
//! The chase lives here, over a page source the caller hands in, so the
//! server fetches over the wire and a test fetches from a second kernel's
//! ledger, and both run the one loop (ACT1-hold-b, LD0's defect 3b).

use crate::chronometry::KStepReceipt;
use crate::cognition::Kernel;
use crate::court::admission::Admitted;
use crate::observe::{note_replication, rep, set_replication};
use std::sync::Mutex;

/// WHAT ONE PASS OF THE CHASE DID: whether it admitted anything (the
/// caller re-admits the reporting frame on that) and whether the chain
/// REACHED the reported `to − 1` (the gap closed) or the pass ended short
/// — on its budget, or on a page that closed nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pass {
    pub advanced: bool,
    pub reached: bool,
}

/// Fetch and admit pages of `vk`'s receipts below `to`, the K of the frame
/// that reported the gap — from the head this court holds for the signer,
/// or from its genesis when it holds none — until the chain reaches
/// `to − 1` or `budget` is spent. `fetch(signer, lo, hi)` answers a page
/// of the signer's receipts in `lo..=hi`, or `None` where the page could
/// not be had (the peer did not answer, deferred, or sent no page).
///
/// A BACK-FILL CONVERGES ACROSS EXCHANGES; IT DOES NOT FINISH IN ONE. Every
/// page admitted is progress that is KEPT — `admit_backfill` writes it into
/// the known set and `known_head` moves — so the next exchange starts
/// further along (the epoch-11 lesson: a must-finish pass took the fleet's
/// load to 29).
///
/// THE MESH IS A POSET: every signer a fetched receipt cites and this node
/// does not hold is pushed on the frontier and chased, parents before
/// children (47.2 step 3).
pub fn chase(
    kernel: &Mutex<Kernel>,
    vk: [u8; 32],
    to: u64,
    budget: &mut crate::physics::Budget,
    mut fetch: impl FnMut(&[u8; 32], u64, u64) -> Option<Vec<KStepReceipt>>,
) -> Pass {
    let short = |advanced| Pass { advanced, reached: false };
    let Some(hi0) = to.checked_sub(1) else { return Pass { advanced: false, reached: true } };
    let mut advanced = false;
    let mut frontier: Vec<([u8; 32], u64)> = vec![(vk, hi0)];
    let mut asked: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    while let Some((who, hi)) = frontier.pop() {
        let held = match kernel.lock() {
            Ok(k) => k.known_head(&who),
            Err(_) => return short(advanced),
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
            return short(advanced);
        }
        let Some(receipts) = fetch(&who, lo, hi) else { return short(advanced) };
        let (n, missing) = match kernel.lock() {
            Ok(mut k) => k.admit_backfill(&receipts),
            Err(_) => return short(advanced),
        };
        // CV5: the receipts of `who` this page admitted, on the observer —
        // the typed reading, and no per-page print beside it (RC2).
        note_replication(Some(who), rep::BACKFILL, n as u64);
        if n > 0 {
            advanced = true;
        }
        match missing {
            // The page stopped on a citation: fetch that signer up to the
            // K it was cited at, then come back to this range.
            Some(p) => {
                frontier.push((who, hi));
                frontier.push((p.at.vk, p.at.k));
            }
            // Nothing missing and nothing admitted: this source cannot
            // serve the range, and the gap does not close here.
            None if n == 0 => return short(advanced),
            None => frontier.push((who, hi)),
        }
    }
    Pass { advanced, reached: true }
}

/// ONE HEARD HEAD (45.2, 47.2 step 3): the reporting frame admitted; on a
/// gap, the chase over `fetch`, and the frame admitted again where the
/// chase advanced. `admit` admits the one frame — the server's verified
/// digest section, a test's verified receipt — so the same frame is what is
/// admitted both times.
///
/// THE CHASE'S APERTURE IS MEASURED AGAINST THE LAG IT IS CLOSING, never a
/// constant (ACT1-hold-b's fix, rule 5). A pass spends one `stroke` per
/// page; where a stroke ends short and the lag it was closing SHRANK, the
/// pass takes the next stroke and continues; it stops where the chain
/// reaches `to − 1`, or where a stroke closes nothing. So the pages one
/// exchange spends are the lag's own — no more than the gap, and no fewer
/// while the gap is closing — and a signer who outruns one stroke per
/// exchange is still caught: its head re-admitted once `to − 1` is held is
/// the parent 45.2 asks for. Back-fill still never cites and never
/// advances a head. It is not a scheduler (L27): nothing waits and
/// nothing is timed; the pass ends on what the chain reads.
///
/// ACT1-hold-b — THE CHASE ON THE OBSERVER, per signer (CV5): the K lag at
/// the report (a level: the reported K against the head held), whether the
/// pass REACHED `to − 1` or ended short of it, and whether the head was
/// then admitted as a PARENT — the one admission that advances this
/// court's height (45.2).
pub fn hear<E>(
    kernel: &Mutex<Kernel>,
    mut admit: impl FnMut() -> Result<Admitted, E>,
    mut stroke: impl FnMut() -> Option<crate::physics::Budget>,
    mut fetch: impl FnMut(&[u8; 32], u64, u64) -> Option<Vec<KStepReceipt>>,
) -> Result<Admitted, E> {
    let admitted = admit();
    let Ok(Admitted::Gap { vk, to, .. }) = &admitted else { return admitted };
    let (vk, to) = (*vk, *to);
    // The lag at the report: the reported K against the next K this court
    // would hold. A head held past the report is no lag, and none is set
    // (rule 11: never floored to zero).
    let lag_now = || to.checked_sub(kernel.lock().ok().and_then(|k| k.known_head(&vk)).map_or(0, |(k, _)| k + 1));
    let mut lag = lag_now();
    if let Some(l) = lag {
        set_replication(vk, rep::LAG, l);
    }
    let mut pass = Pass { advanced: false, reached: false };
    while let Some(mut budget) = stroke() {
        let step = chase(kernel, vk, to, &mut budget, &mut fetch);
        pass = Pass { advanced: pass.advanced || step.advanced, reached: step.reached };
        if step.reached || !step.advanced {
            break;
        }
        // Continue only while the lag this pass is closing shrinks.
        let after = lag_now();
        match (after, lag) {
            (Some(a), Some(b)) if a < b => lag = after,
            _ => break,
        }
    }
    note_replication(
        Some(vk),
        match pass.reached {
            true => rep::BACKFILL_REACHED,
            false => rep::BACKFILL_BUDGET,
        },
        1,
    );
    // Admitted again where the pass moved the chain or found it already at
    // `to − 1` — as the server's exchange always re-admitted.
    if !pass.advanced && !pass.reached {
        return admitted;
    }
    let again = admit();
    if let Ok(Admitted::Parent(_)) = &again {
        note_replication(Some(vk), rep::PARENT_AFTER_BACKFILL, 1);
    }
    again
}
