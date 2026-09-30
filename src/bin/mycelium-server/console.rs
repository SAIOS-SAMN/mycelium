// §8.1 — THE DIAGNOSTIC CONSOLE: any hypha reads what a hypha holds about
// itself and its voices, and renders it whole.
//
//! `mycelium-server --console [--at ADDR] [--home DIR] [--voice HEX | --court
//! HEX] [--reading NAME]` asks one hypha (the
//! home's own bind by default) for every reading it holds, renders it once
//! and exits. It reads no clock (law 9): a live view is the operator's
//! `watch -n 5 mycelium-server --console`, and each frame is one question.
//!
//! WHAT IT SHOWS, per source, each reading in its own coordinates:
//!
//! - DIMENSIONS — the plane census in its algebra: every plane `n` declares,
//!   harmonic and luminal kept apart, each with the count it was turned or
//!   boosted on. Declared and never populated is FLATTENING made visible
//!   (Anti-Flattening clause 5); a census whose turns stand only on the
//!   sensor's plane is named so.
//! - LOSS — every axis a reading was passed over on, non-zero ones by name
//!   and meaning.
//! - MEMORY — the allocator's bytes at the source's first and latest `K`,
//!   the growth per `K` between them, and the peak: a leak is growth over
//!   the source's own clock.
//! - TUNIVERSAL — FIRST, before any other reading (ACT1, 78.21(4)): every
//!   source by its height with its own `K` beside it, and the spread.
//! - CLOCK — per source: a court's genesis, per-peer heights and gaps,
//!   where its snapshot and last fold stand, and the three counts (acts
//!   sealed without a frame, words held without a stamp by row, rulings
//!   admitted against applied); a voice's `K`, court height as last heard
//!   and genome persist.
//! - COURT — the court's `K`, basis, treasury and the acts it served.
//! - PHASE — the court's last phase window and adjoined-by-closure (PH5).
//! - LEARNING — a mind's curve at its `K` (LC1): dialect span, cover, what
//!   its closures did, and every field still owed, by its row.
//! - CIRCUITS — the last percept's kept circuits step by step (PL1): the
//!   bricks a holonomy is built of, and its first refraction named.
//! - CELLS, VISCOSITY, SPEND, PROGRESS — the last percept's cells (CO1),
//!   and summed over the voice's percepts its bends' arms (VO1), the stroke
//!   by reader (SP1) and E1's done-when (PG1).
//! - CARRIAGE — the bytes per award, per receipt delta by act kind and per
//!   frame by act tag, and the largest of each (LD1).
//!
//! CQ1 — `--voice HEX` or `--court HEX` (a key or a hex prefix of it) asks
//! one source's fiber, and `--reading NAME` one kind of reading: the hypha
//! serves only the prefix asked, and the console reads nothing else.

use mycelium::observe::{self, Court, Loss, Memory, Planes, Reading, Source, LOSS_AXES};
use mycelium::transducer::cga;
use std::path::Path;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const CYAN: &str = "\x1b[96m";
const GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const RED: &str = "\x1b[91m";

/// The address the console asks: `--at`, or the bind this home's own mesh
/// record names. `None` where neither reads.
fn address(home: &Path, at: Option<&str>) -> Option<String> {
    if let Some(a) = at {
        return Some(a.to_string());
    }
    let key = mycelium::court::identity::HyphaKey::load_only(&home.join("hypha.key").to_string_lossy())?;
    let bytes = std::fs::read(home.join("mesh")).ok()?;
    let (rec, _) = mycelium::network::mesh::MeshRecord::open_layout(&bytes, &key.vk_bytes())?;
    rec.bind
}

/// CQ1 — WHAT THE OPERATOR ASKS: one voice or one court by its key or a hex
/// prefix of it, and one kind of reading by its name.
pub(crate) struct Ask<'a> {
    pub(crate) voice: Option<&'a str>,
    pub(crate) court: Option<&'a str>,
    pub(crate) reading: Option<&'a str>,
}

/// A key's hex prefix as bytes: an even count of hex digits, at most 32
/// bytes' worth. `None` for anything else, refused by name at the caller.
fn hex_prefix(s: &str) -> Option<Vec<u8>> {
    let s = s.trim().as_bytes();
    if s.len() % 2 != 0 || s.len() > 64 {
        return None;
    }
    s.chunks(2).map(|d| u8::from_str_radix(std::str::from_utf8(d).ok()?, 16).ok()).collect()
}

impl Ask<'_> {
    /// The query's prefix — the source's kind and key bytes, and the
    /// reading's tag where the key is whole — and the tag to keep where the
    /// key is only a prefix. `Err` names what was not read.
    fn prefix(&self) -> Result<(Vec<u8>, Option<u8>), String> {
        let tag = match self.reading {
            None => None,
            Some(name) => match observe::READING_NAMES.iter().find(|(n, _)| *n == name) {
                Some(&(_, t)) => Some(t),
                None => {
                    let names: Vec<&str> = observe::READING_NAMES.iter().map(|(n, _)| *n).collect();
                    return Err(format!("no reading named {name}: one of {}", names.join(", ")));
                }
            },
        };
        let (kind, hex) = match (self.voice, self.court) {
            (Some(_), Some(_)) => return Err("ask one source: --voice or --court, not both".into()),
            (Some(v), None) => (observe::SOURCE_VOICE, v),
            (None, Some(c)) => (observe::SOURCE_COURT, c),
            (None, None) => return Ok((Vec::new(), tag)),
        };
        let Some(key) = hex_prefix(hex) else {
            return Err(format!("{hex} is not a key's hex prefix (an even count of hex digits, at most 64)"));
        };
        let mut prefix = vec![kind];
        prefix.extend_from_slice(&key);
        match (key.len(), tag) {
            (32, Some(t)) => {
                prefix.push(t);
                Ok((prefix, None))
            }
            _ => Ok((prefix, tag)),
        }
    }
}

