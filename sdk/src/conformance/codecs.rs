// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// THE CONFORMANCE READ IN PROCESS: each act a client speaks, encoded as the
// client writes it and decoded as the hypha reads it, and each reply
// encoded as the hypha writes it and decoded as the client reads it — the
// same exact values at both ends, never golden bytes (Concordance 1.4).

use super::{ActRequirement, Need, Profile, Report, StatementRead, STATEMENT};
use mycelium::anastome::dialogue as d;
use mycelium::anastome::dialogue::domain as dom;
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
    let acts: Vec<String> = STATEMENT.profiles.iter().map(|p| format!("{} {}", p.acts.len(), p.profile.name())).collect();
    Ok(format!("acts by profile: {}", acts.join(", ")))
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

fn laws_verify() -> Result<String, String> {
    use mycelium::physics::laws::{verify_sized, Law};
    let ts = super::probes::transitions().ok_or("the turn carries no body")?;
    for t in &ts {
        same("the verify request", laws::decode_laws_request(&laws::encode_verify_request(t)), laws::LawsRequest::Verify(t.clone()))?;
        let v = verify_sized(t);
        same("the verdict", laws::decode_verify_reply(&laws::encode_verify_reply(&v)), v)?;
    }
    let [lawful, still] = ts.map(|t| verify_sized(&t));
    same("the lawful turn admitted", Some(lawful.admitted()), true)?;
    same("the still height's breaches", Some(still.breaches.iter().map(|(l, _)| *l).collect::<Vec<_>>()), vec![Law::Time])?;
    Ok("both transitions and the verdict on each decode; the turn admitted, the still height refused on time".to_string())
}

fn laws_batch() -> Result<String, String> {
    let ts = super::probes::transitions().ok_or("the turn carries no body")?.to_vec();
    same("the batch request", laws::decode_laws_request(&laws::encode_batch_request(&ts)), laws::LawsRequest::Batch(ts.clone()))?;
    let verdicts: Vec<_> = ts.iter().map(mycelium::physics::laws::verify_sized).collect();
    same("the batch's verdicts", laws::decode_batch_reply(&laws::encode_batch_reply(&verdicts)), verdicts)?;
    Ok("the batch and its verdicts decode in order".to_string())
}

fn stamp() -> mycelium::chronometry::Stamp {
    mycelium::chronometry::Stamp { h: 7, vk: voice(), k: 1, i: 2 }
}

/// A domain act read at the hypha as `want`, and each `Founded` reply read
/// back exactly.
fn domain_act(what: &str, bytes: &[u8], want: dom::DomainRequest, replies: &[dom::Founded]) -> Result<(), String> {
    same(what, dom::decode_domain_request(bytes), want)?;
    for f in replies {
        same("the court's answer", dom::decode_domain_reply(&dom::encode_domain_reply(f)), f.clone())?;
    }
    Ok(())
}

fn domain_founding() -> Result<String, String> {
    let signed = dom::DomainGenesis { domain: voice(), tiers: vec![voice()] }.sign(&key());
    let replies = [dom::Founded::Sealed { at: stamp(), receipt: CITATION }, dom::Founded::AlreadyFounded, dom::Founded::TierHeld];
    domain_act("the genesis", &dom::encode_domain(&signed), dom::DomainRequest::Genesis(signed.clone()), &replies)?;
    Ok("the signed genesis and the answers decode".to_string())
}

fn domain_designation() -> Result<String, String> {
    let signed = dom::Reservation { domain: voice(), tier: voice(), nonce: CITATION }.sign(&key());
    let replies = [dom::Founded::Reserved { at: stamp(), receipt: CITATION, now: true }, dom::Founded::NotATier];
    domain_act("the designation", &dom::encode_reservation(&signed), dom::DomainRequest::Reserve(signed.clone()), &replies)?;
    Ok("the designation and the answers decode".to_string())
}

fn domain_origin() -> Result<String, String> {
    let signed = super::probes::origin(voice()).sign(&key());
    let replies = [dom::Founded::Originated { at: stamp(), receipt: CITATION }, dom::Founded::AlreadyOriginated];
    domain_act("the origin", &dom::encode_origin(&signed), dom::DomainRequest::Origin(signed.clone()), &replies)?;
    Ok("the origin, its universe, and the answers decode".to_string())
}

