// 78.151 — THE PUBLISHED READING: what a court's door serves without its
// kernel, taken whole at the court stroke under the lock the stroke holds.

use super::runs::{Copied, Runs};
use crate::anastome::dialogue::RootProof;
use crate::chronometry::{CoreStamp, Era, Filed, KStepReceipt, LineTally, Place, Stamp};
use crate::cognition::court::era::LineHolder;
use crate::cognition::Court;
use crate::economy::{Holdings, Touched};
use crate::network::nerve::Keyed;
use std::sync::Arc;

/// The place past every place: a line read to its end.
const END: Place = Place { k: u64::MAX, i: u64::MAX };

/// The shards the accounts are held in, by the first byte of the party's
/// key: a moved account copies its shard, never the whole.
pub const ACCOUNT_SHARDS: usize = 256;

/// One receipt as published, with the bytes of its tape.
#[derive(Debug, Clone)]
pub(crate) struct Kept {
    pub(crate) r: KStepReceipt,
    pub(crate) tape: u64,
}

/// ONE SIGNER'S LINE AS PUBLISHED: its hot line in runs, the root it
/// stands on here, and the tally it was read at.
#[derive(Debug, Clone, Default)]
pub(crate) struct Line {
    pub(crate) tally: LineTally,
    pub(crate) standing: Option<Place>,
    pub(crate) run: Runs<Kept>,
}

/// ONE ERA AS PUBLISHED: its place range, and where its payload is read —
/// its span of the cold segment, or the payload itself where it was never
/// filed.
#[derive(Debug, Clone)]
pub(crate) struct EraRef {
    pub(crate) root: Place,
    pub(crate) first: Place,
    pub(crate) filed: Option<Filed>,
    pub(crate) payload: Option<Arc<[KStepReceipt]>>,
    pub(crate) tape: u64,
}

/// ONE SIGNER'S ERAS AS PUBLISHED, in root order, and the tally read.
#[derive(Debug, Clone, Default)]
pub(crate) struct EraRun {
    pub(crate) tally: LineTally,
    pub(crate) run: Runs<EraRef>,
}

/// WHAT ONE PUBLICATION COPIED, counted exactly (law 9: no clock): the
/// receipts and the eras, with the bytes of the tapes (`bytes`); the
/// accounts re-read and the accounts a moved shard copied, each a key and
/// three lane vectors; and the per-signer entries re-made. A count past its
/// carrier is named (`passed`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PublishCost {
    pub receipts: u64,
    pub eras: u64,
    pub accounts: u64,
    pub keys: u64,
    pub bytes: u64,
    pub passed: bool,
}

impl PublishCost {
    /// Every entry copied, of every kind; `None` past its carrier.
    pub fn entries(&self) -> Option<u64> {
        self.receipts.checked_add(self.eras)?.checked_add(self.accounts)?.checked_add(self.keys)
    }

    fn receipts(&mut self, c: Copied) {
        self.passed |= c.passed;
        self.receipts = self.add(self.receipts, c.members);
        self.bytes = self.add(self.bytes, c.weight);
    }

    fn eras(&mut self, c: Copied) {
        self.passed |= c.passed;
        self.eras = self.add(self.eras, c.members);
        self.bytes = self.add(self.bytes, c.weight);
    }

    fn accounts(&mut self, n: u64) {
        self.accounts = self.add(self.accounts, n);
    }

    fn keys(&mut self, n: u64) {
        self.keys = self.add(self.keys, n);
    }

    fn add(&mut self, a: u64, b: u64) -> u64 {
        match a.checked_add(b) {
            Some(s) => s,
            None => {
                self.passed = true;
                a
            }
        }
    }
}

/// 78.151 — THE READING A COURT PUBLISHES at its stroke, read by its door
/// without the kernel: each signer's hot line and fold root, its eras, the
/// root proofs, every account, and the core clock — all at one head
/// (`height`). Immutable once built; the next publication shares every
/// piece that did not move.
pub struct Published {
    pub(crate) head: Option<Stamp>,
    pub(crate) court: [u8; 32],
    pub(crate) identity: [u8; 32],
    pub(crate) clock: Option<CoreStamp>,
    pub(crate) lines: Keyed<Arc<Line>>,
    pub(crate) eras: Keyed<Arc<EraRun>>,
    pub(crate) eras_path: Option<Arc<std::path::Path>>,
    pub(crate) proofs: Arc<Keyed<RootProof>>,
    pub(crate) proofs_mark: u64,
    pub(crate) accounts: Vec<Arc<Keyed<Holdings>>>,
    /// 78.154 — each universe's ledger chain, shared with the kernel's
    /// atlas until it grows, and the atlas's mark it was read at.
    pub(crate) ledgers: Keyed<Arc<Vec<crate::anastome::dialogue::atlas::HeldEntry>>>,
    pub(crate) atlas_mark: u64,
    /// 78.153b — the treasury's journal, shared with the last reading where
    /// its mark stands: a witness's `ACT_ROWS` is answered off it, never
    /// under the kernel (a fold compacts the journal, so it stays small).
    pub(crate) journal: Arc<Vec<crate::economy::Entry>>,
    pub(crate) journal_mark: u64,
    pub(crate) cost: PublishCost,
    /// 78.153b — THE COURT'S OWN READS (`Own`): its cover, room, bond, ruled
    /// cover and head receipt, read off the shelf by the door and the
    /// exchange without the kernel.
    pub(crate) own: Own,
}

