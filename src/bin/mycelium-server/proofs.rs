// MU-ECONOMY §0, §3.4, §5.3; 78.174 §1 item 7 — THE PROOFS OF WORK, READ
// OVER THE MESH: for every payment a court's line holds that carries a
// proof (POUW, POWC, POWR, POWV), what was paid, to which key, at which act
// and the words that act names, the language level read off those words,
// and each other court's reading of the same act at the same stamp; then
// every closure two courts or two voices were paid for as a first closure.
//
//! `mycelium-server --console --proofs [--at ADDR] [--home DIR]` asks the
//! home's court (or `--at`) and every peer its mesh record names. It is a
//! READING: it asks, renders and exits, and nothing it reads is stored.
//!
//! WHAT IT READS, each from a reader the courts already serve:
//! - the payments: the court's own journal rows (`ACT_ENTRIES`, the rows a
//!   witness re-derives a fold over), each row's proof (`Entry.arm`), payee,
//!   per-lane amount and the stamp of the act it stands at. A payment
//!   folded into an epoch row is not listed: the court's reading by proof
//!   counts it at the fold's checkpoint (MU-ECONOMY §6.1), and the epoch
//!   rows are counted so;
//! - the closure each payment cites: the act at that stamp and the words it
//!   names, each read off the word the court holds (`ACT_WHY`, own place);
//! - the language level (78.174 §1 item 7): a named word with a sentence's
//!   reasons is a SENTENCE; named words without them are WORDS (a letter
//!   and a word are not told apart by any reading the court serves, nor is
//!   a conjugate class); an act that names no word leaves the level unread;
//! - the re-traversal at each other court (MU-ECONOMY §5.3): the same act
//!   asked of that court by its stamp on the paying court's line
//!   (`ACT_WHY`, any line the court holds). The same act kind and the same
//!   words, each read off that court's own copy, is the SAME POINT; anything
//!   else is a residue naming what differs; an act that court does not
//!   hold is said so. This reads what each court holds; the full
//!   re-traversal of §5.3, from the payment through the release and its
//!   verifications to the closure's reconstruction on the withheld pairs,
//!   needs the payment to be an event citing its causes (v0.5.0).
//! - one closure, one point (§3.4): every court's judged solves
//!   (`Reading::Judged`) by the task's whole citation. A citation answered
//!   as a first closure more than once, at two courts or to two voices, is
//!   two mints of one closure: an EQUIVOCATION OF THE ADVANCE, named with
//!   each court, voice and height. §3.4 burns the second mint; no court
//!   burns it yet (one closure per network is v0.5.0's), and the reading
//!   says both mints stand.

use mycelium::anastome::dialogue::why::{decode_why_reply, encode_why, Named, Why, WhyAsk};
use mycelium::anastome::dialogue::{decode_entries_reply, encode_entries_request, is_deferral, PENDING_CODE};
use mycelium::chronometry::{Place, Stamp};
use mycelium::economy::{Arm, Entry, Kind};
use mycelium::observe::{self, JudgedSolves, Reading, Source};
use std::path::Path;

const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// A key's first four bytes, the head a console names it by.
fn short(b: &[u8; 32]) -> String {
    hex(&b[..4])
}

/// THE LANGUAGE LEVEL A PAYMENT STANDS AT (78.174 §1 item 7, the ledger's
/// grade axis), as far as the words its act names tell it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Level {
    /// The act names a word carrying a sentence's reasons.
    Sentence,
    /// The act names words, none of them a sentence: a word or a letter.
    Word,
    /// The act names no word the court holds.
    Unread,
}

impl Level {
    /// The level of an act, read off the words it names.
    pub(crate) fn of(named: &[Named]) -> Level {
        match (named.iter().any(|n| n.reasons.is_some()), named.is_empty()) {
            (true, _) => Level::Sentence,
            (false, false) => Level::Word,
            (false, true) => Level::Unread,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Level::Sentence => "sentence",
            Level::Word => "word",
            Level::Unread => "unread",
        }
    }

