// 25.1, 25.2 — THE DRIVE: what makes a held thought move, and what a
// closed one becomes.
//
//! `expand_all` spends a budget opening axes on the bubbles that have room
//! and runs the pivot orbit of A16 over each; an idea whose form closes
//! crystallizes (25.2) and is returned to the mind to stand and utter.
//! `pivot_on` runs the pivot orbit over the ideas a word meets and files
//! what it obstructs. Neither searches: each reads what the complex already
//! names.

use super::*;

/// What one stroke of `expand_all` did: the charts spent, the lane measure
/// of the A16 snaps, the forms that crystallized (25.2) in bubble key
/// order, and the ring's circuit reading — circuits and period in strokes —
/// when the stroke completed a circuit (45.7).
/// 37.7's ADJUNCTION CANDIDATES: a rotor the seventeen cannot spell and the
/// chart it was met on. Named because BOTH of the drive's paths produce them
/// — the expand ring and `pivot_on` — and a type they share is how the two
/// stay one reading (law 12).
pub type Adjoined = Vec<(crate::transducer::alphabet::Rotor, crate::transducer::stalk::Invariant)>;

pub struct Expanded {
    pub spent: usize,
    pub pivoted: [u64; 4],
    pub crystallized: Vec<Element>,
    /// The ring's circuit completed this stroke, `(circuits, period)`.
    pub circuit: Option<(u64, u64)>,
    /// 37.7's ADJUNCTION CANDIDATES: the rotors this stroke's cover meets
    /// produced that the seventeen cannot spell, each with the chart it was
    /// met on. They are carried rather than minted here because minting
    /// writes a word into the language and the language is the entity's.
    pub adjoined: Vec<(crate::transducer::alphabet::Rotor, crate::transducer::stalk::Invariant)>,
}

impl Contemplation {
    /// Expand dirty bubbles from the atlas, one Budget of charts per call
    /// and at most one circuit of the ring per stroke (45.7).
    pub fn expand_all(
        &mut self,
        atlas: &dyn AtlasRead,
        section_at: crate::transducer::bubble::idea::SectionAt,
        crystal: &dyn Crystal,
        budget: &mut Budget,
    ) -> Expanded {
        let mut spent = 0usize;
        let mut pivoted = [0u64; 4];
        let mut crystallized = Vec::new();
        let mut adjoined: Vec<(
            crate::transducer::alphabet::Rotor,
            crate::transducer::stalk::Invariant,
        )> = Vec::new();
        let began = self.expand_cursor.clone();
        let mut from = began.clone();
        let mut wraps = 0u64;
        // index: budgeted keyed cursor over dirty bubbles — a circuit (45.7,
        // S13.3): past the last key it continues from the first, so a ring
        // whose last key was reached no longer stalls every later stroke;
        // the stroke stops where it began, one circuit of the ring, and
        // each expand spends the rest of the budget.
        while budget.remaining() > 0 {
            let mut nxt = None;
            {
                let mut cur = self.dirty.cursor(from.as_deref(), budget);
                if let Some((k, _)) = Iterator::next(&mut cur) {
                    nxt = Some(k.clone());
                }
                wraps += u64::from(cur.circuit());
            }
            let Some(k) = nxt else {
                break;
            };
            // The winding of the circuit: past the end once and back to a
            // key beyond where the stroke began, or past the end twice,
            // the ring has been applied once.
            let closed = match wraps {
                0 => false,
                1 => began.as_deref().is_none_or(|b| k.as_slice() > b),
                _ => true,
            };
            if closed {
                break;
            }
            from = Some(k.clone());
            self.expand_cursor = Some(k.clone());
            if let Some(b) = self.bubbles.get_mut(&k) {
                let (s, p, mut a) = b.expand_with(atlas, section_at, self.now_k, crystal, budget, &mut self.pivots);
                spent += s;
                for i in 0..4 {
                    pivoted[i] = pivoted[i].saturating_add(p[i]);
                }
                adjoined.append(&mut a);
                if !b.dirty {
                    self.dirty.remove(&k);
                }
                // The pivot may have left a vortex on the form: file it at
                // its class so a ruled candidate can collapse it.
                self.refile_vortex(&k);
            } else {
                self.dirty.remove(&k);
            }
            // THE PIVOT MAY HAVE REWRITTEN THE FORM, and a bubble's key is
            // its form's address. The entry follows it, carrying the
            // inception stamp and everything the bubble has built, so the
            // form that speaks can still be found by the deliberation that
            // produced it (`Contemplation::rekey`).
            let moved_to = self
                .bubbles
                .get(&k)
                .and_then(|b| b.form.as_ref().map(super::idea::idea_key));
            if let Some(nk) = moved_to {
                self.rekey(&k, nk.as_bytes());
            }
            if let Some(e) = self.crystallize(&k, crystal) {
                crystallized.push(e);
            }
        }
        let circuit = self.ring.stroke(u64::from(wraps > 0));
        Expanded { spent, pivoted, crystallized, circuit, adjoined }
    }

