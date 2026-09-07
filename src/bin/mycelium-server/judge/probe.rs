// ACT_PROBE — a tentative sentence against an open listing.
//
//! Replayed and settled on graded descent or on closure, never germinated:
//! a probe is a claim about a listing that already exists, so it either
//! descends against it or it says nothing.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::network::wire::Frame;
use std::net::TcpStream;


/// One probe against one open listing (23.8, 26).
pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let kernel = &door.kernel;

    let Some((pvoice, target, cites)) = dialogue::decode_probe(&frame.body) else { return };
    let (code, paid, remaining) = mycelium::synthesis::probe(kernel, pvoice, &target, &cites);
    if matches!(code, dialogue::PROBE_CLOSED | dialogue::PROBE_DESCENT | dialogue::PROBE_SETTLED) {
        if let Ok(mut k) = kernel.lock() {
            k.release_t_lease(pvoice);
        }
    }
    if code != dialogue::PROBE_NOTHING {
        eprintln!(
            "  probe voice {:02x} on {:02x}{:02x}… code {code} paid {paid:?} remaining {remaining:?}",
            pvoice[0], target[0], target[1]
        );
    }
    let reply = Frame { lane: frame.lane, body: dialogue::encode_probe_reply(code, &paid, &remaining) };
    answer(stream, reply);
}
