// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// READ A HANDOFF'S STANDING off a hypha: whether a mouth (or any handoff)
// is offered, accepted or returned, read from the court that holds it.
// Read-only.
//
//! `handoff <home> <hypha address> <handoff id hex 64>`

use mycelium_sdk::house::{decode_read_reply, decode_tube_reading, encode_handoff_read, encode_tube_read};
use mycelium_sdk::prelude::*;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let (Some(home), Some(endpoint), Some(hex)) = (args.get(1), args.get(2), args.get(3)) else {
        eprintln!("usage: handoff <home> <hypha address> <handoff id hex>");
        return ExitCode::from(2);
    };
    let bytes: Option<Vec<u8>> = (0..hex.len()).step_by(2).map(|i| hex.get(i..i + 2).and_then(|b| u8::from_str_radix(b, 16).ok())).collect();
    let Some(id) = bytes.and_then(|b| <[u8; 32]>::try_from(b).ok()) else {
        eprintln!("a handoff id is 64 hex digits");
        return ExitCode::from(2);
    };
    let (voice, _) = mycelium_sdk::voice::load_or_mint(&format!("{home}/voice.key"));
    let session = match mycelium_sdk::session::NoiseHome::open(std::path::Path::new(home)) {
        Ok(n) => Session::open(endpoint, voice).with_noise(n),
        Err(_) => Session::open(endpoint, voice),
    };
    let reply = match session.speak(encode_handoff_read(id)) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{endpoint}: {e}");
            return ExitCode::from(1);
        }
    };
    match decode_read_reply(&reply) {
        Some(Some((state, a, b))) => println!("state {state:?} · sides consumed {a} {b}"),
        Some(None) => println!("no handoff of that id among offers and conversions"),
        None => {
            eprintln!("{} bytes that do not decode as a handoff reading", reply.len());
            return ExitCode::from(1);
        }
    }
    // A mouth is held as a tube: read its tube too.
    match session.speak(encode_tube_read(id)).ok().and_then(|r| decode_tube_reading(&r)) {
        Some(Some(t)) => println!("tube held: {t:?}"),
        Some(None) => println!("no tube of that id"),
        None => println!("the tube reading does not decode"),
    }
    ExitCode::SUCCESS
}
