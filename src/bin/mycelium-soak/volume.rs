// 78.80 — INSTRUMENTATION ONLY (§8.1): volume and density against shape. Per
// task, the single changes of the remainder (`incongruence::single_changes`)
// read by the changes' moments, and two questions asked of the slice.
// CONSERVATION: does a change keep its volume, per hue, its spread, its
// density; where it gains, is the gain all light; where it loses, all
// carving. DECISION: of the recolours in place, which key — none, volume,
// density, hue-mass, shape, direction, mass rank or density rank in the pair —
// decides the new hues, leave one out, judged by replay (`incongruence::
// decide`, the voice's own reading). Data; nothing is decided.

use mycelium::client::sdk::Epigenome;
use mycelium::transducer::family::incongruence::{decide, replayable, single_changes, Decision, KEYS};
use mycelium::transducer::family::{extract_rule_with, form_deflation};

const STROKE: usize = 1 << 20;

pub(crate) fn report(dir: &str, names: &[String]) {
    let empty = Epigenome::new();
    let (mut changes, mut mass_kept, mut hues_kept, mut spread_kept, mut density_kept) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut gained, mut gained_on_light, mut lost, mut lost_carved, mut both) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut tasks_recoloured, mut own) = (0u64, 0u64);
    let mut total = [Decision::default(); 8];
    let mut whole = [0u64; 8];
    for name in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{name}")) else { continue };
        let Some(task) = mycelium::arc::parse_task(&json) else { continue };
        let mut physics = mycelium::physics::Physics::default();
        let mut budget = physics.stroke_n(STROKE);
        let Some(rule) = extract_rule_with(&task.train, &empty, &mut budget) else { continue };
        let (_, rems) = form_deflation(&rule.form, &task.train, &empty);
        let found = single_changes(&task.train, &rems);
        for c in &found {
            let (mi, mo, d) = (&c.moments, &c.output_moments, &c.difference);
            changes += 1;
            mass_kept += u64::from(mi.mass == mo.mass);
            hues_kept += u64::from(mi.hues == mo.hues);
            spread_kept += u64::from(mi.spread == mo.spread);
            density_kept += u64::from(mi.density().is_some() && mi.density() == mo.density());
            match (d.accreted.is_empty(), d.extinguished.is_empty()) {
                (false, true) => {
                    gained += 1;
                    gained_on_light += u64::from(d.on_light == d.accreted.len() as u64);
                }
                (true, false) => {
                    lost += 1;
                    lost_carved += u64::from(d.carved == d.extinguished.len() as u64);
                }
                (false, false) => both += 1,
                (true, true) => {}
            }
        }
        own += found.iter().filter(|c| c.difference.kinematic() && !c.difference.recoloured.is_empty() && replayable(c)).count() as u64;
        let decided = decide(&found);
        let n = decided[0].predicted + decided[0].refused + decided[0].alone;
        if n == 0 {
            continue;
        }
        tasks_recoloured += 1;
        for (i, d) in decided.iter().enumerate() {
            total[i].predicted += d.predicted;
            total[i].reproduced += d.reproduced;
            total[i].refused += d.refused;
            total[i].alone += d.alone;
            whole[i] += u64::from(d.whole);
        }
        let line: Vec<String> = KEYS.iter().zip(&decided).map(|(k, d)| format!("{k} {}/{n}", d.reproduced)).collect();
        println!("  volume {name}: recolours {n} | reproduced by {}", line.join(" | "));
    }
    println!(
        "volume conservation over {changes} single changes: mass kept {mass_kept} | mass per hue kept {hues_kept} | spread kept {spread_kept} | density kept {density_kept} | gained {gained} (all on light {gained_on_light}) lost {lost} (all carved {lost_carved}) both {both}"
    );
    let n = total[0].predicted + total[0].refused + total[0].alone;
    println!(
        "volume decision over {n} recolours in place on {tasks_recoloured} tasks (leave one out, judged by replay); replayed by each task's own hue map {own} — the ceiling of any body-level key:"
    );
    for ((k, j), w) in KEYS.iter().zip(total).zip(whole) {
        println!(
            "  key {k}: predicted {} reproduced {} | refused {} alone {} | tasks whose every recolour reproduces {w} of {tasks_recoloured}",
            j.predicted, j.reproduced, j.refused, j.alone
        );
    }
}
