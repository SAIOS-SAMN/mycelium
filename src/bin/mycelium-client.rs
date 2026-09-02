// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! mycelium-client — THE ENTITY (the beta binary end users run). One kernel,
//! one genome, three verbs on a single two-stroke metabolism:
//!
//!   FEED  (--feed DIR)      present a corpus/universe's transitions as
//!                           percepts — ARC-AGI-2 ingestion is the first
//!                           proof-of-useful-work corpus; a world model's
//!                           board (--conductor) may schedule the order.
//!   WORK  (always)          pull the highest-yield pending demands off the
//!                           market (quote), think them over the OWN genome,
//!                           and attest the honest finding — a derived
//!                           sentence (closure) or an irreducibility witness
//!                           (coverage). The hypha replays either; false
//!                           speech never installs.
//!   THINK (Entity::project) the genome's anchored words project world
//!                           states forward — the intelligence embodied
//!                           kernels (games, universes) pull back out.
//!
//! Cadence is the entity's own two-stroke: feeding and working are the
//! entropic stroke (spending per-lane reserve on the lanes each reply
//! pressured); depletion flips to the syntropic phase (crystallize the
//! genome, voice the tally on change, regenerate, dwelling with the judge's
//! load). No counters, no timers. The genome persists (continuity); the
//! epigenome re-forms each run (dialect, by ruling).
//!
//! Usage: mycelium-client [ADDR] --kernel NAME [--state DIR]
//!            [--feed CORPUS_DIR] [--loop] [--limit N] [--conductor BOARD]
use mycelium::arc::{embed, frame_ratio, grid_to_cells, parse_task, readback};
use mycelium::physics::{Physics, RESERVE_MAX};
use mycelium::sdk::{Comprehension, Entity, Kernel};
use std::path::PathBuf;
use std::time::Instant;

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].as_str())
}
fn has(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a.max(1) } else { gcd(b, a % b) }
}
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

