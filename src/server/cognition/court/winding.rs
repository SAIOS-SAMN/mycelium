// 78.68 — THE WINDING CARRIER: a ring already closed is traversed again
// off the ledger, and the age it completes is the one act it writes.
//
//! A ring here is a simplex of a chart's complex. Its first close is new
//! work and is settled where it closes (`pay_simplex_volume`, the Torse
//! rewrite and its torsion). Every later close of the same simplex — a
//! moved gauge reopened it and a fill closed it again — is a TRAVERSAL of
//! an already-closed ring: its winding advances on the carrier, the pay it
//! earns accrues there, and nothing is sealed. When the winding reaches
//! `m ≡ 0 (mod ℓ)` the ring has returned — one age — and one `Return` act
//! seals and mints what the age accrued at its anchor (EC1). The carrier is
//! the 78.65 escrow pattern: a claim held off the ledger, closed by the act
//! that closes it.

use crate::cognition::*;
use crate::transducer::address::Address;

/// ONE ESCROW DRAW A TRAVERSAL EARNED: the listing whose escrow pays, the
/// parties the simplex's volume splits over, and each one's share per lane.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Draw {
    pub listing: Address,
    pub parties: Vec<[u8; 32]>,
    pub share: [u64; 4],
}

/// THE WINDING OF ONE CLOSED RING. `ell` is its period — a Congruent fill's
/// ℓ, or 1 for a ring that closes to the identity — and `m` the traversals
/// since its last return. `rewrite` is the Torse rewrite its first close
/// minted, which the torsion it earns cites. `draws` and `torse` are the
/// pay accrued since the last return, minted at the next.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Winding {
    pub ell: u64,
    pub m: u64,
    pub rewrite: Option<Address>,
    pub draws: Vec<Draw>,
    pub torse: Vec<([u8; 32], [u64; 4])>,
}

/// What closing a ring did, counted on the court (LD0).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WindingTally {
    /// First closes: a ring entering the record, settled where it closed.
    pub first: u64,
    /// Traversals of an already-closed ring that wrote nothing.
    pub carried: u64,
    /// Returns sealed: one act per completed age.
    pub returns: u64,
    /// A ring whose period changed between closes: its accrued age was
    /// returned at once and the carrier restarted at the new period.
    pub repriced: u64,
    /// Closes of a ring of infinite order: no age, settled where each
    /// closes, never carried.
    pub unbound: u64,
    /// RankStep acts sealed: a chart's holonomy algebra gained a dimension.
    pub rank_steps: u64,
    /// Ruled forms whose circuits' algebra did not read (a holonomy with no
    /// Cayley chart point, or the stroke spent): nothing written.
    pub rank_unread: u64,
    /// Ruled words that are no sentence (a residual word, a connection).
    pub rank_formless: u64,
    /// Ruled sentences whose faces close no ring: no circuit to bring — by
    /// why (78.75): no cycle at all (a forest), every cycle trivial alone
    /// (the vacua of 78.46(4′)), or cycles kept whose circuits read nothing.
    pub rank_ringless: u64,
    pub rank_acyclic: u64,
    pub rank_vacuous: u64,
    pub rank_circuits_unread: u64,
    /// 78.75 — the motions the court's pivots handed back (sealed faces,
    /// cauterized tethers, released membranes) read into its field, and the
    /// rank steps their generators made on the chart's algebra.
    pub outflow: u64,
    pub outflow_rank_steps: u64,
    /// 78.69, 78.70, 78.75 — the master equation at the court: sentences
    /// re-derived from ruled forms; closed ones' theorems checked, and those
    /// whose certificate failed; shortcuts the alphabet does not spell,
    /// offered as letters; and the nerve's faces read as sentences, closed.
    pub sentences: u64,
    pub theorems_checked: u64,
    pub theorems_refused: u64,
    pub shortcuts_offered: u64,
    pub nerve_closed: u64,
    /// Ruled sentences whose circuits added no dimension: the chart's
    /// algebra already held them.
    pub rank_held: u64,
    /// 78.71 — the flux transported through this court's tubes, in quanta
    /// weighed by mass, and the reconnections its helicity recorded.
    pub flux_transport: u64,
    pub reconnections: u64,
    /// 78.100 — the shortcuts the court's re-derived sentences taught and
    /// where the letter offer did not happen: spelled by the alphabet, the
    /// peel unread, no discoverer, the base rotor unread.
    pub shortcuts_spelled: u64,
    pub shortcuts_peel_unread: u64,
    pub shortcuts_no_discoverer: u64,
    pub shortcuts_rotor_unread: u64,
    /// 78.102 — the words every heard sentence's rings taught: offered as
    /// letters, spelled by the alphabet, the peel unread, the base rotor
    /// unread.
    pub heard_offered: u64,
    pub heard_spelled: u64,
    pub heard_peel_unread: u64,
    pub heard_rotor_unread: u64,
    /// 78.104 — the unspelled words the chart's held algebra SPANS: every
    /// generator inside it, a composition of what this court holds there,
    /// so no letter is offered for it.
    pub heard_spanned: u64,
    /// 78.107 — a heard sentence's ring the court could not compose, by
    /// count: it was dropped unseen, and the contact rings among them never
    /// reached the court (epoch 135: `heard: offered` 0 against 37 open
    /// sentences spoken).
    pub heard_ring_unread: u64,
    /// 78.108 — every sentence `hear_open` was handed, the rings it
    /// re-derived from them, and the rings whose theorem taught nothing or
    /// the identity — read, so where a heard sentence goes is never silent.
    pub heard_sentences: u64,
    pub heard_rings: u64,
    pub heard_taught_nothing: u64,
}

