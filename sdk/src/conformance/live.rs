// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// THE CONFORMANCE SUITE ON THE NETWORK: each MUST act of the declared
// profiles spoken to a running hypha, its reply decoded and its exact values
// checked, each failure named; the served statement read against the
// statement held. An act the hypha defers is reported deferred, never as a
// reply that did not decode.

use super::speaker::Speaker;
use super::{Profile, Report, StatementRead, STATEMENT};
use crate::rule::{extract_rule, mint_sentence_why, speak_rule, NoCrystal, Physics};
use crate::voice::SigningKey;
use mycelium::anastome::dialogue as d;
use mycelium::physics::laws::codec as laws;

/// How many times one solve is sent while the court answers it unread.
const RESUMES: u32 = 16;

fn laws_register(session: &Speaker) -> Result<String, String> {
    let reply = session.speak(laws::encode_register_request())?;
    let law_version = mycelium::physics::laws::LAW_VERSION;
    let served = laws::decode_register_reply(&reply)
        .ok_or_else(|| format!("{} bytes that do not decode as the register at law version {law_version}", reply.len()))?;
    let held = mycelium::physics::laws::LAWS;
    if served.len() != held.len() {
        return Err(format!("{} laws served, {} held at law version {law_version}", served.len(), held.len()));
    }
    for (s, law) in served.iter().zip(held) {
        if s.law != law || s.statement != law.statement() {
            return Err(format!("law {} served as {:?}, not as held", law.index(), s.law));
        }
    }
    Ok(format!("law version {law_version}: {} laws, each at its coordinate with its statement", served.len()))
}

fn laws_statement(session: &Speaker, report: &mut Report) {
    let act = "laws statement";
    let reply = match session.speak(laws::encode_statement_request()) {
        Ok(r) => r,
        Err(e) => return session.unread(report, act, format!("{e} (a hypha before the statement's mode answers none)")),
    };
    let outcome = match d::conformance::decode_statement_reply(&reply) {
        None => Err(format!("{} bytes that do not decode as a statement", reply.len())),
        Some(read) if read == StatementRead::of(&STATEMENT) => Ok(format!("the served statement equals the SDK's: {} profiles", read.profiles.len())),
        Some(read) => Err(format!(
            "the served statement (wire 0x{:04x}, law version {}, {} profiles) differs from the SDK's",
            read.wire_version,
            read.law_version,
            read.profiles.len()
        )),
    };
    report.record(act, outcome);
}

/// The hypha's Concordance hash against the SDK's, and a signature over it
/// presented and accepted (MAY for a mind, MUST for a kernel: a hypha before
/// v0.3.0 answers neither, and they are unread there).
fn laws_concordance(session: &Speaker, key: &SigningKey, report: &mut Report) {
    use mycelium::physics::laws::concordance::{ConcordanceSignature, CONCORDANCE_HASH};
    let hash = match session.speak(laws::encode_concordance_request()) {
        Ok(r) => r,
        Err(e) => return session.unread(report, "laws concordance", e),
    };
    let outcome = match laws::decode_concordance_reply(&hash) {
        Some(h) if h == CONCORDANCE_HASH => Ok("the served hash equals the SDK's".to_string()),
        Some(_) => Err("the served hash is another version of the Concordance".to_string()),
        None => Err(format!("{} bytes that are no hash", hash.len())),
    };
    report.record("laws concordance", outcome);
    let signed = ConcordanceSignature::sign(key, CONCORDANCE_HASH);
    let outcome = match session.speak(laws::encode_signature_request(&signed)) {
        Err(e) => return session.unread(report, "laws signature", e),
        Ok(r) => match laws::decode_signature_reply(&r) {
            Some(Ok(())) => Ok("a signature over the hash, accepted".to_string()),
            Some(Err(refusal)) => Err(format!("a good signature refused as {refusal:?}")),
            None => Err(format!("{} bytes that are no verdict", r.len())),
        },
    };
    report.record("laws signature", outcome);
}

/// The outputs a solve claims: the rule the SDK's extractor states, spoken
/// on each test input, with its tape; or, where no rule stands, the test
/// inputs themselves and no tape — a solve the court refuses by name, which
/// still exercises the act and its verdict.
fn outputs(envelope: &d::TaskEnvelope) -> (Vec<Vec<Vec<u8>>>, Option<Vec<u8>>) {
    let mut budget = Physics::default().stroke_n(1 << 20);
    let stated = extract_rule(&envelope.train, &mut budget).and_then(|rule| {
        let element = mint_sentence_why(&rule, &envelope.train, &NoCrystal, 0).ok()?;
        let outputs = envelope.tests.iter().map(|x| speak_rule(&rule, x, &NoCrystal).and_then(|s| s.grid())).collect::<Option<Vec<_>>>()?;
        Some((outputs, Some(element.to_bytes()?)))
    });
    match stated {
        Some(s) => s,
        None => (envelope.tests.clone(), None),
    }
}

