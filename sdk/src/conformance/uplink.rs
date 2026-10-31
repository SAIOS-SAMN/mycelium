// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// THE KERNEL PROFILE ON THE NETWORK: the acts of a kernel's uplink (its
// house) spoken to a running hypha — the clock read, transitions verified
// against the court's own verdict, a domain founded on the voice's key, its
// reserve, origin and manifest, its universe and endowment read, and the
// house's seats posted and read back — each reply decoded and checked by
// exact values, each failure named.

use super::speaker::Speaker;
use super::Report;
use crate::voice::SigningKey;
use mycelium::anastome::dialogue::domain as dom;
use mycelium::anastome::dialogue::endowment::{self, GrantRefusal, Granted};
use mycelium::anastome::dialogue::{seat, universe};
use mycelium::physics::laws::codec as laws;

fn spoken(session: &Speaker, act: Vec<u8>) -> Result<Vec<u8>, String> {
    session.speak(act)
}

fn undecoded(reply: &[u8], what: &str) -> String {
    format!("{} bytes that do not decode as {what}", reply.len())
}

fn laws_clock(session: &Speaker) -> Result<String, String> {
    let reply = spoken(session, laws::encode_clock_request())?;
    match laws::decode_clock_reply(&reply) {
        Some(None) => Ok("no head yet".to_string()),
        Some(Some(stamp)) => Ok(format!("the head at {stamp:?}")),
        None => Err(undecoded(&reply, "the core clock")),
    }
}

/// Each transition's served verdict against the verdict this build rules
/// (`verify_sized`): the court and the kernel agree exactly, or the
/// difference is named.
fn laws_verify(session: &Speaker) -> Result<String, String> {
    let ts = super::probes::transitions().ok_or("the turn carries no body")?;
    let mut read = Vec::new();
    for t in &ts {
        let reply = spoken(session, laws::encode_verify_request(t))?;
        let served = laws::decode_verify_reply(&reply).ok_or_else(|| undecoded(&reply, "a verdict"))?;
        let held = mycelium::physics::laws::verify_sized(t);
        if served != held {
            return Err(format!("the served verdict {served:?} is not the verdict ruled here, {held:?}"));
        }
        read.push(match served.admitted() {
            true => "admitted".to_string(),
            false => format!("refused on {:?}", served.breaches.iter().map(|(l, _)| l.name()).collect::<Vec<_>>()),
        });
    }
    Ok(format!("each verdict equals the verdict ruled here: {}", read.join("; ")))
}

fn laws_batch(session: &Speaker) -> Result<String, String> {
    let ts = super::probes::transitions().ok_or("the turn carries no body")?.to_vec();
    let reply = spoken(session, laws::encode_batch_request(&ts))?;
    let served = laws::decode_batch_reply(&reply).ok_or_else(|| undecoded(&reply, "verdicts"))?;
    let held: Vec<_> = ts.iter().map(mycelium::physics::laws::verify_sized).collect();
    match served == held {
        true => Ok(format!("{} verdicts in order, each the verdict ruled here", served.len())),
        false => Err("the served verdicts are not the verdicts ruled here".to_string()),
    }
}

fn founded(reply: &[u8]) -> Result<dom::Founded, String> {
    dom::decode_domain_reply(reply).ok_or_else(|| undecoded(reply, "the court's answer"))
}

/// The voice founds a domain of its own key, its one tier the same key.
fn domain_founding(session: &Speaker, key: &SigningKey) -> Result<String, String> {
    let domain = key.verifying_key().to_bytes();
    let signed = dom::DomainGenesis { domain, tiers: vec![domain] }.sign(key);
    match founded(&spoken(session, dom::encode_domain(&signed))?)? {
        dom::Founded::Sealed { at, .. } => Ok(format!("sealed at height {}", at.h)),
        dom::Founded::AlreadyFounded => Ok("founded already, by this voice".to_string()),
        other => Err(format!("a signed, ordered genesis refused: {other:?}")),
    }
}

/// The domain's reserve read; the domain's one tier designated where it is
/// not the reserve yet.
fn domain_reserve(session: &Speaker, key: &SigningKey, report: &mut Report) {
    let domain = key.verifying_key().to_bytes();
    let reply = match spoken(session, dom::encode_reserve_read(domain)) {
        Ok(r) => r,
        Err(e) => return session.fail(report, "domain reserve", e),
    };
    let Some(reading) = dom::decode_reserve_reading(&reply) else { return report.fail("domain reserve", undecoded(&reply, "a reserve")) };
    if reading.current == Some(domain) {
        return report.pass("domain reserve", "the domain's tier is its reserve");
    }
    report.pass("domain reserve", "no reserve yet");
    let signed = dom::Reservation { domain, tier: domain, nonce: mycelium::chronometry::payload_hash(&domain) }.sign(key);
    let outcome = spoken(session, dom::encode_reservation(&signed)).and_then(|r| founded(&r)).and_then(|f| match f {
        dom::Founded::Reserved { now, .. } => Ok(format!("designated, holding {}", if now { "now" } else { "from the next fold" })),
        other => Err(format!("the founder's designation refused: {other:?}")),
    });
    session.record(report, "domain designation", outcome);
}

