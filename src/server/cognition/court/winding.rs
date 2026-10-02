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
    /// Ruled sentences whose faces close no ring: no circuit to bring.
    pub rank_ringless: u64,
    /// Ruled sentences whose circuits added no dimension: the chart's
    /// algebra already held them.
    pub rank_held: u64,
    /// 78.71 — the flux transported through this court's tubes, in quanta
    /// weighed by mass, and the reconnections its helicity recorded.
    pub flux_transport: u64,
    pub reconnections: u64,
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
        let chart = crate::transducer::stalk::invariant_of(w).key();
        let circuits = crate::transducer::sentence::circuits_of(&form);
        let holonomies: Vec<Option<&crate::transducer::cga::Mv>> = circuits.iter().map(|(_, t)| t.composite.as_ref()).collect();
        let gens = match crate::transducer::cech::cayley_generators(&holonomies) {
            Ok(g) => g,
            Err(_) => {
                self.winding_tally.rank_unread += 1;
                return;
            }
        };
        // A sentence whose faces close no ring has no circuit, and so no
        // holonomy to bring: counted by that name.
        if gens.is_empty() {
            self.winding_tally.rank_ringless += 1;
            return;
        }
        // A chart with no algebra held yet holds the zero algebra: nothing
        // closed, every generator new.
        let held: Vec<crate::transducer::cga::Mv> = match self.lie_basis.get(&chart) {
            Some(b) => b.clone(),
            None => Vec::new(),
        };
        let closed = match crate::transducer::cech::lie_close(&held, &gens, budget) {
            Ok(b) => b,
            Err(_) => {
                self.winding_tally.rank_unread += 1;
                return;
            }
        };
        if closed.len() <= held.len() {
            self.winding_tally.rank_held += 1;
            return;
        }
        let me = self.identity;
        self.append_step(crate::chronometry::ActKind::RankStep, addr, me);
        self.lie_basis.insert(&chart, closed);
        self.winding_tally.rank_steps += 1;
        self.mark_persist();
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
                    lines.push(((*a.carriage).clone(), a.gauge.clone(), mass));
                }
            }
        }
        for (_, f) in form.faces.entries() {
            lines.push((f.place.carriage.clone(), f.place.gauge.clone(), mass));
        }
        if lines.is_empty() {
            return;
        }
        // A reconnection is the links moving, never the helicity alone: flux
        // injected into a linked tube moves helicity at every admission.
        let before = self.flux_field.tubes.links();
        let r = crate::transducer::family::pairing::transport_order(&lines, &[], &mut self.flux_field, budget);
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
    pub fn tube_reading(&self) -> (usize, num_bigint::BigInt, num_bigint::BigInt, num_bigint::BigInt, num_rational::BigRational) {
        let t = &self.flux_field.tubes;
        let (net, transport, open) = t.totals();
        (t.len(), net, transport, open, t.helicity())
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
