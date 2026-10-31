// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-ECONOMY §5.2 — THE COMPUTE LEASE on this court: escrow against a
// provider's capacity, released by proof.
//
//! A payer opens a lease (`ACT_WALLET` sub-kind 3, signed): `mu` of its
//! balance is HELD under the lease's name for one provider, over a list of
//! jobs (task citations), until the court's height passes `opened + window`.
//! Each time the court verifies the provider's solve of a leased job
//! (`pay_solve`, first closure or again), the lease pays the provider that
//! solve's measured discharge out of the hold, per lane, capped by what the
//! hold has left — a transfer of value that exists, never a mint — and the
//! job is done. When the jobs are done, the hold is spent, the window
//! passes (`lapse_leases`, on the court stroke) or the payer closes it
//! (sub-kind 4, signed), what the hold has left returns to the payer.
//!
//! Compute is paid as its results verify and never for cycles spent: the
//! lease reserves value against capacity, and the court's replay releases
//! it. A job is under at most one open lease per provider, so a verified
//! solve pays exactly one lease.

use super::Court;
use crate::anastome::dialogue::lease::{LeaseRefusal, SignedClose, SignedLease};
use crate::chronometry::Stamp;
use crate::transducer::address::{put_key_count, take_key_count};

/// A LEASE AS THIS COURT HOLDS IT: its payer and provider, the jobs not yet
/// paid, the height its window closes at, and the act it opened at. What it
/// holds is the treasury's hold under [`hold_key`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Lease {
    pub(crate) payer: [u8; 32],
    pub(crate) provider: [u8; 32],
    pub(crate) jobs: Vec<[u8; 32]>,
    pub(crate) until: u64,
    pub(crate) at: Stamp,
}

impl Lease {
    pub(crate) fn put(&self, v: &mut Vec<u8>) {
        v.extend_from_slice(&self.payer);
        v.extend_from_slice(&self.provider);
        put_key_count(v, self.jobs.len());
        for j in &self.jobs {
            v.extend_from_slice(j);
        }
        v.extend_from_slice(&self.until.to_le_bytes());
        self.at.put_canonical(v);
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Lease> {
        use crate::anastome::dialogue::atlas::{key32, u64_at};
        let payer = key32(b, p)?;
        let provider = key32(b, p)?;
        let jobs = (0..take_key_count(b, p)?).map(|_| key32(b, p)).collect::<Option<Vec<_>>>()?;
        let until = u64_at(b, p)?;
        Some(Lease { payer, provider, jobs, until, at: Stamp::take_canonical(b, p)? })
    }
}

/// The treasury key a lease's hold stands under.
pub(crate) fn hold_key(lease: &[u8; 32]) -> Vec<u8> {
    let mut k = b"lease ".to_vec();
    k.extend_from_slice(lease);
    k
}

impl Court {
    /// THE LEASE OF `provider` NAMING `job`, if one is open: by its name and
    /// terms, one keyed read of the job index. One per provider and job
    /// (`open_lease` refuses a second).
    fn lease_of(&self, provider: &[u8; 32], job: &[u8; 32]) -> Option<([u8; 32], Lease)> {
        let id = *self.atlas.lease_jobs.get(&super::atlas::Atlas::lease_job_key(provider, job))?;
        Some((id, self.atlas.leases.get(&id)?.clone()))
    }

    /// MU-ECONOMY §5.2 — A LEASE OPENED: signed by its payer, its nonce
    /// unspent, some value, a window, distinct jobs, a provider that is not
    /// the payer, no job already leased to that provider; `mu` held from the
    /// payer's balance under the lease's name. Answers the lease's name and
    /// what it holds.
    pub fn open_lease(&mut self, s: &SignedLease) -> Result<([u8; 32], [u64; 4]), LeaseRefusal> {
        let a = &s.act;
        let (Some(bytes), Some(id)) = (a.signed_bytes(), a.id()) else { return Err(LeaseRefusal::Malformed) };
        if !crate::court::identity::verify(&a.payer, &bytes, &s.sig) {
            return Err(LeaseRefusal::Unsigned);
        }
        let mut spent = a.payer.to_vec();
        spent.extend_from_slice(&a.nonce);
        if self.atlas.spent.contains(&spent) {
            return Err(LeaseRefusal::Spent);
        }
        let distinct = a.jobs.iter().enumerate().all(|(i, j)| !a.jobs.iter().take(i).any(|k| k == j));
        if a.mu.iter().all(|m| *m == 0) || a.window == 0 || a.jobs.is_empty() || !distinct {
            return Err(LeaseRefusal::Malformed);
        }
        if a.payer == a.provider {
            return Err(LeaseRefusal::Oneself);
        }
        if a.jobs.iter().any(|j| self.lease_of(&a.provider, j).is_some()) {
            return Err(LeaseRefusal::Leased);
        }
        let Some(until) = self.height().checked_add(a.window) else { return Err(LeaseRefusal::Malformed) };
        let anchor = self.anchor();
        if self.treasury.hold(a.payer, &hold_key(&id), a.mu, anchor).is_err() {
            return Err(LeaseRefusal::Short);
        }
        self.atlas.hold_lease(id, Lease { payer: a.payer, provider: a.provider, jobs: a.jobs.clone(), until, at: anchor.0 });
        self.atlas.spent.insert(spent, ());
        self.mark_persist();
        Ok((id, a.mu))
    }

