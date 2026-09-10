// 23.8, 50.5 (L7) — WHAT LISTING A SENTENCE BINDS AND WRITES: the pair the
// court adjudicated as the sentence's family, and its demand vertices by kind.
//
//! Split from `ingest.rs` at the reading bound (MU-STANDARDS 1.1): the
//! commit of a reduction is there; what a listed sentence is bound to and
//! what the court says of its demands is here.

use crate::cognition::*;
use crate::transducer::element::Element;

impl Kernel {
    /// THE COURT BINDS THE PAIR IT ADJUDICATED AS THE SENTENCE'S FAMILY
    /// (23.8, L7): a percept-born sentence is replayed by G on the court's
    /// own copy of the pair — every vertex carried, every edge re-measured,
    /// the closed sub-complex read (23.1) — and its install mints what it
    /// closes there (E2). Found under L6b: bound to nothing, such a
    /// sentence replayed without its family and minted nothing.
    pub(crate) fn bind_percept_family(
        &mut self,
        germs: &[Element],
        ca: &[Vec<(i64, i64, u8)>],
        cb: &[Vec<(i64, i64, u8)>],
    ) {
        let Some(family) = percept_family(ca, cb) else {
            return;
        };
        for g in germs {
            if !g.is_sentence() {
                continue;
            }
            let addr = g.address();
            self.bind_listing_task(&addr, &addr);
            self.bind_task_family(&addr, family.clone());
        }
    }

    /// 50.5, 34.2 (L7): the demand vertices of a listed sentence, written
    /// by kind and chart as the court lists them — the market relays and
    /// prices them under the sentence; under L8 they are listed beside its
    /// closed sub-complex.
    pub(crate) fn list_demands(&mut self, sentence: &Element, voice: &[u8; 32]) {
        let Some(f) = sentence.form() else {
            return;
        };
        for (_, kind, chart, _) in f.demand_vertices() {
            let chart_key = chart.key();
            let prefix: String = chart_key.as_bytes().iter().take(2).map(|b| format!("{b:02x}")).collect();
            eprintln!(
                "  court demand listed {} chart {prefix}… from {:02x}{:02x}…",
                kind.label(),
                voice[0],
                voice[1]
            );
            self.court_demands_listed = self.court_demands_listed.saturating_add(1);
        }
    }
}