/// ASK, RENDER, EXIT. Exit 0 with a rendering, 2 where no hypha answered —
/// the rendering then says why, so a `watch` shows the refusal in place.
/// CQ1 — the question is the prefix the operator names; the hypha serves
/// only that fiber.
pub(crate) fn run(home: &Path, at: Option<&str>, ask: &Ask) -> ! {
    let (prefix, keep) = match ask.prefix() {
        Ok(p) => p,
        Err(why) => {
            println!("{RED}{why}{RESET}");
            std::process::exit(2)
        }
    };
    let Some(addr) = address(home, at) else {
        println!("{RED}no address to ask{RESET}: pass --at ADDR, or --home a home whose mesh names its bind");
        std::process::exit(2)
    };
    let Some(query) = observe::encode_query(&prefix) else {
        println!("{RED}the prefix passes its length field{RESET}");
        std::process::exit(2)
    };
    // THE OPERATOR WAITS A RESERVE, NOT A PEER'S FLOOR. The console asked
    // at `tell_peer`'s literal 5 (≈ 40 polls), so a court whose workers were
    // busy for longer read "did not answer" — hyphae b and c at 12:04 while
    // they verified a burst of fold prefixes. The console's patience is a
    // full reserve of polls.
    let Some((reply, _)) = crate::transport::tell_peer_within(&addr, query, mycelium::physics::reserve_max()) else {
        println!("{RED}{addr} did not answer{RESET}");
        std::process::exit(2)
    };
    let Some((total, mut held)) = observe::decode_reply(&reply) else {
        println!("{RED}{addr} answered with no reading this build reads{RESET} ({} bytes)", reply.len());
        std::process::exit(2)
    };
    // What the frame budget cut stays said after the kind is kept.
    // In the count's own carrier: `held` is the reply's entries, and the
    // arithmetic is exact or the line says it could not count.
    let cut = u64::from(total).abs_diff(held.len() as u64);
    if let Some(t) = keep {
        held.retain(|(k, _)| Source::of_key(k).is_some_and(|(_, tag)| tag == t));
    }
    let Some(total) = (held.len() as u64).checked_add(cut) else {
        println!("{RED}{addr} answered a total past any count{RESET}");
        std::process::exit(2)
    };
    for line in render(&addr, total, &held) {
        println!("{line}");
    }
    std::process::exit(0)
}

fn short(vk: &[u8; 32]) -> String {
    format!("{:02x}{:02x}{:02x}{:02x}…", vk[0], vk[1], vk[2], vk[3])
}

/// Every source's readings gathered under it, in key order (courts first,
/// by the source kind the key opens with).
fn by_source(held: &[observe::Held]) -> Vec<(Source, Vec<Reading>)> {
    let mut out: Vec<(Source, Vec<Reading>)> = Vec::new();
    for (k, r) in held {
        let Some((source, _)) = Source::of_key(k) else { continue };
        match out.last_mut() {
            Some((s, rs)) if *s == source => rs.push(r.clone()),
            _ => out.push((source, vec![r.clone()])),
        }
    }
    out
}

pub(crate) fn render(addr: &str, total: u64, held: &[observe::Held]) -> Vec<String> {
    let sources = by_source(held);
    let mut lines = vec![format!(
        "{CYAN}{BOLD}═══ MYCELIUM OBSERVER ═══{RESET}  {addr}  {DIM}{} readings of {total} held, {} sources{RESET}",
        held.len(),
        sources.len()
    )];
    if (held.len() as u64) < total {
        lines.push(format!("{YELLOW}the reply held {} of {total}: the frame budget cut it{RESET}", held.len()));
    }
    lines.extend(tuniversal(&sources));
    for (source, readings) in &sources {
        lines.push(String::new());
        lines.push(match source {
            Source::Court(vk) => format!("{CYAN}{BOLD}── COURT {}{RESET}", short(vk)),
            Source::Voice(vk) => format!("{CYAN}{BOLD}── VOICE {}{RESET}", short(vk)),
        });
        for r in readings {
            lines.extend(match r {
                Reading::Court(c) => court(c),
                Reading::Planes(p) => dimensions(p),
                Reading::Loss(l) => loss(l),
                Reading::Memory(m) => memory(m),
                Reading::Replication(r) => replication(r),
                Reading::Adjudication(a) => adjudication(a),
                Reading::Learning(l) => learning(l),
                Reading::Circuit(c) => circuits(c),
                Reading::Cells(c) => cells(c),
                Reading::Viscosity(v) => viscosity(v),
                Reading::Spend(s) => spend(s),
                Reading::Progress(p) => progress(p),
                Reading::Phase(p) => phase(p),
                Reading::Carriage(c) => carriage(c),
                Reading::Clock(c) => clock(c),
                Reading::Economy(e) => economy(e),
                Reading::Senses(f) => f.lines().into_iter().map(|l| format!("  {l}")).collect(),
                Reading::Acts(a) => a.lines().into_iter().map(|l| format!("  {l}")).collect(),
            });
        }
    }
    lines
}

