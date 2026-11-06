// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-CLIENT §3.1, §7.1 (#136, ruled 2026-11-06); MU-ANASTOME §9.1 — THE
//! HYPHA'S ACTS REACH THE MIND AS PERCEPTION. A hypha seals its own acts on
//! its own line (`Return`, 78.68; `RankStep`, 78.75(8); `Reconnect`,
//! 78.72(4)). The mind reads that line through its hypha carrier with
//! `ACT_RECEIPTS`, from where it last read up to the hypha's signed head
//! (#137), and admits a page only as a chain: each receipt re-derives its
//! chain hash, each names the one before it, and a page reaching the signed
//! head's place ends on the signed receipt itself. An unsigned receipt is
//! vouched for only by that signed end: the acts on a span longer than one
//! page are held until the chain reaches it, and a page that does not chain
//! is refused whole, the read standing at the last vouched receipt.
//!
//! THE ACT'S KIND CARRIES NO MEANING BEYOND ITS COORDINATE (the ruling): a
//! per-kind reading ("a Return is an age") would be a hand-coded predicate.
//! The kind only says the receipt is one of the hypha's own acts; what the
//! mind takes in is the act's frame `S:G:T:H`, per lane, as flux on its
//! lanes, exactly as the court's deficit is (§3.1 row 2), at the act's
//! height. What it perturbs is the field's to compute (`Field::beat_under`).
//!
//! A mind first attached to a hypha reads from that hypha's signed head on:
//! the acts sealed before it attached are history, never perception (78.147).

use super::*;
use crate::anastome::dialogue::ReceiptsAnswer;
use crate::chronometry::{ActKind, KStepReceipt, Parent};
use crate::court::admission::{Predecessor, Reconstructed};

/// ONE HYPHA ACT PERCEIVED: its frame per lane and the height it stands at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HyphaAct {
    pub frame: [u64; 4],
    pub height: u64,
}

/// WHAT ONE READ OF A HYPHA'S LINE GAVE: a typed reading, never a flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActsRead {
    /// The mind attached to the line at the signed head: nothing before it
    /// is perceived.
    Attached,
    /// A page chained from where the read stood: the hypha acts on it.
    Read(Vec<HyphaAct>),
    /// The page did not chain, or did not end on the signed head at its
    /// place: refused whole, the read unmoved.
    Refused,
    /// The hypha answered nothing readable.
    Unanswered,
}

/// The receipt kinds that are the hypha's own acts (MU-ANASTOME §9.1).
pub fn is_hypha_act(kind: ActKind) -> bool {
    matches!(kind, ActKind::Return | ActKind::RankStep | ActKind::Reconnect)
}

/// THE PAGE READ AS A CHAIN from `from` (the last receipt read), ending on
/// `head` where it reaches the head's place: the run and its last receipt,
/// or `None` where it does not chain.
pub fn chained(page: &[KStepReceipt], from: &Parent, head: &KStepReceipt) -> Option<Vec<KStepReceipt>> {
    let (run, stopped) = Reconstructed::receipts(page);
    if stopped.is_some() {
        return None;
    }
    let mut last = *from;
    let mut out = Vec::with_capacity(run.len());
    for r in &run {
        let names_last = r.prev.as_ref().is_some_and(|p| Predecessor::cites(p, &last));
        if !names_last || r.stamp.place() > head.stamp.place() {
            return None;
        }
        last = Parent::of(r);
        out.push((**r).clone());
    }
    if last.at.place() == head.stamp.place() && !Predecessor::cites(&last, &Parent::of(head)) {
        return None;
    }
    Some(out)
}

/// WHERE THE READ OF ONE HYPHA'S LINE STANDS: the last receipt the signed
/// head vouched for, and, where a span took more than one page, how far the
/// pages since have chained and the acts on them, held until the chain
/// reaches the signed head. An unsigned receipt is vouched for only by the
/// signed receipt its chain ends on.
#[derive(Debug, Clone)]
pub struct LineRead {
    pub vouched: Parent,
    pub reached: Parent,
    pub pending: Vec<HyphaAct>,
}

