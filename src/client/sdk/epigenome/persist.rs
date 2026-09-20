// 28 — THE EPIGENOME ON DISK: crystallized whole, and a tape that does not
// parse refused rather than partly restored.

use crate::client::sdk::*;
use crate::transducer::element::Element;

impl Epigenome {
    /// CRYSTALLIZE the genome for persistence — the permanent language only.
    /// The epigenome is memory-only. Format: `GEN3 | n u32 | (len u32 | element)…`.
    pub fn crystallize(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + self.genome.len() * 64);
        out.extend_from_slice(b"GEN3");
        out.extend_from_slice(&(self.genome.len() as u32).to_le_bytes());
        for (_, g) in self.resident.scoped() {
            let b = g.held.to_bytes();
            out.extend_from_slice(&(b.len() as u32).to_le_bytes());
            out.extend_from_slice(&b);
        }
        out
    }

    /// Restore GEN3 only. An older file is refused.
    pub fn uncrystallize(&mut self, bytes: &[u8]) -> bool {
        if bytes.len() < 8 {
            return false;
        }
        let tag = &bytes[..4];
        if tag != b"GEN3" {
            return false;
        }
        let Some(n) = bytes.get(4..8).and_then(|b| b.try_into().ok()).map(|b| u32::from_le_bytes(b) as usize) else {
            return false;
        };
        let Some(left) = bytes.len().checked_sub(8) else {
            return false;
        };
        if n > left / 5 {
            return false; // untrusted count vs remaining bytes
        }
        let mut genome = Vec::with_capacity(n);
        let mut pos = 8usize;
        let mut skipped = 0usize;
        for _ in 0..n {
            let Some(len) = bytes.get(pos..pos + 4).and_then(|b| b.try_into().ok()).map(|b| u32::from_le_bytes(b) as usize)
            else {
                break;
            };
            pos += 4;
            // A position past the end is a malformed tape; the read stops
            // there rather than reading a length against nothing.
            let Some(left) = bytes.len().checked_sub(pos) else {
                break;
            };
            if len > left {
                break;
            }
            let slice = &bytes[pos..pos + len];
            pos += len;
            let mut ep = 0usize;
            match Element::from_bytes(slice, &mut ep) {
                // The tape is decoded to a form HERE, once, as the word is
                // taken in — not on the first read of its form (L16).
                Some(e) => genome.push(crate::transducer::element::Held::of(e)),
                None => {
                    // Previous GEN1 layouts (pre-fibre-index tails, a
                    // word that will not uncrystallize) must not sink the
                    // rest: residuals are never lost.
                    skipped += 1;
                }
            }
        }
        if genome.is_empty() {
            return false;
        }
        if skipped > 0 {
            eprintln!("genome restored: {} words, {skipped} skipped", genome.len());
        }
        self.genome = crate::network::nerve::Keyed::new();
        self.resident = crate::network::nerve::Keyed::new();
        self.next_seq = 0;
        self.atlas = crate::transducer::stalk::Stalks::default();
        self.axis_index = crate::transducer::sentence::AxisIndex::new();
        // 21 step 3 (L8r): the body-chart index is derived from the genome
        // like the atlas and the axis index, and must be rebuilt with them.
        // Left empty, `on_body` answered nothing after every restart, so a
        // fleet client — which boots from its genome — could not glue until
        // it learned something new, and the index was live only in a
        // process that had never restarted.
        self.by_moduli = crate::network::nerve::Keyed::new();
        self.generators = crate::transducer::sentence::GeneratorIndex::new();
        // THE SAME ADMISSION THE LIVE PATH MAKES, once per word (`admit_word`),
        // so a restored genome and a learned one are indexed identically.
        // This used to file at the sentence's own chart while `axis_admit`
        // files where each vertex ACTS, so every word restored from disk
        // was invisible to the pivot until it happened to be re-admitted
        // live — and a fleet client boots from its genome.
        //
        // The admission ordinals are handed out in restore order, which is
        // the file's order; nothing downstream reads them as anything but
        // "later than" a watermark.
        for w in genome {
            self.admit_word(w, 0);
        }
        self.atlas_len = self.genome.len();
        true
    }

}