/// ACT1, 78.21(4) — EVERY SOURCE ON TUNIVERSAL FIRST: each by its height
/// with its own `K` beside it, and the spread of heights over the sources
/// read. Disparity is read in height only, and no threshold colours it
/// (rule 5); a source that carried no clock says so.
fn tuniversal(sources: &[(Source, Vec<Reading>)]) -> Vec<String> {
    let mut lines = vec![String::new(), format!("{CYAN}{BOLD}── TUNIVERSAL{RESET}")];
    let mut heights: Vec<u64> = Vec::new();
    for (source, readings) in sources {
        let name = match source {
            Source::Court(vk) => format!("court {}", short(vk)),
            Source::Voice(vk) => format!("voice {}", short(vk)),
        };
        let clock = readings.iter().find_map(|r| match r {
            Reading::Clock(c) => Some(c),
            _ => None,
        });
        lines.push(match clock {
            Some(observe::Clock::Court(c)) => {
                heights.push(c.head[0]);
                let [h, s, g, t, hh, k, i] = c.head;
                format!(
                    "  {name}  {BOLD}h {h}{RESET}  K {}  stamp {h}:{s}:{g}:{t}:{hh}:{k}.{i}{}",
                    c.k,
                    orbits(&c.orbits)
                )
            }
            Some(observe::Clock::Voice(v)) => {
                heights.push(v.heard);
                format!(
                    "  {name}  {BOLD}h {}{RESET} {DIM}(its court, last heard){RESET}  K {}  radius {}",
                    v.heard,
                    v.k,
                    v.radius
                )
            }
            None => format!("  {name}  {DIM}no clock carried{RESET}"),
        });
    }
    #[expect(
        clippy::disallowed_methods,
        reason = "78.21(1) — the fleet's spread IS the range of heads, max − min: a reading of the range, and nothing is chosen by it"
    )]
    let range = (heights.iter().min().copied(), heights.iter().max().copied());
    match range {
        (Some(min), Some(max)) => lines.push(format!("  {BOLD}spread{RESET}      h {min}..{max}  ({} heights)", max - min)),
        _ => lines.push(format!("  {DIM}no source carried a clock{RESET}")),
    }
    lines
}

/// OK1, 78.23 as amended — A NODE'S ORBITS AGAINST THE OTHERS' on its
/// TUniversal line: the closes withheld for separation, and at its last
/// close the per-lane signed distance to each other orbit closed at that
/// tick. A zero lane is a lane the two share; the boundary withholds a
/// close whose four lanes would all be zero.
fn orbits(o: &observe::Orbits) -> String {
    let mut out = String::new();
    if o.withheld > 0 {
        out.push_str(&format!("  {YELLOW}withheld {}{RESET}", o.withheld));
    }
    for (vk, [s, g, t, h]) in &o.distance {
        out.push_str(&format!("  Δ{} {s:+}:{g:+}:{t:+}:{h:+}", short(vk)));
    }
    out
}

/// EC1, 78.33 — A COURT'S PAYMENTS BY PROOF OF WORK: each arm on its own
/// four lanes, paid and burned, never summed across arms.
fn economy(e: &observe::Economy) -> Vec<String> {
    let mut out: Vec<String> = mycelium::economy::Arm::ALL
        .iter()
        .map(|arm| {
            let i = arm.index();
            format!("  {BOLD}{:<12}{RESET}paid {:?}  burned {:?}", arm.name(), e.paid[i], e.burned[i])
        })
        .collect();
    // STD1 — what the treasury refused past its carrier.
    out.push(format!("  {BOLD}{:<12}{RESET}{}", "overflowed", e.overflowed));
    // What moved, by kind, on the journal since its last fold; a kind
    // with no row is not printed.
    out.push(format!("  {DIM}flows on the journal since its last fold{RESET}"));
    for (name, f) in observe::FLOWS.iter().zip(&e.flows) {
        if f.rows > 0 {
            out.push(format!("    {name:<26}rows {:<8}{:?}", f.rows, f.mu));
        }
    }
    out.push(format!("  {BOLD}{:<12}{RESET}payments {}  {:?}  {DIM}learners to this court, for sentences taught{RESET}", "tuition", e.tuition.rows, e.tuition.mu));
    // E3 — each identity's standing: what it can spend, has staked, holds.
    for a in &e.accounts {
        out.push(format!(
            "  {}  balance {:?}  bond {:?}  held {:?}",
            short(&a.who),
            a.balance,
            a.bond,
            a.held
        ));
    }
    out
}

/// ACT1 — ONE NODE ON THE CLOCK, and its three counts.
fn clock(c: &observe::Clock) -> Vec<String> {
    // A persist above where the node now stands is a count that went
    // backwards, named as such and never floored at zero (rule 11).
    let since = |at: Option<(u64, u64)>, h: u64, k: u64| match at {
        None => format!("{DIM}never{RESET}"),
        Some((ph, pk)) => match (h.checked_sub(ph), k.checked_sub(pk)) {
            (Some(dh), Some(dk)) => format!("at h {ph} K {pk}  ({dh} heights, {dk} K since)"),
            _ => format!("{YELLOW}at h {ph} K {pk}, above where it stands now (h {h} K {k}){RESET}"),
        },
    };
    match c {
        observe::Clock::Court(c) => {
            let [h, ..] = c.head;
            let genesis = match c.genesis {
                None => format!("{YELLOW}no origin adopted{RESET}"),
                Some((id, epoch)) => format!("{} epoch {epoch}", short(&id)),
            };
            let peers: Vec<String> =
                c.peers.iter().map(|(vk, ph, gap)| format!("{} h {ph} gap {gap}", short(vk))).collect();
            let [ub, up, ul, uc, uf] = c.unstamped;
            let unstamped = format!("basis {ub}  provisional {up}  listing {ul}  cavity {uc}  federation {uf}");
            let flag = |bad: bool, s: String| match bad {
                true => format!("{YELLOW}{s}{RESET}"),
                false => s,
            };
            // EC1: per ruling arm, admitted = applied + awaiting + refused;
            // an arm with none admitted says nothing. A refusal is flagged.
            let arms: Vec<String> = mycelium::chronometry::Ruled::ALL
                .iter()
                .filter_map(|arm| {
                    let [admitted, applied, awaiting, refused] = c.rulings[arm.index()];
                    // F5 — each refusal by its reason, where any counted.
                    let why: Vec<String> = mycelium::chronometry::RulingRefused::ALL
                        .iter()
                        .map(|r| (r.name(), c.ruling_refusals[arm.index()][r.index()]))
                        .filter(|(_, n)| *n > 0)
                        .map(|(name, n)| format!("{name} {n}"))
                        .collect();
                    let why = match why.is_empty() {
                        true => String::new(),
                        false => format!(" ({})", why.join(", ")),
                    };
                    (admitted > 0).then(|| {
                        flag(
                            refused > 0,
                            format!(
                                "{} admitted {admitted} applied {applied} awaiting {awaiting} refused {refused}{why}",
                                arm.name()
                            ),
                        )
                    })
                })
                .collect();
            let rulings = match arms.is_empty() {
                true => format!("{DIM}no peer ruling admitted{RESET}"),
                false => format!("rulings {}", arms.join("  ")),
            };
            vec![
                format!("  {BOLD}genesis{RESET}     {genesis}"),
                format!("  {BOLD}peers{RESET}       {}", match peers.is_empty() {
                    true => format!("{DIM}none held{RESET}"),
                    false => peers.join("  "),
                }),
                format!("  {BOLD}persisted{RESET}   {}", since(c.persisted, h, c.k)),
                format!("  {BOLD}folded{RESET}      {}", since(c.folded, h, c.k)),
                format!("  {BOLD}unframed{RESET}    {}", flag(c.unframed > 0, format!("{} acts sealed on a word nothing measured", c.unframed))),
                format!("  {BOLD}unstamped{RESET}   {}", flag(ub + up + ul + uc + uf > 0, unstamped)),
                format!("  {BOLD}economy{RESET}     {rulings}"),
            ]
        }
        observe::Clock::Voice(v) => vec![
            format!("  {BOLD}clock{RESET}       K {}  court h {} as last heard", v.k, v.heard),
            format!("  {BOLD}genome{RESET}      persisted {}", since(v.persisted, v.heard, v.k)),
        ],
    }
}

