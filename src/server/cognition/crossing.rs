// 78.154 — THE TRAVERSE, THE STAKE IT DRAWS ON, AND THE CONNECTION TAX on
// this court (MU-ECONOMY §7.1, §7.2; 78.156 §4–5).
//
//! A TRAVERSE buys coordinates: both mouths mapped, each universe's latest
//! entry within its own mouth's freshness `N` of the crossing's final point,
//! the two reserves' declarations equal where matter crosses. Its MAP FEE is
//! the route's gas per lane, priced at this court's READ PRICE — one μ per
//! unit of gas at rest, and the court's pressure on the lane above that
//! (`economy::yield_surface`: the reserve's deficit and the entropic excess,
//! emergent, never a published rate) — drawn from the payer's STAKE as a
//! CLAIM, held for the act's window `W` and then paid WHOLE to the court
//! keeping the route's map: a transfer, never a mint, no burn. An unstake is
//! refused while a claim drawn on the stake stands.
//!
//! The CONNECTION TAX is a declared flow on the reserve's tier fold
//! (`uplink::CONNECTION_TAX`), paid from the reserve to this court's
//! RECYCLED POOL, from which a closer's POUW is paid first (`mint_to`).

use super::atlas::record_key;
use super::challenge::window_end;
use super::Court;
use crate::anastome::dialogue::atlas::MapRecord;
use crate::anastome::dialogue::crossing::{CrossRefusal, Crossed, Edge, Gas, Side, SignedTraverse};
use crate::anastome::dialogue::uplink::{Child, TaxRefusal, TierFold, CONNECTION_TAX};
use crate::anastome::dialogue::wallet::{SignedStake, StakeRefusal};
use crate::chronometry::tuniversal::TPoint;
use crate::chronometry::{ActKind, Cite, KStepReceipt};
use num_rational::BigRational as Q;

/// The escrow key a claim is held under.
pub(crate) fn claim_key(at: &[u8]) -> Vec<u8> {
    let mut k = b"claim".to_vec();
    k.extend_from_slice(at);
    k
}

/// `⌊q · price⌋` exact, or `None` past the carrier.
fn priced(q: &Q, price: u64) -> Option<u64> {
    let f = (q * Q::from_integer(price.into())).floor();
    u64::try_from(f.to_integer()).ok()
}

/// 78.154 (4) — WHY A COORDINATE RETRIEVAL WAS NOT SERVED.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unretrieved {
    /// The point is not final in this court's view.
    NotFinal,
    /// The stake does not cover the read cost on a lane.
    StakeShort,
    /// A cost passes its carrier, or the claim's id is drawn already.
    Overflow,
}

impl Court {
    /// THE COURT'S READ PRICE per lane: one μ a unit of gas at rest, and its
    /// pressure on the lane above it. `None` where it passes its carrier.
    pub fn read_price(&self) -> Option<[u64; 4]> {
        let surface = crate::economy::yield_surface(&self.physics)?;
        let mut price = [0u64; 4];
        for (p, s) in price.iter_mut().zip(surface) {
            *p = s.checked_add(1)?;
        }
        Some(price)
    }

    /// A claim drawn on `staker`'s stake for `fee`, its parts to this court,
    /// held to the window from the act at `r`. `None` where it cannot be held.
    fn draw_claim(&mut self, staker: [u8; 32], id: &[u8; 32], fee: [u64; 4], r: &KStepReceipt) -> Option<()> {
        if fee.iter().all(|m| *m == 0) {
            return Some(());
        }
        let mut at = staker.to_vec();
        at.extend_from_slice(id);
        self.treasury.claim_stake(staker, &claim_key(&at), fee, (r.stamp, r.chain_hash)).ok()?;
        let claim = super::atlas::Claim { parts: vec![(self.identity, fee)], until: window_end(r.stamp.h), at: r.stamp, receipt: r.chain_hash };
        self.atlas.claims.insert(at, claim);
        Some(())
    }