    /// MU-ECONOMY §5.2 — A LEASE CLOSED BY ITS PAYER: signed, its nonce
    /// unspent, the lease open and the payer's; what it holds returns to the
    /// payer. Answers the lease's name and what returned.
    pub fn close_lease(&mut self, s: &SignedClose) -> Result<([u8; 32], [u64; 4]), LeaseRefusal> {
        let a = &s.act;
        if !crate::court::identity::verify(&a.payer, &a.signed_bytes(), &s.sig) {
            return Err(LeaseRefusal::Unsigned);
        }
        let mut spent = a.payer.to_vec();
        spent.extend_from_slice(&a.nonce);
        if self.atlas.spent.contains(&spent) {
            return Err(LeaseRefusal::Spent);
        }
        if self.atlas.leases.get(&a.lease).is_none_or(|l| l.payer != a.payer) {
            return Err(LeaseRefusal::UnknownLease);
        }
        let returned = self.end_lease(&a.lease);
        self.atlas.spent.insert(spent, ());
        self.mark_persist();
        Ok((a.lease, returned))
    }

    /// A LEASE ENDED: its record leaves, and what its hold has left returns
    /// to the payer. Answers what returned.
    fn end_lease(&mut self, lease: &[u8; 32]) -> [u64; 4] {
        let _ = self.atlas.drop_lease(lease);
        let anchor = self.anchor();
        match self.treasury.refund_held(&hold_key(lease), anchor) {
            Ok(mu) => mu,
            Err(_) => [0; 4],
        }
    }

    /// MU-ECONOMY §5.2 — A VERIFIED SOLVE PAYS ITS LEASE: where `provider`
    /// solved `job` under an open lease, the lease pays the provider the
    /// solve's measured `discharge` out of its hold, per lane, capped by what
    /// the hold has left, and the job is done. A lease whose jobs are done or
    /// whose hold is spent ends, the rest returning to its payer. Answers
    /// what the lease paid.
    pub(crate) fn pay_lease(&mut self, provider: [u8; 32], job: &[u8; 32], discharge: [u64; 4], at: crate::economy::Anchor) -> [u64; 4] {
        let Some((id, mut lease)) = self.lease_of(&provider, job) else { return [0; 4] };
        if self.height() >= lease.until {
            let _ = self.end_lease(&id);
            return [0; 4];
        }
        let paid = match self.treasury.draw_held(&hold_key(&id), provider, discharge, at) {
            Ok(mu) => mu,
            Err(_) => [0; 4],
        };
        // The job is done: its index entry leaves, the rest of the lease
        // stands, or the lease ends where nothing is left to pay or do.
        let _ = self.atlas.drop_lease(&id);
        lease.jobs.retain(|j| j != job);
        let spent = self.treasury.held_of(&hold_key(&id)).is_none();
        match lease.jobs.is_empty() || spent {
            true => {
                let _ = self.end_lease(&id);
            }
            false => {
                self.atlas.hold_lease(id, lease);
            }
        }
        if paid.iter().any(|&m| m > 0) {
            self.acts.pay("lease payments", paid);
        }
        self.mark_persist();
        paid
    }

    /// MU-ECONOMY §5.2 — THE LEASES WHOSE WINDOW HAS PASSED end on the
    /// court stroke, what each holds returning to its payer. Answers how
    /// many ended.
    pub(crate) fn lapse_leases(&mut self) -> usize {
        let h = self.height();
        let due: Vec<[u8; 32]> = self
            .atlas
            .leases
            .scoped()
            .filter(|(_, l)| h >= l.until)
            .filter_map(|(k, _)| <[u8; 32]>::try_from(k.as_slice()).ok())
            .collect();
        for id in &due {
            let _ = self.end_lease(id);
        }
        if !due.is_empty() {
            self.mark_persist();
        }
        due.len()
    }

    /// THE OPEN LEASES, each by its name with its terms and what it holds:
    /// a reading.
    pub fn leases(&self) -> Vec<LeaseReading> {
        self.atlas
            .leases
            .scoped()
            .filter_map(|(k, l)| {
                let id = <[u8; 32]>::try_from(k.as_slice()).ok()?;
                let held = self.treasury.held_of(&hold_key(&id)).map_or([0; 4], |h| h.mu);
                Some(LeaseReading { id, payer: l.payer, provider: l.provider, jobs: l.jobs.len(), until: l.until, held })
            })
            .collect()
    }
}

/// ONE OPEN LEASE AS READ: its name, payer and provider, the jobs not yet
/// paid, the height its window closes at, and what its hold has left.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeaseReading {
    pub id: [u8; 32],
    pub payer: [u8; 32],
    pub provider: [u8; 32],
    pub jobs: usize,
    pub until: u64,
    pub held: [u64; 4],
}
