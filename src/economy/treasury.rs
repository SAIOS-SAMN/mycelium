// E1 — THE TREASURY: who is bonded, what is staked, what has been minted,
// and the audit that holds across all of it.
//
//! Every entry is a movement with an anchor; the balance is derived from
//! the entries and never stored beside them. `audit` re-derives the supply
//! from the ledger and refuses if the two disagree.


/// THE ACT A LEDGER ROW STANDS AT (78.24): the stamp of the court's head
/// receipt when the row was written, and that receipt's chain hash — the
/// row named by its act's whole coordinate, never by a `K` alone. Before a
/// court's genesis its head is the empty stamp under its own key and the
/// empty hash: the reading of absence.
pub type Anchor = (crate::chronometry::Stamp, [u8; 32]);

/// THE PLACE OF THE ACT AN ANCHOR STANDS AT on the court's own line: the
/// row was written after that act and before the next, which is what
/// orders rows against a fold's prefix. `None` before genesis, where the
/// anchor stands at no act (the empty hash) — below every place.
pub fn stands_at(anchor: &Anchor) -> Option<crate::chronometry::Place> {
    (anchor.1 != [0u8; 32]).then(|| anchor.0.place())
}

/// Economic tiers for a decentralized mesh running cryptographic semantic
/// operations: yields scale by the participant's tier. A hypha/client's standing
/// weights its reward, so the topology supports staked, tiered roles — never a
/// §2's ENTRY KINDS, the closed vocabulary of what moves μ⃗: "Entry kinds:
/// `genesis`, `mint`, `stake`, `unstake`, `transfer`, `escrow`, `release`,
/// `revoke`, `slash`, `burn`, and `epoch`. Nothing else moves μ." An
/// `epoch` row (45.8, S13.8) is the net an identity's rows moved over a
/// folded prefix: it stands on the issued side of the supply in place of
/// the genesis, mint, burn and slash rows it replaced, and moves none.
///
/// It was a `&'static str` matched by literal. B5 made that load-bearing:
/// `write` folds the supply on `"mint" | "genesis" => issued,
/// "burn" | "slash" => burned, _ => continue`, so a row whose kind did not
/// spell one of five strings was silently dropped from the supply, and the
/// audit then reported carriers that did not match a journal it had not
/// read whole. A typo in a kind is a ledger that does not describe the
/// mesh, and nothing would say so. The chapter fixes a list; the type is
/// that list, and every match on it is exhaustive. A stake dropped for a
/// key outside the live genesis is a `Burn` (§4, §6); the `Legacy` kind
/// that once marked it moved no supply and failed the audit by exactly its
/// vector (I38).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Genesis,
    Mint,
    Stake,
    Unstake,
    Transfer,
    Escrow,
    Release,
    Revoke,
    Slash,
    Burn,
    Epoch,
}

impl Kind {
    /// Which side of `supply = genesis + mint − burn` this row falls on.
    /// Exhaustive: a kind that moved the supply and was not named here
    /// would not compile. An `epoch` row stands for the net of the issued
    /// and burned rows it replaced, so it reads on the issued side.
    pub(crate) fn issues(&self) -> Option<bool> {
        match self {
            Kind::Genesis | Kind::Mint | Kind::Epoch => Some(true),
            Kind::Burn | Kind::Slash => Some(false),
            Kind::Stake
            | Kind::Unstake
            | Kind::Transfer
            | Kind::Escrow
            | Kind::Release
            | Kind::Revoke => None,
        }
    }
}

/// A settlement receipt (below-boundary storage — a Vec, not a map): a
/// polytonic μ⃗ amount, chronometrically anchored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub kind: Kind,
    pub who: [u8; 32],
    /// The per-lane vector amount — never a scalar.
    pub mu: [u64; 4],
    /// The stamp of the act the row stands at (78.24).
    pub at: crate::chronometry::Stamp,
    /// That act's chain hash (the unforgeable temporal binding).
    pub receipt: [u8; 32],
    /// The proof of work this row pays (78.33), or none.
    pub arm: Option<Arm>,
}

impl Entry {
    /// The place of the act this row stands at (see [`stands_at`]).
    pub fn stands_at(&self) -> Option<crate::chronometry::Place> {
        stands_at(&(self.at, self.receipt))
    }
}

/// 78.33 — THE PROOF OF WORK A PAYMENT ROW CARRIES, named apart and never
/// summed across: POUW, a closure ruled and its deflation minted to the
/// closer; POWC, a cited generator paid by the use that cited it; POWR, a
/// retirement's compression yield paid to the retirer. A row that moves
/// no proof of work — genesis, a stake, an escrow, a revoke or slash, a
/// burn, a fee, a subsidy — carries none.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arm {
    Pouw,
    Powc,
    Powr,
}

