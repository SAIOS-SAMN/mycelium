// 45.8, 47.3 — THE TWO-ORGAN FOLD: the home proposes, the witness fetches
// the prefix and re-derives it, and the home seals only on both signatures.
//
//! A proposal is the finality reading and the nets over the prefix, held
//! until a witness co-signs the root; the commit archives the prefix, seals
//! the `Fold` receipt citing the root, writes the epoch rows anchored to
//! it, and files the era with the witness and both signatures. The witness
//! re-derives the receipts as a chain ending at the root and the rows'
//! nets as the epochs proposed, co-signs, and holds the same era under the
//! home's key, so every archived prefix has two holders.

use crate::chronometry::{ActKind, Cite, Era, KStepReceipt, Place, Root, Stamp};
use crate::cognition::Kernel;
use crate::economy::{nets_of, Entry};
use crate::verdict::{
    REFUSE_FOLD_IDLE, REFUSE_FOLD_INEXACT, REFUSE_FOLD_NO_TRANSITION, REFUSE_FOLD_OPEN, REFUSE_RECEIPT, REFUSE_WITNESS_DEPENDENT,
    REFUSE_WITNESS_ROOM,
};

/// What a fold needs a witness for: the place the prefix stands on (the
/// previous fold's root, `None` from genesis), its root, and the epoch
/// rows the home will write. The prefix is the line strictly after the
/// one up to the other. Held as `fold_pending` until a witness co-signs
/// the root or the reading changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    pub after: Option<Place>,
    pub root: Root,
    pub epochs: Vec<([u8; 32], [u64; 4])>,
}

/// What a fold did, read for the line 34.2 writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoldDone {
    /// The fold receipt's place (none where it could not be sealed) and
    /// height.
    pub at: Option<Place>,
    pub h: u64,
    /// The archived prefix, first place to last.
    pub first: Place,
    pub last: Place,
    /// Identities the epoch rows named: a count of the mesh's, `u64`.
    pub identities: u64,
    pub witness: [u8; 32],
}