/// 78.153b — WHAT THE COURT READS OF ITSELF, published: each piece read at
/// the stroke as the kernel answers it, and shared with the last reading
/// where it did not move.
#[derive(Clone)]
pub(crate) struct Own {
    /// `Court::own_cover_charts`, copied where `own_cover_gen` moved.
    pub(crate) cover: Arc<Vec<(u8, crate::transducer::stalk::Invariant)>>,
    pub(crate) cover_mark: u64,
    /// `Court::lane_room`, read at every publication (its credit walk is
    /// the court's own discoveries; the memo sealed at a generation is owed,
    /// `room.rs`).
    pub(crate) room: Option<[u64; 4]>,
    /// Whether this court holds bond (`Treasury::bond_of` is some): the
    /// witness's room to co-sign a fold (`verify_fold`).
    pub(crate) bonded: bool,
    /// The court's own voice, whose digests asks are answered with the
    /// ruled cover (`ruled_digests_for`).
    pub(crate) voice: Option<[u8; 32]>,
    /// The ruled cover: root, digests and syllabus seed, read where
    /// `ruled_gen` moved.
    pub(crate) ruled: Arc<Ruled>,
    pub(crate) ruled_mark: u64,
    /// `ReceiptChain::head`, the receipt at `head`.
    pub(crate) head_receipt: Option<KStepReceipt>,
}

/// THE RULED COVER AS PUBLISHED (`Court::ruled_reading`).
pub(crate) struct Ruled {
    pub(crate) root: [u8; 32],
    pub(crate) digests: Vec<crate::transducer::sheaf::StratumDigest>,
    pub(crate) seed: Vec<Vec<u8>>,
}

/// A receipt's tape bytes; a tape that does not write weighs nothing and is
/// served as the hot line serves it, refused where the page writes it.
fn kept(r: KStepReceipt) -> Kept {
    let tape = tape_of(&r);
    Kept { r, tape }
}

fn tape_of(r: &KStepReceipt) -> u64 {
    let mut tape = Vec::new();
    match r.write(&mut tape) {
        Some(()) => tape.len() as u64,
        None => 0,
    }
}

/// An era's descriptor as published: its span, or its payload in hand.
fn era_ref(e: &Era) -> EraRef {
    let payload = match e.at {
        Some(_) => None,
        None => Some(Arc::from(e.receipts.clone())),
    };
    let held: u64 = e.receipts.iter().map(tape_of).sum();
    EraRef { root: e.root.at, first: e.first, filed: e.at, payload, tape: Era::DESCRIPTOR as u64 + held }
}

/// The shard a party's account is held in.
pub(crate) fn shard_of(who: &[u8; 32]) -> usize {
    usize::from(who[0])
}

impl Court {
    /// 78.151 — PUBLISH THE READING the door serves without the kernel, at
    /// this court's head: from `prev`, every piece that did not move is
    /// shared, a line that only grew at its end is extended by what it grew,
    /// and anything else is read again whole. `None` where nothing moved
    /// since `prev` — the door goes on reading it. Takes the treasury's
    /// moved parties (`Treasury::take_touched`), so one publisher, and only
    /// one, reads them.
    pub fn publish(&mut self, prev: Option<&Published>) -> Option<Published> {
        let mut cost = PublishCost::default();
        let head = self.chain.head_stamp();
        let mut moved = prev.is_none_or(|p| p.head != head);
        let lines = self.publish_lines(prev, &mut cost, &mut moved);
        let eras = self.publish_eras(prev, &mut cost, &mut moved);
        let proofs = match prev {
            Some(p) if p.proofs_mark == self.proofs_mark => Arc::clone(&p.proofs),
            _ => {
                moved = true;
                cost.keys(self.root_proofs.len() as u64);
                Arc::new(self.root_proofs.clone())
            }
        };
        let accounts = self.publish_accounts(prev, &mut cost, &mut moved);
        // 78.154 — the ledgers: shared where no entry sealed since.
        let ledgers = match prev {
            Some(p) if p.atlas_mark == self.atlas.mark => p.ledgers.clone(),
            _ => {
                moved = true;
                cost.keys(self.atlas.ledgers.len() as u64);
                self.atlas.ledgers.clone()
            }
        };
        // 78.153b — the journal: shared where no row moved since.
        let journal = match prev {
            Some(p) if p.journal_mark == self.treasury.journal_mark() => Arc::clone(&p.journal),
            _ => {
                moved = true;
                cost.keys(self.treasury.entries().len() as u64);
                Arc::new(self.treasury.entries().to_vec())
            }
        };
        let eras_path = match prev.and_then(|p| p.eras_path.clone()) {
            Some(held) if Some(&*held) == self.eras_path.as_deref() => Some(held),
            _ => {
                moved |= prev.is_some_and(|p| p.eras_path.as_deref() != self.eras_path.as_deref());
                self.eras_path.as_deref().map(Arc::from)
            }
        };
        let own = self.publish_own(prev, head, &mut cost, &mut moved);
        if !moved {
            return None;
        }
        Some(Published {
            head,
            court: self.chain.vk(),
            identity: self.identity,
            clock: self.core_stamp(),
            lines,
            eras,
            eras_path,
            proofs,
            proofs_mark: self.proofs_mark,
            accounts,
            ledgers,
            atlas_mark: self.atlas.mark,
            journal,
            journal_mark: self.treasury.journal_mark(),
            cost,
            own,
        })
    }