fn domain_statement() -> Result<String, String> {
    let stated = d::universe::Stated::Basket(vec![(1, (1, 1)), (8, (2, 3))]);
    let signed = d::universe::Statement { domain: voice(), stated }.sign(&key());
    let replies = [dom::Founded::Stated { at: stamp(), receipt: CITATION }, dom::Founded::AlreadyStated, dom::Founded::NotAUniverse];
    domain_act("the statement", &d::universe::encode_statement(&signed), dom::DomainRequest::State(signed.clone()), &replies)?;
    Ok("the basket stated and the answers decode".to_string())
}

fn domain_reserve() -> Result<String, String> {
    same("the reserve read", dom::decode_domain_request(&dom::encode_reserve_read(voice())), dom::DomainRequest::ReadReserve(voice()))?;
    let r = dom::ReserveReading { current: Some(voice()), pending: None, balance: [1, 0, 2, 3] };
    same("the reserve", dom::decode_reserve_reading(&dom::encode_reserve_reading(&r)), r)?;
    Ok("the read and the reserve decode".to_string())
}

fn domain_origin_reading() -> Result<String, String> {
    same("the origin read", dom::decode_domain_request(&dom::encode_origin_read(voice())), dom::DomainRequest::ReadOrigin(voice()))?;
    let o = super::probes::origin(voice());
    let r = dom::OriginReading { kind: o.kind, version: o.version, commitment: o.commitment, at: stamp(), receipt: CITATION };
    same("the origin held", dom::decode_origin_reading(&dom::encode_origin_reading(Some(&r))), Some(r))?;
    same("no origin", dom::decode_origin_reading(&dom::encode_origin_reading(None)), None)?;
    Ok("the read and both readings decode".to_string())
}

fn domain_manifest() -> Result<String, String> {
    use d::endowment::{decode_grant_reply, encode_grant_reply, encode_manifest, GrantRefusal, Granted};
    let signed = super::probes::manifest(voice()).sign(&key()).ok_or("the manifest does not sign")?;
    let bytes = encode_manifest(&signed).ok_or("the manifest does not encode")?;
    same("the manifest", dom::decode_domain_request(&bytes), dom::DomainRequest::Manifest(Box::new(signed)))?;
    for g in [Granted::Sealed { mu: [1, 0, 2, 3], founding: true, at: stamp(), receipt: CITATION }, Granted::Refused(GrantRefusal::Short)] {
        same("the grant", decode_grant_reply(&encode_grant_reply(&g)), g.clone())?;
    }
    Ok("the manifest, a grant and a refusal decode".to_string())
}

fn domain_universe() -> Result<String, String> {
    use d::universe::{decode_universe_reading, encode_universe_read, encode_universe_reading, UniverseReading};
    same("the universe read", dom::decode_domain_request(&encode_universe_read(voice())), dom::DomainRequest::ReadUniverse(voice()))?;
    let g = super::probes::genesis();
    let r = UniverseReading { originated: true, geometry: Some(g.geometry), parent: Some(g.parent), basket: vec![(1, (1, 1))] };
    same("the universe", decode_universe_reading(&encode_universe_reading(&r)), r)?;
    Ok("the read and the universe decode".to_string())
}

fn domain_endowment() -> Result<String, String> {
    use d::endowment::{decode_endowment_reading, encode_endowment_read, encode_endowment_reading, EndowmentReading, GrantRead};
    same("the endowment read", dom::decode_domain_request(&encode_endowment_read(voice())), dom::DomainRequest::ReadEndowment(voice()))?;
    let m = super::probes::manifest(voice());
    let grant = GrantRead { manifest: m.hash().ok_or("the manifest has no hash")?, mu: [1, 0, 2, 3], founding: true, applied: false };
    let r = EndowmentReading { earned: [9, 9, 9, 9], founding: [1, 0, 2, 3], growth: [0; 4], grants: vec![grant], nuclei: m.nuclei };
    let bytes = encode_endowment_reading(&r).ok_or("the endowment does not encode")?;
    same("the endowment", decode_endowment_reading(&bytes), r)?;
    Ok("the read and the endowment decode".to_string())
}

