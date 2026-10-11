// §8.1 — THE DIAGNOSTIC CONSOLE: any hypha reads what a hypha holds about
// itself and its voices, and renders it whole.
//
//! `mycelium-server --console [--at ADDR] [--home DIR] [--court HEX]
//! [--reading NAME]` asks one hypha (the
//! home's own bind by default) for every reading it holds, renders it once
//! and exits. It reads no clock (law 9): a live view is the operator's
//! `watch -n 5 mycelium-server --console`, and each frame is one question.
//!
//! WHAT IT SHOWS, per court, each reading in its own coordinates (a mind
//! reports nothing about itself):
//!
//! - MEMORY — the allocator's bytes at the source's first and latest `K`,
//!   the growth per `K` between them, and the peak: a leak is growth over
//!   the source's own clock.
//! - TUNIVERSAL — FIRST, before any other reading (ACT1, 78.21(4)): every
//!   source by its height with its own `K` beside it, and the spread.
//! - CLOCK — per court: its genesis, per-peer heights and gaps, where its
//!   snapshot and last fold stand, and the three counts (acts sealed without
//!   a frame, words held without a stamp by row, rulings admitted against
//!   applied).
//! - COURT — the court's `K`, basis, treasury and the acts it served.
//! - PHASE — the court's last phase window and adjoined-by-closure (PH5).
//! - CARRIAGE — the bytes per award, per receipt delta by act kind and per
//!   frame by act tag, and the largest of each (LD1).
//!
//! CQ1 — `--court HEX` (a key or a hex prefix of it) asks one source's
//! fiber, and `--reading NAME` one kind of reading: the hypha serves only
//! the prefix asked, and the console reads nothing else.

use mycelium::observe::{self, Court, Memory, Reading, Source};
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
pub(crate) fn address(home: &Path, at: Option<&str>) -> Option<String> {
    if let Some(a) = at {
        return Some(a.to_string());
    }
    let key = mycelium::court::identity::HyphaKey::load_only(&home.join("hypha.key").to_string_lossy())?;
    let bytes = std::fs::read(home.join("mesh")).ok()?;
    let (rec, _) = mycelium::network::mesh::MeshRecord::open_layout(&bytes, &key.vk_bytes())?;
    rec.bind
}

/// CQ1 — WHAT THE OPERATOR ASKS: one court by its key or a hex prefix of it,
/// and one kind of reading by its name.
pub(crate) struct Ask<'a> {
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
        let (kind, hex) = match self.court {
            Some(c) => (observe::SOURCE_COURT, c),
            None => return Ok((Vec::new(), tag)),
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
        });
        for r in readings {
            lines.extend(match r {
                Reading::Court(c) => court(c),
                Reading::Memory(m) => memory(m),
                Reading::Replication(r) => replication(r),
                Reading::Adjudication(a) => adjudication(a),
                Reading::Phase(p) => phase(p),
                Reading::Carriage(c) => carriage(c),
                Reading::Clock(c) => clock(c),
                Reading::Economy(e) => economy(e),
                Reading::Acts(a) => a.lines().into_iter().map(|l| format!("  {l}")).collect(),
                Reading::Carriers(c) => c.lines().into_iter().map(|l| format!("  {l}")).collect(),
                Reading::Relay(r) => r.lines().into_iter().map(|l| format!("  {l}")).collect(),
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
            None => format!("  {name}  {DIM}no clock carried{RESET}"),
        });
        // 78.152 — the court's quorum-final point, its keepers, its residual.
        if let Some(c) = readings.iter().find_map(|r| match r {
            Reading::Court(c) => Some(c),
            _ => None,
        }) {
            lines.extend(standing(&c.tuniversal));
        }
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

/// 78.152 TU-3, TU-4, TU-7 — A COURT'S TUNIVERSAL STANDING: the door's
/// quorum-final point written whole (`TPoint::print`), its advances, each
/// keeper with its newest verified tick and its own final point, and the
/// residual — each edge's exact lane difference and the triangle's holonomy.
fn standing(s: &observe::TStanding) -> Vec<String> {
    let lanes = |d: &[i128; 4]| {
        let [a, b, c, e] = d;
        format!("{a:+}:{b:+}:{c:+}:{e:+}")
    };
    let mut out = vec![
        match &s.final_point {
            Some(p) => format!("    final {}", p.print()),
            None => format!("    {DIM}none final yet{RESET}"),
        },
        format!("    advances {}", s.advances),
    ];
    for k in &s.keepers {
        let own = match k.own {
            true => " (own)",
            false => "",
        };
        let newest = match &k.newest {
            Some(t) => {
                let [ls, lg, lt, lh] = t.lanes;
                format!("newest h {} {ls}:{lg}:{lt}:{lh} K {}", t.h, t.k)
            }
            None => format!("{DIM}no reading held{RESET}"),
        };
        let fin = match &k.final_of {
            Some(p) => format!("final {}", p.local()),
            None => format!("{DIM}final none{RESET}"),
        };
        out.push(format!("    keeper {}{own}  {newest}  {fin}", short(&k.key)));
    }
    for e in &s.residual.edges {
        out.push(format!("    residual {}→{} h {} Δ {}", short(&e.from), short(&e.to), e.h, lanes(&e.delta)));
    }
    out.push(match &s.residual.holonomy {
        Some(d) => format!("    holonomy {}", lanes(d)),
        None => format!("    {DIM}holonomy unmeasured{RESET}"),
    });
    out
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
        format!(
            "  {BOLD}serving{RESET}     {}",
            observe::SERVE_AXES.iter().zip(c.serve).map(|(axis, n)| format!("{axis} {n}")).collect::<Vec<_>>().join(" · ")
        ),
    ]
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
