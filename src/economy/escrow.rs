// E1, E2 — ESCROW: value set aside against a listing, and what releases it.
//
//! An escrow is a CLAIM pending issuance and is not counted in the supply.
//! Nothing here mints on the way in; a mint happens on closure and a burn
//! on retirement.

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

/// HOW AN ESCROW IS REVOKED: dropped, its claim returning nothing; or
/// slashed, every voice it names debited, the slash standing at the act
/// that revoked it (EC1) — never at the act that escrowed it, which a fold
/// may already have passed (78.65).
#[derive(Debug, Clone, Copy)]
pub enum Revocation {
    Drop,
    Slash(Anchor),
}

/// WHAT AN ESCROW HOLDS: a claim on issuance (§5), pending the court.
#[derive(Debug, Clone)]
pub struct Escrowed {
    /// The voices whose consilience backs the claim. Two or more split the
    /// mint; one takes the vector. Parties, never vantages.
    pub voices: Vec<[u8; 32]>,
    /// The μ⃗ claimed, per lane. Unfinalized speech earns nothing, so this
    /// is a claim and not a balance until the verdict releases it. It is
    /// priced from the word's discharge alone (78.149) and is a CAP on what
    /// a release pays, never the payment.
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
    /// HOLD a consilience award in escrow pending the court's finality verdict
    /// (Step 5: two-voice consilience is provisional, not final). Nothing books
    /// to the ledger yet — unfinalized speech earns nothing.
    ///
    /// The award is filed under the word's own address bytes, SHARED with
    /// the row that holds the word (78.22): it is a field of that row, and
    /// never a second copy of the address.
    pub fn escrow_award(
        &mut self,
        word: &crate::transducer::address::Address,
        voices: Vec<[u8; 32]>,
        mu: [u64; 4],
        anchor: Anchor,
    ) {
        let addr = word.as_bytes();
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
                self.escrowed.insert_shared(word.shared(), Escrowed { voices, mu, anchor });
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

    /// A filled triangle releases the escrow of the listings its dialects
    /// settled, each at its own measured discharge (78.149 (b),
    /// [`Treasury::release_discharged`]), minted AT `at`, the act that
    /// released them (EC1), never at the acts that escrowed them. The
    /// released total is a reading: `None`, counted, where it would pass its
    /// carrier (STD1) — every release still minted, each by its own row.
    pub fn release_filled<A: AsRef<[u8]>>(&mut self, listings: &[(A, [u64; 4])], at: Anchor) -> Option<[u64; 4]> {
        let mut released = Some([0u64; 4]);
        for (addr, discharge) in listings {
            if let Some(paid) = self.release_discharged(addr.as_ref(), *discharge, at) {
                released = released.and_then(|r| crate::core::lanes::lanes_add(r, paid));
            }
        }
        if released.is_none() {
            self.overflowed += 1;
        }
        released
    }

    /// 78.149 (b) — A CLAIM IS PAID THE CLOSER'S MEASURED DISCHARGE, NEVER
    /// MORE. The escrow is released and its voices are minted, lane by lane,
    /// what the escrow held less what it held above `discharge`: the escrow
    /// CAPS the payment and never raises it. What it held above the
    /// discharge is not minted — a claim escrow is a claim on issuance and
    /// never a transfer (E1), so the excess returns to issuance by never
    /// being issued. The discharge is read by the caller on the closer's own
    /// replay (`Court::discharge_of`, the one reader); a zero discharge
    /// releases the claim and mints nothing.
    ///
    /// Answers the payment released to the voices, per lane, split among
    /// them as `mint_claim` splits it; `None` where the address held no
    /// escrow.
    pub fn release_discharged(&mut self, addr: &[u8], discharge: [u64; 4], at: Anchor) -> Option<[u64; 4]> {
        let Escrowed { voices, mu, .. } = self.release_award(addr)?;
        // The shortfall of the claim against the discharge is what the claim
        // asked beyond it, per lane; the claim less that is the payment —
        // lane-wise the lesser of the two, read as the shortfall reading.
        let above = super::shortfall(&mu, &discharge);
        let paid: [u64; 4] = std::array::from_fn(|l| mu[l] - above[l]);
        self.mint_claim(&voices, paid, at);
        Some(paid)
    }

    /// Clear the listing claim. Payment is a mint of the same vector, done
    /// by the caller after this returns (release then mint).
    pub fn release_award(&mut self, addr: &[u8]) -> Option<Escrowed> {
        self.escrowed.remove(addr)
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
                self.mint_to(*one, mu, anchor, Some(Arm::Pouw));
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
    pub fn revoke_award(&mut self, addr: &[u8], how: Revocation) -> Option<Revoked> {
        let Escrowed { voices, mu, .. } = self.escrowed.remove(addr)?;
        let mut out = Revoked { released: mu, slashed: Vec::new(), short: Vec::new() };
        let anchor = match how {
            Revocation::Drop => return Some(out),
            Revocation::Slash(at) => at,
        };
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
            self.write(Entry { kind: Kind::Slash, who, mu: took, at: anchor.0, receipt: anchor.1, arm: None });
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

    /// A VERIFIED SOLVE (POUW): the registry confirmed a prediction against
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
        self.mint_to(voice, take, anchor, Some(Arm::Pouw))
    }

    /// A CARRIAGE OF A PROVISIONAL WORD DRAWS ITS OWN FRAME from its escrow
    /// (78.42 (3), 65.5): the energy a transition carries leaves the escrow,
    /// per lane, exactly. `None` where no escrow stands; `Err` with the
    /// shortfall per lane where the escrow cannot pay the draw — the word is
    /// exhausted on those lanes, and nothing is drawn.
    pub fn carry_escrow(&mut self, addr: &[u8], draw: [u64; 4]) -> Option<Result<[u64; 4], [u64; 4]>> {
        let row = self.escrowed.get_mut(addr)?;
        let short = super::shortfall(&draw, &row.mu);
        if !super::lanes_short(&short).is_empty() {
            return Some(Err(short));
        }
        row.mu = std::array::from_fn(|l| row.mu[l] - draw[l]);
        Some(Ok(row.mu))
    }

    /// PRESSURE ARRIVING: a frame transported to a provisional word (a
    /// sentence that cites it) is credited to its escrow, per lane. `None`
    /// where no escrow stands or a lane would pass its carrier (STD1).
    pub fn credit_escrow(&mut self, addr: &[u8], mu: [u64; 4]) -> Option<[u64; 4]> {
        let row = self.escrowed.get_mut(addr)?;
        row.mu = crate::core::lanes::lanes_add(row.mu, mu)?;
        Some(row.mu)
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
