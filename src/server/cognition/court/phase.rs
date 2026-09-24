// 45.9 — PHASE IS A READING: four coordinates computed from the receipt
// kinds over a height window and the ruled cover at the window's ends,
// never stored, announced or thresholded.
//
//! `phase_over` is a pure function of a slice of receipts and two cover
//! readings; the court writes `court phase …` once per window — an
//! aperture of docket circuits (34.2, R7) — and keeps only the cover
//! reading the next window starts from.
//! The regimes the words name — ooze, congruence, presentation,
//! equilibrium — are readings, and two nodes holding the same receipts
//! read the same phase (13.7).

use crate::chronometry::{ActKind, KStepReceipt};
use crate::cognition::Kernel;
use crate::network::nerve::Keyed;

/// The four coordinates of 45.9 over one height window, as counts read off
/// the ledger and the cover. Nothing here is a state of the mesh.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Phase {
    pub h0: u64,
    pub h1: u64,
    /// Comprehension: listings a held word deflated, against listings held.
    pub deflated: u64,
    pub held: u64,
    /// Congruence: strata holding more than one section, against strata;
    /// installs reached by an independent lineage (23.9).
    pub sections: u64,
    pub strata: u64,
    pub lineages: u64,
    /// I2 — INSTALLED: ruled words entering the cover against words leaving
    /// it between the ends. It was printed as `growth` and read as
    /// adjunction; it counts installed words, a different reading from the
    /// generators a closure opened (45.9, 34.2).
    pub installed: u64,
    pub retired: u64,
    /// Expansion: axes opened and strata split between the ends (3.7).
    pub axes: u64,
    pub splits: u64,
    /// I1 — ADJOINED BY CLOSURE: the generators this court opened, read off
    /// its ruled cover at the window's end (`Kernel::adjoined`, 37.7, 45.9)
    /// — the one number that bears on the architecture, on the running
    /// court's own line.
    pub adjoined_by_closure: u64,
}

impl Phase {
    /// The line 34.2 writes, once per window of docket circuits.
    pub fn line(&self) -> String {
        format!(
            "  court phase h {}..{} comprehension {}/{} congruence {}/{} lineages {} installed +{} -{} \
             adjoined-by-closure {} expansion {}/{}",
            self.h0,
            self.h1,
            self.deflated,
            self.held,
            self.sections,
            self.strata,
            self.lineages,
            self.installed,
            self.retired,
            self.adjoined_by_closure,
            self.axes,
            self.splits
        )
    }
}

/// The ruled cover at one end of a window: stratum key → the words it
/// holds, and chart key → the sections the court holds on it. A reading
/// taken off the carriers, kept only as the start of the next window.
#[derive(Clone, Debug, Default)]
pub struct CoverReading {
    pub strata: Keyed<usize>,
    pub sections: Keyed<usize>,
}

/// THE PHASE over the receipts in `window` (heights in `h0..=h1`) and the
/// cover read at each end. A stratum present at both ends adjoins or
/// retires by its count; one gone at `h1` whose refinements stand there
/// was split, and its words are re-homed, not adjoined; one at `h1` that
/// refines nothing gone is an axis opened with its words.
pub fn phase_over(window: &[KStepReceipt], at_h0: &CoverReading, at_h1: &CoverReading, h0: u64, h1: u64) -> Phase {
    let mut p = Phase { h0, h1, ..Phase::default() };
    for r in window {
        match r.kind {
            ActKind::Settled => p.deflated += 1,
            ActKind::List => p.held += 1,
            ActKind::Install if !r.parents.is_empty() => p.lineages += 1,
            _ => {}
        }
    }
    p.strata = at_h1.strata.len() as u64;
    p.sections = at_h1.sections.scoped().filter(|(_, &n)| n > 1).count() as u64;
    let mut rehomed: Keyed<()> = Keyed::new();
    for (k0, &n0) in at_h0.strata.scoped() {
        let n0 = n0 as u64;
        match at_h1.strata.get(&k0) {
            Some(&n1) => {
                let n1 = n1 as u64;
                // A stratum MOVED ONE WAY between the two readings, and the
                // direction is what says which reading it is: one that grew
                // adjoined the difference, one that shrank retired it, and
                // one that held did neither. Taking both differences and
                // keeping whichever did not go negative says the same thing
                // through a refusal, which is not what a direction is.
                if n1 >= n0 {
                    p.installed += n1 - n0;
                } else {
                    p.retired += n0 - n1;
                }
            }
            None => {
                // index: the refinements of a gone stratum, a keyed fiber.
                // The root stratum refines into nothing — it has no
                // coordinate to descend on — so it reads as no refinement
                // rather than as every stratum there is.
                let mut under = 0u64;
                let mut any = false;
                let Some(at) = crate::network::nerve::Prefix::new(&k0) else {
                    continue;
                };
                for (k1, &n1) in at_h1.strata.fiber(at) {
                    if k1.len() > k0.len() {
                        any = true;
                        under += n1 as u64;
                        rehomed.insert(&k1, ());
                    }
                }
                if any {
                    p.splits += 1;
                    // As above: the refinements hold what the split stratum
                    // held, more, or less, and the DIRECTION says which
                    // reading it is.
                    if under >= n0 {
                        p.installed += under - n0;
                    } else {
                        p.retired += n0 - under;
                    }
                } else {
                    p.retired += n0;
                }
            }
        }
    }
    for (k1, &n1) in at_h1.strata.scoped() {
        if at_h0.strata.contains(&k1) || rehomed.contains(&k1) {
            continue;
        }
        p.axes += 1;
        p.installed += n1 as u64;
    }
    p
}