fn seat_post() -> Result<String, String> {
    use d::seat::{decode_post_reply, decode_seat_request, encode_post, encode_post_reply, Refusal, SeatRequest};
    let signed = super::probes::seats(voice()).sign(&key()).ok_or("the listing does not sign")?;
    let bytes = encode_post(&signed).ok_or("the listing does not encode")?;
    same("the listing posted", decode_seat_request(&bytes), Ok(SeatRequest::Post(signed)))?;
    for r in [Ok(5), Err(Refusal::NotTheHouse), Err(Refusal::UnknownDomain)] {
        same("the relay's answer", decode_post_reply(&encode_post_reply(r)), r)?;
    }
    Ok("the signed listing and the answers decode".to_string())
}

fn seat_listing() -> Result<String, String> {
    use d::seat::{decode_listing_reply, decode_seat_request, encode_read_listing, SeatRequest};
    same("the listing read", decode_seat_request(&encode_read_listing(&voice(), None)), Ok(SeatRequest::ReadListing(voice(), None)))?;
    let signed = super::probes::seats(voice()).sign(&key()).ok_or("the listing does not sign")?;
    let body = signed.body().ok_or("the listing has no body")?;
    same("the listing served", decode_listing_reply(&body, None), Some(signed))?;
    same("no listing", decode_listing_reply(&[], None), None)?;
    Ok("the read and the listing served decode".to_string())
}

fn seat_mail() -> Result<String, String> {
    use d::seat::{decode_mail_reply_standing, decode_seat_request, encode_mail_reply_standing, encode_read_mail, Mail, MailRead, ReadMail, SeatRequest};
    let read = ReadMail::sign_standing(&key(), 3, 11, CITATION);
    same("the mail read", decode_seat_request(&encode_read_mail(&read)), Ok(SeatRequest::ReadMail(read)))?;
    let mail = MailRead { epoch: 11, seq: 4, mail: vec![Mail { seq: 4, sub: 1, body: vec![1, 2, 3] }] };
    let bytes = encode_mail_reply_standing(Ok(&mail)).ok_or("the mail does not encode")?;
    same("the mail at the relay's standing", decode_mail_reply_standing(&bytes), Ok(mail))?;
    Ok("the signed read and the mail decode".to_string())
}

fn address(b: &[u8]) -> mycelium::transducer::address::Address {
    mycelium::transducer::address::Address::new(b.to_vec())
}

fn quote() -> Result<String, String> {
    let request = d::encode_quote_request(&voice()).ok_or("the quote request does not encode")?;
    same("the quote request", d::decode_quote_request(&request), (voice(), Vec::new()))?;
    let items = vec![d::WorkItem { addr: address(&[1, 2, 3]), mu: [1, 0, 0, 1], element: vec![7, 8], open: Vec::new() }];
    let bytes = d::encode_quote_reply_within(9, &[1, 2, 3, 4], 5, &items, mycelium::network::wire::BODY_BUDGET).ok_or("the quote does not encode")?;
    same("the quote", d::decode_quote_reply(&bytes), (9, [1, 2, 3, 4], 5, items))?;
    Ok("the request and a quote of one work item decode".to_string())
}

fn receipts() -> Result<String, String> {
    use mycelium::chronometry::{ActKind, KStepReceipt, Place};
    let (after, upto) = (Place { k: 1, i: 2 }, Place { k: 3, i: 4 });
    let want = d::ReceiptsRequest { voice: voice(), signer: CITATION, after: Some(after), upto, replay: false };
    same("the receipts request", d::decode_receipts_request(&d::encode_receipts_request(&voice(), &CITATION, Some(after), upto)), want)?;
    let receipt = KStepReceipt {
        stamp: stamp(),
        kind: ActKind::Domain,
        mu: [1, 0, 2, 3],
        prev: None,
        parents: Vec::new(),
        cites: Vec::new(),
        operator_hash: CITATION,
        chain_hash: [6; 32],
    };
    let bytes = d::encode_receipts_reply(std::slice::from_ref(&receipt), mycelium::network::wire::BODY_BUDGET);
    same("the receipts", d::decode_receipts_reply(&bytes), vec![receipt])?;
    Ok("the span asked and a page of one receipt decode".to_string())
}