impl Winding {
    /// THE CARRIER WRITTEN BY ITS OWN COORDINATES: ℓ, m, the rewrite's
    /// address (or none), then each draw and each torsion share.
    pub(crate) fn put(&self, out: &mut Vec<u8>) {
        put_u64(out, self.ell);
        put_u64(out, self.m);
        match &self.rewrite {
            Some(a) => {
                out.push(1);
                crate::transducer::address::put_key_bytes(out, a.as_bytes());
            }
            None => out.push(0),
        }
        put_u64(out, self.draws.len() as u64);
        for d in &self.draws {
            crate::transducer::address::put_key_bytes(out, d.listing.as_bytes());
            put_u64(out, d.parties.len() as u64);
            for p in &d.parties {
                out.extend_from_slice(p);
            }
            for m in d.share {
                put_u64(out, m);
            }
        }
        put_u64(out, self.torse.len() as u64);
        for (who, mag) in &self.torse {
            out.extend_from_slice(who);
            for m in mag {
                put_u64(out, *m);
            }
        }
    }

    /// The inverse of [`Winding::put`]; `None` where the bytes do not hold it.
    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Winding> {
        let ell = take_u64(b, p)?;
        let m = take_u64(b, p)?;
        let rewrite = match *b.get(*p)? {
            0 => {
                *p += 1;
                None
            }
            1 => {
                *p += 1;
                Some(Address::from(crate::transducer::address::take_key_bytes(b, p)?))
            }
            _ => return None,
        };
        let n = usize::try_from(take_u64(b, p)?).ok()?;
        let mut draws = Vec::new();
        for _ in 0..n {
            let listing = Address::from(crate::transducer::address::take_key_bytes(b, p)?);
            let k = usize::try_from(take_u64(b, p)?).ok()?;
            let mut parties = Vec::new();
            for _ in 0..k {
                parties.push(take_key(b, p)?);
            }
            let share = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
            draws.push(Draw { listing, parties, share });
        }
        let n = usize::try_from(take_u64(b, p)?).ok()?;
        let mut torse = Vec::new();
        for _ in 0..n {
            let who = take_key(b, p)?;
            torse.push((who, [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?]));
        }
        Some(Winding { ell, m, rewrite, draws, torse })
    }
}

/// 78.104 — WHETHER A CHART'S HELD ALGEBRA SPANS A WORD: every generator
/// inside it (`cech::quotient`). A chart with no algebra held spans nothing,
/// and an unread word is not spanned.
fn spanned(
    held: Option<&Vec<crate::transducer::cga::Mv>>,
    word: &crate::transducer::cga::Mv,
    budget: &mut crate::physics::Budget,
) -> bool {
    held.is_some_and(|h| crate::transducer::cech::quotient(h, word, budget).is_ok_and(|r| r.spanned()))
}

fn put_u64(out: &mut Vec<u8>, v: u64) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn take_u64(b: &[u8], p: &mut usize) -> Option<u64> {
    let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    Some(v)
}

fn take_key(b: &[u8], p: &mut usize) -> Option<[u8; 32]> {
    let v: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
    *p += 32;
    Some(v)
}