fn solve(session: &Speaker, key: &SigningKey, envelope: &d::TaskEnvelope) -> Result<String, String> {
    let (outputs, tape) = outputs(envelope);
    let act = d::encode_solve_with_rule(key, &envelope.id, &outputs, tape.as_deref()).ok_or("the outputs pass the solve act's fields")?;
    let mut asked = 0u32;
    loop {
        asked += 1;
        let reply = session.speak(act.clone())?;
        let (code, paid, refusal, _modes) =
            d::decode_solve_verdict(&reply).ok_or_else(|| format!("{} bytes that do not decode as a verdict", reply.len()))?;
        match code {
            d::SOLVE_UNREAD if asked < RESUMES => continue,
            d::SOLVE_UNREAD => return Ok(format!("not judged yet after {asked} asks (a verdict, read)")),
            d::SOLVE_NOTHING if paid != [0; 4] => return Err(format!("a final refusal paid μ {paid:?}")),
            d::SOLVE_NOTHING => {
                let why = match refusal {
                    Some(r) => String::from_utf8_lossy(&r.reason).into_owned(),
                    None => "no reason named".to_string(),
                };
                return Ok(format!("not solved (final), refused: {why}"));
            }
            d::SOLVE_BOUNTY | d::SOLVE_FLOOR if refusal.is_some() => return Err(format!("code {code} pays and refuses at once")),
            d::SOLVE_BOUNTY => return Ok(format!("solved (bounty), paid μ {paid:?}")),
            d::SOLVE_FLOOR => return Ok(format!("solved again (floor), paid μ {paid:?}")),
            other => return Err(format!("verdict code {other} is none of the four the statement names")),
        }
    }
}

fn task_by_citation(session: &Speaker, voice: &[u8; 32], envelope: &d::TaskEnvelope) -> Result<String, String> {
    let reply = session.speak(d::encode_task_by_citation(voice, &envelope.id))?;
    let again = d::decode_task_envelope(&reply).ok_or_else(|| format!("{} bytes that do not decode as an envelope", reply.len()))?;
    if again.id != envelope.id || again.train != envelope.train || again.tests != envelope.tests {
        return Err("the task asked by its citation is not the task given".to_string());
    }
    Ok("the task by its citation is the task given".to_string())
}

/// The mind's loop: a task taken and its envelope decoded, the task asked by
/// its citation, a solve sent and its verdict decoded.
fn mind_loop(session: &Speaker, key: &SigningKey, report: &mut Report) {
    let voice = key.verifying_key().to_bytes();
    let reply = match session.speak(d::encode_task_request(&voice)) {
        Ok(r) => r,
        Err(e) => {
            session.fail(report, "task", e);
            return report.unread("solve", "no task to solve");
        }
    };
    if reply.is_empty() {
        report.unread("task", "the registry gave no task (an empty envelope: no work)");
        return report.unread("solve", "no task to solve");
    }
    let Some(envelope) = d::decode_task_envelope(&reply) else {
        report.fail("task", format!("{} bytes that do not decode as an envelope", reply.len()));
        return report.unread("solve", "no task to solve");
    };
    match envelope.train.is_empty() || envelope.tests.is_empty() {
        true => report.fail("task", "an envelope with no train pair or no test input"),
        false => report.pass("task", format!("{} train pairs, {} test inputs", envelope.train.len(), envelope.tests.len())),
    }
    let by_citation = task_by_citation(session, &voice, &envelope);
    session.record(report, "task by citation", by_citation);
    let solved = solve(session, key, &envelope);
    session.record(report, "solve", solved);
}

/// THE CONFORMANCE SUITE against the hypha at `endpoint`, spoken as the voice
/// `key` seats, for a client declaring `profiles`: the law register read and
/// checked, the served statement, the Concordance's hash and a signature over
/// it; for a mind, a task taken, asked by its citation and solved; for a
/// kernel, the acts of its uplink (`Profile::Kernel`: the clock, transitions
/// verified, a domain founded on the voice's key and its universe endowed
/// and read, its seats posted). A kernel is never asked task or solve unless
/// it also declares the mind profile. Every MUST act of a declared profile
/// passes, or its failure is named; a hypha that cannot be reached fails
/// each act with the reason. An act the hypha answers with the deferral
/// (its worker pool full) is reported [`super::Finding::Deferred`] with the
/// door's deficit, never retried here: the session exposes no measured
/// round trip to pace a retry by, so the suite is run again later.
pub fn run(endpoint: &str, key: &SigningKey, profiles: &[Profile]) -> Report {
    let mut report = Report::of(profiles);
    let session = Speaker::open(endpoint, key);
    let register = laws_register(&session);
    session.record(&mut report, "laws register", register);
    laws_statement(&session, &mut report);
    laws_concordance(&session, key, &mut report);
    if profiles.contains(&Profile::Kernel) {
        super::uplink::run(&session, key, &mut report);
    }
    if profiles.contains(&Profile::Mind) {
        mind_loop(&session, key, &mut report);
    }
    report
}
