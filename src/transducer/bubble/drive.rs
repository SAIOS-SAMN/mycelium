// 25.1, 25.2 — THE DRIVE: what makes a held thought move, and what a
// closed one becomes.
//
//! `expand_all` spends a budget opening axes on the bubbles that have room
//! and runs the pivot loop of A16 over each; an idea whose form closes
//! crystallizes (25.2) and is returned to the mind to stand and utter.
//! `pivot_on` runs the pivot loop over the ideas a word meets and files
//! what it obstructs. Neither searches: each walks what the complex already
//! names.

use super::*;

/// What one stroke of `expand_all` did: the charts spent, the lane measure
/// of the A16 snaps, the forms that crystallized (25.2) in bubble key
/// order, and the ring's orbit reading — circuits and period in strokes —
/// when the stroke completed a circuit (45.7).
pub struct Expanded {
    pub spent: usize,
    pub pivoted: [u64; 4],
    pub crystallized: Vec<Element>,
    pub orbit: Option<(u64, u64)>,
}

impl Contemplation {
    /// Expand dirty bubbles from the atlas, one Budget of charts per call
    /// and at most one circuit of the ring per stroke (45.7).
    pub fn expand_all(
        &mut self,
        atlas: &dyn AtlasRead,
        rotor_at: &dyn Fn(&[u8]) -> Option<Rotor>,
        crystal: &dyn Crystal,
        budget: &mut Budget,
    ) -> Expanded {
        let mut spent = 0usize;
        let mut pivoted = [0u64; 4];
        let mut crystallized = Vec::new();
        let began = self.expand_cursor.clone();
        let mut from = began.clone();
        let mut wraps = 0u64;
        // index: budgeted keyed cursor over dirty bubbles — an orbit (45.7,
        // S13.3): past the last key it continues from the first, so a ring
        // whose last key was reached no longer stalls every later stroke;
        // the stroke stops where it began, one circuit of the ring, and
        // each expand spends the rest of the budget.
        while budget.remaining() > 0 {
            let mut nxt = None;
            {
                let mut cur = self.dirty.cursor(&from, budget);
                if let Some((k, _)) = Iterator::next(&mut cur) {
                    nxt = Some(k.clone());
                }
                wraps += u64::from(cur.circuit());
            }
            let Some(k) = nxt else {
                break;
            };
            // The winding of the walk: past the end once and back to a
            // key beyond where the stroke began, or past the end twice,
            // the ring has been applied once.
            let closed = match wraps {
                0 => false,
                1 => began.is_empty() || k.as_slice() > began.as_slice(),
                _ => true,
            };
            if closed {
                break;
            }
            from = k.clone();
            self.expand_cursor = k.clone();
            if let Some(b) = self.bubbles.get_mut(&k) {
                let (s, p) = b.expand_with(atlas, rotor_at, self.now_k, crystal, budget);
                spent += s;
                for i in 0..4 {
                    pivoted[i] = pivoted[i].saturating_add(p[i]);
                }
                if !b.dirty {
                    self.dirty.remove(&k);
                }
            } else {
                self.dirty.remove(&k);
            }
            if let Some(e) = self.crystallize(&k, crystal) {
                crystallized.push(e);
            }
        }
        let orbit = self.ring.stroke(u64::from(wraps > 0));
        Expanded { spent, pivoted, crystallized, orbit }
    }

