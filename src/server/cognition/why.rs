// 78.92 (2) — WHY AN ACT ON THIS COURT'S LEDGER STANDS: the act at the
// coordinates asked, and every word it names with the reasons read off the
// word this court holds (`sentence::Reasons::of`, the one reader the speaker
// read with).

use super::*;
use crate::anastome::dialogue::why::{head_of, Named, Refusal, Why, WhyAsk};
use crate::chronometry::Cite as Cited;

impl Court {
    /// THE ACT AT THE COORDINATES ASKED, AND WHY IT STANDS. A word the act
    /// names is located by the stamp that admitted it; one this court no
    /// longer holds is counted as unheld, never answered with reasons it
    /// cannot read. `None` where this court holds no act there.
    pub fn why(&self, ask: WhyAsk) -> Option<Why> {
        // THE HOT CHAIN, THEN THE ARCHIVE: a fold archives every act before
        // its root, and an act asked about stands where it stands whether or
        // not a fold has passed it — read through the one reader the
        // back-fill pages by (`receipts_page`), bounded to exactly that place.
        let own = self.chain.signer();
        let (vk, p) = match ask {
            WhyAsk::Own(p) => (own, p),
            WhyAsk::Of(s) => (s.vk, s.place()),
        };
        let r = match self.chain.at(&vk, p) {
            Some(r) => r,
            None => self.archived_at(&vk, p)?,
        };
        if let WhyAsk::Of(s) = ask {
            if r.stamp != s {
                return None;
            }
        }
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

    /// ONE ARCHIVED ACT AT ITS PLACE on a signer's line, through the page
    /// reader: the page from the place just before it up to it, one act long.
    /// `None` where no act stands there in what this court holds.
    fn archived_at(&self, signer: &[u8; 32], p: crate::chronometry::Place) -> Option<crate::chronometry::KStepReceipt> {
        use crate::chronometry::Place;
        let before = match (p.k, p.i) {
            (_, i) if i > 0 => Some(Place { k: p.k, i: i - 1 }),
            (k, _) if k > 0 => Some(Place { k: k - 1, i: u64::MAX }),
            _ => None,
        };
        let (_, bytes) = Court::receipts_frame()?;
        match self.receipts_page(signer, before, p, 1, bytes).as_slice() {
            [r] if r.stamp.place() == p => Some(r.clone()),
            _ => None,
        }
    }
}