fn court(c: &Court) -> Vec<String> {
    let served: Vec<String> = c.served.iter().map(|(kind, n)| format!("{kind:02x}×{n}")).collect();
    vec![
        format!(
            "  {BOLD}standing{RESET}    K {}  basis {}  treasury {}  reopened filled {} congruent {}",
            c.k, c.basis, c.treasury, c.reopened[0], c.reopened[1]
        ),
        format!("  {BOLD}served{RESET}      {}", match served.is_empty() {
            true => format!("{DIM}nothing yet{RESET}"),
            false => served.join(" "),
        }),
        format!(
            "  {BOLD}remainders{RESET}  {} | basis words with a filed torsion {}",
            c.remainders.line(),
            c.torsion_words
        ),
        format!(
            "  {BOLD}refused{RESET}     room broken {} past carrier {} | overflowed {}",
            c.room_broken, c.room_overflowed, c.overflowed
        ),
        format!(
            "  {BOLD}fold{RESET}        {}",
            observe::FOLD_OUTCOMES.iter().zip(c.folds).map(|(axis, n)| format!("{axis} {n}")).collect::<Vec<_>>().join("  ")
        ),
        format!(
            "  {BOLD}catch-up{RESET}    {}",
            observe::CATCHUP_PATHS
                .iter()
                .zip(c.catchup)
                .map(|(path, row)| format!(
                    "{path}: {}",
                    observe::CATCHUP_OUTCOMES.iter().zip(row).map(|(o, n)| format!("{o} {n}")).collect::<Vec<_>>().join(" ")
                ))
                .collect::<Vec<_>>()
                .join(" | ")
        ),
    ]
}

/// THE PLANE CENSUS IN ITS ALGEBRA: every declared plane, generated from
/// `n`, each with its count; the halves apart; and what was never turned.
fn dimensions(p: &Planes) -> Vec<String> {
    let row = |planes: &[usize]| -> String {
        planes
            .iter()
            .map(|&b| match p.count(b) {
                0 => format!("{DIM}{} 0{RESET}", cga::plane_name(b)),
                c => format!("{GREEN}{} {c}{RESET}", cga::plane_name(b)),
            })
            .collect::<Vec<_>>()
            .join("  ")
    };
    let (harmonic, luminal) = (cga::harmonic_planes(p.n), cga::transvection_planes(p.n));
    // PC1 — the parabolic planes the algebra declares, each a null bivector
    // counted once: every base axis with each of the two poles.
    let parabolic: Vec<(u8, cga::Pole)> =
        (1..=p.n).flat_map(|i| [cga::Pole::Infinity, cga::Pole::Origin].map(|q| (i, q))).collect();
    let parabolic_row = parabolic
        .iter()
        .map(|&(i, q)| match p.count_parabolic(i, q) {
            0 => format!("{DIM}{} 0{RESET}", cga::parabolic_name(usize::from(i), q)),
            c => format!("{GREEN}{} {c}{RESET}", cga::parabolic_name(usize::from(i), q)),
        })
        .collect::<Vec<_>>()
        .join("  ");
    let parabolic_used = parabolic.iter().filter(|&&(i, q)| p.count_parabolic(i, q) > 0).count();
    let used = |planes: &[usize]| planes.iter().filter(|&&b| p.count(b) > 0).count();
    // The classification is the reading's own (`Planes::turning`); the
    // console renders it.
    let verdict = match p.turning() {
        observe::Turning::Unread => format!("{DIM}no transition read yet{RESET}"),
        observe::Turning::Unturned => {
            format!("{YELLOW}nothing turned: every transition was a boost, a slip or the identity{RESET}")
        }
        observe::Turning::Flattened { other } => format!(
            "{RED}{BOLD}FLATTENED{RESET}{RED} — every turn stands on the sensor's plane {}; {other} other harmonic planes declared and never turned{RESET}",
            cga::plane_name(cga::E1 | cga::E2)
        ),
        observe::Turning::Turns { turned, of } => format!("{GREEN}turns on {turned} of {of} harmonic planes{RESET}"),
    };
    vec![
        format!(
            "  {BOLD}dimensions{RESET}  n {}  Cl({},1)  read {}  coupled {}  harmonic {}/{}  luminal {}/{}  parabolic {}/{}",
            p.n,
            u32::from(p.n) + 1,
            p.read,
            p.coupled,
            used(&harmonic),
            harmonic.len(),
            used(&luminal),
            luminal.len(),
            parabolic_used,
            parabolic.len()
        ),
        format!("    harmonic  {}", row(&harmonic)),
        format!("    luminal   {}", row(&luminal)),
        format!("    parabolic {parabolic_row}"),
        format!("    {verdict}"),
    ]
}

