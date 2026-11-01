// E3 — THE SYNTHETIC ECONOMY: a payment between two identities, and value a
// payer holds against a key until it is released to a payee or refunded.
//
//! It stands in for the contracts until they are deployed: the court is the
//! ledger, every movement is a row at the act it stands at, and nothing here
//! issues or burns. A payment moves μ⃗ between two balances; a hold moves it
//! from the payer's balance into the HELD carrier, still the payer's until a
//! release pays it on or a refund returns it. §6's second equation counts
//! the held carrier beside the balances and the bonds (`audit`), so both
//! equations hold through every movement here. Every amount is a per-lane
//! exact [`Mu`]; a movement is whole or refused, named, and a refused one
//! writes no row.

use super::*;

/// 78.154a — THE RECYCLED POOL'S KEY: the connection tax a court received,
/// held by the court, paying its closers' POUW before anything is minted.
pub const RECYCLED: &[u8] = b"recycled pool";

/// VALUE A PAYER SET ASIDE AGAINST A KEY: whose it is, how much per lane,
/// and the act it was set aside at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Held {
    pub payer: [u8; 32],
    pub mu: Mu,
    pub anchor: Anchor,
}

/// WHY A MOVEMENT DID NOT HAPPEN, by name — nothing moved and no row was
/// written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unpaid {
    /// The amount is zero on every lane: nothing to move.
    Nothing,
    /// Payer and payee are one identity: a payment to oneself moves nothing.
    Oneself,
    /// The payer's balance lacks this, per lane.
    Short(Mu),
    /// A hold already stands under this key.
    Taken,
    /// No hold stands under this key.
    Unheld,
    /// 78.133 — parts that do not sum to the hold exactly.
    Split,
}

impl Treasury {
    /// PAY: `mu` from `from`'s balance to `to`'s, whole or refused. Two
    /// `Transfer` rows at one act, the debit then the credit, as the fold
    /// reads a movement between two identities (`nets_of`); they carry no
    /// proof of work — a payment is not a verdict on work.
    pub fn pay(&mut self, from: [u8; 32], to: [u8; 32], mu: impl Into<Mu>, anchor: Anchor) -> Result<(), Unpaid> {
        let mu = mu.into();
        if mu.is_zero() {
            return Err(Unpaid::Nothing);
        }
        if from == to {
            return Err(Unpaid::Oneself);
        }
        self.transfer(from, to, &mu, anchor)
    }

    /// The debit, the credit and the two rows: a short payer moves nothing.
    fn transfer(&mut self, from: [u8; 32], to: [u8; 32], mu: &Mu, anchor: Anchor) -> Result<(), Unpaid> {
        self.transfer_paying(from, to, mu, anchor, None)
    }

    /// A transfer whose two rows carry the proof of work it pays (78.154a:
    /// a POUW paid out of the recycled pool), or none.
    fn transfer_paying(&mut self, from: [u8; 32], to: [u8; 32], mu: &Mu, anchor: Anchor, arm: Option<Arm>) -> Result<(), Unpaid> {
        let short = mu.shortfall(&self.balance_mu(&from));
        if !short.is_zero() {
            return Err(Unpaid::Short(short));
        }
        let drawn = self.debit(from, mu);
        debug_assert!(drawn.is_ok(), "the shortfall was read as none");
        self.credit(to, mu);
        self.write(Entry { kind: Kind::Transfer, who: from, mu: mu.clone(), at: anchor.0, receipt: anchor.1, arm });
        self.write(Entry { kind: Kind::Transfer, who: to, mu: mu.clone(), at: anchor.0, receipt: anchor.1, arm });
        Ok(())
    }