    /// WHERE A PAYMENT STANDS IN THE RULED MAP (78.174 §1 item 7): letters
    /// never paid; words POWR and citation shares; conjugate classes POWC;
    /// sentences POUW; POWV at every level. A conjugate class is told apart
    /// by no reading the courts serve, so POWC at a word stands only as a
    /// citation share.
    pub(crate) fn ruled(self, arm: Arm) -> Ruled {
        match (self, arm) {
            (_, Arm::Powv) => Ruled::AtEveryLevel,
            (Level::Sentence, Arm::Pouw) | (Level::Word, Arm::Powr) => Ruled::AtThisLevel,
            (Level::Word, Arm::Pouw | Arm::Powc) => Ruled::CitationShare,
            (Level::Sentence, Arm::Powc | Arm::Powr) => Ruled::NotAtThisLevel,
            (Level::Unread, _) => Ruled::LevelUnread,
        }
    }
}

/// A payment's standing in the ruled map, read off its level and its arm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Ruled {
    /// POWV, ruled at every level.
    AtEveryLevel,
    /// The arm the map rules at this level: POUW at a sentence, POWR at a
    /// word.
    AtThisLevel,
    /// At a word, POUW or POWC stands only as a citation share.
    CitationShare,
    /// At a sentence, POWC or POWR is not in the map.
    NotAtThisLevel,
    /// The act names no word the court holds, so no level is read.
    LevelUnread,
}

impl Ruled {
    fn says(self) -> &'static str {
        match self {
            Ruled::AtEveryLevel => "ruled (POWV at every level)",
            Ruled::AtThisLevel => "ruled at this level (sentences POUW, words POWR)",
            Ruled::CitationShare => "ruled only as a citation share (words: POWR plus citation shares)",
            Ruled::NotAtThisLevel => "not in the ruled map at a sentence (sentences: POUW)",
            Ruled::LevelUnread => "the level is unread: the act names no word the court serves",
        }
    }
}

/// WHAT ANOTHER COURT HOLDS OF ONE ACT, against what the paying court holds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Traversal {
    /// The same act kind and the same words, each read off its own copy.
    Same,
    /// What differs, each named.
    Residue(Vec<String>),
    /// The court holds no act at that stamp on the paying court's line.
    NotHeld,
    /// The court's door was full: the question is left for its next stroke.
    Deferred,
    /// No answer this build reads.
    Unanswered,
}

/// Two readings of one word, by the head the court names it by, the task
/// it was spoken for and its reasons.
fn same_word(a: &Named, b: &Named) -> bool {
    a.head == b.head && a.task == b.task && a.reasons == b.reasons
}

/// THE RE-TRAVERSAL AT ANOTHER COURT: `there`, what that court holds at the
/// stamp, read against `here`, what the paying court holds at its place.
pub(crate) fn retraverse(here: &Why, there: &Why) -> Traversal {
    let mut residue = Vec::new();
    if here.kind != there.kind {
        residue.push(format!(
            "the act is {} at the paying court, {} here",
            String::from_utf8_lossy(here.kind.name()),
            String::from_utf8_lossy(there.kind.name())
        ));
    }
    for n in &here.named {
        match there.named.iter().find(|m| m.head == n.head) {
            None => residue.push(format!("word {} is held at the paying court, not here", hex(&n.head))),
            Some(m) if !same_word(n, m) => residue.push(format!("word {} reads otherwise here (its task or its reasons)", hex(&n.head))),
            Some(_) => {}
        }
    }
    for m in &there.named {
        if !here.named.iter().any(|n| n.head == m.head) {
            residue.push(format!("word {} is held here, not at the paying court", hex(&m.head)));
        }
    }
    match residue.is_empty() {
        true => Traversal::Same,
        false => Traversal::Residue(residue),
    }
}

/// ONE FIRST CLOSURE A COURT JUDGED: the court, the voice paid and the
/// court's height at the verdict, with how many first verdicts it gave.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Paid {
    pub(crate) court: [u8; 32],
    pub(crate) voice: [u8; 32],
    pub(crate) first: u64,
    pub(crate) height: u64,
}

/// The slot of the first-closure verdict in `JudgedSolve::times`: its wire
/// code.
const FIRST: usize = mycelium::anastome::dialogue::SOLVE_BOUNTY as usize;