/// EVERY AXIS OF LOSS: the non-zero ones by name and meaning, the zero ones
/// named on one line so their absence is read and not assumed.
fn loss(l: &Loss) -> Vec<String> {
    let mut out = vec![format!("  {BOLD}loss{RESET}")];
    let mut zero: Vec<&str> = Vec::new();
    for ((name, meaning), &v) in LOSS_AXES.iter().zip(l.axes.iter()) {
        match v {
            0 => zero.push(name),
            v => out.push(format!("    {YELLOW}{name} {v}{RESET}  {DIM}{meaning}{RESET}")),
        }
    }
    if !zero.is_empty() {
        out.push(format!("    {DIM}zero: {}{RESET}", zero.join(" ")));
    }
    out
}

/// The non-zero axes of one row of counters, each by its name, and a dim
/// note naming how many read zero.
fn axes_line(names: &[(&str, &str)], values: &[u64]) -> String {
    let set: Vec<String> = names
        .iter()
        .zip(values)
        .filter(|(_, &v)| v > 0)
        .map(|((name, _), v)| format!("{name} {v}"))
        .collect();
    match set.is_empty() {
        true => format!("{DIM}nothing counted{RESET}"),
        false => set.join("  "),
    }
}

/// REPLICATION PER SIGNER (CV5): what this court did with each signer's
/// receipts, by axis, and what no signer could be read off.
fn replication(r: &observe::Replication) -> Vec<String> {
    use observe::{rep, REPLICATION_AXES};
    let mut out = vec![format!("  {BOLD}replication{RESET} {} signers heard", r.signers.len())];
    for (vk, axes) in &r.signers {
        let admitted = axes[rep::PARENT] + axes[rep::BACKFILL];
        let colour = match admitted {
            0 => YELLOW,
            _ => GREEN,
        };
        out.push(format!(
            "    {colour}{}{RESET}  admitted {admitted}  {}",
            short(vk),
            axes_line(&REPLICATION_AXES, axes)
        ));
    }
    if r.unattributed.iter().any(|&v| v > 0) {
        out.push(format!(
            "    {YELLOW}no signer{RESET}  {}",
            axes_line(&REPLICATION_AXES, &r.unattributed)
        ));
    }
    out
}

/// ADJUDICATION BY KIND (CV5): the verdicts, and the refusals by their own
/// kind's name.
fn adjudication(a: &observe::Adjudication) -> Vec<String> {
    let refused: Vec<String> = a
        .refused
        .iter()
        .map(|&(kind, n)| match mycelium::transducer::sentence::RefusalKind::from_byte(kind) {
            Some(k) => format!("{} {n}", k.label()),
            None => format!("kind {kind:#04x} {n}"),
        })
        .collect();
    vec![
        format!(
            "  {BOLD}adjudication{RESET} ratified {}  stood {}  awaiting {}  deferred {}",
            a.ratified, a.stood, a.awaiting, a.deferred
        ),
        format!(
            "    refused  {}",
            match refused.is_empty() {
                true => format!("{DIM}nothing{RESET}"),
                false => format!("{YELLOW}{}{RESET}", refused.join("  ")),
            }
        ),
        // CV1 — the same refusals by the kind of obstruction (78.9).
        format!(
            "    by kind  {}",
            mycelium::transducer::sentence::ObstructionKind::ALL
                .iter()
                .zip(a.obstructed)
                .map(|(k, n)| format!("{} {n}", k.label()))
                .collect::<Vec<_>>()
                .join("  ")
        ),
    ]
}

/// LC1 — THE LEARNING CURVE OF ONE MIND AT ITS `K` (78.1, 78.3): every field
/// by name, zero printed as zero, and each field no producer reads yet by the
/// row that owes it. The planes the mind reads are its DIMENSIONS reading
/// above, never repeated here.
fn learning(l: &observe::Learning) -> Vec<String> {
    let words: u64 = l.strata.iter().map(|(_, n)| n).sum();
    let owed: Vec<String> = observe::OWED.iter().map(|(field, row)| format!("{field} ({row})")).collect();
    vec![
        format!("  {BOLD}learning{RESET}    K {}", l.k),
        format!("    dialect span  {} strata holding {words} words", l.strata.len()),
        format!(
            "    closures      adjoined-by-closure {}  installed {}  closing {}",
            l.adjoined_by_closure, l.installed, l.closing
        ),
        // CS2 — the pivots against the cells their orbits cross (78.32(c)).
        format!("    {}", l.pivots.line()),
        // 13.12 — the contemplation's merges (TP1's line).
        format!("    {}", l.merges.line()),
        // 78.37(1), TP1 — the meet graph, its rings, its decoherences, and
        // what TP1's fix owes, by name.
        format!("    {}", l.meet.line()),
        // PR1 — the voice's probes: closure states, lanes moved, the residual
        // versors' classes.
        format!("    {}", l.probes.line()),
        // 78.47 — why the mind did not move, by kind.
        format!("    {}", l.reasons.line()),
        format!("    {DIM}owed: {}{RESET}", owed.join("  ")),
    ]
}

