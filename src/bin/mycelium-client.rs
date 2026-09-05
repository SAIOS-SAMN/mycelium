// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! mycelium-client — THE ENTITY (the beta binary end users run). One kernel,
//! one genome, three verbs on a single two-stroke metabolism:
//!
//!   FEED  (home/feed/)      present a corpus/universe's transitions as
//!                           percepts — files dropped here are spoken until
//!                           the directory is empty. A conductor file at
//!                           home/conductor may schedule the order.
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
//! Usage: mycelium-client --home DIR
use mycelium::arc::{embed, grid_to_cells, parse_task};
use mycelium::physics::Physics;
use mycelium::transducer::element::measure_lanes;
use mycelium::transducer::germinate::germinate;
use mycelium::court::identity::HyphaKey;
use mycelium::network::mesh;
use mycelium::sdk::{Comprehension, Entity, Kernel};
use mycelium::voice::Voice;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].as_str())
}

fn home_dir(args: &[String]) -> Result<PathBuf, String> {
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--home" {
            i += 2;
            continue;
        }
        if args[i].starts_with("--") {
            return Err(format!("unknown flag {}", args[i]));
        }
        return Err(format!("unexpected argument {}", args[i]));
    }
    if let Some(p) = flag(args, "--home") {
        return Ok(PathBuf::from(p));
    }
    if let Ok(p) = std::env::var("MYCELIUM_HOME") {
        return Ok(PathBuf::from(p));
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    Ok(PathBuf::from(home).join(".mycelium"))
}

fn pending_feed(home: &Path) -> Vec<PathBuf> {
    let dir = home.join("feed");
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.extension().is_some_and(|x| x == "json"))
                .collect()
        })
        .unwrap_or_default();
    files.sort();
    files
}

