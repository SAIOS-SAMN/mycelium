// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 24.3, 78.164 — THE VOICE, a mind on the mesh. The binary opens the mind's
//! boundaries — the court it speaks to, the corpus its home holds, the other
//! minds it chatters with, the seats it sits in a game — and hands the mind
//! its ticks. What the mind does with them, and in what order, is the mind's
//! own (`Entity::tick`); the binary keeps no order of work.

#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

use mycelium::court::identity::HyphaKey;
use mycelium::network::mesh;
use mycelium::sdk::{Entity, Kernel};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].as_str())
}

fn home_dir(args: &[String]) -> Result<PathBuf, String> {
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--home" | "--call" | "--mind" | "--rendezvous" => i += 2,
            // 78.145 — `--seat <domain-hex>[/<sub-origin-hex>] <seat-id>[,…] | all`.
            "--seat" => i += 3,
            a if a.starts_with("--") => return Err(format!("unknown flag {a}")),
            a => return Err(format!("unexpected argument {a}")),
        }
    }
    if let Some(p) = flag(args, "--home") {
        return Ok(PathBuf::from(p));
    }
    if let Ok(p) = std::env::var("MYCELIUM_HOME") {
        return Ok(PathBuf::from(p));
    }
    // The fleet's homes are `var/<unit>` under the repository the binary
    // runs from, never the operator's home directory.
    Ok(PathBuf::from("var").join("voice"))
}

/// The seated domain, its sub-origin where one is named, and the seats sat.
type Seated = ([u8; 32], Option<[u8; 32]>, mycelium::sdk::Sit);

/// THE SEATS `--seat` NAMES: `<domain-hex>[/<sub-origin-hex>] <seat-id>[,…]`
/// or `… all`; none where the flag is absent.
fn seats(args: &[String]) -> Result<Option<Seated>, String> {
    use mycelium::sdk::Sit;
    let Some(at) = args.iter().position(|a| a == "--seat") else { return Ok(None) };
    let (Some(d), Some(s)) = (args.get(at + 1), args.get(at + 2)) else {
        return Err("--seat takes <domain-hex>[/<sub-origin-hex>] <seat-id>[,<seat-id>…] | all".to_string());
    };
    let (d, sub) = match d.split_once('/') {
        Some((d, sub)) => (d, Some(sub)),
        None => (d.as_str(), None),
    };
    let hex = |x: &str| mycelium::court::identity::parse_hex32(x).ok_or_else(|| format!("--seat: {x} is not 32 hex bytes"));
    let domain = hex(d)?;
    let sub_origin = match sub {
        Some(x) => Some(hex(x)?),
        None => None,
    };
    let sit = match s.as_str() {
        "all" => Sit::All,
        list => Sit::Seats(list.split(',').map(|one| one.trim().parse::<u32>().map_err(|e| format!("--seat: {one} is not a seat id: {e}"))).collect::<Result<_, _>>()?),
    };
    Ok(Some((domain, sub_origin, sit)))
}

