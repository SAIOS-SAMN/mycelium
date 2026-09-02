// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! arc-solve — THE THIN SCORER (and nothing else). This binary does NO
//! cognition: no decomposition, no consensus, no term derivation, no
//! task-local anything. It loads a language snapshot, calls exactly the
//! library organs every entity uses (`arc::project_anchored` — the same
//! function behind `Entity::project` — plus the exact frame measurement),
//! and counts. If the ENGINE's language cannot say a task, the scorer
//! reports it honestly; it never compensates. The solving itself happens
//! where it belongs: on clients, through the wire, settled as proof of
//! useful work — this instrument only reads the Γ vector off a frozen
//! substrate so runs compare by dominance.
//!
//! Buckets:
//!   SOLVED       — every test output reconstructed exactly by projection
//!   wrong        — projection spoke but missed
//!   silent       — no anchored word spoke on any test input (the language
//!                  has nothing court-refined to say here yet)
//!   frame-refuse — inconsistent family ratio or non-integral application
//!                  (refusal over rounding, always)
//!
//! Usage: arc-solve --state SNAPSHOT --dir CORPUS [--limit N]
use mycelium::arc::{embed, frame_ratio, parse_task, project_anchored, readback, Task};
use mycelium::cognition::Kernel;
use std::path::PathBuf;

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].as_str())
}
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a.max(1) } else { gcd(b, a % b) }
}
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

/// MEASURE the family's frame ratio (perception: reading dimensions).
fn task_ratio(task: &Task) -> Option<((usize, usize), (usize, usize))> {
    let mut ratio = None;
    for (a, b) in &task.train {
        let f =
            (a.len(), a.first().map_or(0, |r| r.len()), b.len(), b.first().map_or(0, |r| r.len()));
        if f.0 == 0 || f.1 == 0 || f.2 == 0 || f.3 == 0 {
            return None;
        }
        let r = frame_ratio(&f);
        match &ratio {
            None => ratio = Some(r),
            Some(held) if *held != r => return None,
            _ => {}
        }
    }
    ratio
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let state = flag(&args, "--state").unwrap_or("state/hypha-a.ledger").to_string();
    let dir = flag(&args, "--dir").unwrap_or("data/arc-agi-2/training").to_string();
    let limit: usize = flag(&args, "--limit").and_then(|s| s.parse().ok()).unwrap_or(usize::MAX);

    let mut kernel = Kernel::new();
    match std::fs::read(&state) {
        Ok(bytes) if kernel.restore(&bytes) => {
            eprintln!("arc-solve: language restored — {} words from {state}", kernel.basis_len())
        }
        _ => eprintln!("arc-solve: no language at {state}; scoring an empty tongue"),
    }
    let language = kernel.basis().to_vec();
    let lang_anchored = language.iter().filter(|w| w.anchor().is_some()).count();

    let mut tasks: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| {
            eprintln!("arc-solve: cannot read {dir}: {e}");
            std::process::exit(1);
        })
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    tasks.sort();
    tasks.truncate(limit);
    let total = tasks.len();

    let (mut solved, mut wrong, mut silent, mut frame_refuse) = (0u64, 0u64, 0u64, 0u64);
    let mut mu_solved: [u64; 4] = [0; 4];

    'task: for path in &tasks {
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        let Some(task) = parse_task(&text) else { continue };
        let Some(((hn, hd), (wn, wd))) = task_ratio(&task) else {
            frame_refuse += 1;
            continue 'task;
        };
        let mut any_spoke = false;
        let mut all_exact = !task.test.is_empty();
        for (x, y) in &task.test {
            if y.is_empty() {
                continue;
            }
            let (h, w) = (x.len(), x.first().map_or(0, |r| r.len()));
            if h == 0 || w == 0 || (h * hn) % hd != 0 || (w * wn) % wd != 0 {
                frame_refuse += 1;
                continue 'task; // non-integral frame: the language does not apply
            }
            let (oh, ow) = (h * hn / hd, w * wn / wd);
            let (lh, lw) = (lcm(h, oh), lcm(w, ow));
            let rx = embed(x, lh, lw);
            // THE ONLY THINKING CALL — the engine's own projection organ.
            let (guess, spoke) = project_anchored(&rx, &language);
            if !spoke.is_empty() {
                any_spoke = true;
                if readback(&guess, oh, ow).is_some_and(|g| g == *y) {
                    for &wi in &spoke {
                        let m = language[wi].magnitude();
                        for l in 0..4 {
                            mu_solved[l] = mu_solved[l].saturating_add(m[l]);
                        }
                    }
                    continue;
                }
            }
            all_exact = false;
        }
        if !any_spoke {
            silent += 1;
        } else if all_exact {
            solved += 1;
            eprintln!("  SOLVED {}", path.file_name().unwrap().to_string_lossy());
        } else {
            wrong += 1;
        }
    }

    // Γ_gate — the polytonic vector (dominance-compared between runs; never
    // a scalar). Every coordinate is a count of the ENGINE's own speech.
    println!("Γ_gate over {total} tasks, language {state}:");
    println!("  buckets   [solved {solved}, wrong {wrong}, silent {silent}, frame-refuse {frame_refuse}]");
    println!(
        "  language  [|basis| {}, |anchored| {lang_anchored}, ratio {lang_anchored}/{}]",
        language.len(),
        language.len(),
    );
    println!("  μ⃗_solved {mu_solved:?}");
}
