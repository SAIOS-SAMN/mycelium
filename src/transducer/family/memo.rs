// 50.2 step 2 — THE SECOND STEP: the peeled form's word vertices derived
// against a mind's presentation, per call and per mind, and the rule's
// memo by task and atlas root.
//
//! The percept is memoised in `extract.rs` on the sensor's bytes; the
//! derivation is a function of the mind that is speaking and is never
//! cached beside it — a presentation is content, never a cache key.

use super::*;

/// 50.2 step 2 — THE SECOND STEP, RUN PER CALL AND PER MIND. The peeled
/// form's word vertices are spelled from the identity; each is factored
/// through the presentation this crystal holds, and the vertex carries
/// what came back. No root and no key: the mind IS the argument.
///
/// `None` is a vertex the budget could not measure. It is not a form
/// with that vertex spelled — that would assert the presentation reached
/// nothing when nothing was looked at (rule 11) — so the whole form is
/// unmeasured and the caller withholds and resumes next stroke (L8w).
pub(crate) fn derive_against(
    mut peeled: Form,
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
) -> Option<Form> {
    for v in &mut peeled.vertices {
        match v {
            // THE `𝒢` LANE IS THE VERTEX'S (42.6 as amended, L12). A
            // vertex's GAUGE says whether it recolours, and a target that
            // recolours is not derived by a generator that only turns.
            // Read from the derivation alone, every target looked purely
            // holonomic and reduced against `𝒽`'s basis whatever hue it
            // carried.
            // THE DERIVATION IS THE VERTEX'S, and the gauge it is read
            // against is the representative member's: one member today, and
            // D7b decides what a multi-member vertex spells against.
            Vertex::Word { derivation, acts, .. } => {
                let gauge = match acts.scoped().next() {
                    Some((_, a)) => a.gauge.clone(),
                    None => Vec::new(),
                };
                *derivation = derivation_of_spelled(crystal, budget, derivation, &gauge)?;
            }
            // 37.7 AT THE SECOND STEP. A demand the peel typed is not
            // final: the peel spells with the seventeen alone, and this
            // mind may hold an adjoined generator that divides the very
            // rotor the peel stood on. The ring is asked here and nowhere
            // earlier, because the percept is memoised and a presentation
            // is content, never a cache key (50.2 step 2).
            //
            // A demand the ring answers becomes a WORD vertex carrying the
            // derivation it found. One it does not answer stays exactly the
            // demand F typed — the ring having reached nothing is a
            // reading, not a downgrade of the demand. THE RING ANSWERS THE
            // PEEL'S DEMANDS AND NO OTHER: a `Source`, a `Creation` or a
            // deformation stands for bodies the eyes could not pair (50.5),
            // and a versor measured between its links would pair them
            // behind the chart's back — the word the court refuses for not
            // carrying every body of its chart.
            Vertex::Demand {
                kind:
                    crate::transducer::sentence::Demand::RotorPrime { .. }
                    | crate::transducer::sentence::Demand::Standing { .. },
                ..
            } => {
                if let Some(word) = ring_answer(crystal, budget, v) {
                    *v = word;
                }
            }
            _ => {}
        }
    }
    // `reindex` reads the composed word of every word vertex, so it runs
    // after the derivations land and not before.
    peeled.reindex();
    Some(peeled)
}

/// 37.7 — ASK THE RING WHETHER THIS MIND CLOSES A DEMAND THE PEEL LEFT.
///
/// The demand carries the bodies it stands for (50.5), so the carriage is
/// RE-DERIVED from them rather than stored beside them (law 12) — the two
/// bodies are the chain's links split by which side of the transition they
/// occupy. `factor_versor` then runs the peel and, where it stands, the
/// ring; a derivation that comes back is one this mind reached through a
/// generator the seventeen cannot spell, and it is a word.
///
/// `None` is every other outcome and leaves the demand exactly as F typed
/// it: no versor between the bodies, a budget that ended, or a ring that
/// divided nothing. None of those is a smaller demand (rule 11).
fn ring_answer(
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
    v: &Vertex,
) -> Option<Vertex> {
    let Vertex::Demand { bodies, .. } = v else {
        return None;
    };
    // The demand's bodies are one section's links (16.5), read in the
    // section's own frame.
    let before: Vec<Lamina> = bodies.iter().filter(|c| c.2 != 0).map(|c| (c.0, c.1, 0, c.2)).collect();
    let after: Vec<Lamina> = bodies.iter().filter(|c| c.3 != 0).map(|c| (c.0, c.1, 0, c.3)).collect();
    if before.is_empty() || after.is_empty() {
        return None;
    }
    let (a, b) = (body_of(before), body_of(after));
    let car = carriage(&a, &b)?;
    // The carriage's OWN hue map is the gauge: this demand's target
    // recolours exactly where `sigma` is not the identity, so the `𝒢` lane
    // is occupied or not by what was measured (L12).
    let derivation = factor_versor(crystal, budget, &car.v, &car.sigma)?.ok()?;
    word_vertex_with(&a, &b, &car, derivation)
}

