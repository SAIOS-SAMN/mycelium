// 28 — THE EPIGENOME ON DISK: crystallized whole, and a tape that does not
// parse refused rather than partly restored.

use crate::client::sdk::*;
use crate::transducer::element::Element;

/// What a restored genome hands back beside its words: each word's
/// admitting stamp (78.24 (2b)), and where the words end on the tape.
pub struct Uncrystallized {
    pub admitted: Vec<(crate::transducer::address::Address, crate::chronometry::Stamp)>,
    pub end: usize,
}

impl Epigenome {
    /// CRYSTALLIZE the genome for persistence — the permanent language only.
    /// The epigenome is memory-only. Format: `GEN4 | n u32 | (len u32 |
    /// element | admitted)…`, where `admitted` is `0`, or `1` and the stamp
    /// of the act that admitted the word (78.24 (2b)).
    pub fn crystallize(&self, admitted: impl Fn(&[u8]) -> Option<crate::chronometry::Stamp>) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + self.genome.len() * 64);
        out.extend_from_slice(b"GEN4");
        out.extend_from_slice(&(self.genome.len() as u32).to_le_bytes());
        for (_, g) in self.resident.scoped() {
            let b = g.held.to_bytes();
            out.extend_from_slice(&(b.len() as u32).to_le_bytes());
            out.extend_from_slice(&b);
            match admitted(g.held.address().as_bytes()) {
                None => out.push(0),
                Some(at) => {
                    out.push(1);
                    at.put_canonical(&mut out);
                }
            }
        }
        out
    }

    /// Restore GEN4 only, whole or refused: an older file, or a word that
    /// does not parse, refuses the tape (rule 11), and nothing is restored.
    pub fn uncrystallize(&mut self, bytes: &[u8]) -> Option<Uncrystallized> {
        if bytes.get(..4)? != b"GEN4" {
            return None;
        }
        let n = u32::from_le_bytes(bytes.get(4..8)?.try_into().ok()?) as usize;
        if n > bytes.len().checked_sub(8)? / 5 {
            return None; // untrusted count vs remaining bytes
        }
        let mut genome = Vec::with_capacity(n);
        let mut admitted = Vec::new();
        let mut pos = 8usize;
        for _ in 0..n {
            let len = u32::from_le_bytes(bytes.get(pos..pos + 4)?.try_into().ok()?) as usize;
            pos += 4;
            let slice = bytes.get(pos..pos.checked_add(len)?)?;
            pos += len;
            let mut ep = 0usize;
            let e = Element::from_bytes(slice, &mut ep)?;
            let tag = *bytes.get(pos)?;
            pos += 1;
            match tag {
                0 => {}
                1 => admitted.push((e.address(), crate::chronometry::Stamp::take_canonical(bytes, &mut pos)?)),
                _ => return None,
            }
            // The tape is decoded to a form HERE, once, as the word is
            // taken in — not on the first read of its form (L16).
            genome.push(crate::transducer::element::Held::of(e));
        }
        if genome.is_empty() {
            return None;
        }
        self.genome = crate::network::nerve::Keyed::new();
        self.resident = crate::network::nerve::Keyed::new();
        self.unpivoted = crate::network::nerve::Keyed::new();
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
        // A RESTORED WORD IS WORK THE PIVOT HAS NOT REACHED, exactly as a
        // learned one is: `admit_word` puts its coordinate in the pending
        // set. A boot that skipped that left every restored word unpivoted
        // and unrecorded — the pivot could not tell them from words it had
        // already taken, because nothing said which was which.
        for w in genome {
            self.admit_word(w, 0);
        }
        self.atlas_len = self.genome.len();
        Some(Uncrystallized { admitted, end: pos })
    }

}
