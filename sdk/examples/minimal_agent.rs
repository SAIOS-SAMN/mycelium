//! Minimal standalone agent walk-through on `mycelium-sdk` (#285):
//!
//! 1. Bootstrap an agent identity (`Voice`) and `Session::open`.
//! 2. Connect over Noise (`NoiseHome::open` + `Session::with_noise`) to a running hypha (default `127.0.0.1:8500`).
//! 3. Read the court's L0 law register (`ACT_LAWS`).
//! 4. Request a task quotation (`ACT_QUOTE`).
//!
//! Usage: `minimal_agent [voice_home] [hypha_addr]`

use mycelium_sdk::laws;
use mycelium_sdk::prelude::*;
use mycelium_sdk::session::{decode_quote_reply, deferral_of, encode_quote_request, read_deferral, NoiseHome};
use mycelium_sdk::voice::{hex32, load_or_mint};
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let home = match args.get(1) {
        Some(h) => h.as_str(),
        None => "minimal_agent_home",
    };
    let endpoint = match args.get(2) {
        Some(e) => e.as_str(),
        None => "127.0.0.1:8500",
    };

    if let Err(e) = std::fs::create_dir_all(home) {
        eprintln!("{home}: {e}");
        return ExitCode::from(2);
    }

    // 1. Bootstrap the agent's Ed25519 identity.
    let (voice, _key) = load_or_mint(&format!("{home}/voice.key"));
    println!("voice {}", hex32(&voice.id()));

    // 2. Open a Noise-authenticated session with the hypha.
    let noise = match NoiseHome::open(Path::new(home)) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{home}: failed to open Noise home: {e}");
            return ExitCode::from(2);
        }
    };
    let session = Session::open(endpoint, voice.clone()).with_noise(noise);

    // 3. Read the court's current L0 law register (`ACT_LAWS` mode 0).
    let law_req = laws::encode_register_request();
    match session.speak(law_req) {
        Ok(reply) => {
            if let Some(d) = read_deferral(&reply) {
                println!("ACT_LAWS deferred (pool full, deficit {:?})", d.deficit);
                return ExitCode::SUCCESS;
            }
            match laws::decode_register_reply(&reply) {
                Some(served) => println!("ACT_LAWS: version {}, {} laws served", laws::LAW_VERSION, served.len()),
                None => {
                    eprintln!("ACT_LAWS: reply did not decode at LAW_VERSION {}", laws::LAW_VERSION);
                    return ExitCode::from(1);
                }
            }
        }
        Err(e) => {
            if let Some(d) = deferral_of(&e) {
                println!("ACT_LAWS deferred (deficit {:?})", d.deficit);
                return ExitCode::SUCCESS;
            }
            eprintln!("ACT_LAWS failed: {e}");
            return ExitCode::from(1);
        }
    }

    // 4. Request a task quotation (`ACT_QUOTE`).
    let Some(quote_req) = encode_quote_request(&voice.id()) else {
        eprintln!("failed to encode ACT_QUOTE request");
        return ExitCode::from(1);
    };
    match session.speak(quote_req) {
        Ok(reply) => {
            if let Some(d) = read_deferral(&reply) {
                println!("ACT_QUOTE deferred (pool full, deficit {:?})", d.deficit);
                return ExitCode::SUCCESS;
            }
            match decode_quote_reply(&reply) {
                Some((k, _head, _basis, items)) => {
                    println!("ACT_QUOTE: K {k}, {} quoted items", items.len());
                }
                None => {
                    eprintln!("ACT_QUOTE: reply did not decode ({} bytes)", reply.len());
                    return ExitCode::from(1);
                }
            }
        }
        Err(e) => {
            if let Some(d) = deferral_of(&e) {
                println!("ACT_QUOTE deferred (deficit {:?})", d.deficit);
                return ExitCode::SUCCESS;
            }
            eprintln!("ACT_QUOTE failed: {e}");
            return ExitCode::from(1);
        }
    }

    ExitCode::SUCCESS
}
