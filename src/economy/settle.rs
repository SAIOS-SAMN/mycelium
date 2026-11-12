// E2, 26 — SETTLEMENT: what each closed act pays, per lane.
//
//! One function per act the market closes. Each is a movement of value
//! that already exists or a mint against a measured deflation; none of them
//! prices anything — the metabolism governs the rate, never the price.

use super::*;

impl Treasury {
    /// A germ install admits iff the germ measures something on some lane;
    /// nothing is ordered and no direction is read (12.6, law 5). Installing pays
    /// NOTHING here — a germ that only closes its own birth demand has proven
    /// nothing about the corpus. Reward flows later, on USE (`settle_use`),
    /// proportional to real demand unlocked.
    pub fn settle_install(&mut self, g: &Element, _voice: [u8; 32]) -> bool {
        // The germ MEASURES something on some lane. This was written as a
        // lane reading against the zero vector, which is a nonzero test wearing a
        // lane comparison's clothes.
        magnitude(g).iter().any(|&m| m > 0)
    }

    /// POUW — a word was USED: a real later demand decomposed through it. THE
    /// CIRCULATION LAW: the DEMANDER pays the word's DISCOVERER the per-lane
    /// structure exercised, from the demander's EARNED balance — a peer transfer,
    /// bounded by what the payer really holds. Knowledge is nonrival: a broke
    /// demander is still served; the discoverer earns what was real. A demander
    /// paying itself transfers nothing (no wash trades). Returns the paid μ⃗.
    /// Two `transfer` entries share the anchor: first debits the demander,
    /// second credits the discoverer. Neither is a mint.
    pub fn settle_use(&mut self, demander: [u8; 32], discoverer: [u8; 32], mag: impl Into<Mu>, anchor: Anchor) -> Mu {
        if demander == discoverer {
            return Mu::zero();
        }
        // paid ≤ held on every lane: it is the ask divided at the balance
        // just read, and it is debited against that same balance below.
        let (paid, _) = self.balance_mu(&demander).split_at(&mag.into());
        if paid.is_zero() {
            return paid;
        }
        // The rows are written for μ⃗ that MOVED, after the debit — which
        // cannot be short, by `paid ≤ held`.
        let Ok(moved) = self.debit(demander, &paid) else { return Mu::zero() };
        self.credit(discoverer, &moved);
        // POWC (78.33): a cited generator paid by the use that cited it.
        let arm = Some(Arm::Powc);
        self.write(Entry { kind: Kind::Transfer, who: demander, mu: moved.clone(), at: anchor.0, receipt: anchor.1, arm });
        self.write(Entry { kind: Kind::Transfer, who: discoverer, mu: moved.clone(), at: anchor.0, receipt: anchor.1, arm });
        moved
    }

    /// MU-CLIENT §5.6 (E2) — POWC MINTED: a demand closed at another
    /// perspective — another mind's citation discharging what a speaker
    /// could not — pays the closer the citation's measured discharge, a mint
    /// carrying the POWC arm (78.33). The caller measures it and refuses
    /// self-pay; nothing here prices it.
    pub fn settle_convergence(&mut self, closer: [u8; 32], mu: impl Into<Mu>, anchor: Anchor) -> Mu {
        self.mint_to(closer, mu, anchor, Some(Arm::Powc))
    }

    /// A SUBSIDY from the pool (system-initiated yields with no payer: the
    /// redundancy probe, bootstrap awards). Bounded by the pool, like all pay.
    /// No proof of work (78.33 as ruled): a subsidy is a policy ratio
    /// (MU-ECONOMY §8), and its row carries no arm.
    pub fn settle_subsidy(&mut self, party: [u8; 32], mu: impl Into<Mu>, anchor: Anchor) -> Mu {
        self.mint_to(party, mu, anchor, None)
    }

    /// Yield of an optimisation, read on the carriers as a lane vector.
    /// Zero on every lane is not work — refused, not paid zero.
    pub fn settle_rewrite(
        &self,
        kind: crate::transducer::rewrite::RewriteKind,
        before: &[crate::transducer::element::Element],
        after: &crate::transducer::element::Element,
    ) -> Option<[u64; 4]> {
        let mu = crate::transducer::rewrite::yield_vector(kind, before, after);
        if mu.iter().all(|&x| x == 0) {
            None
        } else {
            Some(mu)
        }
    }

