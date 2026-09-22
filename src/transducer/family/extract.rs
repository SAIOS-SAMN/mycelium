// 21 as ruled in 52.5 — EXTRACTION: the family's one form. Every chart's
// fiber is read frame by frame as a quotient in the frame's own
// coordinates; frames that read one versor are one vertex, frames that
// read another are sections along e₃, and what no quotient reaches is a
// cover or a demand. No correspondence, no candidate list, no search.
//
//! The percept — the peeled form, its sections, what move 6 reads and the
//! frames the budget did not reach — is memoised on the sensor's bytes; the
//! derivation against a mind's presentation runs per call (50.2 step 2),
//! and moves 3 to 7 run on the one form after it.

pub use super::finish::assignment_key;
use super::finish::finish;
use super::percept::{family_percept, Percept, Ratio};
use super::*;
use crate::network::nerve::Keyed;
use crate::transducer::sentence::vertex_ix_key;
use std::cell::RefCell;

/// The percept, memoised on the sensor's bytes at the sensor boundary —
/// the one place bytes may be a key (rule 6). A percept the budget cut
/// short is not memoised: the next caller with a fuller stroke would be
/// handed this one's shortfall as if it were the scene (rule 11).
struct PerceptMemo {
    family: Vec<(Grid, Grid)>,
    scale: Ratio,
    percept: Percept,
}

thread_local! {
    static PERCEPT_MEMO: RefCell<Option<PerceptMemo>> = const { RefCell::new(None) };
}

/// Seven moves on the family's one form (chapter 21 as ruled in 52.5),
/// under the stroke (L8w, rule 10). An extraction the budget cuts off keeps
/// what it measured and says what it did not reach (rule 11): the frames it
/// never read leave as the demands of `finish`.
pub fn extract_rule_with(
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
) -> Option<crate::transducer::sentence::SentenceRule> {
    if family.is_empty() {
        return None;
    }
    let scale = family_ratio(family)?;
    let hit = PERCEPT_MEMO.with(|m| {
        m.borrow().as_ref().and_then(|held| {
            (held.family.as_slice() == family && held.scale == scale).then(|| held.percept.clone())
        })
    });
    let percept = match hit {
        Some(p) => p,
        None => {
            let p = family_percept(family, scale, budget);
            if budget.remaining() > 0 && p.unreached.is_empty() {
                PERCEPT_MEMO.with(|m| {
                    *m.borrow_mut() = Some(PerceptMemo { family: family.to_vec(), scale, percept: p.clone() });
                });
            }
            p
        }
    };
    let Percept { form, mut sections, varying, unreached } = percept;
    let unreached_pairs: Vec<(Grid, Grid)> = unreached.iter().filter_map(|&k| family.get(k).cloned()).collect();
    trace::trace("extract/1 derive_against", trace::Wrote::Nothing, Some(budget));
    let mut form = derive_against(form, crystal, budget)?;
    trace::trace("extract/3 glue", trace::Wrote::Nothing, Some(budget));
    let measured = trace::carriage_of(&form);
    // 3. Glue: the placement is the vertex's own word from the Frame; the
    // crystal is asked at every chart the class acts on (21 step 3).
    //
    // THE CLASSES BY THE CHART THEY ACT ON, in chart-key order, which is
    // content: the crystal is asked once per chart, and every class
    // standing on that chart reads the one answer.
    let mut by_chart: Keyed<(Invariant, Vec<(TransportClass, crate::transducer::alphabet::Word)>)> =
        Keyed::new();
    for v in &form.vertices {
        let (Some(tc), Some(acts), Some(placement)) = (transport_class(v), v.acts(), v.word_in(&form))
        else {
            continue;
        };
        for (ck, a) in acts.scoped() {
            match by_chart.get_mut(&ck) {
                Some((_, on)) => on.push((tc.clone(), placement.clone())),
                None => {
                    by_chart.insert(&ck, (a.chart.clone(), vec![(tc.clone(), placement.clone())]));
                }
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
    // reaches them in, so the set is the pass's and the walk is taken once
    // after it.
    let mut retract: Keyed<(), coord::VertexIx> = Keyed::new();
    for (_, (moduli, classes)) in by_chart.scoped() {
        let held = crystal.on_body(moduli);
        for (tc, placement) in classes {
            if !budget.spend() {
                return finish(form, scale, sections, &unreached_pairs);
            }
            let mut placement_key = Vec::new();
            placement.put_canonical(&mut placement_key);
            for (e, s, _st) in &held {
                if !s.is_closed() {
                    continue;
                }
                let mut key = e.address().as_bytes().to_vec();
                crate::transducer::address::put_bytes(&mut key, &placement_key);
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
                    // the form keeps current (rule 1), never a walk of every
                    // vertex asking each its class. The index is the one the
                    // form carried into the pass: an append writes no
                    // transport class, so nothing here reads a stale entry.
                    if let Some(on) = form.by_transport.at(&tc.key()) {
                        for &i in on {
                            retract.put(&vertex_ix_key(i), ());
                        }
                    }
                }
            }
        }
    }
    // ONCE, AFTER THE PASS. `retract_vertices` rebuilds the cycle basis and
    // reindexes at its end, so a pass that retracted anything needs no
    // second walk; a pass that cited without retracting still appended
    // faces and walks them here.
    if retract.is_empty() {
        form.loops = Form::loops_of(&form.faces, &form.vertices);
        form.reindex();
    } else {
        form.retract_vertices(&retract);
    }
    trace::trace("extract/3 glue walked", trace::Wrote::Nothing, Some(budget));
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
    if !contact::contact_named_vertices(&mut form, family, &varying, &mut sections, budget) {
        return finish(form, scale, sections, &unreached_pairs);
    }
    trace::trace_carriage("extract/6 contact", &measured, &form);
    trace::trace("extract/7 coherence", trace::Wrote::Nothing, Some(budget));
    // 7. Coherence: tree gauge, else hold the loops.
    if !form.coheres() {
        if let Some(gamma) = form.solve_coboundary() {
            let _ = form.regauge(&gamma);
        }
    }
    trace::trace("extract/8 finish", trace::Wrote::Nothing, Some(budget));
    finish(form, scale, sections, &unreached_pairs)
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

/// The percept memo, emptied: a test that asks whether two courts reach one
/// address with no shared runtime memory clears it between them (50.9).
#[cfg(test)]
pub(crate) fn clear_percept_memo() {
    PERCEPT_MEMO.with(|m| *m.borrow_mut() = None);
}
