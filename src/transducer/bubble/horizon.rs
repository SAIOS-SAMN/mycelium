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
    /// Admit syllabus class records as RULED. Copies into every live bubble.
    ///
    /// `Standing::Held` stood here, and it is the same standing the mind's
    /// own obstructions go under — `paint_standing` for the classes a form
    /// arrives with, `Fill::Class` for a cell it cannot close. So the one
    /// reader that acts on this, `pivot_loop`'s A19 suppression, could not
    /// tell a class the COURT has ruled from a class this mind merely has,
    /// and since `paint_standing` puts every standing loop's own class on
    /// the horizon, every loop deferred to itself.
    ///
    /// These records are the syllabus: another mind ruled them and can
    /// supply the word. That is `Ruled` — "court-ratified" — and it is the
    /// only standing on which waiting is the right move. The mind's own
    /// stay `Held`, "an idea this mind is holding, not yet a word", which is
    /// exactly what they are.
    /// The symmetries every collapse kept ride out beside the count. A
    /// ruling that collapses a vortex resolves the mind's superposition, and
    /// the hypothesis it did not take is the loop's symmetry — a measured
    /// rotor, and 37.7's object (Expansion 4). It is carried rather than
    /// minted here for the reason every adjunction is: the language is the
    /// entity's.
    pub fn seed_horizon_with(
        &mut self,
        records: &[ClassRecord],
    ) -> (usize, Vec<crate::transducer::alphabet::Rotor>) {
        let mut kept: Vec<crate::transducer::alphabet::Rotor> = Vec::new();
        let n = self.seed_horizon_inner(records, &mut kept);
        (n, kept)
    }

    pub fn seed_horizon(&mut self, records: &[ClassRecord]) -> usize {
        self.seed_horizon_with(records).0
    }

    fn seed_horizon_inner(
        &mut self,
        records: &[ClassRecord],
        kept: &mut Vec<crate::transducer::alphabet::Rotor>,
    ) -> usize {
        let mut n = 0usize;
        for rec in records {
            if self.seeded.contains(&rec.key) {
                continue;
            }
            self.seeded.insert(
                rec.key.clone(),
                (rec.representative.clone(), rec.order, Standing::Ruled),
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
                            (rec.representative.clone(), rec.order, Standing::Ruled),
                        );
                    }
                    if let Some(syms) =
                        b.collapse_vortex(&rec.key, &rec.representative.key(), "ruling")
                    {
                        kept.extend(syms);
                    }
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
            // The loop's own class, on ANY member path that occupies a
            // coordinate: the holonomy is one word, so the horizon takes it
            // once, but whether it stands is a reading per path.
            if f.class_vectors(i).scoped().any(|(_, cv)| !cv.coordinates().is_empty()) {
                horizon_put(horizon, lp.holonomy.rotor(), Standing::Held);
            }
            // EVERY MEMBER PATH whose edge holonomy occupies a coordinate puts
            // its rotor on the horizon. A loop running through two pairs of
            // bodies can stand on one and be flat on the other, and reading one
            // put the other's obstruction nowhere.
            let stands = f.edge_class_vectors(lp);
            if let Some(hs) = f.edge_holonomies(lp) {
                for (k, e) in hs.scoped() {
                    if stands.get(&k).is_some_and(|cv| !cv.coordinates().is_empty()) {
                        horizon_put(horizon, e.word.rotor(), Standing::Held);
                    }
                }
            }
        }
    }
}