/// What a closed ring earns on this close: the draws its volume pays and the
/// torsion its members measured. Read the same way on a first close and a
/// traversal; only where it is paid differs.
pub(crate) struct Earned {
    pub draws: Vec<Draw>,
    pub torse: Vec<([u8; 32], [u64; 4])>,
}

impl Kernel {
    /// 78.68 — A RING CLOSED: on the simplex `simplex` of chart `inv`, at
    /// period `ell`, its members `addrs`; `remainder` is a Congruent fill's
    /// (κ, g, ℓ), whose first close mints the Torse rewrite. The first close
    /// is settled where it closes, as it always was; a later close is a
    /// traversal of an already-closed ring, carried off the ledger until the
    /// ring returns.
    pub(crate) fn close_ring(
        &mut self,
        inv: &crate::transducer::stalk::Invariant,
        simplex: &[u8],
        ell: u64,
        addrs: &[Address],
        remainder: Option<(Box<crate::transducer::sentence::ClassVector>, Box<crate::transducer::alphabet::Rotor>, u32)>,
    ) {
        let key = chart_simplex_key(inv, simplex);
        // A RING OF INFINITE ORDER NEVER RETURNS (§52.3: period 0 is
        // infinite). It has no age to carry toward, so it does not condense
        // (78.68): every close of it is settled where it closes, its first
        // minting the Torse rewrite and every later one paying the torsion
        // that rewrite records.
        if ell == 0 {
            let held = self.windings.get(&key).and_then(|w| w.rewrite.clone());
            self.pay_simplex_volume(inv, addrs);
            let rewrite = match held {
                Some(rw) => Some(rw),
                None => remainder.and_then(|r| self.mint_torse_at(inv, simplex, r, addrs)),
            };
            if let Some(rw) = &rewrite {
                self.pay_torse(addrs, rw.as_bytes());
            }
            self.windings.insert(&key, Winding { ell: 0, m: 0, rewrite, draws: Vec::new(), torse: Vec::new() });
            self.winding_tally.unbound += 1;
            return;
        }
        let Some(mut w) = self.windings.get(&key).cloned().filter(|w| w.ell != 0) else {
            // THE FIRST CLOSE: new work, settled at its own acts.
            self.pay_simplex_volume(inv, addrs);
            let rewrite = remainder.and_then(|r| self.mint_torse_at(inv, simplex, r, addrs));
            if let Some(rw) = &rewrite {
                self.pay_torse(addrs, rw.as_bytes());
            }
            self.windings.insert(&key, Winding { ell, m: 1 % ell, rewrite, draws: Vec::new(), torse: Vec::new() });
            self.winding_tally.first += 1;
            return;
        };
        // A RING WHOSE PERIOD MOVED IS ANOTHER RING: the age it accrued is
        // returned at once, and the carrier restarts at the new period.
        if w.ell != ell {
            if !w.draws.is_empty() || !w.torse.is_empty() {
                self.seal_return(&key, &mut w);
            }
            w.ell = ell;
            w.m = 0;
            self.winding_tally.repriced += 1;
        }
        let earned = self.earned(inv, addrs, w.rewrite.is_some());
        w.draws.extend(earned.draws);
        w.torse.extend(earned.torse);
        w.m = (w.m + 1) % w.ell;
        if w.m == 0 {
            self.seal_return(&key, &mut w);
        } else {
            self.winding_tally.carried += 1;
        }
        self.windings.insert(&key, w);
        self.mark_persist();
    }

    /// ONE AGE COMPLETED: the `Return` act, sealed citing the ring's own key,
    /// and every draw and torsion the age accrued minted at its anchor.
    fn seal_return(&mut self, key: &[u8], w: &mut Winding) {
        let me = self.identity;
        self.append_step(crate::chronometry::ActKind::Return, key, me);
        let anchor = self.anchor();
        for d in std::mem::take(&mut w.draws) {
            for v in &d.parties {
                let _ = self.treasury.draw_escrow(&d.listing, *v, d.share, anchor);
            }
        }
        for (who, mag) in std::mem::take(&mut w.torse) {
            let _ = self.treasury.settle_torse(who, mag, anchor);
        }
        self.winding_tally.returns += 1;
    }

