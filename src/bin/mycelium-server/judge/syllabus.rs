// A19, 47.1 — ACT_DIGESTS and ACT_FETCH: the sheaf sync, section digests
// over the atlas and the stratum fetch that follows a discordance.
//
//! Both are reads over the language snapshot with no lock held. The digests
//! reply carries this court's signed cover digest behind it, so one round
//! trip tells a peer what sections differ and where this court stands.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::network::wire::Frame;
use std::net::TcpStream;
use crate::admission::sign_cover_digest;


/// The section digests over this court's atlas, with the signed cover
/// digest behind them (A19, 47.1).
pub(crate) fn digests(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let kernel = &door.kernel;
    let mesh = &door.mesh;

    let Some(dvoice) = dialogue::decode_digests_request(&frame.body) else { return };
    let (ruled, language) = match kernel.lock() {
        Ok(mut k) => (k.ruled_digests_for(&dvoice), k.language()),
        Err(_) => return,
    };
    // The voice's own ruled digests where this court holds them; the
    // language's atlas where it does not. Both are measured on a cover.
    let digests = match ruled {
        Some(d) => d,
        None => mycelium::transducer::sheaf::digests(&language.stalks),
    };
    let mut body = dialogue::encode_digests_reply(&digests);
    body.extend(sign_cover_digest(kernel, mesh));
    let reply = Frame { lane: frame.lane, body };
    answer(stream, reply);
}

/// The stratum fetch: the ruled words of the charts a peer named, up to
/// the frame budget, with the class records behind them (A19).
#[expect(
    clippy::manual_unwrap_or_default,
    reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; a page wider than the budget leaves none of it"
)]
pub(crate) fn fetch(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let kernel = &door.kernel;
    let Some((fvoice, charts)) = dialogue::decode_fetch(&frame.body) else { return };
    // The body's share of the frame, exact.
    let Some(mut budget) = mycelium::network::wire::frame_budget().checked_sub(64) else {
        return;
    };
    let mut words: Vec<Vec<u8>> = Vec::new();
    let mut classes: Vec<dialogue::ClassPage> = Vec::new();
    if let Ok(mut k) = kernel.lock() {
        let mut have_all: Vec<mycelium::transducer::address::Address> = Vec::new();
        for dialogue::ChartOf { inv, have } in &charts {
            have_all.extend(have.iter().cloned());
            let page = k.serve_ruled(inv, have, budget);
            // `serve_ruled` fills to the budget and no further, so what is
            // left is exact; a page wider than the budget leaves none.
            budget = match budget.checked_sub(page.iter().map(|w| w.len() + 4).sum::<usize>()) {
                Some(left) => left,
                None => 0,
            };
            words.extend(page);
            classes.extend(k.class_records(inv));
            if budget == 0 {
                break;
            }
        }
        k.confirm_peer_holds_addrs(&have_all, fvoice);
    }
    if !words.is_empty() {
        eprintln!(
            "  fetch voice {:02x} ← {} words across {} charts",
            fvoice[0],
            words.len(),
            charts.len()
        );
    }
    let mut body = dialogue::encode_syllabus_reply(&words);
    dialogue::append_class_records(&mut body, &classes);
    let reply = Frame { lane: frame.lane, body };
    answer(stream, reply);
}
