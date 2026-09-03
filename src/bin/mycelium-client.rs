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
use mycelium::arc::{embed, grid_to_cells, parse_task};
use mycelium::physics::{reserve_max, Physics};
use mycelium::transducer::element::measure_lanes;
use mycelium::transducer::germinate::germinate;
use mycelium::sdk::{Comprehension, Entity, Kernel, YieldQuote};
use std::path::PathBuf;

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].as_str())
}
fn has(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}
/// The magnitude of a family, per lane: the sum over its pairs of the
/// coboundary's lane magnitudes — what thinking it through is priced on.
/// SPEECH IS A CLOSED CONNECTION (the overlay is apply g). A thought that
/// still has a remainder is work, not a message: the family's inputs and
/// the tests are replayed through the transport, and any cell the
/// connection cannot speak keeps the whole thought here, as this mind's
/// open remainder, until it closes. Only when every replay is closed is one
/// connection per rule minted — the versor's affine block and the gauge as
/// the group component, the volume spoken across the family on the lanes,
/// and an EMPTY chain — held in the epigenome and spoken up as the word.
/// Nothing residual reaches the wire. Returns (spoken, held as thought,
/// errors).
fn speak_connections(
    entity: &mut Entity,
    name: &str,
    transport: &mycelium::transducer::family::Transport,
    train: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)],
    tests: &[Vec<Vec<u8>>],
) -> (u64, u64, u64) {
    let mut carried = [0u64; 4];
    let mut remainder: Vec<(i64, i64, u8, u8)> = Vec::new();
    for x in train.iter().map(|(a, _)| a).chain(tests.iter()) {
        let Some(speech) = mycelium::transducer::family::speak(transport, x) else {
            // The connection refuses this scene outright: the scene is the remainder.
            remainder.extend(grid_to_cells(x).into_iter().map(|(r, c, h)| (r, c, h, h)));
            continue;
        };
        carried[0] = carried[0].saturating_add(speech.cells.len() as u64);
        remainder.extend(speech.remainder.iter().map(|&(r, c, h)| (r, c, h, h)));
    }
    remainder.sort_unstable();
    remainder.dedup();
    if let [_, ..] = remainder.as_slice() {
        // THOUGHT, NOT SPEECH: the remainder stays here until it closes.
        entity.hold_thought(remainder.clone());
        eprintln!("[{name}] thought held: {} rules leave {} cells unspoken — not uttered", transport.rules.len(), remainder.len());
        return (0, 1, 0);
    }
    let (mut spoken, mut errors) = (0u64, 0u64);
    for rule in &transport.rules {
        let Some((_, u)) = rule.readings.iter().find(|(r, _)| *r == 0).or(rule.readings.first()) else { continue };
        let Some(affine) = mycelium::transducer::element::affine_of(u) else {
            eprintln!("[{name}] connection not affine: a rule's versor has no affine block — held, not uttered");
            errors += 1;
            continue;
        };
        let gauge = rule.sigma.iter().filter(|(f, t)| f != t).count() as u64;
        let mut mag = carried;
        mag[1] = mag[1].saturating_add(gauge);
        let word = mycelium::transducer::element::from_connection(&affine, &rule.sigma, &Vec::new(), mag);
        match entity.speak_word(&word) {
            Ok(()) => {
                spoken += 1;
                eprintln!(
                    "[{name}] connection spoken: {:02x}{:02x}… closed, carried {:?}",
                    word.address()[0],
                    word.address()[1],
                    word.magnitude()
                );
            }
            Err(_) => errors += 1,
        }
    }
    (spoken, 0, errors)
}

