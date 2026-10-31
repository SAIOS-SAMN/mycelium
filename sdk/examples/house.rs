// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// A MINIMAL HOUSE ON THE SDK: what a kernel's uplink does first on the
// network, with `mycelium-sdk` as its only Mycelium dependency — the
// Concordance signed, the core clock read, a domain founded — then the
// conformance read of the kernel profile against the same hypha.
//
//! `house <house home> <hypha address>`
//!
//! 1. Identity — the house's Ed25519 key, read from `<house home>/voice.key`
//!    or minted there.
//! 2. The Concordance — its hash (`house::CONCORDANCE_HASH`) signed by the
//!    key, verified here, then presented to the hypha (`ACT_LAWS` mode 8).
//! 3. The clock — the court's head stamp (`ACT_LAWS` mode 4).
//! 4. The domain — founded on the house's key, its one tier the same key
//!    (`ACT_DOMAIN`): sealed, or founded already on a run before.
//! 5. Conformance — `conformance::run(endpoint, key, &[Profile::Kernel])`:
//!    every MUST act of the kernel profile, checked by decoded exact values.
//!
//! Exit 0 where the house conforms; 1 where it does not or the hypha could
//! not be reached; 2 on a usage refusal.

use mycelium_sdk::conformance::{run, Profile};
use mycelium_sdk::house::{
    decode_clock_reply, decode_signature_reply, encode_clock_request, encode_signature_request, verify_concordance_signature, ConcordanceSignature,
    DomainGenesis, Founded, CONCORDANCE_HASH,
};
use mycelium_sdk::prelude::*;
use mycelium_sdk::voice::hex32;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let (Some(home), Some(endpoint)) = (args.get(1), args.get(2)) else {
        eprintln!("usage: house <house home> <hypha address>");
        return ExitCode::from(2);
    };
    if let Err(e) = std::fs::create_dir_all(home) {
        eprintln!("{home}: {e}");
        return ExitCode::from(2);
    }
    let (voice, key) = mycelium_sdk::voice::load_or_mint(&format!("{home}/voice.key"));
    let domain = key.verifying_key().to_bytes();
    println!("house {}", hex32(&domain));
    let session = Session::open(endpoint, voice);

    let signed = ConcordanceSignature::sign(&key, CONCORDANCE_HASH);
    println!("concordance {}: signed, verified here: {:?}", hex32(&CONCORDANCE_HASH), verify_concordance_signature(&signed));
    match session.speak(encode_signature_request(&signed)) {
        Ok(reply) => println!("  the hypha's verdict: {:?}", decode_signature_reply(&reply)),
        Err(e) => {
            eprintln!("{endpoint}: {e}");
            return ExitCode::from(1);
        }
    }

    match session.speak(encode_clock_request()).map(|r| decode_clock_reply(&r)) {
        Ok(Some(Some(stamp))) => println!("clock: the head at {stamp:?}"),
        Ok(Some(None)) => println!("clock: no head yet"),
        Ok(None) => println!("clock: a reply this build does not read"),
        Err(e) => println!("clock: {e}"),
    }

    let genesis = DomainGenesis { domain, tiers: vec![domain] }.sign(&key);
    match session.found_domain(&genesis) {
        Ok(Founded::Sealed { at, receipt }) => println!("domain: sealed at height {}, receipt {}", at.h, hex32(&receipt)),
        Ok(Founded::AlreadyFounded) => println!("domain: founded already"),
        Ok(other) => println!("domain: refused, {other:?}"),
        Err(e) => println!("domain: {e}"),
    }

    println!("\nconformance, kernel profile ({endpoint}):");
    let report = run(endpoint, &key, &[Profile::Kernel]);
    print!("{report}");
    match report.conforms() {
        true => ExitCode::SUCCESS,
        false => ExitCode::from(1),
    }
}
