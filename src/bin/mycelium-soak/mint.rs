// Y3, §8.1 — INSTRUMENTATION ONLY: what `mint_sentence` is handed and
// what it hands back, for every task of a slice whose rule form closes.
//
//! TRACE LINES AND NOTHING ELSE. No library function is changed, none is
//! made more visible, and nothing here decides anything: every reading is
//! taken from the public API the engine already exposes, by calling the
//! same functions `standable` calls in the same order with the same
//! arguments. If this file were deleted the engine would behave
//! identically, which is the whole of what §8.1 permits and the whole of
//! what this row's first commit is allowed to be.
//!
//! Y1 handed over the instance: over sixty tasks, 54 extract a rule, 5 of
//! those close, and all 5 are refused at the mint. `no_form 0` and
//! `sentence_open 0` place the break at `mint_sentence` and nowhere else.
//! `mint_sentence` is two calls —
//!
//!     let mag = sentence_measure(&rule.form, family, crystal)?;
//!     as_element_form(&rule.form, mag)
//!
//! — so the refusal is one of those two, and this prints which, beside
//! the readings each of them was working from.
//!
//! NO CAUSE IS WRITTEN HERE. The trace is the deliverable.

use mycelium::client::sdk::Epigenome;
use mycelium::transducer::element::measure_lanes;
use mycelium::transducer::family::{extract_rule_with, form_descent, sentence_measure};
use mycelium::transducer::sentence::{as_element_form, Vertex};

/// The stroke Y1 set for itself, so the same slice reads the same way
/// under both instruments and a task that closes for one closes for the
/// other.
const MINT_STROKE: usize = 1 << 20;

/// Walk the slice and trace the mint for every task whose rule form
/// closes — the five, and any others a larger slice turns up.
pub(crate) fn trace(dir: &str, names: &[String]) {
    let empty = Epigenome::new();
    let mut closed_seen = 0u64;
    for name in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{name}")) else {
            continue;
        };
        let Some(task) = mycelium::arc::parse_task(&json) else {
            continue;
        };
        let mut physics = mycelium::physics::Physics::default();
        let mut budget = physics.stroke_n(MINT_STROKE);
        let Some(rule) = extract_rule_with(&task.train, &empty, &mut budget) else {
            continue;
        };
        // Only the tasks Y1 named: a rule whose form closes and whose
        // sentence does not mint. A rule that does not close never reaches
        // `mint_sentence` at all and is `rule_open`, not this row's.
        if !rule.form.is_closed() {
            continue;
        }
        closed_seen += 1;

        // WHAT THE MINT IS HANDED. The form, by what its vertices are, and
        // the family, by what its pairs are.
        let (mut words, mut glued, mut demands) = (0u64, 0u64, 0u64);
        for v in &rule.form.vertices {
            match v {
                Vertex::Word { .. } => words += 1,
                Vertex::Glued { .. } => glued += 1,
                Vertex::Demand { .. } => demands += 1,
                _ => {}
            }
        }
        println!(
            "  mint {name} handed: pairs {} | vertices words {words} glued {glued} demands {demands} \
             | faces {} loops {} | closed {} | ratio {:?}",
            task.train.len(),
            rule.form.faces.len(),
            rule.form.loops.len(),
            u8::from(rule.form.is_closed()),
            rule.ratio,
        );
        for (i, (x, y)) in task.train.iter().enumerate() {
            // The pair as the descent sees it: the delta where the pair
            // refines, and `refine_delta` answering None where it does
            // not — reported as the engine's own reading and not
            // re-derived here.
            let refined = mycelium::arc::refine_delta(x, y);
            // A grid's width is its rows' common length, and the slice
            // pattern reads the row the shape is stated by without taking
            // a member by position (rule 15).
            let wide = |g: &mycelium::arc::Grid| match g.split_first() {
                Some((row, _)) => row.len(),
                None => 0,
            };
            println!(
                "  mint {name} pair {i}: in {}x{} out {}x{} | refine_delta {} | delta mu {:?}",
                x.len(),
                wide(x),
                y.len(),
                wide(y),
                match refined {
                    Some(_) => "some",
                    None => "none",
                },
                refined.as_ref().map(|r| measure_lanes(&r.delta)),
            );
        }

        // WHAT `form_descent` RETURNS — the pay it accumulated and the
        // remainder it left on every pair, which is what `sentence_measure`
        // tests when the form carries no demand vertex.
        let (pay, remainders) = form_descent(&rule.form, &task.train, &empty);
        println!("  mint {name} form_descent: pay {pay:?} remainders {}", remainders.len());
        for (i, r) in remainders.iter().enumerate() {
            println!(
                "  mint {name} remainder {i}: cells {} empty {} mu {:?}",
                r.len(),
                u8::from(r.is_empty()),
                measure_lanes(r),
            );
        }

        // WHICH OF THE MINT'S TWO CALLS REFUSED. `sentence_measure`
        // returns `None` on exactly one path — a form with NO demand
        // vertex that left a remainder — and `as_element_form` is the
        // other half.
        let has_demands = rule.form.demand_vertices().next().is_some();
        let measured = sentence_measure(&rule.form, &task.train, &empty);
        println!(
            "  mint {name} sentence_measure: {} | has_demands {} | all_remainders_empty {}",
            match measured {
                Some(m) => format!("some {m:?}"),
                None => "NONE".to_string(),
            },
            u8::from(has_demands),
            u8::from(remainders.iter().all(Vec::is_empty)),
        );
        match measured {
            None => println!("  mint {name} refused at: sentence_measure"),
            Some(mag) => match as_element_form(&rule.form, mag) {
                Some(e) => println!(
                    "  mint {name} as_element_form: some, {} bytes — and the mint did NOT refuse here",
                    e.to_bytes().len()
                ),
                None => println!("  mint {name} refused at: as_element_form, on mag {mag:?}"),
            },
        }
    }
    println!("  mint tasks with a closed rule form: {closed_seen}");
}
