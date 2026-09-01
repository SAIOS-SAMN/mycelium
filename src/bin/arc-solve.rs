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
use mycelium::transducer::anchor::{derive_exact_all, Sel};
use mycelium::transducer::element::{assignment, Element};
use std::path::PathBuf;

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].as_str())
}

/// The FULL complex of a grid — every cell including background hue 0: the
/// selection algebra must see holes and halos, which live on hue-0 cells.
fn full_scene(g: &Grid) -> Vec<(i64, i64, u8)> {
    g.iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, &h)| (r as i64, c as i64, h)))
        .collect()
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

/// A word's placements inside a train pair's ABSOLUTE coboundary: every
/// translation of every family variant whose cells all sit in the δ with the
/// exact (before, after) transitions. Returns the union footprint — the
/// word's measured domain in that pair.
fn chain_footprint(chain: &[(i64, i64, u8, u8)], d: &[(i64, i64, u8, u8)]) -> Vec<(i64, i64)> {
    let mut fp: Vec<(i64, i64)> = Vec::new();
    let Some(&(r0, c0, _, _)) = chain.first() else { return fp };
    for &(ar, ac, ab, aa) in d {
        // Anchor the chain's first cell on this δ cell.
        if (ab, aa) != (chain[0].2, chain[0].3) {
            continue;
        }
        let (dr, dc) = (ar - r0, ac - c0);
        let all_in =
            chain.iter().all(|(r, c, b, a)| d.binary_search(&(r + dr, c + dc, *b, *a)).is_ok());
        if all_in {
            fp.extend(chain.iter().map(|(r, c, _, _)| (r + dr, c + dc)));
        }
    }
    fp.sort_unstable();
    fp.dedup();
    fp
}

/// THE INVERSE DEDUCTION — apply anchored words to a grid: each cited word
/// rewrites at every translation where its whole before-side holds on the
/// pre-state AND (when the word is BOUND to a derived selection term) its
/// footprint lies inside the term's evaluation on this scene — γ ∘ π: the
/// substance restricted to the anchored subcomplex, never an ambient sweep.
/// Placements are found against the unmodified grid, then written together.
fn apply(grid: &Grid, spoken: &[(Vec<(i64, i64, u8, u8)>, Option<Sel>)]) -> Grid {
    let mut out = grid.clone();
    let (rows, cols) = (grid.len() as i64, grid.first().map_or(0, |r| r.len()) as i64);
    let scene = full_scene(grid);
    let mut writes: Vec<(i64, i64, u8)> = Vec::new();
    for (chain, sel) in spoken {
        // π evaluated once per scene: the admissible subcomplex.
        let region: Option<Vec<(i64, i64)>> = sel.as_ref().map(|t| t.eval(&scene));
        let max_r = chain.iter().map(|(r, _, _, _)| *r).max().unwrap_or(0);
        let max_c = chain.iter().map(|(_, c, _, _)| *c).max().unwrap_or(0);
        for dr in 0..(rows - max_r).max(0) {
            for dc in 0..(cols - max_c).max(0) {
                let fits = chain.iter().all(|(r, c, before, _)| {
                    grid[(r + dr) as usize][(c + dc) as usize] == *before
                });
                let anchored = region.as_ref().map_or(true, |reg| {
                    chain.iter().all(|(r, c, _, _)| reg.binary_search(&(r + dr, c + dc)).is_ok())
                });
                if fits && anchored {
                    for (r, c, _, after) in chain {
                        writes.push((r + dr, c + dc, *after));
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
    let mut probe: usize = flag(&args, "--probe").and_then(|s| s.parse().ok()).unwrap_or(0);

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
        let mut deltas: Vec<Vec<(i64, i64, u8, u8)>> = Vec::new();
        for (a, b) in &task.train {
            let Some(mut d) = delta(a, b) else {
                dim_skip += 1;
                continue 'task;
            };
            d.sort_unstable();
            match decompose_over(&language, &center(&d), None, 4, 16) {
                Ok(sentence) => {
                    for wi in sentence {
                        if !cited.contains(&wi) {
                            cited.push(wi);
                        }
                    }
                    deltas.push(d);
                }
                Err(_) => {
                    mute += 1;
                    continue 'task;
                }
            }
        }

        // WITNESS + ANCHOR, per (word, VARIANT): a punctured thought speaks
        // many variants, but only the variants the train family actually
        // WITNESSED belong to this task's dialect — the rest stay silent (a
        // 73-section cell-word is not "anything goes"). Each witnessed
        // variant then derives its own selection term on every pair it placed
        // in; the binding term is the minimal member of the INTERSECTION
        // across those pairs (one pair underdetermines; the family decides).
        // No common term → the variant applies ambient, and the replay gate
        // below decides whether ambient speech survives.
        let mut spoken: Vec<(Vec<(i64, i64, u8, u8)>, Option<Sel>)> = Vec::new();
        for &wi in &cited {
            for chain in variants(&language[wi]) {
                let mut sets: Vec<Vec<Sel>> = Vec::new();
                let mut witnessed = false;
                for ((a, _), d) in task.train.iter().zip(&deltas) {
                    let fp = chain_footprint(&chain, d);
                    if fp.is_empty() {
                        continue; // this variant did not speak in this pair
                    }
                    witnessed = true;
                    sets.push(derive_exact_all(&fp, &full_scene(a)));
                }
                if !witnessed {
                    continue;
                }
                let consensus = match sets.split_first() {
                    Some((first, rest)) => {
                        first.iter().find(|t| rest.iter().all(|s| s.contains(t))).cloned()
                    }
                    None => None,
                };
                spoken.push((chain, consensus));
            }
        }

        // VERIFY: the sentence must REPLAY its own evidence — every train
        // input must reconstruct its train output exactly, or nothing is
        // predicted (the court's determinism, applied to speech).
        for (pi, (a, b)) in task.train.iter().enumerate() {
            let got = apply(a, &spoken);
            if got != *b {
                if probe > 0 {
                    probe -= 1;
                    let extra = got.iter().flatten().zip(b.iter().flatten()).filter(|(g, e)| g != e).count();
                    let dsz = deltas.get(pi).map_or(0, |d| d.len());
                    eprintln!(
                        "  PROBE {}: pair {pi}: {} spoken [{}], δ {dsz} cells, {extra} wrong cells",
                        path.file_name().unwrap().to_string_lossy(),
                        spoken.len(),
                        spoken
                            .iter()
                            .map(|(c, s)| format!(
                                "sup{}{}",
                                c.len(),
                                if s.is_some() { "@A" } else { "@amb" }
                            ))
                            .collect::<Vec<_>>()
                            .join(" "),
                    );
                }
                unverified += 1;
                continue 'task;
            }
        }

        // SOLVE: the inverse deduction on every test input, scored exactly.
        let all_exact = task.test.iter().all(|(x, y)| {
            delta(x, y).is_some() && apply(x, &spoken) == *y
        });
        if all_exact && !task.test.is_empty() {
            solved += 1;
            let anchored = spoken.iter().filter(|(_, s)| s.is_some()).count();
            eprintln!(
                "  SOLVED {} — {} chains spoken, {anchored} anchored",
                path.file_name().unwrap().to_string_lossy(),
                spoken.len()
            );
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
