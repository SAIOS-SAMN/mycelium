// 26, law 7 — THE LAPSE: what a docket that ran out of time releases.

use crate::cognition::*;
use crate::transducer::address::Address;

impl Kernel {
    /// 47.5 / 5.5: a listing whose standing height is an aperture of
    /// admitted receipts behind the head releases its draw. Escrow returns;
    /// nothing is slashed. Λ is aperture_cycle (the measured reserve ratio).
    /// THE LAPSE IS AN ORBIT, NOT A SWEEP (45.7, rule 10). It walked the
    /// whole `pending` fiber with no budget, collected every expired
    /// address into a `Vec`, and then ran a second unbudgeted loop that
    /// revoked an escrow, unlisted a draw and — since I10 — sealed a
    /// receipt per address. `pending` grows with demand arriving from every
    /// peer, so on a mesh of ten thousand nodes it is one of the carriers
    /// that grows with the mesh; a stroke that meets a large expiry cohort,
    /// which is what follows any pause, did all of that work in one
    /// unbounded pass while holding the kernel.
    ///
    /// Now it spends the stroke's own `Budget` and resumes from
    /// `lapse_cursor`, so "every stroke over a carrier resumes where it
    /// left off" and the docket is walked whole across strokes rather than
    /// all at once in one. The extent each lapse reports is the one read
    /// when it was measured, not a second read guarded by `unwrap_or(now)`
    /// — a default standing in for a height (checklist (i)).
    pub(crate) fn lapse_docket(&mut self, budget: &mut crate::physics::Budget) {
        let now = self.height();
        let lambda = crate::physics::aperture_cycle() as u64;
        let mut lapse: Vec<(Address, u64)> = Vec::new();
        let mut last: Option<Vec<u8>> = None;
        {
            let from = self.lapse_cursor.clone();
            for (addr, _) in self.pending.cursor(&from, budget) {
                last = Some(addr.clone());
                let Some(&h0) = self.standing_h.get(&addr) else {
                    continue;
                };
                let extent = now.saturating_sub(h0);
                if extent >= lambda {
                    lapse.push((Address::from(addr), extent));
                }
            }
        }
        if let Some(k) = last {
            self.lapse_cursor = k;
        }
        let me = self.identity;
        for (addr, extent) in lapse {
            eprintln!(
                "  court citation lapsed {:02x}{:02x}… after {extent} receipts",
                addr[0], addr[1]
            );
            let _ = self.treasury.revoke_award(&addr, false);
            self.unlist(&addr, false);
            // 45.1: the release is this court's own act -- it returns an
            // escrow and unlists a draw -- so it is a step of this court's
            // clock, authored by this court. A lapse that sealed nothing
            // moved the book at no position, and the height a later lapse
            // measures its aperture against is a count of receipts (47.5):
            // a court whose lapses are not receipted stops advancing the
            // very quantity it lapses on.
            self.append_step(&addr, me);
        }
    }

    pub(crate) fn listings_on<A: AsRef<[u8]>>(
        &self,
        inv: &crate::transducer::stalk::Invariant,
        verts: &[A],
    ) -> crate::network::nerve::Keyed<()> {
        let Some(on_chart) = self.chart_listings.get(&inv.key()) else {
            return crate::network::nerve::Keyed::new();
        };
        let mut acc: Option<crate::network::nerve::Keyed<()>> = None;
        for v in verts {
            let Some(held) = self.spoken_by.get(v.as_ref()) else {
                return crate::network::nerve::Keyed::new();
            };
            let here = held.meet(on_chart).meet(&self.open);
            acc = Some(match acc {
                None => here,
                Some(prev) => prev.meet(&here),
            });
        }
        acc.unwrap_or_default()
    }

}
