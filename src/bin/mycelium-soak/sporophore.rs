// 78.88 — INSTRUMENTATION ONLY (§8.1): does reading a scene by its
// SPOROPHORE — the enclosure its bodies' centres join (`cga::enclosure`), the
// lexicon's "an enclosure, the join of centres, nested" — matter for
// perception now? Per train pair: whether the frame's extent changes (and
// in proportion); the input bodies some output body on their chart is
// carried onto (`carriage`) and those none is, by whether the extent
// changed; and of the carriages, those whose motion also carries the input
// scene's sporophore onto the output's (the body moved WITH its scene).
// Read projectively, no root. Data; nothing is decided.

use mycelium::transducer::cga::{enclosure, projective_ratio};
use mycelium::transducer::family::{carriage, perceive};

const STROKE: usize = 1 << 20;

#[derive(Default)]
struct Tally {
    pairs: u64,
    resized: u64,
    proportional: u64,
    /// Input bodies carried / uncarried, in pairs whose extent is kept and
    /// in pairs whose extent changed.
    carried_kept: u64,
    uncarried_kept: u64,
    carried_resized: u64,
    uncarried_resized: u64,
    /// Pairs whose sporophores both read (no flat enclosure), and of them
    /// those whose sporophore stands still (one sphere, projectively).
    enclosed: u64,
    still: u64,
    /// Carriages read against both sporophores, and those whose motion
    /// carries the input's onto the output's: in kept and in resized pairs.
    asked_kept: u64,
    with_scene_kept: u64,
    asked_resized: u64,
    with_scene_resized: u64,
    /// The changed extents by kind: the output inside the input on every
    /// axis (cut), around it (grown), a whole multiple of it on every axis
    /// (tiled, a kind of grown), or larger on one axis and smaller on another.
    cut: u64,
    grown: u64,
    tiled: u64,
    mixed: u64,
}

pub(crate) fn report(dir: &str, names: &[String]) {
    let mut t = Tally::default();
    for name in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{name}")) else { continue };
        let Some(task) = mycelium::arc::parse_task(&json) else { continue };
        let mut physics = mycelium::physics::Physics::default();
        for (x, y) in &task.train {
            let mut budget = physics.stroke_n(STROKE);
            t.pairs += 1;
            let (ex, ey) = (mycelium::arc::extents(x), mycelium::arc::extents(y));
            let resized = ex != ey;
            t.resized += u64::from(resized);
            // In proportion: one ratio on every axis.
            t.proportional += u64::from(resized && ex[0] * ey[1] == ex[1] * ey[0]);
            if resized {
                let (le, ge) = (ey.iter().zip(&ex).all(|(o, i)| o <= i), ey.iter().zip(&ex).all(|(o, i)| o >= i));
                match (le, ge) {
                    (true, _) => t.cut += 1,
                    (_, true) => {
                        t.grown += 1;
                        t.tiled += u64::from(ey.iter().zip(&ex).all(|(o, i)| *i > 0 && o % i == 0));
                    }
                    _ => t.mixed += 1,
                }
            }
            let (ins, outs) = (perceive(x), perceive(y));
            let nuclei = |bs: &[mycelium::transducer::family::Body]| -> Vec<_> { bs.iter().filter_map(|b| b.shape.blade.vertex_nucleus()).collect() };
            let spores = (enclosure(&nuclei(&ins)), enclosure(&nuclei(&outs)));
            if let (Some(si), Some(so)) = &spores {
                t.enclosed += 1;
                t.still += u64::from(projective_ratio(si, so).is_some());
            }
            for a in &ins {
                let mut any = false;
                for b in &outs {
                    if a.moduli.key() != b.moduli.key() {
                        continue;
                    }
                    let Some(c) = carriage(a, b, &mut budget) else { continue };
                    any = true;
                    if let (Some(si), Some(so)) = &spores {
                        let with = projective_ratio(&c.v.sandwich(si), so).is_some();
                        match resized {
                            true => {
                                t.asked_resized += 1;
                                t.with_scene_resized += u64::from(with);
                            }
                            false => {
                                t.asked_kept += 1;
                                t.with_scene_kept += u64::from(with);
                            }
                        }
                    }
                }
                match (resized, any) {
                    (false, true) => t.carried_kept += 1,
                    (false, false) => t.uncarried_kept += 1,
                    (true, true) => t.carried_resized += 1,
                    (true, false) => t.uncarried_resized += 1,
                }
            }
        }
    }
    println!(
        "sporophore: pairs {} | extent changed {} (in proportion {}) | sporophores read on both sides {} (standing still {})",
        t.pairs, t.resized, t.proportional, t.enclosed, t.still
    );
    println!("sporophore extent changes: cut {} | grown {} (whole multiples, tiled {}) | mixed {}", t.cut, t.grown, t.tiled, t.mixed);
    println!(
        "sporophore bodies: extent kept — carried {} uncarried {} | extent changed — carried {} uncarried {}",
        t.carried_kept, t.uncarried_kept, t.carried_resized, t.uncarried_resized
    );
    println!(
        "sporophore carriages moving WITH their scene: extent kept {} of {} | extent changed {} of {}",
        t.with_scene_kept, t.asked_kept, t.with_scene_resized, t.asked_resized
    );
}