/// PL1 — THE CIRCUITS OF THE LAST PERCEPT, STEP BY STEP (78.12(4)): each
/// circuit by its chord with the class its steps compose to and its first
/// refraction named, then every step — the face and which way, `|` where
/// the face refracts, the step's class, its modes, the class composed
/// through it and its ray — and the fields still owed, by their rows.
/// An overlap as the console names it: its two vertices' coordinates, the
/// refusal's own label (AT1 — a face is an overlap, never a position).
fn overlap(o: &mycelium::network::nerve::coord::Overlap) -> String {
    mycelium::transducer::sentence::At::face(o.from_vertex(), o.to_vertex()).label()
}

fn circuits(c: &observe::Circuits) -> Vec<String> {
    let kind = |k: Option<mycelium::network::nerve::coord::BivectorKind>| match k {
        Some(k) => format!("{k:?}").to_lowercase(),
        None => "unread".into(),
    };
    let owed: Vec<String> = observe::CIRCUIT_OWED.iter().map(|(field, row)| format!("{field} ({row})")).collect();
    let mut out = vec![format!("  {BOLD}circuits{RESET}    {} kept by the last percept", c.circuits.len())];
    for circuit in &c.circuits {
        let holonomy = match circuit.steps.as_slice() {
            [.., observe::CircuitStep { carried: observe::Carried::Read { after, .. }, .. }] => kind(*after),
            _ => "unread".into(),
        };
        let refraction = match circuit.first_refraction() {
            Some(i) => format!("{YELLOW}first refraction at step {} (face {}){RESET}", i + 1, overlap(&circuit.steps[i].face)),
            None => format!("{DIM}no step refracts{RESET}"),
        };
        out.push(format!(
            "    chord {}  {} steps  holonomy {holonomy}  {refraction}",
            overlap(&circuit.chord),
            circuit.steps.len()
        ));
        for (i, s) in circuit.steps.iter().enumerate() {
            let way = match (s.forward, s.refracted) {
                (true, false) => "→",
                (false, false) => "←",
                (true, true) => "|→",
                (false, true) => "|←",
            };
            let read = match &s.carried {
                observe::Carried::Unread => format!("{YELLOW}unread{RESET}"),
                observe::Carried::Cone => format!("{YELLOW}on the cone{RESET}"),
                observe::Carried::Read { ray, kind: k, modes, after } => {
                    let ray: Vec<String> = ray.iter().map(|(b, q)| format!("{} {q}", cga::blade_name(usize::from(*b)))).collect();
                    format!(
                        "{}  modes [{}]  ⇒ {}  {DIM}ray [{}]{RESET}",
                        kind(*k),
                        modes.label(),
                        kind(*after),
                        ray.join(", ")
                    )
                }
            };
            out.push(format!("      {:>2} face {}{way}  {read}", i + 1, overlap(&s.face)));
        }
        // CT1 — the reuse ladder's record: every arm that ran, what it read,
        // and the class their disagreement gives (79.3).
        if let Some(t) = &circuit.test {
            let arms: Vec<String> = t
                .arms
                .iter()
                .map(|(arm, read)| {
                    let r = match read {
                        observe::ArmRead::Empty => "empty".to_string(),
                        observe::ArmRead::Reads(n) => format!("reads {n}"),
                        observe::ArmRead::Routes { agree, disagree } => format!("agree {agree} disagree {disagree}"),
                        observe::ArmRead::Permanent => "permanent".to_string(),
                    };
                    format!("{}: {r}", arm.label())
                })
                .collect();
            out.push(format!("      {BOLD}test{RESET} {}  {DIM}[{}]{RESET}", t.class.label(), arms.join("; ")));
            // CT2 — each construction offered: a candidate, never installed.
            for o in &t.offers {
                let ray = |r: &[(u16, num_rational::BigRational)]| {
                    r.iter().map(|(b, q)| format!("{} {q}", cga::blade_name(usize::from(*b)))).collect::<Vec<_>>().join(", ")
                };
                out.push(format!(
                    "        {YELLOW}offer{RESET} {} at face {}: {}  modes [{}]  {DIM}constructor [{}] holonomy [{}]{RESET}",
                    o.arm.label(),
                    overlap(&o.face),
                    if o.flat { "flat" } else { "curved" },
                    o.modes.label(),
                    ray(&o.constructor),
                    ray(&o.holonomy)
                ));
            }
        }
    }
    out.push(format!("    {DIM}owed: {}{RESET}", owed.join("  ")));
    out
}

