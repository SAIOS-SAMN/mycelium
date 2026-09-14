// 24.3 arm 1 — FEED: present the corpus, one family at a time, until the
// stroke spends.
//
//! A family is thought through whole (21, 26): the form engine is the only
//! extractor, the closed form is spoken as its sentence with its demands,
//! and a family it does not close is a withheld form. No pair leaves this
//! voice as cells (law 8, ruled 2026-09-10): the percept act is retired,
//! and the per-pair presentation that spoke it with it.

use crate::feed::{family_magnitude, retire_feed, Feed};
use crate::speak::{speak_contracted, speak_form, speak_owed, withhold_unextracted};
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
    let Stroke { entity, physics, c, outcome, m, tag } = stroke;
    // 50.9: what the last refusals contracted is uttered first, citing
    // the receipts, before any new family is presented. And 21 step 3:
    // what a court is waiting on is taught before it, so the generator a
    // sentence derives through is ruled before the sentence is heard.
    if !occluded {
        speak_contracted(entity, tag);
        speak_owed(entity, tag);
    }
    // X6 — THE FEED IS PACED AS THE QUOTE IS. The dwell, read exactly as
    // `quote_arm` reads it: the T lane is excluded entirely (C4), a lane
    // that was never pressed does not hold this voice back, and a voice
    // that has pressed nothing yet is not paced at all.
    let dwell_elapsed = [0usize, 1, 3]
        .iter()
        .any(|&l| m.last_pressure[l] > 0 && m.dwell_remaining[l] == 0);
    if m.last_pressure != [0; 4] && !dwell_elapsed {
        return;
    }
    // ONE FAMILY PER STROKE, which is what this module's own header says
    // and what the loop below did not do before its `break`: it presented families
    // until the reserve ran short, and `family_magnitude` is [0,0,0,0] for
    // a family that does not germinate, so `physics.bump` of nothing never
    // ran anything short and the drain was the whole corpus in one stroke
    // (rule 10 — a loop bounded by neither a Budget nor one cell's
    // boundary). The quote, the task and the syntropic arms ran after it,
    // once, on a stroke that had already spent itself.
    //
    // The loop that remains steps over feed files that carry no family --
    // each one is retired as it is read, so it advances by a file it
    // removes -- and stops at the first family it presents.
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
        // A task whose members have no A5 form has no identity, so there is
        // nothing to extract a rule for and nothing to name it by (U14).
        let Some(fam) = mycelium::registry::task_id(&task) else {
            retire_feed(&path);
            feed.finish();
            continue;
        };
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
                        if speech.grid().is_some_and(|g| g == *y) {
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
        break;
    }
}
