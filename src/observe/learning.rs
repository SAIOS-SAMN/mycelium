// 78.1, 78.3, LC1 — THE LEARNING CURVE, one mind at one `K`: its perspective
// by content and what its closures did, read off the mind's own genome and
// served to the console. Honest at zero; a field no producer yet reads is
// OWED, by the row that owes it, never printed as zero.
//
//! The three coordinates of a perspective (74, 78.1) are carried by what
//! they are, never by an orbital label:
//!
//! - the DIALECT SPAN — every stratum of the mind's genome atlas, by its
//!   chart key, with the words it holds;
//! - the COVER — the readings the mind's charts are cut by (53.2). OWED: the
//!   roster holds one reading, Bands, and a genome chart does not record the
//!   reading it was cut by, so every mind would print the same constant — a
//!   default standing in for a measurement (rule 11);
//! - the PLANES READ — the voice's own [`super::Planes`] reading, read
//!   beside this one and never copied into it (law 12).
//!
//! What the closures did: the generators a closure opened in this genome
//! (`Held::generators_opened`, the reader a court's `Kernel::adjoined`
//! shares), the genome's words, and of those the ones whose form closes
//! (50.1: coheres, no remainder, no demand).

use crate::client::sdk::Epigenome;

/// The fields LC1 names that no producer reads yet, each with the row that
/// owes it. The ONE statement of what is owed (law 12), read by every
/// renderer: a row that lands its producer moves its field into
/// [`Learning`] and off this list.
/// The coboundary's agreement per grade is PG1's (`Progress`, the round kept
/// per grade), read beside this one and never copied into it (law 12).
pub const OWED: [(&str, &str); 4] = [
    ("cover", "53.2's readings per mind: K4, K5 widen the roster past Bands"),
    ("warm-cold delta", "Y1, re-taken on the fleet"),
    ("distinct perspectives per class", "CV3"),
    ("mass per lane per layer per grade", "E5 (MS1, Phase P)"),
];

/// ONE MIND'S CURVE AT ONE `K`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Learning {
    /// The mind's own `K` when read.
    pub k: u64,
    /// The dialect span: each stratum of the genome atlas, by its chart key,
    /// and its words, in the key's own order.
    pub strata: Vec<(Vec<u8>, u64)>,
    /// Generators a closure opened in this genome (I1's reading, per mind).
    pub adjoined_by_closure: u64,
    /// Words the genome holds: the court-ruled words taught to this mind.
    pub installed: u64,
    /// Of them, those whose form closes.
    pub closing: u64,
}

impl Learning {
    /// THE CURVE OF THIS MIND, read off its own genome at its own `k`, at the
    /// read; nothing is stored beside the carriers (law 12).
    pub fn of(epigenome: &Epigenome, k: u64) -> Learning {
        let strata = epigenome
            .atlas
            .strata()
            .into_iter()
            .map(|(inv, members)| (inv.key().as_bytes().to_vec(), members.len() as u64))
            .collect();
        let (mut installed, mut closing, mut adjoined_by_closure) = (0u64, 0u64, 0u64);
        for held in epigenome.genome_words() {
            installed += 1;
            adjoined_by_closure += held.generators_opened();
            if held.form().is_some_and(|f| f.is_closed()) {
                closing += 1;
            }
        }
        Learning { k, strata, adjoined_by_closure, installed, closing }
    }

    pub(crate) fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.k.to_le_bytes());
        out.extend_from_slice(&(self.strata.len() as u32).to_le_bytes());
        for (key, n) in &self.strata {
            out.extend_from_slice(&(key.len() as u32).to_le_bytes());
            out.extend_from_slice(key);
            out.extend_from_slice(&n.to_le_bytes());
        }
        for v in [self.adjoined_by_closure, self.installed, self.closing] {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Learning> {
        let u64_at = |b: &[u8], p: &mut usize| -> Option<u64> {
            let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(v)
        };
        let u32_at = |b: &[u8], p: &mut usize| -> Option<usize> {
            let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
            *p += 4;
            Some(v as usize)
        };
        let keyed = |b: &[u8], p: &mut usize| -> Option<Vec<(Vec<u8>, u64)>> {
            let m = u32_at(b, p)?;
            // Each entry is at least a length and a count; a count the body
            // cannot hold is refused before anything is reserved for it.
            if b.len().checked_sub(*p)? < m.checked_mul(12)? {
                return None;
            }
            let mut out = Vec::with_capacity(m);
            for _ in 0..m {
                let len = u32_at(b, p)?;
                let key = b.get(*p..*p + len)?.to_vec();
                *p += len;
                out.push((key, u64_at(b, p)?));
            }
            Some(out)
        };
        let k = u64_at(b, p)?;
        let strata = keyed(b, p)?;
        let (adjoined_by_closure, installed, closing) = (u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?);
        Some(Learning { k, strata, adjoined_by_closure, installed, closing })
    }
}