/// ONE CLOSURE, ONE POINT (MU-ECONOMY §3.4): every task citation that more
/// than one first closure was judged for, over the courts' judged solves,
/// each with every first closure of it. In citation order.
pub(crate) fn second_mints(judged: &[([u8; 32], JudgedSolves)]) -> Vec<([u8; 32], Vec<Paid>)> {
    // Held in the citations' own order, each filed where it stands.
    let mut by_point: Vec<([u8; 32], Vec<Paid>)> = Vec::new();
    for (court, j) in judged {
        for s in &j.solves {
            let first = match s.times.get(FIRST) {
                Some(&f) if f > 0 => f,
                _ => continue,
            };
            let paid = Paid { court: *court, voice: s.voice, first, height: s.height };
            match by_point.binary_search_by(|(c, _)| c.cmp(&s.citation)) {
                Ok(at) => {
                    if let Some((_, all)) = by_point.get_mut(at) {
                        all.push(paid);
                    }
                }
                Err(at) => by_point.insert(at, (s.citation, vec![paid])),
            }
        }
    }
    by_point.retain(|(_, all)| all.len() > 1 || all.iter().any(|p| p.first > 1));
    by_point
}

fn kind_name(k: Kind) -> &'static str {
    match k {
        Kind::Genesis => "genesis",
        Kind::Mint => "mint",
        Kind::Stake => "stake",
        Kind::Unstake => "unstake",
        Kind::Transfer => "transfer",
        Kind::Escrow => "escrow",
        Kind::Release => "release",
        Kind::Revoke => "revoke",
        Kind::Slash => "slash",
        Kind::Burn => "burn",
        Kind::Epoch => "epoch",
    }
}

/// What one court answered `ACT_WHY` with.
#[derive(Clone, Debug)]
enum Asked {
    Held(Why),
    NotHeld,
    Deferred,
    Unanswered,
}

fn ask_why_once(endpoint: &str, ask: WhyAsk) -> Asked {
    let Some((reply, _)) = crate::transport::tell_peer_within(endpoint, encode_why(ask), crate::transport::Stall::Beyond) else {
        return Asked::Unanswered;
    };
    match decode_why_reply(&reply) {
        Some(Some(w)) => Asked::Held(w),
        Some(None) => Asked::NotHeld,
        None if is_deferral(&reply, PENDING_CODE) => Asked::Deferred,
        None => Asked::Unanswered,
    }
}

/// 78.92 — a door that was full left the question for its next stroke,
/// which answers it under its own lock: the question asked again is
/// answered. A second deferral is said as one.
fn ask_why(endpoint: &str, ask: WhyAsk) -> Asked {
    match ask_why_once(endpoint, ask) {
        Asked::Deferred => ask_why_once(endpoint, ask),
        answered => answered,
    }
}

/// ONE COURT AS THE READING MET IT: its address, its key, its judged
/// solves and its readings of payments by proof.
struct Court {
    endpoint: String,
    key: [u8; 32],
    judged: JudgedSolves,
    economy: Option<observe::Economy>,
}

/// The court at `endpoint`, by its own readings: its key from the source
/// its readings stand under. `None` where it answered none.
fn court_at(endpoint: &str) -> Option<Court> {
    let held = |prefix: &[u8]| -> Option<Vec<observe::KeyedReading>> {
        let query = observe::encode_query(prefix)?;
        let (reply, _) = crate::transport::tell_peer_within(endpoint, query, crate::transport::Stall::Beyond)?;
        Some(observe::decode_reply(&reply)?.1)
    };
    let key = held(&[observe::SOURCE_COURT])?.iter().find_map(|(k, _)| match Source::of_key(k)? {
        (Source::Court(vk), _) => Some(vk),
        _ => None,
    })?;
    let mut court = Court { endpoint: endpoint.to_string(), key, judged: JudgedSolves::default(), economy: None };
    for tag in [observe::TAG_JUDGED, observe::TAG_ECONOMY] {
        let mut prefix = vec![observe::SOURCE_COURT];
        prefix.extend_from_slice(&key);
        prefix.push(tag);
        let Some(readings) = held(&prefix) else { continue };
        for (_, r) in readings {
            match r {
                Reading::Judged(j) => court.judged = *j,
                Reading::Economy(e) => court.economy = Some(*e),
                _ => {}
            }
        }
    }
    Some(court)
}

/// Every row of `court`'s own journal it serves, page after page; the
/// total the span holds beside them. `None` where a page did not answer.
fn rows_of(court: &Court) -> Option<(u64, Vec<Entry>)> {
    let upto = Place { k: u64::MAX, i: u64::MAX };
    let mut rows: Vec<Entry> = Vec::new();
    loop {
        let cursor = rows.len() as u64;
        let ask = encode_entries_request(&[0; 32], &court.key, None, upto, cursor);
        let (reply, _) = crate::transport::tell_peer_within(&court.endpoint, ask, crate::transport::Stall::Beyond)?;
        let (total, page) = decode_entries_reply(&reply)?;
        let ended = page.is_empty();
        rows.extend(page);
        if ended || rows.len() as u64 >= total {
            return Some((total, rows));
        }
    }
}

