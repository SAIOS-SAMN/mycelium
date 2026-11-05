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
//! 2. The Concordance — its clause register read (`ACT_LAWS` mode 7), each
//!    clause's fixtures reconstructed here (`house::sign_concordance`) and
//!    the readings signed by the key, presented to the hypha (mode 8).
//! 3. The clock — the court's head stamp (`ACT_LAWS` mode 4).
//! 4. The domain — founded on the house's key, its one tier the same key
//!    (`ACT_DOMAIN`): sealed, or founded already on a run before.
//! 5. Conformance — `conformance::run(endpoint, key, &[Profile::Kernel])`:
//!    every MUST act of the kernel profile, checked by decoded exact values;
//!    an act the hypha deferred is reported deferred, with its deficit.
//!
//! A hypha whose worker pool is full answers any act with the deferral; the
//! house reads it (`deferral::deferral_of`, `deferral::read_deferral`) and
//! says so; the act is spoken again on a later run.
//!
//! Exit 0 where the house conforms; 1 where it does not or the hypha could
//! not be reached; 2 on a usage refusal.

use mycelium_sdk::conformance::{run, Profile};
use mycelium_sdk::deferral::{deferral_of, read_deferral, Deferral};
use mycelium_sdk::house::{
    decode_clock_reply, decode_concordance_reply, decode_signature_reply, encode_clock_request, encode_concordance_request, encode_signature_request,
    sign_concordance, Assent, DomainGenesis, Founded,
};
use mycelium_sdk::prelude::*;
use mycelium_sdk::voice::hex32;
use std::process::ExitCode;

/// A deferral, said: the hypha's pool was full, nothing was ruled, and the
/// act is spoken again later.
fn said(d: Deferral) -> String {
    format!("a deferral: the hypha's pool is full, deficit S:G:T:H {:?}; retry later", d.deficit)
}

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

    let served = match session.speak(encode_concordance_request()) {
        Ok(reply) => decode_concordance_reply(&reply),
        Err(e) => match deferral_of(&e) {
            Some(d) => {
                println!("concordance: {}", said(d));
                None
            }
            None => {
                eprintln!("{endpoint}: {e}");
                return ExitCode::from(1);
            }
        },
    };
    match served {
        Some(Ok(clauses)) => {
            let named: Vec<String> = clauses.iter().map(|c| format!("{} v{} ({} fixtures)", c.id.law, c.id.version, c.fixtures.len())).collect();
            println!("concordance: {}", named.join(", "));
            match sign_concordance(&key, &clauses, |_| Assent::Enabled) {
                Some(signed) => match session.speak(encode_signature_request(&signed)) {
                    Ok(reply) => println!("  signed by reconstruction; the hypha's verdict: {:?}", decode_signature_reply(&reply)),
                    Err(e) => match deferral_of(&e) {
                        Some(d) => println!("  {}", said(d)),
                        None => {
                            eprintln!("{endpoint}: {e}");
                            return ExitCode::from(1);
                        }
                    },
                },
                None => println!("  a served fixture this house does not reconstruct"),
            }
        }
        Some(Err(r)) => println!("concordance: the hypha cannot read clause {:?} of its register", r.id),
        None => println!("concordance: a reply this build does not read"),
    }

    match session.speak(encode_clock_request()) {
        Ok(reply) => match (read_deferral(&reply), decode_clock_reply(&reply)) {
            (Some(d), _) => println!("clock: {}", said(d)),
            (None, Some(Some(stamp))) => println!("clock: the head at {stamp:?}"),
            (None, Some(None)) => println!("clock: no head yet"),
            (None, None) => println!("clock: a reply this build does not read"),
        },
        Err(e) => match deferral_of(&e) {
            Some(d) => println!("clock: {}", said(d)),
            None => println!("clock: {e}"),
        },
    }

    let genesis = DomainGenesis { domain, tiers: vec![domain] }.sign(&key);
    match session.found_domain(&genesis) {
        Ok(Founded::Sealed { at, receipt }) => println!("domain: sealed at height {}, receipt {}", at.h, hex32(&receipt)),
        Ok(Founded::AlreadyFounded) => println!("domain: founded already"),
        Ok(other) => println!("domain: refused, {other:?}"),
        Err(e) => match deferral_of(&e) {
            Some(d) => println!("domain: {}", said(d)),
            None => println!("domain: {e}"),
        },
    }

    println!("\nconformance, kernel profile ({endpoint}):");
    let report = run(endpoint, &key, &[Profile::Kernel]);
    print!("{report}");
    match report.conforms() {
        true => ExitCode::SUCCESS,
        false => ExitCode::from(1),
    }
}