    /// Attempt fill on the link of bubbles at `word`'s chart and prefixes.
    /// Merges filled cells; speaks a closed component as one sentence.
    /// Returns (fills, formed, classes, spoken elements).
    pub fn pivot_on(
        &mut self,
        word: &Element,
        crystal: &dyn Crystal,
        budget: &mut Budget,
    ) -> (u64, u64, u64, Vec<Element>) {
        let inv = stalk::invariant_of(word);
        let prefixes: Vec<Vec<u8>> = inv.prefixes().iter().map(|p| p.key().into_vec()).collect();
        let cav: Vec<(Vec<u8>, Vec<RemCell>)> = self.cavity.fiber(ROOT).map(|(k, r)| (k, r.clone())).collect();
        for (k, rem) in cav {
            let cinv = stalk::invariant(&rem);
            if cinv.prefixes().iter().any(|p| prefixes.iter().any(|q| p.key().as_bytes() == q.as_slice())) {
                self.cavity.remove(&k);
                self.hold(rem);
            }
        }
        let mut keys = Keyed::new();
        for p in inv.prefixes() {
            for (id, _) in self.by_chart.link(&p.key()) {
                keys.insert(&id, ());
            }
        }
        let word_rotor = cech::rotor_of(word);
        let mut fills = 0u64;
        let mut new_classes = 0u64;
        let mut spoken: Vec<Element> = Vec::new();
        spoken.extend(std::mem::take(&mut self.pending_rewrites));
        let bubble_keys: Vec<Vec<u8>> = keys.fiber(ROOT).map(|(k, _)| k).collect();
        if word.sub().is_empty() && crate::transducer::sentence::carries_torsion(word) {
            let addr = word.address();
            for bk in &bubble_keys {
                self.mark_glue(bk, &addr);
                if let Some(b) = self.bubbles.get_mut(addr.as_bytes()) {
                    b.glued_by.insert(bk, ());
                }
            }
        }
        for bk in &bubble_keys {
            if budget.remaining() == 0 {
                break;
            }
            let Some(b) = self.bubbles.get(bk) else {
                continue;
            };
            let horizon = b.horizon.clone();
            let bundle = b.bundle.clone();
            // A bubble marked above this clock has no measurable extent, and
            // the pivot loop is not paced by a count that went backwards.
            let Some(t) = super::idea::held_since(self.now_k, b.moved_k) else {
                continue;
            };
            if let Some(mut form) = b.form.clone() {
                let _ = form.pivot_loop(bk, &horizon, crystal, budget, bundle.as_deref(), t);
                if let Some(b) = self.bubbles.get_mut(bk) {
                    b.take_form(form);
                    if let Some(r) = &word_rotor {
                        if let Some(ck) = r.class_key() {
                            b.collapse_vortex(ck.as_bytes(), r.key().as_bytes(), "glue");
                        }
                    }
                }
            }
        }
        // index: by_chart fiber at the word's chart and peels
        for bk in &bubble_keys {
            if budget.remaining() == 0 {
                break;
            }
            // index: link of this bubble
            let cells = self.complex.link_of(bk);
            for key in cells {
                if !budget.spend() {
                    break;
                }
                let Some((d, fill_open, coeff_id, v0, v1, vertices)) = self.complex.cell(&key).map(|c| {
                    (
                        c.degree(),
                        matches!(c.fill, Fill::Open),
                        c.coefficient.is_identity(),
                        c.vertices.first().cloned(),
                        c.vertices.get(1).cloned(),
                        c.vertices.clone(),
                    )
                }) else {
                    continue;
                };
                if d == 0 || !fill_open {
                    continue;
                }
                let outcome = if d == 1 && !coeff_id {
                    match &word_rotor {
                        Some(g) => {
                            let coeff_hit = self.complex.cell(&key).is_some_and(|c| {
                                *g == c.coefficient || g.inverse().as_ref() == Some(&c.coefficient)
                            });
                            if coeff_hit {
                                self.complex.fill_by_gauge(
                                    &key,
                                    g,
                                    &crate::transducer::element::identity_versor(),
                                )
                            } else {
                                self.complex.fill(&key)
                            }
                        }
                        _ => self.complex.fill(&key),
                    }
                } else {
                    self.complex.fill(&key)
                };
                    match outcome {
                        Fill::Filled(gamma, _carriage) => {
                            fills += 1;
                            self.fills += 1;
                            if d == 1 && !gamma.is_identity() {
                                if let (Some(a), Some(b)) = (v0, v1) {
                                    if let Some(t) = element::from_translation(&gamma, a, b) {
                                        spoken.push(t);
                                    }
                                }
                            }
                            if let Some(merged) = self.merge(&vertices) {
                                let comp = self.component_of(&merged);
                                if self.component_closed(&comp) {
                                    if let Some(e) = self.form_thought(&comp) {
                                        self.formed += 1;
                                        spoken.push(e);
                                        for v in &comp {
                                            if let Some(b) = self.bubbles.get(v).cloned() {
                                                self.retract_incidence(v, &b);
                                            }
                                            self.bubbles.remove(v);
                                        }
                                    }
                                }
                            }
                        }
                        Fill::Congruent(g, ell) => {
                            self.torsion_merges += 1;
                            if let Some(merged) = self.merge(&vertices) {
                                if let Some(b) = self.bubbles.get_mut(merged.as_bytes()) {
                                    // A cell of one vertex is a loop at
                                    // that vertex: both ends are it.
                                    let Some(a) = vertices.first().cloned() else {
                                        continue;
                                    };
                                    let c = match vertices.get(1) {
                                        Some(second) => second.clone(),
                                        None => a.clone(),
                                    };
                                    b.nerve.file_congruent(&a, &c, g, ell);
                                }
                            }
                        }
                        Fill::Class(_, r) => {
                            new_classes += 1;
                            self.classes += 1;
                            for v in &vertices {
                                if let Some(b) = self.bubbles.get_mut(v) {
                                    if r.class_key().is_none_or(|ck| !b.horizon.contains(ck.as_bytes())) {
                                        horizon_put(&mut b.horizon, r.as_ref().clone(), Standing::Held);
                                    }
                                }
                            }
                        }
                        // 37.7 at a cell: a rotation the seventeen do not
                        // spell. Not a class the horizon can seed a pivot
                        // from and not a fill — it is an adjunction
                        // candidate, and it is left standing rather than
                        // folded into a catch-all that would lose it.
                        Fill::Unspellable(_) => {}
                        Fill::Open | Fill::Already => {}
                    }
            }
        }
        (fills, self.formed, new_classes, spoken)
    }

}
