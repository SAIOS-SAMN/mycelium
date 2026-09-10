// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 24.3 — THE VOICE, a client on the mesh: the arguments it is started
//! with, the order it comes up in, and the skeleton of the stroke loop that
//! runs its four arms.

mod arms;
mod feed;
mod speak;

pub(crate) use crate::feed::{Feed};
use crate::arms::{feed::feed_arm, quote::quote_arm, syntropy::syntropy_arm, task::task_arm};
use crate::feed::{pending_feed};
use mycelium::physics::{Physics, StrokeOutcome};
use mycelium::court::identity::HyphaKey;
use mycelium::network::mesh;
use mycelium::sdk::{Entity, Kernel};
use mycelium::voice::Voice;
use std::path::{PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// WHAT THE STROKE PACES ON — the readings a stroke carries forward to the
/// next one. The quote asks when the language moved or the book rose or the
/// dwell elapsed; the dwell falls from the last reply's pressure; K is the
/// court's own index. One carrier, because they are one state: the arms
/// took them as seven `&mut` parameters and each needed
/// `#[allow(clippy::too_many_arguments)]` to say so.
#[derive(Default)]
pub(crate) struct Metabolism {
    pub(crate) last_lang: (usize, usize, usize),
    pub(crate) last_height: u64,
    pub(crate) last_quoted_height: u64,
    pub(crate) last_pressure: [u64; 4],
    pub(crate) dwell_remaining: [u64; 4],
    pub(crate) k_step: u64,
    pub(crate) last_genome: usize,
    /// The language and the book height the syntropy arm last synced at
    /// (24.3, I37): the sheaf sync, the syllabus pull and peer discovery
    /// are asked when either moved or a lane's dwell elapsed, exactly as
    /// the quote is, never on every depleted stroke.
    pub(crate) last_synced_lang: (usize, usize, usize),
    pub(crate) last_synced_height: u64,
}

/// WHAT ONE STROKE WORKS ON: this voice's entity and the metabolism it
/// breathes on, the counters the stroke line reads, the outcome being
/// accumulated, and the tag it logs under. Every arm of a stroke takes all
/// of it — there is no arm that reads the entity and not the physics.
pub(crate) struct Stroke<'a> {
    pub(crate) entity: &'a mut Entity,
    pub(crate) physics: &'a mut Physics,
    pub(crate) c: &'a mut Counts,
    pub(crate) outcome: &'a mut StrokeOutcome,
    pub(crate) m: &'a mut Metabolism,
    pub(crate) tag: &'a str,
}

