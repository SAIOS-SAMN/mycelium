// 47.2 step 3, 45.2 — THE BACK-FILL: a gapped signer's receipts, fetched by
// place range and admitted by their chain, page after page until the chain
// holds the reported frame's predecessor; and the heard head admitted again
// once it does.
//
//! A gap is not a refusal: the frame that reported it is admitted again
//! once the chain reaches it, and nothing back-filled is cited as a
//! parent. Only the reported head, re-admitted once its predecessor is
//! held, is a parent (45.2); back-fill never advances a head. The pages of
//! one gap are paced by a stroke — one spend per page.
//!
//! The chase lives here, over a page source the caller hands in, so the
//! server fetches over the wire and a test fetches from a second kernel's
//! ledger, and both run the one loop (ACT1-hold-b, LD0's defect 3b).

use crate::chronometry::{KStepReceipt, Place};
use crate::cognition::Kernel;
use crate::court::admission::Admitted;
use crate::observe::{note_replication, rep, set_replication};
use std::sync::Mutex;

/// WHAT ONE PASS OF THE CHASE DID: whether it admitted anything (the
/// caller re-admits the reporting frame on that) and whether the chain
/// REACHED the reported frame's predecessor (the gap closed) or the pass
/// ended short — on its budget, or on a page that closed nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pass {
    pub advanced: bool,
    pub reached: bool,
}

/// Fetch and admit pages of `vk`'s receipts up to `to`, the place of the
/// reported frame's predecessor — strictly after the head this court holds
/// for the signer, or from its genesis when it holds none — until the
/// chain holds `to` or `budget` is spent. `fetch(signer, after, upto)`
/// answers a page of the signer's receipts strictly after `after` (from
/// its genesis where `None`) up to `upto`, or `None` where the page could
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
    to: Place,
    budget: &mut crate::physics::Budget,
    mut fetch: impl FnMut(&[u8; 32], Option<Place>, Place) -> Option<Vec<KStepReceipt>>,
) -> Pass {
    let short = |advanced| Pass { advanced, reached: false };
    let mut advanced = false;
    let mut frontier: Vec<([u8; 32], Place)> = vec![(vk, to)];
    let mut asked: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    while let Some((who, upto)) = frontier.pop() {
        let held = match kernel.lock() {
            Ok(k) => k.known_head(&who).map(|(at, _)| at),
            Err(_) => return short(advanced),
        };
        if held.is_some_and(|h| h >= upto) {
            continue;
        }
        // One range asked once per back-fill, so a citation two receipts
        // both name is fetched once and a cycle in the frontier ends.
        let mut key = who.to_vec();
        match held {
            None => key.push(0),
            Some(h) => {
                key.push(1);
                h.put_canonical(&mut key);
            }
        }
        upto.put_canonical(&mut key);
        if asked.contains(&key) {
            continue;
        }
        asked.insert(&key, ());
        // The budget ending is not the chase failing: what this pass
        // admitted is kept, and the caller re-admits against it.
        if !budget.spend() {
            return short(advanced);
        }
        let Some(receipts) = fetch(&who, held, upto) else { return short(advanced) };
        let (n, missing) = match kernel.lock() {
            Ok(mut k) => k.admit_backfill(&receipts, budget),
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
            // place it was cited at, then come back to this range.
            Some(p) => {
                frontier.push((who, upto));
                frontier.push((p.at.vk, p.at.place()));
            }
            // Nothing missing and nothing admitted: this source cannot
            // serve the range, and the gap does not close here.
            None if n == 0 => return short(advanced),
            None => frontier.push((who, upto)),
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
/// THE CHASE'S APERTURE IS MEASURED AGAINST THE GAP IT IS CLOSING, never a
/// constant (ACT1-hold-b's fix, rule 5). A pass spends one `stroke` per
/// page; where a stroke ends short and the head held for the signer
/// ADVANCED — a page admitted something — the pass takes the next stroke
/// and continues; it stops where the chain holds `to`, or where a stroke
/// admits nothing. Progress is read in PLACES, never in orbits: a gap
/// inside one orbit closes page by page while its `K` stands still. So the
/// pages one exchange spends are the gap's own — no more than the gap, and
/// no fewer while the gap is closing — and a signer who outruns one stroke
/// per exchange is still caught: its head re-admitted once `to` is held is
/// the parent 45.2 asks for. Back-fill still never cites and never
/// advances a head. It is not a scheduler (L27): nothing waits and
/// nothing is timed; the pass ends on what the chain reads.
///
/// ACT1-hold-b — THE CHASE ON THE OBSERVER, per signer (CV5): the lag at
/// the report in orbits (a level: the reported frame's orbit against the
/// head held), whether the pass REACHED `to` or ended short of it, and
/// whether the head was then admitted as a PARENT — the one admission that
/// advances this court's height (45.2).
pub fn hear<E>(
    kernel: &Mutex<Kernel>,
    mut admit: impl FnMut() -> Result<Admitted, E>,
    mut stroke: impl FnMut() -> Option<crate::physics::Budget>,
    mut fetch: impl FnMut(&[u8; 32], Option<Place>, Place) -> Option<Vec<KStepReceipt>>,
) -> Result<Admitted, E> {
    let mut admitted = admit();
    // EVERY HOLE THE FRAME NAMES IS CHASED, not only its own line's. A
    // frame whose own line is held can still cite a parent of ANOTHER
    // signer this court does not hold (`MissingParent`), and this chased
    // only `Gap`: on the fleet, c's frames cited b's receipts a did not
    // hold, b's cited c's, and neither line ever advanced at a — "reached"
    // with nothing admitted, the tally reading both holes as one "gap"
    // (2026-09-30). Each hole is chased at most once per hearing, so a
    // hole the chase cannot close ends it.
    let mut chased: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    loop {
        let (vk, to) = match &admitted {
            Ok(Admitted::Gap { vk, to, .. }) => (*vk, *to),
            Ok(Admitted::MissingParent(p)) => (p.at.vk, p.at.place()),
            _ => return admitted,
        };
        let mut hole = vk.to_vec();
        to.put_canonical(&mut hole);
        if chased.contains(&hole) {
            return admitted;
        }
        chased.insert(&hole, ());
        // The lag at the report, in orbits: the orbit after the hole's
        // place against the orbit after the head held. A head held past
        // the report is no lag, and none is set (rule 11: never floored).
        let held = kernel.lock().ok().and_then(|k| k.known_head(&vk)).map(|(at, _)| at);
        if let Some(l) = (to.k + 1).checked_sub(held.map_or(0, |h| h.k + 1)) {
            set_replication(vk, rep::LAG, l);
        }
        let mut pass = Pass { advanced: false, reached: false };
        while let Some(mut budget) = stroke() {
            let step = chase(kernel, vk, to, &mut budget, &mut fetch);
            pass = Pass { advanced: pass.advanced || step.advanced, reached: step.reached };
            // Continue only while a stroke moved the head held for the signer.
            if step.reached || !step.advanced {
                break;
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
        // Admitted again where the pass moved the chain or found it already
        // at `to` — as the server's exchange always re-admitted.
        if !pass.advanced && !pass.reached {
            return admitted;
        }
        admitted = admit();
        if let Ok(Admitted::Parent(p)) = &admitted {
            note_replication(Some(p.at.vk), rep::PARENT_AFTER_BACKFILL, 1);
        }
    }
}