/// The name a key is said by: a court of the mesh by its address, any
/// other key by its head.
fn said(key: &[u8; 32], courts: &[Court]) -> String {
    match courts.iter().find(|c| &c.key == key) {
        Some(c) => format!("court {} ({})", short(key), c.endpoint),
        None => format!("key {}", short(key)),
    }
}

fn named_line(n: &Named) -> String {
    let task = match n.task {
        Some(t) => format!(" for task {}", hex(&t)),
        None => String::new(),
    };
    let kind = match n.reasons {
        Some(_) => "sentence",
        None => "word",
    };
    format!("{kind} {}{task}, admitted at h {} {}", hex(&n.head), n.admitted.h, n.admitted.place().print())
}

/// One court's payments by proof, each with its closure, level and
/// re-traversals; the lines said and the payments read.
fn payments(court: &Court, courts: &[Court], lines: &mut Vec<String>) -> u64 {
    lines.push(String::new());
    lines.push(format!("{BOLD}── PAYMENTS ON THE LINE OF {}{RESET}", said(&court.key, courts)));
    let Some((total, rows)) = rows_of(court) else {
        lines.push(format!("  {RED}the journal did not answer{RESET}"));
        return 0;
    };
    let folded = rows.iter().filter(|e| e.kind == Kind::Epoch).count();
    lines.push(format!(
        "  {DIM}{} rows served of {total}; {folded} epoch rows: a payment folded before them is not listed here, and is counted under BY PROOF at the fold's checkpoint (MU-ECONOMY §6.1){RESET}",
        rows.len()
    ));
    let mut whys: Vec<(Stamp, Asked)> = Vec::new();
    let mut read = 0u64;
    for e in rows.iter() {
        let Some(arm) = e.arm else { continue };
        read += 1;
        lines.push(format!(
            "  {BOLD}{}{RESET} {} {} to {} · S:G:T:H {} · at h {} {}",
            arm.name(),
            kind_name(e.kind),
            match e.kind {
                Kind::Transfer => "(one side of the pair)",
                _ => "",
            },
            said(&e.who, courts),
            e.mu,
            e.at.h,
            e.at.place().print()
        ));
        let here = match whys.iter().find(|(s, _)| *s == e.at) {
            Some((_, a)) => a.clone(),
            None => {
                let a = ask_why(&court.endpoint, WhyAsk::Own(e.at.place()));
                whys.push((e.at, a.clone()));
                a
            }
        };
        let Asked::Held(here) = here else {
            lines.push(format!("    {YELLOW}the act at its place did not read at the paying court: {here:?}{RESET}"));
            continue;
        };
        let level = Level::of(&here.named);
        lines.push(format!(
            "    closure: the {} act; level {} — {}",
            String::from_utf8_lossy(here.kind.name()),
            level.name(),
            level.ruled(arm).says()
        ));
        for n in &here.named {
            lines.push(format!("      names {}", named_line(n)));
        }
        if here.unheld > 0 {
            lines.push(format!("      {} named words the court no longer holds", here.unheld));
        }
        for other in courts.iter().filter(|c| c.key != court.key) {
            let reading = match ask_why(&other.endpoint, WhyAsk::Of(e.at)) {
                Asked::Held(there) => retraverse(&here, &there),
                Asked::NotHeld => Traversal::NotHeld,
                Asked::Deferred => Traversal::Deferred,
                Asked::Unanswered => Traversal::Unanswered,
            };
            let at = said(&other.key, courts);
            match reading {
                Traversal::Same => lines.push(format!("    re-traversal at {at}: the same point")),
                Traversal::Residue(r) => {
                    lines.push(format!("    {YELLOW}re-traversal at {at}: another point, the residue held{RESET}"));
                    lines.extend(r.iter().map(|l| format!("      {l}")));
                }
                Traversal::NotHeld => lines.push(format!("    {YELLOW}re-traversal at {at}: the act is not held there{RESET}")),
                Traversal::Deferred => lines.push(format!("    re-traversal at {at}: deferred (its door was full)")),
                Traversal::Unanswered => lines.push(format!("    {RED}re-traversal at {at}: no answer{RESET}")),
            }
        }
    }
    if read == 0 {
        lines.push("  no payment carrying a proof stands on the rows served".to_string());
    }
    read
}

