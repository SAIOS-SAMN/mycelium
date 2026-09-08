// E1 — THE TREASURY: who is bonded, what is staked, what has been minted,
// and the audit that holds across all of it.
//
//! Every entry is a movement with an anchor; the balance is derived from
//! the entries and never stored beside them. `audit` re-derives the supply
//! from the ledger and refuses if the two disagree.


/// The chronometric anchor of a ledger entry: (K-step, chain head hash).
pub type Anchor = (u64, [u8; 32]);

/// Economic tiers for a decentralized mesh running cryptographic semantic
/// operations: yields scale by the participant's tier. A hypha/client's standing
/// weights its reward, so the topology supports staked, tiered roles — never a
/// A settlement receipt (below-boundary storage — a Vec, not a map): a
/// polytonic μ⃗ amount, chronometrically anchored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub kind: &'static str,
    pub who: [u8; 32],
    /// The per-lane vector amount — never a scalar.
    pub mu: [u64; 4],
    /// K-step at booking.
    pub k_step: u64,
    /// The chain head hash at booking (the unforgeable temporal binding).
    pub receipt: [u8; 32],
}

/// Default operator bond when none is chosen — one unit on every lane.
pub const UNIT_BOND: [u64; 4] = [1, 1, 1, 1];

#[derive(Debug, Default, Clone)]
pub struct Treasury {
    pub(crate) entries: Vec<Entry>,
    /// Sum of live identity bonds. Escrowed awards are `escrowed`, never this.
    pub(crate) bond_total: [u64; 4],
    /// Enrolled identities and their staked μ⃗, keyed by identity.
    pub(crate) stakes: crate::network::nerve::Keyed<[u64; 4]>,
    /// AWARDS IN ESCROW (Step 5): consilience settlements held unfinalized —
    /// keyed by germ address. Only the court's verdict releases or revokes.
    pub(crate) escrowed: crate::network::nerve::Keyed<super::escrow::Escrowed>,
    /// Per-voice balances, updated on every entry. Never a scan of the ledger.
    pub(crate) balances: crate::network::nerve::Keyed<[u64; 4]>,
    /// FUNDED escrow, per listing: the part of an award that was debited
    /// from a balance to create it, as against the part that is a claim on
    /// issuance which does not exist yet (`MU-ECONOMY.md` §3.1).
    ///
    /// The two are different facts and the invariant needs them apart.
    /// R9's `Σ escrow` is the funded part: μ⃗ that left a balance is still
    /// in the mesh and must be counted, or a burn would not conserve. A
    /// claim-escrow contributes nothing, because nobody gave anything up
    /// for it and a ruling MINTS it rather than moving it. Removing escrow
    /// from the equation outright, as this briefly did, makes the audit
    /// hold by not looking; counting all of it says value exists that does
    /// not. Both are wrong and the difference is exactly what R9 turns on.
    pub(crate) funded: crate::network::nerve::Keyed<([u8; 32], [u64; 4])>,
}

impl Treasury {
    pub fn new() -> Self {
        Treasury::default()
    }

    pub(crate) fn credit(&mut self, who: [u8; 32], mu: [u64; 4]) {
        let mut b = self.balances.get(&who).copied().unwrap_or([0; 4]);
        for l in 0..4 {
            b[l] = b[l].saturating_add(mu[l]);
        }
        self.balances.insert(who, b);
    }

    pub(crate) fn debit(&mut self, who: [u8; 32], mu: [u64; 4]) {
        let mut b = self.balances.get(&who).copied().unwrap_or([0; 4]);
        for l in 0..4 {
            b[l] = b[l].saturating_sub(mu[l]);
        }
        self.balances.insert(who, b);
    }

    /// ENROLL an identity with an operator-chosen per-lane bond. Tops up
    /// when the recorded stake is below `bond`. Pool is the sum of bonds;
    /// leftover escrow history does not enter it. Returns the vector added
    /// this call.
    pub fn enroll(&mut self, who: [u8; 32], bond: [u64; 4], anchor: Anchor) -> [u64; 4] {
        let held = self.stakes.get(&who).copied().unwrap_or([0; 4]);
        let asked: [u64; 4] = std::array::from_fn(|l| bond[l].saturating_sub(held[l]));
        // BOND IS μ AT RISK, STAKED FROM A BALANCE (§4). This inserted the
        // stake and wrote a "stake" row without debiting anything, so bond
        // existed that was never issued and §6's second equation --
        // Σ balance + Σ bond + Σ escrow = genesis + mint − burn -- failed on
        // the first enrolment. A stake MOVES value between two carriers of
        // one identity; it never creates any. What the balance cannot cover
        // is not staked, so an identity stakes what it holds and no more.
        let bal = self.balance_mu(&who);
        let add: [u64; 4] = std::array::from_fn(|l| asked[l].min(bal[l]));
        if add.iter().any(|&x| x > 0) {
            self.debit(who, add);
            let next: [u64; 4] = std::array::from_fn(|l| held[l] + add[l]);
            self.stakes.insert(who, next);
            self.entries.push(Entry { kind: "stake", who, mu: add, k_step: anchor.0, receipt: anchor.1 });
        } else if self.stakes.get(&who).is_none() {
            self.stakes.insert(who, held);
        }
        self.refresh_bond();
        add
    }