fn domain_origin(session: &Speaker, key: &SigningKey) -> Result<String, String> {
    let signed = super::probes::origin(key.verifying_key().to_bytes()).sign(key);
    match founded(&spoken(session, dom::encode_origin(&signed))?)? {
        dom::Founded::Originated { at, .. } => Ok(format!("originated at height {}", at.h)),
        dom::Founded::AlreadyOriginated => Ok("originated already".to_string()),
        other => Err(format!("a signed origin of a founded domain refused: {other:?}")),
    }
}

/// The manifest signed by the sitting reserve: granted, or refused by a name
/// that is the court's standing (what it has earned, a grant made or in
/// flight), never the house's act.
fn domain_manifest(session: &Speaker, key: &SigningKey) -> Result<String, String> {
    let signed = super::probes::manifest(key.verifying_key().to_bytes()).sign(key).ok_or("the manifest does not sign")?;
    let reply = spoken(session, endowment::encode_manifest(&signed).ok_or("the manifest does not encode")?)?;
    match endowment::decode_grant_reply(&reply).ok_or_else(|| undecoded(&reply, "a grant"))? {
        Granted::Sealed { mu, founding, .. } => Ok(format!("granted μ {mu:?}, founding {founding}")),
        Granted::Refused(r @ (GrantRefusal::Short | GrantRefusal::Granted | GrantRefusal::InFlight | GrantRefusal::NoNewClosures)) => {
            Ok(format!("refused by name, nothing moved: {r:?}"))
        }
        Granted::Refused(r) => Err(format!("a manifest of the sitting reserve refused: {r:?}")),
    }
}

/// The universe read back: originated, in the geometry and parent face its
/// origin stated.
fn domain_universe(session: &Speaker, domain: [u8; 32]) -> Result<String, String> {
    let reply = spoken(session, universe::encode_universe_read(domain))?;
    let read = universe::decode_universe_reading(&reply).ok_or_else(|| undecoded(&reply, "a universe"))?;
    let g = super::probes::genesis();
    match (read.originated, &read.geometry, read.parent) {
        (true, Some(geometry), Some(parent)) if *geometry == g.geometry && parent == g.parent => {
            Ok(format!("originated, {geometry:?}, parent {parent:?}, basket of {}", read.basket.len()))
        }
        _ => Err(format!("the universe read is not the universe originated: {read:?}")),
    }
}

fn domain_endowment(session: &Speaker, domain: [u8; 32]) -> Result<String, String> {
    let reply = spoken(session, endowment::encode_endowment_read(domain))?;
    let read = endowment::decode_endowment_reading(&reply).ok_or_else(|| undecoded(&reply, "an endowment"))?;
    Ok(format!("earned μ {:?}, {} grants to this universe", read.earned, read.grants.len()))
}

/// The house's seats posted as its sitting reserve, the listing read back
/// equal to the post, and the house's mail read at the relay's standing.
fn seats(session: &Speaker, key: &SigningKey, report: &mut Report) {
    let domain = key.verifying_key().to_bytes();
    let Some(signed) = super::probes::seats(domain).sign(key) else { return report.fail("seat post", "the listing does not sign") };
    let posted = seat::encode_post(&signed).ok_or("the listing does not encode".to_string()).and_then(|b| spoken(session, b)).and_then(|r| {
        match seat::decode_post_reply(&r) {
            Some(Ok(seq)) => Ok(format!("listed, sequence {seq}")),
            Some(Err(why)) => Err(format!("the house's listing refused: {why:?}")),
            None => Err(undecoded(&r, "the relay's answer")),
        }
    });
    session.record(report, "seat post", posted);
    let listed = spoken(session, seat::encode_read_listing(&domain, None)).and_then(|r| match seat::decode_listing_reply(&r, None) {
        Some(Some(s)) if s == signed => Ok("the listing served is the listing posted".to_string()),
        Some(Some(_)) => Err("the listing served is another listing".to_string()),
        Some(None) => Err("no listing served".to_string()),
        None => Err(undecoded(&r, "a listing")),
    });
    session.record(report, "seat listing", listed);
    let read = seat::ReadMail::sign_standing(key, 0, 0, mycelium::chronometry::payload_hash(&domain));
    let mail = spoken(session, seat::encode_read_mail(&read)).and_then(|r| match seat::decode_mail_reply_standing(&r) {
        Some(Ok(m)) => Ok(format!("relay epoch {}, sequence {}, {} pieces", m.epoch, m.seq, m.mail.len())),
        Some(Err(why)) => Err(format!("the house's mail refused: {why:?}")),
        None => Err(undecoded(&r, "mail")),
    });
    session.record(report, "seat mail", mail);
}

/// THE KERNEL PROFILE'S ACTS against the hypha `session` speaks to, as the
/// house whose domain is the voice `key`'s own key.
pub(super) fn run(session: &Speaker, key: &SigningKey, report: &mut Report) {
    let domain = key.verifying_key().to_bytes();
    session.record(report, "laws clock", laws_clock(session));
    session.record(report, "laws verify", laws_verify(session));
    session.record(report, "laws batch", laws_batch(session));
    session.record(report, "domain founding", domain_founding(session, key));
    domain_reserve(session, key, report);
    session.record(report, "domain origin", domain_origin(session, key));
    session.record(report, "domain manifest", domain_manifest(session, key));
    session.record(report, "domain universe", domain_universe(session, domain));
    session.record(report, "domain endowment", domain_endowment(session, domain));
    seats(session, key, report);
}
