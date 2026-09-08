// 26, law 7 — THE LAPSE: what a docket that ran out of time releases.

use crate::cognition::*;
use crate::transducer::address::Address;

impl Kernel {
    /// 47.5 / 5.5: a listing whose standing height is an aperture of
    /// admitted receipts behind the head releases its draw. Escrow returns;
    /// nothing is slashed. Λ is aperture_cycle (the measured reserve ratio).
    pub(crate) fn lapse_docket(&mut self) {
        let now = self.height();
        let lambda = crate::physics::aperture_cycle() as u64;
        let mut lapse: Vec<crate::transducer::address::Address> = Vec::new();
        for (addr, _) in self.pending.fiber(crate::network::nerve::ROOT) {
            let Some(&h0) = self.standing_h.get(&addr) else {
                continue;
            };
            if now.saturating_sub(h0) >= lambda {
                lapse.push(Address::from(addr));
            }
        }
        let me = self.identity;
        for addr in lapse {
            let n = now.saturating_sub(self.standing_h.get(&addr).copied().unwrap_or(now));
            eprintln!(
                "  court citation lapsed {:02x}{:02x}… after {n} receipts",
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
