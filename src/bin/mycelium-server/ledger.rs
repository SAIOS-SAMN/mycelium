// 78.94 — THE OPERATOR READS A COURT'S WHOLE LINE: `--console --ledger` asks
// the hypha for its own receipts by place, page after page through the
// receipts action (which serves the archived prefix as well as the hot
// chain), from its genesis to its head, and prints every act by its action
// (`physics::actions::Action`) and where each refusal stands — the place
// `--why` asks by. The console's carriage counts the hot chain since the
// last fold; this reads the line whole.
//
//! It asks and exits, as the console does, before anything is created here.

use mycelium::anastome::dialogue::{decode_receipts_reply, encode_replay_request};
use mycelium::chronometry::Place;
use mycelium::network::nerve::Keyed;
use std::path::Path;

const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

pub(crate) fn run(home: &Path, at: Option<&str>) -> ! {
    let Some(key) = mycelium::court::identity::HyphaKey::load_only(&home.join("hypha.key").to_string_lossy()) else {
        println!("{RED}{} holds no key this build reads{RESET}", home.display());
        std::process::exit(2)
    };
    let vk = key.vk_bytes();
    let Some(addr) = crate::console::address(home, at) else {
        println!("{RED}no address to ask{RESET}: pass --at ADDR, or --home a home whose mesh names its bind");
        std::process::exit(2)
    };
    let upto = Place { k: u64::MAX, i: u64::MAX };
    // 2026-10-08 (ruling 6a) — THE READER OPENS AT THE
    // LINE'S FOLD ROOT, never at genesis: a court that folded most of its
    // line answers a page from genesis only by paging its whole cold
    // archive, and hypha b's did not answer within the console's patience
    // ("stopped answering after 0 acts"). The court's own clock reading
    // names the last fold root's orbit; the line is read from that orbit's
    // start (the one era the root closes is paged, no other), and the
    // header says where it began.
    let opened = fold_root_k(&addr, &vk);
    let mut after: Option<Place> = opened.and_then(|k| k.checked_sub(1)).map(|k| Place { k, i: u64::MAX });
    let mut by_action: Keyed<u64> = Keyed::new();
    let mut refused: Vec<Place> = Vec::new();
    let (mut acts, mut pages) = (0u64, 0u64);
    loop {
        // 78.147: the reader is an era replay — the one ask the cold archive
        // answers; a catch-up below the fold root is answered with the root.
        let ask = encode_replay_request(&[0; 32], &vk, after, upto);
        let Some((reply, _)) = crate::transport::tell_peer_within(&addr, ask, mycelium::physics::reserve_max()) else {
            println!("{RED}{addr} stopped answering after {acts} acts{RESET}");
            break;
        };
        let Some(page) = decode_receipts_reply(&reply) else {
            println!("{RED}a page that does not decode after {acts} acts{RESET}");
            break;
        };
        // The next page opens after the furthest place this one read: a
        // coordinate of the line, never a member chosen by its position.
        let furthest = page.iter().fold(None, |at: Option<Place>, r| match at {
            Some(p) if p >= r.stamp.place() => Some(p),
            _ => Some(r.stamp.place()),
        });
        let Some(next) = furthest else { break };
        pages += 1;
        for r in &page {
            acts += 1;
            if r.kind == mycelium::physics::actions::Action::Refused {
                refused.push(r.stamp.place());
            }
            let key = [r.kind.byte()];
            // An action not yet seen has been seen no times: the reading of absence.
            let n = match by_action.get(&key) {
                Some(n) => *n,
                None => 0,
            };
            by_action.insert(key, n + 1);
        }
        after = Some(next);
    }
    let span = match opened {
        Some(k) => format!("from orbit {k} (the line's last fold root) to head; the archive below it is not read here"),
        None => "genesis to head (no fold)".to_string(),
    };
    println!("{BOLD}═══ LEDGER {}… ═══{RESET}  {addr}  {DIM}{acts} acts over {pages} pages, {span}{RESET}", &hex(&vk)[..8]);
    // The court's whole verifying key (public): the party `--wallet` reads.
    println!("  court key   {}", hex(&vk));
    for (k, n) in by_action.scoped() {
        let [b] = k.as_slice() else { continue };
        let name = mycelium::physics::actions::Action::of(*b)
            .map_or_else(|| format!("byte {b:#04x}"), |a| String::from_utf8_lossy(a.name()).into_owned());
        println!("  {name:<12} {n}");
    }
    let places: Vec<String> = refused.iter().map(|p| p.print()).collect();
    println!("  refusals at {}", places.join(" "));
    std::process::exit(0)
}

/// The orbit of the court's last fold root, from its own clock reading
/// (`CourtClock::folded`), or none where it has never folded or does not
/// answer — then the line is read from genesis, as it always was.
fn fold_root_k(addr: &str, vk: &[u8; 32]) -> Option<u64> {
    use mycelium::observe::{self, Clock, Reading};
    let mut prefix = vec![observe::SOURCE_COURT];
    prefix.extend_from_slice(vk);
    prefix.push(observe::TAG_CLOCK);
    let query = observe::encode_query(&prefix)?;
    let (reply, _) = crate::transport::tell_peer_within(addr, query, mycelium::physics::reserve_max())?;
    let (_, held) = observe::decode_reply(&reply)?;
    held.into_iter().find_map(|(_, r)| match r {
        Reading::Clock(Clock::Court(c)) => c.folded.map(|(_, k)| k),
        _ => None,
    })
}
