// A19, 25.4 — THE HORIZON: the classes a mind has been taught to expect,
// seeded from the syllabus and never re-derived.
//
//! A class record travels as coordinates (A19), so a seeded class is
//! recognised on arrival rather than recomputed from the word that carries
//! it. The horizon is keyed by the class key and survives a restart.

use super::*;

pub(crate) fn horizon_put(h: &mut Keyed<(Rotor, Order, Standing)>, r: Rotor, standing: Standing) {
    let Some(order) = r.word_order() else {
        return;
    };
    let Some(key) = r.class_key() else {
        return;
    };
    h.insert(key, (r, order, standing));
}

impl Contemplation {
    /// Admit syllabus class records as Held. Copies into every live bubble.
    pub fn seed_horizon(&mut self, records: &[ClassRecord]) -> usize {
        let mut n = 0usize;
        for rec in records {
            if self.seeded.contains(&rec.key) {
                continue;
            }
            self.seeded.insert(
                rec.key.clone(),
                (rec.representative.clone(), rec.order, Standing::Held),
            );
            n += 1;
        }
        let keys: Vec<Vec<u8>> = self.bubbles.fiber(ROOT).map(|(k, _)| k).collect();
        for k in keys {
            if let Some(b) = self.bubbles.get_mut(&k) {
                for rec in records {
                    if !b.horizon.contains(&rec.key) {
                        b.horizon.insert(
                            rec.key.clone(),
                            (rec.representative.clone(), rec.order, Standing::Held),
                        );
                    }
                    b.collapse_vortex(
                        &rec.key,
                        &rec.representative.key(),
                        "ruling",
                    );
                }
            }
        }
        n
    }

    pub(crate) fn paint_seeded(&self, horizon: &mut Keyed<(Rotor, Order, Standing)>) {
        for (k, v) in self.seeded.fiber(ROOT) {
            if !horizon.contains(&k) {
                horizon.insert(&k, v.clone());
            }
        }
    }

}
