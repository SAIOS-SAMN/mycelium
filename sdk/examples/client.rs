// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// A MINIMAL CLIENT ON THE SDK: what a third-party mind does to work on the
// network, end to end, with `mycelium-sdk` as its only Mycelium dependency.
//
//! `client <voice home> <hypha address> [tasks]`
//!
//! 1. Identity — the voice's Ed25519 key, read from `<voice home>/voice.key`
//!    or minted there (`mycelium-client --init` writes the same file).
//! 2. Session — `Session::open` on a hypha's address.
//! 3. Work — `ACT_TASK` asks the court for a task; the envelope carries the
//!    train pairs and the test inputs, never the test outputs.
//! 4. Rule — the task's family read by the SDK's extractor
//!    (`rule::extract_rule`), minted as an element the court can replay
//!    (`rule::mint_sentence_why`), and spoken on each test input.
//! 5. Solve — the outputs and the rule's tape, signed by the voice
//!    (`task::encode_solve_with_rule`); the court replays the rule on pairs
//!    the client never saw and answers a verdict. A replay the court's stroke
//!    did not finish answers `SOLVE_UNREAD`; the same solve sent again
//!    resumes it.
//!
//! Exit 0 where every task asked was answered by the court (solved or
//! refused by name); 1 where the court could not be reached.

use mycelium_sdk::prelude::*;
use mycelium_sdk::rule::{extract_rule, mint_sentence_why, speak_rule, Physics};
use mycelium_sdk::task;
use mycelium_sdk::voice::hex32;
use std::process::ExitCode;

/// What one task came to.
enum Outcome {
    /// The court answered the solve: its verdict code, the μ paid, and the
    /// refusal it named where it refused.
    Judged { code: u8, paid: [u64; 4], refused: Option<String> },
    /// The client could not state a rule for the task; nothing was sent.
    NoRule(&'static str),
    /// The court had no task to give.
    NoWork,
}

/// How many times one solve is sent while the court answers it unread.
const RESUMES: u32 = 16;

/// The verdict codes the court answers a solve with.
fn verdict_name(code: u8) -> &'static str {
    match code {
        SOLVE_BOUNTY => "solved: first verified closure (bounty)",
        SOLVE_FLOOR => "solved again: verification floor",
        SOLVE_NOTHING => "not solved (final)",
        SOLVE_UNREAD => "not judged yet (retry resumes the replay)",
        _ => "other",
    }
}

/// ONE TASK, ASKED AND ANSWERED: `Err` where the court could not be reached
/// or answered with bytes this build does not read.
fn one_task(session: &Session, key: &SigningKey) -> Result<(String, Outcome), String> {
    let voice = key.verifying_key().to_bytes();
    let reply = session.speak(task::encode_task_request(&voice)).map_err(|e| format!("task request: {e}"))?;
    if reply.is_empty() {
        return Ok((String::new(), Outcome::NoWork));
    }
    let Some(envelope) = task::decode_task_envelope(&reply) else {
        return Err(format!("the court answered the task request with {} bytes that are no task envelope (a deferral: retry later)", reply.len()));
    };
    let cite = hex32(&envelope.id);
    // THE RULE: read off the train pairs on a stroke of the SDK's own
    // metabolism, minted as an element the court replays.
    let mut budget = Physics::default().stroke_n(1 << 20);
    let Some(rule) = extract_rule(&envelope.train, &mut budget) else {
        return Ok((cite, Outcome::NoRule("the extractor states no rule for this family")));
    };
    let Ok(element) = mint_sentence_why(&rule, &envelope.train, &NoCrystal, 0) else {
        return Ok((cite, Outcome::NoRule("the rule does not mint over its family")));
    };
    let mut outputs = Vec::with_capacity(envelope.tests.len());
    for x in &envelope.tests {
        match speak_rule(&rule, x, &NoCrystal).and_then(|s| s.grid()) {
            Some(g) => outputs.push(g),
            None => return Ok((cite, Outcome::NoRule("the rule does not speak a test input"))),
        }
    }
    let Some(tape) = element.to_bytes() else {
        return Ok((cite, Outcome::NoRule("the rule's tape passes its carrier")));
    };
    // THE SOLVE, signed by the voice the court pays.
    let Some(solve) = task::encode_solve_with_rule(key, &envelope.id, &outputs, Some(&tape)) else {
        return Ok((cite, Outcome::NoRule("the outputs pass the solve act's fields")));
    };
    // A replay the court's stroke did not finish answers UNREAD and resumes
    // where it stopped when the same solve is sent again.
    let mut asked = 0u32;
    loop {
        asked += 1;
        let reply = session.speak(solve.clone()).map_err(|e| format!("solve: {e}"))?;
        let Some((code, paid, refusal, _modes)) = task::decode_solve_verdict(&reply) else {
            return Err(format!("the court answered the solve with {} bytes that are no verdict", reply.len()));
        };
        if code == SOLVE_UNREAD && asked < RESUMES {
            continue;
        }
        let refused = refusal.map(|r| String::from_utf8_lossy(&r.reason).into_owned());
        return Ok((cite, Outcome::Judged { code, paid, refused }));
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
    for _ in 0..tasks {
        match one_task(&session, &key) {
            Ok((_, Outcome::NoWork)) => {
                println!("no task: the court's registry has none to give");
                break;
            }
            Ok((cite, Outcome::NoRule(why))) => println!("task {cite:.16}… not sent: {why}"),
            Ok((cite, Outcome::Judged { code, paid, refused })) => match refused {
                Some(why) => println!("task {cite:.16}… {} — refused: {why}", verdict_name(code)),
                None => println!("task {cite:.16}… {} — paid μ {paid:?}", verdict_name(code)),
            },
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::from(1);
            }
        }
    }
    ExitCode::SUCCESS
}
