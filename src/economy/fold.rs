// 45.8, §2, §6 — THE FOLD: a finalised prefix of the ledger re-expressed as
// one `epoch` row per identity, conserving on every lane.
//
//! An `epoch` row is the net an identity's rows moved over the folded
//! prefix — what it was issued or received less what left it — and it
//! moves no supply: the rows it replaces already did. The audit holds
//! before and after or nothing is written (`REFUSE_FOLD_INEXACT`).

use super::*;
use crate::chronometry::Place;
use crate::network::nerve::Keyed;

/// What a fold did: the rows it replaced, the epoch rows it wrote, and
/// the net those rows carry, per lane.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Folded {
    pub replaced: usize,
    pub rows: usize,
    pub net: Mu,
}

/// THE ISSUANCE A FOLD CHECKPOINTS BESIDE ITS NETS (MU-ECONOMY §6, §6.1;
/// GLOSSARY, Era / fold): every row a fold replaces moved issuance or a
/// balance, and its `epoch` rows keep only the balances. What the replaced
/// rows issued and burned, by the proof each carried, is kept here, so a
/// fold removes nothing a reading reads: the earned supply apart from
/// genesis (§6), what each proof of work paid and burned (§0), issued and
/// burned each, never netted into one another.
///
/// Each source is a row kind and the arm byte the row carries (`Arm::byte`:
/// `0` for none, then POUW, POWC, POWR, POWV), summed gross over every
/// folded prefix; an `epoch` row is a net of balances and is never read
/// here, so a fold over an earlier fold's epoch rows counts that fold's
/// issuance once. Written only by [`Treasury::fold`] and read back with the
/// carriers (`persist.rs`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FoldedIssuance {
    /// Genesis allocations.
    pub genesis: Mu,
    /// Mint rows, by the arm byte each carried.
    pub minted: [Mu; 5],
    /// Transfer rows, by arm byte: both halves of each pair, as the rows
    /// stood (a reading takes one of each pair).
    pub transferred: [Mu; 5],
    /// Burn rows, by arm byte.
    pub burned: [Mu; 5],
    /// Slash rows: issuance leaving, read apart from a burn (§6).
    pub slashed: Mu,
}

impl FoldedIssuance {
    /// What `rows` issued and burned, by source; an `epoch` row and every
    /// kind that moves no supply and pays no proof read nothing.
    pub fn of_rows(rows: &[Entry]) -> FoldedIssuance {
        let mut f = FoldedIssuance::default();
        for e in rows {
            let slot = usize::from(Arm::byte(e.arm));
            let into = match e.kind {
                Kind::Genesis => &mut f.genesis,
                Kind::Mint => &mut f.minted[slot],
                Kind::Transfer => &mut f.transferred[slot],
                Kind::Burn => &mut f.burned[slot],
                Kind::Slash => &mut f.slashed,
                Kind::Epoch | Kind::Stake | Kind::Unstake | Kind::Escrow | Kind::Release | Kind::Revoke => continue,
            };
            into.step(&e.mu);
        }
        f
    }

    /// This checkpoint and `other`, source by source.
    pub fn add(&self, other: &FoldedIssuance) -> FoldedIssuance {
        let by = |a: &[Mu; 5], b: &[Mu; 5]| -> [Mu; 5] { std::array::from_fn(|i| a[i].add(&b[i])) };
        FoldedIssuance {
            genesis: self.genesis.add(&other.genesis),
            minted: by(&self.minted, &other.minted),
            transferred: by(&self.transferred, &other.transferred),
            burned: by(&self.burned, &other.burned),
            slashed: self.slashed.add(&other.slashed),
        }
    }

    /// What the folded prefixes issued: genesis and every mint.
    pub fn issued(&self) -> Mu {
        Mu::sum(std::iter::once(&self.genesis).chain(&self.minted))
    }

    /// What the folded prefixes took out of circulation: every burn and
    /// every slash.
    pub fn burned(&self) -> Mu {
        Mu::sum(self.burned.iter().chain(std::iter::once(&self.slashed)))
    }

    /// What the folded prefixes earned: the mint rows alone, every proof
    /// and the subsidy, never genesis.
    pub fn minted(&self) -> Mu {
        Mu::sum(self.minted.iter())
    }

