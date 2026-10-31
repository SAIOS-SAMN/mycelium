// 78.92 (2) — THE OPERATOR ASKS WHY: `--console --why K.i` asks a hypha about
// the act at place `K.i` on its own line and prints the act and every word it
// names, each with its reasons in plain sentences (`Reasons::says`).
//
//! It asks and exits, as the console does, before anything is created here.

use mycelium::anastome::dialogue::why::{decode_why_reply, encode_why, WhyAsk};
use mycelium::chronometry::Place;
use std::path::Path;

const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

/// `K.i`, two counts and nothing else; `None` for anything else.
fn place(s: &str) -> Option<Place> {
    let (k, i) = s.trim().split_once('.')?;
    Some(Place { k: k.parse().ok()?, i: i.parse().ok()? })
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

pub(crate) fn run(home: &Path, at: Option<&str>, asked: &str) -> ! {
    let Some(p) = place(asked) else {
        println!("{RED}--why takes a place K.i on the hypha's own line{RESET}, not {asked}");
        std::process::exit(2)
    };
    let Some(endpoint) = crate::console::endpoint(home, at) else {
        println!("{RED}no address to ask{RESET}: pass --at ADDR, or --home a home whose mesh names its bind");
        std::process::exit(2)
    };
    let Some((reply, _)) = crate::transport::tell_peer_within(&endpoint, encode_why(WhyAsk::Own(p)), mycelium::physics::reserve_max())
    else {
        println!("{RED}{endpoint} did not answer{RESET}");
        std::process::exit(2)
    };
    let why = match decode_why_reply(&reply) {
        Some(Some(w)) => w,
        Some(None) => {
            println!("{endpoint} holds no act at {}", p.print());
            std::process::exit(1)
        }
        // 78.92 — THE DOOR WAS FULL: the question is left for the court's
        // next stroke, which answers it under its own lock; asked again, the
        // answer is served.
        None if mycelium::anastome::dialogue::is_deferral(&reply, mycelium::anastome::dialogue::PENDING_CODE) => {
            println!("{endpoint}'s door was full: the question is left for its next stroke — ask again for the answer");
            std::process::exit(3)
        }
        None => {
            println!("{RED}{endpoint} answered with no why this build reads{RESET} ({} bytes)", reply.len());
            std::process::exit(2)
        }
    };
    let kind = String::from_utf8_lossy(why.kind.name());
    println!(
        "{BOLD}═══ WHY {} ═══{RESET}  {endpoint}  {DIM}{kind} at h {} · frame {:?}{RESET}",
        why.at.place().print(),
        why.at.h,
        why.mu
    );
    if why.named.is_empty() && why.unheld == 0 && why.refused.is_empty() {
        println!("  the act names no word");
    }
    // 78.93 B0 — a refusal on the record, by its kind and the refused
    // sentence's chart.
    for r in &why.refused {
        let kind = match mycelium::transducer::sentence::RefusalKind::of(r.kind) {
            Some(k) => k.label(),
            None => "a kind this build does not name",
        };
        println!("  {BOLD}refused{RESET}: {kind} · the sentence on chart {}…", hex(&r.chart));
    }
    for n in &why.named {
        let task = match n.task {
            Some(t) => format!("spoken for task {}…", hex(&t)),
            None => "spoken for no task".to_string(),
        };
        println!(
            "  {BOLD}word {}…{RESET}  admitted at h {} {} · {task}",
            hex(&n.head),
            n.admitted.h,
            n.admitted.place().print()
        );
        match &n.reasons {
            Some(r) => {
                for s in r.says() {
                    println!("    {s}");
                }
            }
            None => println!("    {DIM}not a sentence: a letter or a connection says itself{RESET}"),
        }
    }
    if why.unheld > 0 {
        println!("  {DIM}{} named word(s) this court no longer holds{RESET}", why.unheld);
    }
    std::process::exit(0)
}
