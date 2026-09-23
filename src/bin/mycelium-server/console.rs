// §8.1 — THE DIAGNOSTIC CONSOLE: any hypha reads what a hypha holds about
// itself and its voices, and renders it whole.
//
//! `mycelium-server --console [--at ADDR] [--home DIR]` asks one hypha (the
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
//! - COURT — the court's `K`, basis, treasury and the acts it served.

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

/// ASK, RENDER, EXIT. Exit 0 with a rendering, 2 where no hypha answered —
/// the rendering then says why, so a `watch` shows the refusal in place.
pub(crate) fn run(home: &Path, at: Option<&str>) -> ! {
    let Some(addr) = address(home, at) else {
        println!("{RED}no address to ask{RESET}: pass --at ADDR, or --home a home whose mesh names its bind");
        std::process::exit(2)
    };
    let Some((reply, _)) = crate::transport::tell_peer(&addr, observe::encode_query(&[])) else {
        println!("{RED}{addr} did not answer{RESET}");
        std::process::exit(2)
    };
    let Some((total, held)) = observe::decode_reply(&reply) else {
        println!("{RED}{addr} answered with no reading this build reads{RESET} ({} bytes)", reply.len());
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

pub(crate) fn render(addr: &str, total: u32, held: &[observe::Held]) -> Vec<String> {
    let sources = by_source(held);
    let mut lines = vec![format!(
        "{CYAN}{BOLD}═══ MYCELIUM OBSERVER ═══{RESET}  {addr}  {DIM}{} readings of {total} held, {} sources{RESET}",
        held.len(),
        sources.len()
    )];
    if held.len() < total as usize {
        lines.push(format!("{YELLOW}the reply held {} of {total}: the frame budget cut it{RESET}", held.len()));
    }
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
            });
        }
    }
    lines
}

fn court(c: &Court) -> Vec<String> {
    let served: Vec<String> = c.served.iter().map(|(kind, n)| format!("{kind:02x}×{n}")).collect();
    vec![
        format!("  {BOLD}standing{RESET}    K {}  basis {}  treasury {}", c.k, c.basis, c.treasury),
        format!("  {BOLD}served{RESET}      {}", match served.is_empty() {
            true => format!("{DIM}nothing yet{RESET}"),
            false => served.join(" "),
        }),
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
    let used = |planes: &[usize]| planes.iter().filter(|&&b| p.count(b) > 0).count();
    let grid = cga::E1 | cga::E2;
    let turned: Vec<usize> = harmonic.iter().copied().filter(|&b| p.count(b) > 0).collect();
    let verdict = match (p.read, turned.as_slice()) {
        (0, _) => format!("{DIM}no transition read yet{RESET}"),
        (_, []) => format!("{YELLOW}nothing turned: every transition was a boost or the identity{RESET}"),
        (_, [only]) if *only == grid => format!(
            "{RED}{BOLD}FLATTENED{RESET}{RED} — every turn stands on the sensor's plane {}; {} other harmonic planes declared and never turned{RESET}",
            cga::plane_name(grid),
            harmonic.len() - 1
        ),
        _ => format!("{GREEN}turns on {} of {} harmonic planes{RESET}", turned.len(), harmonic.len()),
    };
    vec![
        format!(
            "  {BOLD}dimensions{RESET}  n {}  Cl({},1)  read {}  coupled {}  harmonic {}/{}  luminal {}/{}",
            p.n,
            u32::from(p.n) + 1,
            p.read,
            p.coupled,
            used(&harmonic),
            harmonic.len(),
            used(&luminal),
            luminal.len()
        ),
        format!("    harmonic  {}", row(&harmonic)),
        format!("    luminal   {}", row(&luminal)),
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
    ]
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