    /// 78.153b — THE COURT'S OWN READS: its cover charts where
    /// `own_cover_gen` moved, the ruled cover where `ruled_gen` moved, the
    /// head receipt where the head moved; its room, bond and voice at every
    /// publication, each compared with the last so a moved one publishes.
    fn publish_own(&mut self, prev: Option<&Published>, head: Option<Stamp>, cost: &mut PublishCost, moved: &mut bool) -> Own {
        let held = prev.map(|p| &p.own);
        let cover = match held {
            Some(o) if o.cover_mark == self.own_cover_gen => Arc::clone(&o.cover),
            _ => {
                *moved = true;
                let charts = self.own_cover_charts();
                cost.keys(charts.len() as u64);
                Arc::new(charts)
            }
        };
        let ruled = match held {
            Some(o) if o.ruled_mark == self.ruled_gen => Arc::clone(&o.ruled),
            _ => {
                *moved = true;
                let (root, digests, seed) = self.ruled_reading();
                cost.keys(seed.len() as u64);
                Arc::new(Ruled { root, digests, seed })
            }
        };
        let head_receipt = match held {
            Some(o) if prev.is_some_and(|p| p.head == head) => o.head_receipt.clone(),
            _ => self.chain.head(),
        };
        let (room, bonded, voice) = (self.lane_room(), self.treasury.bond_of(&self.identity).is_some(), self.court_voice);
        *moved |= held.is_none_or(|o| o.room != room || o.bonded != bonded || o.voice != voice);
        Own { cover, cover_mark: self.own_cover_gen, room, bonded, voice, ruled, ruled_mark: self.ruled_gen, head_receipt }
    }

    /// Every signer's line: its own and every peer's held here, and every
    /// signer whose fold root alone is held.
    fn publish_lines(&self, prev: Option<&Published>, cost: &mut PublishCost, moved: &mut bool) -> Keyed<Arc<Line>> {
        let mut signers: Keyed<()> = Keyed::new();
        for vk in self.chain.signers().into_iter().chain(self.chain.rooted_signers()) {
            signers.insert(vk, ());
        }
        let mut lines: Keyed<Arc<Line>> = Keyed::new();
        for vk in signers.scoped().filter_map(|(k, _)| <[u8; 32]>::try_from(k.as_slice()).ok()) {
            let tally = self.chain.line_tally(&vk);
            let standing = LineHolder::standing(self, &vk);
            let line = match prev.and_then(|p| p.lines.get(&vk)) {
                Some(held) if held.tally == tally && held.standing == standing => Arc::clone(held),
                Some(held) if held.tally == tally => {
                    *moved = true;
                    Arc::new(Line { standing, ..Line::clone(held) })
                }
                Some(held) => {
                    *moved = true;
                    let end = held.run.end().map(|k| k.r.stamp.place());
                    let grew = self.chain.line_between(&vk, end, END, usize::MAX);
                    let only_grew = held.tally.left == tally.left
                        && tally.filed.checked_sub(held.tally.filed) == Some(grew.len() as u64);
                    match only_grew {
                        true => {
                            let mut run = held.run.clone();
                            cost.receipts(run.extend(grew.into_iter().map(kept).collect(), |k| k.tape));
                            Arc::new(Line { tally, standing, run })
                        }
                        false => Arc::new(self.read_line(&vk, tally, standing, cost)),
                    }
                }
                None => {
                    *moved = true;
                    Arc::new(self.read_line(&vk, tally, standing, cost))
                }
            };
            cost.keys(1);
            lines.insert(vk, line);
        }
        *moved |= prev.is_some_and(|p| p.lines.len() != lines.len());
        lines
    }