/// MEASURE a family's frame ratio from its pairs — perception (reading
/// dimensions), not derivation. None when the family disagrees: no single
/// morphism exists to think with.
fn family_ratio(train: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)]) -> Option<((usize, usize), (usize, usize))> {
    let mut ratio = None;
    for (a, b) in train {
        let f = (a.len(), a.first().map_or(0, |r| r.len()), b.len(), b.first().map_or(0, |r| r.len()));
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

/// The conductor's cursor — a world model's board schedules the corpus order.
fn conductor_cursor(path: &str) -> usize {
    let Ok(text) = std::fs::read_to_string(path) else { return 0 };
    text.split("\"arc_next\":")
        .nth(1)
        .and_then(|t| t.chars().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().ok())
        .unwrap_or(0)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let addr = args
        .get(1)
        .filter(|a| !a.starts_with("--"))
        .cloned()
        .unwrap_or_else(|| "127.0.0.1:7500".to_string());
    let name = flag(&args, "--kernel").unwrap_or("worker").to_string();
    let state_dir = flag(&args, "--state").unwrap_or("state").to_string();
    let feed_dir = flag(&args, "--feed").map(String::from);
    let looping = has(&args, "--loop");
    let limit: usize = flag(&args, "--limit").and_then(|s| s.parse().ok()).unwrap_or(usize::MAX);
    let conductor = flag(&args, "--conductor").map(String::from);
    let genome_path = format!("{state_dir}/{name}.genome");
    let _ = std::fs::create_dir_all(&state_dir);

    // The corpus (feeder role): every train pair, flattened once, re-walked
    // per cycle under the conductor's schedule when a board is given.
    let mut tasks: Vec<PathBuf> = Vec::new();
    if let Some(dir) = &feed_dir {
        tasks = std::fs::read_dir(dir)
            .map(|rd| {
                rd.filter_map(|e| e.ok().map(|e| e.path()))
                    .filter(|p| p.extension().is_some_and(|x| x == "json"))
                    .collect()
            })
            .unwrap_or_default();
        tasks.sort();
        tasks.truncate(limit);
    }

    let mut entity = Entity::open_persistent(&addr, Kernel::named(&name), &genome_path);
    eprintln!(
        "mycelium-client '{name}' → {addr}; genome {} words{}{}",
        entity.epigenome.genome_len(),
        match &feed_dir {
            Some(d) => format!("; feeding {} tasks from {d}{}", tasks.len(), if looping { " (looping)" } else { "" }),
            None => String::new(),
        },
        match &conductor {
            Some(b) => format!("; conducted by {b}"),
            None => String::new(),
        },
    );

    let mut physics = Physics::default();
    let (mut fed, mut derived, mut witnessed, mut settles, mut errors) = (0u64, 0u64, 0u64, 0u64, 0u64);
    // THE POUW SOLVE COUNTERS — projected = registry submissions tried;
    // proj_solved = verified correct (bounty + floor); uniq_solved = FIRST
    // verified solves (the only ones that count as new useful work).
    let (mut projected, mut proj_solved, mut uniq_solved) = (0u64, 0u64, 0u64);
    let (mut last_genome, mut last_settles) = (usize::MAX, u64::MAX);
    let start = Instant::now();
    // The feed walk: (task index within the cycle order, pair index within it).
    let mut cycle: Vec<PathBuf> = Vec::new();
    let (mut ti, mut pi) = (0usize, 0usize);
    let mut feeding = feed_dir.is_some() && !tasks.is_empty();

    loop {
        let mut judge_load = 0u8;

        // ── FEED (entropic): present corpus transitions until the stroke spends.
        while feeding && !physics.depleted() {
            if ti >= cycle.len() {
                // A new cycle: the conductor (if any) re-steers the order.
                cycle = tasks.clone();
                if let Some(board) = &conductor {
                    let cursor = conductor_cursor(board) % cycle.len().max(1);
                    cycle.rotate_left(cursor);
                }
                ti = 0;
                pi = 0;
                if !looping && fed > 0 {
                    feeding = false; // one full pass; the worker verb continues
                    break;
                }
            }
            let Some(path) = cycle.get(ti) else { break };
            let Ok(text) = std::fs::read_to_string(path) else {
                ti += 1;
                pi = 0;
                continue;
            };
            let Some(task) = parse_task(&text) else {
                ti += 1;
                pi = 0;
                continue;
            };
            let Some((input, output)) = task.train.get(pi) else {
                // TASK BOUNDARY — the entity THINKS before moving on:
                // STUDY the family it just fed (the epigenetic dialect
                // engine: lift → beam → rollout → crystallize, with every
                // verified binding seeding the court), then PROJECT each
                // test input dialect-first and score its instinct. The frame
                // ratio is MEASURED (perception); the output frame is
                // CONSTRUCTED from it — integral or refused, never rounded.
                let studied = entity.study(&task.train);
                physics.spend([true; 4]); // thought is metabolically real
                if studied > 0 {
                    let _ = entity.flush_evidence(); // local findings → the court
                }
                if let Some(((hn, hd), (wn, wd))) = family_ratio(&task.train) {
                    for (x, y) in &task.test {
                        if y.is_empty() {
                            continue; // withheld output: nothing to score against
                        }
                        let (h, w) = (x.len(), x.first().map_or(0, |r| r.len()));
                        if h == 0 || w == 0 || (h * hn) % hd != 0 || (w * wn) % wd != 0 {
                            continue; // non-integral frame: refusal
                        }
                        let (oh, ow) = (h * hn / hd, w * wn / wd);
                        let (lh, lw) = (lcm(h, oh), lcm(w, ow));
                        let rx = embed(x, lh, lw);
                        let (guess, spoke) = entity.project(&rx);
                        if !spoke.is_empty() {
                            projected += 1;
                            if readback(&guess, oh, ow).is_some_and(|g| g == *y) {
                                proj_solved += 1;
                            }
                        }
                    }
                }
                ti += 1;
                pi = 0;
                continue;
            };
            pi += 1;
            // THE EYE EMBEDS (sensor hardware, ruled): a dimension-changing
            // pair lifts to its common refinement BEFORE speech, so the
            // engine digests an ordinary chain — all cognition stays in the
            // engine; the client only widened its retina.
            let (hi, wi) = (input.len(), input.first().map_or(0, |r| r.len()));
            let (ho, wo) = (output.len(), output.first().map_or(0, |r| r.len()));
            let (a, b) = if (hi, wi) == (ho, wo) {
                (grid_to_cells(input), grid_to_cells(output))
            } else {
                if hi == 0 || wi == 0 || ho == 0 || wo == 0 {
                    continue;
                }
                let (lh, lw) = (lcm(hi, ho), lcm(wi, wo));
                (grid_to_cells(&embed(input, lh, lw)), grid_to_cells(&embed(output, lh, lw)))
            };
            if a.is_empty() || b.is_empty() {
                continue;
            }
            match entity.submit(&a, &b) {
                Ok((reply, heard)) => {
                    fed += 1;
                    if reply.settle.is_some() {
                        settles += 1;
                    }
                    match heard {
                        Some(Comprehension::Understood(_)) => derived += 1,
                        Some(Comprehension::Novel { .. }) => witnessed += 1,
                        None => {}
                    }
                    let mut lanes: [bool; 4] = std::array::from_fn(|l| reply.metabolism[l] > 0);
                    if lanes.iter().all(|&x| !x) {
                        lanes[2] = true;
                    }
                    physics.bump(lanes);
                    physics.spend(lanes);
                    judge_load = reply.load();
                }
                Err(_) => {
                    errors += 1;
                    physics.spend([true; 4]);
                }
            }
        }

        // ── WORK (entropic): pull the market's open demands and attest.
        match entity.quote() {
            Ok(q) => {
                for cluster in &q.clusters {
                    if cluster.chain.is_empty() {
                        continue;
                    }
                    let e = mycelium::transducer::element::from_residual(&cluster.chain);
                    let thought = entity.epigenome.comprehend(&e);
                    match entity.attest(&cluster.chain) {
                        Ok(reply) => {
                            match thought {
                                Comprehension::Understood(_) => derived += 1,
                                Comprehension::Novel { .. } => witnessed += 1,
                            }
                            if reply.settle.is_some() {
                                settles += 1;
                            }
                            if let t @ [_, ..] = reply.verdict.taught() {
                                entity.epigenome.learn(t);
                            }
                            let mut lanes: [bool; 4] =
                                std::array::from_fn(|l| reply.metabolism[l] > 0);
                            if lanes.iter().all(|&x| !x) {
                                lanes[2] = true;
                            }
                            physics.bump(lanes);
                            physics.spend(lanes);
                            judge_load = reply.load();
                        }
                        Err(_) => errors += 1,
                    }
                }
                if !feeding && (q.clusters.iter().all(|c| c.chain.is_empty()) || q.clusters.is_empty()) {
                    // MARKET CLEAR with nothing to feed — the idle signal:
                    // spend the stroke so the dwell paces the next ask.
                    physics.spend([true; 4]);
                }
            }
            Err(_) => {
                errors += 1;
                physics.spend([true; 4]); // an unreachable judge earns a full dwell
            }
        }

        // ── PULL REGISTERED WORK (PoUW): one signed envelope per stroke from
        // the hypha's authoritative registry — study the family, think the
        // tests through, submit. The REGISTRY rules bounty vs floor
        // (anti-replay yield collapse); unique solves count ONLY on BOUNTY.
        if let Ok(Some((tid, train, tests))) = entity.pull_task() {
            let studied = entity.study(&train);
            physics.spend([true; 4]); // thought is metabolically real
            if studied > 0 {
                let _ = entity.flush_evidence();
                if let Some(((hn, hd), (wn, wd))) = family_ratio(&train) {
                    let mut outs: Vec<Vec<Vec<u8>>> = Vec::new();
                    let mut complete = !tests.is_empty();
                    for x in &tests {
                        let (h, w) = (x.len(), x.first().map_or(0, |r| r.len()));
                        if h == 0 || w == 0 || (h * hn) % hd != 0 || (w * wn) % wd != 0 {
                            complete = false;
                            break;
                        }
                        let (oh, ow) = (h * hn / hd, w * wn / wd);
                        let (lh, lw) = (lcm(h, oh), lcm(w, ow));
                        let rx = embed(x, lh, lw);
                        let (guess, spoke) = entity.project(&rx);
                        if spoke.is_empty() {
                            complete = false;
                            break;
                        }
                        match readback(&guess, oh, ow) {
                            Some(g) => outs.push(g),
                            None => {
                                complete = false;
                                break;
                            }
                        }
                    }
                    if complete {
                        if let Ok((code, _mu)) = entity.submit_solve(&tid, &outs) {
                            projected += 1;
                            match code {
                                mycelium::anastome::dialogue::SOLVE_BOUNTY => {
                                    proj_solved += 1;
                                    uniq_solved += 1;
                                }
                                mycelium::anastome::dialogue::SOLVE_FLOOR => proj_solved += 1,
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        // ── SYNTROPIC PHASE — crystallize, voice change, regenerate.
        if physics.depleted() {
            if entity.epigenome.genome_len() != last_genome || settles != last_settles {
                if entity.epigenome.genome_len() != last_genome {
                    let _ = entity.save_genome(&genome_path);
                }
                last_genome = entity.epigenome.genome_len();
                last_settles = settles;
                eprintln!(
                    "[{name}] {:.0}s | fed {fed} derived {derived} witnessed {witnessed} settles {settles} err {errors} | SOLVES uniq {uniq_solved} verified {proj_solved}/{projected} | genome {} (peak expr {})",
                    start.elapsed().as_secs_f64(),
                    entity.epigenome.genome_len(),
                    entity.epigenome.peak_expression(),
                );
            }
            while physics.depleted() {
                physics.regenerate();
                physics.relax();
                let dwell = (judge_load as u64).max(RESERVE_MAX as u64);
                std::thread::sleep(std::time::Duration::from_millis(dwell));
            }
        }
    }
}
