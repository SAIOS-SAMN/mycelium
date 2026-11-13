// §8.1 — THE DIAGNOSTIC CONSOLE: any hypha reads what a hypha holds about
// itself and its voices, and renders it whole.
//
//! `mycelium-server --console [--at ADDR] [--home DIR] [--court HEX]
//! [--reading NAME]` asks one hypha (the
//! home's own bind by default) for every reading it holds, renders it once
//! and exits. It reads no clock (law 9): a live view is the operator's
//! `watch -n 5 mycelium-server --console`, and each refresh is one question.
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
//!   a measure, words held without a stamp by row, rulings admitted against
//!   applied).
//! - COURT — the court's `K`, basis, treasury and the acts it served.
//! - PHASE — the court's last phase window and adjoined-by-closure (PH5).
//! - CARRIAGE — the bytes per award, per receipt delta by act kind and per
//!   envelope by act tag, and the largest of each (LD1).
//!
//! CQ1 — `--court HEX` (a key or a hex prefix of it) asks one source's
//! fibre, and `--reading NAME` one kind of reading: the hypha serves only
//! the prefix asked, and the console reads nothing else.

use mycelium::observe::{self, CourtReading, Memory, Reading, Source};
use std::path::Path;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const CYAN: &str = "\x1b[96m";
const GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const RED: &str = "\x1b[91m";

/// The endpoint the console asks: `--at`, or the bind this home's own mesh
/// record names. `None` where neither reads.
pub(crate) fn endpoint(home: &Path, at: Option<&str>) -> Option<String> {
    if let Some(a) = at {
        return Some(a.to_string());
    }
    let key = mycelium::court::identity::HyphaKey::load_only(&home.join("hypha.key").to_string_lossy())?;
    let bytes = std::fs::read(home.join("mesh")).ok()?;
    let (rec, _) = mycelium::network::mesh::MeshRecord::open_layout(&bytes, &key.verifying_bytes())?;
    rec.bind
}

/// CQ1 — WHAT THE OPERATOR ASKS: one court by its key or a hex prefix of it,
/// and one kind of reading by its name.
pub(crate) struct Ask<'a> {
    pub(crate) court: Option<&'a str>,
    pub(crate) reading: Option<&'a str>,
    pub(crate) json: bool,
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

fn fail(json: bool, plain: &str, msg: &str) -> ! {
    if json {
        if let Ok(s) = serde_json::to_string_pretty(&serde_json::json!({ "error": msg })) {
            println!("{s}");
        }
    } else {
        println!("{plain}");
    }
    std::process::exit(2)
}

