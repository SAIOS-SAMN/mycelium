// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.183 §7.5 (ruled 2026-11-13) — THE METABOLIC RATE PER LANE, read over a
// bell span: the entropic and syntropic events each lane booked, the faces
// they give, and the share of the lane's holding a turn may draw.
//
//! The two-stroke metabolism moves the reserve two ways per lane: ENTROPIC,
//! what it gives to expansion (the reserve's ledger, `drawn`), and
//! SYNTROPIC, what comes back to it (`restored`). Both are in reads, the
//! holding's own unit. Over one bell span, the span's own amounts `E_l` and
//! `Y_l` are the lane's metabolic rate. The event counters `torsion` and
//! `syntropic` do not carry it: on the standing mesh a court's span booked
//! none of either while its reserve moved by millions of reads
//! (2026-11-13). Its
//! faces are the held share `c_l = Y_l/(E_l + Y_l)` (cohesion, what binds)
//! and the open share `d_l = E_l/(E_l + Y_l)` (fluidity), with
//! `c_l + d_l = 1`. A turn may draw `d_l · M_l` of the lane's holding `M_l`:
//! a lane running entropic draws more and is pushed toward consolidation,
//! one running syntropic draws less, and the throat `E_l = Y_l` releases
//! half. A lane that booked nothing in the span draws nothing (an idle
//! court spends nothing; MU-EQUATIONS §7.6). Every reading is exact and
//! per lane, never summed (law 5).

use super::Physics;

/// ONE LANE'S RATE OVER A BELL SPAN: the events it booked and the draw its
/// open share sizes. `drawn + kept = holding` exactly: the integer part of
/// `d_l · M_l` is drawn, and what the division leaves stays held.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LaneRate {
    /// What the span drew from the lane, `E_l`, in reads.
    pub entropic: u64,
    /// What the span restored to the lane, `Y_l`, in reads.
    pub syntropic: u64,
    /// The lane's holding at the span's close, `M_l`.
    pub holding: u64,
    /// What a turn may draw: `⌊E_l · M_l / (E_l + Y_l)⌋`, zero where the
    /// span booked nothing.
    pub drawn: u64,
}

/// THE RATE OVER THE BELL SPANS, PER LANE: the marks the last span closed
/// at, and the bell it closed on.
#[derive(Clone, Debug, Default)]
pub struct Rate {
    drawn_mark: [u64; 4],
    restored_mark: [u64; 4],
    bell: Option<u64>,
    last: Option<[LaneRate; 4]>,
}

impl Rate {
    /// THE SPAN CLOSED AT BELL `h`: where the bell moved since the last
    /// close, each lane's events since the marks are read against its
    /// holding, the marks move, and the four lane rates are answered. `None`
    /// where the bell has not moved (the span is still open) or where a count
    /// stands behind its mark (a broken counter books nothing; rule 11).
    pub fn close(&mut self, physics: &Physics, h: u64) -> Option<[LaneRate; 4]> {
        if self.bell.is_some_and(|b| b >= h) {
            return None;
        }
        let holding = physics.reserve.lanes();
        let (drawn, restored, _) = physics.reserve.ledger();
        let mut out = [LaneRate::default(); 4];
        for l in 0..4 {
            let entropic = drawn[l].checked_sub(self.drawn_mark[l])?;
            let syntropic = restored[l].checked_sub(self.restored_mark[l])?;
            out[l] = LaneRate { entropic, syntropic, holding: holding[l], drawn: open_draw(entropic, syntropic, holding[l])? };
        }
        self.drawn_mark = drawn;
        self.restored_mark = restored;
        self.bell = Some(h);
        self.last = Some(out);
        Some(out)
    }

    /// The rates of the last span closed; `None` before the first.
    pub fn last(&self) -> Option<[LaneRate; 4]> {
        self.last
    }
}

/// `⌊E · M / (E + Y)⌋`: the open share of the holding, zero where the span
/// moved nothing. `None` where the product passes the carrier (counted by
/// the caller as unread, never wrapped; rule 11).
pub fn open_draw(entropic: u64, syntropic: u64, holding: u64) -> Option<u64> {
    let events = entropic.checked_add(syntropic)?;
    if events == 0 {
        return Some(0);
    }
    let product = u128::from(entropic).checked_mul(u128::from(holding))?;
    u64::try_from(product / u128::from(events)).ok()
}
