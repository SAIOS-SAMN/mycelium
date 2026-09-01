// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! arc-solve — THE ALPHA GATE (Step 6): solve-and-verify over ARC-AGI.
//!
//! No solver lives here. The engine's learned language (a hypha's germinated
//! basis, restored from its live ledger) is the ONLY transform vocabulary:
//! each train pair's coboundary decomposes into a normal-form sentence of
//! germinated words (projective reduction — the same `decompose_over` the
//! fleet speaks with), and the SOLUTION is the inverse deduction: each cited
//! word is a centered rewrite chain (before→after up to translation), applied
//! to the test input wherever its before-side holds. Doctrine: description is
//! language; solution is its inverse.
//!
//! VERIFY BEFORE PREDICT: a task is attempted only when its sentence REPLAYS —
//! applying the spoken words to every train input must reproduce every train
//! output exactly. A sentence that cannot re-derive its own evidence predicts
//! nothing (the same determinism the court enforces). Honest buckets:
//!   dim-skip    — output dims ≠ input dims (a chain rewrites, never resizes)
//!   mute        — some train pair does not decompose over today's language
//!   unverified  — the sentence decomposed but did not replay the train pairs
//!   wrong       — replayed on train, applied to test, mismatch
//!   SOLVED      — exact reconstruction of every test output
//!
//! Usage: arc-solve --state state/hypha-a.ledger --dir data/arc-agi-2/training [--limit N]
use mycelium::arc::{parse_task, Grid};
use mycelium::cognition::{decompose_over, Kernel};
use mycelium::transducer::element::{assignment, Element};
use std::path::PathBuf;

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].as_str())
}

/// The coboundary of a train pair — per-cell (r, c, before, after) where the
/// grids differ. None when the dims differ (a chain cannot resize a grid).
fn delta(a: &Grid, b: &Grid) -> Option<Vec<(i64, i64, u8, u8)>> {
    if a.len() != b.len() || a.iter().zip(b).any(|(ra, rb)| ra.len() != rb.len()) {
        return None;
    }
    let mut d = Vec::new();
    for (r, (ra, rb)) in a.iter().zip(b).enumerate() {
        for (c, (&va, &vb)) in ra.iter().zip(rb).enumerate() {
            if va != vb {
                d.push((r as i64, c as i64, va, vb));
            }
        }
    }
    Some(d)
}

/// The substrate framing projection: min-corner → origin, sorted (the same
/// canonical form every hypha derives, so the words peel identically).
fn center(chain: &[(i64, i64, u8, u8)]) -> Vec<(i64, i64, u8, u8)> {
    let min_r = chain.iter().map(|(r, _, _, _)| *r).min().unwrap_or(0);
    let min_c = chain.iter().map(|(_, c, _, _)| *c).min().unwrap_or(0);
    let mut out: Vec<_> =
        chain.iter().map(|(r, c, b, a)| (r - min_r, c - min_c, *b, *a)).collect();
    out.sort_unstable();
    out
}

/// Every concrete rewrite chain a word can speak: the base chain plus each
/// section/orbit assignment materialized over the base geometry (positions
/// held, hue-transition classes substituted class-for-class).
fn variants(w: &Element) -> Vec<Vec<(i64, i64, u8, u8)>> {
    let base = w.sub().to_vec();
    if base.is_empty() {
        return Vec::new();
    }
    let base_pairs = assignment(&base);
    let mut out = vec![base.clone()];
    for asg in w.assignments() {
        if asg.len() != base_pairs.len() || asg == base_pairs {
            continue;
        }
        let variant: Vec<_> = base
            .iter()
            .map(|(r, c, b, a)| {
                let i = base_pairs.iter().position(|p| *p == (*b, *a)).unwrap_or(0);
                (*r, *c, asg[i].0, asg[i].1)
            })
            .collect();
        out.push(variant);
    }
    out
}

/// THE INVERSE DEDUCTION — apply a sentence to a grid: each cited word (each
/// family variant) rewrites at EVERY translation where its whole before-side
/// holds on the pre-state. Placements are found against the unmodified grid,
/// then written together (one coboundary application, not a feedback scan).
fn apply(grid: &Grid, words: &[&Element]) -> Grid {
    let mut out = grid.clone();
    let (rows, cols) = (grid.len() as i64, grid.first().map_or(0, |r| r.len()) as i64);
    let mut writes: Vec<(i64, i64, u8)> = Vec::new();
    for w in words {
        for chain in variants(w) {
            let max_r = chain.iter().map(|(r, _, _, _)| *r).max().unwrap_or(0);
            let max_c = chain.iter().map(|(_, c, _, _)| *c).max().unwrap_or(0);
            for dr in 0..(rows - max_r).max(0) {
                for dc in 0..(cols - max_c).max(0) {
                    let fits = chain.iter().all(|(r, c, before, _)| {
                        grid[(r + dr) as usize][(c + dc) as usize] == *before
                    });
                    if fits {
                        for (r, c, _, after) in &chain {
                            writes.push((r + dr, c + dc, *after));
                        }
                    }
                }
            }
        }
    }
    for (r, c, v) in writes {
        out[r as usize][c as usize] = v;
    }
    out
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
        _ => {
            eprintln!("arc-solve: no language at {state}; the gate measures an empty tongue");
        }
    }
    let language = kernel.basis().to_vec();

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

    let (mut solved, mut wrong, mut unverified, mut mute, mut dim_skip) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let total = tasks.len();

    'task: for path in &tasks {
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        let Some(task) = parse_task(&text) else { continue };

        // SPEAK: every train pair must decompose over today's language.
        let mut cited: Vec<usize> = Vec::new();
        for (a, b) in &task.train {
            let Some(d) = delta(a, b) else {
                dim_skip += 1;
                continue 'task;
            };
            match decompose_over(&language, &center(&d), None, 4, 16) {
                Ok(sentence) => {
                    for wi in sentence {
                        if !cited.contains(&wi) {
                            cited.push(wi);
                        }
                    }
                }
                Err(_) => {
                    mute += 1;
                    continue 'task;
                }
            }
        }
        let words: Vec<&Element> = cited.iter().map(|&i| &language[i]).collect();

        // VERIFY: the sentence must REPLAY its own evidence — every train
        // input must reconstruct its train output exactly, or nothing is
        // predicted (the court's determinism, applied to speech).
        for (a, b) in &task.train {
            if apply(a, &words) != *b {
                unverified += 1;
                continue 'task;
            }
        }

        // SOLVE: the inverse deduction on every test input, scored exactly.
        let all_exact = task.test.iter().all(|(x, y)| {
            delta(x, y).is_some() && apply(x, &words) == *y
        });
        if all_exact && !task.test.is_empty() {
            solved += 1;
            eprintln!("  SOLVED {} — {} words cited", path.file_name().unwrap().to_string_lossy(), words.len());
        } else {
            wrong += 1;
        }
    }

    let attempted = solved + wrong;
    println!(
        "arc-solve: {total} tasks | SOLVED {solved} ({:.1}%) | wrong {wrong} | unverified {unverified} | mute {mute} | dim-skip {dim_skip} | attempted {attempted} (verify-gated)",
        100.0 * solved as f64 / total.max(1) as f64,
    );
}
