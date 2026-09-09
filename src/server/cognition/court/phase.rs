// 45.9 — PHASE IS A READING: four coordinates computed from the receipt
// kinds over a height window and the ruled cover at the window's ends,
// never stored, announced or thresholded.
//
//! `phase_over` is a pure function of a slice of receipts and two cover
//! readings; the court writes `court phase …` once per docket orbit
//! (34.2) and keeps only the cover reading the next window starts from.
//! The regimes the words name — ooze, congruence, presentation,
//! equilibrium — are readings, and two nodes holding the same receipts
//! read the same phase (13.7).

use crate::chronometry::{ActKind, KStepReceipt};
use crate::cognition::Kernel;
use crate::network::nerve::{Keyed, ROOT};

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
    /// Growth: generators adjoined against retirements between the ends (E2).
    pub adjoined: u64,
    pub retired: u64,
    /// Expansion: axes opened and strata split between the ends (3.7).
    pub axes: u64,
    pub splits: u64,
}

impl Phase {
    /// The line 34.2 writes, once per docket orbit.
    pub fn line(&self) -> String {
        format!(
            "  court phase h {}..{} comprehension {}/{} congruence {}/{} lineages {} growth +{} -{} expansion {}/{}",
            self.h0,
            self.h1,
            self.deflated,
            self.held,
            self.sections,
            self.strata,
            self.lineages,
            self.adjoined,
            self.retired,
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
    p.sections = at_h1.sections.fiber(ROOT).filter(|(_, &n)| n > 1).count() as u64;
    let mut rehomed: Keyed<()> = Keyed::new();
    for (k0, &n0) in at_h0.strata.fiber(ROOT) {
        let n0 = n0 as u64;
        match at_h1.strata.get(&k0) {
            Some(&n1) => {
                let n1 = n1 as u64;
                p.adjoined += n1.saturating_sub(n0);
                p.retired += n0.saturating_sub(n1);
            }
            None => {
                // index: the refinements of a gone stratum, a keyed fiber
                let mut under = 0u64;
                let mut any = false;
                for (k1, &n1) in at_h1.strata.fiber(&k0) {
                    if k1.len() > k0.len() {
                        any = true;
                        under += n1 as u64;
                        rehomed.insert(&k1, ());
                    }
                }
                if any {
                    p.splits += 1;
                    p.adjoined += under.saturating_sub(n0);
                    p.retired += n0.saturating_sub(under);
                } else {
                    p.retired += n0;
                }
            }
        }
    }
    for (k1, &n1) in at_h1.strata.fiber(ROOT) {
        if at_h0.strata.contains(&k1) || rehomed.contains(&k1) {
            continue;
        }
        p.axes += 1;
        p.adjoined += n1 as u64;
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
        for (key, cx) in self.complexes.fiber(ROOT) {
            sections.insert(&key, cx.sections.len());
        }
        CoverReading { strata, sections }
    }

    /// The phase over the window since the last docket orbit (45.9),
    /// written once per orbit; the reading the next window starts from is
    /// kept, the phase is not.
    pub(crate) fn read_phase(&mut self) {
        let now = self.cover_reading();
        let receipts = self.chain.receipts();
        let k1 = receipts.len();
        if let Some((k0, at_h0)) = self.phase_mark.take() {
            let k0 = k0.min(k1);
            let h0 = k0.checked_sub(1).and_then(|i| receipts.get(i)).map_or(0, |r| r.height);
            let h1 = self.chain.head_height();
            let phase = phase_over(&receipts[k0..k1], &at_h0, &now, h0, h1);
            eprintln!("{}", phase.line());
        }
        self.phase_mark = Some((k1, now));
    }
}