/// ASK, RENDER, EXIT. Exit 0 with a rendering, 2 where no hypha answered —
/// the rendering then says why, so a `watch` shows the refusal in place.
/// CQ1 — the question is the prefix the operator names; the hypha serves
/// only that fibre.
pub(crate) fn run(home: &Path, at: Option<&str>, ask: &Ask) -> ! {
    let (prefix, keep) = match ask.prefix() {
        Ok(p) => p,
        Err(why) => fail(ask.json, &format!("{RED}{why}{RESET}"), &why),
    };
    let Some(endpoint) = endpoint(home, at) else {
        fail(
            ask.json,
            &format!("{RED}no address to ask{RESET}: pass --at ADDR, or --home a home whose mesh names its bind"),
            "no address to ask: pass --at ADDR, or --home a home whose mesh names its bind",
        )
    };
    let Some(query) = observe::encode_query(&prefix) else {
        fail(
            ask.json,
            &format!("{RED}the prefix passes its length field{RESET}"),
            "the prefix passes its length field",
        )
    };
    // THE OPERATOR'S ONE ASK READS TO THE STREAM'S END (D2.7): nothing is
    // measured before it, so no count bounds it — the reserve it waited was
    // burst capacity in reads, never a round trip — and the process ends
    // with the peer.
    let Some((reply, _)) = crate::transport::tell_peer_within(&endpoint, query, crate::transport::Stall::Beyond) else {
        fail(
            ask.json,
            &format!("{RED}{endpoint} did not answer{RESET}"),
            &format!("{endpoint} did not answer"),
        )
    };
    let Some((total, mut held)) = observe::decode_reply(&reply) else {
        fail(
            ask.json,
            &format!("{RED}{endpoint} answered with no reading this build reads{RESET} ({} bytes)", reply.len()),
            &format!("{endpoint} answered with no reading this build reads ({} bytes)", reply.len()),
        )
    };
    // What the body budget cut stays said after the kind is kept.
    // In the count's own carrier: `held` is the reply's entries, and the
    // arithmetic is exact or the line says it could not count.
    let cut = u64::from(total).abs_diff(held.len() as u64);
    // The standing's age is kept beside whatever kind was asked for.
    if let Some(t) = keep {
        held.retain(|(k, _)| Source::of_key(k).is_some_and(|(_, tag)| tag == t || tag == observe::TAG_ASKED));
    }
    let Some(total) = (held.len() as u64).checked_add(cut) else {
        fail(
            ask.json,
            &format!("{RED}{endpoint} answered a total past any count{RESET}"),
            &format!("{endpoint} answered a total past any count"),
        )
    };
    if ask.json {
        if let Ok(s) = serde_json::to_string_pretty(&render_json(&endpoint, total, &held, &[])) {
            println!("{s}");
        }
    } else {
        for line in render(&endpoint, total, &held) {
            println!("{line}");
        }
    }
    std::process::exit(0)
}

/// MU-CLIENT §5.3 — EVERY MIND AT THE RENDEZVOUS, ASKED ON ITS OWN POST, its
/// readings rendered as a court's are. A mind that does not answer is said
/// so, in place; exit 2 where none did.
pub(crate) fn run_minds(dir: &Path, ask: &Ask) -> ! {
    use mycelium::anastome::dialogue::chatter;
    let tag = match ask.reading {
        None => None,
        Some(name) => observe::READING_NAMES.iter().find(|(n, _)| *n == name).map(|&(_, t)| t),
    };
    let Some(query) = observe::encode_query(&[]) else {
        fail(ask.json, &format!("{RED}the question does not write{RESET}"), "the question does not write")
    };
    let mut entries: Vec<(String, String)> = match std::fs::read_dir(dir) {
        Ok(rd) => rd
            .flatten()
            .filter_map(|e| {
                let name = e.file_name().to_string_lossy().into_owned();
                let endpoint = std::fs::read_to_string(e.path()).ok()?.trim().to_string();
                (!name.starts_with('.') && !endpoint.is_empty()).then_some((name, endpoint))
            })
            .collect(),
        Err(e) => fail(
            ask.json,
            &format!("{RED}no rendezvous at {}{RESET}: {e}", dir.display()),
            &format!("no rendezvous at {}: {e}", dir.display()),
        ),
    };
    entries.sort();
    let mut held: Vec<observe::KeyedReading> = Vec::new();
    let mut silent: Vec<String> = Vec::new();
    let mut silent_plain: Vec<String> = Vec::new();
    for (voice, endpoint) in &entries {
        match chatter::ask(endpoint, &query) {
            Ok(reply) => match observe::decode_reply(&reply) {
                Some((_, readings)) => held.extend(readings),
                None => {
                    let msg = format!("mind {voice:.8}… at {endpoint} answered with no reading this build reads ({} bytes)", reply.len());
                    silent_plain.push(format!("{RED}{msg}{RESET}"));
                    silent.push(msg);
                }
            },
            Err(e) => {
                let msg = format!("mind {voice:.8}… at {endpoint} did not answer: {e}");
                silent_plain.push(format!("{RED}{msg}{RESET}"));
                silent.push(msg);
            }
        }
    }
    if let Some(t) = tag {
        held.retain(|(k, _)| Source::of_key(k).is_some_and(|(_, g)| g == t));
    }
    let total = held.len() as u64;
    if ask.json {
        if let Ok(s) = serde_json::to_string_pretty(&render_json(&dir.display().to_string(), total, &held, &silent)) {
            println!("{s}");
        }
    } else {
        for line in render(&dir.display().to_string(), total, &held).into_iter().chain(silent_plain) {
            println!("{line}");
        }
    }
    let answered = entries.len() > silent.len();
    std::process::exit(if answered { 0 } else { 2 })
}

