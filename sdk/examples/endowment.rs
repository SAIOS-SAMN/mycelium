// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// READ A UNIVERSE'S ENDOWMENT off a hypha: the grants made to a domain,
// their μ and whether its ledger applied each, and the nuclei it holds,
// read through the paged READ-ENDOWMENT (#332). Read-only.
//
//! `endowment <home> <hypha address> <domain hex 64>`

use mycelium_sdk::house::read_endowment;
use mycelium_sdk::prelude::*;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let (Some(home), Some(endpoint), Some(hex)) = (args.get(1), args.get(2), args.get(3)) else {
        eprintln!("usage: endowment <home> <hypha address> <domain hex>");
        return ExitCode::from(2);
    };
    let bytes: Option<Vec<u8>> = (0..hex.len()).step_by(2).map(|i| hex.get(i..i + 2).and_then(|b| u8::from_str_radix(b, 16).ok())).collect();
    let Some(domain) = bytes.and_then(|b| <[u8; 32]>::try_from(b).ok()) else {
        eprintln!("a domain is 64 hex digits");
        return ExitCode::from(2);
    };
    let (voice, _) = mycelium_sdk::voice::load_or_mint(&format!("{home}/voice.key"));
    let session = match mycelium_sdk::session::NoiseHome::open(std::path::Path::new(home)) {
        Ok(n) => Session::open(endpoint, voice).with_noise(n),
        Err(_) => Session::open(endpoint, voice),
    };
    let Some(r) = read_endowment(domain, |act| session.speak(act).ok()) else {
        eprintln!("{endpoint}: no reading");
        return ExitCode::from(1);
    };
    println!("earned {}", r.earned);
    println!("founding {}", r.founding);
    for g in &r.grants {
        println!("grant {}… founding {} applied {} μ {}", g.manifest.iter().take(4).map(|b| format!("{b:02x}")).collect::<String>(), g.founding, g.applied, g.mu);
    }
    let planes = r.nuclei.iter().filter(|(n, _)| !n.planes.is_empty()).count();
    let rest = r.nuclei.iter().filter(|(n, _)| n.at_rest()).count();
    println!("nuclei {} entries, {} with planes, {} at rest", r.nuclei.len(), planes, rest);
    ExitCode::SUCCESS
}
