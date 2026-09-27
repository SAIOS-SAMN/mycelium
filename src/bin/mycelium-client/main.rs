// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 24.3 — THE VOICE, a client on the mesh: the arguments it is started
//! with, the order it comes up in, and the skeleton of the stroke loop that
//! runs its four arms.

#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

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
    /// THE BOOK'S K, NOT THIS MIND'S (45.4). It is `YieldQuote::k` — "the
    /// judge's k-step at the quote, the temporal coordinate of the book
    /// this quote reads" — taken off a hypha's reply and used to pace the
    /// stroke line.
    ///
    /// It was named `k_step` and printed as `K`, so the stroke line read a
    /// COURT-SIDE coordinate under the mind's own label, beside
    /// `Height::stamp_of(now_tu())` — which is the mesh height, the other
    /// court-side clock. Four voices printed `[h 16504] K 16504` and the
    /// fifth `[h 16502] K 2587`, and the pair read as one mind racing and
    /// four in lockstep. They were two readings of the same organ's time,
    /// and the outlier was a voice whose quotes were not landing.
    ///
    /// A mind's own K is `Entity::own_k` — the orbits it acted in (78.49) —
    /// and it is what `shed_at` ages the dialect by. Both
    /// are now on the line, under their own names, so a per-voice rate has
    /// a denominator that belongs to the voice.
    pub(crate) book_k: u64,
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
    pub(crate) reanchored: u64,
    pub(crate) condensed: u64,
    pub(crate) learned: u64,
    pub(crate) solves_paid: u64,
}

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].as_str())
}

/// THE HEAP AS THE ALLOCATOR COUNTS IT (L33, §8.1): the bytes live in
/// allocations against the bytes resident in the process, read after
/// advancing the allocator's epoch so both are one reading. The two apart
/// tell retention from growth, which is what every counted carrier being
/// flat while RSS climbs leaves open. Unmeasured where the allocator does
/// not answer; nothing is put in place of a reading (rule 11).
fn heap() -> String {
    let read = || -> Option<(usize, usize)> {
        tikv_jemalloc_ctl::epoch::advance().ok()?;
        let allocated = tikv_jemalloc_ctl::stats::allocated::read().ok()?;
        let resident = tikv_jemalloc_ctl::stats::resident::read().ok()?;
        Some((allocated, resident))
    };
    match read() {
        Some((allocated, resident)) => format!("allocated {allocated} resident {resident}"),
        None => "unmeasured".to_string(),
    }
}

/// THE HEAP'S LIVE BYTES, one reading (R3, §8.1): taken before and after
/// each arm of the stroke so the signed difference, summed per arm since
/// the voice came up, says WHICH arm retains what the heap line says grows.
/// Unmeasured where the allocator does not answer; a difference is taken
/// only between two readings.
/// Z10 — THE PER-CARRIER VECTOR AS ONE FIELD: `name=keys+values/entries`
/// each, never summed across carriers.
///
/// Keys and values are apart because they answer different questions: a
/// carrier can grow in bytes without gaining an entry (a bigger form held)
/// or gain entries without bytes (short keys), and the genome grows only
/// in VALUES — sixty-five decoded forms in six minutes, which the
/// key-only instrument read as zero.
///
/// `account` is the reconciliation the row requires: the carriers' total
/// against what jemalloc says this process has allocated. A table that
/// cannot account for the process is not reading the process, and the
/// residue is printed rather than hidden — what the carriers do NOT reach
/// is as much a reading as what they do.
fn carrier_line(entity: &mycelium::client::sdk::Entity) -> String {
    let mut out = String::new();
    let mut total = 0usize;
    for (name, keys, values, n) in entity.carrier_bytes() {
        #[expect(clippy::disallowed_methods, reason = "rule 11 — a byte total of resident carriers: bytes this process holds cannot pass a 64-bit usize")]
        let next = total.saturating_add(keys).saturating_add(values);
        total = next;
        if keys == 0 && values == 0 && n == 0 {
            continue;
        }
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(&format!("{name}={keys}+{values}/{n}"));
    }
    // An allocator that does not read is an unread value, not a zero
    // (rule 11): the account says so rather than reconciling against a
    // number nothing measured.
    match allocated() {
        Some(live) => out.push_str(&format!(" | account carriers {total} live {live}")),
        None => out.push_str(&format!(" | account carriers {total} live unread")),
    }
    out
}

