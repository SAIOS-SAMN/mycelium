// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-STANDARDS 8.1 — INSTRUMENTATION: one voice walks a corpus in
//! process and writes what it read. Trace lines only; it changes no
//! behaviour and it is not the fleet.
//!
//! What it measures is the chain L8r opened: a mind extracts a rule over
//! a family with its own epigenome as the crystal (21 step 3), stands
//! what closes, and the next family may cite what the last one stood.
//! The reading that matters is `glued` — rules citing a form this mind
//! holds — which was structurally zero while the glue's read was
//! performed against the form index (26, 27, 50.6).

use mycelium::client::sdk::Epigenome;
use mycelium::transducer::family::{extract_rule_with, mint_sentence};
use mycelium::transducer::sentence::{from_element_form, Vertex};

/// What one walk of the corpus read. Every field is a count of acts, read
/// and never branched on (rule 2).
#[derive(Default)]
struct Reading {
    tasks: u64,
    extracted: u64,
    stood: u64,
    glued: u64,
    demands: u64,
    words: u64,
    /// The demand kinds by 50.5's own order — `RotorPrime`, `Standing`,
    /// `Source`, `Creation`, `TopologicalDeformation` — which name what
    /// the alphabet could not spell and the eyes could not pair, and so
    /// which of L10 and L11 a corpus is actually asking for.
    kinds: [u64; 5],
}

fn main() {
    let mut args = std::env::args().skip(1);
    // The corpus path and how much of it to walk are the operator's, and
    // an absent argument is an absent argument, never a value this binary
    // invented (rule 11): each arm is written out.
    let dir = match args.next() {
        Some(d) => d,
        None => {
            eprintln!("  soak refused: name the corpus directory");
            std::process::exit(1);
        }
    };
    let limit: usize = match args.next().map(|n| n.parse::<usize>()) {
        Some(Ok(n)) => n,
        Some(Err(e)) => {
            eprintln!("  soak refused: the task count does not read: {e}");
            std::process::exit(1);
        }
        None => usize::MAX,
    };
    match walk(&dir, limit) {
        Ok(r) => {
            println!(
                "  soak tasks {} extracted {} stood {} glued {} demands {} words {}",
                r.tasks, r.extracted, r.stood, r.glued, r.demands, r.words
            );
            println!(
                "  soak demands rotor_prime {} standing {} source {} creation {} deformation {}",
                r.kinds[0], r.kinds[1], r.kinds[2], r.kinds[3], r.kinds[4]
            );
        }
        Err(e) => {
            eprintln!("  soak refused: {e}");
            std::process::exit(1);
        }
    }
}

/// The corpus is the boundary this loop walks (rule 10): one file, one
/// task, in name order so two runs read the same corpus in the same
/// order.
fn walk(dir: &str, limit: usize) -> Result<Reading, String> {
    let mut names: Vec<String> = Vec::new();
    let entries = std::fs::read_dir(dir).map_err(|e| format!("{dir}: {e}"))?;
    for e in entries {
        let e = e.map_err(|e| format!("{dir}: {e}"))?;
        if let Some(n) = e.file_name().to_str() {
            names.push(n.to_string());
        }
    }
    names.sort();
    names.truncate(limit);

    let mut epi = Epigenome::new();
    let mut r = Reading::default();
    let mut k = 0u64;
    for name in &names {
        let path = format!("{dir}/{name}");
        let Ok(json) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Some(task) = mycelium::arc::parse_task(&json) else {
            continue;
        };
        r.tasks += 1;
        k += 1;
        // One line per task before the extraction, so a family that does
        // not return is named by the last line written rather than by a
        // guess (rule 10: a loop whose bound is neither a Budget nor a
        // boundary is unbounded work, and this is how it is found).
        eprintln!("  soak task {name} pairs {}", task.train.len());
        // 21 with the mind as its own crystal: what it stood on an earlier
        // family is what it may cite on this one.
        let Some(rule) = extract_rule_with(&task.train, &epi) else {
            continue;
        };
        r.extracted += 1;
        for v in &rule.form.vertices {
            match v {
                Vertex::Glued { .. } => r.glued += 1,
                Vertex::Demand { kind, .. } => {
                    r.demands += 1;
                    if let Some(slot) = r.kinds.get_mut(usize::from(kind.kind_byte())) {
                        *slot += 1;
                    }
                }
                Vertex::Word { .. } => r.words += 1,
                _ => {}
            }
        }
        // Stand what closed, so the next family can cite it (24.3).
        let Some(spoken) = mint_sentence(&rule, &task.train, &epi) else {
            continue;
        };
        // Only a closed form is citable (13.6, `Form::glue`), so only a
        // closed form is stood here for the next family to reach.
        let Some(form) = from_element_form(&spoken) else {
            continue;
        };
        let Some(()) = form.is_closed().then_some(()) else {
            continue;
        };
        if epi.stand_dialect_at(spoken, k, k).is_some() {
            r.stood += 1;
        }
    }
    Ok(r)
}