    /// RM1 — A TORSE PRICED OFF ITS OWN RECORD: the cycle the rewrite will
    /// carry, `None` where it measures nothing.
    pub fn price_torse(&self, cycles: &[crate::transducer::element::TorsionCycle]) -> Option<[u64; 4]> {
        let mu = crate::transducer::rewrite::torse_yield(cycles);
        match mu.iter().any(|&x| x > 0) {
            true => Some(mu),
            false => None,
        }
    }

    /// Release an escrowed Rewrite: mint the claimed vector to the discoverer.
    /// Release an escrowed Rewrite to the parties the escrow names: EVERY
    /// voice that stood it, split per lane, as a consilience award is split.
    /// `escrow_award` accumulates voices — each new one is pushed onto the
    /// entry — so they are all parties to the claim, and paying
    /// `voices.first()` paid one of several who did the work, chosen by the
    /// order they happened to arrive in (rule 15). The `voice` argument it
    /// fell back on was an invented payee besides (rule 11).
    ///
    /// `None` when the address holds no escrow, or holds one with no voice:
    /// a mint with no party is a mint to nobody, and the escrow stands
    /// rather than being spent into the void. The mint stands at `at`, the
    /// act that released it (EC1), never at the escrow's.
    ///
    /// The mint carries `absorbed`, the modes of the family the reduction
    /// absorbs (MU-ECONOMY §2.5): each lane of the escrowed yield set on
    /// that lane's absorbed modes in proportion, a lane they do not reach at
    /// its founding mode (`Mu::carried_by`). Where nothing absorbed reads a
    /// mode, the escrow's own modes are released.
    pub fn release_rewrite(&mut self, address: &[u8], absorbed: &crate::economy::modes::Modes, at: Anchor) -> Option<Mu> {
        let row = self.escrowed.get(address)?;
        if row.voices.is_empty() {
            return None;
        }
        let mu = match absorbed.is_empty() {
            true => row.mu.clone(),
            false => Mu::carried_by(&row.mu, absorbed),
        };
        let voices = row.voices.clone();
        self.escrowed.remove(address);
        // THE SPLIT CANNOT EXCEED WHAT IT RELEASES. This read
        // `(mu[l] / n).max(1)`, so a lane holding less than one unit per
        // voice minted ONE to each of them: an escrow of 1 released to four
        // voices minted 4, and the treasury created three units it was not
        // holding. `.max(1)` is also a monetary constant, which MU-ECONOMY §8
        // says there are none of — a floor on a share is a price.
        //
        // The share is the exact quotient `mu / n` over ℚ: every voice the
        // same part, so no voice is ranked above another (rule 4), and
        // `n · share = mu` on every lane — nothing invented, nothing left
        // unminted. (Over counts it was `⌊mu / n⌋` and the remainder was
        // never issued.)
        let share = mu.part(voices.len())?;
        if share.is_zero() {
            // Nothing divides. The escrow is already removed and the row is
            // spent; minting zero is not a payment and is not written.
            return Some(share);
        }
        // POWR (78.33): the compression yield, standing at the act that
        // released it.
        for v in &voices {
            self.mint_to(*v, share.clone(), at, Some(Arm::Powr));
        }
        Some(share)
    }

    pub fn settle_solve(&mut self, voice: [u8; 32], mu: impl Into<Mu>, anchor: Anchor) -> Mu {
        self.mint_to(voice, mu, anchor, Some(Arm::Pouw))
    }

    /// POWV — a verifying court's share of a release (`economy::verify`),
    /// minted to the court at the act that released the claim.
    pub fn settle_verification(&mut self, organ: [u8; 32], mu: impl Into<Mu>, anchor: Anchor) -> Mu {
        self.mint_to(organ, mu, anchor, Some(Arm::Powv))
    }