/// ASK, RENDER, EXIT: 0 where every court answered, 1 where one did not, 2
/// where no court could be asked.
pub(crate) fn run(home: &Path, at: Option<&str>) -> ! {
    let Some(endpoint) = crate::console::endpoint(home, at) else {
        println!("{RED}no address to ask{RESET}: pass --at ADDR, or --home a home whose mesh names its bind");
        std::process::exit(2)
    };
    let mut endpoints = vec![endpoint];
    endpoints.extend(peers(home));
    let mut courts: Vec<Court> = Vec::new();
    let mut silent: Vec<String> = Vec::new();
    for e in &endpoints {
        match court_at(e) {
            Some(c) => courts.push(c),
            None => silent.push(e.clone()),
        }
    }
    let mut lines = vec![format!(
        "{BOLD}═══ POW++ — the proofs of work over the mesh ═══{RESET}  {} courts answered{}",
        courts.len(),
        match silent.is_empty() {
            true => String::new(),
            false => format!(", {RED}{} did not ({}){RESET}", silent.len(), silent.join(", ")),
        }
    )];
    lines.push(format!(
        "{DIM}pay by level (78.174 §1 item 7): letters never paid · words POWR and citation shares · conjugate classes POWC · sentences POUW · POWV at every level{RESET}"
    ));
    for c in &courts {
        lines.push(String::new());
        lines.push(format!("{BOLD}── BY PROOF AT {}{RESET}", said(&c.key, &courts)));
        match &c.economy {
            Some(e) => {
                for arm in Arm::ALL {
                    lines.push(format!("  {} paid S:G:T:H {} · burned {}", arm.name(), e.paid[arm.index()], e.burned[arm.index()]));
                }
            }
            None => lines.push("  the economy reading is not yet published (asked; its next stroke publishes it)".to_string()),
        }
    }
    for c in &courts {
        payments(c, &courts, &mut lines);
    }
    lines.push(String::new());
    lines.push(format!("{BOLD}── ONE CLOSURE, ONE POINT (MU-ECONOMY §3.4){RESET}"));
    let judged: Vec<([u8; 32], JudgedSolves)> = courts.iter().map(|c| (c.key, c.judged.clone())).collect();
    let firsts: u64 = judged.iter().flat_map(|(_, j)| j.solves.iter()).filter(|s| s.times.get(FIRST).is_some_and(|f| *f > 0)).count() as u64;
    lines.push(format!("  {DIM}first closures judged, by citation and voice, over the courts: {firsts}{RESET}"));
    let twice = second_mints(&judged);
    if twice.is_empty() {
        lines.push("  no closure was minted twice".to_string());
    }
    for (citation, all) in &twice {
        let times = match all.iter().try_fold(0u64, |n, p| n.checked_add(p.first)) {
            Some(n) => n.to_string(),
            None => "more than a count holds".to_string(),
        };
        lines.push(format!("  {RED}equivocation of the advance{RESET}: task {} was paid as a first closure {times} times", hex(citation)));
        for p in all {
            lines.push(format!("    first closure to voice {} at {} (height {}, {} first verdicts)", short(&p.voice), said(&p.court, &courts), p.height, p.first));
        }
        lines.push(format!(
            "    {YELLOW}§3.4 burns a second mint naming both receipts; no court burns it yet (one closure per network, v0.5.0): both mints stand{RESET}"
        ));
    }
    for l in lines {
        println!("{l}");
    }
    std::process::exit(if silent.is_empty() { 0 } else { 1 })
}

/// The peers this home's mesh record names.
fn peers(home: &Path) -> Vec<String> {
    let Some(key) = mycelium::court::identity::HyphaKey::load_only(&home.join("hypha.key").to_string_lossy()) else { return Vec::new() };
    let Ok(bytes) = std::fs::read(home.join("mesh")) else { return Vec::new() };
    match mycelium::network::mesh::MeshRecord::open_layout(&bytes, &key.verifying_bytes()) {
        Some((rec, _)) => rec.peers,
        None => Vec::new(),
    }
}
