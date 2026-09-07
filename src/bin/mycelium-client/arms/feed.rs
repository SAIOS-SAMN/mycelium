// 24.3 arm 1 — FEED: present corpus transitions until the stroke spends.
//
//! A percept is a PROOF spoken over this genome: what the genome reduces is
//! cited, what it cannot is this voice's irreducible claim. The sync
//! converges the genome between strokes.

use crate::speak::{speak_form, withhold_unextracted};
use crate::feed::{family_magnitude, retire_feed, Feed};
use crate::Counts;
use mycelium::arc::{embed, grid_to_cells, parse_task};
use mycelium::physics::{Physics, StrokeOutcome};
use mycelium::sdk::{Comprehension, Entity};
use mycelium::transducer::element::measure_lanes;
use mycelium::transducer::germinate::germinate;

#[allow(clippy::too_many_arguments)]
pub(crate) fn feed_arm(
    entity: &mut Entity,
    physics: &mut Physics,
    feed: &mut Feed,
    extract_memo: &mut mycelium::network::nerve::Keyed<(
        mycelium::transducer::a21::Address,
        Option<mycelium::transducer::sentence::SentenceRule>,
    )>,
    c: &mut Counts,
    outcome: &mut StrokeOutcome,
    last_height: &mut u64,
    last_pressure: &mut [u64; 4],
    dwell_remaining: &mut [u64; 4],
    occluded: bool,
    tag: &str,
) {
// ── FEED (entropic): present corpus transitions until the stroke
// spends. A percept is a PROOF spoken over this genome: what the
// genome reduces is cited, what it cannot is this voice's
// irreducible claim; the sync converges the genome between strokes.
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
    let Some((input, output)) = task.train.get(feed.pair_index()) else {
        // TASK BOUNDARY — the form engine is the only extractor.
        // A family it does not close is a withheld form (U1 stands it).
        let mag = family_magnitude(&task.train);
        if !physics.can_expand(Physics::lanes_of(&mag)) {
            break;
        }
        physics.bump(&mag);
        StrokeOutcome::add(&mut outcome.fed, &mag);
        let fam = mycelium::registry::task_id(&task);
        let gen = entity.epigenome.atlas_root();
        let mut extract_budget = physics.stroke();
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
        continue;
    };
    feed.advance_pair();
    // THE EYE READS THROUGH ρ⁻¹ (sensor hardware, ruled): a
    // dimension-changing pair is spoken on the codomain complex — the
    // input pulled back along the exact frame map, the output native —
    // so the engine digests an ordinary chain bounded by the output.
    let (hi, wi) = (input.len(), input.first().map_or(0, |r| r.len()));
    let (ho, wo) = (output.len(), output.first().map_or(0, |r| r.len()));
    if hi == 0 || wi == 0 || ho == 0 || wo == 0 {
        continue;
    }
    let (a, b) = (grid_to_cells(&embed(input, ho, wo)), grid_to_cells(output));
    if a.is_empty() || b.is_empty() {
        continue;
    }
    let Some(g) = germinate(&a, &b) else {
        continue;
    };
    let mag = measure_lanes(&g);
    if !physics.can_expand(Physics::lanes_of(&mag)) {
        break;
    }
    match entity.submit(&a, &b, &(hi, wi, ho, wo)) {
        Ok((reply, heard)) => {
            c.fed += 1;
            if reply.settle.is_some() {
                c.settles += 1;
            }
            match heard {
                Some(Comprehension::Understood(_)) | Some(Comprehension::Held) | Some(Comprehension::Admitted) => c.derived += 1,
                Some(Comprehension::Novel { .. })
                | Some(Comprehension::Dialect(_))
                | Some(Comprehension::Withheld { .. }) => c.witnessed += 1,
                None => {}
            }
            physics.bump(&mag);
            StrokeOutcome::add(&mut outcome.fed, &mag);
            *last_pressure = reply.pressure;
            *last_height = reply.height;
            // C4. T NEVER PACES A STROKE. Lane 2 is temporal extent
            // -- a COST the listing carries, counted in heights
            // (law 9, 45.4) -- not a coordinate a voice can expand
            // on. The hypha's reported T pressure is the standing
            // extent of listings that have not closed, so it climbs
            // for as long as the book holds anything and never
            // falls; dwelling on it paced voices into a stop, the
            // soak reading `dwell [.., .., 909 -> 4048, ..]`. The
            // voice dwells on the lanes it can actually expand.
            *dwell_remaining = reply.pressure;
            dwell_remaining[2] = 0;
        }
        Err(_) => {
            c.errors += 1;
        }
    }
}

}