    /// 78.154a — THE CONNECTION TAX RECEIVED INTO THE RECYCLED POOL: `mu`
    /// paid from `from` (a domain's reserve) to `court`, and held by the
    /// court under [`RECYCLED`] — a transfer, never a mint and never a burn.
    /// Whole or refused.
    pub fn recycle(&mut self, from: [u8; 32], court: [u8; 32], mu: impl Into<Mu>, anchor: Anchor) -> Result<(), Unpaid> {
        let mu = mu.into();
        if mu.is_zero() {
            return Err(Unpaid::Nothing);
        }
        let pool = self.held.get(RECYCLED).cloned();
        let now = match &pool {
            Some(h) if h.payer != court => return Err(Unpaid::Taken),
            Some(h) => h.mu.add(&mu),
            None => mu.clone(),
        };
        if from != court {
            self.transfer(from, court, &mu, anchor)?;
        }
        let short = mu.shortfall(&self.balance_mu(&court));
        if !short.is_zero() {
            return Err(Unpaid::Short(short));
        }
        let drawn = self.debit(court, &mu);
        debug_assert!(drawn.is_ok(), "the shortfall was read as none");
        let anchored = match pool {
            Some(h) => h.anchor,
            None => anchor,
        };
        self.held.insert(RECYCLED, Held { payer: court, mu: now, anchor: anchored });
        self.write(Entry { kind: Kind::Escrow, who: court, mu, at: anchor.0, receipt: anchor.1, arm: None });
        Ok(())
    }

    /// 78.154a — what this court's recycled pool holds, per lane.
    pub fn recycled(&self) -> Mu {
        match self.held.get(RECYCLED) {
            Some(h) => h.mu.clone(),
            None => Mu::zero(),
        }
    }

    /// 78.154a — A PROOF OF WORK PAID FROM THE RECYCLED POOL FIRST: per lane
    /// the lesser of `mu` and what the pool holds, paid to `to` as a
    /// transfer carrying `arm`. Answers what the pool paid; nothing where it
    /// holds nothing.
    pub(crate) fn draw_pool(&mut self, to: [u8; 32], mu: &Mu, anchor: Anchor, arm: Option<Arm>) -> Mu {
        let Some(h) = self.held.get(RECYCLED).cloned() else { return Mu::zero() };
        let (take, left) = h.mu.split_at(mu);
        if take.is_zero() {
            return take;
        }
        match left.is_zero() {
            true => {
                self.held.remove(RECYCLED);
            }
            false => {
                self.held.insert(RECYCLED, Held { mu: left, ..h.clone() });
            }
        }
        self.credit(h.payer, &take);
        if to == h.payer {
            self.write(Entry { kind: Kind::Revoke, who: h.payer, mu: take.clone(), at: anchor.0, receipt: anchor.1, arm });
            return take;
        }
        match self.transfer_paying(h.payer, to, &take, anchor, arm) {
            Ok(()) => take,
            Err(_) => Mu::zero(),
        }
    }

    /// 78.154 (4) — A CLAIM DRAWN ON A STAKE: `mu` of `who`'s stake set
    /// aside under `key` — leaving the stake (an Unstake row) and held (an
    /// Escrow row) until it is released to the courts it pays. Whole or
    /// refused: a stake short on a lane moves nothing.
    pub fn claim_stake(&mut self, party: [u8; 32], key: &[u8], mu: impl Into<Mu>, anchor: Anchor) -> Result<(), Unpaid> {
        let mu = mu.into();
        if mu.is_zero() {
            return Err(Unpaid::Nothing);
        }
        if self.held.contains(key) {
            return Err(Unpaid::Taken);
        }
        let staked = match self.stake_of(&party) {
            Some(s) => s,
            None => Mu::zero(),
        };
        let next = staked.draw(&mu).map_err(|_| Unpaid::Short(mu.shortfall(&staked)))?;
        self.stakes.insert(party, next);
        self.refresh_bond();
        self.credit(party, &mu);
        self.write(Entry { kind: Kind::Unstake, who: party, mu: mu.clone(), at: anchor.0, receipt: anchor.1, arm: None });
        self.hold(party, key, mu, anchor)
    }

    /// HOLD: `mu` of `payer`'s balance set aside under `key`, whole or
    /// refused. One `Escrow` row: the value stays the payer's until released,
    /// so the fold nets it as a movement within one identity.
    pub fn hold(&mut self, payer: [u8; 32], key: &[u8], mu: impl Into<Mu>, anchor: Anchor) -> Result<(), Unpaid> {
        let mu = mu.into();
        if mu.is_zero() {
            return Err(Unpaid::Nothing);
        }
        if self.held.contains(key) {
            return Err(Unpaid::Taken);
        }
        let short = mu.shortfall(&self.balance_mu(&payer));
        if !short.is_zero() {
            return Err(Unpaid::Short(short));
        }
        let drawn = self.debit(payer, &mu);
        debug_assert!(drawn.is_ok(), "the shortfall was read as none");
        self.held.insert(key, Held { payer, mu: mu.clone(), anchor });
        self.write(Entry { kind: Kind::Escrow, who: payer, mu, at: anchor.0, receipt: anchor.1, arm: None });
        Ok(())
    }

