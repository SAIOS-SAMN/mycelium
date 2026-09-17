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

/// Extract, memoized by family id and working-atlas generation.
pub fn extract_rule_cached(
    cache: &mut crate::network::nerve::Keyed<(crate::transducer::address::Address, Option<crate::transducer::sentence::SentenceRule>)>,
    fam: &[u8],
    family: &[(Grid, Grid)],
    atlas_root: crate::transducer::address::Address,
    budget: &mut crate::physics::Budget,
    crystal: &dyn Crystal,
) -> Option<crate::transducer::sentence::SentenceRule> {
    if let Some((g, rule)) = cache.get(fam) {
        if *g == atlas_root {
            return rule.clone();
        }
    }
    if !budget.spend() {
        return cache.get(fam).and_then(|(_, r)| r.clone());
    }
    let rule = extract_rule_with(family, crystal, budget);
    cache.insert(fam, (atlas_root, rule.clone()));
    rule
}
