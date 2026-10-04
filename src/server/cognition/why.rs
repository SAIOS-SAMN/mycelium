// 78.92 (2) — WHY AN ACT ON THIS COURT'S LEDGER STANDS: the act at the
// coordinates asked, and every word it names with the reasons read off the
// word this court holds (`sentence::Reasons::of`, the one reader the speaker
// read with).

use super::*;
use crate::anastome::dialogue::why::{head_of, Named, Refusal, Why, WhyAsk};
use crate::chronometry::Cite as Cited;

impl Kernel {
    /// THE ACT AT THE COORDINATES ASKED, AND WHY IT STANDS. A word the act
    /// names is located by the stamp that admitted it; one this court no
    /// longer holds is counted as unheld, never answered with reasons it
    /// cannot read. `None` where this court holds no act there.
    pub fn why(&self, ask: WhyAsk) -> Option<Why> {
        let r = match ask {
            WhyAsk::Own(p) => self.chain.at_place(p)?,
            WhyAsk::Of(s) => self.chain.node(&s)?,
        };
        let mut named = Vec::new();
        let mut unheld = 0u64;
        let mut refused = Vec::new();
        for c in &r.cites {
            // 78.93 B0 — a refusal the act records, by its kind and chart.
            if let Cited::Refusal { chart, kind } = c {
                refused.push(Refusal { kind: *kind, chart: head_of(chart.key().as_bytes()) });
                continue;
            }
            let Cited::Word { at, .. } = c else { continue };
            let held = self.word_admitted_at(at).and_then(|a| Some((a.clone(), self.named_word(a.as_bytes())?)));
            let Some((addr, word)) = held else {
                unheld += 1;
                continue;
            };
            named.push(Named {
                head: head_of(addr.as_bytes()),
                admitted: *at,
                task: self.speech_task.get(addr.as_bytes()).map(|t| head_of(t.as_bytes())),
                reasons: crate::transducer::sentence::Reasons::of(word),
            });
        }
        Some(Why { at: r.stamp, kind: r.kind, mu: r.mu, named, unheld, refused })
    }
}