fn retire_feed(path: &Path) {
    let Some(parent) = path.parent() else { return };
    let done = parent.join(".done");
    let _ = std::fs::create_dir_all(&done);
    if let Some(name) = path.file_name() {
        let _ = std::fs::rename(path, done.join(name));
    }
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
/// Speak a closed multi-germ sentence as one Element; a withheld sentence
/// is thought. Returns true when the sentence was uttered, so per-rule
/// minting is skipped.
fn speak_sentence(entity: &mut Entity, tag: &str, n: &mycelium::transducer::sentence::Nerve, family: &[u8; 32]) -> bool {
    let fam = format!("{:02x}{:02x}…", family[0], family[1]);
    let why_open = || {
        if let Some((i, c)) = n.cycles.iter().enumerate().find(|(_, c)| !c.holonomy.abelian_identity()) {
            format!(
                "holonomy on cycle {i} = {:02x}{:02x}…",
                c.holonomy.key()[0],
                c.holonomy.key()[1]
            )
        } else if !n.remainder.is_empty() {
            format!("remainder {} cells", n.remainder.len())
        } else if n.class.is_some() {
            "H² class present".to_string()
        } else if n.germ_count() == 1 {
            "identity 1-morphism is not a sentence".to_string()
        } else {
            "not closed".to_string()
        }
    };
    if !n.is_sentence() {
        entity.hold_thought(n.remainder.clone());
        if let Some(word) = mycelium::transducer::sentence::as_element(n) {
            entity.hold_sentence(&word);
        }
        eprintln!("[{tag}] sentence withheld: family {fam}, {}", why_open());
        return false;
    }
    let Some(word) = mycelium::transducer::sentence::as_element(n) else {
        eprintln!("[{tag}] sentence withheld: family {fam}, not an element");
        return false;
    };
    if entity.already_taught(&word) {
        entity.remember_spoken(&word);
        return true;
    }
    match entity.speak(std::slice::from_ref(&word)) {
        Ok(codes) => match codes.first().copied() {
            Some(mycelium::anastome::dialogue::SPEECH_REFUSED) => {
                eprintln!("[{tag}] sentence withheld: family {fam}, refused on the wire");
                false
            }
            _ => {
                eprintln!(
                    "[{tag}] sentence formed: {:02x}{:02x}… {} germs, {} faces (constant relative words), carried {:?}",
                    word.address()[0],
                    word.address()[1],
                    n.germs.len(),
                    n.faces.len(),
                    word.magnitude()
                );
                true
            }
        }
        Err(e) => {
            eprintln!("[{tag}] sentence withheld: family {fam}, {e}");
            false
        }
    }
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
    let home = match home_dir(&args) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("mycelium-client --home DIR: {e}");
            std::process::exit(2);
        }
    };
    let _ = std::fs::create_dir_all(&home);
    let key_path = home.join("voice.key");
    let voice_key = HyphaKey::load_or_mint(key_path.to_str().unwrap_or("voice.key"));
    let voice = Voice::from_identity(voice_key.vk_bytes());
    let tag = voice.name().to_string();
    let mesh_rec = mesh::load_or_migrate_voice(&home, &voice_key);
    let genome_path = home.join("genome").to_string_lossy().into_owned();
    let thoughts_path = home.join("thoughts").to_string_lossy().into_owned();
    let bootstrap = if mesh_rec.bootstrap.is_empty() {
        "127.0.0.1:7500".to_string()
    } else {
        mesh_rec.bootstrap.clone()
    };
    let conductor = None::<String>;
    let tuniversal = mycelium::chronometry::TUniversal::from_origin(mesh_rec.origin_unix);

    let mut tasks = pending_feed(&home);
    let mut entity = Entity::open_persistent(
        &bootstrap,
        Kernel::from_voice(voice.identity()),
        &genome_path,
        &thoughts_path,
    );
    entity.set_tuniversal(tuniversal);
    entity.discover();
    eprintln!(
        "[{tag}] mycelium-client → {bootstrap}; genome {} words{}",
        entity.epigenome.genome_len(),
        if tasks.is_empty() {
            String::new()
        } else {
            format!("; feeding {} from {}/feed", tasks.len(), home.display())
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
    let (mut rules_extracted, mut rules_held_class, mut tests_full, mut solves_paid) = (0u64, 0u64, 0u64, 0u64);
    let (mut last_genome, mut last_settles) = (usize::MAX, u64::MAX);
    let mut k_step = 0u64; // the judge's k-step last read: this voice's temporal coordinate
    // The hypha's spoken receipts, not a local memo, decide which listings
    // this voice has already worked; the quote page is already filtered.
    // The feed walk: (task index within the cycle order, pair index within it).
    let mut cycle: Vec<PathBuf> = Vec::new();
    let (mut ti, mut pi) = (0usize, 0usize);
    let mut feeding = !tasks.is_empty();
    let stop = Arc::new(AtomicBool::new(false));
    {
        let stop = Arc::clone(&stop);
        let _ = ctrlc::set_handler(move || stop.store(true, Ordering::SeqCst));
    }
    let mut last_genome_save = Instant::now();
    let mut saved_genome = entity.epigenome.genome_len();
    let mut extract_memo: mycelium::network::nerve::Keyed<(
        [u8; 32],
        Option<mycelium::transducer::sentence::SentenceRule>,
    )> = mycelium::network::nerve::Keyed::new();

    loop {
        if stop.load(Ordering::SeqCst) {
            let _ = entity.save_genome(&genome_path);
            let _ = entity.save_thoughts(&thoughts_path);
            break;
        }
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
            // Sync waits for the depletion dwell: every-stroke digest/fetch
            // saturates the door and starves quote and percept.
        }

        // THE ENTITY'S REDUCTION BUDGET — from its own metabolism: the beam
        // is one expansion's worth of branches, the depth one reserve of
        // strokes. Hardware-calibrated, never a constant.

        // ── FEED (entropic): present corpus transitions until the stroke
        // spends. A percept is a PROOF spoken over this genome: what the
        // genome reduces is cited, what it cannot is this voice's
        // irreducible claim; the sync converges the genome between strokes.
        if !feeding {
            tasks = pending_feed(&home);
            feeding = !tasks.is_empty();
        }
        while feeding && !occluded && !physics.depleted() {
            if ti >= cycle.len() {
                tasks = pending_feed(&home);
                cycle = tasks.clone();
                if cycle.is_empty() {
                    feeding = false;
                    break;
                }
                if let Some(board) = &conductor {
                    let cursor = conductor_cursor(board) % cycle.len().max(1);
                    cycle.rotate_left(cursor);
                }
                ti = 0;
                pi = 0;
            }
            let Some(path) = cycle.get(ti) else { break };
            let Ok(text) = std::fs::read_to_string(path) else {
                retire_feed(path);
                ti += 1;
                pi = 0;
                continue;
            };
            let Some(task) = parse_task(&text) else {
                retire_feed(path);
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
                // The rule's nerve is the sentence: constant relative words,
                // not pair-0 geometry. No calculator fallback.
                let fam = mycelium::registry::task_id(&task);
                let gen = entity.epigenome.atlas_root();
                let mut extract_budget = physics.stroke();
                let extracted = mycelium::transducer::family::extract_rule_cached(
                    &mut extract_memo,
                    &fam,
                    &task.train,
                    gen,
                    &mut extract_budget,
                    &entity.epigenome,
                );
                if let Some(rule) = extracted.as_ref().or(comprehended.rule.as_ref()) {
                    for (x, y) in &task.test {
                        if y.is_empty() {
                            continue;
                        }
                        if let Some(speech) = mycelium::transducer::family::speak_rule(rule, x, &entity.epigenome) {
                            if speech.remainder.is_empty() {
                                projected += 1;
                                if speech.grid() == *y {
                                    proj_solved += 1;
                                }
                            }
                        }
                    }
                    let _ = speak_sentence(&mut entity, &tag, &rule.nerve, &fam);
                }
                retire_feed(path);
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
                        Some(Comprehension::Understood(_)) | Some(Comprehension::Held) | Some(Comprehension::Admitted) => derived += 1,
                        Some(Comprehension::Novel { .. }) | Some(Comprehension::Dialect(_)) => witnessed += 1,
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
        match entity.quote() {
            Ok(q) => {
                // An empty WouldBlock is impedance, not a clock: do not
                // stamp K=0 over the receipt-chain head last read.
                let heard = q.k != 0 || q.surface.iter().any(|&x| x != 0) || !q.clusters.is_empty();
                if heard {
                    k_step = q.k;
                    entity.at(q.k);
                }
                let (mut n_closed, mut n_descent, mut n_nothing, mut n_settled, mut n_silent, mut n_attested, mut n_err) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
                let (mut n_closing, mut n_stood, mut n_held) = (0u32, 0u32, 0u32);
                for cluster in &q.clusters {
                    if cluster.chain.is_empty() {
                        continue;
                    }
                    // Incidence: working atlas fiber, each chart with its standing.
                    if !cluster.open.is_empty() {
                        let closing: Vec<mycelium::transducer::stalk::Invariant> = cluster
                            .open
                            .iter()
                            .filter(|c| {
                                let f = entity.working_fiber(c);
                                if f.is_empty() {
                                    return false;
                                }
                                n_closing += 1;
                                for (_, st) in &f {
                                    match st {
                                        mycelium::sdk::Standing::Stood => n_stood += 1,
                                        mycelium::sdk::Standing::Held => n_held += 1,
                                        mycelium::sdk::Standing::Ruled => {}
                                    }
                                }
                                true
                            })
                            .cloned()
                            .collect();
                        if closing.is_empty() {
                            entity.hold_thought(cluster.chain.clone());
                            n_silent += 1;
                            continue;
                        }
                        // MULTIPLICITY: several charts close at once. No
                        // best angle — the glue between them is a new
                        // coordinate, opened as an axis and spoken up.
                        if closing.len() >= 2 {
                            if let Some(a) = entity.open_axis(&cluster.address, &cluster.chain, &closing) {
                                eprintln!(
                                    "[{tag}] axis opened on {:02x}{:02x}…: {} charts closed at once, glue spoken as {:02x}{:02x}…",
                                    cluster.address[0],
                                    cluster.address[1],
                                    closing.len(),
                                    a[0],
                                    a[1]
                                );
                            }
                        }
                    }
                    // THINK, then CONVERSE. The hypha's quote already skipped
                    // listings this voice has spoken on (keyed receipts).
                    // The page is new work; do not mask it locally.
                    let t0 = std::time::Instant::now();
                    let (cites, residual, familiar) = entity.conclude(&cluster.chain);
                    eprintln!(
                        "[{tag}] [{}] cluster {} cells: concluded in {} ms -> {} cites, {} residual{}",
                        tuniversal.stamp(),
                        cluster.chain.len(),
                        t0.elapsed().as_millis(),
                        cites.len(),
                        residual.len(),
                        if familiar { " (familiar)" } else { "" },
                    );
                    // Two utterances at most: the sentence settles the
                    // listing; a residual not yet claimed is attested once
                    // as this voice's irreducible claim. Puncture cites
                    // (u32::MAX sections) are speech, not silence.
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
                                    let _ = entity.puncture_verified(&cites);
                                }
                                mycelium::anastome::dialogue::PROBE_DESCENT => {
                                    witnessed += 1;
                                    n_descent += 1;
                                    physics.bump(&mag);
                                    let _ = entity.puncture_verified(&cites);
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
                                let _ = entity.puncture_verified(&cites);
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
                    "[{tag}] [{}] quote closing {n_closing} charts stood {n_stood} held {n_held}",
                    tuniversal.stamp(),
                );
                eprintln!(
                    "[{tag}] [{}] quote {} clusters: closed {n_closed} descent {n_descent} nothing {n_nothing} settled {n_settled} silent {n_silent} attested {n_attested} err {n_err}",
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
                eprintln!("[{tag}] [{}] quote err: {e}", tuniversal.stamp());
                physics.spend([true; 4]); // an unreachable judge earns a full dwell
            }
        }

        lap("quote");
        // Syllabus after quote so a voice can close dialect/idea charts
        // before its first SYLLABUS line. Under the Budget, not a timer.
        if !occluded
            && !physics.depleted()
            && (entity.epigenome.genome_len() == 0 || entity.epigenome.atlas_is_empty())
        {
            if let Ok(n) = entity.pull_syllabus() {
                for a in entity.take_promoted() {
                    eprintln!("[{tag}] promoted {:02x}{:02x}…", a[0], a[1]);
                }
                if n > 0 {
                    learned += n as u64;
                    eprintln!(
                        "[{tag}] SYLLABUS +{n} ratified words (genome {})",
                        entity.epigenome.genome_len(),
                    );
                }
            }
        }
        // ── PULL REGISTERED WORK (PoUW): one signed envelope per stroke from
        // the hypha's authoritative registry — study the family, think the
        // tests through, submit. The REGISTRY rules bounty vs floor
        // (anti-replay yield collapse); unique solves count ONLY on BOUNTY.
        if let Ok(Some((tid, train, tests))) = entity.pull_task() {
            // The family is comprehended as a connection in the entity's own
            // algebra; every test is spoken through it, or the family's
            // residual is the demand it raises.
            let comprehended = mycelium::transducer::family::comprehend(&train);
            let mag = family_magnitude(&train);
            physics.bump(&mag);
            physics.spend(Physics::lanes_of(&mag)); // thought is priced on the lanes it occupies
            let gen = entity.epigenome.atlas_root();
            let mut extract_budget = physics.stroke();
            let extracted = mycelium::transducer::family::extract_rule_cached(
                &mut extract_memo,
                &tid,
                &train,
                gen,
                &mut extract_budget,
                &entity.epigenome,
            );
            let rule = extracted.as_ref().or(comprehended.rule.as_ref());
            eprintln!(
                "[{tag}] [{}] family {:02x}{:02x}… {} pairs, {} tests: {}",
                tuniversal.stamp(),
                tid[0],
                tid[1],
                train.len(),
                tests.len(),
                match (rule, &comprehended.transport, comprehended.framed) {
                    (Some(r), _, _) => format!(
                        "rule {} germs, {} faces, sections {}",
                        r.nerve.germ_count(),
                        r.faces.len(),
                        r.section_count()
                    ),
                    (None, Some(t), _) => format!("comprehended ({} rules, {} created, {} removed)", t.rules.len(), t.created.len(), t.removed.len()),
                    (None, None, true) => format!("residual {} cells", comprehended.residual.len()),
                    (None, None, false) => "frames disagree".to_string(),
                },
            );
            if let Some(rule) = rule {
                rules_extracted += 1;
                let _ = speak_sentence(&mut entity, &tag, &rule.nerve, &tid);
                let mut reached = true;
                let mut outs: Vec<Option<Vec<Vec<u8>>>> = Vec::new();
                for x in &tests {
                    match mycelium::transducer::family::speak_rule(rule, x, &entity.epigenome) {
                        Some(s) if s.remainder.is_empty() => {
                            tests_full += 1;
                            outs.push(Some(s.grid()));
                        }
                        Some(s) => {
                            reached = false;
                            entity.hold_thought(s.remainder.iter().map(|&(r, c, h)| (r, c, h, h)).collect());
                            outs.push(None);
                        }
                        None => {
                            reached = false;
                            rules_held_class += 1;
                            outs.push(None);
                        }
                    }
                }
                if reached && !tests.is_empty() && outs.iter().all(|o| o.is_some()) {
                    let outs: Vec<Vec<Vec<u8>>> = outs.into_iter().flatten().collect();
                    let spoken = mycelium::transducer::family::mint_sentence(rule, &train);
                    if let Ok((code, _mu)) = entity.submit_solve_rule(&tid, &outs, spoken.as_ref()) {
                        projected += 1;
                        match code {
                            mycelium::anastome::dialogue::SOLVE_BOUNTY => {
                                proj_solved += 1;
                                uniq_solved += 1;
                                solves_paid += 1;
                            }
                            mycelium::anastome::dialogue::SOLVE_FLOOR => proj_solved += 1,
                            _ => {}
                        }
                    }
                }
            } else {
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
                    eprintln!("[{tag}] thought held: family {:02x}{:02x}… {held} pairs uncollapsed — not uttered", tid[0], tid[1]);
                }
            }
        }

        // ── SYNTROPIC PHASE — crystallize, voice change, regenerate.
        if physics.depleted() {
            lap("task");
            let shed = entity.shed();
            if shed > 0 {
                eprintln!("[{tag}] shed {shed} dialect");
            }
            entity.discover();
            // Syllabus and sheaf sync live on the depletion dwell so the
            // door serves quotes and percepts on the hot stroke.
            {
                // THE PIVOT: expand dirty ideas once this phase, then pivot
                // new words on their charts only.
                entity.expand_ideas();
                let pivoted = entity.pivot_thoughts();
                if pivoted > 0 {
                    eprintln!("[{tag}] pivot: {pivoted} held thoughts spoken as demands on charts the language now holds");
                }
                if let Ok(n) = entity.pull_syllabus() {
                    for a in entity.take_promoted() {
                        eprintln!("[{tag}] promoted {:02x}{:02x}…", a[0], a[1]);
                    }
                    if n > 0 {
                        learned += n as u64;
                        eprintln!(
                            "[{tag}] SYLLABUS +{n} ratified words (genome {})",
                            entity.epigenome.genome_len(),
                        );
                    }
                }
            }
            // THE SHEAF SYNC (Q3): converge the genome on the hypha's
            // language by discordant stratum — the entity thinks in the
            // whole language, not the syllabus drip.
            if let Ok(n) = entity.sync_genome() {
                if n > 0 {
                    learned += n as u64;
                    eprintln!(
                        "[{tag}] SYNC +{n} words by stratum (genome {})",
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
                            "[{tag}] REFINED {:02x}{:02x}… (vocabulary compressed, genome {}) μ⃗ {mu:?}",
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
            eprintln!("[{tag}] [{}] stroke laps ms {laps:?}", tuniversal.stamp());
            if entity.epigenome.genome_len() != last_genome {
                let _ = entity.save_genome(&genome_path);
                saved_genome = entity.epigenome.genome_len();
                last_genome_save = Instant::now();
                last_genome = entity.epigenome.genome_len();
            }
            last_settles = settles;
            eprintln!(
                "[{tag}] [{}] K {k_step} | genome {} | dialect {} | ideas {} | shed {} | fed {fed} derived {derived} witnessed {witnessed} settles {settles} err {errors} Z {} | learned {learned} discordance {} axes {}",
                tuniversal.stamp(),
                entity.epigenome.genome_len(),
                entity.dialect_len(),
                entity.ideas_len(),
                entity.shed_n(),
                entity.impedance(),
                entity.discordance(),
                entity.axes_opened(),
            );
            while physics.depleted() {
                physics.regenerate();
                physics.relax();
                // Impedance routes; it does not sleep. Dwell is at most one
                // aperture_cycle of measured transport. High Z drops that
                // hypha from open_rooms (route_chart) and the next stroke
                // speaks to the next hypha.
                let cycle = mycelium::physics::aperture_cycle() as u64;
                let strokes = (judge_load as u64).max(1).min(cycle);
                std::thread::sleep(mycelium::physics::from_transport(entity.transport_rtt(), strokes));
            }
        }
    }
}