    /// Attempt fill on the link of bubbles at `word`'s chart and prefixes.
    /// Merges filled simplices; speaks a closed component as one sentence.
    /// Returns (fills, formed, classes, spoken elements).
    pub fn pivot_on(
        &mut self,
        word: &Element,
        crystal: &dyn Crystal,
        budget: &mut Budget,
    ) -> (u64, u64, u64, Vec<Element>, Adjoined) {
        // 37.7's candidates from this path, carried out as `expand_all`
        // carries its own. Both paths run `pivot_orbit` and both must report
        // what it adjoined, or the mint is wired to whichever one the stroke
        // happened to take.
        let mut adjoined: Adjoined = Vec::new();
        let inv = stalk::invariant_of(word);
        // THE CAVITY IS READ AT THE CHART, not copied and filtered:
        // `cavity_key` files a retired remainder under its chart first, so
        // the entries a chart can re-hold are the SUBTREE under that chart's
        // key — `Keyed::fiber`, one keyed read per prefix of the word's chart.
        // The cavity grows by one on every retirement, every stroke, and
        // drains only here, so a read that cloned every entry to find the
        // few whose chart matched would grow without bound while the thing
        // it searched for stayed rare.
        let mut back: Vec<(Vec<u8>, Vec<Link>)> = Vec::new();
        for p in inv.prefixes() {
            let pk = p.key();
            let Some(f) = self.cavity.fiber_at(pk.as_bytes()) else {
                continue;
            };
            for (k, rem) in f {
                back.push((k, rem.clone()));
            }
        }
        for (k, rem) in back {
            self.cavity.remove(&k);
            self.hold(rem);
        }
        let mut keys = Keyed::new();
        for p in inv.prefixes() {
            for (id, _) in self.by_chart.link(&p.key()) {
                // The index names bubbles by ordinal (Z4 item 2); the
                // address is what the readers below ask the bubbles with.
                let Some(addr) = Contemplation::ordinal_of_key(&id).and_then(|o| self.address_at(o))
                else {
                    continue;
                };
                keys.insert(addr, ());
            }
        }
        let word_rotor = cech::rotor_of(word);
        // EVERY OPERATOR THE ARRIVING WORD CARRIES (Expansion 4): a form's
        // word vertices — letters among them — or a connection's letters.
        // `rotor_of` reads a connection alone and answers `None` for a
        // learned form, so read through it no vortex would be asked.
        let carried: Vec<crate::transducer::alphabet::Rotor> = crate::transducer::sentence::acting_of_element(word)
            .iter()
            .map(|t| t.word.rotor())
            .filter(|r| !r.is_identity())
            .collect();
        let mut fills = 0u64;
        let mut new_classes = 0u64;
        let mut spoken: Vec<Element> = Vec::new();
        spoken.extend(std::mem::take(&mut self.pending_rewrites));
        let bubble_keys: Vec<Vec<u8>> = keys.scoped().map(|(k, _)| k).collect();
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
            // the pivot orbit is not paced by a count that went backwards.
            let Some(t) = super::idea::held_since(self.now_k, b.moved_k) else {
                continue;
            };
            if let Some(mut form) = b.form.clone() {
                // CS2 — the cells this pivot reads against, from the idea's
                // memo where the form stands, read afresh where it moved.
                let mut memo = b.cells_read.clone();
                let cells = super::idea::cells_for_pivot(&mut memo, &form, budget);
                if let Some(b) = self.bubbles.get_mut(bk) {
                    b.cells_read = memo;
                }
                // Every snap this pivot makes is read: the adjunction is
                // carried out, and an orbit's standing demand is written on
                // the form (16.16 step 6). Both pivot paths read the same
                // snaps (law 12).
                let mut tally = sentence::PivotTally::default();
                let snaps =
                    form.pivot_orbit_read(bk, &horizon, crystal, budget, bundle.as_deref(), t, Some(&cells), &mut tally);
                self.pivots.add(&tally);
                for s in &snaps {
                    match s {
                        crate::transducer::sentence::Snap::Adjoin { rotor, chart, .. } => {
                            adjoined.push((rotor.as_ref().clone(), chart.clone()));
                        }
                        crate::transducer::sentence::Snap::Demand { rotor, chart, .. } => {
                            if let Some(Err(kind)) = crate::transducer::alphabet::factor_rotor(rotor) {
                                form.stand_demand(kind, chart, Vec::new());
                            }
                        }
                        _ => {}
                    }
                }
                if let Some(b) = self.bubbles.get_mut(bk) {
                    b.take_form(form);
                }
                self.refile_vortex(bk);
            }
        }
        // THE COLLAPSE, READ AT THE VORTEX'S OWN CLASS and not under the
        // word's chart: every bubble holding a vortex of a carried rotor's
        // class is asked, whatever chart it covers. The symmetries the
        // collapse kept join this path's adjunction candidates — an orbit that
        // admitted two discharges admits their difference, a rotor the mind
        // measured (Expansion 4, 37.7).
        {
            let chart = stalk::invariant_of(word);
            adjoined.extend(self.collapse_on(&carried, &chart, "glue"));
        }
        // index: by_chart fiber at the word's chart and peels
        for bk in &bubble_keys {
            if budget.remaining() == 0 {
                break;
            }
            // index: link of this bubble
            let simplices = self.complex.link_of(bk);
            for key in simplices {
                if !budget.spend() {
                    break;
                }
                let Some((d, fill_open, coeff_id, v0, v1, vertices)) = self.complex.simplex(&key).map(|c| {
                    // A simplex's vertices are `sorted_verts`, so the two
                    // ends are its least and next-least keys — the simplex's
                    // own order, read as a slice pattern.
                    let (v0, v1) = match c.vertices.as_slice() {
                        [a, b, ..] => (Some(a.clone()), Some(b.clone())),
                        [a] => (Some(a.clone()), None),
                        [] => (None, None),
                    };
                    (
                        c.degree(),
                        c.fill.unsettled(),
                        c.coefficient.is_identity(),
                        v0,
                        v1,
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
                            let coeff_hit = self.complex.simplex(&key).is_some_and(|c| {
                                *g == c.coefficient || (g.is_versor() && g.reverse() == c.coefficient)
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
                // 78.26(6) — a ring cut open and read again settles here.
                self.settle_decoherence(&key, &outcome);
                    match outcome {
                        CoboundaryOutcome::Attached(Fill::Filled(gamma, _carriage)) => {
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
                        CoboundaryOutcome::Attached(Fill::Congruent(..)) => {
                            self.torsion_merges += 1;
                            // KB1 (C) — the class between the two thoughts
                            // stays on this complex's simplex; it is never
                            // filed again onto the merged idea's nerve, whose
                            // vertices are its own sections (law 12).
                            let _ = self.merge(&vertices);
                        }
                        CoboundaryOutcome::Attached(Fill::Class(_, r)) => {
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
                        // 37.7 at a simplex: a rotation the seventeen do not
                        // spell. Not a class the horizon can seed a pivot
                        // from and not a fill — it is an adjunction
                        // candidate, and it is left standing rather than
                        // folded into a catch-all that would lose it.
                        CoboundaryOutcome::Attached(Fill::Unspellable(_)) => {}
                        // Evaluated and still open, evaluated and not
                        // measured (by its reason, on the simplex), already
                        // settled, or not there to evaluate — four facts,
                        // none of which drives a merge here.
                        CoboundaryOutcome::Attached(Fill::Open)
                        | CoboundaryOutcome::Attached(Fill::Unread(_))
                        | CoboundaryOutcome::AlreadyFilled
                        | CoboundaryOutcome::Absent => {}
                    }
            }
        }
        (fills, self.formed, new_classes, spoken, adjoined)
    }

}
