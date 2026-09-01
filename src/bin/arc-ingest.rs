// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! arc-ingest — a KERNEL that feeds the real ARC-AGI corpus onto the mesh via the
//! SDK and does NO cognition of its own: it presents each train pair (input→output)
//! as an A→B demand and records the hypha's verdict. The engine perceives,
//! germinates, and composes; this only turns the pages and tallies the live soak.
//! Usage:
//!   arc-ingest [ADDR] --kernel NAME --dir CORPUS_DIR [--loop] [--limit N] [--every N]
use mycelium::arc::{grid_to_cells, parse_task};
use mycelium::physics::Physics;
use mycelium::sdk::{Comprehension, Entity, Kernel, Verdict};
use std::path::PathBuf;
use std::time::Instant;

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].as_str())
}
fn has(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

#[derive(Default)]
struct Tally {
    pairs: u64,
    decomposed: u64,
    germ_pending: u64,
    installed: u64,
    fragmented: u64,
    pivot: u64,
    unknown: u64,
    errors: u64,
    taught_bytes: u64,
    settles: u64,
    /// C-stack outcomes: teachings COMPREHENDED (decomposed over the genome)
    /// vs heard as novel (residual toward the mesh's next work).
    comprehended: u64,
    heard_novel: u64,
}

impl Tally {
    fn record(&mut self, v: &Verdict) {
        self.pairs += 1;
        self.taught_bytes += v.taught().len() as u64;
        match v {
            Verdict::Decomposed { .. } => self.decomposed += 1,
            Verdict::GermPending => self.germ_pending += 1,
            Verdict::Installed { .. } => self.installed += 1,
            Verdict::Fragmented => self.fragmented += 1,
            Verdict::Pivot => self.pivot += 1,
            Verdict::Unknown(_) => self.unknown += 1,
        }
    }
    fn report(&self, kernel: &str, secs: f64) {
        let p = self.pairs.max(1) as f64;
        let pct = |n: u64| 100.0 * n as f64 / p;
        eprintln!(
            "[{kernel}] {} pairs in {secs:.0}s ({:.1}/s) | decomposed {} ({:.0}%) germ {} installed {} pivot {} frag {} | settles {} | err {} | taught {}KB",
            self.pairs,
            self.pairs as f64 / secs.max(1.0),
            self.decomposed, pct(self.decomposed),
            self.germ_pending, self.installed, self.pivot, self.fragmented,
            self.settles,
            self.errors, self.taught_bytes / 1024,
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let addr = args.get(1).filter(|a| !a.starts_with("--")).cloned().unwrap_or_else(|| "127.0.0.1:7500".to_string());
    let name = flag(&args, "--kernel").unwrap_or("arc").to_string();
    let dir = flag(&args, "--dir").unwrap_or("data/arc-agi-2/training").to_string();
    let looping = has(&args, "--loop");
    let limit: usize = flag(&args, "--limit").and_then(|s| s.parse().ok()).unwrap_or(usize::MAX);

    // P8 — THE CONDUCTOR-FOLLOWING FEEDER: with --conductor <board.json>, the
    // netstratum world-model's board schedules the ingest — the feed rotates
    // to the conductor's arc_next cursor, re-read every cycle, so a LIVE
    // conductor daemon steers the corpus order and a resting one leaves its
    // last cosmological schedule in force. The universe conducts; we feed.
    let conductor = flag(&args, "--conductor").map(String::from);
    let conductor_cursor = |path: &str| -> usize {
        let Ok(text) = std::fs::read_to_string(path) else { return 0 };
        text.split("\"arc_next\":")
            .nth(1)
            .and_then(|t| {
                t.chars()
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse()
                    .ok()
            })
            .unwrap_or(0)
    };
    let mut tasks: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| {
            eprintln!("arc-ingest: cannot read {dir}: {e}");
            std::process::exit(1);
        })
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    tasks.sort();
    tasks.truncate(limit);
    if let Some(board) = &conductor {
        let cursor = conductor_cursor(board) % tasks.len().max(1);
        tasks.rotate_left(cursor);
        eprintln!("arc-ingest: conducted by {board} — cursor {cursor}/{}", tasks.len());
    }

    let genome_path = format!("state/{name}.genome");
    if let Some(dir) = std::path::Path::new(&genome_path).parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let mut entity = Entity::open_persistent(&addr, Kernel::named(&name), &genome_path);
    eprintln!(
        "arc-ingest: kernel '{name}' → {addr}; {} tasks from {dir}{}",
        tasks.len(),
        if looping { " (looping)" } else { "" }
    );

    let mut tally = Tally::default();
    // THE ENTITY'S OWN TWO-STROKE — ingestion is ENTROPIC (each submission
    // spends the per-lane reserve on the lanes the judge's reply pressured);
    // depletion flips the entity into its SYNTROPIC phase, where it stops
    // ingesting and does the reflective work — tally, epigenome digest, the
    // market quote — while the reserve regenerates. Cadence is a phase
    // alignment with the judge's metabolism, never a counter or a timer.
    let mut physics = Physics::default();
    // Voiced-on-change state: the syntropic work runs EVERY stroke; the log
    // speaks only when the stroke found something different from the last.
    let (mut last_settles, mut last_genome, mut last_top) = (u64::MAX, usize::MAX, [0u8; 2]);
    let start = Instant::now();
    loop {
        // A LIVE conductor re-steers each cycle: re-read the board's cursor
        // and rotate the corpus to its schedule before the pass.
        let cycle_tasks: Vec<PathBuf> = match &conductor {
            Some(board) => {
                let cursor = conductor_cursor(board) % tasks.len().max(1);
                let mut t = tasks.clone();
                t.rotate_left(cursor);
                t
            }
            None => tasks.clone(),
        };
        for path in &cycle_tasks {
            let Ok(text) = std::fs::read_to_string(path) else {
                continue;
            };
            let Some(task) = parse_task(&text) else { continue };
            for (input, output) in &task.train {
                let (a, b) = (grid_to_cells(input), grid_to_cells(output));
                if a.is_empty() || b.is_empty() {
                    continue; // an all-background side carries no percept
                }
                let mut judge_load = 0u8;
                match entity.submit(&a, &b) {
                    Ok((reply, heard)) => {
                        tally.record(&reply.verdict);
                        if reply.settle.is_some() {
                            tally.settles += 1; // a consilience receipt rode this reply
                        }
                        match heard {
                            Some(Comprehension::Understood(_)) => tally.comprehended += 1,
                            Some(Comprehension::Novel { .. }) => tally.heard_novel += 1,
                            None => {}
                        }
                        // ENTROPIC STROKE: this ingestion spent reserve on the
                        // lanes the judge's reply reported pressured (a percept
                        // always exercises at least the residual lane).
                        let mut lanes: [bool; 4] = std::array::from_fn(|l| reply.metabolism[l] > 0);
                        if lanes.iter().all(|&x| !x) {
                            lanes[2] = true;
                        }
                        physics.bump(lanes);
                        physics.spend(lanes);
                        judge_load = reply.load();
                    }
                    Err(_) => {
                        tally.errors += 1;
                        tally.pairs += 1;
                    }
                }
                // PHASE ALIGNMENT: depletion flips the entity into its
                // SYNTROPIC phase — the reflective work runs exactly once per
                // stroke, then the reserve regenerates, each tick dwelling in
                // proportion to the judge's own load (a strained judge earns a
                // longer quiet; an idle one, an immediate return to intake).
                if physics.depleted() {
                    let grew = tally.settles != last_settles || entity.epigenome.genome_len() != last_genome;
                    if grew {
                        last_settles = tally.settles;
                        if entity.epigenome.genome_len() != last_genome {
                            let _ = entity.save_genome(&genome_path); // crystallize continuity
                        }
                        last_genome = entity.epigenome.genome_len();
                        tally.report(&name, start.elapsed().as_secs_f64());
                        eprintln!(
                            "[{name}] entity: genome {} words (peak expression {}), heard {} understood / {} novel",
                            entity.epigenome.genome_len(),
                            entity.epigenome.peak_expression(),
                            tally.comprehended,
                            tally.heard_novel,
                        );
                    }
                    // SPEAK the quote — the market survey is syntropic work —
                    // and voice it when the offered work moved.
                    if let Ok(q) = entity.quote() {
                        match q.clusters.first() {
                            Some(top) if grew || [top.address[0], top.address[1]] != last_top => {
                                last_top = [top.address[0], top.address[1]];
                                eprintln!(
                                    "[{name}] quote: surface {:?}; top work {} cells @ μ⃗ {:?} ({:02x}{:02x}…)",
                                    q.surface,
                                    top.chain.len(),
                                    top.mu,
                                    top.address[0],
                                    top.address[1],
                                );
                            }
                            _ => {}
                        }
                    }
                    while physics.depleted() {
                        physics.regenerate();
                        physics.relax();
                        if judge_load > 0 {
                            std::thread::sleep(std::time::Duration::from_millis(judge_load as u64));
                        }
                    }
                }
            }
        }
        if !looping {
            break;
        }
    }
    tally.report(&name, start.elapsed().as_secs_f64());
}
