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
/// 37.7's ADJUNCTION CANDIDATES: a rotor the seventeen cannot spell and the
/// chart it was met on. Named because BOTH of the drive's paths produce them
/// — the expand ring and `pivot_on` — and a type they share is how the two
/// stay one reading (law 12).
pub type Adjoined = Vec<(crate::transducer::alphabet::Rotor, crate::transducer::stalk::Invariant)>;

pub struct Expanded {
    pub spent: usize,
    pub pivoted: [u64; 4],
    pub crystallized: Vec<Element>,
    pub orbit: Option<(u64, u64)>,
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
        // index: budgeted keyed cursor over dirty bubbles — an orbit (45.7,
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
            // The winding of the walk: past the end once and back to a
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
                let (s, p, mut a) = b.expand_with(atlas, section_at, self.now_k, crystal, budget);
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
        let orbit = self.ring.stroke(u64::from(wraps > 0));
        Expanded { spent, pivoted, crystallized, orbit, adjoined }
    }

    /// Attempt fill on the link of bubbles at `word`'s chart and prefixes.
    /// Merges filled cells; speaks a closed component as one sentence.
    /// Returns (fills, formed, classes, spoken elements).
    pub fn pivot_on(
        &mut self,
        word: &Element,
        crystal: &dyn Crystal,
        budget: &mut Budget,
    ) -> (u64, u64, u64, Vec<Element>, Adjoined) {
        // 37.7's candidates from this path, carried out as `expand_all`
        // carries its own. Both paths run `pivot_loop` and both must report
        // what it adjoined, or the mint is wired to whichever one the stroke
        // happened to take.
        let mut adjoined: Adjoined = Vec::new();
        let inv = stalk::invariant_of(word);
        // THE CAVITY IS READ AT THE CHART, not copied and filtered. This
        // cloned EVERY entry — key and the whole `Vec<RemCell>` — on every
        // call, re-derived each one's chart, and kept the few whose prefix
        // matched the arriving word's. The cavity grows by one on every
        // retirement, every stroke, and drains only on that coincidence, so
        // the clone grew without bound on the pivot path while the thing it
        // was searching for stayed rare.
        //
        // `cavity_key` files a retired remainder under its chart first, so
        // the entries a chart can re-hold are the SUBTREE under that chart's
        // key — `Keyed::fiber`, one descent per prefix of the word's chart,
        // which is the same set the filter selected and none of the copying.
        let mut back: Vec<(Vec<u8>, Vec<RemCell>)> = Vec::new();
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
                keys.insert(&id, ());
            }
        }
        let word_rotor = cech::rotor_of(word);
        // EVERY OPERATOR THE ARRIVING WORD CARRIES (Expansion 4): a form's
        // word vertices — letters among them since R55 — or a connection's
        // letters. `rotor_of` reads a connection alone and answered `None`
        // for every learned form, so no vortex was ever asked.
        let carried: Vec<crate::transducer::alphabet::Rotor> = crate::transducer::sentence::words_of_element(word)
            .iter()
            .map(|w| w.rotor())
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
            // the pivot loop is not paced by a count that went backwards.
            let Some(t) = super::idea::held_since(self.now_k, b.moved_k) else {
                continue;
            };
            if let Some(mut form) = b.form.clone() {
                // EVERY SNAP THIS PIVOT MAKES, not none of them. `let _ =`
                // stood here and threw away the whole return — the demand,
                // the torsion filing, the vortex and the adjunction — so a
                // pivot on this path made its moves and reported nothing.
                //
                // Measured at 1d98080, one recut: `adjoin from residue` 8
                // and every drain outcome 0 — not `letter adjoined`, not
                // `letter held`, not `letter refused`. The drain never ran,
                // because `Expanded.adjoined` was empty while the pivot had
                // emitted eight adjunctions. R21 carried `Snap::Adjoin` out
                // through `expand_with`, which keeps its snaps, and not
                // through this, which discards them. The mint was wired to
                // one of the two paths a pivot runs on.
                let snaps = form.pivot_loop(bk, &horizon, crystal, budget, bundle.as_deref(), t);
                for s in &snaps {
                    if let crate::transducer::sentence::Snap::Adjoin { rotor, chart, .. } = s {
                        adjoined.push((rotor.as_ref().clone(), chart.clone()));
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
        // collapse kept join this path's adjunction candidates — a loop that
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
            let cells = self.complex.link_of(bk);
            for key in cells {
                if !budget.spend() {
                    break;
                }
                let Some((d, fill_open, coeff_id, v0, v1, vertices)) = self.complex.cell(&key).map(|c| {
                    // A cell's vertices are `sorted_verts`, so the two ends
                    // are its least and next-least keys — the cell's own
                    // order, read as a slice pattern.
                    let (v0, v1) = match c.vertices.as_slice() {
                        [a, b, ..] => (Some(a.clone()), Some(b.clone())),
                        [a] => (Some(a.clone()), None),
                        [] => (None, None),
                    };
                    (
                        c.degree(),
                        matches!(c.fill, Fill::Open),
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
                                    // that vertex: both ends are it, which
                                    // the slice pattern says on the cell's
                                    // own sorted vertices.
                                    let (a, c) = match vertices.as_slice() {
                                        [a, b, ..] => (a.clone(), b.clone()),
                                        [a] => (a.clone(), a.clone()),
                                        [] => continue,
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
        (fills, self.formed, new_classes, spoken, adjoined)
    }

}
