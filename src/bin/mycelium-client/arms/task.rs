// 24.3 arm 3 — TASK: one signed envelope per stroke from the hypha's
// registry — study the family, think the tests through, submit.
//
//! The registry rules bounty against floor, so a replayed solve collapses
//! its own yield; unique solves count only on a bounty.

use crate::speak::{speak_contracted, speak_form, speak_owed, withhold_unextracted};
use crate::feed::{family_magnitude};
use crate::Stroke;
use mycelium::physics::{Physics, StrokeOutcome};

pub(crate) fn task_arm(
    stroke: &mut Stroke<'_>,
    extract_memo: &mut mycelium::transducer::family::ExtractMemo,
    occluded: bool,
) {
let Stroke { entity, physics, c, outcome, tag, .. } = stroke;
if !occluded {
    speak_contracted(entity, tag);
    speak_owed(entity, tag);
}
// ── PULL REGISTERED WORK (PoUW): one signed envelope per stroke from
// the hypha's authoritative registry — study the family, think the
// tests through, submit. The REGISTRY rules bounty vs floor
// (anti-replay yield collapse); unique solves count ONLY on BOUNTY.
if let Ok(Some(mycelium::anastome::dialogue::TaskEnvelope { id: tid, train, tests })) =
    entity.pull_task()
{
    // The form engine is the only extractor. A family it does not
    // close is a withheld form; U1 stands it. No second extractor.
    let mag = family_magnitude(&train);
    if mycelium::economy::lanes_short(&physics.reserve_shortfall(Physics::lanes_of(&mag))).is_empty() {
        physics.bump(&mag);
        StrokeOutcome::add(&mut outcome.fed, &mag);
    }
    let gen = entity.epigenome.atlas_root();
    // Sized by the family, not by the box (L8w): its frames perceived once
    // where the memo does not answer it, the stroke sized by them and the
    // percept reading them (rule 10).
    let frames = match extract_memo.holds(&tid, &gen) {
        true => mycelium::transducer::family::Frames::new(),
        false => mycelium::transducer::family::perceive_family(&train),
    };
    let mut extract_budget =
        physics.stroke_n(mycelium::transducer::family::extraction_spend(&train, &frames));
    let extracted = mycelium::transducer::family::extract_rule_cached(
        extract_memo,
        &tid,
        &train,
        gen,
        &mut extract_budget,
        &entity.epigenome,
        Some(&frames),
    );
    eprintln!(
        "[{tag}] [{}] family {} {} pairs, {} tests: {}",
        mycelium::chronometry::Height::stamp_of(entity.now_tu()),
        crate::speak::fam_id(&tid),
        train.len(),
        tests.len(),
        match extracted.as_ref() {
            Some(r) => format!(
                "rule {} vertices, {} faces, sections {}",
                r.form.vertices.len(),
                r.form.faces.len(),
                r.section_count()
            ),
            None => "extract withheld".to_string(),
        },
    );
    if let Some(rule) = extracted.as_ref() {
        if let Some(mag) = speak_form(entity, tag, rule, &train, &frames, &tid, &mut extract_budget) {
            StrokeOutcome::add(&mut outcome.uttered, &mag);
        }
        let outs: Option<Vec<mycelium::arc::Grid>> = tests
            .iter()
            .map(|x| {
                let spoken = mycelium::transducer::family::speak_rule(rule, x, &entity.epigenome);
                if let Some(s) = &spoken {
                    if !s.remainder.is_empty() {
                        entity.hold_chain(
                            s.remainder.iter().map(|&(r, c, h)| (r, c, h, h)).collect(),
                        );
                    }
                }
                spoken.filter(|s| s.remainder.is_empty()).and_then(|s| s.grid())
            })
            .collect();
        if let Some(outs) = outs {
            // An unmeasurable span mints nothing, and a solve with no rule
            // is not sent (`solve_task`): the voice sends only a rule its
            // own train does not refute, and not one the court refused.
            let spoken = entity.span_of(&rule.form).and_then(|d| {
                mycelium::transducer::family::mint_sentence(
                    rule,
                    &train,
                    &entity.epigenome,
                    d.span(),
                )
            });
            if let Some((code, _mu)) = entity.solve_task(&tid, &train, &outs, spoken.as_ref(), &mut extract_budget) {
                c.projected += 1;
                match code {
                    mycelium::anastome::dialogue::SOLVE_BOUNTY => {
                        c.proj_solved += 1;
                        c.uniq_solved += 1;
                        c.solves_paid += 1;
                    }
                    mycelium::anastome::dialogue::SOLVE_FLOOR => c.proj_solved += 1,
                    _ => {}
                }
            }
        }
    } else {
        withhold_unextracted(entity, tag, &train, &tid);
    }
}

}
