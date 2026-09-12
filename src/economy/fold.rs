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

/// The nets a fold writes: one (identity, net) per identity with a
/// nonzero net, in identity order, and their total.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nets {
    pub rows: Vec<([u8; 32], [u64; 4])>,
    pub total: [u64; 4],
}

/// THE NET PER IDENTITY over `rows`, read off the rows themselves (§6
/// across a fold): genesis, mint and an earlier epoch credit, burn and
/// slash debit, a transfer's two halves (one anchor, one vector, debit
/// then credit) move between two identities, and a stake, an escrow or a
/// discharge moves within one and nets nothing. A net below zero on any
/// lane or an unpaired transfer half is `REFUSE_FOLD_INEXACT`. A witness
/// re-derives a fold with this same reading over the rows it fetched
/// (47.3).
pub fn nets_of(rows: &[Entry]) -> Result<Nets, &'static str> {
    let inexact = crate::verdict::REFUSE_FOLD_INEXACT;
    let mut net: Keyed<[i128; 4]> = Keyed::new();
    let mut half: Option<&Entry> = None;
    for e in rows {
        let sign: i128 = match e.kind {
            Kind::Genesis | Kind::Mint | Kind::Epoch => 1,
            Kind::Burn | Kind::Slash => -1,
            Kind::Stake | Kind::Unstake | Kind::Escrow | Kind::Release | Kind::Revoke => 0,
            Kind::Transfer => match half.take() {
                None => {
                    half = Some(e);
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
        // A party with no net yet nets nothing: the reading of absence on
        // a keyed carrier (0.11c).
        let mut n = match net.get(&e.who) {
            Some(n) => *n,
            None => [0; 4],
        };
        for (t, &m) in n.iter_mut().zip(&e.mu) {
            *t += sign * i128::from(m);
        }
        net.insert(e.who, n);
    }
    if half.is_some() {
        return Err(inexact);
    }
    let mut out = Vec::new();
    let mut total = [0u64; 4];
    for (who, n) in net.fiber(ROOT) {
        let who = <[u8; 32]>::try_from(who.as_slice()).map_err(|_| inexact)?;
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
        out.push((who, mu));
    }
    Ok(Nets { rows: out, total })
}

impl Treasury {
    /// The rows anchored at or below `k_head`, in journal order: the
    /// prefix a fold re-expresses and a witness fetches (`ACT_ROWS`).
    pub fn rows_below(&self, k_head: u64) -> Vec<Entry> {
        self.entries.iter().filter(|e| e.k_step <= k_head).cloned().collect()
    }

    /// The rows anchored in `from..=to`, in journal order: what `ACT_ROWS`
    /// serves a witness.
    /// THE JOURNAL IS IN K ORDER, so the span is a run of it and not a
    /// filter over the whole of it. This cloned every matching row out of
    /// an append-only ledger on every `ACT_ROWS` page — O(journal) for a
    /// page of a few rows, per witness, per fold (law 3: `k_step` is the
    /// coordinate that orders them).
    pub fn rows_span(&self, from: u64, to: u64) -> Vec<Entry> {
        let at = self.entries.partition_point(|e| e.k_step < from);
        let Some(run) = self.entries.get(at..) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for e in run {
            if e.k_step > to {
                break;
            }
            out.push(e.clone());
        }
        out
    }

    /// `nets_of` over the rows anchored at or below `k_head`, with the
    /// count of rows the fold replaces.
    fn nets(&self, k_head: u64) -> Result<(usize, Nets), &'static str> {
        let prefix = self.rows_below(k_head);
        Ok((prefix.len(), nets_of(&prefix)?))
    }

    /// What a fold at `k_head` would do, without writing it: the audit
    /// must hold now and the nets must read exact.
    pub fn fold_reading(&self, k_head: u64) -> Result<Folded, &'static str> {
        self.audit().map_err(|_| crate::verdict::REFUSE_FOLD_INEXACT)?;
        let (replaced, nets) = self.nets(k_head)?;
        Ok(Folded { replaced, rows: nets.rows.len(), net: nets.total })
    }

    /// FOLD the rows anchored at or below `k_head` into `epoch` rows
    /// anchored to the fold receipt. The audit failing before or after, a
    /// net below zero on a lane, or an unpaired transfer half is
    /// `REFUSE_FOLD_INEXACT` and the journal is exactly as it was.
    pub fn fold(&mut self, k_head: u64, anchor: Anchor) -> Result<Folded, &'static str> {
        let inexact = crate::verdict::REFUSE_FOLD_INEXACT;
        self.audit().map_err(|_| inexact)?;
        let (replaced, nets) = self.nets(k_head)?;
        let epochs: Vec<Entry> = nets
            .rows
            .iter()
            .map(|&(who, mu)| Entry { kind: Kind::Epoch, who, mu, k_step: anchor.0, receipt: anchor.1 })
            .collect();
        let before = std::mem::take(&mut self.entries);
        let (issued, burned) = (self.issued, self.burned);
        let rest = before.iter().filter(|e| e.k_step > k_head).cloned();
        self.entries = epochs;
        self.entries.extend(rest);
        self.refold_supply();
        if self.audit().is_err() {
            self.entries = before;
            self.issued = issued;
            self.burned = burned;
            return Err(inexact);
        }
        Ok(Folded { replaced, rows: nets.rows.len(), net: nets.total })
    }
}
