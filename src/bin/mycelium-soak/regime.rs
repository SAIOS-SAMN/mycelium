// D1, §8.1 — THE FLEET REGIME ON ONE FAMILY: what a voice's own stroke
// buys on a named task, after the mind has stood on a slice.
//
//! EVERY LANDING'S READING (standing terms, 2026-09-19). A reading taken
//! with an empty mind and a stroke of `1 << 20` is not the reading a voice
//! gets: the voice holds what it stood on earlier families and its budget
//! is `extraction_spend(&train)` — the family's own area, not the box's.
//! Both matter, and in opposite directions: the mind makes the glue do
//! more work, the budget gives it less to do it with.
//!
//! So this stands a mind on the slice exactly as `walk` does — the same
//! calls in the same order — and then pulls ONE named family under the
//! voice's budget, timing the pull and the mint apart. Two numbers, taken
//! at the boundary the directives name.
//!
//! THE CLOCK IS THIS BINARY'S, NEVER A KERNEL'S. Law 9 and rule 5 forbid a
//! kernel reading a wall clock; an instrument that reports seconds is not
//! a kernel, nothing in the engine consults these numbers, and the process
//! exits on them.
//!
//! TRACE LINES AND NOTHING ELSE. No library function is changed and no
//! reading is taken that the public API does not already expose. If this
//! file were deleted the engine would behave identically.

// THE SAME SCOPED EXEMPTION AS `family/trace.rs`, for the same reason and
// with the same bound. `clippy.toml` disallows `std::time::Instant` under
// law 9 — "the clock is the receipt chain's k and the height, never wall
// time" — and that ban is right for the engine. This is an INSTRUMENT
// BINARY: it is not a kernel, no engine branch reads these numbers, and the
// process prints them and exits. The standing terms of 2026-09-19 require
// the pull and the mint to be timed APART, and two numbers taken at that
// boundary cannot be had from a carrier with no seconds in it.
#![allow(
    clippy::disallowed_types,
    reason = "D1 2026-09-19: the standing reading's pull and mint times; law 9 binds kernels and this is an instrument"
)]

use mycelium::client::sdk::Epigenome;
use mycelium::transducer::family::{extract_rule_with, extraction_spend, mint_sentence};
use std::time::Instant;

/// A corpus file's name without its suffix — the family's own id, which is
/// what the voice writes and what the timeline groups on.
fn stem_of(file: &str) -> &str {
    match file.split_once('.') {
        Some((stem, _)) => stem,
        None => file,
    }
}

/// Stand the mind on `slice`, then pull `name` under the voice's budget.
pub(crate) fn report(dir: &str, slice: &[String], name: &str) {
    let mut epi = Epigenome::new();
    let mut physics = mycelium::physics::Physics::default();
    let mut stood = 0u64;
    let stand_start = Instant::now();
    for (k, file) in slice.iter().enumerate() {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{file}")) else { continue };
        let Some(task) = mycelium::arc::parse_task(&json) else { continue };
        // THE MARKER A TRACE LINE IS GROUPED UNDER, written in the shape a
        // voice writes it so `scripts/timeline` reads one renderer for
        // both streams. It goes to stderr, where the traces are: a marker
        // in the other stream cannot be interleaved with what it marks.
        eprintln!("  regime [h {k}] family {}… {} pairs", stem_of(file), task.train.len());
        // THE VOICE'S OWN BUDGET on the standing slice too: a mind stood
        // under a stroke it never gets holds forms it never would.
        let mut budget = physics.stroke_n(extraction_spend(&task.train));
        let Some(rule) = extract_rule_with(&task.train, &epi, &mut budget) else { continue };
        let Some(spoken) = mint_sentence(&rule, &task.train, &epi, 0) else { continue };
        let held = mycelium::transducer::element::Held::of(spoken);
        let Some(form) = held.form() else { continue };
        if !form.is_closed() {
            continue;
        }
        let k = k as u64;
        if epi.stand_dialect_at(held.element().clone(), k, k).is_some() {
            stood += 1;
        }
    }
    let stand_s = stand_start.elapsed();
    println!(
        "  regime stood {stood} of {} in {}.{:03}s",
        slice.len(),
        stand_s.as_secs(),
        stand_s.subsec_millis()
    );

    let path = format!("{dir}/{name}");
    let Ok(json) = std::fs::read_to_string(&path) else {
        eprintln!("  regime refused: {path} does not read");
        std::process::exit(1);
    };
    let Some(task) = mycelium::arc::parse_task(&json) else {
        eprintln!("  regime refused: {path} is not a task");
        std::process::exit(1);
    };
    let spend = extraction_spend(&task.train);
    println!("  regime pull {name} pairs {} budget {spend}", task.train.len());
    eprintln!("  regime [h pull] family {}… {} pairs", stem_of(name), task.train.len());

    // THE PULL: the extraction a voice's task arm runs, under the voice's
    // budget and against the mind it just stood.
    let mut budget = physics.stroke_n(spend);
    let pull_start = Instant::now();
    let rule = extract_rule_with(&task.train, &epi, &mut budget);
    let pull_s = pull_start.elapsed();
    let left = budget.remaining();
    match &rule {
        Some(r) => println!(
            "  regime pull {}.{:03}s rule {} vertices {} faces {} loops sections {} | budget left {left} of {spend}",
            pull_s.as_secs(),
            pull_s.subsec_millis(),
            r.form.vertices.len(),
            r.form.faces.len(),
            r.form.loops.len(),
            r.section_count()
        ),
        None => println!(
            "  regime pull {}.{:03}s withheld | budget left {left} of {spend}",
            pull_s.as_secs(),
            pull_s.subsec_millis()
        ),
    }

    // THE MINT, timed apart: a pull that returns and a mint that does not
    // are two different faults and one number cannot tell them apart.
    let Some(rule) = rule else {
        println!("  regime mint - (no rule to mint)");
        return;
    };
    let mint_start = Instant::now();
    let spoken = mint_sentence(&rule, &task.train, &epi, 0);
    let mint_s = mint_start.elapsed();
    match spoken {
        Some(w) => {
            let held = mycelium::transducer::element::Held::of(w);
            let closed = held.form().is_some_and(|f| f.is_closed());
            println!(
                "  regime mint {}.{:03}s minted closed {closed}",
                mint_s.as_secs(),
                mint_s.subsec_millis()
            );
        }
        None => println!("  regime mint {}.{:03}s refused", mint_s.as_secs(), mint_s.subsec_millis()),
    }
}