    /// 26 AS RULED 2026-09-10 — THE USE OF A CITED GENERATOR IS A SHARE
    /// OF THE INSTALL'S OWN MINT, NEVER A TRANSFER. A citer's mint is
    /// what the citer closed, and part of what it closed is what the
    /// forms it cited close inside it; that part belongs to the cited forms. Drawn from
    /// the released award and minted to each cited form's own discoverer,
    /// the remainder to the escrow's voices.
    ///
    /// Read this against `settle_use`, which stays what it was: a
    /// TRANSFER out of a demanding voice's balance, for a probe against
    /// an open listing. As the use of a generator it was wrong twice over
    /// — a voice holding nothing cited for free, and where the citer's
    /// own draw already occupied the lane the generator is priced on, the
    /// transfer paid zero however much either party held (measured at
    /// cdb917f: mag [0,1,0,0] against a balance of [8,0,8,8]).
    ///
    /// `shares` is one entry per CITED ADDRESS — the caller keys them, so
    /// a form cited by two vertices is one share of one verdict. A share
    /// larger than what is left is paid what is left: the shares are
    /// measured on the citer's own family and cannot exceed its mint
    /// except by a rounding the caller did not intend, so the floor is a
    /// bound on arithmetic and never an invented value (rule 11).
    ///
    /// MU-CLIENT §5.6 (S) — `premium`, where it is given, is what the escrow's
    /// voices are paid: a sentence's GLUING, read off the sites its rings
    /// close beyond its words', never the arithmetic left after the shares —
    /// capped by what is left, and what is left above it never issued (E1).
    /// `None` for a word, whose voices take what is left.
    ///
    /// K3 (78.177 §4a(1), ruled 2026-11-06) — `converged` are the minds
    /// whose own words converged with this one, each with its share of the
    /// enclosure: they are paid POWC out of the composition premium, after the
    /// base and use shares (`Treasury::pay_converging`); a word has no
    /// premium, and its convergences are held as claims, never minted.
    pub fn settle_recursive(
        &mut self,
        escrow: &[u8],
        shares: &[([u8; 32], Mu)],
        pay: &Mu,
        premium: Option<Mu>,
        converged: &[([u8; 32], Mu)],
        anchor: Anchor,
    ) -> Mu {
        let Some(crate::economy::escrow::Escrowed { voices, mu, .. }) = self.release_award(escrow)
        else {
            return Mu::zero();
        };
        // 50.5 (Y3): A sentence with word vertices and demand vertices mints
        // on its closed sub-complex: the amount is the deflation its closed
        // words achieved (`pay`) and nothing else, bounded by what was escrowed.
        let mut left = pay.capped(&mu);
        for (discoverer, share) in shares {
            let (paid, rest) = left.split_at(share);
            if paid.is_zero() {
                continue;
            }
            left = rest;
            // MU-ECONOMY §2.5: a use or citation share carries the cited
            // word's closure modes (`share`'s), each lane at what the mint
            // had left for it, never the citer's.
            self.mint_to(*discoverer, Mu::carried_by(&paid, share.modes()), anchor, Some(Arm::Pouw));
        }
        let (taken, composed) = match premium {
            Some(p) => {
                // The gluing is a reading (§2.5): what the voices take is
                // what is left, capped lane-wise at it, keeping the
                // closure's modes.
                let p = left.capped(&p);
                (p.clone(), p)
            }
            None => (left, Mu::zero()),
        };
        // K3: the converging minds out of the premium; the voices keep the
        // rest of it. `to_minds ≤ composed ≤ taken` on every lane.
        let to_minds = self.pay_converging(escrow, &composed, converged, anchor);
        // to_minds ≤ taken on every lane: the voices' rest keeps the
        // closure's modes, drawn in proportion (§2.5), never a reading's.
        let rest = match taken.draw(&to_minds) {
            Ok(rest) => rest,
            Err(_) => taken.shortfall(&to_minds),
        };
        self.mint_claim(&voices, rest, anchor);
        taken
    }

    /// PoWc — split the consilience award (the germ's μ⃗ PRICED at the moment's
    /// surface) across the attesting voices, per lane. The share is the exact
    /// quotient `mu[l] / n` over ℚ: the voices' shares sum to the award.
    pub fn settle_consilience(&mut self, voices: &[[u8; 32]], mu: impl Into<Mu>, anchor: Anchor) {
        if voices.len() < 2 {
            return;
        }
        let Some(share) = mu.into().part(voices.len()) else {
            return;
        };
        if share.is_zero() {
            return;
        }
        for v in voices {
            self.mint_to(*v, share.clone(), anchor, Some(Arm::Pouw));
        }
    }

    pub fn settle_torse(&mut self, discoverer: [u8; 32], mu: impl Into<Mu>, anchor: Anchor) -> Mu {
        self.mint_to(discoverer, mu, anchor, Some(Arm::Pouw))
    }

