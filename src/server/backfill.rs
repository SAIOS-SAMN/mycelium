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

use crate::anastome::dialogue::ReceiptsAnswer;
use crate::chronometry::Place;
use crate::cognition::{Kernel, RootAdoption};
use crate::court::admission::Admitted;
use crate::observe::{note_replication, rep, set_replication};
use crate::server::held::held;
use std::sync::Mutex;

/// 78.147 — WHAT A PAGE IS ASKED AS: a catch-up, which a court answers below
/// the signer's fold root with the root, or an era replay — a citation
/// proved by its era — the one ask the cold archive answers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ask {
    CatchUp,
    Replay,
}

/// This node's own `(H_genesis, E_origin)`, which a fold root's home
/// signature is verified at; `None` before it adopted one.
pub type Origin = Option<([u8; 32], u32)>;

/// WHAT ONE PASS OF THE CHASE DID: whether it admitted anything (the
/// caller re-admits the reporting frame on that) and whether the chain
/// REACHED the reported frame's predecessor (the gap closed) or the pass
/// ended short — on its budget, or on a page that closed nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pass {
    pub advanced: bool,
    pub reached: bool,
}

/// The place just before `at` on a line, so a page asked strictly after it
/// opens AT `at`: the act before it in its orbit, or past every act of the
/// orbit before. `None` at the line's genesis: the page opens at the start.
fn place_before(at: Place) -> Option<Place> {
    match (at.k, at.i) {
        (_, i) if i > 0 => Some(Place { k: at.k, i: i - 1 }),
        (0, _) => None,
        (k, _) => Some(Place { k: k - 1, i: u64::MAX }),
    }
}

/// A CITATION INTO A PEER'S CLOSED ERA (78.65, 78.87 R2), proved by the
/// era's own chain: where `p` stands at or below the root this court holds
/// for its signer's fold, the signer's line is asked from `p` up to that
/// root — one page, a fold cuts within one — and the ledger walks it
/// (`Kernel::prove_folded`). `None` where `p` is no citation into a closed
/// era (no fold of its signer held, or `p` past the root): the caller reads
/// it as it always did. `Some(false)` where the stroke, the page or the
/// proof did not stand: the receipt waits for the next hearing, which asks
/// again — nothing in a closed era moves, so the answer can only arrive.
fn prove_folded(
    kernel: &Mutex<Kernel>,
    p: &crate::chronometry::Parent,
    budget: &mut crate::physics::Budget,
    fetch: &mut impl FnMut(&[u8; 32], Option<Place>, Place, Ask) -> Option<ReceiptsAnswer>,
) -> Option<bool> {
    let root = held!(kernel).ok()?.run(|k, _| k.chain().peer_root(&p.at.vk))?;
    if p.at.place() > root {
        return None;
    }
    // THE PROOF IS WALKED PAGE BY PAGE (the wedge read on hypha b,
    // 2026-10-15): it asked one page from the citation to the root and failed
    // wherever the era between them was longer than a frame — and every later
    // fold moved the root further, so a court behind its peers' folds could
    // never prove an old citation, its signer's line never admitted again
    // (a and c admitted 0 of b's receipts in 20 hours). Each page now opens
    // where the walk stands, its progress kept across hearings, one spend a
    // page; the root is read again as each page lands.
    loop {
        if !budget.spend() {
            return Some(false);
        }
        let from = held!(kernel).ok()?.run(|k, _| k.era_walk_from(p));
        let root = held!(kernel).ok()?.run(|k, _| k.chain().peer_root(&p.at.vk))?;
        // THE ONE ERA-REPLAY ASK (78.147): the era's own receipts, from where
        // the walk stands to the root; a root answer proves no citation.
        let Some(ReceiptsAnswer::Page(page)) = fetch(&p.at.vk, place_before(from.at.place()), root, Ask::Replay) else {
            return Some(false);
        };
        match held!(kernel).ok()?.run(|k, _| k.walk_folded(p, &page)) {
            crate::chronometry::EraWalk::Proved => {
                note_replication(Some(p.at.vk), rep::FOLDED_PROVED, 1);
                return Some(true);
            }
            crate::chronometry::EraWalk::Continues => {}
            crate::chronometry::EraWalk::Stalled | crate::chronometry::EraWalk::Broken => return Some(false),
        }
    }
}