    /// 78.154 — A TRAVERSE: the payer's signature; both universes founded
    /// and the mouth an open tube with A on the side named (or a LOCAL
    /// THROAT on A's map, both its ends in A, so B is A); the crossing's
    /// point final; every point at one origin; BOTH MOUTHS MAPPED, each at
    /// the point cited and not above the crossing's; each universe's LATEST
    /// entry cited, within ITS OWN mouth's `N` advances of the crossing's
    /// point (else NOT YET AT h, naming the side); the declarations both or
    /// neither, each its reserve's, equal on every lane; the route on the two
    /// maps. Sealed, the edge is recorded pending and the fee claimed.
    pub fn traverse(&mut self, s: &SignedTraverse, fin: Option<TPoint>) -> Crossed {
        use CrossRefusal as R;
        let refuse = Crossed::Refused;
        let t = &s.traverse;
        if !crate::court::identity::verify(&t.payer, &t.signed_bytes(), &s.sig) {
            return refuse(R::Unsigned);
        }
        let crossing = t.crossing();
        if self.atlas.edges.contains(&crossing) {
            return refuse(R::Crossed);
        }
        if self.domains.get(&t.from).is_none() {
            return refuse(R::UnknownDomain);
        }
        let other = match t.side {
            0 => 1,
            1 => 0,
            _ => return refuse(R::NoMouth),
        };
        // A mouth pair's sides each stand in its tier's universe; a local
        // throat's both stand in A's.
        let (Some(da), Some(to)) = (self.place_domain(&t.from, &(t.mouth, t.side)), self.place_domain(&t.from, &(t.mouth, other))) else { return refuse(R::NoMouth) };
        if da != t.from {
            return refuse(R::NoMouth);
        }
        if self.domains.get(&to).is_none() {
            return refuse(R::UnknownDomain);
        }
        if !self.final_in_view(&t.at, fin) {
            return refuse(R::NotFinal);
        }
        if [&t.point_a, &t.point_b].iter().any(|p| p.origin != t.at.origin) {
            return refuse(R::Origins);
        }
        let mut freshness = [0u64; 2];
        for (i, (domain, side, point, who)) in [(t.from, t.side, &t.point_a, Side::A), (to, other, &t.point_b, Side::B)].into_iter().enumerate() {
            let Some(mapped) = self.mapped_place(&domain, &(t.mouth, side)) else { return refuse(R::Unmapped(who)) };
            let n = mapped.freshness;
            if mapped.held.point != *point || point.h > t.at.h {
                return refuse(R::MapPoint(who));
            }
            if let Some(f) = freshness.get_mut(i) {
                *f = n;
            }
        }
        let [na, nb] = freshness;
        for (domain, entry, u, n, who) in [(t.from, t.entry_a, t.u_a, na, Side::A), (to, t.entry_b, t.u_b, nb, Side::B)] {
            let Some(head) = self.atlas.head(&domain) else { return refuse(R::NotLatest(who)) };
            if head.hash != entry || head.entry.u != u {
                return refuse(R::NotLatest(who));
            }
            // 78.156 §4 — bound at its crossings only, by its own mouth's N.
            match self.atlas.advances(&head.entry.point, &t.at) {
                Some(behind) if behind <= n => {}
                _ => return refuse(R::NotYetAtH(who)),
            }
        }
        let carried = match (&t.debit, &t.credit) {
            (None, None) => [0; 4],
            (Some(_), None) | (None, Some(_)) => return refuse(R::OneSided),
            (Some(d), Some(c)) => {
                let signed = |domain: &[u8; 32], bytes: Vec<u8>, sig: &[u8; 64]| self.reserve_of(domain).is_some_and(|r| crate::court::identity::verify(&r, &bytes, sig));
                if !signed(&t.from, t.debit_bytes(&d.lanes), &d.sig) {
                    return refuse(R::DeclarationUnsigned(Side::A));
                }
                if !signed(&to, t.credit_bytes(&c.lanes), &c.sig) {
                    return refuse(R::DeclarationUnsigned(Side::B));
                }
                if d.lanes != c.lanes {
                    return refuse(R::Mismatched);
                }
                d.lanes
            }
        };
        // THE GAS, per lane, exact: S the route's span, G and H what the two
        // reserves declared crossing, T the ticks between the mouths' points.
        let mut span = Q::from_integer(0.into());
        for id in &t.route {
            let seg = [t.from, to].iter().find_map(|d| self.atlas.records.get(&record_key(d, id)).map(|h| h.record.clone()));
            let Some(MapRecord::Segment { span: (n, d), .. }) = seg else { return refuse(R::Route) };
            span += Q::new(n.into(), d.into());
        }
        let (Ok(sn), Ok(sd)) = (u64::try_from(span.numer()), u64::try_from(span.denom())) else { return refuse(R::Overflow) };
        let ticks = t.point_a.h.abs_diff(t.point_b.h);
        let gas = Gas { s: (sn, sd), g: carried[1], t: ticks, h: carried[3] };
        let Some(price) = self.read_price() else { return refuse(R::Overflow) };
        let fee = [priced(&span, price[0]), gas.g.checked_mul(price[1]), gas.t.checked_mul(price[2]), gas.h.checked_mul(price[3])];
        let [Some(fs), Some(fg), Some(ft), Some(fh)] = fee else { return refuse(R::Overflow) };
        let fee = [fs, fg, ft, fh];
        let staked = self.treasury.stake_of(&t.payer).map_or([0; 4], |s| s);
        if !crate::economy::lanes_short(&crate::economy::shortfall(&fee, &staked)).is_empty() {
            return refuse(R::StakeShort);
        }
        let cite = Cite::Stated { domain: t.from, kind: crate::anastome::dialogue::atlas::KIND_TRAVERSE, body: crossing };
        let Some(r) = self.seal_step(ActKind::Domain, [0; 4], t.payer, |_| vec![cite.clone()]) else { return refuse(R::LineRefused) };
        let Some(court) = fin else { return refuse(R::NotFinal) };
        if self.draw_claim(t.payer, &crossing, fee, &r).is_none() {
            return refuse(R::StakeShort);
        }
        let edge = Edge { from: t.from, to, mouth: t.mouth, at: t.at, u_a: t.u_a, u_entry: t.u_b, u_applied: None };
        self.atlas.edges.insert(crossing, edge);
        self.mark_persist();
        Crossed::Sealed { crossing, h: r.stamp.h, point: court, gas, price, fee }
    }