    /// A reading (decomposition) charges the READER what the reading
    /// exercised, per lane, out of its earned balance. This is the RECHARGE
    /// stroke: the pool drains on discovery (a subsidy mints) and refills on
    /// use (a fee is burned), so it breathes instead of dying.
    ///
    /// THE FEE IS WHAT WAS MEASURED. It took `lanes: [bool; 4]` and charged
    /// `(lanes[l] as u64)` — one unit per lane touched, a monetary constant
    /// where `MU-ECONOMY.md` §8 says there are none and §5 says a reading is
    /// priced against the structure it moved. `Element::lanes()` threw the
    /// extent away and this cast a bit back into a magnitude; the μ⃗ was
    /// already measured at the call site. `price` is the magnitude on the
    /// lane (§5), so the fee is that, and a reader who moved more of a lane
    /// pays more of it.
    ///
    /// WHAT CANNOT BE PAID IS READ, NOT CLAMPED. `fee[l].min(held[l])` is a
    /// scalar comparison per lane returning a value; the doctrine's form is
    /// the shortfall reading (law 5, §2.3), which says on which lanes the
    /// reader came up short and by how much. The caller acts on the lanes it
    /// reads. Returns `(paid, short)`.
    ///
    /// THE FEE IS A BURN, not a transfer. It stood as a `transfer` entry
    /// with only the debit half written — the μ⃗ left a balance and reached
    /// no carrier, so `Σ balance` fell while `supply` held and §6's second
    /// equation failed on every lane a reader had ever paid on. §2's
    /// carriers are balance, bond and escrow: there is no pool among them
    /// to hold the fee, and the pool a subsidy draws on is issuance
    /// headroom, which a burn is exactly what returns to.
    pub fn settle_recall(
        &mut self,
        voice: [u8; 32],
        exercised: &[u64; 4],
        pressure: &[u64; 4],
        anchor: Anchor,
    ) -> (Mu, Mu) {
        // The pressure surface comes from the caller's own physics, as it
        // does at every other `price` site in the tree. R2 passed `&[0; 4]`
        // -- a constant filling an argument so the call compiled, which is
        // rule 8's named shape, and the one call site in seven that did not
        // measure what it handed in.
        let asked = Mu::counts(super::price(exercised, pressure));
        self.burn_fee(voice, &asked, anchor)
    }

    /// A FEE BURNED OUT OF A BALANCE: the part of `asked` the balance holds,
    /// and the shortfall it lacked, per lane. The burn row is written for
    /// μ⃗ that LEFT, after the debit — which cannot be short, by
    /// `paid ≤ held`.
    fn burn_fee(&mut self, voice: [u8; 32], asked: &Mu, anchor: Anchor) -> (Mu, Mu) {
        let held = self.balance_mu(&voice);
        let short = asked.shortfall(&held);
        let (paid, _) = held.split_at(asked);
        if paid.is_zero() {
            return (paid, short);
        }
        let Ok(moved) = self.debit(voice, &paid) else { return (Mu::zero(), short) };
        self.write(Entry { kind: Kind::Burn, who: voice, mu: moved.clone(), at: anchor.0, receipt: anchor.1, arm: None });
        (moved, short)
    }

    /// THE DOOR FEE (D12) — a strained mesh prices its own door: the speaker
    /// pays the congestion fee (per lane, derived from the reserve deficit —
    /// zero on a calm mesh) out of earned balance. The door is an economic
    /// rate limit, not a gate: what cannot be paid is simply not paid, and
    /// the caller decides deferral from the shortfall. Returns the per-lane
    /// amount actually paid.
    ///
    /// A burn for the same reason `settle_recall` is: the congestion fee
    /// leaves circulation, and a `transfer` with no payee was μ⃗ leaving a
    /// balance for nowhere. And the part that cannot be paid is the
    /// shortfall reading, not a `min` — it is exactly what the caller
    /// defers on, per lane. Returns `(paid, short)`.
    pub fn settle_door(
        &mut self,
        voice: [u8; 32],
        fee: impl Into<Mu>,
        anchor: Anchor,
    ) -> (Mu, Mu) {
        self.burn_fee(voice, &fee.into(), anchor)
    }

}