fn family_magnitude(train: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)]) -> [u64; 4] {
    let mut mag = [0u64; 4];
    for (x, y) in train {
        let (ho, wo) = (y.len(), y.first().map_or(0, |r| r.len()));
        if x.is_empty() || ho == 0 || wo == 0 {
            continue;
        }
        let (a, b) = (grid_to_cells(&embed(x, ho, wo)), grid_to_cells(y));
        if let Some(g) = germinate(&a, &b) {
            let m = measure_lanes(&g);
            for l in 0..4 {
                mag[l] = mag[l].saturating_add(m[l]);
            }
        }
    }
    mag
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
    let mut k_step = 0u64; // the judge's k-step last read: this voice's temporal coordinate
    // Listings this voice has concluded in their current state: a listing
    // that descends changes address and is read again.
    let mut concluded: Vec<[u8; 32]> = Vec::new();
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
            // The inward stroke glues: one sync contact per stroke of the
            // cycle, until the digests agree or the cycle is spent.
            let cycle = (reserve_max() / mycelium::physics::spend_unit().max(1)).max(1);
            for _ in 0..cycle {
                match entity.sync_genome() {
                    Ok(n) if n > 0 => learned += n as u64,
                    _ => break,
                }
            }
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
                // TASK BOUNDARY — the entity thinks before moving on: the
                // family it just fed is comprehended as a connection, each
                // test input is spoken through it and scored, and a family
                // the connection does not speak raises its residual as
                // this voice's demand.
                let comprehended = mycelium::transducer::family::comprehend(&task.train);
                let mag = family_magnitude(&task.train);
                physics.bump(&mag);
                physics.spend(Physics::lanes_of(&mag)); // thought is priced on the lanes it occupies
                match comprehended.transport {
                    Some(transport) => {
                        // Score each held answer through the connection, then
                        // mint the connection as the word (one helper, the
                        // same mint as the live task path).
                        for (x, y) in &task.test {
                            if y.is_empty() {
                                continue; // withheld output: nothing to score against
                            }
                            if let Some(speech) = mycelium::transducer::family::speak(&transport, x) {
                                if let [] = speech.remainder.as_slice() {
                                    projected += 1;
                                    if speech.grid() == *y {
                                        proj_solved += 1;
                                    }
                                }
                            }
                        }
                        let tests: Vec<Vec<Vec<u8>>> = task.test.iter().map(|(x, _)| x.clone()).collect();
                        let (_, _, e) = speak_connections(&mut entity, &name, &transport, &task.train, &tests);
                        errors += e;
                    }
                    // The family's pairs were already spoken as percepts
                    // by the feed; an uncomprehended family is on the book.
                    None => {}
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
                    // The percept's own magnitude, per lane, is the pressure
                    // this stroke adds and the lanes it spends along.
                    let mag = germinate(&a, &b).map(|g| measure_lanes(&g)).unwrap_or([0; 4]);
                    physics.bump(&mag);
                    physics.spend(Physics::lanes_of(&mag));
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
        match if occluded { Ok(YieldQuote { surface: [0; 4], k: k_step, clusters: Vec::new() }) } else { entity.quote() } {
            Ok(q) => {
                k_step = q.k;
                entity.at(q.k);
                concluded.retain(|a| q.clusters.iter().any(|c| c.address == *a));
                let (mut n_closed, mut n_descent, mut n_nothing, mut n_settled, mut n_silent, mut n_attested, mut n_err) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
                for cluster in &q.clusters {
                    if cluster.chain.is_empty() {
                        continue;
                    }
                    // THINK, then CONVERSE. The superposition of hypotheses
                    // this genome forms for the listing, least holonomy
                    // defect first; the beam width is this lane's free
                    // reserve in expansions (the listing's strongest lane).
                    // The superposition is rolled out and settled here, in
                    // the genome's own algebra; the wire carries one
                    // commitment per cluster: the concluded sentence with
                    // its residual. A genome with nothing to say stays silent.
                    if concluded.contains(&cluster.address) {
                        n_silent += 1;
                        continue; // already concluded in this state
                    }
                    concluded.push(cluster.address);
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
                    if let (0, []) = (cites.len(), residual.as_slice()) {
                        n_silent += 1;
                        continue;
                    }
                    let mag = measure_lanes(&cluster.chain);
                    physics.spend(Physics::lanes_of(&mag)); // the stroke spoken is metabolically real
                    if !cites.is_empty() {
                        match entity.probe(&cluster.address, &cites) {
                            Ok((code, _paid, _remaining)) => match code {
                                mycelium::anastome::dialogue::PROBE_CLOSED => {
                                    derived += 1;
                                    n_closed += 1;
                                    physics.bump(&mag);
                                }
                                mycelium::anastome::dialogue::PROBE_DESCENT => {
                                    witnessed += 1;
                                    n_descent += 1;
                                    physics.bump(&mag);
                                }
                                mycelium::anastome::dialogue::PROBE_SETTLED => n_settled += 1,
                                _ => n_nothing += 1,
                            },
                            Err(_) => {
                                errors += 1;
                                n_err += 1;
                            }
                        }
                    }
                    // The residual is attested whenever it stands: consensus is
                    // the hypha's receipts across voices, never this voice's
                    // memory of having said it. A familiar residual is one this
                    // genome already expressed — said again for the quorum.
                    let _ = familiar;
                    if let [_, ..] = residual.as_slice() {
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
                    "[{name}] [{}] quote {} clusters: closed {n_closed} descent {n_descent} nothing {n_nothing} settled {n_settled} silent {n_silent} attested {n_attested} err {n_err}",
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
            // The family is comprehended as a connection in the entity's own
            // algebra; every test is spoken through it, or the family's
            // residual is the demand it raises.
            let comprehended = mycelium::transducer::family::comprehend(&train);
            let mag = family_magnitude(&train);
            physics.bump(&mag);
            physics.spend(Physics::lanes_of(&mag)); // thought is priced on the lanes it occupies
            eprintln!(
                "[{name}] [{}] family {:02x}{:02x}… {} pairs, {} tests: {}",
                tuniversal.stamp(),
                tid[0],
                tid[1],
                train.len(),
                tests.len(),
                match (&comprehended.transport, comprehended.framed) {
                    (Some(t), _) => format!("comprehended ({} rules, {} created, {} removed)", t.rules.len(), t.created.len(), t.removed.len()),
                    (None, true) => format!("residual {} cells", comprehended.residual.len()),
                    (None, false) => "frames disagree".to_string(),
                },
            );
            match comprehended.transport {
                Some(transport) => {
                    // THE MINT ON THE LIVE PATH: the comprehended connection
                    // is spoken up as the word before any solve is projected.
                    let (_, _, e) = speak_connections(&mut entity, &name, &transport, &train, &tests);
                    errors += e;
                    let outs: Vec<Option<Vec<Vec<u8>>>> =
                        tests
                            .iter()
                            .map(|x| {
                                mycelium::transducer::family::speak(&transport, x).and_then(|s| match s.remainder.as_slice() {
                                    [] => Some(s.grid()),
                                    _ => None, // an unspoken hue is not submitted as a guess
                                })
                            })
                            .collect();
                    if !tests.is_empty() && outs.iter().all(|o| o.is_some()) {
                        let outs: Vec<Vec<Vec<u8>>> = outs.into_iter().flatten().collect();
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
                None => {
                    // THINKING, NOT SPEECH: a family the connection does not
                    // collapse is this mind's open remainder. Its pairs are
                    // germinated and held here as thought — never spoken as
                    // percept pairs, never entered as vocabulary. The corpus
                    // itself stands on the hypha's book as market demand; the
                    // entity's work is to close the remainder into a word.
                    let mut held = 0usize;
                    for (x, y) in &train {
                        let (hi, wi) = (x.len(), x.first().map_or(0, |r| r.len()));
                        let (ho, wo) = (y.len(), y.first().map_or(0, |r| r.len()));
                        if hi == 0 || wi == 0 || ho == 0 || wo == 0 {
                            continue;
                        }
                        let (a, b) = (grid_to_cells(&embed(x, ho, wo)), grid_to_cells(y));
                        let Some(germ) = germinate(&a, &b) else { continue };
                        entity.hold_thought(germ);
                        held += 1;
                    }
                    if held > 0 {
                        eprintln!("[{name}] thought held: family {:02x}{:02x}… {held} pairs uncollapsed — not uttered", tid[0], tid[1]);
                    }
                }
            }
        }

        // THE SYLLABUS: the court-ratified stream from this voice's cursor,
        // pulled every stroke — teaching is part of the conversation, not a
        // charity of the depleted phase.
        if !occluded {
            if let Ok(n) = entity.pull_syllabus() {
                if n > 0 {
                    learned += n as u64;
                    eprintln!(
                        "[{name}] SYLLABUS +{n} ratified words (genome {})",
                        entity.epigenome.genome_len(),
                    );
                }
            }
        }

        // ── SYNTROPIC PHASE — crystallize, voice change, regenerate.
        if physics.depleted() {
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
                    } else if entity.epigenome.retire(&word) {
                        // This genome derives the word by its other words and
                        // the judge does not hold it: the copy is compression
                        // this dialect owes itself. Retired locally; a word the
                        // judge does hold returns through the sync.
                        refined += 1;
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
                    "[{name}] [{}] K {k_step} | fed {fed} derived {derived} witnessed {witnessed} settles {settles} err {errors} Z {} | SOLVES uniq {uniq_solved} verified {proj_solved}/{projected} | refined {refined} learned {learned} | genome {} (peak expr {}) discordance {} obstructions {} thoughts {}",
                    tuniversal.stamp(),
                    entity.impedance(),
                    entity.epigenome.genome_len(),
                    entity.epigenome.peak_expression(),
                    entity.discordance(),
                    entity.obstructions().len(),
                    entity.thoughts().len(),
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