/// CO1 — THE CELLS OF THE LAST PERCEPT AS THEIR ORBIT ALGEBRAS (78.14, 78.15):
/// the form's mirrors and their group, then each cell — its grade, its Gram
/// form's signature and pairs, its group with the nuclei and with the polars
/// as mirrors, its dual frame, each product kept with the blade its sealing
/// step deposits, and each nucleus against the mirrors it stands between.
fn cells(c: &observe::Cells) -> Vec<String> {
    let closure = |c: Option<observe::CellClosure>| match c {
        Some(c) => c.label().to_string(),
        None => "unread".into(),
    };
    let regime = |r: &cga::PairRegime| match r {
        cga::PairRegime::Finite(m) => format!("m={m}"),
        cga::PairRegime::Parabolic => "parabolic".into(),
        cga::PairRegime::Boost => "boost".into(),
        cga::PairRegime::Rosette => "rosette".into(),
        cga::PairRegime::Null => "null".into(),
    };
    // 78.26(2) — the cross-cell bracket, where the sentence spans cells.
    let bracket = match c.bracket {
        Some(b) => format!(" | bracket held {} lost {} unread {}", b.held, b.lost, b.unread),
        None => String::new(),
    };
    let mut out = vec![format!(
        "  {BOLD}cells{RESET}       {} certified | mirrors {} planes, group {}{bracket}",
        c.cells.len(),
        c.mirror_planes,
        closure(c.mirror_group)
    )];
    for (i, cell) in c.cells.iter().enumerate() {
        let sig = match cell.signature {
            Some(((p, q, z), r)) => format!("({p},{q},{z}) rank {r}"),
            None => "unread".into(),
        };
        let dual = match cell.dual {
            Some(observe::CellDual::Frame) => "frame",
            Some(observe::CellDual::Redundant) => "redundant",
            Some(observe::CellDual::Degenerate) => "degenerate",
            None => "unread",
        };
        // CS2 — the held constructors and the rank against its own cap.
        let held = match cell.constructors {
            Some(k) => format!("{k}"),
            None => "unread".into(),
        };
        // MT1 — the anastomosis's degeneracy, bodies less the nuclei's rank.
        let degeneracy = match cell.degeneracy {
            Some(d) => format!("{d}"),
            None => "unread".into(),
        };
        let rank = match cell.capacity {
            Some(r) if r > c.cap => format!("rank {r} ABOVE cap {}", c.cap),
            Some(r) => format!("rank {r} of cap {}", c.cap),
            None => format!("rank unread of cap {}", c.cap),
        };
        out.push(format!(
            "    #{i} grade {}  constructors {held}  degeneracy {degeneracy}  {rank}  gram {sig}  pairs [{}]  group {}  polars {}  dual {dual}  classes {}",
            cell.grade,
            cell.pairs.iter().map(regime).collect::<Vec<_>>().join(", "),
            closure(cell.coxeter),
            closure(cell.polars),
            cell.classes
        ));
        for p in &cell.products {
            let blade: Vec<String> = p.seal.iter().map(|(b, q)| format!("{} {q}", cga::blade_name(usize::from(*b)))).collect();
            let det = match p.det {
                Some(d) => format!("det {d}"),
                None => "det unread".into(),
            };
            out.push(format!(
                "      product order {:?} grades {:?} {det}{}  {DIM}seal [{}]{RESET}",
                p.order,
                p.grades,
                if p.classed { " classed" } else { "" },
                blade.join(", ")
            ));
        }
        match &cell.enclosure {
            Some(observe::CellEnclosure::Flat) => out.push(format!("      {DIM}enclosure flat: the centres span no round{RESET}")),
            Some(observe::CellEnclosure::Round { bodies, within }) => out.push(format!(
                "      enclosure  bodies inside {}  on {}  apart {}  wraps {}  unread {}  {}",
                bodies[0],
                bodies[1],
                bodies[2],
                bodies[3],
                bodies[4],
                match within {
                    Some(w) => format!("within #{w}"),
                    None => "a root of the parse".into(),
                }
            )),
            None => {}
        }
        for (s, ms) in cell.mirrors.iter().enumerate() {
            let on: Vec<String> = ms.iter().map(|(axis, k, d2, r)| format!("x{axis}={k}: δ² {d2} {}", regime(r))).collect();
            out.push(format!("      {DIM}nucleus {s} on mirrors [{}]{RESET}", on.join("; ")));
        }
        // CV1b — what the cell's readers could not hold, by content.
        if cell.refusals.any() {
            out.push(format!("      {YELLOW}refused{RESET}  {}", observe::refusals_line(&cell.refusals)));
        }
        // 78.26(5) — the rings through the cell, by their residue.
        let rings = observe::rings_line(&cell.rings);
        if !rings.is_empty() {
            out.push(format!("      {DIM}{rings}{RESET}"));
        }
    }
    out
}

/// VO1 — THE VISCOSITY A VOICE'S BENDS READ (78.7 as amended): per lane,
/// each pair of planes a trial ran on with its four arms; the lanes that
/// carry no bend named; the ledger's entropy by lane.
fn viscosity(v: &observe::Viscosity) -> Vec<String> {
    use mycelium::transducer::sentence::{bend_plane_name, BendLane};
    let mut out = vec![format!("  {BOLD}viscosity{RESET}   entropy by lane (𝒮, 𝒢, 𝒯, 𝒽) {:?}", v.entropy)];
    for (lane, name) in [(BendLane::Spatial, "𝒮"), (BendLane::Holonomic, "𝒽")] {
        let pairs: Vec<&observe::ViscosityPair> = v.pairs.iter().filter(|p| usize::from(p.lane) == lane.index()).collect();
        if pairs.is_empty() {
            out.push(format!("    {name}  {DIM}no held loop{RESET}"));
        }
        for p in pairs {
            let colour = match (p.arms[3], p.arms[2]) {
                (0, 0) => DIM,
                (0, _) => GREEN,
                _ => YELLOW,
            };
            out.push(format!(
                "    {name}  {colour}{}·{}{RESET}  inviscid {}  elastic {}  curved {}  viscous {}  {DIM}refused {} unread {} no-class {}{RESET}",
                bend_plane_name(p.lane, p.planes.0),
                bend_plane_name(p.lane, p.planes.1),
                p.arms[0],
                p.arms[1],
                p.arms[2],
                p.arms[3],
                p.refused,
                p.unread,
                p.no_class
            ));
        }
    }
    out.push(format!("    {DIM}𝒢, 𝒯: no bend on this lane{RESET}"));
    out
}

/// SP1 — THE STROKE BY READER: what each reader spent over the voice's
/// percepts, and what each left unbounded — whether comprehension starves
/// perception, on the stroke's own counters.
fn spend(s: &observe::Spend) -> Vec<String> {
    let total: u64 = s.reads.iter().sum();
    let mut out = vec![format!("  {BOLD}spend{RESET}       {} percepts, {total} reads", s.percepts)];
    for (((name, meaning), reads), unbounded) in observe::SPEND_READERS.iter().zip(s.reads).zip(s.unbounded) {
        let left = match unbounded {
            0 => String::new(),
            u => format!("  {YELLOW}unbounded {u}{RESET}"),
        };
        out.push(format!("    {name:<14} {reads:>10}{left}  {DIM}{meaning}{RESET}"));
    }
    out
}

