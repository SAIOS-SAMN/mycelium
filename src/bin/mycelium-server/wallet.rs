// 78.93 B1 — THE OPERATOR READS A WALLET: `--console --wallet KEY` asks a
// hypha for one party's account — any party, by its 32-byte key in hex — and
// prints its balance, bond and holds per lane `S:G:T:H`, and where the
// reading stands on the court's line.
//
//! It asks and exits, as the console does, before anything is created here.

use mycelium::anastome::dialogue::wallet::{decode_wallet_reply, encode_wallet};
use std::path::Path;

const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

/// A whole 32-byte key in hex, or nothing.
fn key(s: &str) -> Option<[u8; 32]> {
    let s = s.trim().as_bytes();
    if s.len() != 64 {
        return None;
    }
    let bytes: Option<Vec<u8>> = s.chunks(2).map(|d| u8::from_str_radix(std::str::from_utf8(d).ok()?, 16).ok()).collect();
    bytes?.try_into().ok()
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

pub(crate) fn run(home: &Path, at: Option<&str>, asked: &str) -> ! {
    let Some(who) = key(asked) else {
        println!("{RED}--wallet takes a party's whole key, 64 hex digits{RESET}");
        std::process::exit(2)
    };
    let Some(endpoint) = crate::console::endpoint(home, at) else {
        println!("{RED}no address to ask{RESET}: pass --at ADDR, or --home a home whose mesh names its bind");
        std::process::exit(2)
    };
    let Some((reply, _)) = crate::transport::tell_peer_within(&endpoint, encode_wallet(&who), mycelium::physics::reserve_max()) else {
        println!("{RED}{endpoint} did not answer{RESET}: an account it cannot measure is not answered");
        std::process::exit(2)
    };
    let Some(a) = decode_wallet_reply(&reply) else {
        println!("{RED}{endpoint} answered with no wallet this build reads{RESET} ({} bytes)", reply.len());
        std::process::exit(2)
    };
    let stands = match a.at {
        Some(s) => format!("at h {} {}", s.h, s.place().print()),
        None => "before the court's first act".to_string(),
    };
    println!("{BOLD}═══ WALLET {}… ═══{RESET}  court {}…  {DIM}{stands}{RESET}", &hex(&who)[..8], &hex(&a.court)[..8]);
    println!("  balance  S:G:T:H {:?}", a.balance);
    println!("  bond     S:G:T:H {:?}", a.bond);
    println!("  held     S:G:T:H {:?}", a.held);
    match a.owned() {
        Some(o) => println!("  {BOLD}owned{RESET}    S:G:T:H {o:?}"),
        None => println!("  {RED}owned passes its carrier on a lane{RESET}"),
    }
    std::process::exit(0)
}