    /// The inverse: bond returns to the balance it was staked from (§9's
    /// unstake). Never more than is bonded, and an identity with escrow
    /// naming it is not the treasury's business to check here -- the caller
    /// holds that rule (MU-PRODUCT §5: unstake when no escrow names the
    /// voice). Conservation holds because the μ⃗ moves between two carriers
    /// of one identity, exactly as staking it did.
    pub fn unstake(&mut self, who: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        let held = self.stakes.get(&who).copied().unwrap_or([0; 4]);
        let back: [u64; 4] = std::array::from_fn(|l| mu[l].min(held[l]));
        if back.iter().all(|&x| x == 0) {
            return back;
        }
        let next: [u64; 4] = std::array::from_fn(|l| held[l] - back[l]);
        self.stakes.insert(who, next);
        self.credit(who, back);
        self.entries.push(Entry { kind: "unstake", who, mu: back, k_step: anchor.0, receipt: anchor.1 });
        self.refresh_bond();
        back
    }

    pub(crate) fn refresh_bond(&mut self) {
        let mut e = [0u64; 4];
        crate::snapshot::persist(&self.stakes, |_, mu| {
            for l in 0..4 {
                e[l] = e[l].saturating_add(mu[l]);
            }
        });
        self.bond_total = e;
    }

    /// Bond recorded for an identity, if enrolled. None is unbonded.
    pub fn bond_of(&self, who: &[u8; 32]) -> Option<[u64; 4]> {
        self.stakes.get(who).copied()
    }

    /// Bond recorded for an identity, if enrolled.
    pub fn stake_of(&self, who: &[u8; 32]) -> Option<[u64; 4]> {
        self.bond_of(who)
    }

    /// Sum of live stakes. Legacy entries are excluded.
    pub fn bond_total(&self) -> [u64; 4] {
        self.bond_total
    }

    /// Stakes whose identity is not in the live genesis become `legacy`
    /// entries and leave `stakes` / `bond_total`.
    pub fn bind_live(&mut self, live: &[[u8; 32]]) {
        let mut drop: Vec<([u8; 32], [u64; 4])> = Vec::new();
        crate::snapshot::persist(&self.stakes, |who, mu| {
            let Ok(w) = <[u8; 32]>::try_from(who) else { return };
            if !live.contains(&w) {
                drop.push((w, *mu));
            }
        });
        for (who, mu) in drop {
            self.stakes.remove(&who);
            self.entries.push(Entry {
                kind: "legacy",
                who,
                mu,
                k_step: 0,
                receipt: [0; 32],
            });
        }
        self.refresh_bond();
    }

    /// `supply = genesis + mint − burn`, per lane (MU-ECONOMY §6). Genesis
    /// is a finite signed allocation recorded as its own kind, so seeded
    /// value and earned value are always separable in the ledger; burn
    /// leaves circulation and reduces supply by the same vector, so the
    /// equation holds through a forfeiture (47.4, R9).
    ///
    /// `None` when a lane overflows or a burn exceeds what was issued on it:
    /// a supply that does not add up is not a smaller supply, it is a
    /// ledger that does not describe the mesh (rule 11). Summing mint alone
    /// with `saturating_add`, as this did, could report neither.
    pub fn supply(&self) -> Option<[u64; 4]> {
        let mut issued = [0u64; 4];
        let mut burned = [0u64; 4];
        for e in &self.entries {
            // A SLASH IS μ⃗ LEAVING CIRCULATION. It debits a bond (§5) and
            // §2's carriers -- balance, bond, escrow -- hold nothing that
            // could receive it, so it is issuance returned exactly as a
            // burn is; 47.4's forfeiture says the same of the whole bond.
            // Counted nowhere, `Σ bond` fell while `supply` held, and the
            // audit failed on every lane anyone had ever been slashed on.
            // It stays its OWN KIND so the ledger can still answer "was
            // anyone slashed?" separately from "what was burned?".
            let into = match e.kind {
                "mint" | "genesis" => &mut issued,
                "burn" | "slash" => &mut burned,
                _ => continue,
            };
            for (t, d) in into.iter_mut().zip(&e.mu) {
                *t = t.checked_add(*d)?;
            }
        }
        let mut s = [0u64; 4];
        for ((n, i), b) in s.iter_mut().zip(&issued).zip(&burned) {
            *n = i.checked_sub(*b)?;
        }
        Some(s)
    }

    /// EARNED supply alone: the mint rows, without the genesis allocations
    /// §6 keeps separable from them. "Nothing was minted" is a claim about
    /// verified work and must not be read off `supply()`, which includes
    /// seeded value by construction — a testnet differs from the beta only
    /// in that term.
    pub fn minted(&self) -> Option<[u64; 4]> {
        let mut m = [0u64; 4];
        for e in &self.entries {
            if e.kind != "mint" {
                continue;
            }
            for (t, d) in m.iter_mut().zip(&e.mu) {
                *t = t.checked_add(*d)?;
            }
        }
        Some(m)
    }

