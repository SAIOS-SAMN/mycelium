// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.184 (ruled 2026-11-13), MU-EQUATIONS §6.13 — μ CREATED AT A MANIFEST
// against the elemental rows of the nuclei it carries, and granted in
// proportion to the kernel's ledger alignment.

use super::Court;
use crate::anastome::dialogue::endowment::Nuclei;
use crate::economy::elemental::{row, RowRefusal};
use crate::economy::modes::Modes;
use crate::physics::laws::clauses::{register, ClauseReading};
use num_rational::BigRational as Q;

impl Court {
    /// THE CREATION: `Σ n · row(Z, Q)` over the manifest's nuclei, each row
    /// the coupled nucleon dipole's (`economy::elemental::row`), exact.
    pub(crate) fn elemental_creation(&self, nuclei: &Nuclei) -> Result<Modes, RowRefusal> {
        let mut reads = self.physics.clone().stroke_n(usize::MAX);
        let mut out = Modes::new();
        for (k, n) in nuclei {
            let r = row(&Q::from_integer(k.z.into()), &Q::from_integer(k.q.into()), &mut reads)?;
            let count = Q::from_integer(num_bigint::BigInt::from(n.clone()));
            out.add(&r.scaled(&count).ok_or(RowRefusal::Unread)?);
        }
        Ok(out)
    }

    /// THE ALIGNMENT SHARE of `domain`'s sitting reserve: the clauses of the
    /// register it reads Converses on, over the clauses the register serves
    /// (`ACT_LAWS` mode 9). `None` where the domain has no reserve or its
    /// alignment does not read; zero where it converses on none.
    pub(crate) fn alignment_share(&mut self, domain: &[u8; 32]) -> Option<Q> {
        let reserve = self.reserve_of(domain)?;
        let served = register().ok()?;
        let mut reads = self.physics.stroke();
        let read = self.alignment(Some(reserve), &mut reads).ok()?;
        let readings = read.kernels.into_iter().find(|k| k.kernel == reserve).map(|k| k.clauses).unwrap_or_default();
        let conversed = served
            .iter()
            .filter(|r| readings.iter().any(|c| c.id == r.clause.id && matches!(c.reading, ClauseReading::Converses { .. })))
            .count();
        (!served.is_empty()).then(|| Q::new(conversed.into(), served.len().into()))
    }
}