fn rows() -> Result<String, String> {
    use mycelium::chronometry::Place;
    use mycelium::economy::{Arm, Entry, Kind};
    let upto = Place { k: 3, i: 4 };
    let want = d::RowsRequest { voice: voice(), signer: CITATION, from: None, upto, cursor: 7 };
    same("the rows request", d::decode_rows_request(&d::encode_rows_request(&voice(), &CITATION, None, upto, 7)), want)?;
    let row = Entry { kind: Kind::Transfer, who: voice(), mu: [1, 0, 2, 3], at: stamp(), receipt: CITATION, arm: Some(Arm::Pouw) };
    let bytes = d::encode_rows_reply(12, std::slice::from_ref(&row), mycelium::network::wire::BODY_BUDGET).ok_or("the rows do not encode")?;
    same("the rows", d::decode_rows_reply(&bytes), (12, vec![row]))?;
    Ok("the span asked and a page of one row decode".to_string())
}

fn why() -> Result<String, String> {
    use d::why::{decode_why, decode_why_reply, encode_why, encode_why_reply, Named, Refusal, Why, WhyAsk};
    for ask in [WhyAsk::Own(mycelium::chronometry::Place { k: 1, i: 2 }), WhyAsk::Of(stamp())] {
        same("the ask", decode_why(&encode_why(ask)), ask)?;
    }
    let named = Named { head: [1; 8], admitted: stamp(), task: Some([2; 8]), reasons: None };
    let w = Why {
        at: stamp(),
        kind: mycelium::chronometry::ActKind::Domain,
        mu: [1, 0, 2, 3],
        named: vec![named],
        unheld: 3,
        refused: vec![Refusal { kind: 1, chart: [3; 8] }],
    };
    same("the answer", decode_why_reply(&encode_why_reply(Some(&w))), Some(w))?;
    same("no act at the coordinates", decode_why_reply(&encode_why_reply(None)), None)?;
    Ok("both asks, an answer and its absence decode".to_string())
}

fn probe() -> Result<String, String> {
    let bytes = d::encode_probe(&voice(), [4u8, 5, 6], &[]).ok_or("the probe does not encode")?;
    same("the probe", d::decode_probe(&bytes), (voice(), address(&[4, 5, 6]), Vec::new()))?;
    for closure in [d::ProbeClosure::Closed, d::ProbeClosure::Nothing] {
        let r = d::ProbeReading { settled: [1, 0, 0, 2], closure, remaining: [0, 0, 3, 0] };
        same("the probe's reading", d::decode_probe_reply(&d::encode_probe_reply(&r)), r)?;
    }
    Ok("the probe and its readings decode".to_string())
}

fn peers() -> Result<String, String> {
    same("the peers request", d::decode_peers_request(&d::encode_peers_request(&voice())), voice())?;
    let endpoints = vec!["127.0.0.1:7700".to_string(), "127.0.0.1:7701".to_string()];
    let bytes = d::encode_peers_reply(&endpoints).ok_or("the peers do not encode")?;
    same("the peers", d::decode_peers_reply(&bytes), endpoints)?;
    Ok("the request and two endpoints decode".to_string())
}

fn syllabus() -> Result<String, String> {
    same("the syllabus request", d::decode_syllabus_request(&d::encode_syllabus_request(&voice())), voice())?;
    let taught = vec![d::Taught { at: stamp(), word: vec![1, 2, 3] }];
    let bytes = d::encode_syllabus_reply(&taught).ok_or("the syllabus does not encode")?;
    same("the syllabus", d::decode_syllabus_reply(&bytes), taught)?;
    Ok("the request and a taught word decode".to_string())
}

