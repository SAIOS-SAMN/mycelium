// 78.37(4), RM1 — THE REMAINDERS A COMPLEX HOLDS: every simplex standing
// `Congruent(κ, g, ℓ)` — a sentence coherent up to its class — read by its
// period and its class, and the re-fills.
//
//! One reading, two sites named apart: the mind's contemplation (the
//! receptor's site, a chart per fill read off its thoughts' covers) and the
//! court's complexes (the durable store, a chart per complex). A class
//! standing at two or more charts is the receptor's precondition. A
//! RE-FILL is `Congruent` written `Filled`, counted where a fill is written
//! (`Complex::refilled`); no door re-reads one today.

use super::*;
use crate::network::nerve::Keyed;

/// ONE REMAINDER STANDING: its class key, its period, its simplex's vertices.
#[derive(Clone, Debug)]
pub struct Standing {
    pub class: Vec<u8>,
    pub ell: u32,
    pub vertices: Vec<Vec<u8>>,
}

/// RM1 — THE REMAINDERS, READ.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RemainderReading {
    /// `Congruent` fills standing, by `ℓ`, ascending.
    pub by_ell: Vec<(u32, u64)>,
    /// Distinct classes `κ` among them.
    pub classes: u64,
    /// Classes standing at two or more charts.
    pub across: u64,
    /// Fills whose chart did not read (a mind's fill whose thoughts share
    /// no chart).
    pub unplaced: u64,
    /// `Congruent` simplices written `Filled`.
    pub refilled: u64,
}

/// The reading as it accumulates: `ℓ` → count, class → the charts it stands
/// at.
#[derive(Default)]
pub struct RemainderCount {
    by_ell: Keyed<u64>,
    at: Keyed<Keyed<()>>,
    unplaced: u64,
    refilled: u64,
}

impl RemainderCount {
    /// One remainder standing at `chart`, or at no chart that read.
    pub fn add(&mut self, s: &Standing, chart: Option<&[u8]>) {
        let ek = s.ell.to_be_bytes();
        match self.by_ell.get_mut(&ek) {
            Some(n) => *n += 1,
            None => {
                self.by_ell.insert(ek, 1);
            }
        }
        match chart {
            Some(c) => self.at.nested_insert(&s.class, c, ()),
            None => {
                self.unplaced += 1;
                if !self.at.contains(&s.class) {
                    self.at.insert(&s.class, Keyed::new());
                }
            }
        }
    }

    pub fn refilled(&mut self, n: u64) {
        self.refilled += n;
    }

    pub fn read(self) -> RemainderReading {
        // Big-endian `ℓ` keys: the trie's order is ascending `ℓ`.
        let by_ell = self
            .by_ell
            .scoped()
            .filter_map(|(k, n)| Some((u32::from_be_bytes(k.as_slice().try_into().ok()?), *n)))
            .collect();
        RemainderReading {
            by_ell,
            classes: self.at.len() as u64,
            across: self.at.scoped().filter(|(_, charts)| charts.len() >= 2).count() as u64,
            unplaced: self.unplaced,
            refilled: self.refilled,
        }
    }
}

impl RemainderReading {
    pub fn line(&self) -> String {
        let ells = match self.by_ell.is_empty() {
            true => "none".to_string(),
            false => self.by_ell.iter().map(|(l, n)| format!("ℓ{l} {n}")).collect::<Vec<_>>().join(" "),
        };
        format!(
            "remainders: congruent {ells}; classes {} across charts {} unplaced {}; re-filled {}",
            self.classes, self.across, self.unplaced, self.refilled
        )
    }

    pub(crate) fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(self.by_ell.len() as u32).to_le_bytes());
        for (l, n) in &self.by_ell {
            out.extend_from_slice(&l.to_le_bytes());
            out.extend_from_slice(&n.to_le_bytes());
        }
        for v in [self.classes, self.across, self.unplaced, self.refilled] {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<RemainderReading> {
        let u64_at = |b: &[u8], p: &mut usize| -> Option<u64> {
            let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(v)
        };
        let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
        *p += 4;
        // An entry is an `ℓ` and a count: 12 bytes; a count the body cannot
        // hold is refused before anything is reserved for it.
        if b.len().checked_sub(*p)? < n.checked_mul(12)? {
            return None;
        }
        let mut by_ell = Vec::with_capacity(n);
        for _ in 0..n {
            let l = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
            *p += 4;
            by_ell.push((l, u64_at(b, p)?));
        }
        Some(RemainderReading {
            by_ell,
            classes: u64_at(b, p)?,
            across: u64_at(b, p)?,
            unplaced: u64_at(b, p)?,
            refilled: u64_at(b, p)?,
        })
    }
}

impl Complex {
    /// EVERY SIMPLEX STANDING `Congruent`, at any degree.
    pub fn congruent_standing(&self) -> Vec<Standing> {
        let mut out = Vec::new();
        for layer in &self.degrees {
            for (_, s) in layer.scoped() {
                if let Fill::Congruent(kappa, _, ell) = &s.fill {
                    out.push(Standing { class: kappa.class_key().into_vec(), ell: *ell, vertices: s.vertices.clone() });
                }
            }
        }
        out
    }

    /// The re-fills counted on this complex.
    pub fn refilled(&self) -> u64 {
        self.refilled
    }
}