/// 78.147 — A ROOT ANSWERED FOR A CATCH-UP, adopted on its two signatures
/// at this node's origin: whether the line now stands on it. A root no
/// origin can verify is refused and counted, never silent.
fn adopt(kernel: &Mutex<Kernel>, origin: Origin, proof: &crate::anastome::dialogue::RootProof) -> bool {
    let Some(origin) = origin else {
        note_replication(Some(proof.signer), rep::ROOT_REFUSED, 1);
        return false;
    };
    held!(kernel).ok().is_some_and(|mut h| h.run(|k, _| k.adopt_root(origin, proof)) == RootAdoption::Adopted)
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
///
/// A CATCH-UP FROM A FOLD ROOT (78.147): a range opening below its signer's
/// fold root is answered with the root, which is adopted on its two
/// signatures, and the signer's line is paged on from it, hot — nothing
/// below a root this court holds is asked again (`known_head` reads it).
pub fn chase(
    kernel: &Mutex<Kernel>,
    origin: Origin,
    vk: [u8; 32],
    to: Place,
    budget: &mut crate::physics::Budget,
    mut fetch: impl FnMut(&[u8; 32], Option<Place>, Place, Ask) -> Option<ReceiptsAnswer>,
) -> Pass {
    let short = |advanced| Pass { advanced, reached: false };
    let mut advanced = false;
    let mut frontier: Vec<([u8; 32], Place)> = vec![(vk, to)];
    let mut asked: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    while let Some((who, upto)) = frontier.pop() {
        let held = match held!(kernel) {
            Ok(mut h) => h.run(|k, _| k.known_head(&who).map(|(at, _)| at)),
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
        let receipts = match fetch(&who, held, upto, Ask::CatchUp) {
            Some(ReceiptsAnswer::Page(page)) => page,
            // The signer's line below its root is the root: adopted, the
            // range is asked again from it — the progress clears what was
            // asked, the budget still bounding the pass.
            Some(ReceiptsAnswer::Root(proof)) => {
                if !adopt(kernel, origin, &proof) {
                    return short(advanced);
                }
                advanced = true;
                asked = crate::network::nerve::Keyed::new();
                frontier.push((who, upto));
                continue;
            }
            None => return short(advanced),
        };
        // The page admitted on the chase's stroke, one read per receipt
        // (78.153b): a stroke spent mid-page keeps what it admitted, and the
        // next pass asks on from the head that moved.
        let (n, missing) = match held!(kernel) {
            Ok(mut h) => h.run(|k, _| k.admit_backfill(&receipts, budget)),
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
                // A CITATION INTO A CLOSED ERA is proved by the era's chain,
                // never fetched as a hole below the head (78.65): proved, the
                // page that stopped on it is read again; unproved, the pass
                // ends short and the next hearing asks.
                match prove_folded(kernel, &p, budget, &mut fetch) {
                    Some(true) => {
                        asked = crate::network::nerve::Keyed::new();
                        frontier.push((who, upto));
                        continue;
                    }
                    Some(false) => return short(advanced),
                    None => {}
                }
                // WHAT IS HELD AT THE CITED PLACE, where the line's head
                // already stands past it: nothing (the line holds a hole
                // below its head) or another receipt (a fork). The chase
                // judged "held" by the head alone (Phase 1 diagnostic).
                let past = held!(kernel).ok().and_then(|mut h| {
                    h.run(|k, _| {
                        let head = k.known_head(&p.at.vk).map(|(at, _)| at)?;
                        (head >= p.at.place()).then(|| k.chain().at(&p.at.vk, p.at.place()).map(|r| r.chain_hash == p.hash))
                    })
                });
                if let Some(at) = past {
                    static SEEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
                    let n = SEEN.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                    if n.is_power_of_two() {
                        let what = match at {
                            None => "no receipt held there (a hole below the head)",
                            Some(true) => "the cited receipt itself",
                            Some(false) => "another receipt (a fork)",
                        };
                        eprintln!("  back-fill cited parent ({n}) {:02x}{:02x}… {:?}: head past it, {what}", p.at.vk[0], p.at.vk[1], p.at.place());
                    }
                }
                frontier.push((who, upto));
                frontier.push((p.at.vk, p.at.place()));
            }
            // Nothing missing and nothing admitted: this source cannot
            // serve the range, and the gap does not close here.
            None if n == 0 => return short(advanced),
            None => frontier.push((who, upto)),
        }
    }
    // REACHED IS READ OFF THE CHAIN, never off an empty frontier. The
    // frontier also empties where a page admitted nothing and named a
    // citation skipped as held, and its own range then skipped as asked —
    // on the development deployment, ~50k passes per court counted "reached" with nothing
    // admitted and the head never cited (H2, 2026-10-03).
    let reached = held!(kernel).ok().and_then(|mut h| h.run(|k, _| k.known_head(&vk))).is_some_and(|(at, _)| at >= to);
    Pass { advanced, reached }
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
    origin: Origin,
    mut admit: impl FnMut() -> Result<Admitted, E>,
    mut stroke: impl FnMut() -> Option<crate::physics::Budget>,
    mut fetch: impl FnMut(&[u8; 32], Option<Place>, Place, Ask) -> Option<ReceiptsAnswer>,
) -> Result<Admitted, E> {
    let mut admitted = admit();
    // EVERY HOLE THE FRAME NAMES IS CHASED, not only its own line's. A
    // frame whose own line is held can still cite a parent of ANOTHER
    // signer this court does not hold (`MissingParent`), and this chased
    // only `Gap`: on the development deployment, c's frames cited b's receipts a did not
    // hold, b's cited c's, and neither line ever advanced at a — "reached"
    // with nothing admitted, the tally reading both holes as one "gap"
    // (2026-09-30). Each hole is chased at most once per hearing, so a
    // hole the chase cannot close ends it.
    let mut chased: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    // The holes a proof by their era was asked for, once each per hearing.
    let mut proving: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    loop {
        let (vk, to) = match &admitted {
            Ok(Admitted::Gap { vk, to, .. }) => (*vk, *to),
            Ok(Admitted::MissingParent(p)) => (p.at.vk, p.at.place()),
            _ => return admitted,
        };
        let mut hole = vk.to_vec();
        to.put_canonical(&mut hole);
        // H2 — A CITATION INTO A FOLDED RANGE, counted once per hearing: the
        // cited parent stands at or below the root this court holds for its
        // signer's own fold, where `shed_folded` cut the line.
        //
        // A CITATION INTO A CLOSED ERA is proved by the era's own chain,
        // never chased as a hole below the head (78.65, 78.87 R2): proved,
        // the frame is admitted again; unproved, it waits for the next
        // hearing — nothing in a closed era moves, so the answer can only
        // arrive. 78.147: asked before the hole is read as chased, since the
        // root a chase adopted is what puts a citation inside an era.
        if let Ok(Admitted::MissingParent(p)) = &admitted {
            let p = *p;
            let folded = held!(kernel).ok().and_then(|mut h| h.run(|k, _| k.chain().peer_root(&p.at.vk))).is_some_and(|root| p.at.place() <= root);
            if folded && !chased.contains(&hole) {
                note_replication(Some(p.at.vk), rep::FOLDED_PARENT, 1);
            }
            if folded && !proving.contains(&hole) {
                proving.insert(&hole, ());
                if let Some(mut budget) = stroke() {
                    match prove_folded(kernel, &p, &mut budget, &mut fetch) {
                        Some(true) => {
                            admitted = admit();
                            continue;
                        }
                        Some(false) => return admitted,
                        None => {}
                    }
                }
            }
        }
        if chased.contains(&hole) {
            // THE SAME HOLE AFTER A CHASE THAT REACHED IT: this court holds
            // the signer's line at the cited place, and holds ANOTHER receipt
            // there — the line forked. Counted as the equivocation it is,
            // per signer, never read as a hole still open.
            if let Ok(Admitted::MissingParent(p)) = &admitted {
                let held = held!(kernel).ok().and_then(|mut h| h.run(|k, _| k.chain().at(&p.at.vk, p.at.place()).map(|r| r.chain_hash)));
                if held.is_some_and(|h| h != p.hash) {
                    note_replication(Some(p.at.vk), rep::BACKFILL_EQUIVOCATION, 1);
                }
            }
            return admitted;
        }
        chased.insert(&hole, ());
        // The lag at the report, in orbits: the orbit after the hole's
        // place against the orbit after the head held. A head held past
        // the report is no lag, and none is set (rule 11: never floored).
        let held = held!(kernel).ok().and_then(|mut h| h.run(|k, _| k.known_head(&vk))).map(|(at, _)| at);
        if let Some(l) = (to.k + 1).checked_sub(held.map_or(0, |h| h.k + 1)) {
            set_replication(vk, rep::LAG, l);
        }
        let mut pass = Pass { advanced: false, reached: false };
        while let Some(mut budget) = stroke() {
            let step = chase(kernel, origin, vk, to, &mut budget, &mut fetch);
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
        match &admitted {
            Ok(Admitted::Parent(p)) => note_replication(Some(p.at.vk), rep::PARENT_AFTER_BACKFILL, 1),
            // What a chase that moved or reached its hole left the frame
            // as, where it is not a parent: the line on the journal at
            // powers of two, until the reading carries it (Phase 1).
            Ok(other) => {
                static SEEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
                let n = SEEN.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                if n.is_power_of_two() {
                    eprintln!(
                        "  back-fill re-admit ({n}): hole {:02x}{:02x}… {to:?} reached {} advanced {} → {other:?}",
                        vk[0], vk[1], pass.reached, pass.advanced
                    );
                }
            }
            Err(_) => {}
        }
    }
}
