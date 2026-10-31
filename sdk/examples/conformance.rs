// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// THE CLIENT CONFORMANCE SUITE, RUNNABLE: the MUST acts of the declared
// profiles spoken to a running hypha and checked by decoded exact values,
// the report printed (MU-CONFORMANCE).
//
//! `conformance --endpoint <hypha address> [--home <voice home>] [--profile mind|kernel|both]`
//!
//! Prints the declared profiles' MUST and MAY acts, then the in-process
//! codec check and the suite's report against the hypha. The voice is read
//! from `<voice home>/voice.key` or minted there (default: a
//! `mycelium-conformance` directory under the system's temporary directory).
//! The profile is `mind` unless declared; `kernel` checks a kernel's uplink,
//! founding a domain on the voice's key; `both` holds the client to both.
//!
//! Exit 0 where both reads conform; 1 where either does not; 2 on a usage
//! refusal.

use mycelium_sdk::conformance::{check_codecs, run, Need, Profile, STATEMENT};
use std::process::ExitCode;

const USAGE: &str = "usage: conformance --endpoint <hypha address> [--home <voice home>] [--profile mind|kernel|both]";

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].as_str())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let Some(endpoint) = flag(&args, "--endpoint") else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    let profiles: &[Profile] = match flag(&args, "--profile") {
        None | Some("mind") => &[Profile::Mind],
        Some("kernel") => &[Profile::Kernel],
        Some("both") => &Profile::ALL,
        Some(other) => {
            eprintln!("{other}: no such profile\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    let home = match flag(&args, "--home") {
        Some(h) => std::path::PathBuf::from(h),
        None => std::env::temp_dir().join("mycelium-conformance"),
    };
    if let Err(e) = std::fs::create_dir_all(&home) {
        eprintln!("{}: {e}", home.display());
        return ExitCode::from(2);
    }
    let Some(key_path) = home.join("voice.key").to_str().map(str::to_string) else {
        eprintln!("{}: the voice home's path is not UTF-8", home.display());
        return ExitCode::from(2);
    };
    let (_voice, key) = mycelium_sdk::voice::load_or_mint(&key_path);
    println!("statement: wire 0x{:04x}, law version {}", STATEMENT.wire_version, STATEMENT.law_version);
    let acts = STATEMENT.union(profiles);
    let declared: Vec<&str> = profiles.iter().map(|p| p.name()).collect();
    println!("  profile: {}", declared.join(" + "));
    for (need, label) in [(Need::Must, "MUST"), (Need::May, "MAY")] {
        let names: Vec<&str> = acts.iter().filter(|a| a.need == need).map(|a| a.name).collect();
        println!("  {label}: {}", names.join(", "));
    }
    println!("\nin process (every act of the declared profiles):");
    let codecs = check_codecs(profiles, |_| true);
    print!("{codecs}");
    println!("\non the network ({endpoint}):");
    let report = run(endpoint, &key, profiles);
    print!("{report}");
    match codecs.conforms() && report.conforms() {
        true => ExitCode::SUCCESS,
        false => ExitCode::from(1),
    }
}
