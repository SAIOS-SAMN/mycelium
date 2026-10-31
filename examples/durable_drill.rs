// 78.137 — THE DURABLE-ADMISSION DRILL'S CLIENT (`scripts/durable-drill`):
// court acts put to a hypha the drill kills with SIGKILL between them, then
// read back after the last restart. A court act answered is on the disk:
// none acknowledged may be lost.
//
//! `durable_drill act ADDR I` — domain `I` founded and its reserve
//! designated: two court acts, each answered only once durable.
//! `durable_drill check ADDR N` — every domain `0..N` read back with its
//! reserve; the count lost printed, and a nonzero exit where any was.
//! `durable_drill prefill ADDR D T` — the home grown first: `D` domains of
//! `T` tiers, so the drill is measured on a court's size.

use mycelium::anastome::dialogue::domain::{DomainGenesis, Founded, Reservation};
use mycelium::sdk::{Voice, Session};
use std::process::ExitCode;

/// Domain `i`'s key and its one tier's, distinct for every `i`.
fn keys(i: u64) -> (ed25519_dalek::SigningKey, ed25519_dalek::SigningKey) {
    let mut d = [7u8; 32];
    d[..8].copy_from_slice(&i.to_le_bytes());
    let mut t = d;
    t[31] = 8;
    (ed25519_dalek::SigningKey::from_bytes(&d), ed25519_dalek::SigningKey::from_bytes(&t))
}

fn act(endpoint: &str, i: u64) -> ExitCode {
    let s = Session::open(endpoint, Voice::from_voice([9; 32]));
    let (domain, tier) = keys(i);
    let (dv, tv) = (domain.verifying_key().to_bytes(), tier.verifying_key().to_bytes());
    let genesis = DomainGenesis { domain: dv, tiers: vec![tv] }.sign(&domain);
    match s.found_domain(&genesis) {
        Ok(Founded::Sealed { .. } | Founded::AlreadyFounded) => {}
        other => {
            println!("act {i}: genesis {other:?}");
            return ExitCode::FAILURE;
        }
    }
    let reservation = Reservation { domain: dv, tier: tv, nonce: [0; 32] }.sign(&domain);
    match s.reserve(&reservation) {
        Ok(Founded::Reserved { .. }) => {
            println!("act {i}: sealed");
            ExitCode::SUCCESS
        }
        other => {
            println!("act {i}: reserve {other:?}");
            ExitCode::FAILURE
        }
    }
}

/// The home grown to a court's size before the drill: `domains` founded with
/// `tiers` tier keys each (32 bytes a key), from a seed space no drill
/// domain uses.
fn prefill(endpoint: &str, domains: u64, tiers: u64) -> ExitCode {
    let s = Session::open(endpoint, Voice::from_voice([9; 32]));
    for d in 0..domains {
        let mut seed = [5u8; 32];
        seed[..8].copy_from_slice(&d.to_le_bytes());
        let domain = ed25519_dalek::SigningKey::from_bytes(&seed);
        let mut keys: Vec<[u8; 32]> = (0..tiers)
            .map(|t| {
                let mut ts = seed;
                ts[8..16].copy_from_slice(&t.to_le_bytes());
                ts[31] = 6;
                ed25519_dalek::SigningKey::from_bytes(&ts).verifying_key().to_bytes()
            })
            .collect();
        keys.sort_unstable();
        let genesis = DomainGenesis { domain: domain.verifying_key().to_bytes(), tiers: keys }.sign(&domain);
        if !matches!(s.found_domain(&genesis), Ok(Founded::Sealed { .. } | Founded::AlreadyFounded)) {
            println!("prefill: domain {d} not founded");
            return ExitCode::FAILURE;
        }
    }
    println!("prefilled {domains} domains of {tiers} tiers");
    ExitCode::SUCCESS
}

fn check(endpoint: &str, n: u64) -> ExitCode {
    let s = Session::open(endpoint, Voice::from_voice([9; 32]));
    let mut lost = 0u64;
    for i in 0..n {
        let (domain, tier) = keys(i);
        let held = s.reserve_reading(domain.verifying_key().to_bytes()).map(|r| r.current);
        if !matches!(held, Ok(Some(t)) if t == tier.verifying_key().to_bytes()) {
            println!("lost: domain {i}");
            lost += 1;
        }
    }
    println!("checked {n}, lost {lost}");
    match lost {
        0 => ExitCode::SUCCESS,
        _ => ExitCode::FAILURE,
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let number = |k: usize| args.get(k).and_then(|a| a.parse::<u64>().ok());
    match (args.get(1).map(String::as_str), args.get(2), number(3)) {
        (Some("act"), Some(endpoint), Some(i)) => act(endpoint, i),
        (Some("check"), Some(endpoint), Some(n)) => check(endpoint, n),
        (Some("prefill"), Some(endpoint), Some(d)) => match number(4) {
            Some(t) => prefill(endpoint, d, t),
            None => ExitCode::FAILURE,
        },
        _ => {
            println!("usage: durable_drill act ADDR I | check ADDR N | prefill ADDR DOMAINS TIERS");
            ExitCode::FAILURE
        }
    }
}
