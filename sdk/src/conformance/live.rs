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

/// The hypha's clause register against the SDK's: the same clauses, each
/// with the same fixtures' inputs.
fn read_clauses(session: &Speaker) -> Result<Vec<crate::signing::ConcordanceClause>, String> {
    let reply = session.speak(laws::encode_concordance_request())?;
    let served = match laws::decode_concordance_reply(&reply) {
        Some(Ok(served)) => served,
        Some(Err(r)) => return Err(format!("the hypha cannot read clause {:?} of its own register", r.id)),
        None => return Err(format!("{} bytes that are no clause register", reply.len())),
    };
    let held = mycelium::physics::laws::clauses::register().map_err(|r| format!("the SDK cannot read clause {:?}", r.id))?;
    let held: Vec<_> = held.into_iter().map(|r| r.clause).collect();
    match served == held {
        true => Ok(served),
        false => Err(format!("the served register ({} clauses) differs from the SDK's ({})", served.len(), held.len())),
    }
}

/// The signing act on the served clauses: the SDK's own reconstructions,
/// every clause enabled, admitted on each clause; and one fixture's object
/// presented wrong, refused `NotSignatory` naming that clause and fixture.
fn sign_clauses(session: &Speaker, key: &SigningKey, served: &[crate::signing::ConcordanceClause]) -> Result<(String, Option<crate::signing::ConcordanceClauseId>), String> {
    use crate::signing::{sign_concordance, Admitted, Assent, Choice, Signing, SigningRefusal};
    let signed = sign_concordance(key, served, |_| Assent::Enabled).ok_or("a served fixture the SDK does not reconstruct")?;
    let reply = session.speak(laws::encode_signature_request(&signed))?;
    let admitted: Vec<Admitted> = served.iter().map(|c| Admitted { id: c.id.clone(), assent: Assent::Enabled }).collect();
    match laws::decode_signature_reply(&reply) {
        Some(Ok(a)) if a == admitted => {}
        Some(Ok(a)) => return Err(format!("admitted as {a:?}, not every clause enabled")),
        Some(Err(refusal)) => return Err(format!("the SDK's reconstructions refused as {refusal:?}")),
        None => return Err(format!("{} bytes that are no verdict", reply.len())),
    }
    // One clause's reading moved: its first object presented for its last
    // fixture, where the two objects differ.
    let mut wrong = signed.clauses.clone();
    let moved = wrong.iter_mut().find_map(|c| match &mut c.choice {
        Choice::Enabled { reached } if reached.len() > 1 && reached.iter().any(|o| *o != reached[0]) => {
            let at = reached.iter().rposition(|o| *o != reached[0])?;
            reached[at] = reached[0].clone();
            Some((c.id.clone(), c.fixtures.get(at).cloned()?))
        }
        _ => None,
    });
    let Some((id, fixture)) = moved else { return Ok((format!("{} clauses signed by reconstruction and admitted", served.len()), None)) };
    let reply = session.speak(laws::encode_signature_request(&Signing::sign(key, wrong)))?;
    match laws::decode_signature_reply(&reply) {
        Some(Err(SigningRefusal::NotSignatory { id: i, fixture: f, .. })) if i == id && *f == fixture => Ok((
            format!("{} clauses signed by reconstruction and admitted; a fixture not reached refused NotSignatory, naming {} v{}", served.len(), id.law, id.version),
            Some(id),
        )),
        other => Err(format!("a fixture not reached answered {other:?}, not NotSignatory naming {} v{}", id.law, id.version)),
    }
}

/// Track 24 B2 — the key's alignment read from the hypha after its two
/// signings: the clause whose fixture was presented wrong collides, its
/// residue the `NotSignatory` naming it; every other served clause
/// converses; the history stands beside it.
fn read_alignment(session: &Speaker, key: &SigningKey, served: &[crate::signing::ConcordanceClause], collided: Option<&crate::signing::ConcordanceClauseId>) -> Result<String, String> {
    use crate::signing::{ClauseReading, SigningRefusal};
    let kernel = key.verifying_key().to_bytes();
    let reply = session.speak(laws::encode_alignment_request(Some(&kernel)))?;
    let read = match laws::decode_alignment_reply(&reply) {
        Some(Ok(read)) => read,
        Some(Err(refusal)) => return Err(format!("the alignment refused: {refusal:?}")),
        None => return Err(format!("{} bytes that are no alignment reading", reply.len())),
    };
    let [k] = read.kernels.as_slice() else { return Err(format!("{} kernels read for one key", read.kernels.len())) };
    for c in served {
        let reading = k.clauses.iter().find(|r| r.id == c.id).map(|r| &r.reading);
        let lawful = match (reading, collided == Some(&c.id)) {
            (Some(ClauseReading::Collides { residue, .. }), true) => matches!(**residue, SigningRefusal::NotSignatory { ref id, .. } if *id == c.id),
            (Some(ClauseReading::Converses { .. }), false) => true,
            _ => false,
        };
        if !lawful {
            return Err(format!("{} v{} read as {reading:?}", c.id.law, c.id.version));
        }
    }
    Ok(format!("{} clauses read from the ledger, each as signed; {} signings of the past beside them", served.len(), read.history.len()))
}