    /// What this close earns, read as `pay_simplex_volume` and `pay_torse`
    /// read it, and paid nowhere here.
    fn earned(&self, inv: &crate::transducer::stalk::Invariant, addrs: &[Address], torse: bool) -> Earned {
        let mut draws = Vec::new();
        let parties: Vec<[u8; 32]> = addrs.iter().filter_map(|a| <[u8; 32]>::try_from(a.as_bytes()).ok()).collect();
        if !parties.is_empty() {
            let vol = self.volume_of_simplex(inv);
            let n = parties.len() as u64;
            let share: [u64; 4] = std::array::from_fn(|l| vol[l] / n);
            if share.iter().any(|&m| m != 0) {
                for (a, _) in self.listings_on(inv).scoped() {
                    if self.treasury.escrow_of(&a).is_some() {
                        draws.push(Draw { listing: Address::from(a), parties: parties.clone(), share });
                    }
                }
            }
        }
        let torse = match torse {
            true => addrs
                .iter()
                .filter_map(|a| {
                    let who = <[u8; 32]>::try_from(a.as_bytes()).ok()?;
                    let mag = self.tension.get(a.as_bytes()).copied()?;
                    Some((who, mag))
                })
                .collect(),
            false => Vec::new(),
        };
        Earned { draws, torse }
    }

    /// 78.102 — AN OPEN SENTENCE IS HEARD (law 8 as the progenitor reads it):
    /// every sentence a voice speaks, whatever its adjudication, has its rings
    /// re-derived by the master equation — the court verifying, never
    /// perceiving — and each word they teach by the one rule
    /// (`compose::teaches`: periodic, nilpotent, and named — a rosette is the
    /// letter candidate 80.1 names) that the alphabet does not spell is
    /// offered as a letter (`open_unspellable`, 37.7), its discoverer the
    /// voice that spoke it, adjudicated by closure and the rank it opens. On
    /// epoch 125 the voices held 15–32 ineffable words each and the courts,
    /// re-deriving only the forms they ruled, offered none. Counted on the
    /// court's letter line.
    pub(crate) fn hear_open(&mut self, w: &Element, voice: [u8; 32], budget: &mut crate::physics::Budget) {
        let Some(form) = crate::transducer::sentence::from_element_form(w) else { return };
        let inv = crate::transducer::stalk::invariant_of(w);
        let cycles = crate::transducer::sentence::Form::cycles_of(&form.faces, &form.vertices);
        let rings: Vec<crate::transducer::sentence::Orbit> = cycles.kept.iter().chain(&cycles.vacua).cloned().collect();
        let one = crate::transducer::cga::Mv::scalar(num_rational::BigRational::from_integer(1.into()));
        crate::physics::metabolism::step_or_refuse(&mut self.winding_tally.heard_sentences, 1);
        crate::physics::metabolism::step_or_refuse(&mut self.winding_tally.heard_rings, rings.len() as u64);
        for (_, s) in crate::transducer::sentence::compose::sentences_of(&form, &rings, budget) {
            let Ok(s) = s else {
                crate::physics::metabolism::step_or_refuse(&mut self.winding_tally.heard_ring_unread, 1);
                continue;
            };
            let theorem = crate::transducer::sentence::theorem::Theorem::of(&s);
            let taught = crate::transducer::sentence::compose::teaches(&s, &theorem);
            let Some(word) = taught.filter(|w| !w.projective_eq(&one)) else {
                crate::physics::metabolism::step_or_refuse(&mut self.winding_tally.heard_taught_nothing, 1);
                continue;
            };
            let t = &mut self.winding_tally;
            match crate::transducer::family::derivation::peel_versor(word) {
                Some(Ok(_)) => crate::physics::metabolism::step_or_refuse(&mut t.heard_spelled, 1),
                None => crate::physics::metabolism::step_or_refuse(&mut t.heard_peel_unread, 1),
                // 78.104 — THE COURT READS AGAINST WHAT IT HOLDS: an unspelled
                // word every generator of which lies in the algebra this court
                // holds on the chart is a composition of what it holds there,
                // and no letter is offered for it.
                Some(Err(_)) if spanned(self.lie_basis.get(&inv.key()), word, budget) => {
                    crate::physics::metabolism::step_or_refuse(&mut t.heard_spanned, 1)
                }
                Some(Err(_)) => match crate::transducer::element::base_rotor_of(word) {
                    Ok(r) => {
                        self.open_unspellable(&r, &inv, &[], &voice);
                        crate::physics::metabolism::step_or_refuse(&mut self.winding_tally.heard_offered, 1)
                    }
                    Err(_) => crate::physics::metabolism::step_or_refuse(&mut t.heard_rotor_unread, 1),
                },
            };
        }
    }