    /// RELEASE what `key` holds to `to`: the hold leaves the held carrier
    /// and is paid on as a movement from its payer (the `Transfer` pair).
    /// Released to its own payer it is a refund. Answers what moved.
    pub fn release_held(&mut self, key: &[u8], to: [u8; 32], anchor: Anchor) -> Result<Mu, Unpaid> {
        let Some(h) = self.held.get(key).cloned() else { return Err(Unpaid::Unheld) };
        if h.payer == to {
            return self.refund_held(key, anchor);
        }
        self.held.remove(key);
        // Back to the payer, then on as one transfer: the rows say who paid
        // whom, and the balances end where the release puts them.
        self.credit(h.payer, &h.mu);
        self.transfer(h.payer, to, &h.mu, anchor)?;
        Ok(h.mu)
    }

    /// MU-ECONOMY §5.2 — DRAW PART of what `key` holds to `to`: per lane the
    /// lesser of `mu` and what is held, paid on as a movement from the payer
    /// (the `Transfer` pair); the hold keeps the rest, and leaves the held
    /// carrier when nothing remains. Answers what moved — nothing where the
    /// draw meets no lane the hold carries.
    pub fn draw_held(&mut self, key: &[u8], to: [u8; 32], mu: impl Into<Mu>, anchor: Anchor) -> Result<Mu, Unpaid> {
        let Some(h) = self.held.get(key).cloned() else { return Err(Unpaid::Unheld) };
        if h.payer == to {
            return Err(Unpaid::Oneself);
        }
        let (take, left) = h.mu.split_at(&mu.into());
        if take.is_zero() {
            return Err(Unpaid::Nothing);
        }
        match left.is_zero() {
            true => {
                self.held.remove(key);
            }
            false => {
                self.held.insert(key, Held { mu: left, ..h.clone() });
            }
        }
        // Back to the payer, then on as one transfer, as a release moves it.
        self.credit(h.payer, &take);
        self.transfer(h.payer, to, &take, anchor)?;
        Ok(take)
    }

    /// 78.133 — RELEASE what `key` holds IN PARTS, each to its recipient,
    /// whole or refused: the parts must sum to the hold exactly (`Split`),
    /// so all parts are paid or none. Each part a `Transfer` pair at
    /// `anchor` (the act it ties back to). Answers what moved.
    pub fn release_split(&mut self, key: &[u8], parts: &[([u8; 32], Mu)], anchor: Anchor) -> Result<Mu, Unpaid> {
        let Some(h) = self.held.get(key).cloned() else { return Err(Unpaid::Unheld) };
        if Mu::sum(parts.iter().map(|(_, p)| p)) != h.mu {
            return Err(Unpaid::Split);
        }
        self.held.remove(key);
        self.credit(h.payer, &h.mu);
        for (to, part) in parts.iter().filter(|(_, p)| !p.is_zero()) {
            if *to == h.payer {
                continue;
            }
            self.transfer(h.payer, *to, part, anchor)?;
        }
        Ok(h.mu)
    }

    /// REFUND what `key` holds to its payer. One `Revoke` row, netting
    /// nothing: the value never stopped being the payer's.
    pub fn refund_held(&mut self, key: &[u8], anchor: Anchor) -> Result<Mu, Unpaid> {
        let Some(h) = self.held.get(key).cloned() else { return Err(Unpaid::Unheld) };
        self.held.remove(key);
        self.credit(h.payer, &h.mu);
        self.write(Entry { kind: Kind::Revoke, who: h.payer, mu: h.mu.clone(), at: anchor.0, receipt: anchor.1, arm: None });
        Ok(h.mu)
    }

    /// What stands held under `key`, if anything.
    pub fn held_of(&self, key: &[u8]) -> Option<&Held> {
        self.held.get(key)
    }
}
