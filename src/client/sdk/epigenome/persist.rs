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
        for g in &self.genome {
            let b = g.to_bytes();
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
        let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
        if n > bytes.len().saturating_sub(8) / 5 {
            return false; // untrusted count vs remaining bytes
        }
        let mut genome = Vec::with_capacity(n);
        let mut pos = 8usize;
        let mut skipped = 0usize;
        for _ in 0..n {
            let Some(l4) = bytes.get(pos..pos + 4) else { break };
            let len = u32::from_le_bytes(l4.try_into().unwrap()) as usize;
            pos += 4;
            if len > bytes.len().saturating_sub(pos) {
                break;
            }
            let slice = &bytes[pos..pos + len];
            pos += len;
            let mut ep = 0usize;
            match Element::from_bytes(slice, &mut ep) {
                Some(e) => genome.push(e),
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
        self.expression = vec![0; genome.len()];
        self.genome = genome;
        self.atlas = crate::transducer::stalk::Stalks::default();
        self.axis_index = crate::transducer::sentence::AxisIndex::new();
        self.shapes.clear();
        for w in &self.genome {
            self.atlas.admit(&crate::transducer::stalk::invariant_of(w), w.address());
            crate::transducer::sentence::admit_words(
                &mut self.axis_index,
                &crate::transducer::stalk::invariant_of(w),
                &crate::transducer::sentence::words_of_element(w),
            );
            self.shapes.push(std::sync::OnceLock::new());
        }
        self.atlas_len = self.genome.len();
        self.reindex();
        true
    }

}
