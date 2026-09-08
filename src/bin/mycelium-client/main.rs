// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE VOICE — a client on the mesh. This file is the boot order and the
//! stroke loop's skeleton and nothing else (MU-STANDARDS 1.3): the four
//! arms of a stroke are `arms/{feed, quote, task, syntropy}.rs`, the corpus
//! feed is `feed.rs`, and what a voice speaks is `speak.rs`.

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
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    Ok(PathBuf::from(home).join(".mycelium"))
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
        mycelium::transducer::a21::Address,
        Option<mycelium::transducer::sentence::SentenceRule>,
    )> = mycelium::network::nerve::Keyed::new();
    m.last_lang = (usize::MAX, usize::MAX, usize::MAX);
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

        feed_arm(
            &mut entity, &mut physics, &mut feed, &mut extract_memo, &mut c, &mut outcome,
            &mut m, occluded, &tag,
        );
        quote_arm(
            &mut entity, &mut physics, &mut c, &mut outcome, &mut m, occluded, &tag,
        );
        task_arm(&mut entity, &mut physics, &mut extract_memo, &mut c, &mut outcome, &tag);
        if physics.depleted() {
            syntropy_arm(
                &mut entity, &mut physics, &mut c, &mut outcome, &genome_path, &mut m, &tag,
            );
            physics.relax();
        }
        if outcome.dormant() {
            physics.spend([true; 4]); // the tick, not work
        } else {
            physics.spend(Physics::lanes_of(&outcome.total()));
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
            if outcome.dormant() {
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
                    "[{tag}] [{}] K {k_step} | genome {} | dialect {} | ideas {} | atlas words ruled {atlas_ruled} stood {atlas_stood} held {atlas_held} | shed {} | fed {fed} derived {derived} witnessed {witnessed} settles {settles} err {errors} Z {} | learned {learned} projected {projected} solved {proj_solved} uniq {uniq_solved} paid {solves_paid} refined {refined} discordance {} axes {}",
                    mycelium::chronometry::Height::stamp_of(entity.now_tu()),
                    entity.epigenome.genome_len(),
                    entity.dialect_len(),
                    entity.ideas_len(),
                    entity.shed_n(),
                    entity.impedance(),
                    entity.discordance(),
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
        let want = Physics::lanes_of(&outcome.total());
        let wait = std::array::from_fn(|l| want[l] && !physics.can_expand(std::array::from_fn(|i| i == l)));
        m.dwell_remaining = physics.dwell(&m.last_pressure, wait);
    }
}
