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
    let Some(mut body) = dialogue::encode_digests_reply(&digests) else {
        eprintln!("  digests reply refused: a chart or a root passes the field that carries it");
        return;
    };
    body.extend(sign_cover_digest(kernel, mesh));
    let reply = Frame { lane: frame.lane, body };
    answer(stream, reply);
}

/// The stratum fetch: the ruled words of the charts a peer named, up to
/// the frame budget, with the class records behind them (A19).
pub(crate) fn fetch(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let kernel = &door.kernel;
    let Some((fvoice, charts)) = dialogue::decode_fetch(&frame.body) else { return };
    // The body's share of the frame, exact.
    let Some(mut budget) = mycelium::network::wire::frame_budget().checked_sub(64) else {
        return;
    };
    let mut words: Vec<dialogue::Taught> = Vec::new();
    let mut classes: Vec<dialogue::ClassPage> = Vec::new();
    let mut overran = false;
    if let Ok(mut k) = kernel.lock() {
        // A VOICE'S HAVE-LIST IS NEVER A VANTAGE (47.3, 1.7). `have` says
        // which words the voice already holds so the page carries only what
        // it lacks; it stood here ALSO as evidence for
        // `confirm_peer_holds_addrs(&have_all, fvoice)`, which recorded the
        // voice as an organ that re-derived every provisional it named and
        // installed on it. A voice holds the sentences it spoke, so the
        // speaker witnessed its own speech — "free identities witnessing
        // themselves" — and 8 of 8 installs on a5da8d5 came through this
        // door. A second vantage is a hypha's replay (`judge::offer`) or a
        // hypha's served page (`exchange`); a mouth is not one.
        for dialogue::ChartOf { inv, have } in &charts {
            let page = k.serve_ruled(inv, have, budget);
            // `serve_ruled` fills to the budget and no further, so what is
            // left is exact. A page WIDER than the budget is a broken page,
            // and the frame it would go in cannot carry it: the fetch is
            // answered with nothing rather than a reply the wire truncates.
            let width = |t: &dialogue::Taught| mycelium::chronometry::Stamp::LEN + 4 + t.word.len();
            let Some(left) = budget.checked_sub(page.iter().map(width).sum::<usize>())
            else {
                overran = true;
                break;
            };
            budget = left;
            words.extend(page);
            classes.extend(k.class_records(inv));
            if budget == 0 {
                break;
            }
        }
    }
    if overran {
        eprintln!("  fetch refused: a served page ran past the frame budget");
        words.clear();
        classes.clear();
    }
    if !words.is_empty() {
        eprintln!(
            "  fetch voice {:02x} ← {} words across {} charts",
            fvoice[0],
            words.len(),
            charts.len()
        );
    }
    let Some(mut body) = dialogue::encode_syllabus_reply(&words) else {
        eprintln!("  fetch refused: the page does not fit its count");
        return;
    };
    if dialogue::append_class_records(&mut body, &classes).is_none() {
        eprintln!("  fetch refused: the class records cannot be framed");
        return;
    }
    let reply = Frame { lane: frame.lane, body };
    answer(stream, reply);
}
