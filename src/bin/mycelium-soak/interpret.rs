// 78.81's caution before a recut — INSTRUMENTATION ONLY (§8.1): what the
// voice's remainder interpreter (`family::interpret`, called by `speak_form`
// on every family it speaks with a remainder) costs, in the engine's own
// unit — the stroke's spend — against what the family's extraction spends.
// Per task: the extraction's spend, then the interpreter's on the remainder
// the speech left. Data; nothing is decided.

use mycelium::client::sdk::Epigenome;
use mycelium::transducer::family::interpret::interpret;
use mycelium::transducer::family::{extract_rule_with, form_deflation};

const STROKE: usize = 1 << 20;

pub(crate) fn report(dir: &str, names: &[String]) {
    let empty = Epigenome::new();
    let (mut families, mut with_remainder, mut extract_spend, mut interpret_spend, mut over) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut widest = 0u64;
    for name in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{name}")) else { continue };
        let Some(task) = mycelium::arc::parse_task(&json) else { continue };
        let mut physics = mycelium::physics::Physics::default();
        let mut budget = physics.stroke_n(STROKE);
        let before = budget.remaining();
        let Some(rule) = extract_rule_with(&task.train, &empty, &mut budget) else { continue };
        let extracted = (before - budget.remaining()) as u64;
        families += 1;
        extract_spend += extracted;
        let (_, rems) = form_deflation(&rule.form, &task.train, &empty);
        if rems.iter().all(Vec::is_empty) {
            continue;
        }
        with_remainder += 1;
        let mut budget = physics.stroke_n(STROKE);
        let before = budget.remaining();
        let _ = interpret(&rule.form, &task.train, &rems, &empty, &mut budget);
        let spent = (before - budget.remaining()) as u64;
        interpret_spend += spent;
        widest = widest.max(spent);
        over += u64::from(spent > extracted);
        println!("  interpret {name}: extraction spent {extracted} | interpreter spent {spent}");
    }
    println!(
        "interpret cost: families extracted {families}, with a remainder {with_remainder} | extraction spent {extract_spend} | interpreter spent {interpret_spend} (widest one family {widest}) | families where the interpreter spent more than their extraction {over}"
    );
}