/// PG1 — E1'S DONE-WHEN: contact cycles and those carrying holonomy; the
/// certified cells by grade with the round kept (the coboundary's agreement
/// at the grade), reversed or carried, their groups and dual frames.
fn progress(p: &observe::Progress) -> Vec<String> {
    let mut out = vec![
        format!("  {BOLD}progress{RESET}    {} percepts  kept cycles curved {}", p.percepts, p.cycles_curved),
        format!(
            "    contact cycles {}  curved {} (gauge static {})  coset closed {} mod-held {} open {}  {DIM}unread {}{RESET}",
            p.contact_cycles,
            p.contact_cycles_curved,
            p.contact_cycles_static,
            p.contact_cycles_coset[0],
            p.contact_cycles_coset[1],
            p.contact_cycles_coset[2],
            p.contact_cycles_unread
        ),
    ];
    if p.grades.is_empty() {
        out.push(format!("    {DIM}no cell certified{RESET}"));
    }
    for g in &p.grades {
        out.push(format!(
            "    grade {}  cells {}  round kept {}  held-z2 {}  {}carried {}{RESET}  group finite {} affine {} infinite {}  dual frames {}",
            g.grade,
            g.cells,
            g.round_kept,
            g.held_z2,
            if g.refused > 0 { YELLOW } else { DIM },
            g.refused,
            g.finite,
            g.affine,
            g.infinite,
            g.dual_frames
        ));
    }
    out
}

/// PH5 — A COURT'S PHASE (45.9): its last window's coordinates as it wrote
/// them, adjoined-by-closure (I1) — the one number that bears on the
/// architecture — named on its own line.
fn phase(p: &observe::Phase) -> Vec<String> {
    let adjoined = match p.adjoined_by_closure {
        0 => format!("{DIM}adjoined-by-closure 0{RESET}"),
        n => format!("{GREEN}{BOLD}adjoined-by-closure {n}{RESET}"),
    };
    vec![
        format!("  {BOLD}phase{RESET}       h {}..{}  ({} windows written)", p.h0, p.h1, p.windows),
        format!(
            "    comprehension {}/{}  congruence {}/{}  lineages {}  installed +{} -{}  expansion {}/{}",
            p.deflated, p.held, p.sections, p.strata, p.lineages, p.installed, p.retired, p.axes, p.splits
        ),
        format!("    {adjoined}"),
    ]
}

/// LD1 — THE BYTES A CARRIER CARRIES (78.22): per carrier and kind, the
/// records, their bytes, the bytes per record, and the largest. A largest
/// past the u16 wall is a form copied where a citation belongs, in red; the
/// largest of each carrier is named.
fn carriage(c: &observe::Carriage) -> Vec<String> {
    let kind_name = |carrier: u8, kind: u8| -> String {
        match carrier {
            observe::CARRIER_DELTA => match mycelium::chronometry::ActKind::from_byte(kind) {
                Some(k) => String::from_utf8_lossy(k.name()).into_owned(),
                None => format!("kind {kind}"),
            },
            observe::CARRIER_FRAME => format!("act {kind:#04x}"),
            _ => String::new(),
        }
    };
    let mut out = vec![format!("  {BOLD}carriage{RESET}    {} rows", c.rows.len())];
    for (carrier, (name, meaning)) in observe::CARRIERS.iter().enumerate() {
        let rows: Vec<&observe::CarrierRow> =c.rows.iter().filter(|r| usize::from(r.carrier) == carrier).collect();
        if rows.is_empty() {
            out.push(format!("    {name:<6} {DIM}nothing carried — {meaning}{RESET}"));
            continue;
        }
        for r in &rows {
            let colour = match r.largest > u64::from(u16::MAX) {
                true => RED,
                false => DIM,
            };
            out.push(format!(
                "    {name:<6} {:<10} records {:>8}  bytes {:>12}  per record {:>8}  {colour}largest {}{RESET}",
                kind_name(r.carrier, r.kind),
                r.records,
                r.bytes,
                r.bytes / r.records.max(1),
                r.largest
            ));
        }
    }
    out
}

/// THE ALLOCATOR OVER THE SOURCE'S OWN CLOCK, and the growth per `K`.
fn memory(m: &Memory) -> Vec<String> {
    // Mebibytes to one place, in integers: a rendering is no reason to
    // leave the exact numbers.
    let mb = |b: u64| format!("{}.{}M", b / 1_048_576, b % 1_048_576 * 10 / 1_048_576);
    let dk = i128::from(m.last_k) - i128::from(m.first_k);
    let da = i128::from(m.last_allocated) - i128::from(m.first_allocated);
    let growth = match (dk, da) {
        (0, 0) => format!("{DIM}one K sampled, nothing moved{RESET}"),
        // The source's clock did not advance and its bytes did: growth no
        // K accounts for, which is the reading, not a rate.
        (0, da) => format!("{YELLOW}{da:+} bytes allocated within one K ({} samples){RESET}", m.samples),
        (dk, _) => {
            let per = da / dk;
            match per > 0 {
                true => format!("{YELLOW}+{per} bytes allocated per K over {dk} K{RESET}"),
                false => format!("{GREEN}{per} bytes allocated per K over {dk} K{RESET}"),
            }
        }
    };
    vec![
        format!(
            "  {BOLD}memory{RESET}      allocated {} → {}  resident {} → {}  peak {}  ({} samples, K {}→{})",
            mb(m.first_allocated),
            mb(m.last_allocated),
            mb(m.first_resident),
            mb(m.last_resident),
            mb(m.peak_resident),
            m.samples,
            m.first_k,
            m.last_k
        ),
        format!("    {growth}"),
    ]
}