impl Kernel {
    /// PROPOSE (45.8, 47.3): the finality reading and the nets over the
    /// prefix, without writing; the root is the hot receipt at the prefix
    /// head. A prefix that is not final proposes nothing
    /// (`REFUSE_FOLD_OPEN`); a ledger that does not conserve over it
    /// proposes nothing (`REFUSE_FOLD_INEXACT`).
    pub fn propose_fold(&mut self) -> Result<Proposal, &'static str> {
        let head = self.fold_head().ok_or(REFUSE_FOLD_OPEN)?;
        self.propose_fold_at(head)
    }

    /// The proposal at `head`, read as [`Kernel::propose_fold`] reads it.
    fn propose_fold_at(&mut self, head: Place) -> Result<Proposal, &'static str> {
        // 78.153b — the rows below the head read ONCE: the audit and the nets
        // `fold_reading` asks, the epochs, and the diagnostic below each read
        // them again, three clones of the journal's prefix under the kernel
        // at every close (the lock audit, row 8).
        let below = self.treasury.rows_below(head);
        self.treasury.audit().map_err(|_| REFUSE_FOLD_INEXACT)?;
        let epochs = nets_of(&below)?.rows;
        let after = self.chain.root().map(|r| r.at);
        // The line relation names the receipt at a place (78.24 P1).
        let at = self.chain.at_place(head).ok_or(REFUSE_FOLD_OPEN)?;
        let root = Root { at: at.stamp.place(), hash: at.chain_hash, height: at.stamp.h };
        // PHASE 1 DIAGNOSTIC — the rows a witness will fetch (`rows_range`:
        // the span from `after`) against the rows these nets are over: every
        // court refused its peers' folds as not conserving on epoch 115.
        {
            let span = self.treasury.rows_span(after, root.at);
            if nets_of(&span).map(|n| n.rows) != Ok(epochs.clone()) {
                static SEEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
                let n = SEEN.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                if n.is_power_of_two() {
                    let only_below: Vec<String> = below
                        .iter()
                        .filter(|e| !span.contains(e))
                        .map(|e| format!("{:?}@{:?}", e.kind, e.stands_at()))
                        .take(6)
                        .collect();
                    let only_span = span.iter().filter(|e| !below.contains(e)).count();
                    eprintln!(
                        "  fold nets differ ({n}): below {} rows, witness span {} rows; only below {only_below:?}; only in span {only_span}; after {after:?} head {head:?} root {:?}",
                        below.len(),
                        span.len(),
                        root.at
                    );
                }
            }
        }
        let p = Proposal { after, root, epochs };
        self.fold_pending = Some(p.clone());
        Ok(p)
    }

    /// THE FOLD ASKED AT A CLOSE (78.67): proposed only where the prefix up
    /// to the fold's head moves work and outweighs its era's framing
    /// (`fold_amortizes`). `propose_fold` answers whether the line CAN fold
    /// — final and conserving; this answers whether it SHOULD.
    ///
    /// 78.166 2a — at the head that carries work (`fold_head_carrying_work`):
    /// a cut at a fold's own close alone falls past the page to the first
    /// close beyond it, where the orbit after a fold is longer than a page.
    pub fn propose_fold_at_close(&mut self) -> Result<Proposal, &'static str> {
        let head = self.fold_head_carrying_work().ok_or(REFUSE_FOLD_OPEN)?;
        self.fold_transitions(head)?;
        self.propose_fold_at(head)
    }

    /// 78.67 (A), 78.68 — A FOLD MOVES WORK, AT A PHASE TRANSITION.
    ///
    /// (A) The prefix up to `head` must hold an act other than a fold or an
    /// orbit's close. A fold's own receipt is an act, so it opens an orbit
    /// whose close proposed the next fold: the courts of epoch 117 folded at
    /// nearly every close, 337 815 eras in two hours on hypha a, each one
    /// moving the fold before it (`REFUSE_FOLD_IDLE`).
    ///
    /// (B) The algorithm must have changed in it (78.68, 78.69): a
    /// `Return` (a ring completed an age, `m ≡ 0 mod ℓ`), a `RankStep` (a
    /// chart's holonomy algebra gained a dimension), a `Reconnect` (the
    /// field's links moved, 78.71), or a generator entering
    /// the presentation — `Install`, `Ruling(Upheld)`, `Adopted`. The ledger
    /// holds the algorithm and never the sentences composed from it (78.69),
    /// so these are the acts that change what it holds. Between them the
    /// history is coefficient motion, and folding it would cut nothing the
    /// physics names (`REFUSE_FOLD_NO_TRANSITION`). This replaces 78.67's
    /// byte break-even, which measured the container and not the physics.
    ///
    /// THE QUESTION IS ASKED OF THE WHOLE HOT LINE, THE CUT STAYS ONE PAGE.
    /// A cut must fit one witness fetch (`fold_head`), and (B) was asked of
    /// that page alone. A court whose next page held no transition was
    /// refused at every close, its root never moved, and the page after was
    /// never asked: hypha b refused 2 516 proposals and held 101 721 own
    /// nodes hot with its fold head at K 468 of 11 554 — the time axis
    /// collapsed onto the first page. The algorithm changed since the root
    /// wherever this court's last transition stands past it
    /// (`last_transition`, kept as acts are sealed), and then folding the
    /// next page moves history the change already supersedes.
    fn fold_transitions(&mut self, head: Place) -> Result<(), &'static str> {
        let me = self.chain.vk();
        let root = self.chain.root().map(|r| r.at);
        let prefix = self.chain.line_between(&me, root, head, usize::MAX);
        if !prefix.iter().any(|r| !matches!(r.kind, ActKind::Fold | ActKind::Orbit)) {
            return Err(REFUSE_FOLD_IDLE);
        }
        let last = match self.last_transition {
            Some(at) => at,
            // Read once after a restore, off the hot line itself.
            None => {
                let at = self.chain.head_place().and_then(|h| {
                    self.chain
                        .line_between(&me, root, h, usize::MAX)
                        .iter()
                        .rev()
                        .find(|r| r.kind.changes_algorithm())
                        .map(|r| r.stamp.place())
                });
                self.last_transition = Some(at);
                at
            }
        };
        match (last, root) {
            (Some(t), Some(r)) if t > r => Ok(()),
            (Some(_), None) => Ok(()),
            _ => Err(REFUSE_FOLD_NO_TRANSITION),
        }
    }

    /// The proposal awaiting a witness, if any.
    /// THE PENDING PROPOSAL STILL STANDS: its nets are still the nets of the
    /// rows below its root, and its root is still final. A proposal was
    /// held and carried to witnesses cycle after cycle after rows were
    /// written below its root (an escrow's release stands at the escrow's
    /// place), so every witness re-derived other nets and refused it — the
    /// "does not conserve" refusals of epoch 115, thousands an hour. A
    /// stale proposal is dropped; the next stroke proposes afresh.
    pub fn pending_fold_stands(&mut self) -> bool {
        let Some(p) = self.fold_pending.as_ref() else { return false };
        let nets = nets_of(&self.treasury.rows_below(p.root.at)).map(|n| n.rows);
        let stands = nets.as_ref() == Ok(&p.epochs) && self.final_prefix_head().is_some_and(|h| h >= p.root.at);
        if !stands {
            self.fold_pending = None;
        }
        stands
    }

    /// A witness refused the pending proposal as not conserving: it is
    /// dropped, never carried again as it stood.
    pub fn drop_fold_proposal(&mut self) {
        self.fold_pending = None;
    }

    pub fn fold_proposal(&self) -> Option<&Proposal> {
        self.fold_pending.as_ref()
    }

    /// COMMIT ON TWO SIGNATURES (47.3): the witness is another key than
    /// this court's, its signed position over the root verifies under its
    /// key (47.2 step 2) with the root hash as payload, the proposal still
    /// reads exact and its prefix still final — then the prefix leaves the
    /// hot chain, the `Fold` receipt is sealed citing the root, the epoch
    /// rows are written anchored to it, and the era is filed with the
    /// witness and both signatures. `court fold …` and `court fold
    /// witnessed …` (34.2).
    pub fn commit_fold(
        &mut self,
        sig_home: [u8; 64],
        witness: &crate::anastome::dialogue::ReceiptSection,
    ) -> Result<FoldDone, &'static str> {
        let p = self.fold_pending.clone().ok_or(REFUSE_FOLD_OPEN)?;
        let me = self.identity;
        let wk = witness.receipt.stamp.vk;
        if wk == me {
            return Err(REFUSE_WITNESS_DEPENDENT);
        }
        crate::court::admission::verify_signature(&witness.prefix, &wk, &witness.sig)?;
        if witness.prefix.payload != p.root.hash {
            return Err(REFUSE_RECEIPT);
        }
        let epochs = nets_of(&self.treasury.rows_below(p.root.at))?.rows;
        if epochs != p.epochs {
            self.fold_pending = None;
            return Err(REFUSE_FOLD_INEXACT);
        }
        if self.final_prefix_head().is_none_or(|h| h < p.root.at) {
            self.fold_pending = None;
            return Err(REFUSE_FOLD_OPEN);
        }
        let (cold, root) = self.chain.fold_before(p.root.at).ok_or(REFUSE_FOLD_OPEN)?;
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — FIFO drain head: the drained prefix is in chain order, so its head is the oldest receipt folded"
        )]
        let first = cold.first().map_or(root.at, |r| r.stamp.place());
        // The Fold node cites the root it leaves the line on, by stamp and
        // hash (78.24): a fold names no word and measured nothing.
        let at = Stamp::new(root.height, me, root.at);
        self.append_cited(ActKind::Fold, vec![Cite::Root { at, hash: root.hash }], [0; 4], me);
        // THE EPOCH ROWS STAND AT THE ROOT they net to (78.65): the balance
        // the folded prefix leaves is the root's, so the journal stays in
        // line order and the next fold's nets below its cut count them. At
        // the fold node, which stands at the line's head, they stood above
        // rows they summarized.
        let folded = match self.treasury.fold(p.root.at, (at, root.hash)) {
            Ok(f) => f,
            Err(reason) => {
                self.fold_pending = None;
                return Err(reason);
            }
        };
        let identities = folded.rows as u64;
        // THE FOLD SHEDS. `archive.push` kept every folded receipt on the
        // heap and the snapshot then wrote all of them on every persist;
        // `file_era` appends the payload to the cold segment once and
        // keeps the descriptor.
        // `first` above is this era's own first place, read once where the
        // fold cut the chain; it is not read a second time here (law 12).
        let count = cold.len() as u64;
        self.file_era(Era {
            root,
            signer: me,
            witness: wk,
            sig_home,
            sig_witness: witness.sig,
            identities,
            first,
            count,
            at: None,
            receipts: cold,
        });
        // What this court's own folded line cited of its peers is no longer
        // cited by its hot line, and their folded lines may leave.
        self.shed_folded_lines();
        let done = FoldDone {
            at: self.chain.head_place(),
            h: self.chain.head_height(),
            first,
            last: root.at,
            identities,
            witness: wk,
        };
        self.fold_pending = None;
        self.mark_persist();
        Ok(done)
    }

    /// THE WITNESS'S RE-DERIVATION (47.3): this court holds bond
    /// (`REFUSE_WITNESS_ROOM` otherwise); the receipts fetched are the
    /// home's chain from the place after `after`, each re-deriving and
    /// linking to the one before, ending at the root; the rows' nets are
    /// the epochs proposed.
    pub fn verify_fold(
        &self,
        home: [u8; 32],
        p: &Proposal,
        receipts: &[KStepReceipt],
        rows: &[Entry],
    ) -> Result<(), &'static str> {
        verify_prefix(self.treasury.bond_of(&self.identity).is_some(), home, p, receipts, rows)
    }

    /// The witness files the era it co-signed under the home's key, so
    /// the archived prefix has two holders (45.8).
    /// A WITNESS SHEDS TOO. This pushed the whole era, receipts and all,
    /// onto the heap and into every later snapshot — the same rename the
    /// home's own fold was doing, and a witness holds one era per fold of
    /// every court it witnesses for.
    pub fn hold_era(&mut self, era: Era) {
        self.file_era(era);
        self.mark_persist();
    }

    /// THE ROWS READ (`ACT_ROWS`): this court's own ledger rows standing at
    /// `from` or later and written before the act at `upto` (see
    /// `Treasury::rows_span`), in journal order, a page of at most
    /// `budget` from `cursor`, with the total the span holds. Another
    /// signer's rows are not this court's to serve.
    pub fn rows_range(
        &self,
        signer: &[u8; 32],
        from: Option<Place>,
        upto: Place,
        cursor: u64,
        budget: usize,
    ) -> (u64, Vec<Entry>) {
        rows_range_of(&self.identity, self.treasury.entries(), signer, from, upto, cursor, budget)
    }
}