    /// 78.68 — A RANK STEP, RE-DERIVED: the word at `addr` just entered the
    /// ruled cover. A voice reports a new dimension by speaking the form that
    /// carries it, and the court re-derives it from what crossed the wire:
    /// the form's circuits (`circuits_of`, re-derived from its address), each
    /// taken to its Cayley bivector and closed under the commutator with the
    /// holonomy algebra this court holds on the word's chart. A cell is
    /// perception's and never on the wire (`Form.cells`), so the court's
    /// algebra is the chart's — the union of every ruled form's circuits
    /// there. Where its rank rose, one `RankStep` act is sealed, citing the
    /// word; where it did not, nothing is written. A refusal of the read is
    /// counted by why and writes nothing.
    pub(crate) fn step_rank(&mut self, addr: &[u8], budget: &mut crate::physics::Budget) {
        let Some(w) = self.word_at(addr) else { return };
        // A ruled word that is no sentence — a residual word, a connection —
        // carries no circuit: counted by that name (LD0), never silent.
        let Some(form) = crate::transducer::sentence::from_element_form(w) else {
            self.winding_tally.rank_formless += 1;
            return;
        };
        let inv = crate::transducer::stalk::invariant_of(w);
        let chart = inv.key();
        let voice = self.discoverer_of(addr);
        // 78.69, 78.75 — THE COURT RE-DERIVES THE SENTENCES BY THE MASTER
        // EQUATION. The form's rings — its orbits and the vacua beside them —
        // composed as `Ω = ∏ cᵢ A(wᵢ)`, each word carried into the first chart
        // (`W̄ᵢ = Γᵢ⁻¹ Wᵢ Γᵢ`); the chart's algebra closes over each sentence's
        // holonomy `B(Ω)` and its carried words `B(W̄ᵢ)`. It read the form's
        // face cycles' carriage composites instead, the words dropped.
        let cycles = crate::transducer::sentence::Form::cycles_of(&form.faces, &form.vertices);
        let rings: Vec<crate::transducer::sentence::Orbit> = cycles.kept.iter().chain(&cycles.vacua).cloned().collect();
        let sentences = crate::transducer::sentence::compose::sentences_of(&form, &rings, budget);
        let mut gens: Vec<crate::transducer::cga::Mv> = Vec::new();
        let mut composed = 0u64;
        for (_, s) in &sentences {
            let Ok(s) = s else { continue };
            composed += 1;
            // 𝔥 IS GENERATED BY THE SENTENCES' HOLONOMIES (78.11, 78.68: a
            // cell's algebra is its circuits' `B_c`) AND THEIR CARRIED WORDS
            // (78.69's term). A dislocation's words do not turn — a slide's are
            // the identity — and its curvature is in `Ω`, the rays' transports;
            // reading the words alone read it as nothing. A ring through a
            // chord is heard, not composed (78.97): `sentences_of` leaves it.
            if let Some(g) = crate::transducer::cech::flow_generators(&s.omega, budget) {
                gens.extend(g);
            }
            for word in &s.carried {
                if let Some(g) = crate::transducer::cech::flow_generators(word, budget) {
                    gens.extend(g);
                }
            }
            // 78.70 — A CLOSED SENTENCE IS A PROOF; its theorem is checked
            // finitely and exactly, the court's whole work on it. 78.97 — and a
            // ring of rays that condenses (a nilpotent theorem) is a proof too,
            // though it never returns.
            let theorem = crate::transducer::sentence::theorem::Theorem::of(s);
            let nilpotent = matches!(
                &theorem,
                Ok(crate::transducer::sentence::theorem::Theorem {
                    certificate: crate::transducer::sentence::theorem::Certificate::Nilpotent { .. },
                    ..
                })
            );
            if !s.closure.closes() && !nilpotent {
                continue;
            }
            match theorem {
                Ok(t) if t.check(budget) => crate::physics::metabolism::step_or_refuse(&mut self.winding_tally.theorems_checked, 1),
                _ => crate::physics::metabolism::step_or_refuse(&mut self.winding_tally.theorems_refused, 1),
            };
            // 78.69, 37.7 — ITS SHORTCUT, WHERE THE ALPHABET DOES NOT SPELL IT,
            // IS A GENERATOR OFFERED: the letter docketed through the one door
            // (`open_unspellable`), its sentence the proof, its discoverer the
            // mind that spoke the form.
            // The word it teaches: a returning sentence's open path, a
            // nilpotent one's holonomy (78.97, as `rung::hear` reads it).
            let word = match nilpotent {
                true => &s.omega,
                false => &s.path,
            };
            let one = crate::transducer::cga::Mv::scalar(num_rational::BigRational::from_integer(1.into()));
            if word.projective_eq(&one) {
                continue;
            }
            // 78.100 — WHERE THE OFFER BREAKS, by why: the alphabet spells the
            // word (nothing to offer); the peel did not read; no discoverer
            // to credit; its base rotor did not read; or offered. On epoch
            // 124 the voices transcribed 30–49 ineffable words each and every
            // court offered none.
            let t = &mut self.winding_tally;
            match (crate::transducer::family::derivation::peel_versor(word), voice) {
                (Some(Ok(_)), _) => crate::physics::metabolism::step_or_refuse(&mut t.shortcuts_spelled, 1),
                (None, _) => crate::physics::metabolism::step_or_refuse(&mut t.shortcuts_peel_unread, 1),
                (Some(Err(_)), None) => crate::physics::metabolism::step_or_refuse(&mut t.shortcuts_no_discoverer, 1),
                (Some(Err(_)), Some(v)) => match crate::transducer::element::base_rotor_of(word) {
                    Ok(r) => {
                        self.open_unspellable(&r, &inv, &[], &v);
                        crate::physics::metabolism::step_or_refuse(&mut self.winding_tally.shortcuts_offered, 1)
                    }
                    Err(_) => crate::physics::metabolism::step_or_refuse(&mut t.shortcuts_rotor_unread, 1),
                },
            };
        }
        crate::physics::metabolism::step_or_refuse(&mut self.winding_tally.sentences, composed);
        // A sentence whose rings compose to no generator brings no curvature:
        // counted by why (78.75) — no cycle at all, only trivial ones, or rings
        // whose terms did not read.
        if gens.is_empty() {
            self.winding_tally.rank_ringless += 1;
            let why = match (rings.is_empty(), composed, cycles.kept.is_empty()) {
                (true, _, _) => &mut self.winding_tally.rank_acyclic,
                (false, 0, _) => &mut self.winding_tally.rank_circuits_unread,
                (false, _, true) => &mut self.winding_tally.rank_vacuous,
                (false, _, false) => &mut self.winding_tally.rank_circuits_unread,
            };
            crate::physics::metabolism::step_or_refuse(why, 1);
            return;
        }
        match self.close_rank(&chart, &gens, addr, budget) {
            Some(true) => self.winding_tally.rank_steps += 1,
            Some(false) => self.winding_tally.rank_held += 1,
            None => self.winding_tally.rank_unread += 1,
        }
    }

