// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// A MINIMAL CLIENT ON THE SDK: what a third-party mind does to work on the
// network, end to end, with `mycelium-sdk` as its only Mycelium dependency.
//
//! `client <voice home> <hypha address> [tasks]`
//!
//! 1. Identity — the voice's Ed25519 key, read from `<voice home>/voice.key`
//!    or minted there (`mycelium-client --init` writes the same file).
//! 2. Session — `Session::open` on a hypha's address; the law register read
//!    (`ACT_LAWS` mode 0), which decodes only at the SDK's `LAW_VERSION`.
//! 3. Work — `ACT_TASK` asks the court for a task; the envelope carries the
//!    train pairs and the test inputs, never the test outputs.
//! 4. Rule — the task's family read by the SDK's extractor
//!    (`rule::extract_rule`), minted as an element the court can replay
//!    (`rule::mint_sentence_why`), and spoken on each test input.
//! 5. Solve — once per perspective of the family's ground
//!    (`rule::on_each_perspective_of`), the outputs, the rule's tape and the
//!    ground the claim was spoken under, signed by the voice
//!    (`task::encode_solve_spoken`); the court replays the rule on pairs
//!    the client never saw and answers a verdict. A replay the court's stroke
//!    did not finish answers `SOLVE_UNREAD`; the same solve sent again
//!    resumes it.
//! 6. Deferral — a hypha whose worker pool is full answers any act with the
//!    deferral (`deferral::deferral_of` on the session's refusal,
//!    `deferral::read_deferral` on raw bytes): nothing was ruled, the door's
//!    per-lane deficit is printed, and the client asks again later.
//!
//! Exit 0 where every task asked was answered by the court (solved or
//! refused by name) or deferred; 1 where the court could not be reached.

use mycelium_sdk::deferral::{deferral_of, read_deferral, Deferral};
use mycelium_sdk::laws;
use mycelium_sdk::prelude::*;
use mycelium_sdk::rule::{extract_rule, mint_sentence_why, on_each_perspective_of, on_test_inputs, speak_rule, stated, Physics, SectionSample};
use mycelium_sdk::task;
use mycelium_sdk::voice::hex32;
use std::process::ExitCode;

/// What one task came to.
enum Outcome {
    /// The court answered the solve: its verdict code, the μ paid, and the
    /// refusal it named where it refused.
    Judged { code: u8, paid: Mu, refused: Option<String> },
    /// The client could not state a rule for the task; nothing was sent.
    NoRule(&'static str),
    /// The court had no task to give.
    NoWork,
    /// The hypha's worker pool was full and it deferred the act, stating
    /// its reserve deficit per lane: nothing was ruled; ask again later.
    Deferred(Deferral),
}

/// A refusal of `speak`, read: the deferral it carries, or the refusal
/// named for `what`.
fn deferred(what: &str, refusal: std::io::Error) -> Result<Deferral, String> {
    deferral_of(&refusal).ok_or_else(|| format!("{what}: {refusal}"))
}

/// How many times one solve is sent while the court answers it unread.
const RESUMES: u32 = 16;

/// The verdict codes the court answers a solve with.
fn verdict_name(code: u8) -> &'static str {
    match code {
        SOLVE_BOUNTY => "solved: first verified closure (bounty)",
        SOLVE_AGAIN => "solved again: a repeat closure, verified, paid nothing",
        SOLVE_NOTHING => "not solved (final)",
        SOLVE_UNREAD => "not judged yet (retry resumes the replay)",
        _ => "other",
    }
}

/// ONE TASK, ASKED AND ANSWERED, one outcome per perspective of its ground:
/// `Err` where the court could not be reached or answered with bytes this
/// build does not read.
fn one_task(session: &Session, key: &SigningKey) -> Result<Vec<(String, Outcome)>, String> {
    let voice = key.verifying_key().to_bytes();
    let reply = match session.speak(task::encode_task_request(&voice)) {
        Ok(r) => r,
        Err(e) => return Ok(vec![(String::new(), Outcome::Deferred(deferred("task request", e)?))]),
    };
    if reply.is_empty() {
        return Ok(vec![(String::new(), Outcome::NoWork)]);
    }
    // A client on a transport of its own reads the deferral off the bytes.
    if let Some(d) = read_deferral(&reply) {
        return Ok(vec![(String::new(), Outcome::Deferred(d))]);
    }
    let Some(samples) = task::decode_task_envelope(&reply) else {
        return Err(format!("the court answered the task request with {} bytes that are no task envelope", reply.len()));
    };
    // The envelope carries samples of any rank (track 26 S5); this client's
    // rule seam spells the alphabet's section, so a task of another rank or
    // width is named and left (track 26 S4b).
    let envelope = match samples.section() {
        Ok(r) => r,
        Err(task::SectionRefusal::Width { .. }) => return Ok(vec![(hex32(&samples.id), Outcome::NoRule("the task's symbols are wider than the section's"))]),
        Err(task::SectionRefusal::Rank(_)) => return Ok(vec![(hex32(&samples.id), Outcome::NoRule("the task's rank is not the one the alphabet spells"))]),
    };
    let cite = hex32(&envelope.id);
    // THE RULE, ONCE PER PERSPECTIVE OF THE FAMILY'S GROUND (MU-SENSING
    // §3.4, §3.8): read off the train pairs on a stroke of the SDK's own
    // metabolism, minted as an element the court replays, and spoken on
    // each test input, all under one perspective; under a vortex once per
    // candidate, every claim kept. The family's symbol set is read with
    // its test inputs posted (§3.3).
    let read = on_test_inputs(envelope.tests.iter().map(SectionSample::sample), || on_each_perspective_of(&envelope.train, || {
        let mut budget = Physics::default().stroke_n(1 << 20);
        let Some(rule) = extract_rule(&envelope.train, &mut budget) else {
            return Err("the extractor states no rule for this family");
        };
        let Ok(element) = mint_sentence_why(&rule, &envelope.train, &NoCrystal, 0) else {
            return Err("the rule does not mint over its family");
        };
        let mut outputs = Vec::with_capacity(envelope.tests.len());
        for x in &envelope.tests {
            match speak_rule(&rule, x, &NoCrystal).and_then(|s| s.grid()) {
                Some(g) => outputs.push(g),
                None => return Err("the rule does not speak a test input"),
            }
        }
        let Some(tape) = element.to_bytes() else {
            return Err("the rule's tape passes its carrier");
        };
        Ok((outputs, tape))
    }));
    let mut outcomes = Vec::with_capacity(read.len());
    for (ground, said) in read {
        // Each claim names the ground it was spoken under.
        let named = match stated(&ground) {
            Some(g) => format!("{cite:.16}… on ground {g:?}"),
            None => format!("{cite:.16}…"),
        };
        let outcome = match said {
            Err(why) => Outcome::NoRule(why),
            Ok((outputs, tape)) => match claim(session, key, &envelope.id, &outputs, &tape, stated(&ground).as_deref())? {
                Ok(o) => o,
                Err(d) => return Ok(vec![(named, Outcome::Deferred(d))]),
            },
        };
        outcomes.push((named, outcome));
    }
    Ok(outcomes)
}

