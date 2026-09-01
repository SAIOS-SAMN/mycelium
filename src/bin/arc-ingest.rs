// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! arc-ingest — a KERNEL that feeds the real ARC-AGI corpus onto the mesh via the
//! SDK and does NO cognition of its own: it presents each train pair (input→output)
//! as an A→B demand and records the hypha's verdict. The engine perceives,
//! germinates, and composes; this only turns the pages and tallies the live soak.
//! Usage:
//!   arc-ingest [ADDR] --kernel NAME --dir CORPUS_DIR [--loop] [--limit N] [--every N]
use mycelium::arc::{grid_to_cells, parse_task};
use mycelium::sdk::{Comprehension, Entity, Kernel, Verdict};
use std::path::PathBuf;
use std::time::Instant;

/// Below this per-lane pressure the mesh is calm — full-rate intake. Matches the
/// hypha's reduce budget knee (calm ≤ 7).
const BACKPRESSURE_KNEE: u8 = 7;
/// Milliseconds of intake wait per unit of pressure above the knee (polytonic
/// proportional backoff — resource stabilization, not a fixed poll).
const BACKPRESSURE_MS: u64 = 4;

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
    let every: u64 = flag(&args, "--every").and_then(|s| s.parse().ok()).unwrap_or(500);

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

    let mut entity = Entity::open(&addr, Kernel::named(&name));
    eprintln!(
        "arc-ingest: kernel '{name}' → {addr}; {} tasks from {dir}{}",
        tasks.len(),
        if looping { " (looping)" } else { "" }
    );

    let mut tally = Tally::default();
    let start = Instant::now();
    loop {
        for path in &tasks {
            let Ok(text) = std::fs::read_to_string(path) else {
                continue;
            };
            let Some(task) = parse_task(&text) else { continue };
            for (input, output) in &task.train {
                let (a, b) = (grid_to_cells(input), grid_to_cells(output));
                if a.is_empty() || b.is_empty() {
                    continue; // an all-background side carries no percept
                }
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
                        // METABOLIC BACKPRESSURE — pace intake by the mesh's per-lane
                        // pressure (Liebig: the bottleneck lane governs). When the
                        // mesh is loaded the kernel waits, letting it metabolize the
                        // backlog into order instead of being brute-forced.
                        let load = reply.load();
                        if load > BACKPRESSURE_KNEE {
                            let excess = (load - BACKPRESSURE_KNEE) as u64;
                            std::thread::sleep(std::time::Duration::from_millis(excess * BACKPRESSURE_MS));
                        }
                    }
                    Err(_) => {
                        tally.errors += 1;
                        tally.pairs += 1;
                    }
                }
                if tally.pairs % every == 0 {
                    tally.report(&name, start.elapsed().as_secs_f64());
                    eprintln!(
                        "[{name}] entity: genome {} words (peak expression {}), heard {} understood / {} novel",
                        entity.epigenome.genome_len(),
                        entity.epigenome.peak_expression(),
                        tally.comprehended,
                        tally.heard_novel,
                    );
                    // SPEAK the quote — "what is the highest-yield work?" — and log
                    // the surface + top offered cluster (the conversation, audible).
                    if let Ok(q) = entity.quote() {
                        match q.clusters.first() {
                            Some(top) => eprintln!(
                                "[{name}] quote: surface {:?}; top work {} cells @ μ⃗ {:?} ({:02x}{:02x}…)",
                                q.surface,
                                top.chain.len(),
                                top.mu,
                                top.address[0],
                                top.address[1],
                            ),
                            None => eprintln!("[{name}] quote: surface {:?}; market clear", q.surface),
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