impl Kernel {
    /// The cover as it stands now: every ruled stratum with the words it
    /// holds, every chart with the sections this court holds on it.
    pub fn cover_reading(&self) -> CoverReading {
        let mut strata = Keyed::new();
        for (inv, members) in self.ruled.strata() {
            strata.insert(inv.key(), members.len());
        }
        let mut sections = Keyed::new();
        for (key, cx) in self.complexes.scoped() {
            sections.insert(&key, cx.sections.len());
        }
        CoverReading { strata, sections }
    }

    /// The phase over the window since the last reading (45.9), written
    /// once per window — the court stroke takes it when the docket circuit
    /// count crosses a multiple of `aperture_cycle()` (R7); the reading
    /// the next window starts from is kept, the phase is not.
    /// A READING OVER AN EMPTY WINDOW IS NOT A READING, and returning
    /// whether one was taken is what lets its caller pace on the same fact.
    ///
    /// The window is the receipts admitted since the last reading. When none
    /// were, `k0 == k1`, the slice is empty, and `phase_over` reports
    /// `comprehension 0/0 ... expansion 0/0` over `h N..N` — zero out of
    /// zero, which is the vacuous assertion at the instrument and not a
    /// measurement of anything. Hypha a wrote 25 598 of those in nine
    /// minutes on the fleet at e98e8a2, 62 a second, because its caller
    /// paces on CIRCUITS and a docket small enough to fit one budget completes
    /// a circuit every stroke — so the pacing quantity was counting strokes,
    /// not work. The caller's own comment records that exact failure being
    /// fixed once by adding the aperture; the aperture is keyed to the same
    /// quantity, so it moved rather than closed.
    ///
    /// The mark still advances on an empty window: the next reading covers
    /// from here, not from the last one that happened to be non-empty.
    pub(crate) fn read_phase(&mut self) -> bool {
        let now = self.cover_reading();
        // THE WINDOW IS A K RANGE ON THIS COURT'S LINE (78.24): the marks
        // are `K`, which a fold does not renumber, and each node is read
        // at its own coordinate through the line relation.
        let k1 = self.chain.next_step_index();
        let mut read = false;
        if let Some((k0, at_h0)) = self.phase_mark.take() {
            let k0 = k0.min(k1);
            if k0 != k1 {
                let window: Vec<KStepReceipt> = (k0..k1).filter_map(|k| self.chain.at_k(k).cloned()).collect();
                let h0 = k0.checked_sub(1).and_then(|k| self.chain.at_k(k)).map_or(0, |r| r.stamp.h);
                let h1 = self.chain.head_height();
                let mut phase = phase_over(&window, &at_h0, &now, h0, h1);
                phase.adjoined_by_closure = self.adjoined();
                eprintln!("{}", phase.line());
                // PH5 — and the same window handed to the observer, the
                // instrument's copy; the court keeps no phase (78.17).
                crate::observe::note_phase(crate::observe::Phase {
                    windows: 0,
                    h0: phase.h0,
                    h1: phase.h1,
                    deflated: phase.deflated,
                    held: phase.held,
                    sections: phase.sections,
                    strata: phase.strata,
                    lineages: phase.lineages,
                    installed: phase.installed,
                    retired: phase.retired,
                    adjoined_by_closure: phase.adjoined_by_closure,
                    axes: phase.axes,
                    splits: phase.splits,
                });
                read = true;
            }
        }
        self.phase_mark = Some((k1, now));
        read
    }
}
