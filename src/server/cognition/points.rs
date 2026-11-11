// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-ECONOMY §3.4, §11.4 F2b — ONE CLOSURE, ONE POINT: a closure is the
// point its closing orbit reaches on the closed demand, written out as the
// task's citation and the closing rule's address. A first closure seals the
// point on its settling act (`Cite::Closure`); every court that back-fills
// that receipt holds the point in its cones, and judges a later closure of
// it again, whichever court it is presented at.
//
//! The points are held by their coordinates written out (A21), never by a
//! digest of them, each with the stamps of the receipts that sealed it, in
//! the order this court held them. They are kept across folds: a point
//! closed in a folded prefix stays closed.
//!
//! TWO SEALS OF ONE POINT ARE CONCURRENT (45.3). A court notes a point the
//! moment it admits the receipt that seals it, and mints no point it holds,
//! so no seal it makes or admits after holding a point has that point's
//! seal in its past cone unseen: every second seal it meets was made where
//! the first was not yet held. The two are held as a residue naming both,
//! never divided, never ordered by arrival (MU-ECONOMY §3.4; 45.3), and
//! resolved by content.

use super::Court;
use crate::chronometry::{Cite, KStepReceipt, Stamp};
use crate::network::nerve::Keyed;

/// THE POINT'S COORDINATES WRITTEN OUT: the task's citation, then the rule's
/// address.
pub(crate) fn point_key(task: &[u8], rule: &[u8]) -> Vec<u8> {
    let mut k = Vec::with_capacity(task.len() + rule.len());
    k.extend_from_slice(task);
    k.extend_from_slice(rule);
    k
}

impl Court {
    /// THE CLOSURE POINTS A RECEIPT SEALS: each `Cite::Closure` it carries,
    /// at the receipt's stamp, after the stamps already held for that point.
    /// A second seal of a held point is the concurrent residue of 45.3,
    /// held beside the first and said; a seal met again is held once.
    pub(crate) fn note_points(&mut self, r: &KStepReceipt) {
        for c in &r.cites {
            if let Cite::Closure { task, rule } = c {
                let k = point_key(task, rule);
                let mut seals = match self.points.get(&k) {
                    Some(held) => held.clone(),
                    None => Vec::new(),
                };
                if seals.contains(&r.stamp) {
                    continue;
                }
                if let Some((first, _)) = seals.split_first() {
                    eprintln!(
                        "  court concurrent closure {:02x}{:02x}…@{} | {:02x}{:02x}…@{} held (§3.4, 45.3)",
                        first.vk[0],
                        first.vk[1],
                        first.place().print(),
                        r.stamp.vk[0],
                        r.stamp.vk[1],
                        r.stamp.place().print()
                    );
                }
                seals.push(r.stamp);
                self.points.insert(&k, seals);
            }
        }
    }

    /// The stamp that first sealed the point `(task, rule)` in this court's
    /// cones, the order held being what decides here (a later seal is the
    /// residue, never a rival); `None` where it is open here.
    pub fn point_held(&self, task: &[u8], rule: &[u8]) -> Option<Stamp> {
        self.points.get(&point_key(task, rule)).and_then(|s| s.split_first().map(|(f, _)| *f))
    }

    /// How many closure points this court's cones hold.
    pub fn points_held(&self) -> usize {
        self.points.len()
    }

    /// THE CONCURRENT RESIDUES (MU-ECONOMY §3.4): every point sealed more
    /// than once in this court's cones, with all its seals, in the order
    /// held. A reading; nothing is divided and nothing is burned on it.
    pub fn concurrent_points(&self) -> Vec<(Vec<u8>, Vec<Stamp>)> {
        self.points.scoped().filter(|(_, s)| s.len() > 1).map(|(k, s)| (k, s.clone())).collect()
    }
}

/// The points as the snapshot carries them: the count, then each point's
/// coordinates and its seals, counted, in the order held; in key order.
pub(crate) fn put_points(points: &Keyed<Vec<Stamp>>) -> Vec<u8> {
    let mut out = Vec::new();
    crate::transducer::address::put_key_count(&mut out, points.len());
    for (k, seals) in points.scoped() {
        let _ = crate::transducer::address::put_bytes(&mut out, &k);
        crate::transducer::address::put_key_count(&mut out, seals.len());
        for at in seals {
            at.put_canonical(&mut out);
        }
    }
    out
}

/// The inverse of [`put_points`]; `None` where a point does not read whole.
pub(crate) fn take_points(b: &[u8], p: &mut usize) -> Option<Keyed<Vec<Stamp>>> {
    let n = crate::transducer::address::take_key_count(b, p)?;
    let mut out = Keyed::new();
    for _ in 0..n {
        let k = crate::transducer::address::take_bytes(b, p)?;
        let m = crate::transducer::address::take_key_count(b, p)?;
        let seals = (0..m).map(|_| Stamp::take_canonical(b, p)).collect::<Option<Vec<_>>>()?;
        if seals.is_empty() {
            return None;
        }
        out.insert(&k, seals);
    }
    Some(out)
}