    /// 78.154 (4) — THE CLAIMS WHOSE WINDOW CLOSED, paid WHOLE to the courts
    /// they name, each part's rows citing the act that drew it. Claims paid.
    pub(crate) fn release_claims(&mut self) -> usize {
        let head = self.chain.head_height();
        let due: Vec<(Vec<u8>, super::atlas::Claim)> = self.atlas.claims.scoped().filter(|(_, c)| head >= c.until).map(|(k, c)| (k, c.clone())).collect();
        let mut paid = 0;
        for (k, c) in due {
            if self.treasury.release_split(&claim_key(&k), &c.parts, (c.at, c.receipt)).is_ok() {
                self.atlas.claims.remove(&k);
                paid += 1;
            }
        }
        if paid > 0 {
            self.mark_persist();
        }
        paid
    }

    /// The claims standing on `who`'s stake, per lane; `None` past the
    /// carrier.
    pub fn claims_on(&self, who: &[u8; 32]) -> Option<[u64; 4]> {
        let Some(fiber) = self.atlas.claims.fiber_at(who) else { return Some([0; 4]) };
        let mut total = [0u64; 4];
        for (_, c) in fiber {
            for (_, mu) in &c.parts {
                total = crate::core::lanes::lanes_add(total, *mu)?;
            }
        }
        Some(total)
    }

    /// 78.154 (4) — A STAKE ACT: the party's signature, its nonce unspent,
    /// a nonzero vector; a stake moves it from the balance (whole or
    /// refused), an unstake back — only with no claim standing on the stake.
    pub fn stake(&mut self, s: &SignedStake) -> Result<[u64; 4], StakeRefusal> {
        let a = &s.act;
        if !crate::court::identity::verify(&a.who, &a.signed_bytes(), &s.sig) {
            return Err(StakeRefusal::Unsigned);
        }
        let mut spent = a.who.to_vec();
        spent.extend_from_slice(&a.nonce);
        if self.atlas.spent.contains(&spent) {
            return Err(StakeRefusal::Spent);
        }
        if a.mu.iter().all(|m| *m == 0) {
            return Err(StakeRefusal::Nothing);
        }
        let staked = self.treasury.stake_of(&a.who).map_or([0; 4], |s| s);
        let anchor = self.anchor();
        match a.unstake {
            false => {
                let short = crate::economy::shortfall(&a.mu, &self.treasury.balance_mu(&a.who));
                let Some(bond) = crate::core::lanes::lanes_add(staked, a.mu) else { return Err(StakeRefusal::Short) };
                if !crate::economy::lanes_short(&short).is_empty() || self.treasury.enroll(a.who, bond, anchor) != a.mu {
                    return Err(StakeRefusal::Short);
                }
            }
            true => {
                if self.atlas.claims.fiber_at(&a.who).is_some_and(|mut f| f.next().is_some()) {
                    return Err(StakeRefusal::Claimed);
                }
                let short = crate::economy::shortfall(&a.mu, &staked);
                if !crate::economy::lanes_short(&short).is_empty() || self.treasury.unstake(a.who, a.mu, anchor) != a.mu {
                    return Err(StakeRefusal::Short);
                }
            }
        }
        self.atlas.spent.insert(spent, ());
        self.mark_persist();
        Ok(self.treasury.stake_of(&a.who).map_or([0; 4], |s| s))
    }

