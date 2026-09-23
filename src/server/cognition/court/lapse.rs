// 26, law 7 — THE LAPSE: what a docket that ran out of time releases.

use crate::cognition::*;
use crate::transducer::address::Address;

impl Kernel {
    /// 47.5 / 5.5: a listing whose standing height is Λ admitted receipts
    /// behind the head releases its draw. Escrow returns; nothing is
    /// slashed.
    ///
    /// Λ IS MEASURED TRANSPORT, NOT A CONSTANT. 47.5 fixes it: "`Λ` is
    /// `from_transport(rtt_floor, aperture_cycle())` on that chart in
    /// strokes, never a constant", and rule 5 names the same shape as the
    /// thing a free constant must become. This read `aperture_cycle()`
    /// alone -- a number that had nothing to do with how long the home it
    /// waits on actually takes to answer, so a citation on a fast wire
    /// waited as long as one on a slow one, and no wire's measurement
    /// moved it. The transport is not measured in this kernel and cannot
    /// be: the caller holds the mesh. `None` -- an unmeasured round trip
    /// -- does NOT lapse, which is what `transport_deadline` already says
    /// of an unmeasured RTT and what rule 11 says of every unread value.
    ///
    /// Λ IS PER CHART (47.5, Stage 11 I20). The caller is asked with the
    /// listing's chart and magnitude — `invariant_of` and `magnitude()` of
    /// the pending element, which only this kernel holds — and this
    /// court's own room, so it can answer with the Cover's home for that
    /// chart (16.11: the fibre on the occupied lanes, then room, then
    /// identity) and the round trip it measured to that home. It was asked
    /// with the listing's address and resolved the home by the router's
    /// XOR distance on the address bytes, which is not where a chart is
    /// homed and never named a hypha the transport had measured.
    ///
    /// THE LAPSE IS A CIRCUIT, NOT A SWEEP (45.7, rule 10). It read the
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
    /// left off" and the docket is read whole across strokes rather than
    /// all at once in one. The extent each lapse reports is the one read
    /// when it was measured, not a second read guarded by `unwrap_or(now)`
    /// — a default standing in for a height (checklist (i)).
    /// THE SWEEP TAKES NO BUDGET FROM ITS CALLER. It was handed
    /// `court_stroke`'s one stroke, after the docket circuit had already
    /// spent from it, so on a busy court the eviction circuit reached no
    /// listing at all — a book whose only sink competes with whatever ran
    /// before it in the same function has no sink.
    pub(crate) fn lapse_docket(
        &mut self,
        lambda: impl Fn(
            &crate::transducer::stalk::Invariant,
            &[u64; 4],
            &[u64; 4],
        ) -> crate::physics::Lambda,
    ) {
        let now = self.height();
        // A room that cannot be read is a broken count, and no listing is
        // lapsed against one.
        let Some(room) = self.lane_room() else {
            return;
        };
        let room: [u64; 4] = std::array::from_fn(|l| room[l] as u64);
        let mut lapse: Vec<(Address, u64)> = Vec::new();
        // Listings whose horizon is wider than the ledger counts. Reported
        // once per sweep, never per listing.
        let mut beyond = 0usize;
        let mut last: Option<Vec<u8>> = None;
        // HOW DEEP IS THE BACKLOG? Read once for the pass, against the
        // high watermark, and answered as the DEPTH — what the book holds
        // beyond what it is entitled to — not as a yes or no.
        let held = self.pending.len();
        // The branch is what makes the subtraction exact: nothing is
        // clamped, and a book under the low watermark has no depth.
        let depth = match held > crate::physics::BOOK_LOW {
            true => held - crate::physics::BOOK_LOW,
            false => 0,
        };
        let over = held > crate::physics::book_ceiling();
        // THE SWEEP HAS ITS OWN QUOTA, AND THE QUOTA IS THE BACKLOG.
        //
        // `court_stroke` built ONE stroke and handed it to the docket circuit
        // first, then here, then to the fills, so on a busy court the circuit
        // spent it and the eviction circuit reached no listing at all. Giving
        // the sweep its own stroke removed the COMPETITION and left the
        // RATE: three spends a pass against an inflow of some 250 listings
        // a minute, which loses by two orders of magnitude. Measured on the
        // fleet at epoch 11 — hypha b climbed 1701 → 1920 → 2888 while a
        // and c held at zero — and that is the same fixed-scalar trap that
        // choked extraction and the wire frame.
        //
        // A sink sized by a constant cannot match a source it does not
        // read. Over the watermark the sweep is sized by the DEPTH it has
        // to clear, so one pass drains to `BOOK_LOW` rather than shaving
        // three off the top and losing ground; under it, one stroke, which
        // is all a book in flight needs.
        let mut sweep = match over {
            true => self.physics.stroke_n(depth),
            false => self.physics.stroke(),
        };
        let budget = &mut sweep;
        {
            let from = self.lapse_cursor.clone();
            for (addr, (listed, _)) in self.pending.cursor(from.as_deref(), budget) {
                last = Some(addr.clone());
                let Some(&h0) = self.standing_h.get(&addr) else {
                    continue;
                };
                let chart = crate::transducer::stalk::invariant_of(listed);
                // THE BOOK HAS A HORIZON OF ITS OWN, and it does not
                // depend on a packet. This read `let Some(lam) = … else {
                // continue }`: where no round trip had been measured to
                // the hypha homing the chart, Λ was unmeasured and the
                // listing stood forever. That is a circular dependency,
                // not a conservation law — a court that cannot reach a
                // peer cannot measure a transport, so it cannot evict, so
                // the book becomes an append-only sink. Measured on the
                // fleet at epoch 10: 250 listings a minute at ~26 KB each,
                // nothing ever leaving, until the node ran out of memory.
                //
                // Λ REMAINS THE BOUND WHERE IT IS MEASURED — an unmeasured
                // transport is still not a fast one (rule 11), and nothing
                // here lapses a citation early against a number the court
                // does not hold. What the unmeasured case gets instead is
                // the court's OWN aperture, read off its own clock: a
                // listing standing longer than that against `standing_h`
                // is past this court's causal horizon whatever the network
                // is doing, and it leaves as the others do.
                //
                // AND ABOVE THE CEILING THE HORIZON COLLAPSES. A book past
                // what this node's memory can carry evicts whatever has
                // stood at all, oldest reached first as the circuit reads it:
                // the manifold is finite and the book is not entitled to
                // exceed it whatever Λ says.
                // AND THE TWO REFUSING ARMS ARE TWO FACTS (law 5).
                // `Unmeasured` is no horizon read, and falls back to this
                // court's own aperture as above. `BeyondTheLedger` is a
                // horizon WIDER than any extent the ledger counts, which is
                // arithmetic and not a failure: no listing has waited that
                // long, so nothing lapses on it.
                let lam = match (over, lambda(&chart, &listed.magnitude(), &room)) {
                    (true, _) => 0,
                    (false, crate::physics::Lambda::Strokes(l)) => l,
                    (false, crate::physics::Lambda::Unmeasured) => {
                        crate::physics::aperture_cycle() as u64
                    }
                    (false, crate::physics::Lambda::BeyondTheLedger) => {
                        beyond += 1;
                        continue;
                    }
                };
                // The stamp is a height this court has passed, so the
                // extent is exact; a stamp in the future measures no
                // extent and nothing lapses on it.
                let Some(extent) = now.checked_sub(h0) else {
                    continue;
                };
                if extent >= lam {
                    lapse.push((Address::from(addr), extent));
                }
            }
        }
        if let Some(k) = last {
            self.lapse_cursor = Some(k);
        }
        let me = self.identity;
        // ONE LINE PER SWEEP, NOT PER LISTING. A lapse was exceptional when
        // it was written — a citation nobody answered inside Λ — and it is
        // routine now that the book has a horizon: 105,389 of these in
        // forty minutes, 58% of everything the fleet said, and the journal
        // they fill is 18 GiB of the disk. What a reader needs from a
        // sweep is that it ran and how far it reached, which is one line;
        // WHICH listings left is the field's to say, and it says it.
        if let [(addr, extent), ..] = lapse.as_slice() {
            eprintln!(
                "  court citations lapsed {} from {:02x}{:02x}… after {extent} receipts",
                lapse.len(),
                addr[0],
                addr[1]
            );
        }
        // 34.2: a horizon the ledger cannot count is a reading, and it is
        // said once per sweep so the two refusing arms are told apart on
        // the line as well as in the type.
        if beyond > 0 {
            eprintln!("  court lambda beyond the ledger listings {beyond} cycle {}", now);
        }
        for (addr, _) in lapse {
            let _ = self.treasury.revoke_award(&addr, false);
            self.unlist(&addr, false);
            // 45.1: the release is this court's own act -- it returns an
            // escrow and unlists a draw -- so it is a step of this court's
            // clock, authored by this court. A lapse that sealed nothing
            // moved the book at no position, and the height a later lapse
            // measures its aperture against is a count of receipts (47.5):
            // a court whose lapses are not receipted stops advancing the
            // very quantity it lapses on.
            self.append_step(crate::chronometry::ActKind::Lapse, &addr, me);
        }
    }

    /// THE LISTINGS A SIMPLEX IS OVER (13.9, 47.3; R6): the open listings
    /// on the chart. A simplex's vertices are vantages — hyphas — and no
    /// speaker's index gates the reading: read through an index keyed by
    /// voice, a hypha vertex measures zero, so no section stands a vertex,
    /// no edge has volume, and nothing fills though the chart holds two
    /// sections. The vantages are the simplex's, not the measure's, so the
    /// simplex's vertex list is not an argument here.
    pub(crate) fn listings_on(
        &self,
        inv: &crate::transducer::stalk::Invariant,
    ) -> crate::network::nerve::Keyed<()> {
        match self.chart_listings.get(&inv.key()) {
            Some(on_chart) => on_chart.meet(&self.open),
            None => crate::network::nerve::Keyed::new(),
        }
    }
}