fn short(key: &[u8; 32]) -> String {
    format!("{:02x}{:02x}{:02x}{:02x}…", key[0], key[1], key[2], key[3])
}

/// Every source's readings gathered under it, in key order (courts first,
/// by the source kind the key opens with).
fn by_source(held: &[observe::KeyedReading]) -> Vec<(Source, Vec<Reading>)> {
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

pub(crate) fn render(endpoint: &str, total: u64, held: &[observe::KeyedReading]) -> Vec<String> {
    let sources = by_source(held);
    let mut lines = vec![format!(
        "{CYAN}{BOLD}═══ MYCELIUM OBSERVER ═══{RESET}  {endpoint}  {DIM}{} readings of {total} held, {} sources{RESET}",
        held.len(),
        sources.len()
    )];
    if (held.len() as u64) < total {
        lines.push(format!("{YELLOW}the reply held {} of {total}: the body budget cut it{RESET}", held.len()));
    }
    lines.extend(tuniversal(&sources));
    for (source, readings) in &sources {
        lines.push(String::new());
        lines.push(match source {
            Source::Court(vk) => format!("{CYAN}{BOLD}── COURT {}{RESET}", short(vk)),
            Source::Mind(vk) => format!("{GREEN}{BOLD}── MIND {}{RESET}", short(vk)),
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
                Reading::Mind(m) => m.lines().into_iter().map(|l| format!("  {l}")).collect(),
                Reading::Agreements(a) => a.lines().into_iter().map(|l| format!("  {l}")).collect(),
                Reading::Dispatched(d) => d.lines().into_iter().map(|l| format!("  {l}")).collect(),
                Reading::Judged(j) => j.lines().into_iter().map(|l| format!("  {l}")).collect(),
                Reading::Boundary(b) => b.lines().into_iter().map(|l| format!("  {l}")).collect(),
                Reading::Channels(c) => c.lines().into_iter().map(|l| format!("  {l}")).collect(),
                Reading::Asked(n) => vec![match n {
                    0 | 1 => format!("  {DIM}standing published since the last question{RESET}"),
                    n => format!(
                        "  {YELLOW}standing STALE: {} questions since the court last published — every stroke reading below is older{RESET}",
                        n
                    ),
                }],
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
            Source::Mind(vk) => format!("mind {}", short(vk)),
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
        // 78.152 — the court's quorum-final point, its keepers, its residue.
        if let Some(c) = readings.iter().find_map(|r| match r {
            Reading::Court(c) => Some(c),
            _ => None,
        }) {
            lines.extend(standing(&c.tuniversal));
        }
    }
    #[expect(
        clippy::disallowed_methods,
        reason = "78.21(1) — the mesh's spread IS the range of heads, max − min: a reading of the range, and nothing is chosen by it"
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
/// residue — each edge's exact lane difference and the triangle's holonomy.
fn standing(s: &observe::TStanding) -> Vec<String> {
    // Each signed lane exact, as `n/d` in lowest terms with its sign.
    let lanes = |d: &[mycelium::core::Q; 4]| {
        let signed = |q: &mycelium::core::Q| match num_traits::Signed::is_negative(q) {
            true => format!("{q}"),
            false => format!("+{q}"),
        };
        d.iter().map(signed).collect::<Vec<_>>().join(":")
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
                let [ls, lg, lt, lh] = t.lanes.lanes();
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
    for e in &s.residue.edges {
        out.push(format!("    residue {}→{} h {} Δ {}", short(&e.from), short(&e.to), e.h, lanes(&e.delta)));
    }
    out.push(match &s.residue.holonomy {
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
            format!("  {BOLD}{:<12}{RESET}paid {}  burned {}", arm.name(), e.paid[i], e.burned[i])
        })
        .collect();
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
                    let why: Vec<String> = mycelium::chronometry::RulingRefusal::ALL
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
                format!("  {BOLD}unmeasured{RESET}  {}", flag(c.unmeasured > 0, format!("{} acts sealed on a word nothing measured", c.unmeasured))),
                format!("  {BOLD}unstamped{RESET}   {}", flag(ub + up + ul + uc + uf > 0, unstamped)),
                format!("  {BOLD}economy{RESET}     {rulings}"),
            ]
        }
    }
}

fn court(c: &CourtReading) -> Vec<String> {
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
        format!("  {BOLD}digests by{RESET}  {}", match c.digest_askers.is_empty() {
            true => format!("{DIM}nobody yet{RESET}"),
            false => c.digest_askers.iter().map(|(v, n)| format!("{:02x}{:02x}{:02x}{:02x}…×{n}", v[0], v[1], v[2], v[3])).collect::<Vec<_>>().join(" "),
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
    use observe::{replication, REPLICATION_AXES};
    let mut out = vec![format!("  {BOLD}replication{RESET} {} signers heard", r.signers.len())];
    for (vk, axes) in &r.signers {
        let admitted = axes[replication::PARENT] + axes[replication::BACKFILL];
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

/// ADJUDICATION BY KIND (CV5): the verdicts, and the refusals by each one's
/// own kind's name.
fn adjudication(a: &observe::Adjudication) -> Vec<String> {
    let refused: Vec<String> = a
        .refused
        .iter()
        .map(|&(kind, n)| match mycelium::transducer::sentence::RefusalKind::of(kind) {
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
        // MU-CLIENT §4.13 — the vertex-body refusals by the fault each proof
        // names, and by whether the vertex lifted a versor on the chart.
        format!(
            "    vertex body by fault  {}  · versor lifted {}  none {}",
            observe::BODY_FAULTS.iter().zip(a.body_faults).map(|(f, n)| format!("{f} {n}")).collect::<Vec<_>>().join("  "),
            a.body_lifted[1],
            a.body_lifted[0]
        ),
    ]
}

/// PH5 — A COURT'S PHASE (45.9): its last window's coordinates as it wrote
/// them, adjoined-by-closure (I1) — the one number that bears on the
/// architecture — named on its own line.
fn phase(p: &observe::CourtStage) -> Vec<String> {
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
/// records, the records' bytes, the bytes per record, and the largest. A largest
/// past the u16 wall is a form copied where a citation belongs, in red; the
/// largest of each carrier is named.
fn carriage(c: &observe::CarrierTally) -> Vec<String> {
    let kind_name = |carrier: u8, kind: u8| -> String {
        match carrier {
            observe::CARRIER_DELTA => match mycelium::chronometry::ActKind::of_sealed(kind) {
                Some(k) => String::from_utf8_lossy(k.name()).into_owned(),
                None => format!("kind {kind}"),
            },
            observe::CARRIER_DELIMITED => format!("act {kind:#04x}"),
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

fn hex_key(key: &[u8; 32]) -> String {
    let mut out = String::with_capacity(64);
    for b in key {
        use std::fmt::Write;
        let _ = write!(out, "{b:02x}");
    }
    out
}

/// Machine-readable JSON rendering of observer readings (`--console --json`).
pub(crate) fn render_json(endpoint: &str, total: u64, held: &[observe::KeyedReading], silent: &[String]) -> serde_json::Value {
    let sources: Vec<serde_json::Value> = by_source(held)
        .into_iter()
        .map(|(source, readings)| {
            let (kind, key) = match source {
                Source::Court(vk) => ("court", vk),
                Source::Mind(vk) => ("mind", vk),
            };
            serde_json::json!({
                "kind": kind,
                "key": hex_key(&key),
                "short": short(&key),
                "readings": readings.iter().map(reading_json).collect::<Vec<_>>(),
            })
        })
        .collect();
    serde_json::json!({
        "endpoint": endpoint,
        "total": total,
        "held": held.len(),
        "sources": sources,
        "silent": silent,
    })
}

fn reading_json(r: &Reading) -> serde_json::Value {
    use serde_json::json;
    match r {
        Reading::Memory(m) => json!({
            "kind": "memory",
            "first_allocated": m.first_allocated,
            "last_allocated": m.last_allocated,
            "first_resident": m.first_resident,
            "last_resident": m.last_resident,
            "peak_resident": m.peak_resident,
            "samples": m.samples,
            "first_k": m.first_k,
            "last_k": m.last_k,
        }),
        Reading::Court(c) => json!({
            "kind": "court",
            "k": c.k,
            "basis": c.basis,
            "treasury": c.treasury,
            "reopened": c.reopened,
            "torsion_words": c.torsion_words,
            "room_broken": c.room_broken,
            "room_overflowed": c.room_overflowed,
            "overflowed": c.overflowed,
            "served": c.served.iter().map(|(kind, count)| json!({ "kind": kind, "count": count })).collect::<Vec<_>>(),
            "digest_askers": c.digest_askers.iter().map(|(v, count)| json!({ "key": hex_key(v), "count": count })).collect::<Vec<_>>(),
            "folds": observe::FOLD_OUTCOMES.iter().zip(c.folds).map(|(outcome, count)| json!({ "outcome": outcome, "count": count })).collect::<Vec<_>>(),
            "serve": observe::SERVE_AXES.iter().zip(c.serve).map(|(axis, count)| json!({ "axis": axis, "count": count })).collect::<Vec<_>>(),
            "tuniversal": {
                "final_point": c.tuniversal.final_point.as_ref().map(|p| p.print()),
                "advances": c.tuniversal.advances,
                "keepers": c.tuniversal.keepers.len(),
                "edges": c.tuniversal.residue.edges.len(),
            },
        }),
        Reading::Clock(observe::Clock::Court(c)) => json!({
            "kind": "clock",
            "head": c.head,
            "k": c.k,
            "genesis": c.genesis.map(|(id, epoch)| json!({ "id": hex_key(&id), "epoch": epoch })),
            "peers": c.peers.iter().map(|(vk, height, gap)| json!({ "key": hex_key(vk), "height": height, "gap": gap })).collect::<Vec<_>>(),
            "persisted": c.persisted,
            "folded": c.folded,
            "unmeasured": c.unmeasured,
            "unstamped": c.unstamped,
        }),
        Reading::Economy(e) => json!({
            "kind": "economy",
            "arms": mycelium::economy::Arm::ALL.iter().map(|arm| {
                let i = arm.index();
                json!({
                    "arm": arm.name(),
                    "paid": format!("{:?}", e.paid[i]),
                    "burned": format!("{:?}", e.burned[i]),
                })
            }).collect::<Vec<_>>(),
            "tuition": { "rows": e.tuition.rows, "mu": format!("{:?}", e.tuition.mu) },
            "accounts": e.accounts.iter().map(|a| json!({
                "who": hex_key(&a.who),
                "balance": format!("{:?}", a.balance),
                "bond": format!("{:?}", a.bond),
                "held": format!("{:?}", a.held),
            })).collect::<Vec<_>>(),
        }),
        Reading::Phase(p) => json!({
            "kind": "phase",
            "h0": p.h0,
            "h1": p.h1,
            "windows": p.windows,
            "deflated": p.deflated,
            "held": p.held,
            "sections": p.sections,
            "strata": p.strata,
            "lineages": p.lineages,
            "installed": p.installed,
            "retired": p.retired,
            "axes": p.axes,
            "splits": p.splits,
            "adjoined_by_closure": p.adjoined_by_closure,
        }),
        Reading::Adjudication(a) => json!({
            "kind": "adjudication",
            "ratified": a.ratified,
            "stood": a.stood,
            "awaiting": a.awaiting,
            "deferred": a.deferred,
            "refused": a.refused.iter().map(|&(kind, count)| json!({
                "kind": kind,
                "label": mycelium::transducer::sentence::RefusalKind::of(kind).map(|k| k.label()),
                "count": count,
            })).collect::<Vec<_>>(),
            "obstructed": mycelium::transducer::sentence::ObstructionKind::ALL.iter().zip(a.obstructed).map(|(k, count)| json!({ "kind": k.label(), "count": count })).collect::<Vec<_>>(),
            "body_faults": observe::BODY_FAULTS.iter().zip(a.body_faults).map(|(fault, count)| json!({ "fault": fault, "count": count })).collect::<Vec<_>>(),
            "body_lifted": a.body_lifted,
        }),
        Reading::Replication(r) => json!({
            "kind": "replication",
            "signers": r.signers.iter().map(|(vk, axes)| json!({ "key": hex_key(vk), "axes": axes.as_slice() })).collect::<Vec<_>>(),
            "unattributed": r.unattributed.as_slice(),
        }),
        Reading::Carriage(c) => json!({
            "kind": "carriage",
            "rows": c.rows.iter().map(|row| json!({
                "carrier": row.carrier,
                "kind": row.kind,
                "records": row.records,
                "bytes": row.bytes,
                "largest": row.largest,
            })).collect::<Vec<_>>(),
        }),
        Reading::Asked(n) => json!({ "kind": "asked", "count": n }),
        Reading::Acts(a) => json!({ "kind": "acts", "lines": a.lines() }),
        Reading::Carriers(c) => json!({ "kind": "carriers", "lines": c.lines() }),
        Reading::Relay(r) => json!({ "kind": "relay", "lines": r.lines() }),
        Reading::Mind(m) => json!({ "kind": "mind", "lines": m.lines() }),
        Reading::Agreements(a) => json!({ "kind": "agreements", "lines": a.lines() }),
        Reading::Dispatched(d) => json!({ "kind": "dispatched", "lines": d.lines() }),
        Reading::Judged(j) => json!({ "kind": "judged", "lines": j.lines() }),
        Reading::Boundary(b) => json!({ "kind": "boundary", "lines": b.lines() }),
        Reading::Channels(c) => json!({ "kind": "channels", "lines": c.lines() }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_json_serializes_court_and_memory_readings() {
        let vk = [0xABu8; 32];
        let src = Source::Court(vk);
        let mem = Memory {
            first_allocated: 1024,
            last_allocated: 2048,
            first_resident: 4096,
            last_resident: 4096,
            peak_resident: 8192,
            samples: 2,
            first_k: 1,
            last_k: 5,
        };
        let held = vec![
            (src.key(observe::TAG_MEMORY), Reading::Memory(mem)),
            (src.key(observe::TAG_ASKED), Reading::Asked(1)),
        ];
        let val = render_json("127.0.0.1:8500", 2, &held, &[]);
        assert_eq!(val["endpoint"], "127.0.0.1:8500");
        assert_eq!(val["total"], 2);
        assert_eq!(val["held"], 2);
        assert_eq!(val["sources"][0]["kind"], "court");
        assert_eq!(val["sources"][0]["key"], "ab".repeat(32));
        assert_eq!(val["sources"][0]["readings"][0]["kind"], "memory");
        assert_eq!(val["sources"][0]["readings"][0]["last_k"], 5);
        assert_eq!(val["sources"][0]["readings"][1]["kind"], "asked");
        assert_eq!(val["sources"][0]["readings"][1]["count"], 1);
    }
}
