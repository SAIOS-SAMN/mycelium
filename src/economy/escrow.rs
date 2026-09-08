// E1, E2 — ESCROW: value set aside against a listing, and what releases it.
//
//! A FUNDED escrow is a liability and is counted in the supply; a CLAIM
//! escrow is pending issuance and is not. Nothing here mints on the way in;
//! a mint happens on closure and a burn on retirement.

use super::*;

/// WHAT AN ESCROW HOLDS: a claim on issuance (§5), pending the court.
#[derive(Debug, Clone)]
pub struct Escrowed {
    /// The voices whose consilience backs the claim. Two or more split the
    /// mint; one takes the vector. Parties, never vantages.
    pub voices: Vec<[u8; 32]>,
    /// The μ⃗ claimed, per lane. Unfinalized speech earns nothing, so this
    /// is a claim and not a balance until the verdict releases it.
    pub mu: [u64; 4],
    /// The k-step and receipt the claim stands at, so a release books at
    /// the moment it was earned and not the moment it was paid.
    pub anchor: Anchor,
}

/// An escrow read off the book, carrying the address it is filed under.
pub struct EscrowedAward {
    /// The listing the claim is against.
    pub addr: Vec<u8>,
    /// What that listing holds.
    pub held: Escrowed,
}

impl Treasury {
    /// FUND an escrow from a payer's balance — the xSEPTUM Note (§5), where
    /// a requester backs a claim with μ⃗ that already exists. The balance is
    /// DEBITED, not merely checked, and the funded part is recorded so R9's
    /// `Σ escrow` counts it. Returns what was actually funded; a payer short
    /// on a lane funds what it holds there and no more.
    pub fn escrow_from_balance(
        &mut self,
        addr: impl AsRef<[u8]>,
        payer: [u8; 32],
        mu: [u64; 4],
        anchor: Anchor,
    ) -> [u64; 4] {
        let held = self.balance_mu(&payer);
        let paid: [u64; 4] = std::array::from_fn(|l| mu[l].min(held[l]));
        if paid.iter().all(|&x| x == 0) {
            return paid;
        }
        let addr = addr.as_ref();
        self.debit(payer, paid);
        self.escrow_award(addr, vec![payer], paid, anchor);
        let prev = self.funded.get(addr).map(|(_, m)| *m).unwrap_or([0; 4]);
        let next: [u64; 4] = std::array::from_fn(|l| prev[l] + paid[l]);
        self.funded.insert(addr, (payer, next));
        self.entries.push(Entry { kind: "escrow", who: payer, mu: paid, k_step: anchor.0, receipt: anchor.1 });
        paid
    }

    /// HOLD a consilience award in escrow pending the court's finality verdict
    /// (Step 5: two-voice consilience is provisional, not final). Nothing books
    /// to the ledger yet — unfinalized speech earns nothing.
    pub fn escrow_award(&mut self, addr: impl AsRef<[u8]>, voices: Vec<[u8; 32]>, mu: [u64; 4], anchor: Anchor) {
        let addr = addr.as_ref();
        match self.escrowed.get_mut(addr) {
            Some(row) => {
                for v in voices {
                    match row.voices.iter().position(|h| *h == v) {
                        Some(_) => {}
                        None => row.voices.push(v),
                    }
                }
            }
            None => {
                self.escrowed.insert(addr, Escrowed { voices, mu, anchor });
            }
        }
    }

    pub fn escrow_of(&self, addr: &[u8]) -> Option<&Escrowed> {
        self.escrowed.get(addr)
    }

    pub fn escrowed_awards(&self) -> Vec<EscrowedAward> {
        self.escrowed
            .fiber(crate::network::nerve::ROOT)
            .map(|(addr, held)| EscrowedAward { addr, held: held.clone() })
            .collect()
    }

    /// A filled triangle releases the escrow of the listings its dialects settled.
    pub fn release_filled<A: AsRef<[u8]>>(&mut self, listings: &[A]) -> [u64; 4] {
        let mut released = [0u64; 4];
        for addr in listings {
            if let Some(Escrowed { voices, mu, anchor }) = self.release_award(addr.as_ref()) {
                self.mint_claim(&voices, mu, anchor);
                for (r, m) in released.iter_mut().zip(&mu) {
                    *r = r.saturating_add(*m);
                }
            }
        }
        released
    }