/// The hypha's clause register against the SDK's, a signing act over it by
/// the SDK's own reconstructions, and the alignment the hypha's ledger then
/// reads (MAY for a mind, MUST for a kernel).
fn laws_concordance(session: &Speaker, key: &SigningKey, report: &mut Report) {
    let served = read_clauses(session);
    let clauses = served.as_ref().ok().cloned();
    session.record(report, "laws concordance", served.map(|s| format!("the served register equals the SDK's: {} clauses", s.len())));
    let Some(served) = clauses else {
        report.unread("laws signature", "no clause register read to sign");
        report.unread("laws alignment", "no clause register read to sign");
        return;
    };
    let signed = sign_clauses(session, key, &served);
    let collided = signed.as_ref().ok().map(|(_, id)| id.clone());
    session.record(report, "laws signature", signed.map(|(said, _)| said));
    match collided {
        Some(collided) => {
            let aligned = read_alignment(session, key, &served, collided.as_ref()).and_then(|read| match collided {
                Some(_) => resign(session, key, &served).map(|again| format!("{read}; {again}")),
                None => Ok(read),
            });
            session.record(report, "laws alignment", aligned);
        }
        None => report.unread("laws alignment", "no signing admitted to read"),
    }
}

/// Track 24 B4 — THE KEY SIGNS AGAIN, every fixture reached, after the
/// signing that collided: a re-signing is a new event and the latest reads,
/// so every clause converses again and the key's universe crosses its
/// universal core tube (a domain whose reserve does not converse on every
/// network clause has its crossings refused `NotSignatory`).
fn resign(session: &Speaker, key: &SigningKey, served: &[crate::signing::ConcordanceClause]) -> Result<String, String> {
    use crate::signing::{sign_concordance, Assent, ClauseReading};
    let signed = sign_concordance(key, served, |_| Assent::Enabled).ok_or("a served fixture the SDK does not reconstruct")?;
    match laws::decode_signature_reply(&session.speak(laws::encode_signature_request(&signed))?) {
        Some(Ok(_)) => {}
        other => return Err(format!("the signing again answered {other:?}")),
    }
    let kernel = key.verifying_key().to_bytes();
    let reply = session.speak(laws::encode_alignment_request(Some(&kernel)))?;
    let Some(Ok(read)) = laws::decode_alignment_reply(&reply) else { return Err("the alignment after signing again is unread".to_string()) };
    let [k] = read.kernels.as_slice() else { return Err(format!("{} kernels read for one key", read.kernels.len())) };
    match served.iter().find(|c| !k.clauses.iter().any(|r| r.id == c.id && matches!(r.reading, ClauseReading::Converses { .. }))) {
        Some(c) => Err(format!("{} v{} does not converse after signing again", c.id.law, c.id.version)),
        None => Ok("signed again, every clause converses".to_string()),
    }
}

/// One claim: the ground it states, its outputs and its rule's tape.
type Claim = (Option<Vec<u8>>, Vec<crate::rule::SectionSample>, Option<Vec<u8>>);

/// The outputs a solve claims: the rule the SDK's extractor states, spoken
/// on each test input, with its tape; or, where no rule stands, the test
/// inputs themselves and no tape — a solve the court refuses by name, which
/// still exercises the act and its verdict. Read once per perspective of
/// the family's ground (MU-SENSING §3.4, §3.8): each claim states the
/// ground it was spoken under, and every perspective's claim is kept.
fn claims(envelope: &d::TaskSection) -> Vec<Claim> {
    let read = crate::rule::on_each_perspective_of(&envelope.train, || {
        let mut budget = Physics::default().stroke_n(1 << 20);
        extract_rule(&envelope.train, &mut budget).and_then(|rule| {
            let element = mint_sentence_why(&rule, &envelope.train, &NoCrystal, 0).ok()?;
            let outputs = envelope.tests.iter().map(|x| speak_rule(&rule, x, &NoCrystal).and_then(|s| s.sample())).collect::<Option<Vec<_>>>()?;
            Some((outputs, Some(element.to_bytes()?)))
        })
    });
    read.into_iter()
        .map(|(ground, said)| match said {
            Some((outputs, tape)) => (crate::rule::stated(&ground), outputs, tape),
            None => (crate::rule::stated(&ground), envelope.tests.clone(), None),
        })
        .collect()
}

