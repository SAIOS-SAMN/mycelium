// A19, 25.4 — THE HORIZON: the classes a mind has been taught to expect,
// seeded from the syllabus and never re-derived.
//
//! A class record travels as coordinates (A19), so a seeded class is
//! recognised on arrival rather than recomputed from the word that carries
//! it. The horizon is keyed by the class key and survives a restart.

use super::*;

pub(crate) fn horizon_put(h: &mut Keyed<(Rotor, Order, Standing)>, r: Rotor, standing: Standing) {
    let Some(order) = r.order() else {
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
        let keys: Vec<Vec<u8>> = self.bubbles.scoped().map(|(k, _)| k).collect();
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
        for (k, v) in self.seeded.scoped() {
            if !horizon.contains(&k) {
                horizon.insert(&k, v.clone());
            }
        }
    }

    /// THE CLASSES A FORM ARRIVED WITH (L24). A bubble was seeded from the
    /// mesh's classes alone, so a loop the form already carries — a
    /// cover's meet cycle whose holonomy stands — reached `pivot_loop` as
    /// nothing at all: the pivot reads the horizon it is given, and the
    /// class the perception had just measured was not in it. The cover
    /// classified it, tallied it, and dropped it.
    ///
    /// `Bubble::expand_with` already does exactly this for a cell it meets
    /// on the atlas (`Fill::Class` → `horizon_put`); this is the same
    /// admission for the classes that arrive on the form, so a perceived
    /// obstruction and a contemplated one are one kind of object.
    ///
    /// BOTH OF A LOOP'S OBSTRUCTIONS are admitted. The transport composite
    /// and 50.3's edge composite are different objects and a loop can
    /// stand on either: an extraction's loop holds its turn on the
    /// transports while its edges compose to the identity, and a cover's
    /// loop is the other way round, since every word a pair form composes
    /// is a sensor-plane word and those commute.
    ///
    /// `Standing::Held`, as the atlas arm writes: met, not ruled. A loop
    /// occupying no coordinate is not a member — there is nothing to
    /// discharge — and a rotor with no order or no class key is refused by
    /// `horizon_put` itself.
    pub(crate) fn paint_standing(
        &self,
        f: &crate::transducer::sentence::Form,
        horizon: &mut Keyed<(Rotor, Order, Standing)>,
    ) {
        for (i, lp) in f.loops.iter().enumerate() {
            if f.class_vector(i).is_some_and(|cv| !cv.coordinates().is_empty()) {
                horizon_put(horizon, lp.holonomy.rotor(), Standing::Held);
            }
            let stands = f
                .edge_class_vector(lp)
                .is_some_and(|cv| !cv.coordinates().is_empty());
            if let (true, Some(e)) = (stands, f.edge_holonomy(lp)) {
                horizon_put(horizon, e.word.rotor(), Standing::Held);
            }
        }
    }
}