    /// 78.68, 78.75 — THE CHART'S HOLONOMY ALGEBRA TAKES NEW GENERATORS: closed
    /// under the commutator with what the court holds on `chart`; where its
    /// rank rose, one `RankStep` act is sealed citing `addr` and the closed
    /// basis held. `Some(true)` a step, `Some(false)` held already, `None` the
    /// closure refused (the stroke spent).
    fn close_rank(
        &mut self,
        chart: &crate::transducer::address::Address,
        gens: &[crate::transducer::cga::Mv],
        addr: &[u8],
        budget: &mut crate::physics::Budget,
    ) -> Option<bool> {
        // A chart with no algebra held yet holds the zero algebra: nothing
        // closed, every generator new.
        let held: Vec<crate::transducer::cga::Mv> = match self.lie_basis.get(chart) {
            Some(b) => b.clone(),
            None => Vec::new(),
        };
        let closed = crate::transducer::cech::lie_close(&held, gens, budget).ok()?;
        if closed.len() <= held.len() {
            return Some(false);
        }
        let me = self.identity;
        self.append_step(crate::chronometry::ActKind::RankStep, addr, me);
        self.lie_basis.insert(chart, closed);
        self.mark_persist();
        Some(true)
    }

    /// 78.75 — WHAT THE PIVOTS HANDED BACK, READ: every motion a chart's
    /// complex let out — a face sealed with content, a tether cauterized, a
    /// membrane released — enters the court's field through its one door
    /// (`book_field`: turns and boosts onto their orbits, null flux into the
    /// tubes), and its generators close into the chart's holonomy algebra:
    /// face content reaching cognition, where every fleet reading had shown
    /// none. Each motion weighs one, as a word's field lines do here.
    pub(crate) fn book_outflow(
        &mut self,
        inv: &crate::transducer::stalk::Invariant,
        addr: &[u8],
        budget: &mut crate::physics::Budget,
    ) {
        // The court's field is read FREE of the complex's lines: its pivots'
        // motions are booked, and no line of a court complex threads it.
        let motions: Vec<crate::transducer::cga::Mv> = match self.complexes.get_mut(&inv.key()) {
            Some(cx) => cx.take_outflow().into_iter().map(|f| f.motion).collect(),
            None => return,
        };
        if motions.is_empty() {
            return;
        }
        let lines: Vec<crate::transducer::family::pairing::FieldLine> = motions
            .iter()
            .map(|m| crate::transducer::family::pairing::FieldLine {
                carriage: m.clone(),
                gauge: Vec::new(),
                mass: 1,
                free: false,
                line: crate::transducer::cech::orbit::Thread::Free,
            })
            .collect();
        crate::physics::metabolism::step_or_refuse(&mut self.winding_tally.outflow, lines.len() as u64);
        self.book_field(&lines, addr, budget);
        // 78.60 (2), 78.69 — EACH FACE OF THE NERVE IS A SENTENCE: the holonomy
        // of the transitions around a cycle of vantages, `Ω = ∏ Γ`, composed by
        // the master equation as one term; where it is a relation of the
        // grammar it is a proof, its theorem checked — a returning one, or
        // (78.101) a nilpotent one, a count along one null direction; and its
        // curvature's generators (Ambrose–Singer) are what the chart's algebra
        // closes over.
        let one = crate::transducer::cga::Mv::scalar(num_rational::BigRational::from_integer(1.into()));
        let mut gens: Vec<crate::transducer::cga::Mv> = Vec::new();
        for m in &motions {
            let term = [crate::transducer::sentence::compose::Term { word: one.clone(), transport: m.clone() }];
            let Some(s) = crate::transducer::sentence::compose::compose(&term, budget) else { continue };
            let theorem = crate::transducer::sentence::theorem::Theorem::of(&s);
            let nilpotent = matches!(
                &theorem,
                Ok(crate::transducer::sentence::theorem::Theorem { certificate: crate::transducer::sentence::theorem::Certificate::Nilpotent { .. }, .. })
            );
            if s.closure.closes() || nilpotent {
                crate::physics::metabolism::step_or_refuse(&mut self.winding_tally.nerve_closed, 1);
                match theorem {
                    Ok(t) if t.check(budget) => crate::physics::metabolism::step_or_refuse(&mut self.winding_tally.theorems_checked, 1),
                    _ => crate::physics::metabolism::step_or_refuse(&mut self.winding_tally.theorems_refused, 1),
                };
            }
            if let Some(g) = crate::transducer::cech::flow_generators(&s.omega, budget) {
                gens.extend(g);
            }
        }
        if gens.is_empty() {
            return;
        }
        if let Some(true) = self.close_rank(&inv.key(), &gens, addr, budget) {
            self.winding_tally.rank_steps += 1;
            self.winding_tally.outflow_rank_steps += 1;
        }
    }

