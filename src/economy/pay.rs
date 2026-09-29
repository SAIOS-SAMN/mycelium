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
//! vector; a movement is whole or refused, named, and a refused one writes
//! no row.

use super::*;

/// VALUE A PAYER SET ASIDE AGAINST A KEY: whose it is, how much per lane,
/// and the act it was set aside at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Held {
    pub payer: [u8; 32],
    pub mu: [u64; 4],
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
    Short([u64; 4]),
    /// The payee's balance would pass its carrier on a lane (STD1), counted.
    Overflow,
    /// A hold already stands under this key.
    Taken,
    /// No hold stands under this key.
    Unheld,
}

impl Treasury {
    /// PAY: `mu` from `from`'s balance to `to`'s, whole or refused. Two
    /// `Transfer` rows at one act, the debit then the credit, as the fold
    /// reads a movement between two identities (`nets_of`); they carry no
    /// proof of work — a payment is not a verdict on work.
    pub fn pay(&mut self, from: [u8; 32], to: [u8; 32], mu: [u64; 4], anchor: Anchor) -> Result<(), Unpaid> {
        if mu.iter().all(|&m| m == 0) {
            return Err(Unpaid::Nothing);
        }
        if from == to {
            return Err(Unpaid::Oneself);
        }
        self.transfer(from, to, mu, anchor)
    }

    /// The debit, the credit and their two rows, both read before either
    /// moves: a short payer or a full payee moves nothing.
    fn transfer(&mut self, from: [u8; 32], to: [u8; 32], mu: [u64; 4], anchor: Anchor) -> Result<(), Unpaid> {
        let short = crate::economy::shortfall(&mu, &self.balance_mu(&from));
        if !crate::economy::lanes_short(&short).is_empty() {
            return Err(Unpaid::Short(short));
        }
        if self.credited(&to, mu).is_none() {
            self.overflowed += 1;
            return Err(Unpaid::Overflow);
        }
        let unmoved = self.debit(from, mu);
        debug_assert!(crate::economy::lanes_short(&unmoved).is_empty(), "the shortfall was read as none");
        let landed = self.credit(to, mu);
        debug_assert!(landed, "the credit was read as landing");
        self.write(Entry { kind: Kind::Transfer, who: from, mu, at: anchor.0, receipt: anchor.1, arm: None });
        self.write(Entry { kind: Kind::Transfer, who: to, mu, at: anchor.0, receipt: anchor.1, arm: None });
        Ok(())
    }

    /// HOLD: `mu` of `payer`'s balance set aside under `key`, whole or
    /// refused. One `Escrow` row: the value stays the payer's until released,
    /// so the fold nets it as a movement within one identity.
    pub fn hold(&mut self, payer: [u8; 32], key: &[u8], mu: [u64; 4], anchor: Anchor) -> Result<(), Unpaid> {
        if mu.iter().all(|&m| m == 0) {
            return Err(Unpaid::Nothing);
        }
        if self.held.contains(key) {
            return Err(Unpaid::Taken);
        }
        let short = crate::economy::shortfall(&mu, &self.balance_mu(&payer));
        if !crate::economy::lanes_short(&short).is_empty() {
            return Err(Unpaid::Short(short));
        }
        let unmoved = self.debit(payer, mu);
        debug_assert!(crate::economy::lanes_short(&unmoved).is_empty(), "the shortfall was read as none");
        self.held.insert(key, Held { payer, mu, anchor });
        self.write(Entry { kind: Kind::Escrow, who: payer, mu, at: anchor.0, receipt: anchor.1, arm: None });
        Ok(())
    }

    /// RELEASE what `key` holds to `to`: the hold leaves the held carrier
    /// and is paid on as a movement from its payer (the `Transfer` pair).
    /// Released to its own payer it is a refund. Answers what moved.
    pub fn release_held(&mut self, key: &[u8], to: [u8; 32], anchor: Anchor) -> Result<[u64; 4], Unpaid> {
        let Some(h) = self.held.get(key).cloned() else { return Err(Unpaid::Unheld) };
        if h.payer == to {
            return self.refund_held(key, anchor);
        }
        if self.credited(&to, h.mu).is_none() {
            self.overflowed += 1;
            return Err(Unpaid::Overflow);
        }
        self.held.remove(key);
        // Back to the payer, then on as one transfer: the rows say who paid
        // whom, and the balances end where the release puts them.
        let back = self.credit(h.payer, h.mu);
        debug_assert!(back, "the hold came out of this balance");
        self.transfer(h.payer, to, h.mu, anchor)?;
        Ok(h.mu)
    }

    /// REFUND what `key` holds to its payer. One `Revoke` row, netting
    /// nothing: the value never stopped being the payer's.
    pub fn refund_held(&mut self, key: &[u8], anchor: Anchor) -> Result<[u64; 4], Unpaid> {
        let Some(h) = self.held.get(key).cloned() else { return Err(Unpaid::Unheld) };
        if self.credited(&h.payer, h.mu).is_none() {
            self.overflowed += 1;
            return Err(Unpaid::Overflow);
        }
        self.held.remove(key);
        let back = self.credit(h.payer, h.mu);
        debug_assert!(back, "the credit was read as landing");
        self.write(Entry { kind: Kind::Revoke, who: h.payer, mu: h.mu, at: anchor.0, receipt: anchor.1, arm: None });
        Ok(h.mu)
    }

    /// What stands held under `key`, if anything.
    pub fn held_of(&self, key: &[u8]) -> Option<&Held> {
        self.held.get(key)
    }
}
