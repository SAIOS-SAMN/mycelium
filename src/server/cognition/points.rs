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
//! digest of them, each with the stamp of the receipt that sealed it first
//! at this court. They are kept across folds: a point closed in a folded
//! prefix stays closed.

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
    /// THE CLOSURE POINTS A RECEIPT SEALS, held where they were not: each
    /// `Cite::Closure` it carries, at the receipt's stamp. A point already
    /// held keeps the stamp it was first held at.
    pub(crate) fn note_points(&mut self, r: &KStepReceipt) {
        for c in &r.cites {
            if let Cite::Closure { task, rule } = c {
                let k = point_key(task, rule);
                if self.points.get(&k).is_none() {
                    self.points.insert(&k, r.stamp);
                }
            }
        }
    }

    /// The stamp that first sealed the point `(task, rule)` in this court's
    /// cones; `None` where it is open here.
    pub fn point_held(&self, task: &[u8], rule: &[u8]) -> Option<Stamp> {
        self.points.get(&point_key(task, rule)).copied()
    }
}

/// The points as the snapshot carries them: the count, then each point's
/// coordinates and the stamp that sealed it first, in key order.
pub(crate) fn put_points(points: &Keyed<Stamp>) -> Vec<u8> {
    let mut out = Vec::new();
    crate::transducer::address::put_key_count(&mut out, points.len());
    for (k, at) in points.scoped() {
        let _ = crate::transducer::address::put_bytes(&mut out, &k);
        at.put_canonical(&mut out);
    }
    out
}

/// The inverse of [`put_points`]; `None` where a point does not read whole.
pub(crate) fn take_points(b: &[u8], p: &mut usize) -> Option<Keyed<Stamp>> {
    let n = crate::transducer::address::take_key_count(b, p)?;
    let mut out = Keyed::new();
    for _ in 0..n {
        let k = crate::transducer::address::take_bytes(b, p)?;
        let at = Stamp::take_canonical(b, p)?;
        out.insert(&k, at);
    }
    Some(out)
}
