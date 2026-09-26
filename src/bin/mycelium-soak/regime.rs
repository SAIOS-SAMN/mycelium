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
use mycelium::transducer::family::{extract_rule_on, extraction_spend, mint_sentence, perceive_family};
use std::time::Instant;

/// Live allocator bytes, for the ten-pull reading. Instrument only.
fn live_now() -> i128 {
    match tikv_jemalloc_ctl::epoch::advance() {
        Ok(_) => match tikv_jemalloc_ctl::stats::allocated::read() {
            Ok(n) => n as i128,
            Err(_) => 0,
        },
        Err(_) => 0,
    }
}

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
        let frames = perceive_family(&task.train);
        let mut budget = physics.stroke_n(extraction_spend(&task.train, &frames));
        let Some(rule) = extract_rule_on(&task.train, &frames, &epi, &mut budget) else { continue };
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
    // Directive 3: the bound is PROBED, not computed from boot invariants.
    let probe_reserve = physics.reserve;
    let probe_stroke = physics.stroke().remaining();
    println!(
        "  regime physics reserve {probe_reserve:?} | stroke {probe_stroke} | reserve_max {} spend_unit {}",
        mycelium::physics::reserve_max(),
        mycelium::physics::spend_unit()
    );
    println!(
        "  regime stood {stood} of {} in {}.{:03}s",
        slice.len(),
        stand_s.as_secs(),
        stand_s.subsec_millis()
    );

    // Z4b's reading: hold the whole slice's forms in ONE contemplation and
    // census the shed's filter, so the term that rejects is measured and
    // not named from one family.
    {
        use mycelium::transducer::bubble::contemplation::Contemplation;
        let mut hold = Contemplation::new();
        let mut held = 0u64;
        for file in slice {
            let Ok(json) = std::fs::read_to_string(format!("{dir}/{file}")) else { continue };
            let Some(task) = mycelium::arc::parse_task(&json) else { continue };
            let frames = perceive_family(&task.train);
            let mut b = physics.stroke_n(extraction_spend(&task.train, &frames));
            let Some(r) = extract_rule_on(&task.train, &frames, &epi, &mut b) else { continue };
            hold.hold_form(r.form, &task.train, &[], [0; 4], &mut b);
            held += 1;
        }
        // EXPAND BEFORE CENSUSING. `hold_form` writes `dirty: true` at
        // construction and `expand_with` is the only writer after it, so a
        // census taken without expanding reads the constructor, not the
        // filter.
        let before = hold.shed_census();
        let atlas = epi.working_atlas();
        let section_at = |a: &[u8]| {
            epi.working_word(a).and_then(|h| mycelium::transducer::cech::section_of(h.element()))
        };
        // COLUMN 2 — THE VOICE'S OWN STROKE, not a million. `expand_ideas`
        // passes `physics.stroke()`, so that is what the fleet's bubbles
        // are expanded under; a reading at a million says nothing about
        // whether a voice finishes.
        let mut voiced = physics.stroke();
        let voice_stroke = voiced.remaining();
        let _ = hold.expand_all(&atlas, &section_at, &epi, &mut voiced);
        let at_voice = hold.shed_census();
        println!(
            "    at voice stroke ({voice_stroke})  glued_by {} glues {} dirty {} sheddable {} | left {}",
            at_voice[0], at_voice[1], at_voice[2], at_voice[3], voiced.remaining()
        );
        // And again, repeatedly, as a voice does one expand per stroke.
        for _ in 0..20 {
            let mut v = physics.stroke();
            let _ = hold.expand_all(&atlas, &section_at, &epi, &mut v);
        }
        let after20 = hold.shed_census();
        println!(
            "    after 21 voice strokes        glued_by {} glues {} dirty {} sheddable {}",
            after20[0], after20[1], after20[2], after20[3]
        );
        let mut b = physics.stroke_n(1 << 20);
        let _ = hold.expand_all(&atlas, &section_at, &epi, &mut b);
        let c = hold.shed_census();
        println!(
            "  regime slice census extracted {held} held {}",
            c[0] + c[1] + c[2] + c[3]
        );
        println!(
            "    before expand  glued_by {} glues {} dirty {} sheddable {}",
            before[0], before[1], before[2], before[3]
        );
        println!(
            "    after expand   glued_by {} glues {} dirty {} sheddable {}",
            c[0], c[1], c[2], c[3]
        );
        println!("    budget left {} of {}", b.remaining(), 1u32 << 20);
        // COLUMN 1 — DEAD PINS. A citing bubble is held by `glues`; is what
        // it cites still standing? Live means the mind holds the word, or a
        // bubble stands at that address. Neither is a count compared to a
        // number: each pin is asked at its own coordinate.
        let (mut pinned, mut live_pins, mut dead_pins, mut all_dead) = (0u64, 0u64, 0u64, 0u64);
        for (_, bub) in hold.bubbles.scoped() {
            if bub.glues.is_empty() {
                continue;
            }
            pinned += 1;
            let (mut l, mut d) = (0u64, 0u64);
            for (addr, ()) in bub.glues.scoped() {
                let held = epi.working_word(&addr).is_some() || hold.bubbles.get(&addr).is_some();
                if held {
                    l += 1;
                } else {
                    d += 1;
                }
            }
            live_pins += l;
            dead_pins += d;
            if l == 0 {
                all_dead += 1;
            }
        }
        println!(
            "    pins: citing bubbles {pinned} | pins live {live_pins} dead {dead_pins} | \
             bubbles whose every pin is dead {all_dead}"
        );
        println!("  regime slice shed_unglued retired {}", hold.shed_unglued());
    }

    let path = format!("{dir}/{name}");
    let Ok(json) = std::fs::read_to_string(&path) else {
        eprintln!("  regime refused: {path} does not read");
        std::process::exit(1);
    };
    let Some(task) = mycelium::arc::parse_task(&json) else {
        eprintln!("  regime refused: {path} is not a task");
        std::process::exit(1);
    };
    let frames = perceive_family(&task.train);
    let spend = extraction_spend(&task.train, &frames);
    println!("  regime pull {name} pairs {} budget {spend}", task.train.len());
    eprintln!("  regime [h pull] family {}… {} pairs", stem_of(name), task.train.len());

    // THE PULL: the extraction a voice's task arm runs, under the voice's
    // budget and against the mind it just stood.
    let mut budget = physics.stroke_n(spend);
    let pull_start = Instant::now();
    let rule = extract_rule_on(&task.train, &frames, &epi, &mut budget);
    let pull_s = pull_start.elapsed();
    let left = budget.remaining();
    match &rule {
        Some(r) => println!(
            "  regime pull {}.{:03}s rule {} vertices {} faces {} orbits sections {} | budget left {left} of {spend}",
            pull_s.as_secs(),
            pull_s.subsec_millis(),
            r.form.vertices.len(),
            r.form.faces.len(),
            r.form.orbits.len(),
            r.section_count()
        ),
        None => println!(
            "  regime pull {}.{:03}s withheld | budget left {left} of {spend}",
            pull_s.as_secs(),
            pull_s.subsec_millis()
        ),
    }

    // Z4's DoD: TEN PULLS, the held bytes and the resident read at each.
    // A leak is a number that does not come back down across repeats of
    // one pull, and one pull cannot show that whatever it reads.
    {
        use mycelium::transducer::bubble::contemplation::Contemplation;
        // WHERE THE FIRST HOLD'S BYTES GO, component by component, so the
        // fix is aimed at a measurement and not at a guess.
        {
            let mut b = physics.stroke_n(spend);
            if let Some(r) = extract_rule_on(&task.train, &frames, &epi, &mut b) {
                let a0 = live_now();
                let mut cloned = r.form.clone();
                let a1 = live_now();
                cloned.reindex();
                let a2 = live_now();
                let train_copy = task.train.to_vec();
                let a3 = live_now();
                println!(
                    "  regime hold parts: form.clone {} B  reindex {} B  train.to_vec {} B",
                    a1 - a0,
                    a2 - a1,
                    a3 - a2
                );
                drop(cloned);
                drop(train_copy);
            }
        }
        let mut hold = Contemplation::new();
        for pull in 0..10u32 {
            Contemplation::reset_held_bytes();
            let mut b = physics.stroke_n(spend);
            let Some(r) = extract_rule_on(&task.train, &frames, &epi, &mut b) else { continue };
            hold.hold_form(r.form.clone(), &task.train, &[], [0; 4], &mut b);
            if pull == 9 {
                let c = hold.shed_census();
                println!(
                    "  regime shed census held {} glued_by {} glues {} dirty {} sheddable {}",
                    c[0] + c[1] + c[2] + c[3], c[0], c[1], c[2], c[3]
                );
                println!("  regime shed_unglued retired {}", hold.shed_unglued());
            }
            println!(
                "  regime pull {pull} held {:+} B  live {} B",
                Contemplation::held_bytes(),
                live_now()
            );
        }
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