fn speech() -> Result<String, String> {
    let sentences = vec![vec![1u8, 2, 3], vec![4]];
    let bytes = d::encode_speech_standing(&voice(), [7u8, 7], 9, &sentences, &[1, 2]).ok_or("the speech does not encode")?;
    let act = d::decode_speech(&bytes).ok_or("the speech did not decode")?;
    same("the speech's voice", Some(act.voice), voice())?;
    same("the speech's task", Some(act.task), address(&[7, 7]))?;
    same("the speech's K", Some(act.k), 9)?;
    same("the speech's sentences", Some(act.sentences), sentences)?;
    same("the speech's standing", Some(act.standing), vec![1, 2])?;
    let adjudications = vec![
        d::Adjudication::Stood { receipt: CITATION, at: Some(stamp()) },
        d::Adjudication::Ratified { receipt: CITATION, at: None },
        d::Adjudication::Awaiting { form: address(&[4, 5]) },
    ];
    let reply = d::encode_speech_reply(&adjudications).ok_or("the adjudications do not encode")?;
    same("the adjudications", d::decode_speech_reply(&reply), adjudications)?;
    Ok("the speech and three adjudications decode".to_string())
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

/// Whether a MUST act's reply decoder reads `reply` as a reply: `None`
/// where the act is not a MUST act of either profile.
fn reply_reads(act: &str, reply: &[u8]) -> Option<bool> {
    use d::{endowment, universe};
    Some(match act {
        "task" => d::decode_task_envelope(reply).is_some(),
        "solve" => d::decode_solve_verdict(reply).is_some(),
        "laws register" => laws::decode_register_reply(reply).is_some(),
        "laws statement" => d::conformance::decode_statement_reply(reply).is_some(),
        "laws concordance" => laws::decode_concordance_reply(reply).is_some(),
        "laws signature" => laws::decode_signature_reply(reply).is_some(),
        "laws clock" => laws::decode_clock_reply(reply).is_some(),
        "laws verify" => laws::decode_verify_reply(reply).is_some(),
        "domain founding" => dom::decode_domain_reply(reply).is_some(),
        "domain manifest" => endowment::decode_grant_reply(reply).is_some(),
        "domain universe" => universe::decode_universe_reading(reply).is_some(),
        _ => return None,
    })
}

/// THE DEFERRAL, MUST for every profile: a deferral reads back as one with
/// its exact deficit, and no MUST act's reply decoder of a declared profile
/// reads its bytes as a reply — a decoder that does is an ambiguity, and
/// fails by the act's name.
fn deferral(profiles: &[Profile]) -> Result<String, String> {
    use crate::deferral::{encode_deferral, read_deferral, Deferral, PENDING_CODE};
    let musts: Vec<ActRequirement> = STATEMENT.union(profiles).into_iter().filter(|a| a.need == Need::Must).collect();
    for deficit in [[0; 4], [1, 0, 2, 3]] {
        let bytes = encode_deferral(PENDING_CODE, &deficit);
        same("the deferral", read_deferral(&bytes), Deferral { deficit })?;
        for a in &musts {
            match reply_reads(a.name, &bytes) {
                Some(true) => return Err(format!("the deferral's bytes read as a {} reply by {}: an ambiguity", a.name, a.reply)),
                Some(false) => {}
                None => return Err(format!("no reply decoder held for the MUST act {}", a.name)),
            }
        }
    }
    Ok(format!("the deferral reads back with its deficit; none of {} MUST reply decoders reads it as a reply", musts.len()))
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
        "laws verify" => laws_verify(),
        "laws batch" => laws_batch(),
        "domain founding" => domain_founding(),
        "domain designation" => domain_designation(),
        "domain origin" => domain_origin(),
        "domain statement" => domain_statement(),
        "domain reserve" => domain_reserve(),
        "domain origin reading" => domain_origin_reading(),
        "domain manifest" => domain_manifest(),
        "domain universe" => domain_universe(),
        "domain endowment" => domain_endowment(),
        "seat post" => seat_post(),
        "seat listing" => seat_listing(),
        "seat mail" => seat_mail(),
        "quote" => quote(),
        "receipts" => receipts(),
        "rows" => rows(),
        "why" => why(),
        "probe" => probe(),
        "peers" => peers(),
        "syllabus" => syllabus(),
        "speech" => speech(),
        "wallet" => wallet(),
        "observe query" => observe_query(),
        _ => return None,
    })
}

/// THE IN-PROCESS CONFORMANCE READ of a client declaring `profiles`,
/// modelled by the acts it speaks (`speaks` answers for each act of those
/// profiles): each act it speaks is checked through both ends of its codec;
/// each MUST act of a declared profile it does not speak fails by name. A
/// MAY act with no in-process check is unread; an act no declared profile
/// names is never asked. Every declared profile is held to recognise the
/// deferral, checked last as the finding [`super::DEFERRAL`].
pub fn check_codecs(profiles: &[Profile], speaks: impl Fn(&ActRequirement) -> bool) -> Report {
    let mut report = Report::of(profiles);
    for a in STATEMENT.union(profiles) {
        let a = &a;
        match (speaks(a), a.need) {
            (false, Need::Must) => report.fail(a.name, "a MUST act the client does not speak"),
            (false, Need::May) => {}
            (true, _) => match check(a) {
                Some(outcome) => report.record(a.name, outcome),
                None => report.unread(a.name, "no in-process check for this act"),
            },
        }
    }
    if !profiles.is_empty() {
        report.record(super::DEFERRAL, deferral(profiles));
    }
    report
}
