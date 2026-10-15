// 21 as ruled in 52.5 — EXTRACTION: the family's one form. Every chart's
// fiber is read frame by frame as a quotient in the frame's own
// coordinates; frames that read one versor are one vertex, frames that
// read another are sections along e₃, and what no quotient reaches is a
// cover or a demand. No correspondence, no candidate list, no search.
//
//! The percept — the peeled form, its sections, what move 6 reads and the
//! frames the budget did not reach — is read off the frames the caller
//! perceived once; nothing about it is held after the call (law 12). The
//! derivation against a mind's presentation runs per call (50.2 step 2),
//! and moves 3 to 7 run on the one form after it.

pub use super::finish::assignment_key;
use super::finish::finish;
use super::percept::{family_percept, Percept};
use super::*;
use crate::network::nerve::Keyed;

/// Seven moves on the family's one form (chapter 21 as ruled in 52.5),
/// under the stroke (L8w, rule 10): the family's frames perceived here, once.
/// An extraction the budget cuts off keeps what it measured and says what
/// it did not reach (rule 11): the frames it never read leave as the
/// demands of `finish`.
pub fn extract_rule_with(
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
) -> Option<crate::transducer::sentence::SentenceRule> {
    let frames = super::perceive::perceive_family(family);
    extract_rule_on(family, &frames, crystal, budget)
}

