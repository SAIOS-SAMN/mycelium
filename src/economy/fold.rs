// 45.8, §2, §6 — THE FOLD: a finalised prefix of the ledger re-expressed as
// one `epoch` row per identity, conserving on every lane.
//
//! An `epoch` row is the net an identity's rows moved over the folded
//! prefix — what it was issued or received less what left it — and it
//! moves no supply: the rows it replaces already did. The audit holds
//! before and after or nothing is written (`REFUSE_FOLD_INEXACT`).

use super::*;
use crate::network::nerve::{Keyed, ROOT};

/// What a fold did: the rows it replaced, the epoch rows it wrote, and
/// the net those rows carry, per lane.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Folded {
    pub replaced: usize,
    pub rows: usize,
    pub net: [u64; 4],
}

impl Treasury {
    /// FOLD the rows anchored at or below `k_head` into `epoch` rows
    /// anchored to the fold receipt. The net per identity is read off the
    /// rows themselves: genesis and mint credit, burn and slash debit, a
    /// transfer's two halves (one anchor, one vector, debit then credit)
    /// move between two identities, and a stake, an escrow or a discharge
    /// moves within one and nets nothing. A net below zero on any lane, an
    /// unpaired transfer, or an audit that fails before or after the fold
    /// is `REFUSE_FOLD_INEXACT` and writes nothing.
    pub fn fold(&mut self, k_head: u64, anchor: Anchor) -> Result<Folded, &'static str> {
        let inexact = crate::verdict::REFUSE_FOLD_INEXACT;
        self.audit().map_err(|_| inexact)?;
        let (prefix, rest): (Vec<Entry>, Vec<Entry>) =
            self.entries.iter().cloned().partition(|e| e.k_step <= k_head);
        let mut net: Keyed<[i128; 4]> = Keyed::new();
        let mut half: Option<Entry> = None;
        for e in &prefix {
            let sign: i128 = match e.kind {
                Kind::Genesis | Kind::Mint | Kind::Epoch => 1,
                Kind::Burn | Kind::Slash => -1,
                Kind::Stake | Kind::Unstake | Kind::Escrow | Kind::Release | Kind::Revoke => 0,
                Kind::Transfer => match half.take() {
                    None => {
                        half = Some(e.clone());
                        -1
                    }
                    Some(debit) => {
                        if (debit.k_step, debit.receipt, debit.mu) != (e.k_step, e.receipt, e.mu) {
                            return Err(inexact);
                        }
                        1
                    }
                },
            };
            if sign == 0 {
                continue;
            }
            let mut n = net.get(&e.who).copied().unwrap_or([0; 4]);
            for (t, &m) in n.iter_mut().zip(&e.mu) {
                *t += sign * i128::from(m);
            }
            net.insert(e.who, n);
        }
        if half.is_some() {
            return Err(inexact);
        }
        let mut epochs = Vec::new();
        let mut total = [0u64; 4];
        for (who, n) in net.fiber(ROOT) {
            let Ok(who) = <[u8; 32]>::try_from(who.as_slice()) else {
                return Err(inexact);
            };
            let mut mu = [0u64; 4];
            for (m, &v) in mu.iter_mut().zip(n) {
                *m = u64::try_from(v).map_err(|_| inexact)?;
            }
            if lanes_short(&mu).is_empty() {
                continue;
            }
            for (t, &m) in total.iter_mut().zip(&mu) {
                *t = t.checked_add(m).ok_or(inexact)?;
            }
            epochs.push(Entry { kind: Kind::Epoch, who, mu, k_step: anchor.0, receipt: anchor.1 });
        }
        let replaced = prefix.len();
        let rows = epochs.len();
        let before = std::mem::take(&mut self.entries);
        let (issued, burned) = (self.issued, self.burned);
        self.entries = epochs;
        self.entries.extend(rest);
        self.refold_supply();
        if self.audit().is_err() {
            self.entries = before;
            self.issued = issued;
            self.burned = burned;
            return Err(inexact);
        }
        Ok(Folded { replaced, rows, net: total })
    }
}
