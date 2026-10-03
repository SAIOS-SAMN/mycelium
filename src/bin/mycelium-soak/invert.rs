// 78.76 — INSTRUMENTATION ONLY (§8.1): the remainder inversion measured over a
// corpus slice. For every task whose rule extracts, the form is spoken over
// its training pairs, its remainder inverted (`family::invert`), and what the
// speech then leaves is read — before and after, per task and in total.
// Nothing here decides anything; the engine's own functions are called as the
// voice calls them, with an empty crystal.

use mycelium::client::sdk::Epigenome;
use mycelium::transducer::family::{extract_rule_with, invert::invert_remainder_as, invert::Painted};

const STROKE: usize = 1 << 20;

pub(crate) fn report(dir: &str, names: &[String], painted: Painted) {
    println!("invert: the remainder painted as {painted:?}");
    let empty = Epigenome::new();
    let (mut tasks, mut extracted, mut whole, mut with_rem) = (0u64, 0u64, 0u64, 0u64);
    let (mut unextracted, mut joined_any, mut accepted, mut closed) = (0u64, 0u64, 0u64, 0u64);
    let (mut links_before, mut links_after, mut accepted_before, mut accepted_after) = (0u64, 0u64, 0u64, 0u64);
    let (mut conflicts, mut erasures, mut generators, mut demands) = (0u64, 0u64, 0u64, 0u64);
    let mut kinds = [0u64; 6];
    for name in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{name}")) else { continue };
        let Some(task) = mycelium::arc::parse_task(&json) else { continue };
        tasks += 1;
        let mut physics = mycelium::physics::Physics::default();
        let mut budget = physics.stroke_n(STROKE);
        let Some(rule) = extract_rule_with(&task.train, &empty, &mut budget) else {
            continue;
        };
        extracted += 1;
        let mut budget = physics.stroke_n(STROKE);
        let Some((_, read)) = invert_remainder_as(&rule.form, &task.train, &empty, &mut budget, painted) else {
            whole += 1;
            println!("  invert {name}: the form speaks its family whole");
            continue;
        };
        with_rem += 1;
        links_before += read.before;
        links_after += read.after;
        unextracted += u64::from(read.unextracted);
        joined_any += u64::from(read.joined > 0);
        conflicts += read.conflicts;
        erasures += read.erasures;
        generators += read.extracted;
        demands += read.demands;
        for (k, n) in kinds.iter_mut().zip(read.kinds) {
            *k += n;
        }
        if read.accepted {
            accepted += 1;
            accepted_before += read.before;
            accepted_after += read.after;
        }
        closed += u64::from(read.closed);
        println!(
            "  invert {name}: remainder {} → {} | generators extracted {} (demands {}) joined {} conflicts {} | erasures {} | {}",
            read.before,
            read.after,
            read.extracted,
            read.demands,
            read.joined,
            read.conflicts,
            read.erasures,
            match (read.unextracted, read.closed, read.accepted) {
                (true, _, _) => "the remainder scene extracts nothing",
                (_, true, _) => "ACCEPTED, CLOSED",
                (_, _, true) => "accepted",
                _ => "refused by the speech",
            }
        );
    }
    println!(
        "invert: tasks {tasks} extracted {extracted} | spoken whole {whole} | with remainder {with_rem}: \
         unextracted {unextracted}, generators joined on {joined_any} (extracted {generators}, demands {demands}, conflicts {conflicts}), \
         accepted {accepted}, closed {closed} | links {links_before} → {links_after} over all, \
         {accepted_before} → {accepted_after} where accepted | erasures {erasures}"
    );
    println!(
        "invert demands by kind: rotor prime {} standing {} source {} creation {} deformation {} dislocation {}",
        kinds[0], kinds[1], kinds[2], kinds[3], kinds[4], kinds[5]
    );
}