/// THE EXTRACTION ON FRAMES ALREADY PERCEIVED (rule 10, ruled 2026-09-26):
/// the caller that sized its stroke by the family's frames
/// ([`super::perceive_family`], [`super::extraction_spend`]) hands them here
/// and the percept reads them; no frame is perceived a second time.
pub fn extract_rule_on(
    family: &[(Grid, Grid)],
    frames: &super::perceive::Frames,
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
) -> Option<crate::transducer::sentence::SentenceRule> {
    if family.is_empty() {
        return None;
    }
    // 78.167 C5 — FRAMES THAT SHARE NO RATIO carry no motion the vertices
    // read, and the act may still be read off the pairs: a crop's frame is
    // the body's, never one ratio of the input's.
    let Some(scale) = family_ratio(family) else {
        if family.len() < 2 {
            return None;
        }
        return match super::acts::read_acts(&Form::default(), family, budget) {
            Ok(clauses) => {
                crate::observe::note_mind(|m| m.acts[0] += 1);
                let form = Form { clauses, ..Form::default() };
                finish(form, ((1, 1), (1, 1)), &[])
            }
            Err(u) => {
                crate::observe::note_mind(|m| m.acts[u.slot()] += 1);
                None
            }
        };
    };
    let Percept { form, varying, unreached } = family_percept(family, frames, scale, budget);
    let unreached_pairs: Vec<(Grid, Grid)> = unreached.iter().filter_map(|&k| family.get(k).cloned()).collect();
    trace::trace("extract/1 derive_against", trace::Wrote::Nothing, Some(budget));
    let mut form = derive_against(form, crystal, budget)?;
    trace::trace("extract/3 glue", trace::Wrote::Nothing, Some(budget));
    let measured = trace::carriage_of(&form);
    // 3. Glue: the placement is the vertex's own word from the Frame; the
    // crystal is asked at the COORDINATE OF WHAT EACH MEMBER DOES — its chart,
    // its whole Cl(4,1) motion and its hue map (`act_key`, A21) — one keyed
    // read, and only the held forms acting exactly so are spoken over the
    // family (21 step 3, rule 17: the key is written, the held language is
    // never walked).
    let mut asks: Vec<(Vec<u8>, TransportClass, crate::transducer::alphabet::Word)> = Vec::new();
    for (_, v) in form.vertices.scoped() {
        let (Some(tc), Some(acts), Some(placement), Vertex::Word { derivation, .. }) =
            (transport_class(v), v.acts(), v.word_in(&form.vertices), v)
        else {
            continue;
        };
        for (_, a) in acts.scoped() {
            if let Some(k) =
                crate::transducer::sentence::presentation::act_key(&a.chart, &derivation.rotor(), &a.carriage, &a.gauge)
            {
                asks.push((k, tc.clone(), placement.clone()));
            }
        }
    }
    // WHAT A CITED FORM SPEAKS OVER THE FAMILY is a function of that form,
    // the placement it is spoken at and the family, and of nothing else:
    // neither the class nor the chart the reading is made for enters
    // `speak_form`. So the reading is keyed by the cited form's address and
    // the placement written out (rule 6), measured once, and read back by
    // every class that asks at that form and that placement: the classes
    // are not a coordinate of the reading, so they are not a factor of it.
    let mut spoken: Keyed<bool> = Keyed::new();
    // THE RETRACTIONS OF THE WHOLE PASS, in one carrier (Z2 item 1). Each
    // citation retracted its class immediately, and `retract_vertices`
    // rebuilds the vertices, the faces, the cycle basis and every index of
    // the form — once per citation, 578 times on `1d0a4b61`. The vertices a
    // citation retracts are the same vertices whatever order the pass
    // reaches them in, so the set is the pass's and the rebuild is taken
    // once after it.
    let mut retract: Keyed<(), coord::VertexAt> = Keyed::new();
    for (act, tc, placement) in &asks {
        let held = crystal.on_act(act);
        {
            let mut placement_key = Vec::new();
            placement.put_canonical(&mut placement_key);
            for (e, s, _st) in &held {
                if !s.is_closed() {
                    continue;
                }
                // One read per held form spoken over the family.
                if !budget.spend() {
                    return finish(form, scale, &unreached_pairs);
                }
                let mut key = e.address().as_bytes().to_vec();
                crate::transducer::address::put_key_bytes(&mut key, &placement_key);
                let reproduces = match spoken.get(&key) {
                    Some(&r) => r,
                    None => {
                        let r = family.iter().all(|(x, y)| {
                            crystal
                                .speak_form(s, x, placement)
                                .is_some_and(|sp| painted_within_grid(&sp.painted, y))
                        });
                        spoken.insert(&key, r);
                        r
                    }
                };
                if reproduces {
                    let _ = form.glue_append(s, e.address(), placement.clone(), e.magnitude());
                    // THE VERTICES OF THE CLASS, read off the transport index
                    // the form keeps current (rule 1), never a read of every
                    // vertex asking each its class. The index is the one the
                    // form carried into the pass: an append writes no
                    // transport class, so nothing here reads a stale entry.
                    if let Some(on) = form.by_transport.at(&tc.key()) {
                        for at in on {
                            retract.put(at, ());
                        }
                    }
                }
            }
        }
    }
    // A DEMAND IS NOT SEARCHED FOR IN THE HELD LANGUAGE: a deformation is
    // answered by its motion in `derive_against` (the ear first), and a
    // source or a creation, which has no motion, stays the demand it is.
    // ONCE, AFTER THE PASS. `retract_vertices` rebuilds the cycle basis and
    // reindexes at its end, so a pass that retracted anything needs no
    // second read; a pass that cited without retracting still appended
    // faces and traverses them here.
    if retract.is_empty() {
        form.orbits = Form::orbits_of(&form.faces, &form.vertices);
        form.reindex();
    } else if !form.retract_vertices(&retract.entries().map(|(at, ())| at).collect::<Vec<_>>()) {
        trace::trace("extract/3 retract: depth unwritable", trace::Wrote::Nothing, Some(budget));
        return None;
    }
    trace::trace("extract/3 glue traversed", trace::Wrote::Nothing, Some(budget));
    trace::trace_carriage("extract/3 glue", &measured, &form);
    trace::trace("extract/4 rebase", trace::Wrote::Nothing, Some(budget));
    // 4. Rebase: w_j = t · w_i · t⁻¹ with t the relative placement, read on
    // the form's own faces. Named faces only; Frame is not a conjugator.

    rebase_named_faces(&mut form);
    trace::trace("extract/5 unfold", trace::Wrote::Nothing, Some(budget));
    // 5. Unfold grow-to-frame.
    if family.iter().all(|(a, b)| grows_to_frame(a, b)) {
        form.unfold(crate::transducer::sentence::Measure::Occupied);
    }
    trace::trace_carriage("extract/4 rebase", &measured, &form);
    // 6. Contact: placement differs across frames, the ring to the Frame is
    // one value, and the word orbits (22).
    if !contact::contact_named_vertices(&mut form, family, &varying, budget) {
        return finish(form, scale, &unreached_pairs);
    }
    trace::trace_carriage("extract/6 contact", &measured, &form);
    trace::trace("extract/7 coherence", trace::Wrote::Nothing, Some(budget));
    // 7. Coherence: the orbits are held. The tree gauge that re-solved the
    // coboundary here retired (AT1 step 2, ruled (a)): an overlap's word is
    // derived from its ends and the pivot's factors, so there is no gauge
    // for a solve to choose and the orbits a form holds are its content.
    trace::trace("extract/8 finish", trace::Wrote::Nothing, Some(budget));
    let rule = finish(form, scale, &unreached_pairs)?;
    Some(stated(rule, family, crystal, budget))
}

