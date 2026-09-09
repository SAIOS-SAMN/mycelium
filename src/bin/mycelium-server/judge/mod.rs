// 23.8, 26 — THE DISPATCH: which act a frame is, and which judge it goes to.
//
//! A demand the client spoke here is forwarded once, to the key's home on
//! the cover trie. A frame that arrived as ACT_DEMAND_ROUTED is judged and
//! never forwarded.

pub(crate) mod equivocation;
pub(crate) mod offer;
pub(crate) mod percept;
pub(crate) mod probe;
pub(crate) mod quote;
pub(crate) mod receipts;
pub(crate) mod speech;
pub(crate) mod syllabus;
pub(crate) mod verdict;

use crate::door::{serve_control, Door};
use mycelium::anastome::dialogue;
use mycelium::network::wire::Frame;
use std::net::TcpStream;

pub(crate) fn judge(stream: &mut TcpStream, frame: Frame, door: &Door) {
    let mesh = &door.mesh;
    if serve_control(stream, &frame, door) {
        return;
    }
    // A demand the client spoke here is forwarded once, to the key's home
    // on the cover trie. A frame that arrived as ACT_DEMAND_ROUTED is
    // judged and never forwarded — the loop a→b→c→a was that unwrap
    // becoming ACT_DEMAND and closest_excluding sending it on again.
    let arrived_routed = frame.body.first() == Some(&dialogue::ACT_DEMAND_ROUTED);
    let frame = match frame.body.first() {
        Some(&k) if k == dialogue::ACT_DEMAND_ROUTED => match dialogue::decode_routed_demand(&frame.body) {
            Some((origin, demand))
                if origin == mesh.vk
                    || mesh.addrs.lock().map(|a| a.contains(&origin)).unwrap_or(false) =>
            {
                Frame { lane: frame.lane, body: demand.to_vec() }
            }
            _ => return,
        },
        _ => frame,
    };
    let Some((&kind, _)) = frame.body.split_first() else { return };
    match kind {
        dialogue::ACT_SPEECH => speech::judge(stream, &frame, door),
        dialogue::ACT_QUOTE => quote::judge(stream, &frame, door),
        dialogue::ACT_DIGESTS => syllabus::digests(stream, &frame, door),
        dialogue::ACT_FETCH => syllabus::fetch(stream, &frame, door),
        dialogue::ACT_PROBE => probe::judge(stream, &frame, door),
        dialogue::ACT_EQUIVOCATION => equivocation::judge(stream, &frame, door),
        dialogue::ACT_RECEIPTS => receipts::judge(stream, &frame, door),
        _ => percept::judge(stream, &frame, door, kind, arrived_routed),
    }
}