    /// EVERY SOURCE HELD, by its coordinate `[kind tag, arm byte]` (the row
    /// tape's own, `Kind::tag` and `Arm::byte`), each where the trie it is
    /// written to keys it; a source that holds nothing is not written, and
    /// reads back as nothing.
    pub fn sources(&self) -> Vec<([u8; 2], &Mu)> {
        let mut out = vec![([Kind::Genesis.tag(), 0], &self.genesis)];
        for (kind, by) in [(Kind::Mint, &self.minted), (Kind::Transfer, &self.transferred), (Kind::Burn, &self.burned)] {
            for (slot, mu) in (0u8..).zip(by.iter()) {
                out.push(([kind.tag(), slot], mu));
            }
        }
        out.push(([Kind::Slash.tag(), 0], &self.slashed));
        out.retain(|(_, mu)| !mu.is_zero());
        out
    }

    /// The source at coordinate `key`, to be written; `None` where `key`
    /// names no source the checkpoint holds (a kind it keeps no sum of, an
    /// arm byte naming no arm, or an arm on genesis or a slash).
    pub fn source_mut(&mut self, key: [u8; 2]) -> Option<&mut Mu> {
        let [tag, arm] = key;
        let slot = usize::from(Arm::byte(Arm::of(arm)?));
        match (Kind::of_tag(tag)?, slot) {
            (Kind::Genesis, 0) => Some(&mut self.genesis),
            (Kind::Slash, 0) => Some(&mut self.slashed),
            (Kind::Mint, s) => self.minted.get_mut(s),
            (Kind::Transfer, s) => self.transferred.get_mut(s),
            (Kind::Burn, s) => self.burned.get_mut(s),
            _ => None,
        }
    }
}

/// The nets a fold writes: one (identity, net) per identity with a
/// nonzero net, in identity order, and the nets' total.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nets {
    pub rows: Vec<([u8; 32], Mu)>,
    pub total: Mu,
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
    nets_read(rows).map_err(|_| crate::verdict::REFUSE_FOLD_INEXACT)
}

/// WHY A FOLD'S NETS DO NOT READ EXACT, by what failed: a transfer half
/// with no partner, at its row; a party whose net stands below zero on a
/// lane; or a party whose key is not 32 bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inexact {
    UnpairedHalf { row: usize, kind: Kind, at: Option<Place> },
    BelowZero { who: [u8; 32], lane: usize },
    Malformed,
}

/// [`nets_of`], with the reason it refused. The nets are exact over ℚ.
pub fn nets_read(rows: &[Entry]) -> Result<Nets, Inexact> {
    use crate::core::q::Q;
    let mut net: Keyed<[Q; 4]> = Keyed::new();
    let mut half: Option<(usize, &Entry)> = None;
    for (row, e) in rows.iter().enumerate() {
        let credit = match e.kind {
            Kind::Genesis | Kind::Mint | Kind::Epoch => true,
            Kind::Burn | Kind::Slash => false,
            Kind::Stake | Kind::Unstake | Kind::Escrow | Kind::Release | Kind::Revoke => continue,
            Kind::Transfer => match half.take() {
                None => {
                    half = Some((row, e));
                    false
                }
                Some((first, debit)) => {
                    if (debit.at, debit.receipt, &debit.mu) != (e.at, e.receipt, &e.mu) {
                        return Err(Inexact::UnpairedHalf { row: first, kind: debit.kind, at: debit.stands_at() });
                    }
                    true
                }
            },
        };
        // A party with no net yet nets nothing: the reading of absence on
        // a keyed carrier (0.11c).
        let mut n = match net.get(&e.who) {
            Some(n) => n.clone(),
            None => Default::default(),
        };
        for (t, m) in n.iter_mut().zip(e.mu.lanes()) {
            match credit {
                true => *t += m,
                false => *t -= m,
            }
        }
        net.insert(e.who, n);
    }
    if let Some((row, debit)) = half {
        return Err(Inexact::UnpairedHalf { row, kind: debit.kind, at: debit.stands_at() });
    }
    let mut out = Vec::new();
    let mut total = Mu::zero();
    // The net this function just accumulated, one entry per party to the
    // fold: every party is paid or the fold is inexact and refuses whole.
    for (who, n) in net.scoped() {
        let who = <[u8; 32]>::try_from(who.as_slice()).map_err(|_| Inexact::Malformed)?;
        let mu = Mu::of(n.clone()).map_err(|r| match r {
            super::MuRefusal::Negative { lane } => Inexact::BelowZero { who, lane },
            _ => Inexact::Malformed,
        })?;
        if mu.is_zero() {
            continue;
        }
        total.step(&mu);
        out.push((who, mu));
    }
    Ok(Nets { rows: out, total })
}