    /// Clear the listing claim. Payment is a mint of the same vector, done
    /// by the caller after this returns (release then mint).
    pub fn release_award(&mut self, addr: &[u8]) -> Option<Escrowed> {
        self.discharge_funded(addr, "release");
        self.escrowed.remove(addr)
    }

    /// Return the funded part of an escrow to the balance it came from. A
    /// funded escrow holds μ⃗ that already existed, so discharging it MOVES
    /// value and never mints or destroys any; a claim-escrow has no funded
    /// part and this does nothing. Dropping the record instead would leave
    /// the carriers short of supply and R9 would fail on the next burn.
    ///
    /// The payee of a funded release is xSEPTUM §5's business, not the
    /// treasury's: until the Note names one, the μ⃗ goes back where it came
    /// from rather than to a party chosen here.
    pub(crate) fn discharge_funded(&mut self, addr: &[u8], kind: &'static str) {
        let Some((payer, mu)) = self.funded.remove(addr) else {
            return;
        };
        if mu.iter().all(|&x| x == 0) {
            return;
        }
        let anchor = self
            .escrowed
            .get(addr)
            .map(|row| row.anchor)
            .unwrap_or((0, [0; 32]));
        self.credit(payer, mu);
        self.entries.push(Entry { kind, who: payer, mu, k_step: anchor.0, receipt: anchor.1 });
    }

    /// Mint a released claim: consilience split when two voices stand, else
    /// the first voice takes the vector.
    pub fn mint_claim(&mut self, voices: &[[u8; 32]], mu: [u64; 4], anchor: Anchor) {
        if voices.len() >= 2 {
            self.settle_consilience(voices, mu, anchor);
        } else if let Some(&v) = voices.first() {
            self.mint_to(v, mu, anchor);
        }
    }

    /// Drop the listing claim. A slash debits the stake bond and mints nothing.
    pub fn revoke_award(&mut self, addr: &[u8], slash: bool) -> bool {
        self.discharge_funded(addr, "revoke");
        let Some(Escrowed { voices, mu, anchor }) = self.escrowed.remove(addr) else {
            return false;
        };
        if slash {
            let Some(&who) = voices.first() else {
                return true;
            };
            if let Some(&held) = self.stakes.get(&who) {
                let next: [u64; 4] = std::array::from_fn(|l| held[l].saturating_sub(mu[l]));
                self.stakes.insert(who, next);
            }
            self.entries.push(Entry { kind: "slash", who, mu, k_step: anchor.0, receipt: anchor.1 });
            self.refresh_bond();
        }
        true
    }

    pub fn stakes(&self) -> Vec<([u8; 32], [u64; 4])> {
        self.stakes
            .fiber(crate::network::nerve::ROOT)
            .filter_map(|(w, mu)| <[u8; 32]>::try_from(w.as_slice()).ok().map(|who| (who, *mu)))
            .collect()
    }

    /// A VERIFIED SOLVE (PoUW): the registry confirmed a prediction against
    /// its held answers and named the task's tension. Occupied lanes pay at
    /// least 1. This is syntropic mining — the bounty is minted as the
    /// task's locked tension, never drawn from the stake bond.
    /// Draw a verified-solve payment from an escrowed listing award, so
    /// reduction and solve share one ledger. Remaining escrow stays for
    /// consilience release.
    pub fn draw_escrow(&mut self, addr: &[u8], voice: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        let Some(row) = self.escrowed.get(addr) else {
            return [0; 4];
        };
        let held = row.mu;
        let asked: [u64; 4] = std::array::from_fn(|l| if mu[l] == 0 { 0 } else { mu[l].max(1) });
        let take: [u64; 4] = std::array::from_fn(|l| asked[l].min(held[l]));
        if take.iter().all(|&m| m == 0) {
            return take;
        }
        if let Some(row) = self.escrowed.get_mut(addr) {
            for (h, t) in row.mu.iter_mut().zip(&take) {
                *h -= t;
            }
        }
        self.mint_to(voice, take, anchor)
    }

    #[cfg(test)]
    pub fn escrowed_remaining(&self, addr: &[u8]) -> [u64; 4] {
        self.escrowed.get(addr).map(|row| row.mu).unwrap_or([0; 4])
    }

}
