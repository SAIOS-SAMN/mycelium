// E1, E2 — ESCROW: value set aside against a listing, and what releases it.
//
//! A FUNDED escrow is a liability and is counted in the supply; a CLAIM
//! escrow is pending issuance and is not. Nothing here mints on the way in;
//! a mint happens on closure and a burn on retirement.

use super::*;

/// WHAT A REVOCATION DID, read per lane and per voice — never a bool. The
/// three outcomes a caller must tell apart are the escrow dropped with
/// nothing slashed, the bond of each named voice debited by what actually
/// left it, and a bond that did not cover the ask, which is the shortfall
/// and names its lanes (law 5, §2.3).
#[derive(Debug, Clone)]
pub struct Revoked {
    /// The escrow that was dropped.
    pub released: [u64; 4],
    /// What left each voice's bond. Empty on a plain revoke.
    pub slashed: Vec<([u8; 32], [u64; 4])>,
    /// Where a voice's bond did not cover the ask, per lane.
    pub short: Vec<([u8; 32], [u64; 4])>,
}

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
        // `paid` is clamped to the balance above, so this holds; a debit
        // that does not escrows nothing rather than funding an award the
        // payer did not pay for (§6).
        if !crate::economy::lanes_short(&self.debit(payer, paid)).is_empty() {
            return [0; 4];
        }
        self.escrow_award(addr, vec![payer], paid, anchor);
        // An address nothing has funded yet is funded by nothing: the
        // reading of absence on a keyed carrier (0.11c).
        let prev = match self.funded.get(addr) {
            Some((_, m)) => *m,
            None => [0; 4],
        };
        let next: [u64; 4] = std::array::from_fn(|l| prev[l] + paid[l]);
        self.funded.insert(addr, (payer, next));
        self.write(Entry { kind: Kind::Escrow, who: payer, mu: paid, k_step: anchor.0, receipt: anchor.1 });
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
        // WHAT IS HELD IN ESCROW, all of it: this is the carrier's own
        // reading, the one a persist and a treasury line are taken from.
        self.escrowed
            .scoped()
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
        self.discharge_funded(addr, Kind::Release);
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
    pub(crate) fn discharge_funded(&mut self, addr: &[u8], kind: Kind) {
        let Some((payer, mu)) = self.funded.remove(addr) else {
            return;
        };
        // A funded escrow carrying nothing on any lane returns nothing.
        // `mu.iter().all(|&x| x == 0)` folds four lanes into one bool to
        // decide the branch (rule 2, §2.3); `lanes_short` is the tree's own
        // reading and names which lanes carry value.
        if super::lanes_short(&mu).is_empty() {
            return;
        }
        // An address with no escrow row was anchored at no step and no
        // receipt, which is the empty anchor read out.
        let anchor = match self.escrowed.get(addr) {
            Some(row) => row.anchor,
            None => (0, [0; 32]),
        };
        self.credit(payer, mu);
        self.write(Entry { kind, who: payer, mu, k_step: anchor.0, receipt: anchor.1 });
    }

    /// Mint a released claim: a consilience split when two or more voices
    /// stand it, the vector to the one voice when one does, nothing when
    /// the escrow names none.
    ///
    /// The arms are read off the slice, not off `voices.len() >= 2` and a
    /// `.first()` behind it — rule 2's "a count is never a branch", and a
    /// `.first()` that looks like rule 15's tie-break by arrival order even
    /// where, as here, the one voice is the only voice. The shape says
    /// which case it is.
    pub fn mint_claim(&mut self, voices: &[[u8; 32]], mu: [u64; 4], anchor: Anchor) {
        match voices {
            [] => {}
            [one] => {
                self.mint_to(*one, mu, anchor);
            }
            _ => self.settle_consilience(voices, mu, anchor),
        }
    }

    /// Drop the listing claim. A slash debits the bond of EVERY voice the
    /// escrow names and mints nothing. `None` when the address held no
    /// escrow to drop.
    ///
    /// Three faults, one line apart, and I11a wrote the middle one two
    /// files from the paragraph in `settle.rs` condemning it.
    ///
    /// `mu[l].min(held[l])` is rule 4's "any `.min()`/`.max()`/clamp on two
    /// measurements" and law 5's "a comparison of two lane vectors returns
    /// the reading per lane and never a verdict". The escrow's ask and the
    /// voice's bond are two measurements; what leaves is the ask less the
    /// shortfall, and the shortfall is the reading — the lanes on which the
    /// bond did not cover what was asked, which is what a court needs to
    /// know and what the clamp threw away.
    ///
    /// `took.iter().all(|&x| x == 0)` folds four lanes into one bool to
    /// decide a branch (rule 2, §2.3), and both sides of it returned the
    /// same `true`, so `revoke_award` answered identically for "the bond
    /// was slashed", "the bond was empty" and "the escrow named no voice".
    /// It returns the reading now.
    ///
    /// `voices.first()` slashed one voice by arrival order — rule 15's
    /// "a tie-break by index, by size, by first found presented as a
    /// decision". `MU-ECONOMY.md` §4 debits "the voice that spoke it", and
    /// `mint_claim` splits an award across every voice the escrow names;
    /// a slash reaches the same set, or a word two voices stood costs one
    /// of them nothing at all.
    pub fn revoke_award(&mut self, addr: &[u8], slash: bool) -> Option<Revoked> {
        self.discharge_funded(addr, Kind::Revoke);
        let Escrowed { voices, mu, anchor } = self.escrowed.remove(addr)?;
        let mut out = Revoked { released: mu, slashed: Vec::new(), short: Vec::new() };
        if !slash {
            return Some(out);
        }
        for who in voices {
            let held = match self.stakes.get(&who) {
                Some(&h) => h,
                None => [0; 4],
            };
            let short = super::shortfall(&mu, &held);
            let took: [u64; 4] = std::array::from_fn(|l| mu[l] - short[l]);
            if !super::lanes_short(&short).is_empty() {
                out.short.push((who, short));
            }
            if super::lanes_short(&took).is_empty() {
                continue;
            }
            let next: [u64; 4] = std::array::from_fn(|l| held[l] - took[l]);
            self.stakes.insert(who, next);
            self.write(Entry { kind: Kind::Slash, who, mu: took, k_step: anchor.0, receipt: anchor.1 });
            out.slashed.push((who, took));
        }
        if !out.slashed.is_empty() {
            self.refresh_bond();
        }
        Some(out)
    }

    pub fn stakes(&self) -> Vec<([u8; 32], [u64; 4])> {
        // One entry per staking key: the whole of it is what "the stakes"
        // means, and the bond is refreshed from all of them.
        self.stakes
            .scoped()
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
        let asked = mu;
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
        // An address with no escrow row holds nothing in escrow.
        match self.escrowed.get(addr) {
            Some(row) => row.mu,
            None => [0; 4],
        }
    }

}