fn solve(session: &Speaker, key: &SigningKey, envelope: &d::TaskSection) -> Result<String, String> {
    let mut said = Vec::new();
    for (ground, outputs, tape) in claims(envelope) {
        said.push(solve_one(session, key, envelope, ground.as_deref(), &outputs, tape.as_deref())?);
    }
    Ok(said.join("; "))
}

fn solve_one(session: &Speaker, key: &SigningKey, envelope: &d::TaskSection, ground: Option<&[u8]>, outputs: &[crate::rule::SectionSample], tape: Option<&[u8]>) -> Result<String, String> {
    // Track 26 S5: the spoken outputs go out as samples, each at its place.
    let outputs = d::spoken_outputs(outputs).map_err(|e| format!("an output is no sample: {e:?}"))?;
    // Track 26 S6: the claim states the section's perspective, the one its
    // outputs were read through.
    let perspective = crate::rule::SectionSample::section_perspective();
    let act = d::encode_solve_spoken(key, &envelope.id, &outputs, tape, ground, &perspective).ok_or("the outputs pass the solve act's fields")?;
    let mut asked = 0u32;
    loop {
        asked += 1;
        let reply = session.speak(act.clone())?;
        let (code, paid, refusal, modes) =
            d::decode_solve_verdict(&reply).ok_or_else(|| format!("{} bytes that do not decode as a verdict", reply.len()))?;
        match code {
            d::SOLVE_UNREAD if asked < RESUMES => continue,
            d::SOLVE_UNREAD => return Ok(format!("not judged yet after {asked} asks (a verdict, read)")),
            d::SOLVE_NOTHING if !paid.is_zero() => return Err(format!("a final refusal paid μ {paid}")),
            d::SOLVE_NOTHING => {
                let why = match refusal {
                    Some(r) => String::from_utf8_lossy(&r.reason).into_owned(),
                    None => "no reason named".to_string(),
                };
                return Ok(format!("not solved (final), refused: {why}"));
            }
            d::SOLVE_BOUNTY | d::SOLVE_AGAIN if refusal.is_some() => return Err(format!("code {code} pays and refuses at once")),
            d::SOLVE_AGAIN if !paid.is_zero() => return Err(format!("a repeat closure paid μ {paid}")),
            d::SOLVE_BOUNTY => {
                let modes = modes.as_ref().map_or_else(|| "none carried".to_string(), mycelium::economy::modes::Modes::print);
                return Ok(format!("solved (bounty), paid μ {paid}; modes {modes}"));
            }
            d::SOLVE_AGAIN => return Ok("solved again (a repeat closure, verified, paid nothing)".to_string()),
            other => return Err(format!("verdict code {other} is none of the four the statement names")),
        }
    }
}

fn task_by_citation(session: &Speaker, voice: &[u8; 32], envelope: &d::TaskFamily) -> Result<String, String> {
    let reply = session.speak(d::encode_task_by_citation(voice, &envelope.id))?;
    let again = d::decode_task(&reply).ok_or_else(|| format!("{} bytes that do not decode as an envelope", reply.len()))?;
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
    let Some(envelope) = d::decode_task(&reply) else {
        report.fail("task", format!("{} bytes that do not decode as an envelope", reply.len()));
        return report.unread("solve", "no task to solve");
    };
    match envelope.train.is_empty() || envelope.tests.is_empty() {
        true => report.fail("task", "an envelope with no train pair or no test input"),
        false => report.pass("task", format!("{} train pairs, {} test inputs", envelope.train.len(), envelope.tests.len())),
    }
    let by_citation = task_by_citation(session, &voice, &envelope);
    session.record(report, "task by citation", by_citation);
    // Track 26 S4b: this client's rule seam spells the alphabet's section;
    // a task of another rank or width is carried exactly and left unread
    // here, never misread.
    let Ok(section) = envelope.section() else {
        return report.unread("solve", "the task's samples are off the alphabet's section this client spells");
    };
    let solved = solve(session, key, &section);
    session.record(report, "solve", solved);
}

/// THE CONFORMANCE SUITE against the hypha at `endpoint`, spoken as the voice
/// `key` seats, for a client declaring `profiles`: the law register read and
/// checked, the served statement, the Concordance's clause register and a
/// signing act over it by the SDK's own reconstructions (and a fixture not
/// reached, refused `NotSignatory`); for a mind, a task taken, asked by its citation and solved; for a
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