/// Re-derive the family's rule and check it agrees with `rule`.
pub fn rule_replays_family(
    rule: &crate::transducer::sentence::SentenceRule,
    family: &[(Grid, Grid)],
    budget: &mut crate::physics::Budget,
) -> bool {
    let Some(again) = extract_rule(family, budget) else {
        return false;
    };
    crate::transducer::sentence::form_vertex_key(&again.form)
        == crate::transducer::sentence::form_vertex_key(&rule.form)
}

/// THE LAST FAMILY'S RULE, memoised by the family it is for and the
/// working atlas it was read against — ONE entry, the shape
/// `extract.rs`'s own `PERCEPT_MEMO` already has at the sensor boundary.
///
/// IT WAS A `Keyed` WITH NO EVICTION, holding one whole `SentenceRule` per
/// task id the voice had ever pulled, and that is the fleet's OOM.
/// Measured on voice 4 of the 2026-09-19 recut, 18 minutes in: 89 pulls,
/// 54 distinct families, task arm +522 MB net — **9.7 MB per stored rule**,
/// which is a `Form`'s vertices, faces, loops and carriers. The training
/// corpus is 1001 tasks, so a voice that sees it once holds **9.4 GB**, and
/// five voices share 15 GB. Every voice OOMed; survival was 1 h 33 m to
/// 4 h 53 m and the arm was recorded as unattributed. It is this.
///
/// AND THE UNBOUNDED CACHE BOUGHT NOTHING. Its hit rate was 39% (89 pulls,
/// 54 distinct), and the CONSECUTIVE-repeat rate over the same journal is
/// 39% on voice 4 and 40% on voice 1 — every hit was the envelope the
/// hypha had just handed back. One entry captures the whole of it.
///
/// The key is the family's own id and the atlas root it was read against,
/// both compared as content: a pull of another family, or of this one
/// after the mind has learned, is a miss and re-extracts. Nothing is
/// evicted by recency or by a bound — there is one slot, and what is in it
/// is what was last read (rule 4: nothing chosen by a scalar).
#[derive(Default)]
pub struct ExtractMemo {
    held: Option<(Vec<u8>, crate::transducer::address::Address, Option<crate::transducer::sentence::SentenceRule>)>,
}

impl ExtractMemo {
    pub fn new() -> Self {
        Self::default()
    }

    /// What the memo holds: one rule, or none. Read by the voice's own
    /// stroke line, where it was `extract_memo.len()`.
    pub fn len(&self) -> usize {
        usize::from(self.held.is_some())
    }

    pub fn is_empty(&self) -> bool {
        self.held.is_none()
    }

    /// The rule this memo holds for `fam` at `atlas_root`, or `None`.
    fn get(&self, fam: &[u8], atlas_root: &crate::transducer::address::Address) -> Option<&Option<crate::transducer::sentence::SentenceRule>> {
        match &self.held {
            Some((f, g, r)) if f == fam && g == atlas_root => Some(r),
            _ => None,
        }
    }
}

/// Extract, memoised on the last family and the working-atlas generation.
///
/// A budget that ends before the extraction starts returns what the memo
/// holds for THIS family and this atlas, and nothing otherwise: a rule read
/// for another family is not this family's answer (rule 11). It read
/// `cache.get(fam)` without the atlas check there, so a starved stroke
/// could hand back a rule the mind had already outgrown.
pub fn extract_rule_cached(
    cache: &mut ExtractMemo,
    fam: &[u8],
    family: &[(Grid, Grid)],
    atlas_root: crate::transducer::address::Address,
    budget: &mut crate::physics::Budget,
    crystal: &dyn Crystal,
) -> Option<crate::transducer::sentence::SentenceRule> {
    if let Some(rule) = cache.get(fam, &atlas_root) {
        return rule.clone();
    }
    if !budget.spend() {
        return cache.get(fam, &atlas_root).and_then(|r| r.clone());
    }
    let rule = extract_rule_with(family, crystal, budget);
    cache.held = Some((fam.to_vec(), atlas_root, rule.clone()));
    rule
}