/// THE ROWS READ over a journal held by the court `identity` — the one
/// reading the kernel (`Kernel::rows_range`) and the published reading
/// (`Published::rows_range`, which the door answers `ACT_ROWS` with, off
/// the kernel) share.
pub(crate) fn rows_range_of(
    identity: &[u8; 32],
    entries: &[Entry],
    signer: &[u8; 32],
    from: Option<Place>,
    upto: Place,
    cursor: u64,
    budget: usize,
) -> (u64, Vec<Entry>) {
    if signer != identity || from.is_some_and(|f| f >= upto) {
        return (0, Vec::new());
    }
    // THE ROWS THE FOLD NETS OVER, and nothing else: the proposer's
    // epochs are the nets of every row below its root — the last fold's
    // epoch rows and all since — so the witness is served exactly that
    // set. The span from `from` left the epoch rows out, and a fold
    // proposed after another was refused by every witness as not
    // conserving: "rows fetched 0 … parties ours 0 proposed 8" (hypha b,
    // epoch 115). A fold compacts the journal, so the set stays small.
    // 78.153b — the page alone cloned (`rows_below_page_of`), never every
    // row below the head for every page of them. The span's rows
    // are the mesh's count, `u64` on the wire. A cursor past every `usize`
    // is past every row this node holds: the page is empty, and the total
    // says so.
    let Ok(skip) = usize::try_from(cursor) else {
        return (crate::economy::fold::rows_below_page_of(entries, upto, 0, 0).0, Vec::new());
    };
    crate::economy::fold::rows_below_page_of(entries, upto, skip, budget)
}