    /// One signer's hot line read whole.
    fn read_line(&self, vk: &[u8; 32], tally: LineTally, standing: Option<Place>, cost: &mut PublishCost) -> Line {
        let mut run = Runs::default();
        cost.receipts(run.extend(self.chain.line_between(vk, None, END, usize::MAX).into_iter().map(kept).collect(), |k| k.tape));
        Line { tally, standing, run }
    }

    /// Every signer's eras: a signer whose tally did not move is shared; one
    /// whose new eras all stand past its last is extended by them; the rest
    /// are read again in one walk of the archive.
    fn publish_eras(&self, prev: Option<&Published>, cost: &mut PublishCost, moved: &mut bool) -> Keyed<Arc<EraRun>> {
        let mut eras: Keyed<Arc<EraRun>> = Keyed::new();
        let mut reread: Keyed<LineTally> = Keyed::new();
        for (k, tally) in self.era_tallies.scoped() {
            let Ok(vk) = <[u8; 32]>::try_from(k.as_slice()) else { continue };
            let tally = *tally;
            let run = match prev.and_then(|p| p.eras.get(&vk)) {
                Some(held) if held.tally == tally => Arc::clone(held),
                Some(held) => {
                    *moved = true;
                    let end = held.run.end().map(|e| e.root);
                    let from = self.archive.partition_point(|e| Some(e.root.at) <= end);
                    let grew: Vec<EraRef> =
                        self.archive.get(from..).into_iter().flatten().filter(|e| e.signer == vk).map(era_ref).collect();
                    match tally.filed.checked_sub(held.tally.filed) == Some(grew.len() as u64) {
                        true => {
                            let mut run = held.run.clone();
                            cost.eras(run.extend(grew, |e| e.tape));
                            Arc::new(EraRun { tally, run })
                        }
                        false => {
                            reread.insert(vk, tally);
                            continue;
                        }
                    }
                }
                None => {
                    *moved = true;
                    reread.insert(vk, tally);
                    continue;
                }
            };
            cost.keys(1);
            eras.insert(vk, run);
        }
        if !reread.is_empty() {
            let mut whole: Keyed<Vec<EraRef>> = Keyed::new();
            for e in self.archive.iter().filter(|e| reread.contains(&e.signer)) {
                match whole.get_mut(&e.signer) {
                    Some(v) => v.push(era_ref(e)),
                    None => {
                        whole.insert(e.signer, vec![era_ref(e)]);
                    }
                }
            }
            for (k, tally) in reread.scoped() {
                let Ok(vk) = <[u8; 32]>::try_from(k.as_slice()) else { continue };
                let mut run = Runs::default();
                if let Some(v) = whole.remove(&vk) {
                    cost.eras(run.extend(v, |e| e.tape));
                }
                cost.keys(1);
                eras.insert(vk, Arc::new(EraRun { tally: *tally, run }));
            }
        }
        *moved |= prev.is_some_and(|p| p.eras.len() != eras.len());
        eras
    }

    /// Every account, by shard: the shards holding a moved party are copied
    /// with its account read again; every other shard is shared.
    fn publish_accounts(&mut self, prev: Option<&Published>, cost: &mut PublishCost, moved: &mut bool) -> Vec<Arc<Keyed<Holdings>>> {
        let touched = self.treasury.take_touched();
        let (mut shards, parties) = match (prev, touched) {
            (Some(p), Touched::Parties(t)) => {
                if t.is_empty() {
                    return p.accounts.clone();
                }
                (p.accounts.iter().map(|s| Shard::Shared(Arc::clone(s))).collect::<Vec<_>>(), t)
            }
            _ => ((0..ACCOUNT_SHARDS).map(|_| Shard::Own(Keyed::new())).collect(), self.treasury.parties()),
        };
        *moved = true;
        for (who, holdings) in self.treasury.accounts_of(&parties) {
            let Some(slot) = shards.get_mut(shard_of(&who)) else { continue };
            if let Shard::Shared(s) = slot {
                cost.accounts(s.len() as u64);
                *slot = Shard::Own(Keyed::clone(s));
            }
            if let Shard::Own(s) = slot {
                cost.accounts(1);
                s.insert(who, holdings);
            }
        }
        shards
            .into_iter()
            .map(|s| match s {
                Shard::Shared(s) => s,
                Shard::Own(s) => Arc::new(s),
            })
            .collect()
    }
}

/// One shard of the accounts while a publication is built: shared with the
/// last, or copied and moved here.
enum Shard {
    Shared(Arc<Keyed<Holdings>>),
    Own(Keyed<Holdings>),
}