/// ONE CLAIM SENT: the solve stating the ground it was spoken under, signed
/// by the voice the court pays, and the court's verdict.
fn claim(session: &Session, key: &SigningKey, id: &[u8; 32], outputs: &[SectionSample], tape: &[u8], ground: Option<&[u8]>) -> Result<Result<Outcome, Deferral>, String> {
    let Ok(outputs) = task::spoken_outputs(outputs) else {
        return Ok(Ok(Outcome::NoRule("an output is no sample")));
    };
    let Some(solve) = task::encode_solve_spoken(key, id, &outputs, Some(tape), ground) else {
        return Ok(Ok(Outcome::NoRule("the outputs pass the solve act's fields")));
    };
    // A replay the court's stroke did not finish answers UNREAD and resumes
    // where it stopped when the same solve is sent again.
    let mut asked = 0u32;
    loop {
        asked += 1;
        let reply = match session.speak(solve.clone()) {
            Ok(r) => r,
            Err(e) => return Ok(Err(deferred("solve", e)?)),
        };
        let Some((code, paid, refusal, _modes)) = task::decode_solve_verdict(&reply) else {
            return Err(format!("the court answered the solve with {} bytes that are no verdict", reply.len()));
        };
        if code == SOLVE_UNREAD && asked < RESUMES {
            continue;
        }
        let refused = refusal.map(|r| String::from_utf8_lossy(&r.reason).into_owned());
        return Ok(Ok(Outcome::Judged { code, paid, refused }));
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let (Some(home), Some(endpoint)) = (args.get(1), args.get(2)) else {
        eprintln!("usage: client <voice home> <hypha address> [tasks]");
        return ExitCode::from(2);
    };
    let tasks: u32 = match args.get(3).map(|n| n.parse()) {
        None => 1,
        Some(Ok(n)) => n,
        Some(Err(e)) => {
            eprintln!("tasks: {e}");
            return ExitCode::from(2);
        }
    };
    if let Err(e) = std::fs::create_dir_all(home) {
        eprintln!("{home}: {e}");
        return ExitCode::from(2);
    }
    // 1. IDENTITY: the voice's key, the same file the reference mind keeps.
    let (voice, key) = mycelium_sdk::voice::load_or_mint(&format!("{home}/voice.key"));
    println!("voice {}", hex32(&voice.id()));
    // 2. SESSION: one hypha's address; the session learns the rest.
    let session = Session::open(endpoint, voice);
    // THE LAW VERSION: the register the court judges by, read at the
    // version this SDK pins to; another version is told apart, never read.
    match session.speak(laws::encode_register_request()) {
        Ok(reply) => match laws::decode_register_reply(&reply) {
            Some(served) => println!("laws: version {}, {} laws", laws::LAW_VERSION, served.len()),
            None => println!("laws: the hypha's register does not read at version {}", laws::LAW_VERSION),
        },
        Err(e) => match deferral_of(&e) {
            Some(d) => {
                println!("laws: a deferral: the hypha's pool is full, deficit S:G:T:H {:?}; retry later", d.deficit);
                return ExitCode::SUCCESS;
            }
            None => {
                eprintln!("laws: {e}");
                return ExitCode::from(1);
            }
        },
    }
    for _ in 0..tasks {
        let outcomes = match one_task(&session, &key) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::from(1);
            }
        };
        for (cite, outcome) in outcomes {
            match outcome {
                Outcome::NoWork => {
                    println!("no task: the court's registry has none to give");
                    return ExitCode::SUCCESS;
                }
                Outcome::NoRule(why) => println!("task {cite} not sent: {why}"),
                Outcome::Deferred(d) => {
                    println!("a deferral: the hypha's pool is full, deficit S:G:T:H {:?}; retry later", d.deficit);
                    return ExitCode::SUCCESS;
                }
                Outcome::Judged { code, paid, refused } => match refused {
                    Some(why) => println!("task {cite} {} — refused: {why}", verdict_name(code)),
                    None => println!("task {cite} {} — paid μ {paid}", verdict_name(code)),
                },
            }
        }
    }
    ExitCode::SUCCESS
}