impl Arm {
    /// Every arm, in the order the readings are carried.
    pub const ALL: [Arm; 3] = [Arm::Pouw, Arm::Powc, Arm::Powr];

    /// The arm as its coordinate on the row: `1..=3`, `0` for none.
    pub fn to_byte(arm: Option<Arm>) -> u8 {
        match arm {
            None => 0,
            Some(Arm::Pouw) => 1,
            Some(Arm::Powc) => 2,
            Some(Arm::Powr) => 3,
        }
    }

    /// The arm a byte names; a byte naming none refuses the row.
    pub fn from_byte(b: u8) -> Option<Option<Arm>> {
        Some(match b {
            0 => None,
            1 => Some(Arm::Pouw),
            2 => Some(Arm::Powc),
            3 => Some(Arm::Powr),
            _ => return None,
        })
    }

    /// Where the arm is carried in a per-arm reading.
    pub fn index(self) -> usize {
        match self {
            Arm::Pouw => 0,
            Arm::Powc => 1,
            Arm::Powr => 2,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Arm::Pouw => "POUW",
            Arm::Powc => "POWC",
            Arm::Powr => "POWR",
        }
    }
}

/// Default operator bond when none is chosen — one unit on every lane.
pub const UNIT_BOND: [u64; 4] = [1, 1, 1, 1];

#[derive(Debug, Clone)]
pub struct Treasury {
    pub(crate) entries: Vec<Entry>,
    /// THE SUPPLY, FOLDED AS IT IS WRITTEN. `supply()` read the whole
    /// journal on every call, and `audit()` calls it, and §6 requires the
    /// audit before every snapshot — so the cost of proving the ledger
    /// conserves grew with the ledger, on a carrier that grows for as long
    /// as the mesh runs (rule 10: a loop whose bound is neither a `Budget`
    /// nor one simplex's boundary is unbounded work in disguise).
    ///
    /// `None` once a fold overflowed, exactly as the whole read returned `None`
    /// from `checked_add`: a supply that does not add up is not a smaller
    /// supply, it is unmeasured (rule 11). `restore` folds the journal once
    /// at load, which is the one O(journal) pass the doctrine sanctions.
    pub(crate) issued: Option<[u64; 4]>,
    pub(crate) burned: Option<[u64; 4]>,
    /// Sum of live identity bonds. Escrowed awards are `escrowed`, never this.
    pub(crate) bond_total: [u64; 4],
    /// Enrolled identities and their staked μ⃗, keyed by identity.
    pub(crate) stakes: crate::network::nerve::Keyed<[u64; 4]>,
    /// AWARDS IN ESCROW (Step 5): consilience settlements held unfinalized —
    /// keyed by germ address. Only the court's verdict releases or revokes.
    pub(crate) escrowed: crate::network::nerve::Keyed<super::escrow::Escrowed>,
    /// Per-voice balances, updated on every entry. Never a scan of the ledger.
    pub(crate) balances: crate::network::nerve::Keyed<[u64; 4]>,
}

/// Written out rather than derived: `#[derive(Default)]` gives the folded
/// supply `None`, and a fresh ledger has issued and burned NOTHING, which
/// is a measurement of zero and not an absence of one. Derived, every
/// treasury that was not built by `new()` reported its supply unmeasured
/// and the audit refused a ledger that added up perfectly.
impl Default for Treasury {
    fn default() -> Self {
        Treasury {
            entries: Vec::new(),
            issued: Some([0; 4]),
            burned: Some([0; 4]),
            bond_total: [0; 4],
            stakes: crate::network::nerve::Keyed::new(),
            escrowed: crate::network::nerve::Keyed::new(),
            balances: crate::network::nerve::Keyed::new(),
        }
    }
}

impl Treasury {
    pub fn new() -> Self {
        Treasury::default()
    }

    pub(crate) fn credit(&mut self, who: [u8; 32], mu: [u64; 4]) {
        let mut b = self.balance_of(&who);
        for l in 0..4 {
            b[l] = b[l].saturating_add(mu[l]);
        }
        self.balances.insert(who, b);
    }