/// What this voice has done since it came up. Every field is read by the
/// stroke line; an accumulator nobody reads is dead.
#[derive(Default)]
pub(crate) struct Counts {
    pub(crate) fed: u64,
    pub(crate) derived: u64,
    pub(crate) witnessed: u64,
    pub(crate) settles: u64,
    pub(crate) errors: u64,
    pub(crate) projected: u64,
    pub(crate) proj_solved: u64,
    pub(crate) uniq_solved: u64,
    pub(crate) refined: u64,
    pub(crate) learned: u64,
    pub(crate) solves_paid: u64,
}

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
    // The fleet's homes are `var/<unit>` under the repository the binary
    // runs from, never the operator's home directory (moved 2026-09-09).
    Ok(PathBuf::from("var").join("voice"))
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
    let height = mycelium::chronometry::Height::default();

    let mut feed = Feed::new(home.clone(), conductor);
    let mut entity = Entity::open_persistent(
        &bootstrap,
        Kernel::from_voice(voice.identity()),
        &genome_path,
        &thoughts_path,
    );
    entity.set_height(height);
    entity.discover();
    let pending = pending_feed(&home);
    eprintln!(
        "[{tag}] mycelium-client → {bootstrap}; genome {} words{}",
        entity.epigenome.genome_len(),
        if pending.is_empty() {
            String::new()
        } else {
            format!("; feeding {} from {}/feed", pending.len(), home.display())
        },
    );

    let mut physics = Physics::default();
    let mut c = Counts::default();
    let mut m = Metabolism { last_genome: usize::MAX, ..Metabolism::default() };
    let stop = Arc::new(AtomicBool::new(false));
    {
        let stop = Arc::clone(&stop);
        let _ = ctrlc::set_handler(move || stop.store(true, Ordering::SeqCst));
    }
    let mut extract_memo: mycelium::network::nerve::Keyed<(
        mycelium::transducer::address::Address,
        Option<mycelium::transducer::sentence::SentenceRule>,
    )> = mycelium::network::nerve::Keyed::new();
    m.last_lang = (usize::MAX, usize::MAX, usize::MAX);
    m.last_synced_lang = (usize::MAX, usize::MAX, usize::MAX);
    let mut stroke_n = 0u64;
    let mut last_logged_n = 0u64;
    let mut last_logged_k = u64::MAX;

    loop {
        if stop.load(Ordering::SeqCst) {
            let _ = entity.save_genome(&genome_path);
            let _ = entity.save_thoughts(&thoughts_path);
            break;
        }
        stroke_n = stroke_n.saturating_add(1);
        for l in 0..4 {
            m.dwell_remaining[l] = m.dwell_remaining[l].saturating_sub(1);
        }
        let mut outcome = StrokeOutcome::default();
        let occluded =
            matches!(entity.transport(), mycelium::sdk::TransportAperture::Occluded { .. });

        {
            let mut stroke = Stroke {
                entity: &mut entity,
                physics: &mut physics,
                c: &mut c,
                outcome: &mut outcome,
                m: &mut m,
                tag: &tag,
            };
            feed_arm(&mut stroke, &mut feed, &mut extract_memo, occluded);
            quote_arm(&mut stroke, occluded);
            task_arm(&mut stroke, &mut extract_memo);
            // The syntropic phase on any lane short of one expansion
            // (S10's reading, S13.2).
            if !mycelium::economy::lanes_short(&stroke.physics.reserve_shortfall([true; 4])).is_empty() {
                syntropy_arm(&mut stroke, &genome_path);
                stroke.physics.relax();
            }
        }
        // 24.3: a working stroke spends the lanes its outcome moved; a stroke
        // that moved no lane spends the tick on all four — the tick, not work.
        let moved = outcome.moved();
        if moved == [false; 4] {
            physics.spend([true; 4]);
        } else {
            physics.spend(moved);
        }
        let Counts {
            fed,
            derived,
            witnessed,
            settles,
            errors,
            projected,
            proj_solved,
            uniq_solved,
            refined,
            learned,
            solves_paid,
        } = c;
        let Metabolism { dwell_remaining, k_step, .. } = m;
        let cycle = mycelium::physics::aperture_cycle() as u64;
        if last_logged_k == u64::MAX || k_step != last_logged_k || stroke_n.saturating_sub(last_logged_n) >= cycle {
            let (atlas_ruled, atlas_stood, atlas_held) = entity.atlas_standing_counts();
            if moved == [false; 4] {
                eprintln!(
                    "[{tag}] [{}] stroke idle: dwell {dwell_remaining:?} K {k_step} | genome {} | dialect {} | ideas {} | atlas words ruled {atlas_ruled} stood {atlas_stood} held {atlas_held} | feed {}",
                    mycelium::chronometry::Height::stamp_of(entity.now_tu()),
                    entity.epigenome.genome_len(),
                    entity.dialect_len(),
                    entity.ideas_len(),
                    if feed.exhausted() { "clear" } else { "held" },
                );
            } else {
                eprintln!(
                    "[{tag}] [{}] K {k_step} | genome {} | dialect {} | ideas {} | atlas words ruled {atlas_ruled} stood {atlas_stood} held {atlas_held} | shed {} | fed {fed} derived {derived} witnessed {witnessed} settles {settles} err {errors} Z {} | learned {learned} projected {projected} solved {proj_solved} uniq {uniq_solved} paid {solves_paid} refined {refined} discordance {} boundary {:?} axes {}",
                    mycelium::chronometry::Height::stamp_of(entity.now_tu()),
                    entity.epigenome.genome_len(),
                    entity.dialect_len(),
                    entity.ideas_len(),
                    entity.shed_n(),
                    entity.impedance(),
                    match entity.discordance() {
                        Some(strata) => strata.to_string(),
                        None => "unmeasured".to_string(),
                    },
                    entity.boundary_measure(),
                    entity.axes_opened(),
                );
            }
            eprintln!(
                "[{tag}] stroke outcome fed {:?} pivoted {:?} stood {:?} uttered {:?} closed {:?} learned {:?}",
                outcome.fed, outcome.pivoted, outcome.stood, outcome.uttered, outcome.closed, outcome.learned
            );
            last_logged_k = k_step;
            last_logged_n = stroke_n;
        }
        // The lanes the next stroke needs to expand on and cannot: one
        // reading per lane (S10, S13.2), never a bit recomputed per lane.
        let short = physics.reserve_shortfall(moved);
        let wait: [bool; 4] = std::array::from_fn(|l| short[l] > 0);
        m.dwell_remaining = physics.dwell(&m.last_pressure, wait);
    }
}
