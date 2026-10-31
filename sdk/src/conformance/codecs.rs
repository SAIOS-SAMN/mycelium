// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// THE CONFORMANCE READ IN PROCESS: each act a client speaks, encoded as the
// client writes it and decoded as the hypha reads it, and each reply
// encoded as the hypha writes it and decoded as the client reads it — the
// same exact values at both ends, never golden bytes (Concordance 1.4).

use super::{ActRequirement, Need, Report, StatementRead, STATEMENT};
use mycelium::anastome::dialogue as d;
use mycelium::physics::laws::codec as laws;

/// A fixed signing key: the in-process check signs as no one's voice.
fn key() -> ed25519_dalek::SigningKey {
    ed25519_dalek::SigningKey::from_bytes(&[9u8; 32])
}

fn voice() -> [u8; 32] {
    key().verifying_key().to_bytes()
}

const CITATION: [u8; 32] = [5u8; 32];

fn train() -> Vec<crate::rule::TrainPair> {
    vec![(vec![vec![1, 0], vec![0, 1]], vec![vec![2, 0], vec![0, 2]]), (vec![vec![3]], vec![vec![4]])]
}

fn tests() -> Vec<Vec<Vec<u8>>> {
    vec![vec![vec![0, 1, 2], vec![3, 4, 5]]]
}

/// `Ok(read)` where `got` equals `want`; `Err` naming what differed.
fn same<T: PartialEq + std::fmt::Debug>(what: &str, got: Option<T>, want: T) -> Result<(), String> {
    match got {
        Some(g) if g == want => Ok(()),
        Some(g) => Err(format!("{what} decoded to {g:?}, not {want:?}")),
        None => Err(format!("{what} did not decode")),
    }
}

fn task() -> Result<String, String> {
    same("the task request", d::decode_task_request(&d::encode_task_request(&voice())), voice())?;
    let bytes = d::encode_task_envelope(&CITATION, &train(), &tests()).ok_or("the envelope does not encode")?;
    let envelope = d::decode_task_envelope(&bytes).ok_or("the envelope did not decode")?;
    same("the envelope's citation", Some(envelope.id), CITATION)?;
    same("the envelope's train pairs", Some(envelope.train), train())?;
    same("the envelope's tests", Some(envelope.tests), tests())?;
    Ok("request and envelope decode to the values sent".to_string())
}

fn task_by_citation() -> Result<String, String> {
    let bytes = d::encode_task_by_citation(&voice(), &CITATION);
    same("the request by citation", d::decode_task_by_citation(&bytes), (voice(), CITATION))?;
    same("the request by citation, read as a plain request", Some(d::decode_task_request(&bytes)), None)?;
    Ok("the citation decodes, and never as a plain request".to_string())
}

fn solve() -> Result<String, String> {
    let tape = vec![1u8, 2, 3, 5, 8];
    let outputs = tests();
    let bytes = d::encode_solve_with_rule(&key(), &CITATION, &outputs, Some(&tape)).ok_or("the solve does not encode")?;
    let act = d::decode_solve(&bytes).ok_or("the solve did not decode")?;
    same("the solve's voice", Some(act.voice), voice())?;
    same("the solve's task", Some(act.task), CITATION)?;
    same("the solve's outputs", Some(act.outputs), outputs)?;
    same("the solve's rule", Some(act.rule), Some(tape))?;
    same("the solve's signature", Some(act.signed), true)?;
    let paid = d::encode_solve_verdict(d::SOLVE_BOUNTY, &[1, 0, 2, 3], None, None).ok_or("the verdict does not encode")?;
    same("the paid verdict", d::decode_solve_verdict(&paid), (d::SOLVE_BOUNTY, [1, 0, 2, 3], None, None))?;
    let refusal = d::SolveRefusal { reason: b"form leaves a remainder on the family".to_vec(), kind: 1 };
    let refused = d::encode_solve_verdict(d::SOLVE_NOTHING, &[0; 4], Some(&refusal), None).ok_or("the refusal does not encode")?;
    same("the refused verdict", d::decode_solve_verdict(&refused), (d::SOLVE_NOTHING, [0; 4], Some(refusal), None))?;
    Ok("the signed solve and both verdicts decode to the values sent".to_string())
}

fn laws_register() -> Result<String, String> {
    same("the register request", laws::decode_laws_request(&laws::encode_register_request()), laws::LawsRequest::Register)?;
    let served = laws::decode_register_reply(&laws::encode_register_reply()).ok_or("the register did not decode at its version")?;
    let want: Vec<laws::Served> = mycelium::physics::laws::LAWS
        .iter()
        .map(|l| laws::Served { law: *l, statement: l.statement().to_string() })
        .collect();
    same("the register", Some(served), want)?;
    Ok(format!("law version {}: {} laws", mycelium::physics::laws::LAW_VERSION, mycelium::physics::laws::LAWS.len()))
}

fn laws_parameters() -> Result<String, String> {
    same("the parameters request", laws::decode_laws_request(&laws::encode_parameters_request()), laws::LawsRequest::Parameters)?;
    let want: Vec<(String, u64)> = mycelium::physics::laws::PARAMETERS.iter().map(|(n, v)| (n.to_string(), *v)).collect();
    same("the parameters", laws::decode_parameters_reply(&laws::encode_parameters_reply()), (mycelium::physics::laws::LAW_VERSION, want))?;
    Ok("the parameters decode at the law version".to_string())
}

