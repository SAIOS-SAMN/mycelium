// §8.1 — THE ONLY QUESTION THE CORPUS ASKS: how many tasks does this
// engine actually solve.
//
//! Extract the rule from a task's train pairs, speak it on each test
//! input, and compare the spoken grid with the corpus's own answer, site
//! for site. No partial credit and no similarity: a solve is the grid or
//! it is not (rule 11).
//!
//! The mind is stood on the slice first, as a voice's is, because what it
//! holds changes what the glue can cite and therefore what it can say.

use mycelium::client::sdk::Epigenome;
use mycelium::transducer::family::{extract_rule_on, extraction_spend, mint_sentence, perceive_family, speak_rule};

pub(crate) fn report(dir: &str, names: &[String], stand: usize) {
    let mut epi = Epigenome::new();
    let mut physics = mycelium::physics::Physics::default();
    // The mind stands on the first `stand` tasks, as a voice does.
    for (k, file) in names.iter().take(stand).enumerate() {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{file}")) else { continue };
        let Some(task) = mycelium::arc::parse_task(&json) else { continue };
        let frames = perceive_family(&task.train);
        let mut b = physics.stroke_n(extraction_spend(&task.train, &frames));
        let Some(r) = extract_rule_on(&task.train, &frames, &epi, &mut b) else { continue };
        let Some(w) = mint_sentence(&r, &task.train, &epi, 0) else { continue };
        let held = mycelium::transducer::element::Held::of(w);
        if held.form().is_some_and(|f| f.is_closed()) {
            let k = k as u64;
            let _ = epi.stand_dialect_at(held.element().clone(), k, k);
        }
    }

    let (mut tasks, mut extracted, mut spoke, mut solved, mut partly) = (0u64, 0u64, 0u64, 0u64, 0u64);
    for file in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{file}")) else { continue };
        let Some(task) = mycelium::arc::parse_task(&json) else { continue };
        tasks += 1;
        let frames = perceive_family(&task.train);
        let mut b = physics.stroke_n(extraction_spend(&task.train, &frames));
        let Some(rule) = extract_rule_on(&task.train, &frames, &epi, &mut b) else { continue };
        extracted += 1;
        // EVERY TEST PAIR, and the task counts as solved only if every one
        // of them is spoken exactly.
        let mut said = 0u64;
        let mut right = 0u64;
        for (x, y) in &task.test {
            let Some(sp) = speak_rule(&rule, x, &epi) else { continue };
            let Some(g) = sp.grid() else { continue };
            said += 1;
            if &g == y {
                right += 1;
            }
        }
        if said > 0 {
            spoke += 1;
        }
        if right > 0 && right == task.test.len() as u64 {
            solved += 1;
            println!("  solve {file} SOLVED {right} of {}", task.test.len());
        } else if right > 0 {
            partly += 1;
            println!("  solve {file} partial {right} of {}", task.test.len());
        }
    }
    println!(
        "  solve tasks {tasks} extracted {extracted} spoke {spoke} solved {solved} partial {partly}"
    );
}
