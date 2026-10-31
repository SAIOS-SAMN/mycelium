// 78.151 — THE READS A PUBLISHED READING ANSWERS: a page of a signer's line
// or its fold root, a party's account, the core clock — each at the height
// the reading was published at, and never with the kernel.

use super::publish::Published;
use crate::anastome::dialogue::wallet::Account;
use crate::anastome::dialogue::{ReceiptsAnswer, RootProof};
use crate::chronometry::{CoreStamp, KStepReceipt, Place, Stamp};
use crate::cognition::court::era::LineHolder;

impl LineHolder for Published {
    fn signs_as(&self) -> [u8; 32] {
        self.identity
    }

    fn proof_of(&self, signer: &[u8; 32]) -> Option<&RootProof> {
        self.proofs.get(signer)
    }

    fn standing(&self, signer: &[u8; 32]) -> Option<Place> {
        self.lines.get(signer).and_then(|l| l.standing)
    }

    fn page_eras(&self, signer: &[u8; 32], after: Option<Place>, upto: Place, each: &mut dyn FnMut(&[KStepReceipt]) -> bool) {
        let Some(eras) = self.eras.get(signer) else { return };
        for e in eras.run.past(|e| e.root, after) {
            if e.first > upto {
                return;
            }
            let page = match (&e.payload, e.filed, &self.eras_path) {
                (Some(p), _, _) => each(p),
                (None, Some(at), Some(path)) => match crate::cognition::state::read_era(path, at) {
                    Some(p) => each(&p),
                    None => true,
                },
                (None, _, _) => true,
            };
            if !page {
                return;
            }
        }
    }

    fn hot(&self, signer: &[u8; 32], after: Option<Place>, upto: Place, n: usize) -> Vec<KStepReceipt> {
        let Some(line) = self.lines.get(signer) else { return Vec::new() };
        line.run.past(|k| k.r.stamp.place(), after).take_while(|k| k.r.stamp.place() <= upto).take(n).map(|k| k.r.clone()).collect()
    }
}

impl Published {
    /// THE HEIGHT this reading was published at: its court's head's, zero
    /// before the court's first act. Every answer read off it carries it.
    pub fn height(&self) -> u64 {
        self.head.map_or(0, |s| s.h)
    }

    /// The court's head this reading stands at.
    pub fn head(&self) -> Option<Stamp> {
        self.head
    }

    /// What publishing this reading copied.
    pub fn cost(&self) -> super::PublishCost {
        self.cost
    }

    /// `ACT_RECEIPTS` answered off this reading, as the court answers it
    /// under its kernel (`Court::serve_receipts`): a range the reading does
    /// not yet reach is answered short, at its height.
    pub fn serve_receipts(&self, signer: &[u8; 32], after: Option<Place>, upto: Place, replay: bool, budget: usize, bytes: usize) -> ReceiptsAnswer {
        crate::cognition::court::catchup::serve_receipts(self, signer, after, upto, replay, budget, bytes)
    }

    /// `ACT_WALLET` answered off this reading, as `Court::wallet` answers
    /// it, standing at this reading's head: a party it holds no account of
    /// holds zero on every lane (the reading of absence); `None` where the
    /// party's holds pass the holds' carrier.
    pub fn wallet(&self, who: &[u8; 32]) -> Option<Account> {
        let at = self.head;
        let court = self.court;
        let shard = self.accounts.get(super::publish::shard_of(who))?;
        match shard.get(who) {
            Some(Some((balance, bond, held))) => Some(Account { court, balance: *balance, bond: *bond, held: *held, at }),
            Some(None) => None,
            None => Some(Account { court, balance: [0; 4], bond: [0; 4], held: [0; 4], at }),
        }
    }

    /// 78.154 — READ-LEDGER answered off this reading, as
    /// `Court::ledger_page` answers it: the entries after `u`, as many as
    /// fit one frame.
    pub fn ledger_page(&self, domain: &[u8; 32], after: u64) -> crate::anastome::dialogue::atlas::LedgerPage {
        crate::cognition::atlas::Atlas::page(self.ledgers.get(domain), after, crate::cognition::atlas::ledger_budget())
    }

    /// 78.153b — `ACT_ROWS` answered off this reading, as `Court::rows_range`
    /// answers it: this court's own journal entries below `upto`, a page
    /// from `cursor`, with the count standing there.
    pub fn rows_range(&self, signer: &[u8; 32], from: Option<Place>, upto: Place, cursor: u64, budget: usize) -> (u64, Vec<crate::economy::Entry>) {
        crate::cognition::court::fold::rows_range_of(&self.identity, &self.journal, signer, from, upto, cursor, budget)
    }

    /// The core clock at this reading's head (`Court::core_stamp`).
    pub fn clock(&self) -> Option<&CoreStamp> {
        self.clock.as_ref()
    }

    /// 78.153b — `Court::own_cover_charts` as published.
    pub fn own_cover_charts(&self) -> &[(u8, crate::transducer::stalk::Invariant)] {
        &self.own.cover
    }

    /// 78.153b — `Court::lane_room` as published.
    pub fn lane_room(&self) -> Option<[u64; 4]> {
        self.own.room
    }

    /// 78.153b — `Court::ruled_root` as published.
    pub fn ruled_root(&self) -> [u8; 32] {
        self.own.ruled.root
    }

    /// 78.153b — `Court::ruled_digests_for` as published: the ruled cover's
    /// digests to the court's own voice, none to any other asker.
    pub fn ruled_digests_for(&self, voice: &[u8; 32]) -> Option<&[crate::transducer::sheaf::StratumDigest]> {
        (self.own.voice.as_ref() == Some(voice)).then_some(self.own.ruled.digests.as_slice())
    }

    /// 78.153b — `Court::syllabus_seed` as published: the ruled addresses,
    /// coarsest chart first.
    pub fn syllabus_seed(&self) -> &[Vec<u8>] {
        &self.own.ruled.seed
    }

    /// 78.153b — the court's head receipt (`ReceiptChain::head`) as
    /// published: the receipt at [`Published::head`].
    pub fn head_receipt(&self) -> Option<&KStepReceipt> {
        self.own.head_receipt.as_ref()
    }

    /// 78.153b — `Court::verify_fold` answered off this reading: the bond it
    /// asks of the witness is the published one; the re-derivation reads
    /// only what the home sent.
    pub fn verify_fold(
        &self,
        home: [u8; 32],
        p: &crate::cognition::Proposal,
        receipts: &[KStepReceipt],
        rows: &[crate::economy::Entry],
    ) -> Result<(), &'static str> {
        crate::cognition::court::fold::verify_prefix(self.own.bonded, home, p, receipts, rows)
    }
}