fn laws_lexicon() -> Result<String, String> {
    same("the lexicon request", laws::decode_laws_request(&laws::encode_lexicon_request()), laws::LawsRequest::Lexicon)?;
    let (version, terms) = laws::decode_lexicon_reply(&laws::encode_lexicon_reply()).ok_or("the lexicon did not decode")?;
    let lexicon = &mycelium::physics::laws::lexicon::TERMS;
    let names: Vec<&str> = terms.iter().map(|t| t.name.as_str()).collect();
    let want: Vec<&str> = lexicon.iter().map(|t| t.name).collect();
    same("the lexicon's version", Some(version), mycelium::physics::laws::lexicon::LEXICON_VERSION)?;
    same("the lexicon's terms", Some(names), want)?;
    Ok(format!("lexicon version {version}: {} terms", terms.len()))
}

fn laws_clock() -> Result<String, String> {
    same("the clock request", laws::decode_laws_request(&laws::encode_clock_request()), laws::LawsRequest::Clock)?;
    same("a clock with no head", laws::decode_clock_reply(&laws::encode_clock_reply(None)), None)?;
    Ok("the request and a headless clock decode".to_string())
}

fn laws_statement() -> Result<String, String> {
    same("the statement request", laws::decode_laws_request(&laws::encode_statement_request()), laws::LawsRequest::Statement)?;
    same("the statement", d::conformance::decode_statement_reply(&d::conformance::encode_statement_reply()), StatementRead::of(&STATEMENT))?;
    Ok(format!("{} acts", STATEMENT.acts.len()))
}

fn laws_concordance() -> Result<String, String> {
    use mycelium::physics::laws::concordance::CONCORDANCE_HASH;
    same("the concordance request", laws::decode_laws_request(&laws::encode_concordance_request()), laws::LawsRequest::Concordance)?;
    same("the concordance hash", laws::decode_concordance_reply(&laws::encode_concordance_reply()), CONCORDANCE_HASH)?;
    same("the statement's concordance hash", Some(STATEMENT.concordance_hash), CONCORDANCE_HASH)?;
    Ok("the hash decodes, and the statement carries it".to_string())
}

fn laws_signature() -> Result<String, String> {
    use mycelium::physics::laws::concordance::{verify_concordance_signature, ConcordanceSignature, CONCORDANCE_HASH};
    let s = ConcordanceSignature::sign(&key(), CONCORDANCE_HASH);
    same("the presented signature", laws::decode_laws_request(&laws::encode_signature_request(&s)), laws::LawsRequest::Signature(s.to_bytes()))?;
    same("the signature's verdict", Some(verify_concordance_signature(&s)), Ok(()))?;
    same("the accepted reply", laws::decode_signature_reply(&laws::encode_signature_reply(Ok(()))), Ok(()))?;
    Ok("the signature decodes and verifies; the reply reads back".to_string())
}

fn wallet() -> Result<String, String> {
    same("the wallet read", d::wallet::decode_wallet(&d::wallet::encode_wallet(&voice())), voice())?;
    let account = d::wallet::Account { court: [3; 32], balance: [1, 2, 3, 4], bond: [0, 1, 0, 1], held: [0; 4], at: None };
    same("the account", d::wallet::decode_wallet_reply(&d::wallet::encode_wallet_reply(&account)), account)?;
    Ok("the read and the account decode to the values sent".to_string())
}

fn observe_query() -> Result<String, String> {
    let prefix = b"court".to_vec();
    let bytes = mycelium::observe::encode_query(&prefix).ok_or("the query does not encode")?;
    same("the query", mycelium::observe::decode_act(&bytes), mycelium::observe::ObserveAct::Query(prefix))?;
    Ok("the query decodes to its prefix".to_string())
}

/// The in-process check of one act, where there is one.
fn check(a: &ActRequirement) -> Option<Result<String, String>> {
    Some(match a.name {
        "task" => task(),
        "task by citation" => task_by_citation(),
        "solve" => solve(),
        "laws register" => laws_register(),
        "laws parameters" => laws_parameters(),
        "laws lexicon" => laws_lexicon(),
        "laws clock" => laws_clock(),
        "laws statement" => laws_statement(),
        "laws concordance" => laws_concordance(),
        "laws signature" => laws_signature(),
        "wallet" => wallet(),
        "observe query" => observe_query(),
        _ => return None,
    })
}

/// THE IN-PROCESS CONFORMANCE READ of a client modelled by the acts it
/// speaks (`speaks` answers for each act of the statement): each act it
/// speaks is checked through both ends of its codec; each MUST act it does
/// not speak fails by name. A MAY act with no in-process check is unread.
pub fn check_codecs(speaks: impl Fn(&ActRequirement) -> bool) -> Report {
    let mut report = Report::default();
    for a in STATEMENT.acts {
        match (speaks(a), a.need) {
            (false, Need::Must) => report.fail(a.name, "a MUST act the client does not speak"),
            (false, Need::May) => {}
            (true, _) => match check(a) {
                Some(outcome) => report.record(a.name, outcome),
                None => report.unread(a.name, "no in-process check for this act"),
            },
        }
    }
    report
}