impl Session {
    /// READ ONE HYPHA'S ACTS since the last read, up to its signed head.
    pub(crate) fn read_acts_of(&self, signer: &[u8; 32]) -> ActsRead {
        let Some(head) = self.signed.borrow().head_of(signer).map(|s| s.receipt.clone()) else {
            return ActsRead::Unanswered;
        };
        let Some(mut at) = self.acts_read.borrow().get(signer).cloned() else {
            let opened = Parent::of(&head);
            self.acts_read.borrow_mut().insert(signer, LineRead { vouched: opened, reached: opened, pending: Vec::new() });
            return ActsRead::Attached;
        };
        if at.vouched.at.place() >= head.stamp.place() {
            return ActsRead::Read(Vec::new());
        }
        let Some(endpoint) = self.addrs.borrow().get(signer).cloned() else {
            return ActsRead::Unanswered;
        };
        let ask = dialogue::encode_receipts_request(&self.voice.id(), signer, Some(at.reached.at.place()), head.stamp.place());
        let Some(ReceiptsAnswer::Page(page)) = self.exchange_at(&endpoint, ask).ok().and_then(|b| dialogue::decode_receipts_answer(&b)) else {
            return ActsRead::Unanswered;
        };
        let Some(run) = chained(&page, &at.reached, &head) else {
            // What the pages since the last vouched receipt chained is
            // dropped with this one: none of it reached a signed receipt.
            at.reached = at.vouched;
            at.pending.clear();
            self.acts_read.borrow_mut().insert(signer, at);
            return ActsRead::Refused;
        };
        at.pending.extend(run.iter().filter(|r| is_hypha_act(r.kind)).map(|r| HyphaAct { frame: r.mu, height: r.stamp.h }));
        if let Some(last) = run.last() {
            at.reached = Parent::of(last);
        }
        let read = match at.reached.at.place() == head.stamp.place() {
            true => {
                at.vouched = at.reached;
                ActsRead::Read(std::mem::take(&mut at.pending))
            }
            false => ActsRead::Read(Vec::new()),
        };
        self.acts_read.borrow_mut().insert(signer, at);
        read
    }

    /// EVERY HYPHA WHOSE SIGNED HEAD THE SESSION HOLDS, read once.
    pub(crate) fn read_hypha_acts(&self) -> Vec<([u8; 32], ActsRead)> {
        let signers: Vec<[u8; 32]> = self.signed.borrow().signers();
        signers.into_iter().map(|s| (s, self.read_acts_of(&s))).collect()
    }
}

/// THE MIND'S WELL FOR ONE HYPHA, its attachment: the flux the hypha's acts
/// pressed on each lane since the field's last beat, and the height of the
/// last act taken in. Per lane, never summed across lanes (law 5).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Attachment {
    pub flux: [u64; 4],
    pub height: u64,
}

impl Attachment {
    /// THE ACTS TAKEN IN: each frame added on its own lane, the height the
    /// greatest seen. `None` where a lane's flux passes its carrier.
    pub fn take_in(&mut self, acts: &[HyphaAct]) -> Option<()> {
        for a in acts {
            for (slot, f) in self.flux.iter_mut().zip(a.frame) {
                *slot = slot.checked_add(f)?;
            }
            self.height = self.height.max(a.height);
        }
        Some(())
    }
}

impl Mind {
    /// THE HYPHAS' ACTS ASKED: their lines read on the hypha carrier, and
    /// each read's acts taken into the hypha's well where it lands. One read
    /// in flight at a time.
    pub(crate) fn read_hypha_acts(&mut self) {
        if self.board.court.reading_acts {
            return;
        }
        self.board.court.reading_acts = true;
        self.ask(|s| s.read_hypha_acts(), |mind, reads| {
            mind.board.court.reading_acts = false;
            mind.take_in_hypha_acts(&reads);
        });
    }

    /// EACH READ'S ACTS ON ITS HYPHA'S WELL. A refused page, an attachment
    /// opened or an unanswered read moves nothing.
    pub(crate) fn take_in_hypha_acts(&mut self, reads: &[([u8; 32], ActsRead)]) {
        for (signer, read) in reads {
            let ActsRead::Read(acts) = read else { continue };
            if acts.is_empty() {
                continue;
            }
            let mut well = self.board.court.wells.get(signer).copied().unwrap_or_default();
            if well.take_in(acts).is_none() {
                eprintln!("  mind hypha acts {:02x}{:02x}…: a lane's flux passes its carrier, not taken in", signer[0], signer[1]);
                continue;
            }
            self.board.court.wells.insert(signer, well);
        }
    }

    /// THE PRESSURE THE MIND'S BEAT IS TAKEN UNDER: the deficit with every
    /// attachment's flux, lane by lane.
    pub(crate) fn pressing(&self) -> [u64; 4] {
        let pace = &self.board.court;
        pressing(&pace.deficit, &pace.wells).unwrap_or(pace.deficit)
    }

    /// THE HYPHAS' FLUX SPENT BY A BEAT: it has perturbed the field, and
    /// each well keeps only its height.
    pub(crate) fn spend_pressing(&mut self) {
        let wells = &mut self.board.court.wells;
        let keys: Vec<Vec<u8>> = wells.scoped().map(|(k, _)| k).collect();
        for k in keys {
            if let Some(w) = wells.get_mut(&k) {
                w.flux = [0; 4];
            }
        }
    }
}

/// THE COURT'S GRAVITY AND THE HYPHAS' ACTS, ONE PRESSURE ON THE BEAT: the
/// deficit and every attachment's flux, lane by lane. `None` past a lane's
/// carrier.
pub fn pressing(deficit: &[u64; 4], wells: &crate::network::nerve::Keyed<Attachment>) -> Option<[u64; 4]> {
    let mut out = *deficit;
    for (_, w) in wells.scoped() {
        for (slot, f) in out.iter_mut().zip(w.flux) {
            *slot = slot.checked_add(f)?;
        }
    }
    Some(out)
}