    /// THE SHORTFALL OF THE MOVEMENT, per lane: zero where the lane moved,
    /// and what the balance lacked where it did not. Nothing moves when any
    /// lane is short — the movement is whole or it is refused — and the
    /// caller is handed the lanes rather than a bit it would have to
    /// recompute (§2.3, law 5).
    ///
    /// Every caller clamps to the balance before asking, so a non-zero
    /// reading is a broken clamp; it is returned rather than floored
    /// because a floor burns the difference against §6's equation.
    #[must_use]
    pub(crate) fn debit(&mut self, who: [u8; 32], mu: [u64; 4]) -> [u64; 4] {
        let b = self.balance_of(&who);
        let short = crate::economy::shortfall(&mu, &b);
        if !crate::economy::lanes_short(&short).is_empty() {
            return short;
        }
        self.balances.insert(who, std::array::from_fn(|l| b[l] - mu[l]));
        short
    }

    /// ENROLL an identity with an operator-chosen per-lane bond. Tops up
    /// when the recorded stake is below `bond`. Pool is the sum of bonds;
    /// leftover escrow history does not enter it. Returns the vector added
    /// this call.
    pub fn enroll(&mut self, who: [u8; 32], bond: [u64; 4], anchor: Anchor) -> [u64; 4] {
        let held = self.staked(&who);
        // The top-up IS the shortfall of what is staked against the bond,
        // and that reading already exists: zero where the lane meets it.
        let asked = crate::economy::shortfall(&bond, &held);
        // BOND IS μ AT RISK, STAKED FROM A BALANCE (§4). This inserted the
        // stake and wrote a "stake" row without debiting anything, so bond
        // existed that was never issued and §6's second equation --
        // Σ balance + Σ bond + Σ escrow = genesis + mint − burn -- failed on
        // the first enrolment. A stake MOVES value between two carriers of
        // one identity; it never creates any. What the balance cannot cover
        // is not staked, so an identity stakes what it holds and no more.
        let bal = self.balance_mu(&who);
        let add: [u64; 4] = std::array::from_fn(|l| asked[l].min(bal[l]));
        // `add` is clamped to the balance above; a debit that does not go
        // through stakes nothing rather than creating bond nothing issued.
        if add.iter().any(|&x| x > 0)
            && crate::economy::lanes_short(&self.debit(who, add)).is_empty()
        {
            let next: [u64; 4] = std::array::from_fn(|l| held[l] + add[l]);
            self.stakes.insert(who, next);
            self.write(Entry { kind: Kind::Stake, who, mu: add, at: anchor.0, receipt: anchor.1, arm: None });
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
        let held = self.staked(&who);
        let back: [u64; 4] = std::array::from_fn(|l| mu[l].min(held[l]));
        if back.iter().all(|&x| x == 0) {
            return back;
        }
        let next: [u64; 4] = std::array::from_fn(|l| held[l] - back[l]);
        self.stakes.insert(who, next);
        self.credit(who, back);
        self.write(Entry { kind: Kind::Unstake, who, mu: back, at: anchor.0, receipt: anchor.1, arm: None });
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

    /// Sum of live stakes.
    pub fn bond_total(&self) -> [u64; 4] {
        self.bond_total
    }

    /// A STAKE OUTSIDE THE LIVE GENESIS IS BURNED (§4: bond is μ⃗ at risk;
    /// §6: every row moves the supply or moves nothing, and a stake that
    /// leaves the carriers under a row that moves no supply breaks the
    /// audit by exactly its vector). Each dropped stake leaves as one `Burn`
    /// anchored to the caller's position, the identity leaves `stakes`, and
    /// what left is returned so the caller can say so. The `Legacy` row this
    /// wrote, at K 0 under a zero receipt, failed `audit` on every live
    /// hypha of the 2026-09-08 soak (I38).
    pub fn bind_live(&mut self, live: &[[u8; 32]], anchor: Anchor) -> Vec<([u8; 32], [u64; 4])> {
        let mut drop: Vec<([u8; 32], [u64; 4])> = Vec::new();
        crate::snapshot::persist(&self.stakes, |who, mu| {
            let Ok(w) = <[u8; 32]>::try_from(who) else { return };
            if !live.contains(&w) {
                drop.push((w, *mu));
            }
        });
        let mut burned = Vec::with_capacity(drop.len());
        for (who, mu) in drop {
            let took = self.burn(who, mu, anchor);
            self.stakes.remove(&who);
            burned.push((who, took));
        }
        self.refresh_bond();
        burned
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
        let (issued, burned) = (self.issued?, self.burned?);
        let mut s = [0u64; 4];
        for ((n, i), b) in s.iter_mut().zip(&issued).zip(&burned) {
            *n = i.checked_sub(*b)?;
        }
        Some(s)
    }

    /// EVERY LEDGER ROW GOES THROUGH HERE, and the two supply sums are
    /// folded as it is written. Nothing else pushes an `Entry`: a row that
    /// bypassed this would leave the folded supply describing a ledger that
    /// is not the one on disk.
    ///
    /// A SLASH IS μ⃗ LEAVING CIRCULATION. It debits a bond (§5) and §2's
    /// carriers -- balance, bond, escrow -- hold nothing that could receive
    /// it, so it is issuance returned exactly as a burn is; 47.4's
    /// forfeiture says the same of the whole bond. Counted nowhere,
    /// `Σ bond` fell while `supply` held, and the audit failed on every
    /// lane anyone had ever been slashed on. It stays its OWN KIND so the
    /// ledger can still answer "was anyone slashed?" apart from "what was
    /// burned?".
    pub(crate) fn write(&mut self, e: Entry) {
        let into = match e.kind.issues() {
            Some(true) => &mut self.issued,
            Some(false) => &mut self.burned,
            None => {
                self.entries.push(e);
                return;
            }
        };
        // An overflow leaves the fold UNMEASURED and it stays unmeasured:
        // the whole read this replaces returned `None` from `checked_add` on the
        // same arithmetic (rule 11).
        *into = into.and_then(|mut t| {
            for (a, d) in t.iter_mut().zip(&e.mu) {
                *a = a.checked_add(*d)?;
            }
            Some(t)
        });
        self.entries.push(e);
    }

    /// Fold the supply from the journal in one pass. `restore` reads the
    /// rows off disk directly, so this is where the two sums come back —
    /// the one O(journal) pass the doctrine sanctions, at load.
    pub(crate) fn refold_supply(&mut self) {
        let (mut issued, mut burned) = (Some([0u64; 4]), Some([0u64; 4]));
        for e in &self.entries {
            let into = match e.kind.issues() {
                Some(true) => &mut issued,
                Some(false) => &mut burned,
                None => continue,
            };
            *into = into.and_then(|mut t| {
                for (a, d) in t.iter_mut().zip(&e.mu) {
                    *a = a.checked_add(*d)?;
                }
                Some(t)
            });
        }
        self.issued = issued;
        self.burned = burned;
    }

    /// EARNED supply alone: the mint rows, without the genesis allocations
    /// §6 keeps separable from them. "Nothing was minted" is a claim about
    /// verified work and must not be read off `supply()`, which includes
    /// seeded value by construction — a testnet differs from the beta only
    /// in that term.
    pub fn minted(&self) -> Option<[u64; 4]> {
        let mut m = [0u64; 4];
        for e in &self.entries {
            if e.kind != Kind::Mint {
                continue;
            }
            for (t, d) in m.iter_mut().zip(&e.mu) {
                *t = t.checked_add(*d)?;
            }
        }
        Some(m)
    }

    /// LD3-a — A LINE REBOUND BEFORE ITS FIRST ACT MOVES ITS ORIGIN: the
    /// rows and the escrow anchors standing at the old pre-genesis origin
    /// (the empty hash) stand at the new one. Nothing else moves: a row at
    /// an act stands at that act.
    pub fn rebind_origin(&mut self, from: crate::chronometry::Stamp, to: crate::chronometry::Stamp) {
        for e in self.entries.iter_mut() {
            if e.at == from && e.receipt == [0; 32] {
                e.at = to;
            }
        }
        let held: Vec<Vec<u8>> = self.escrowed.scoped().map(|(a, _)| a).collect();
        for addr in held {
            if let Some(row) = self.escrowed.get_mut(&addr) {
                if row.anchor == (from, [0; 32]) {
                    row.anchor.0 = to;
                }
            }
        }
    }

    /// EC1, 78.33 — WHAT EACH PROOF OF WORK PAID AND BURNED, per lane, read
    /// off the journal at the question and never stored beside it (law 12).
    /// A mint of an arm is paid; a burn of an arm is burned. A POWC use is
    /// ONE transfer written as TWO equal rows, its debit and its credit, so
    /// the arm's transfer rows read in pairs and the paid is one of each pair
    /// — exact, since every pair is two equal vectors. `None` where a lane
    /// passes its u64: unmeasured, never saturated (rule 11).
    pub fn economy(&self) -> Option<crate::observe::Economy> {
        let mut e = crate::observe::Economy::default();
        let mut moved = [[0u64; 4]; 3];
        for row in &self.entries {
            let Some(arm) = row.arm else { continue };
            let into = match row.kind {
                Kind::Mint => &mut e.paid[arm.index()],
                Kind::Burn => &mut e.burned[arm.index()],
                Kind::Transfer => &mut moved[arm.index()],
                _ => continue,
            };
            for (t, m) in into.iter_mut().zip(&row.mu) {
                *t = t.checked_add(*m)?;
            }
        }
        for (paid, pairs) in e.paid.iter_mut().zip(&moved) {
            for (t, m) in paid.iter_mut().zip(pairs) {
                *t = t.checked_add(m / 2)?;
            }
        }
        Some(e)
    }

    /// A finite, signed allocation at genesis. Its own kind, never a mint:
    /// §6 requires seeded value and earned value to stay separable, and a
    /// testnet differs from the beta only in this term.
    pub fn genesis(&mut self, who: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        if mu.iter().all(|&x| x == 0) {
            return [0; 4];
        }
        self.write(Entry { kind: Kind::Genesis, who, mu, at: anchor.0, receipt: anchor.1, arm: None });
        self.credit(who, mu);
        mu
    }

    /// Forfeiture: μ⃗ leaves circulation (47.4). Bond first, then balance,
    /// on each lane; supply falls by exactly what left, so §6 holds through
    /// it (R9). Returns the vector actually burned — never more than was
    /// held, and nothing is redistributed.
    pub fn burn(&mut self, who: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        let bond = self.staked(&who);
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
        // `from_bal` is clamped to the balance above; a debit that does not
        // go through takes nothing from it.
        if from_bal.iter().any(|&x| x > 0)
            && !crate::economy::lanes_short(&self.debit(who, from_bal)).is_empty()
        {
            return [0; 4];
        }
        self.write(Entry { kind: Kind::Burn, who, mu: took, at: anchor.0, receipt: anchor.1, arm: None });
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
        // R9's equation. Every escrow is a claim on issuance (§3.1): it
        // holds μ⃗ that does not exist yet, and a ruling mints it rather
        // than moving it, so no escrow is a carrier of supply.
        let mut sums: Vec<[u64; 4]> = Vec::new();
        crate::snapshot::persist(&self.balances, |_, mu| sums.push(*mu));
        crate::snapshot::persist(&self.stakes, |_, mu| sums.push(*mu));
        for v in &sums {
            add(v, &mut held)?;
        }
        if held != issued {
            return Err(format!(
                "supply {issued:?} is not the carriers {held:?}: balances + bond must be what was issued"
            ));
        }
        Ok(issued)
    }

    /// A MINT to `who`, standing at the act that minted it and carrying the
    /// proof of work it pays (78.33) — none for a subsidy.
    pub(crate) fn mint_to(&mut self, who: [u8; 32], mu: [u64; 4], anchor: Anchor, arm: Option<Arm>) -> [u64; 4] {
        if mu.iter().all(|&x| x == 0) {
            return [0; 4];
        }
        self.write(Entry { kind: Kind::Mint, who, mu, at: anchor.0, receipt: anchor.1, arm });
        self.credit(who, mu);
        eprintln!(
            "  mint {:02x}{:02x}… μ⃗ {mu:?} at K {} h {}",
            who[0], who[1], anchor.0.k, anchor.0.h
        );
        mu
    }

    /// The POLYTONIC BALANCE — per-lane orientation-weighted sums; there is no
    /// scalar balance. Mint grows supply; the stake bond is independent.
    pub fn balance_mu(&self, who: &[u8; 32]) -> [u64; 4] {
        self.balance_of(who)
    }

    /// THE READING OF ABSENCE (0.11c): a party the ledger has never seen
    /// holds nothing on every lane, and that zero is a measurement of the
    /// carrier rather than a value put in place of one. Every balance,
    /// stake and escrow read passes through here.
    fn balance_of(&self, who: &[u8; 32]) -> [u64; 4] {
        match self.balances.get(who) {
            Some(b) => *b,
            None => [0; 4],
        }
    }

    /// The stake this party holds, or nothing where the ledger has not
    /// seen it — the same reading of absence. `stake_of` above answers
    /// WHETHER there is a bond; this answers HOW MUCH, and an unseen party
    /// holds zero on every lane.
    fn staked(&self, who: &[u8; 32]) -> [u64; 4] {
        match self.stakes.get(who) {
            Some(s) => *s,
            None => [0; 4],
        }
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Whether a mint entry already stands for this identity.
    pub fn has_mint(&self, who: &[u8; 32]) -> bool {
        self.entries.iter().any(|e| e.kind == Kind::Mint && e.who == *who)
    }

}
