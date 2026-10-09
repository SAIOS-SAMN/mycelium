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
                (None, Some(at), Some(path)) => match crate::cognition::kernel::read_era(path, at) {
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
    /// under its kernel (`Kernel::serve_receipts`): a range the reading does
    /// not yet reach is answered short, at its height.
    pub fn serve_receipts(&self, signer: &[u8; 32], after: Option<Place>, upto: Place, replay: bool, budget: usize, bytes: usize) -> ReceiptsAnswer {
        crate::cognition::court::catchup::serve_receipts(self, signer, after, upto, replay, budget, bytes)
    }

    /// `ACT_WALLET` answered off this reading, as `Kernel::wallet` answers
    /// it, standing at this reading's head: a party it holds no account of
    /// holds zero on every lane (the reading of absence); `None` where the
    /// party's holds pass their carrier.
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

    /// The core clock at this reading's head (`Kernel::core_stamp`).
    pub fn clock(&self) -> Option<&CoreStamp> {
        self.clock.as_ref()
    }
}
