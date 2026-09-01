// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! mycelium-client — THE WORKER ENTITY (the flagship end-user kernel). It owns
//! no corpus: work arrives from the hypha's open-demand market. The loop is the
//! ANASTOME conversation itself:
//!
//!   quote  → "I need something to solve" — pull the highest-yield pending
//!            demands (content-addressed chains, priced per lane)
//!   think  → INTERNAL reduction of each pulled chain over the entity's own
//!            genome (the C-stack — comprehension, never a byte-store)
//!   attest → speak the honest finding: a derived sentence (closure work) or
//!            an independent irreducibility witness (coverage work) — both are
//!            true speech; the hypha replays either against its projective
//!            reduction, so false speech never installs
//!   learn  → comprehend whatever the mesh teaches back; the genome grows by
//!            inheritance and CRYSTALLIZES to disk (continuity); the epigenome
//!            (expression, recollections, aperture) stays in memory by ruling
//!
//! Cadence is the entity's own two-stroke metabolism: pulling and attesting is
//! the entropic stroke (spends per-lane reserve); depletion flips to the
//! syntropic phase (persist genome, voice the tally on change, regenerate,
//! dwelling in proportion to the judge's load). No counters, no timers.
//!
//! Usage: mycelium-client [ADDR] --kernel NAME [--state DIR]
use mycelium::physics::{Physics, RESERVE_MAX};
use mycelium::sdk::{Comprehension, Entity, Kernel};
use std::time::Instant;

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].as_str())
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
    let genome_path = format!("{state_dir}/{name}.genome");
    let _ = std::fs::create_dir_all(&state_dir);

    let mut entity = Entity::open_persistent(&addr, Kernel::named(&name), &genome_path);
    eprintln!(
        "mycelium-client '{name}' → {addr}; genome {} words (crystallized at {genome_path})",
        entity.epigenome.genome_len()
    );

    let mut physics = Physics::default();
    let (mut derived, mut witnessed, mut settles, mut errors) = (0u64, 0u64, 0u64, 0u64);
    let (mut last_genome, mut last_settles) = (usize::MAX, u64::MAX);
    let start = Instant::now();

    loop {
        // ENTROPIC STROKE — pull work off the market and think it through.
        let mut judge_load = 0u8;
        match entity.quote() {
            Ok(q) => {
                for cluster in &q.clusters {
                    if cluster.chain.is_empty() {
                        continue;
                    }
                    // THINK: reduce the pulled chain over the OWN genome. The
                    // outcome decides what is honestly speakable — a sentence
                    // (this entity re-derived the reduction independently) or
                    // an irreducibility witness (its genome cannot say it
                    // either: genuine coverage demand). Both attest truly.
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
                if q.clusters.iter().all(|c| c.chain.is_empty()) || q.clusters.is_empty() {
                    // MARKET CLEAR — nothing to digest is itself the idle
                    // signal: spend the stroke so the syntropic phase (and its
                    // dwell) paces the next ask off the metabolism contract.
                    physics.spend([true; 4]);
                }
            }
            Err(_) => {
                errors += 1;
                physics.spend([true; 4]); // an unreachable judge earns a full dwell
            }
        }

        // SYNTROPIC PHASE — persist what crystallized, voice change, regenerate.
        if physics.depleted() {
            if entity.epigenome.genome_len() != last_genome || settles != last_settles {
                if entity.epigenome.genome_len() != last_genome {
                    let _ = entity.save_genome(&genome_path);
                }
                last_genome = entity.epigenome.genome_len();
                last_settles = settles;
                eprintln!(
                    "[{name}] {:.0}s | derived {derived} witnessed {witnessed} settles {settles} err {errors} | genome {} (peak expr {})",
                    start.elapsed().as_secs_f64(),
                    entity.epigenome.genome_len(),
                    entity.epigenome.peak_expression(),
                );
            }
            while physics.depleted() {
                physics.regenerate();
                physics.relax();
                // Dwell ∝ the judge's load; RESERVE_MAX ms is the idle floor —
                // a constant from the metabolism contract, not a new knob.
                let dwell = (judge_load as u64).max(RESERVE_MAX as u64);
                std::thread::sleep(std::time::Duration::from_millis(dwell));
            }
        }
    }
}