fn allocated() -> Option<i128> {
    tikv_jemalloc_ctl::epoch::advance().ok()?;
    Some(tikv_jemalloc_ctl::stats::allocated::read().ok()? as i128)
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
    // A home whose path is not UTF-8 names no key file: minting under the
    // bare name would put the voice's key in the working directory, which
    // is a different key from the one that home holds (rule 11).
    let Some(key_name) = key_path.to_str() else {
        eprintln!("mycelium-client: --home path is not UTF-8 and names no key file");
        std::process::exit(2);
    };
    let voice_key = HyphaKey::load_or_mint(key_name);
    let voice = Voice::from_identity(voice_key.vk_bytes());
    let tag = voice.name().to_string();
    let mesh_rec = match mesh::load_or_migrate_voice(&home, &voice_key) {
        Ok(rec) => rec,
        Err(why) => {
            eprintln!("[{tag}] boot refused: {why}");
            std::process::exit(2);
        }
    };
    let genome_path = home.join("genome").to_string_lossy().into_owned();
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
    let mut extract_memo = mycelium::transducer::family::ExtractMemo::new();
    m.last_lang = (usize::MAX, usize::MAX, usize::MAX);
    m.last_synced_lang = (usize::MAX, usize::MAX, usize::MAX);
    let mut stroke_n = 0u64;
    let mut last_logged_n = 0u64;
    let mut last_logged_k = u64::MAX;
    // What each arm — feed, quote, task, syntropy — has added to the live
    // heap since the voice came up, in bytes (R3, §8.1).
    let mut grew = [0i128; 4];

    loop {
        if stop.load(Ordering::SeqCst) {
            // LD0: a genome that was not saved is said, never discarded.
            if let Err(why) = entity.save_genome(&genome_path) {
                eprintln!("[{tag}] genome not saved: {why}");
            }
            break;
        }
        #[expect(clippy::disallowed_methods, reason = "rule 11 — one per stroke in one process: a u64 cannot pass 2⁶⁴ events before the process ends")]
        let next = stroke_n.saturating_add(1);
        stroke_n = next;
        for l in 0..4 {
            // A lane with no dwell left has none to spend: the reading of
            // absence, not a floor.
            if let Some(next) = m.dwell_remaining[l].checked_sub(1) {
                m.dwell_remaining[l] = next;
            }
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
            let a0 = allocated();
            feed_arm(&mut stroke, &mut feed, &mut extract_memo, occluded);
            let a1 = allocated();
            quote_arm(&mut stroke, occluded);
            let a2 = allocated();
            task_arm(&mut stroke, &mut extract_memo, occluded);
            let a3 = allocated();
            // The syntropic phase on any lane short of one expansion
            // (S10's reading, S13.2).
            if !mycelium::economy::lanes_short(&stroke.physics.reserve_shortfall([true; 4])).is_empty() {
                syntropy_arm(&mut stroke, &genome_path);
                stroke.physics.relax();
            }
            let a4 = allocated();
            for (g, (before, after)) in grew.iter_mut().zip([(a0, a1), (a1, a2), (a2, a3), (a3, a4)]) {
                if let (Some(b), Some(a)) = (before, after) {
                    *g += a - b;
                }
            }
            // EXPANSION 4 — A LEARNED WORD MEETS THE VORTICES THIS MIND
            // HOLDS, on the stroke it was learned. `pivot_thoughts` pivots
            // every genome word added since the last stroke on its own
            // charts, and `Contemplation::pivot_on` is where a held vortex
            // collapses on a ruled candidate ("glue") and the symmetries
            // between its candidates are carried out to be adjoined. It ran
            // in the syntropic arm alone, and there BEFORE the syllabus pull
            // and the sheaf sync of the same stroke, so a word learned on a
            // working stroke — the quote arm learns — or by the sync met no
            // vortex until a later depleted stroke: 481 vortices formed and
            // 0 collapsed in 5½ hours of 2cf578a, 35 and 0 in ten minutes of
            // 00bb4bd. A keyed read of what the genome gained; it spends
            // nothing when it gained nothing.
            let met = stroke.entity.pivot_thoughts();
            if met > 0 {
                eprintln!("[{tag}] pivot: {met} held thoughts spoken on charts the language now holds");
            }
        }
        // 24.3: WHAT THE MIND LEARNED THIS STROKE is the outcome's sixth
        // mode. The syllabus pull and the sheaf sync admit their pages
        // through `absorb_taught`, which measures each at its admission;
        // the stroke takes the measure once, here, so a stroke that learned
        // spends the lanes it learned on and does not read idle.
        StrokeOutcome::add(&mut outcome.learned, &entity.take_learned_mu());
        // OK1: the stroke's arms have run — quote, utter, learn, syntropy —
        // and the voice's cycle closes its orbit by name. `K` moves here
        // and nowhere else; a stroke that acted on nothing closes nothing. A
        // withheld close is counted on the clock, never printed beside it.
        entity.close_orbit();
        // 24.3: a working stroke spends the lanes its outcome moved; a stroke
        // that moved no lane spends the tick on all four — the tick, not work.
        // 24.3: a working stroke spends the lanes its outcome moved; a stroke
        // that moved no lane pays the TICK — one unit, the depletion floor —
        // and not `spend([true; 4])`, which is four entropic expansions
        // charged for germinating nothing. See `Physics::tick`.
        let moved = outcome.moved();
        if moved == [false; 4] {
            physics.tick();
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
            reanchored,
            condensed,
            learned,
            solves_paid,
        } = c;
        let Metabolism { book_k, .. } = m;
        let cycle = mycelium::physics::aperture_cycle() as u64;
        // The mark is a stroke this voice has passed, so the gap is exact.
        let since = stroke_n.checked_sub(last_logged_n).is_some_and(|g| g >= cycle);
        // PACED ON THE MIND'S OWN K (45.4). It paced on the book's, so a
        // voice thinking hard between quotes logged nothing and a voice
        // idle through a busy book logged every reply.
        let own_k = entity.own_k();
        if last_logged_k == u64::MAX || own_k != last_logged_k || since {
            let (atlas_ruled, atlas_stood, atlas_held) = entity.atlas_standing_counts();
            let (axis_charts, axis_words) = entity.axis_index_counts();
            // C3 — NOTHING PRINTS PER IDLE STROKE (ruled 2026-09-26): the
            // `stroke idle` line fired on every idle stroke the pacing let
            // through, 110 lines in five minutes on one voice. The idle stroke
            // is carried by the percept's `Spend` reading and the readings
            // pushed below; only a stroke that moved a lane is lined.
            if moved != [false; 4] {
                eprintln!(
                    "[{tag}] [{}] K {own_k} book {book_k} | genome {} | dialect {} | ideas {} | {} | cavity {} links {} | holds {:?} | entity {:?} memo {} | heap {} | grew {grew:?} | session {:?} | atlas words ruled {atlas_ruled} stood {atlas_stood} held {atlas_held} | axis index charts {axis_charts} words {axis_words} | shed {} census {:?} reserve {:?} carriers {} | fed {fed} derived {derived} witnessed {witnessed} settles {settles} err {errors} Z {} | learned {learned} projected {projected} solved {proj_solved} uniq {uniq_solved} paid {solves_paid} refined {refined} (reanchor {reanchored} condense {condensed}) discordance {} boundary {:?} axes {}",
                    mycelium::chronometry::Height::stamp_of(entity.now_tu()),
                    entity.epigenome.genome_len(),
                    entity.dialect_len(),
                    entity.ideas_len(),
                    entity.pivot_reading().line(),
                    entity.cavity_len(),
                    entity.cavity_links(),
                    entity.idea_weight(),
                    entity.carriers(),
                    extract_memo.len(),
                    heap(),
                    entity.session_sizes(),
                    entity.shed_n(),
                    entity.shed_census(),
                    entity.reserve(),
                    carrier_line(&entity),
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
            // WHAT THE COVERS' ORBITS CLASSIFIED AS, read from the counter
            // the engine already keeps. `tally_orbit` records every `Fill`
            // at `extract.rs` on the live path, and only `mycelium-soak`
            // ever read it: the fleet counted these at every stroke and
            // dropped them, so the only way to ask a running voice what it
            // filed was to grep its prose for a word — and the Congruent
            // arm prints "simplex filled", so that grep answered zero
            // while the counter held the number.
            let cover = mycelium::transducer::family::cover::take_cover_tally();
            eprintln!(
                "[{tag}] stroke covers meets {} edges {} carried {} | orbits flat {} filed {} standing {} unspellable {} unread {} | refused {} propagated {} spend {} pending {}",
                cover.meets,
                cover.edges,
                cover.carried,
                cover.flat,
                cover.filed,
                cover.standing,
                cover.unspellable,
                cover.orbit_unread,
                cover.refused,
                cover.propagated,
                cover.spend,
                cover.pending
            );
            // §8.1 — THE SAME INSTRUMENTS, AS READINGS, to the hypha's
            // console: folded into this voice's totals and pushed whole. A
            // push the hypha did not hold is said here, not dropped.
            if !entity.observe(&cover) {
                eprintln!("[{tag}] observe: the hypha did not hold this voice's readings");
            }
            last_logged_k = own_k;
            last_logged_n = stroke_n;
        }
        // The lanes the next stroke needs to expand on and cannot: one
        // reading per lane (S10, S13.2), never a bit recomputed per lane.
        let short = physics.reserve_shortfall(moved);
        let wait: [bool; 4] = std::array::from_fn(|l| short[l] > 0);
        m.dwell_remaining = physics.dwell(&m.last_pressure, wait);
        // X6 — AN IDLE STROKE COSTS THE PROCESSOR (the stroke-duration
        // ruling of 2026-09-11). A stroke that moved no lane admitted
        // nothing: the feed is clear, the quote is not due, no task stands.
        // It pays the tick above -- one integer decrement per lane -- and
        // then goes round again at once.
        // Measured on the fleet at 3d2987e: every voice between 81% and
        // 87% of a core while its own line read `stroke idle` and `quote 0
        // clusters`, which is five cores burnt to admit nothing and the
        // reason the courts' listeners were starved.
        //
        // The park is the measured transport where one has been measured
        // and the floor where none has, which is the pace this voice
        // already speaks at: a voice waiting on a door 40 polls away asks
        // again in 40 polls' worth of scheduler time, not immediately.
        // A working stroke does not park at all.
        if moved == [false; 4] {
            std::thread::sleep(mycelium::physics::idle_park(entity.transport_rtt()));
        }
    }
}
