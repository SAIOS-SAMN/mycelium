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
use mycelium::physics::{reserve_max, Physics};
use mycelium::sdk::{Comprehension, Entity, Kernel, YieldQuote};
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
    // TUNIVERSAL: the universe's one wall clock (shared genesis on the
    // state) — every status line stamps with it, never local time.
    let tuniversal =
        mycelium::chronometry::TUniversal::load_or_genesis(&format!("{state_dir}/tuniversal"));

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
    let mut refined = 0u64; // accepted compression bids (0x09)
    let mut learned = 0u64; // syllabus words received (0x0D)
    let (mut last_genome, mut last_settles) = (usize::MAX, u64::MAX);
    // The feed walk: (task index within the cycle order, pair index within it).
    let mut cycle: Vec<PathBuf> = Vec::new();
    let (mut ti, mut pi) = (0usize, 0usize);
    let mut feeding = feed_dir.is_some() && !tasks.is_empty();

    loop {
        let mut judge_load = 0u8;
        // Stroke phase laps — one reading across every axis of the stroke.
        let mut laps: Vec<(&'static str, u128)> = Vec::new();
        let mut t_last = std::time::Instant::now();
        let mut lap = |label: &'static str| {
            laps.push((label, t_last.elapsed().as_millis()));
            t_last = std::time::Instant::now();
        };

        // ── TRANSPORT IMPEDANCE: perceive the wire before speaking. An
        // occluded aperture (a full stroke-cycle of consecutive failures)
        // turns this stroke INWARD — internal work instead of hammering a
        // saturated manifold; the dwell below scales with the impedance, so
        // resistance paces speech instead of throwing blind errors.
        let occluded =
            matches!(entity.transport(), mycelium::sdk::TransportAperture::Occluded { .. });
        if occluded {
            physics.spend([true; 4]); // the obstructed stroke is still a stroke
        }

        // THE ENTITY'S REDUCTION BUDGET — from its own metabolism: the beam
        // is one expansion's worth of branches, the depth one reserve of
        // strokes. Hardware-calibrated, never a constant.

        // ── FEED (entropic): present corpus transitions until the stroke
        // spends. A percept is a PROOF spoken over this genome: what the
        // genome reduces is cited, what it cannot is this voice's
        // irreducible claim; the sync converges the genome between strokes.
        while feeding && !occluded && !physics.depleted() {
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
                        let rx = embed(x, oh, ow); // the input on the codomain complex
                        let (guess, spoke) = entity.project(&rx);
                        if !spoke.is_empty() {
                            projected += 1;
                            if guess == *y {
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
            // THE EYE READS THROUGH ρ⁻¹ (sensor hardware, ruled): a
            // dimension-changing pair is spoken on the codomain complex — the
            // input pulled back along the exact frame map, the output native —
            // so the engine digests an ordinary chain bounded by the output.
            let (hi, wi) = (input.len(), input.first().map_or(0, |r| r.len()));
            let (ho, wo) = (output.len(), output.first().map_or(0, |r| r.len()));
            if hi == 0 || wi == 0 || ho == 0 || wo == 0 {
                continue;
            }
            let (a, b) = (grid_to_cells(&embed(input, ho, wo)), grid_to_cells(output));
            if a.is_empty() || b.is_empty() {
                continue;
            }
            match entity.submit(&a, &b, &(hi, wi, ho, wo)) {
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
        // An occluded wire skips the ask entirely (no error counted — the
        // impedance is already perceived); the answer is patience, not
        // repetition.
        lap("feed");
        match if occluded { Ok(YieldQuote { surface: [0; 4], clusters: Vec::new() }) } else { entity.quote() } {
            Ok(q) => {
                let (mut n_closed, mut n_descent, mut n_nothing, mut n_silent, mut n_attested, mut n_err) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
                for cluster in &q.clusters {
                    if cluster.chain.is_empty() {
                        continue;
                    }
                    // THINK, then CONVERSE. The superposition of hypotheses
                    // this genome forms for the listing, least holonomy
                    // defect first; the beam width is this lane's free
                    // reserve in expansions (the listing's strongest lane).
                    let lane = (0..4).max_by_key(|&l| cluster.mu[l]).unwrap_or(2);
                    // The superposition is rolled out and settled here, in
                    // the genome's own algebra; the wire carries one
                    // commitment per cluster: the concluded sentence with
                    // its residual. A genome with nothing to say stays silent.
                    let t0 = std::time::Instant::now();
                    let (cites, residual, familiar) = entity.conclude(&cluster.chain);
                    eprintln!(
                        "[{name}] [{}] cluster {} cells: concluded in {} ms -> {} cites, {} residual{}",
                        tuniversal.stamp(),
                        cluster.chain.len(),
                        t0.elapsed().as_millis(),
                        cites.len(),
                        residual.len(),
                        if familiar { " (familiar)" } else { "" },
                    );
                    // Two utterances at most: the sentence settles the
                    // listing; a residual not yet claimed is attested once
                    // as this voice's irreducible claim.
                    if cites.is_empty() && (residual.is_empty() || familiar) {
                        n_silent += 1;
                        continue;
                    }
                    let mut lanes: [bool; 4] = std::array::from_fn(|l| cluster.mu[l] > 0);
                    if lanes.iter().all(|&x| !x) {
                        lanes[lane] = true;
                    }
                    physics.spend(lanes); // the stroke spoken is metabolically real
                    if !cites.is_empty() {
                        match entity.probe(&cluster.address, &cites) {
                            Ok((code, _paid, _remaining)) => match code {
                                mycelium::anastome::dialogue::PROBE_CLOSED => {
                                    derived += 1;
                                    n_closed += 1;
                                    physics.bump(lanes);
                                }
                                mycelium::anastome::dialogue::PROBE_DESCENT => {
                                    witnessed += 1;
                                    n_descent += 1;
                                    physics.bump(lanes);
                                }
                                _ => n_nothing += 1,
                            },
                            Err(_) => {
                                errors += 1;
                                n_err += 1;
                            }
                        }
                    }
                    if !residual.is_empty() && !familiar {
                        match entity.attest(&residual, &cites) {
                            Ok(reply) => {
                                n_attested += 1;
                                if reply.settle.is_some() {
                                    settles += 1;
                                }
                                if let t @ [_, ..] = reply.verdict.taught() {
                                    entity.epigenome.learn(t);
                                }
                                judge_load = reply.load();
                            }
                            Err(_) => {
                                errors += 1;
                                n_err += 1;
                            }
                        }
                    }
                }
                eprintln!(
                    "[{name}] [{}] quote {} clusters: closed {n_closed} descent {n_descent} nothing {n_nothing} silent {n_silent} attested {n_attested} err {n_err}",
                    tuniversal.stamp(),
                    q.clusters.len(),
                );
                if !feeding && (q.clusters.iter().all(|c| c.chain.is_empty()) || q.clusters.is_empty()) {
                    // MARKET CLEAR with nothing to feed — the idle signal:
                    // spend the stroke so the dwell paces the next ask.
                    physics.spend([true; 4]);
                }
            }
            Err(e) => {
                errors += 1;
                eprintln!("[{name}] [{}] quote err: {e}", tuniversal.stamp());
                physics.spend([true; 4]); // an unreachable judge earns a full dwell
            }
        }

        lap("quote");
        // ── PULL REGISTERED WORK (PoUW): one signed envelope per stroke from
        // the hypha's authoritative registry — study the family, think the
        // tests through, submit. The REGISTRY rules bounty vs floor
        // (anti-replay yield collapse); unique solves count ONLY on BOUNTY.
        if let Ok(Some((tid, train, tests))) = if occluded { Ok(None) } else { entity.pull_task() } {
            let studied = entity.study(&train);
            physics.spend([true; 4]); // thought is metabolically real
            if studied > 0 {
                let _ = entity.flush_evidence();
                if let Some(((hn, hd), (wn, wd))) = family_ratio(&train) {
                    // JOINT CROSS-TEST CONSENSUS: one retained reading must
                    // serve EVERY test input of the family. The spectral
                    // head speaks first; if any sibling goes silent, the
                    // beam's rollout-verified alternative is tried whole.
                    // A reading that explains one grid but not its sibling
                    // is not understanding.
                    for v in 0..entity.dialect_variants() {
                        let mut outs: Vec<Vec<Vec<u8>>> = Vec::new();
                        let mut complete = !tests.is_empty();
                        for x in &tests {
                            let (h, w) = (x.len(), x.first().map_or(0, |r| r.len()));
                            if h == 0 || w == 0 || (h * hn) % hd != 0 || (w * wn) % wd != 0 {
                                complete = false;
                                break;
                            }
                            let (oh, ow) = (h * hn / hd, w * wn / wd);
                            let rx = embed(x, oh, ow); // the input on the codomain complex
                            let (guess, spoke) = entity.project_variant(&rx, v);
                            if spoke.is_empty() {
                                complete = false;
                                break;
                            }
                            outs.push(guess);
                        }
                        if complete {
                            if let Ok((code, _mu)) = entity.submit_solve(&tid, &outs) {
                                projected += 1;
                                match code {
                                    mycelium::anastome::dialogue::SOLVE_BOUNTY => {
                                        proj_solved += 1;
                                        uniq_solved += 1;
                                    }
                                    mycelium::anastome::dialogue::SOLVE_FLOOR => {
                                        proj_solved += 1
                                    }
                                    _ => {}
                                }
                            }
                            break; // the family is jointly served — one submission
                        }
                    }
                }
            }
        }

        // ── SYNTROPIC PHASE — crystallize, voice change, regenerate.
        if physics.depleted() {
            // THE SYLLABUS (distributed cognition): the crystallization phase
            // is the learning phase — pull the court-ratified stream from
            // this voice's cursor; the dialect re-resolves against the
            // higher-order language on the next study.
            if let Ok(n) = entity.pull_syllabus() {
                if n > 0 {
                    learned += n as u64;
                    eprintln!(
                        "[{name}] SYLLABUS +{n} ratified words (genome {})",
                        entity.epigenome.genome_len(),
                    );
                }
            }
            lap("task");
            // THE SHEAF SYNC (Q3): converge the genome on the hypha's
            // language by discordant stratum — the entity thinks in the
            // whole language, not the syllabus drip.
            if let Ok(n) = entity.sync_genome() {
                if n > 0 {
                    learned += n as u64;
                    eprintln!(
                        "[{name}] SYNC +{n} words by stratum (genome {})",
                        entity.epigenome.genome_len(),
                    );
                }
            }
            // REFINEMENT PROBE (0x09): one genome word per syntropic stroke —
            // if the rest of the vocabulary already speaks it, bid the
            // compression. The hypha's replay rules; on acceptance the word
            // retires fleet-wide and the entity earns the compression yield.
            lap("sync");
            if let Some((word, cites)) = entity.seek_refinement() {
                if let Ok((code, mu)) = entity.bid_refinement(&word, &cites) {
                    if code == mycelium::anastome::dialogue::REFINE_ACCEPTED {
                        refined += 1;
                        eprintln!(
                            "[{name}] REFINED {:02x}{:02x}… (vocabulary compressed, genome {}) μ⃗ {mu:?}",
                            word[0],
                            word[1],
                            entity.epigenome.genome_len(),
                        );
                    }
                }
            }
            lap("refine");
            eprintln!("[{name}] [{}] stroke laps ms {laps:?}", tuniversal.stamp());
            if entity.epigenome.genome_len() != last_genome || settles != last_settles {
                if entity.epigenome.genome_len() != last_genome {
                    let _ = entity.save_genome(&genome_path);
                }
                last_genome = entity.epigenome.genome_len();
                last_settles = settles;
                eprintln!(
                    "[{name}] [{}] fed {fed} derived {derived} witnessed {witnessed} settles {settles} err {errors} Z {} | SOLVES uniq {uniq_solved} verified {proj_solved}/{projected} | refined {refined} learned {learned} | genome {} (peak expr {}) discordance {}",
                    tuniversal.stamp(),
                    entity.impedance(),
                    entity.epigenome.genome_len(),
                    entity.epigenome.peak_expression(),
                    entity.discordance(),
                );
            }
            while physics.depleted() {
                physics.regenerate();
                physics.relax();
                // The dwell scales with the measured impedance: resistance on
                // the wire is a longer breath, not a retry storm.
                let dwell = (judge_load as u64)
                    .max(reserve_max())
                    .saturating_mul(1 + entity.impedance());
                std::thread::sleep(std::time::Duration::from_millis(dwell));
            }
        }
    }
}