/// THE WITNESS'S RE-DERIVATION, read off what the home sent and whether
/// this court holds bond (`REFUSE_WITNESS_ROOM` otherwise) — the one reader
/// the kernel (`Kernel::verify_fold`) and the published reading (78.153b,
/// `Published::verify_fold`) share: the receipts fetched are the home's
/// chain from the place after `after`, each re-deriving and linking to the
/// one before, ending at the root; the rows' nets are the epochs proposed.
pub(crate) fn verify_prefix(bonded: bool, home: [u8; 32], p: &Proposal, receipts: &[KStepReceipt], rows: &[Entry]) -> Result<(), &'static str> {
    if !bonded {
        return Err(REFUSE_WITNESS_ROOM);
    }
    let mut prev: Option<&KStepReceipt> = None;
    for r in receipts {
        if r.stamp.vk != home || !r.rederives() {
            return Err(REFUSE_RECEIPT);
        }
        // The first names the place the prefix stands on as its
        // predecessor; each after it names the one before.
        let links = match prev {
            None => r.prev.map(|q| q.at.place()) == p.after,
            Some(q) => r.prev == Some(crate::chronometry::Parent { at: q.stamp, hash: q.chain_hash }),
        };
        if !links {
            return Err(REFUSE_RECEIPT);
        }
        prev = Some(r);
    }
    let last = prev.ok_or(REFUSE_FOLD_OPEN)?;
    if (last.stamp.place(), last.chain_hash, last.stamp.h) != (p.root.at, p.root.hash, p.root.height) {
        return Err(REFUSE_RECEIPT);
    }
    if nets_of(rows)?.rows != p.epochs {
        return Err(REFUSE_FOLD_INEXACT);
    }
    Ok(())
}
