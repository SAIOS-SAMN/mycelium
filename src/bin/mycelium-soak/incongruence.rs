// 78.79 — INSTRUMENTATION ONLY (§8.1): the interpreter for incongruence over a
// corpus slice. Per task, every unsaid output body standing on exactly one
// input body read as a constructor difference; the readings of one subject
// (the input body's chart, across the family's pairs) held together as a
// coherence manifold — core, edges, resolution, phase, what each reading
// moved — and the core judged by the plane: replayed on every reading's input
// body, it reproduces the output or it does not. Data; nothing is decided.

use mycelium::client::sdk::Epigenome;
use mycelium::transducer::family::incongruence::{contrast, difference, manifold, relative, reproduces, Cell};
use mycelium::transducer::family::{extract_rule_with, form_deflation, perceive};

const STROKE: usize = 1 << 20;

/// One reading of a subject: its input and output bodies, its relative
/// difference, and its light's runs.
type Reading = (Vec<Cell>, Vec<Cell>, mycelium::transducer::family::incongruence::Relative, Vec<(u64, bool)>);

fn cells(b: &mycelium::transducer::family::Body) -> Vec<Cell> {
    b.compartments.iter().filter_map(|l| Some((l.site.axis(0)?, l.site.axis(1)?, l.hue))).collect()
}

pub(crate) fn report(dir: &str, names: &[String]) {
    let empty = Epigenome::new();
    let (mut changes, mut kinematic, mut accreted, mut extinguished, mut recoloured, mut on_light, mut carved) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut subjects, mut solo, mut multi) = (0u64, 0u64, 0u64);
    let (mut judged, mut reproduced, mut subjects_whole) = (0u64, 0u64, 0u64);
    let (mut refined, mut closed, mut sharpened, mut recognized) = (0u64, 0u64, 0u64, 0u64);
    let (mut holonomic, mut luminal, mut quantum_one, mut quantum_more) = (0u64, 0u64, 0u64, 0u64);
    let (mut core_nonempty, mut edges) = (0u64, 0u64);
    let (mut c_shared, mut c_first, mut c_second) = (0u64, 0u64, 0u64);
    let (mut tasks_whole, mut tasks_judged) = (0u64, 0u64);
    for name in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{name}")) else { continue };
        let Some(task) = mycelium::arc::parse_task(&json) else { continue };
        let mut physics = mycelium::physics::Physics::default();
        let mut budget = physics.stroke_n(STROKE);
        let Some(rule) = extract_rule_with(&task.train, &empty, &mut budget) else { continue };
        let (_, rems) = form_deflation(&rule.form, &task.train, &empty);
        if rems.iter().all(Vec::is_empty) {
            continue;
        }
        // Readings by subject: the input body's chart, its key the coordinate.
        let mut by_subject: Vec<(Vec<u8>, Vec<Reading>)> = Vec::new();
        for ((x, y), rem) in task.train.iter().zip(&rems) {
            if x.len() != y.len() || x.iter().zip(y).any(|(a, b)| a.len() != b.len()) {
                continue;
            }
            let unsaid: Vec<(i64, i64)> = rem.iter().filter(|l| l.3 != 0).map(|l| (l.0, l.1)).collect();
            let ins = perceive(x);
            for out in perceive(y) {
                let o = cells(&out);
                if !o.iter().any(|c| unsaid.contains(&(c.0, c.1))) {
                    continue;
                }
                let standing: Vec<&mycelium::transducer::family::Body> =
                    ins.iter().filter(|i| cells(i).iter().any(|c| o.iter().any(|d| d.0 == c.0 && d.1 == c.1))).collect();
                let [input] = standing.as_slice() else { continue };
                let i = cells(input);
                let d = difference(&i, &o, y);
                changes += 1;
                kinematic += u64::from(d.kinematic());
                accreted += d.accreted.len() as u64;
                extinguished += d.extinguished.len() as u64;
                recoloured += d.recoloured.len() as u64;
                on_light += d.on_light;
                carved += d.carved;
                let Some(rel) = relative(&i, &d) else { continue };
                let key = input.moduli.key().as_bytes().to_vec();
                let entry = (i, o, rel, d.runs.clone());
                match by_subject.iter_mut().find(|(k, _)| *k == key) {
                    Some((_, v)) => v.push(entry),
                    None => by_subject.push((key, vec![entry])),
                }
            }
        }
        let (mut t_judged, mut t_reproduced) = (0u64, 0u64);
        for (_, readings) in &by_subject {
            subjects += 1;
            if readings.len() < 2 {
                solo += 1;
                continue;
            }
            multi += 1;
            let held: Vec<_> = readings.iter().map(|(_, _, r, runs)| (r.clone(), runs.clone())).collect();
            let m = manifold(&held);
            core_nonempty += u64::from(!m.core.is_empty() || !m.core_recolours.is_empty());
            edges += m.edges;
            holonomic += m.holonomic;
            luminal += m.luminal;
            match m.quantum {
                Some(1) => quantum_one += 1,
                Some(_) => quantum_more += 1,
                None => {}
            }
            for mv in &m.moved {
                refined += u64::from(mv.refined);
                closed += u64::from(mv.closed);
                sharpened += u64::from(mv.sharpened);
                recognized += u64::from(mv.recognized);
            }
            for pair in m.readings.windows(2) {
                let c = contrast(&pair[0], &pair[1]);
                c_shared += c.shared;
                c_first += c.only_first;
                c_second += c.only_second;
            }
            let mut all = true;
            for (i, o, _, _) in readings {
                judged += 1;
                t_judged += 1;
                let ok = reproduces(&m, i, o) == Some(true);
                reproduced += u64::from(ok);
                t_reproduced += u64::from(ok);
                all &= ok;
            }
            subjects_whole += u64::from(all);
        }
        if t_judged > 0 {
            tasks_judged += 1;
            tasks_whole += u64::from(t_judged == t_reproduced);
        }
        println!(
            "  incongruence {name}: subjects {} (of several readings {}) | readings judged {t_judged} reproduced by the core {t_reproduced}",
            by_subject.len(),
            by_subject.iter().filter(|(_, r)| r.len() >= 2).count()
        );
    }
    println!(
        "incongruence: single-carriage changes {changes} | kinematic (recolour or nothing) {kinematic} | constructors accreted {accreted} (on light {on_light}) extinguished {extinguished} (carved interior {carved}) recoloured {recoloured}"
    );
    println!(
        "incongruence subjects {subjects}: of one reading {solo}, of several {multi} | core non-empty {core_nonempty} | edges {edges} | contrast between consecutive readings: shared {c_shared} only first {c_first} only second {c_second}"
    );
    println!(
        "incongruence sharpness: resolution quantum 1 on {quantum_one}, finer than 1 on {quantum_more} | phase holonomic {holonomic} luminal {luminal} | moved: refined {refined} closed {closed} sharpened {sharpened} recognized {recognized}"
    );
    println!(
        "incongruence judged by the plane: readings {judged} reproduced by their subject's core {reproduced} | subjects reproduced on every reading {subjects_whole} of {multi} | tasks whose every judged reading reproduces {tasks_whole} of {tasks_judged}"
    );
}