    /// A finite, signed allocation at genesis. Its own kind, never a mint:
    /// §6 requires seeded value and earned value to stay separable, and a
    /// testnet differs from the beta only in this term.
    pub fn genesis(&mut self, who: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        if mu.iter().all(|&x| x == 0) {
            return [0; 4];
        }
        self.entries.push(Entry { kind: "genesis", who, mu, k_step: anchor.0, receipt: anchor.1 });
        self.credit(who, mu);
        mu
    }

    /// Forfeiture: μ⃗ leaves circulation (47.4). Bond first, then balance,
    /// on each lane; supply falls by exactly what left, so §6 holds through
    /// it (R9). Returns the vector actually burned — never more than was
    /// held, and nothing is redistributed.
    pub fn burn(&mut self, who: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        let bond = self.stakes.get(&who).copied().unwrap_or([0; 4]);
        let bal = self.balance_mu(&who);
        let from_bond: [u64; 4] = std::array::from_fn(|l| mu[l].min(bond[l]));
        let from_bal: [u64; 4] =
            std::array::from_fn(|l| (mu[l] - from_bond[l]).min(bal[l]));
        let took: [u64; 4] = std::array::from_fn(|l| from_bond[l] + from_bal[l]);
        if took.iter().all(|&x| x == 0) {
            return took;
        }
        if from_bond.iter().any(|&x| x > 0) {
            let next: [u64; 4] = std::array::from_fn(|l| bond[l] - from_bond[l]);
            self.stakes.insert(who, next);
            self.refresh_bond();
        }
        if from_bal.iter().any(|&x| x > 0) {
            self.debit(who, from_bal);
        }
        self.entries.push(Entry { kind: "burn", who, mu: took, k_step: anchor.0, receipt: anchor.1 });
        took
    }

    /// THE INVARIANT (§6), both equations, on every lane:
    ///
    /// ```text
    /// supply = genesis + mint − burn
    /// supply = Σ balance + Σ bond + Σ escrow
    /// ```
    ///
    /// The first is read off the journal, the second off the carriers, and
    /// the fold of LEDGER_EPOCH_DESIGN is admissible only where this holds
    /// at the prefix it folds — a compression whose conservation cannot be
    /// checked is not a compression, it is a loss.
    pub fn audit(&self) -> Result<[u64; 4], String> {
        let issued = self
            .supply()
            .ok_or_else(|| "supply does not add up on some lane".to_string())?;
        let mut held = [0u64; 4];
        let add = |v: &[u64; 4], held: &mut [u64; 4]| -> Result<(), String> {
            for l in 0..4 {
                held[l] = held[l]
                    .checked_add(v[l])
                    .ok_or_else(|| format!("carriers overflow lane {l}"))?;
            }
            Ok(())
        };
        // R9's equation, with `Σ escrow` meaning the FUNDED escrow. An
        // escrow that was paid for holds μ⃗ that left a balance and is still
        // in the mesh; an escrow that is a claim on issuance holds μ⃗ that
        // does not exist yet (§3.1), and a ruling mints it rather than
        // moving it. Counting all escrow says value exists that does not,
        // and counting none makes the audit hold by not looking.
        let mut sums: Vec<[u64; 4]> = Vec::new();
        crate::snapshot::persist(&self.balances, |_, mu| sums.push(*mu));
        crate::snapshot::persist(&self.stakes, |_, mu| sums.push(*mu));
        crate::snapshot::persist(&self.funded, |_, (_, mu)| sums.push(*mu));
        for v in &sums {
            add(v, &mut held)?;
        }
        if held != issued {
            return Err(format!(
                "supply {issued:?} is not the carriers {held:?}: balances + bond + funded escrow must be what was issued"
            ));
        }
        Ok(issued)
    }

    pub(crate) fn mint_to(&mut self, who: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        if mu.iter().all(|&x| x == 0) {
            return [0; 4];
        }
        self.entries.push(Entry { kind: "mint", who, mu, k_step: anchor.0, receipt: anchor.1 });
        self.credit(who, mu);
        eprintln!(
            "  mint {:02x}{:02x}… μ⃗ {mu:?} k {}",
            who[0], who[1], anchor.0
        );
        mu
    }

    /// The POLYTONIC BALANCE — per-lane orientation-weighted sums; there is no
    /// scalar balance. Mint grows supply; the stake bond is independent.
    pub fn balance_mu(&self, who: &[u8; 32]) -> [u64; 4] {
        self.balances.get(who).copied().unwrap_or([0; 4])
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Whether a mint entry already stands for this identity.
    pub fn has_mint(&self, who: &[u8; 32]) -> bool {
        self.entries.iter().any(|e| e.kind == "mint" && e.who == *who)
    }

}
