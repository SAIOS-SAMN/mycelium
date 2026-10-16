// MU-CLIENT §4.13, §7.1 Phase 3 — A HELD THOUGHT IS ITS ORBIT, its line in
// the mind's field: memory released by the orbit's physics — its return, its
// beat, its seal — never by a window of the clock.
//
//! A PERCEPTION threads its line on the orbits and tubes its own motion
//! stands on — its faces' motors and hue maps, re-derived from its form, the
//! motion admitted where it was perceived — and is released wherever the
//! field lets that line go; a perception whose motion the field already let
//! go (a slide, dissipated at the beat after its admission) is released at
//! once. A GROUNDED THOUGHT admits its ground's generator on its own line,
//! once for each hour it has seen, so the field's orbit returns when its
//! winding does (`hours / ℓ`); at that return it WINDS ON — the strain vented,
//! what was held for its scene said — and it lapses only where its orbit is
//! dissipated on a beat or seals at its period (78.43).
//!
//! A thought released leaves the contemplation whole: its remainder to the
//! cavity at its chart (it re-forms where a later word lands there), its
//! escrow returned exactly as it was drawn, its scene counted as bound for
//! the stroke — so what the mind held for that scene's return leaves
//! (78.168: a dissipated escaping orbit is bound). Escrow is conserved: every
//! door that removes a thought returns what it holds, once.

use super::core::Door;
use super::*;
use crate::transducer::cech::orbit::Released;

impl Contemplation {
    /// ESCROW RETURNED, once, for the room: whatever door removed the thought
    /// that held it. A sum past its carrier is refused and counted.
    pub(crate) fn return_escrow(&mut self, escrow: &[u64; 4]) {
        if !crate::core::lanes::lanes_step(&mut self.returned_escrow, escrow) {
            self.overflowed += 1;
        }
    }

    /// The escrow returned since last taken, for the room.
    pub fn take_returned_escrow(&mut self) -> [u64; 4] {
        std::mem::take(&mut self.returned_escrow)
    }

    /// The scenes whose thought the field released since last taken: each a
    /// return of a thought of that scene for the stroke.
    pub(crate) fn take_released_scenes(&mut self) -> Vec<Vec<u8>> {
        std::mem::take(&mut self.released_scenes)
    }

    /// A THOUGHT NEWLY STANDING, to thread its line at the next vent.
    pub(crate) fn awaits_thread(&mut self, key: &[u8]) {
        self.unthreaded.insert(key, ());
    }

    /// A THOUGHT RELEASED BY THE FIELD (its line let go): it leaves whole —
    /// its incidence retracted, its remainder to the cavity at its chart, its
    /// escrow returned, its scene counted — and the components it stood in
    /// truncate. A thought no longer held is gone already.
    pub(crate) fn release_thought(&mut self, key: &[u8]) {
        if !self.bubbles.contains(key) {
            return;
        }
        let doomed = vec![key.to_vec()];
        let before = self.shed_before(&doomed);
        let touched = self.touched_by(&doomed);
        if let Some(b) = self.bubbles.remove(key) {
            self.retract_incidence(key, &b);
            self.dirty.remove(key);
            self.unthreaded.remove(key);
            self.unheld.remove(key);
            self.cavity.insert(super::shed::cavity_key(&b.remainder, key), b.remainder.to_vec());
            self.return_escrow(&b.escrow);
            if !b.family.is_empty() {
                self.released_scenes.push(b.family.clone());
            }
        }
        self.shed_after(Door::Orbit, 1, before);
        self.truncate(&touched);
    }

    /// A LINE THE FIELD LET GO THAT IS A THOUGHT'S OWN, read by why: a
    /// grounded thought whose orbit RETURNED winds on (§4.4(2)); every other
    /// release lets the thought go. Answers whether `line` was a thought's.
    pub(crate) fn release_line(&mut self, line: &[u8], why: Released) -> bool {
        let Some(grounded) = self.bubbles.get(line).map(|b| b.ground.is_some()) else { return false };
        match (grounded, why) {
            (true, Released::Returned) => {}
            _ => self.release_thought(line),
        }
        true
    }

    /// THE PERCEPTIONS NEWLY STANDING THREAD THEIR LINES (MU-CLIENT §4.13):
    /// each on the orbits and tubes its faces' motors and hue maps stand on,
    /// in the field as it stands. One the field holds nothing of — its motion
    /// closed or let go already — stands the one vent, as a motion stands one
    /// beat, so the perception after it can meet it, and leaves at the next
    /// vent. A grounded thought winds its own line as its hours are admitted
    /// (`wind_ground`), and a thought with no motion (a remainder held alone,
    /// a merge without ground) has no orbit and stands by the citation and
    /// unglued doors.
    pub(crate) fn thread_new(&mut self, budget: &mut crate::physics::Budget) {
        let stood: Vec<Vec<u8>> = self.unheld.scoped().map(|(k, _)| k).collect();
        self.unheld = Keyed::new();
        for key in stood {
            self.release_thought(&key);
        }
        let fresh: Vec<Vec<u8>> = self.unthreaded.scoped().map(|(k, _)| k).collect();
        self.unthreaded = Keyed::new();
        for key in fresh {
            let motions: Vec<(crate::transducer::cga::Mv, Vec<(u8, u8)>)> = match self.bubbles.get(&key) {
                Some(b) if b.ground.is_none() => match &b.form {
                    Some(f) => f.faces.entries().map(|(_, face)| (face.place.carriage.clone(), face.place.gauge.clone())).collect(),
                    None => continue,
                },
                _ => continue,
            };
            if motions.is_empty() {
                continue;
            }
            let held = crate::transducer::mind::with_field(|field| {
                let mut held = false;
                for (motion, gauge) in &motions {
                    held |= matches!(field.thread(motion, gauge, &key, budget), Some(true));
                }
                held
            });
            if let Ok(false) = held {
                self.unheld.insert(&key, ());
            }
        }
    }

    /// A GROUNDED THOUGHT'S LINE, WOUND (MU-CLIENT §4.13, Q1 as ruled): its
    /// ground's generator admitted into the mind's field on the thought's own
    /// line once for each of `hours` hours seen, so the orbit returns when
    /// the winding does. The line is the key the thought is filed under, its
    /// ground's coordinates on its chart.
    pub(crate) fn wind_ground(&mut self, grounded: &super::idea::Grounded, hours: u64, budget: &mut crate::physics::Budget) {
        let Some(line) = grounded.ground.key_on(&grounded.chart) else { return };
        let Ok(n) = usize::try_from(hours) else { return };
        let lines: Vec<crate::transducer::family::pairing::FieldLine> = (0..n)
            .map(|_| crate::transducer::family::pairing::FieldLine {
                carriage: grounded.ground.generator.clone(),
                gauge: Vec::new(),
                mass: 1,
                free: false,
                line: crate::transducer::cech::Thread::Wound(line.as_bytes().to_vec()),
            })
            .collect();
        if !lines.is_empty() {
            let _ = crate::transducer::family::pairing::mark_flux(&[], &lines, budget);
        }
    }
}