/// 78.167 C — THE ACT READ OFF ITS EFFECT where the vertices do not say the
/// family: a rule whose speech re-derives every pair is left as it is; one
/// whose does not carries the clauses `acts` reads, where they re-derive
/// every pair, and is otherwise left with the reason counted.
fn stated(
    mut rule: crate::transducer::sentence::SentenceRule,
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
) -> crate::transducer::sentence::SentenceRule {
    // A coordinate separating bodies on ONE pair separates nothing: a scene's
    // form (one pair) is the vertices' alone.
    if !rule.form.clauses.is_empty() || family.len() < 2 {
        return rule;
    }
    // SAID HONESTLY: the grid, every body reached, and no creation, merge or
    // reshape left unspoken — a pair right only by a body silently dropped
    // is not said (78.167 B), and its erasure is read as the act it is.
    let says = family.iter().all(|(x, y)| {
        speak::apply_form(&rule.form, x, &crate::transducer::alphabet::Word::identity(), crystal).is_some_and(|s| {
            s.unreached.is_empty() && s.demanded[2] + s.demanded[3] + s.demanded[4] == 0 && s.grid().is_some_and(|g| g == *y)
        })
    });
    if says {
        return rule;
    }
    match super::acts::read_acts(&rule.form, family, budget) {
        Ok(clauses) => {
            crate::observe::note_mind(|m| m.acts[0] += 1);
            rule.form.clauses = clauses;
        }
        Err(u) => crate::observe::note_mind(|m| m.acts[u.slot()] += 1),
    }
    rule
}

/// Extract the invariant cochain of a family.
pub fn extract_rule(
    family: &[(Grid, Grid)],
    budget: &mut crate::physics::Budget,
) -> Option<crate::transducer::sentence::SentenceRule> {
    extract_rule_with(family, &NoCrystal, budget)
}

/// THE FORM OF ONE SCENE — a family of one pair — derived against a mind's
/// presentation under a stroke. `None` is a vertex the stroke could not
/// factor: unmeasured, never a form held with the presentation's reach
/// asserted unread (rule 11).
pub fn scene_form(
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
    a: &Grid,
    b: &Grid,
) -> Option<Form> {
    let pair = [(a.clone(), b.clone())];
    extract_rule_with(&pair, crystal, budget).map(|r| r.form)
}

/// [`scene_form`] on the scene's frames already perceived, the ones its
/// stroke was sized by.
pub fn scene_form_on(
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
    a: &Grid,
    b: &Grid,
    frames: &super::perceive::Frames,
) -> Option<Form> {
    let pair = [(a.clone(), b.clone())];
    extract_rule_on(&pair, frames, crystal, budget).map(|r| r.form)
}