/// ONE PAGE OF A JOURNAL'S ENTRIES BELOW `head`, and how many stand there —
/// the one reading `Treasury::rows_below_page` and the published reading
/// (`Published::rows_range`, 78.153b) both answer a witness with.
pub fn rows_below_page_of(entries: &[Entry], head: Place, skip: usize, n: usize) -> (u64, Vec<Entry>) {
    let below = || entries.iter().filter(|e| e.stands_at() < Some(head));
    let page = below().skip(skip).take(n).cloned().collect();
    (below().count() as u64, page)
}

impl Treasury {
    /// The rows written before the act at `head`, in journal order: the
    /// prefix a fold at `head` re-expresses and a witness fetches
    /// (`ACT_ENTRIES`).
    pub fn rows_below(&self, head: Place) -> Vec<Entry> {
        self.entries.iter().filter(|e| e.stands_at() < Some(head)).cloned().collect()
    }

    /// 78.153b — ONE PAGE OF THE ROWS BELOW `head`, and how many stand
    /// there: the rows [`Treasury::rows_below`] answers, from the `skip`th,
    /// at most `n` — only the page cloned, where `ACT_ENTRIES` cloned every row
    /// below the head for every page of them (the lock audit, row 17).
    ///
    /// THE JOURNAL IS NOT IN LINE ORDER: a release is written at its
    /// escrow's place, after rows standing later (`pending_fold_stands`), so
    /// the rows below a head need not be a prefix of it, and a bisection
    /// would read short where one stands out of line. The walk stays a
    /// count over the journal; the page alone is copied.
    #[cfg(test)]
    pub fn rows_below_page(&self, head: Place, skip: usize, n: usize) -> (u64, Vec<Entry>) {
        rows_below_page_of(&self.entries, head, skip, n)
    }

    /// The rows standing at `from` or later and written before the act at
    /// `upto`, in journal order: what `ACT_ENTRIES` serves a witness. `from`
    /// is the root the prefix stands on, `None` from the journal's start.
    ///
    /// 78.153b — A FILTER, NEVER A RUN. This bisected for `from` and broke at
    /// the first row standing at `upto` or later, which reads the span only
    /// where the journal is in line order; it is not (a release is written
    /// at its escrow's place, after rows standing later — `rows_below_page`),
    /// so a row out of line was dropped or a row past `upto` taken. Its one
    /// caller is `propose_fold`'s diagnostic, which compares the span's nets
    /// with the proposal's: read short, it named a difference that was not
    /// there. Every row is weighed now, exactly.
    pub fn rows_span(&self, from: Option<Place>, upto: Place) -> Vec<Entry> {
        self.entries.iter().filter(|e| e.stands_at() >= from && e.stands_at() < Some(upto)).cloned().collect()
    }

    /// `nets_of` over the rows written before the act at `head`, with the
    /// count of rows the fold replaces.
    fn nets(&self, head: Place) -> Result<(usize, Nets), &'static str> {
        let prefix = self.rows_below(head);
        Ok((prefix.len(), nets_of(&prefix)?))
    }

    /// FOLD the rows written before the act at `head` into `epoch` rows
    /// anchored to the fold receipt, the issuance those rows carried kept in
    /// the checkpoint beside them ([`FoldedIssuance`]). The audit failing
    /// before or after, a net below zero on a lane, or an unpaired transfer
    /// half is `REFUSE_FOLD_INEXACT` and the journal is exactly as it was.
    pub fn fold(&mut self, head: Place, anchor: Anchor) -> Result<Folded, &'static str> {
        let inexact = crate::verdict::REFUSE_FOLD_INEXACT;
        self.audit().map_err(|_| inexact)?;
        let (replaced, nets) = self.nets(head)?;
        let epochs: Vec<Entry> = nets
            .rows
            .iter()
            .map(|(who, mu)| Entry { kind: Kind::Epoch, who: *who, mu: mu.clone(), at: anchor.0, receipt: anchor.1, arm: None })
            .collect();
        let before = std::mem::take(&mut self.entries);
        let (issued, burned, folded) = (self.issued.clone(), self.burned.clone(), self.folded.clone());
        let prefix: Vec<Entry> = before.iter().filter(|e| e.stands_at() < Some(head)).cloned().collect();
        self.folded = folded.add(&FoldedIssuance::of_rows(&prefix));
        let rest = before.iter().filter(|e| e.stands_at() >= Some(head)).cloned();
        self.entries = epochs;
        self.entries.extend(rest);
        self.journal_moved();
        self.refold_supply();
        if self.audit().is_err() || self.issued != issued || self.burned != burned {
            self.entries = before;
            self.issued = issued;
            self.burned = burned;
            self.folded = folded;
            return Err(inexact);
        }
        Ok(Folded { replaced, rows: nets.rows.len(), net: nets.total })
    }
}
