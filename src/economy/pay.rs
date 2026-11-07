// E3 — THE SYNTHETIC ECONOMY: a payment between two identities, and value a
// payer holds against a key until it is released to a payee or refunded.
//
//! It stands in for the contracts until they are deployed: the court is the
//! ledger, every movement is a row at the act it stands at, and nothing here
//! issues or burns. A payment moves μ⃗ between two balances; a hold moves it
//! from the payer's balance into the HELD carrier, still the payer's until a
//! release pays it on or a refund returns it. §6's second equation counts
//! the held carrier beside the balances and the bonds (`audit`), so both
//! equations hold through every movement here. An amount asked is a
//! per-lane reading ([`Mu`]); it is drawn from the payer's modes on each
//! lane in proportion to their amounts, and the modes drawn travel with the
//! movement (#169). A movement is whole or refused, named, and a refused
//! one writes no row.

use super::*;
use super::modes::Modes;

/// 78.154a — THE RECYCLED POOL'S KEY: the connection tax a court received,
/// held by the court, paying its closers' POUW before anything is minted.
pub const RECYCLED: &[u8] = b"recycled pool";

/// VALUE A PAYER SET ASIDE AGAINST A KEY: whose it is, the modes set aside,
/// and the act it was set aside at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetAside {
    pub payer: [u8; 32],
    pub modes: Modes,
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
    /// a POUW paid out of the recycled pool), or none. The amount is drawn
    /// from the payer's modes in proportion on each lane, and those modes
    /// are what the payee receives.
    fn transfer_paying(&mut self, from: [u8; 32], to: [u8; 32], mu: &Mu, anchor: Anchor, arm: Option<Arm>) -> Result<(), Unpaid> {
        let short = mu.shortfall(&self.balance_mu(&from));
        if !short.is_zero() {
            return Err(Unpaid::Short(short));
        }
        let drawn = self.debit(from, mu).map_err(|_| Unpaid::Short(short))?;
        self.transfer_drawn(from, to, drawn, anchor, arm);
        Ok(())
    }

    /// MODES ALREADY DRAWN FROM `from`, credited to `to`: the two rows of
    /// one transfer, the debit then the credit.
    pub(crate) fn transfer_drawn(&mut self, from: [u8; 32], to: [u8; 32], drawn: Modes, anchor: Anchor, arm: Option<Arm>) {
        self.credit(to, &drawn);
        self.write(Treasury::row(Kind::Transfer, from, drawn.clone(), anchor, arm));
        self.write(Treasury::row(Kind::Transfer, to, drawn, anchor, arm));
    }

    /// THESE MODES, held under a set-aside, paid on from its payer to `to`
    /// as one transfer: back to the payer, then the two rows.
    fn pay_on(&mut self, payer: [u8; 32], to: [u8; 32], modes: Modes, anchor: Anchor) {
        self.credit(payer, &modes);
        // Just credited, so held whole.
        if self.debit_modes(payer, &modes).is_ok() {
            self.transfer_drawn(payer, to, modes, anchor, None);
        }
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
        if let Some(h) = &pool {
            if h.payer != court {
                return Err(Unpaid::Taken);
            }
        }
        if from != court {
            self.transfer(from, court, &mu, anchor)?;
        }
        let short = mu.shortfall(&self.balance_mu(&court));
        if !short.is_zero() {
            return Err(Unpaid::Short(short));
        }
        let drawn = self.debit(court, &mu).map_err(|_| Unpaid::Short(short))?;
        let (now, anchored) = match pool {
            Some(h) => (h.modes.with(&drawn), h.anchor),
            None => (drawn.clone(), anchor),
        };
        self.held.insert(RECYCLED, SetAside { payer: court, modes: now, anchor: anchored });
        self.write(Treasury::row(Kind::Escrow, court, drawn, anchor, None));
        Ok(())
    }

    /// 78.154a — what this court's recycled pool holds, read per lane.
    pub fn recycled(&self) -> Mu {
        match self.held.get(RECYCLED) {
            Some(h) => h.modes.reading(),
            None => Mu::zero(),
        }
    }

    /// 78.154a — A PROOF OF WORK PAID FROM THE RECYCLED POOL FIRST: per lane
    /// the lesser of `mu` and what the pool holds, drawn from the pool's
    /// modes in proportion and paid to `to` as a transfer carrying `arm`.
    /// Answers the reading of what the pool paid; nothing where it holds
    /// nothing.
    pub(crate) fn draw_pool(&mut self, to: [u8; 32], mu: &Mu, anchor: Anchor, arm: Option<Arm>) -> Mu {
        let Some(h) = self.held.get(RECYCLED).cloned() else { return Mu::zero() };
        let (take, left) = h.modes.split_at(mu);
        if take.is_empty() {
            return Mu::zero();
        }
        let read = take.reading();
        match left.is_empty() {
            true => {
                self.held.remove(RECYCLED);
            }
            false => {
                self.held.insert(RECYCLED, SetAside { modes: left, ..h.clone() });
            }
        }
        self.credit(h.payer, &take);
        if to == h.payer {
            self.write(Treasury::row(Kind::Revoke, h.payer, take, anchor, arm));
            return read;
        }
        if self.debit_modes(h.payer, &take).is_err() {
            return Mu::zero();
        }
        self.transfer_drawn(h.payer, to, take, anchor, arm);
        read
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
        let mut staked = self.staked(&party);
        let drawn = staked.draw(&mu).map_err(|_| Unpaid::Short(mu.shortfall(&self.staked(&party).reading())))?;
        self.stakes.insert(party, staked);
        self.refresh_bond();
        // The modes leave the stake and are held as they were staked: the
        // two rows carry the same modes, so the balance between them holds
        // exactly what the stake gave up.
        self.write(Treasury::row(Kind::Unstake, party, drawn.clone(), anchor, None));
        self.held.insert(key, SetAside { payer: party, modes: drawn.clone(), anchor });
        self.write(Treasury::row(Kind::Escrow, party, drawn, anchor, None));
        Ok(())
    }

    /// HOLD: `mu` of `payer`'s balance set aside under `key`, whole or
    /// refused, drawn from the balance's modes in proportion on each lane.
    /// One `Escrow` row: the value stays the payer's until released, so the
    /// fold nets it as a movement within one identity.
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
        let drawn = self.debit(payer, &mu).map_err(|_| Unpaid::Short(short))?;
        self.held.insert(key, SetAside { payer, modes: drawn.clone(), anchor });
        self.write(Treasury::row(Kind::Escrow, payer, drawn, anchor, None));
        Ok(())
    }

    /// RELEASE what `key` holds to `to`: the hold leaves the held carrier
    /// and is paid on as a movement from its payer (the `Transfer` pair).
    /// Released to its own payer it is a refund. Answers the reading of what
    /// moved.
    pub fn release_held(&mut self, key: &[u8], to: [u8; 32], anchor: Anchor) -> Result<Mu, Unpaid> {
        let Some(h) = self.held.get(key).cloned() else { return Err(Unpaid::Unheld) };
        if h.payer == to {
            return self.refund_held(key, anchor);
        }
        self.held.remove(key);
        // Back to the payer, then on as one transfer: the rows say who paid
        // whom, and the balances end where the release puts them.
        let read = h.modes.reading();
        self.pay_on(h.payer, to, h.modes, anchor);
        Ok(read)
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
        let (take, left) = h.modes.split_at(&mu.into());
        if take.is_empty() {
            return Err(Unpaid::Nothing);
        }
        match left.is_empty() {
            true => {
                self.held.remove(key);
            }
            false => {
                self.held.insert(key, SetAside { modes: left, ..h.clone() });
            }
        }
        // Back to the payer, then on as one transfer, as a release moves it.
        let read = take.reading();
        self.pay_on(h.payer, to, take, anchor);
        Ok(read)
    }

    /// BURN PART of what `key` holds: per lane the lesser of `mu` and what
    /// is held, drawn from the hold's modes in proportion, leaving
    /// circulation as one `Burn` row of its payer (a forfeit's retired part,
    /// 78.129). The hold keeps the rest. Answers the reading of what left.
    pub fn burn_held(&mut self, key: &[u8], mu: &Mu, anchor: Anchor) -> Result<Mu, Unpaid> {
        let Some(h) = self.held.get(key).cloned() else { return Err(Unpaid::Unheld) };
        let (take, left) = h.modes.split_at(mu);
        if take.is_empty() {
            return Err(Unpaid::Nothing);
        }
        match left.is_empty() {
            true => {
                self.held.remove(key);
            }
            false => {
                self.held.insert(key, SetAside { modes: left, ..h.clone() });
            }
        }
        let read = take.reading();
        self.write(Treasury::row(Kind::Burn, h.payer, take, anchor, None));
        Ok(read)
    }

    /// 78.133 — RELEASE what `key` holds IN PARTS, each to its recipient,
    /// whole or refused: the parts must sum to the hold exactly (`Split`),
    /// so all parts are paid or none. Each part is drawn from the hold's
    /// modes in proportion on each lane, a `Transfer` pair at `anchor` (the
    /// act it ties back to). Answers the reading of what moved.
    pub fn release_split(&mut self, key: &[u8], parts: &[([u8; 32], Mu)], anchor: Anchor) -> Result<Mu, Unpaid> {
        let Some(h) = self.held.get(key).cloned() else { return Err(Unpaid::Unheld) };
        let read = h.modes.reading();
        if Mu::sum(parts.iter().map(|(_, p)| p)) != read {
            return Err(Unpaid::Split);
        }
        self.held.remove(key);
        let mut left = h.modes.clone();
        let mut paid = Vec::with_capacity(parts.len());
        for (to, part) in parts.iter().filter(|(_, p)| !p.is_zero()) {
            // The parts sum to the hold, so each draws whole from what is left.
            let drawn = left.draw(part).map_err(|_| Unpaid::Split)?;
            paid.push((*to, drawn));
        }
        self.credit(h.payer, &h.modes);
        for (to, drawn) in paid {
            if to == h.payer {
                continue;
            }
            if self.debit_modes(h.payer, &drawn).is_ok() {
                self.transfer_drawn(h.payer, to, drawn, anchor, None);
            }
        }
        Ok(read)
    }

    /// REFUND what `key` holds to its payer. One `Revoke` row, netting
    /// nothing: the value never stopped being the payer's.
    pub fn refund_held(&mut self, key: &[u8], anchor: Anchor) -> Result<Mu, Unpaid> {
        let Some(h) = self.held.get(key).cloned() else { return Err(Unpaid::Unheld) };
        self.held.remove(key);
        let read = h.modes.reading();
        self.credit(h.payer, &h.modes);
        self.write(Treasury::row(Kind::Revoke, h.payer, h.modes, anchor, None));
        Ok(read)
    }

    /// What stands held under `key`, if anything.
    pub fn held_of(&self, key: &[u8]) -> Option<&SetAside> {
        self.held.get(key)
    }
}