/// THE OBSERVER'S CALL (the operator 2026-10-15; `observe::live`):
/// `mycelium-client --call "<call>" [--mind HEX] [--rendezvous DIR]` asks
/// every mind at the rendezvous (beside the home, or `DIR`) whose voice opens
/// with `HEX`, prints each answer, and exits — 0 where any mind answered.
/// It starts no mind.
fn call_minds(args: &[String], home: &std::path::Path, line: &str) -> ! {
    let dir = match (flag(args, "--rendezvous"), home.parent()) {
        (Some(d), _) => PathBuf::from(d),
        (None, Some(p)) => p.join("chatter"),
        (None, None) => PathBuf::from("chatter"),
    };
    let only = flag(args, "--mind").map(str::to_lowercase);
    let Some(question) = mycelium::observe::live::encode_call(line) else {
        eprintln!("mycelium-client: the call does not write");
        std::process::exit(2)
    };
    let mut entries: Vec<(String, String)> = match std::fs::read_dir(&dir) {
        Ok(rd) => rd
            .flatten()
            .filter_map(|e| {
                let name = e.file_name().to_string_lossy().into_owned();
                let addr = std::fs::read_to_string(e.path()).ok()?.trim().to_string();
                (!name.starts_with('.') && !addr.is_empty()).then_some((name, addr))
            })
            .filter(|(name, _)| only.as_ref().is_none_or(|p| name.starts_with(p.as_str())))
            .collect(),
        Err(e) => {
            eprintln!("mycelium-client: no rendezvous at {}: {e}", dir.display());
            std::process::exit(2)
        }
    };
    entries.sort();
    let mut answered = false;
    for (voice, addr) in &entries {
        println!("── MIND {voice:.8}… at {addr}");
        match mycelium::sdk::chatter::ask(addr, &question) {
            Ok(reply) => match mycelium::observe::live::decode_lines(&reply) {
                Some(lines) => {
                    answered = true;
                    for l in lines {
                        println!("  {l}");
                    }
                }
                None => println!("  answered with no lines this build reads ({} bytes)", reply.len()),
            },
            Err(e) => println!("  did not answer: {e}"),
        }
    }
    std::process::exit(if answered { 0 } else { 2 })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let refuse = |why: String| -> ! {
        eprintln!("mycelium-client: {why}");
        std::process::exit(2)
    };
    let home = match home_dir(&args) {
        Ok(h) => h,
        Err(e) => refuse(e),
    };
    if let Some(line) = flag(&args, "--call") {
        call_minds(&args, &home, line);
    }
    let _ = std::fs::create_dir_all(&home);
    // A home whose path is not UTF-8 names no key file (rule 11).
    let key_path = home.join("voice.key");
    let Some(key_name) = key_path.to_str() else { refuse("--home path is not UTF-8 and names no key file".to_string()) };
    let voice_key = HyphaKey::load_or_mint(key_name);
    let mesh_rec = match mesh::load_or_migrate_voice(&home, &voice_key) {
        Ok(m) => m,
        Err(why) => refuse(format!("boot refused: {why}")),
    };
    let bootstrap = match mesh_rec.bootstrap.is_empty() {
        true => "127.0.0.1:7500".to_string(),
        false => mesh_rec.bootstrap.clone(),
    };
    let genome_path = home.join("genome").to_string_lossy().into_owned();
    // 78.141 — seated on its key: the solves it is paid for are signed.
    let mut mind = Entity::open_persistent(&bootstrap, Kernel::from_key(voice_key.sk.clone()), &genome_path);
    mind.set_height(mycelium::chronometry::Height::default());
    mind.discover();
    mind.keep_genome(genome_path);
    // THE BOUNDARIES: the corpus the home holds, the other minds at the
    // fleet's rendezvous beside the homes, the seats `--seat` names.
    mind.open_feed(home.clone());
    let rendezvous = match home.parent() {
        Some(p) => p.join("chatter"),
        None => PathBuf::from("chatter"),
    };
    if let Err(e) = mind.open_chatter(&rendezvous) {
        eprintln!("mycelium-client: chatter not opened: {e}");
    }
    let seated = match seats(&args) {
        Ok(s) => s,
        Err(e) => refuse(e),
    };
    if let Some((domain, sub_origin, sit)) = seated {
        mind.sit(domain, sub_origin, sit, voice_key.sk.clone());
    }
    let stop = Arc::new(AtomicBool::new(false));
    {
        let stop = Arc::clone(&stop);
        let _ = ctrlc::set_handler(move || {
            stop.store(true, Ordering::SeqCst);
            mycelium::physics::halt();
        });
    }
    while !stop.load(Ordering::SeqCst) {
        // MU-CLIENT §4.12 — the winding carrier's stroke: the mind's own
        // motion, acting at its returns.
        let wound = mind.wind(&stop);
        // X6 — a stroke that moved nothing and returned nothing admitted
        // nothing: it draws nothing and rests until its court's word lands or
        // the measured transport's park passes, never spinning a core.
        if wound.moved == [false; 4] && wound.returns == 0 {
            mind.rest();
        }
    }
    if let Err(why) = mind.save_kept_genome() {
        eprintln!("mycelium-client: genome not saved: {why}");
    }
}
