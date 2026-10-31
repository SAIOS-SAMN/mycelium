// 23.8, 26 — ACT_QUOTE: the market view, what this book has open in rank
// order, for as much of it as one frame carries.
//
//! The view leaves under a brief lock; pricing and the cohesion witness
//! compute over the snapshot with no lock held. Items ride in rank order
//! until the next would not fit the frame budget.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::network::wire::Frame;
use std::net::TcpStream;


/// The market view for one voice, over the charts it named (23.8).
pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let court = &door.court;

    let Some((qvoice, charts)) = dialogue::decode_quote_request(&frame.body) else { return };
    let Some(quoted) = mycelium::synthesis::quote(court, qvoice, &charts) else {
        return;
    };
    let mycelium::synthesis::Quoted { surface, priced, k_step, cohesion, height } = quoted;
    // A listing whose record does not write is not quoted (STD1 3c).
    let items: Vec<dialogue::WorkItem> = priced
        .iter()
        .filter_map(|p| {
            Some(dialogue::WorkItem {
                addr: p.word.address(),
                mu: p.mu,
                // THE LISTING WHOLE (L8q). Served as `sub()` it would carry
                // the coboundary links alone: a sentence listing would reach
                // a voice with its demand vertices stripped and the only move
                // left would be to subtract links.
                element: p.word.to_bytes()?,
                open: p.open.clone(),
            })
        })
        .collect();
    // The frame budget bounds what one contact carries: items ride in
    // rank order until the next would not fit.
    let Some(body) = dialogue::encode_quote_reply_within(
        height,
        &surface,
        k_step,
        &items,
        mycelium::network::wire::frame_budget(),
    ) else {
        eprintln!("  quote refused: the page does not fit the lengths it writes");
        return;
    };
    // A reply too short to carry a count served nothing.
    let served = match body.get(dialogue::QUOTE_COUNT_AT) {
        Some(n) => *n,
        None => 0,
    };
    if served > 0 {
        if let Ok(mut h) = crate::held::held!(court) {
            h.run(|k, _| k.grant_t_lease(qvoice));
        }
    }
    eprintln!(
        "  court book height {height} frontier {served} for {:02x}{:02x}…",
        qvoice[0], qvoice[1]
    );
    eprintln!(
        "  quote voice {:02x} → surface {surface:?}; {served}/{} clusters ({} bytes); K={k_step}; cohesion {cohesion:?}",
        qvoice[0],
        items.len(),
        body.len()
    );
    let reply = Frame { lane: frame.lane, body };
    answer(stream, reply);
}