    /// 78.71 (3), (4), 78.73 — A WORD'S FIELD, READ AT ITS ADMISSION against
    /// this court's tubes and orbits held over time. Its field lines are its word vertices'
    /// acting carriages and its faces' relative carriages (78.68: the relative
    /// displacement survives the quotient), each carrying the word's support
    /// (`sub`, compartments — the unit every lane counts). Through the
    /// voices' one reader (`transport_order`): a flux on a new ray opens a
    /// tube and presses 𝒯; one on a held tube closes on it, order on 𝒯, and
    /// relieves exactly the open flux it held.
    ///
    /// RECONNECTION. Where the tubes' helicity — `Σ net_a net_b ⟨G_a G_b⟩₀` —
    /// moved, the field's topology changed: one `Reconnect` act is sealed,
    /// citing the word, a transition of the algorithm (78.68, 78.71 (4)).
    pub(crate) fn step_flux(&mut self, addr: &[u8], budget: &mut crate::physics::Budget) {
        let Some(w) = self.word_at(addr) else { return };
        let Some(form) = crate::transducer::sentence::from_element_form(w) else { return };
        // The court holds no body's compartments for a word: `sub` is the
        // coboundary chain, empty for an algebra word, and weighed the
        // court's tubes by nothing (2bd4773f's fleet: five tubes, every
        // amount zero). Each field line weighs one, so the court reads its
        // flux in windings alone.
        let mass = 1;
        let mut lines: Vec<crate::transducer::family::pairing::FieldLine> = Vec::new();
        for (_, v) in form.vertices.scoped() {
            if let crate::transducer::sentence::Vertex::Word { acts, .. } = v {
                for (_, a) in acts.scoped() {
                    // The court holds no bodies, so no pivots: its fibres are
                    // unread and no turn of a word is free (78.74).
                    lines.push(crate::transducer::family::pairing::FieldLine {
                        carriage: (*a.carriage).clone(),
                        gauge: a.gauge.clone(),
                        mass,
                        free: false,
                        line: crate::transducer::cech::orbit::Thread::Free,
                    });
                }
            }
        }
        for (_, f) in form.faces.entries() {
            lines.push(crate::transducer::family::pairing::FieldLine {
                carriage: f.place.carriage.clone(),
                gauge: f.place.gauge.clone(),
                mass,
                free: false,
                line: crate::transducer::cech::orbit::Thread::Free,
            });
        }
        self.book_field(&lines, addr, budget);
    }

