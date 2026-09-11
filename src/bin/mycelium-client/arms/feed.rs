// 24.3 arm 1 — FEED: present the corpus, one family at a time, until the
// stroke spends.
//
//! A family is thought through whole (21, 26): the form engine is the only
//! extractor, the closed form is spoken as its sentence with its demands,
//! and a family it does not close is a withheld form. No pair leaves this
//! voice as cells (law 8, ruled 2026-09-10): the percept act is retired,
//! and the per-pair presentation that spoke it with it.

use crate::feed::{family_magnitude, retire_feed, Feed};
use crate::speak::{speak_contracted, speak_form, withhold_unextracted};
use crate::Stroke;
use mycelium::arc::parse_task;
use mycelium::physics::{Physics, StrokeOutcome};

pub(crate) fn feed_arm(
    stroke: &mut Stroke<'_>,
    feed: &mut Feed,
    extract_memo: &mut mycelium::network::nerve::Keyed<(
        mycelium::transducer::address::Address,
        Option<mycelium::transducer::sentence::SentenceRule>,
    )>,
    occluded: bool,
) {
    let Stroke { entity, physics, c, outcome, tag, .. } = stroke;
    // 50.9: what the last refusals contracted is uttered first, citing
    // the receipts, before any new family is presented.
    if !occluded {
        speak_contracted(entity, tag);
    }
    while let Some(path) = feed.next() {
        if occluded {
            break;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            retire_feed(&path);
            feed.finish();
            continue;
        };
        let Some(task) = parse_task(&text) else {
            retire_feed(&path);
            feed.finish();
            continue;
        };
        // THE FAMILY WHOLE — the form engine is the only extractor. A family
        // it does not close is a withheld form (U1 stands it).
        let mag = family_magnitude(&task.train);
        if !mycelium::economy::lanes_short(&physics.reserve_shortfall(Physics::lanes_of(&mag))).is_empty() {
            break;
        }
        physics.bump(&mag);
        StrokeOutcome::add(&mut outcome.fed, &mag);
        let fam = mycelium::registry::task_id(&task);
        let gen = entity.epigenome.atlas_root();
        // Sized by the family, not by the box (L8w): a fixed spend unit is
        // three on this machine, and step 0 alone needs one per pair with
        // the cover walk out of the same purse.
        let mut extract_budget = physics
            .stroke_n(mycelium::transducer::family::extraction_spend(&task.train));
        let extracted = mycelium::transducer::family::extract_rule_cached(
            extract_memo,
            &fam,
            &task.train,
            gen,
            &mut extract_budget,
            &entity.epigenome,
        );
        if let Some(rule) = extracted.as_ref() {
            for (x, y) in &task.test {
                if y.is_empty() {
                    continue;
                }
                if let Some(speech) = mycelium::transducer::family::speak_rule(rule, x, &entity.epigenome) {
                    if speech.remainder.is_empty() {
                        c.projected += 1;
                        if speech.grid() == *y {
                            c.proj_solved += 1;
                        }
                    }
                }
            }
            let _ = speak_form(entity, tag, rule, &task.train, &fam);
        } else {
            withhold_unextracted(entity, tag, &task.train, &fam);
        }
        retire_feed(&path);
        feed.finish();
    }
}