    /// 78.154 (4) — A COORDINATE RETRIEVAL (TU-6 PINPOINT): the receipts this
    /// court holds at a final point's tick, the receipts' read cost — one unit of
    /// information per receipt, on H, at the read price — drawn from
    /// `staker`'s stake as a claim held to the window. `nonce` names the
    /// claim.
    pub fn retrieve(&mut self, staker: [u8; 32], p: &TPoint, fin: Option<TPoint>, nonce: [u8; 32]) -> Result<Vec<KStepReceipt>, Unretrieved> {
        if !self.final_in_view(p, fin) {
            return Err(Unretrieved::NotFinal);
        }
        let read = self.pinpoint(p);
        let units = u64::try_from(read.len()).map_err(|_| Unretrieved::Overflow)?;
        let price = self.read_price().ok_or(Unretrieved::Overflow)?;
        let fee = [0, 0, 0, units.checked_mul(price[3]).ok_or(Unretrieved::Overflow)?];
        let staked = self.treasury.stake_of(&staker).map_or([0; 4], |s| s);
        if !crate::economy::lanes_short(&crate::economy::shortfall(&fee, &staked)).is_empty() {
            return Err(Unretrieved::StakeShort);
        }
        let mut id = b"retrieve".to_vec();
        p.put(&mut id);
        id.extend_from_slice(&nonce);
        let id = crate::chronometry::payload_hash(&id);
        let Some(head) = self.chain.head() else { return Ok(read) };
        let head = head.clone();
        if self.atlas.claims.contains(&[staker.as_slice(), id.as_slice()].concat()) || self.draw_claim(staker, &id, fee, &head).is_none() {
            return Err(Unretrieved::Overflow);
        }
        self.mark_persist();
        Ok(read)
    }

    /// 78.154a — A FOLD'S CONNECTION TAX, read before it seals: the flow
    /// declared under `CONNECTION_TAX` on the reserve's own fold — a sink on
    /// every lane it moves, the reserve holding it — and none on a carried
    /// line. The μ it pays, zero where none is declared.
    pub(crate) fn connection_tax(&self, fold: &TierFold) -> Result<[u64; 4], TaxRefusal> {
        fn carried_declares(f: &TierFold) -> bool {
            f.children.iter().any(|c| match &c.child {
                Child::Carried(line) => line.fold.declared.iter().any(|(who, _)| *who == CONNECTION_TAX) || carried_declares(&line.fold),
                Child::Sealed(_) => false,
            })
        }
        if carried_declares(fold) {
            return Err(TaxRefusal::Carried);
        }
        let Some((_, lanes)) = fold.declared.iter().find(|(who, _)| *who == CONNECTION_TAX) else { return Ok([0; 4]) };
        let mut mu = [0u64; 4];
        for (lane, (m, v)) in mu.iter_mut().zip(lanes).enumerate() {
            if *v > 0 {
                return Err(TaxRefusal::Source { lane });
            }
            *m = u64::try_from(v.unsigned_abs()).map_err(|_| TaxRefusal::Source { lane })?;
        }
        if !crate::economy::lanes_short(&crate::economy::shortfall(&mu, &self.treasury.balance_mu(&fold.tier))).is_empty() {
            return Err(TaxRefusal::Short);
        }
        Ok(mu)
    }

    /// 78.154a — THE CONNECTION TAX PAID, the fold sealed at `anchor`: from
    /// the reserve into this court's recycled pool.
    pub(crate) fn pay_connection_tax(&mut self, tier: [u8; 32], mu: [u64; 4], anchor: crate::economy::Anchor) {
        if mu.iter().any(|m| *m > 0) && self.treasury.recycle(tier, self.identity, mu, anchor).is_ok() {
            self.mark_persist();
        }
    }
}