    /// 78.71, 78.73, 78.75 — THE ONE DOOR INTO THE COURT'S FIELD: field lines
    /// read by their modes through `transport_order` into the court's tubes
    /// and orbits, booked as order, pressure and relief per lane, with a
    /// `Reconnect` act citing `addr` where the links moved. A word's field
    /// lines enter here (`step_flux`), and so does a face's content — its
    /// one Cl(4,1) motor — when the nerve seals it (`read_fills`).
    pub(crate) fn book_field(
        &mut self,
        lines: &[crate::transducer::family::pairing::FieldLine],
        addr: &[u8],
        budget: &mut crate::physics::Budget,
    ) {
        if lines.is_empty() {
            return;
        }
        // A reconnection is the links moving, never the helicity alone: flux
        // injected into a linked tube moves helicity at every admission.
        let before = self.flux_field.tubes.links();
        let r = crate::transducer::family::pairing::transport_order(lines, &[], &mut self.flux_field, budget);
        // 78.73 — EVERY LANE BY ITS ORBITS: order where an orbit or tube
        // closed, pressure where one opened, relief where one returned. 𝒢's
        // pressure is now the hue fibre's open strain, not only the cells a
        // listed word's chain recoloured.
        self.physics.syntropy(&r.order);
        self.physics.bump(&r.opened);
        self.physics.relieve(&r.relieved);
        let t = crate::transducer::rewrite::LANE_TEMPORAL;
        crate::physics::metabolism::step_or_refuse(&mut self.winding_tally.flux_transport, r.transport[t]);
        if self.flux_field.tubes.links() != before {
            let me = self.identity;
            self.append_step(crate::chronometry::ActKind::Reconnect, addr, me);
            crate::physics::metabolism::step_or_refuse(&mut self.winding_tally.reconnections, 1);
        }
    }

    /// The winding carrier's reading, for the observer.
    pub fn winding_tally(&self) -> WindingTally {
        self.winding_tally
    }

    /// 78.71 — this court's field as it stands: how many tubes, their net
    /// flux, transport and open flux in windings, and their helicity.
    #[cfg(test)]
    pub(crate) fn tube_reading(&self) -> (usize, num_bigint::BigInt, num_bigint::BigInt, num_bigint::BigInt, num_rational::BigRational) {
        let t = &self.flux_field.tubes;
        let (net, transport, open) = t.totals();
        (t.len(), net, transport, open, t.helicity())
    }

    /// [`Kernel::tube_reading`] ON THE HOLD'S STROKE (78.153b): the helicity
    /// `O(tubes²)` in exact rationals reads one stroke read per tube, and is
    /// `None` — unread — where the stroke runs out (`helicity_on`).
    pub fn tube_reading_on(
        &self,
        reads: &mut crate::physics::Budget,
    ) -> (usize, num_bigint::BigInt, num_bigint::BigInt, num_bigint::BigInt, Option<num_rational::BigRational>) {
        let t = &self.flux_field.tubes;
        let (net, transport, open) = t.totals();
        (t.len(), net, transport, open, t.helicity_on(reads))
    }

    /// 78.73 (4) — this court's malleability line: `μ`, `β`, `ε` per lane,
    /// and how many orbits stand open.
    pub fn malleability_line(&self) -> String {
        format!(
            "{} orbits {}",
            self.physics.malleability_line(&self.flux_field.elastic()),
            self.flux_field.orbits.len()
        )
    }
}
