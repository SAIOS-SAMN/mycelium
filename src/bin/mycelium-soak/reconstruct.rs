// 78.82 — INSTRUMENTATION ONLY (§8.1): reconstruction by holonomy (HR1–HR4)
// over a corpus slice, against the one number that matters — families the
// form speaks whole. Per task: the form's speech (spoken whole or not), the
// reconstruction left one pair out at a time (`reconstruct::reconstruct`, the
// voice's own reader through `interpret`), the axes whose loops close on the
// whole family, and the body axis's single changes by their fascicle's
// regime. Data; nothing is decided.

use mycelium::client::sdk::Epigenome;
use mycelium::transducer::family::incongruence::single_changes;
use mycelium::transducer::family::reconstruct::{body_regimes, reconstruct, AXES};
use mycelium::transducer::family::{extract_rule_with, form_deflation};

const STROKE: usize = 1 << 20;

pub(crate) fn report(dir: &str, names: &[String]) {
    let empty = Epigenome::new();
    let (mut tasks, mut extracted, mut spoken_whole) = (0u64, 0u64, 0u64);
    let (mut rebuilt, mut rebuilt_unspoken, mut both) = (0u64, 0u64, 0u64);
    let (mut pairs, mut framed, mut reproduced) = (0u64, 0u64, 0u64);
    let (mut by_axis, mut closing) = ([0u64; 9], [0u64; 9]);
    let mut regimes = [0u64; 5];
    let (mut loops, mut closed, mut unclosed, mut unseen) = ([0u64; 9], [0u64; 9], 0u64, 0u64);
    for name in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{name}")) else { continue };
        let Some(task) = mycelium::arc::parse_task(&json) else { continue };
        tasks += 1;
        let mut physics = mycelium::physics::Physics::default();
        let mut budget = physics.stroke_n(STROKE);
        let speech = extract_rule_with(&task.train, &empty, &mut budget).map(|rule| form_deflation(&rule.form, &task.train, &empty).1);
        let whole_spoken = speech.as_ref().is_some_and(|rems| rems.iter().all(Vec::is_empty));
        extracted += u64::from(speech.is_some());
        spoken_whole += u64::from(whole_spoken);
        if let Some(rems) = &speech {
            let r = body_regimes(&single_changes(&task.train, rems));
            regimes.iter_mut().zip(r).for_each(|(a, b)| *a += b);
        }
        let rec = reconstruct(&task.train);
        pairs += rec.pairs;
        framed += rec.framed;
        reproduced += rec.reproduced;
        by_axis.iter_mut().zip(rec.by_axis).for_each(|(a, b)| *a += b);
        closing.iter_mut().zip(rec.closing).for_each(|(a, b)| *a += u64::from(b));
        for (a, h) in rec.holonomy.iter().enumerate() {
            loops[a] += h.loops;
            closed[a] += h.closed;
        }
        unclosed += rec.unclosed;
        unseen += rec.unseen;
        rebuilt += u64::from(rec.whole);
        rebuilt_unspoken += u64::from(rec.whole && !whole_spoken);
        both += u64::from(rec.whole && whole_spoken);
        let closes: Vec<&str> = AXES.iter().zip(rec.closing).filter(|(_, c)| *c).map(|(a, _)| *a).collect();
        println!(
            "  reconstruct {name}: spoken whole {whole_spoken} | pairs {} framed {} reproduced {} whole {} | closing axes [{}]",
            rec.pairs,
            rec.framed,
            rec.reproduced,
            rec.whole,
            closes.join(", ")
        );
    }
    println!(
        "reconstruct: tasks {tasks} extracted {extracted} spoken whole {spoken_whole} | reconstructed whole {rebuilt} (not spoken whole {rebuilt_unspoken}, both {both}) | pairs {pairs} framed {framed} reproduced (left out) {reproduced}"
    );
    let axes: Vec<String> = AXES.iter().zip(by_axis.iter().zip(&closing)).map(|(a, (n, c))| format!("{a} {n}/{c}")).collect();
    println!("reconstruct by axis (pairs carried / families the axis closes on): {}", axes.join(" | "));
    let held: Vec<String> = AXES.iter().zip(closed.iter().zip(&loops)).map(|(a, (c, l))| format!("{a} {c}/{l}")).collect();
    println!(
        "reconstruct loops closed / loops, over every family: {} | left-out pairs no axis closed for {unclosed}, an axis closed but a key unseen {unseen}",
        held.join(" | ")
    );
    println!(
        "reconstruct body axis, single changes by their fascicle's regime: finite turn {} rosette {} kiss (light) {} apart (dilation) {} null {}",
        regimes[0], regimes[1], regimes[2], regimes[3], regimes[4]
    );
}
