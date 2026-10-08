// 23.8, 26 — ACT_PROBE: a tentative sentence against an open listing.
//
//! Replayed and settled on a graded residual or on closure, never germinated:
//! a probe is a claim about a listing that already exists, so it either
//! measures a trajectory against it or it says nothing.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::network::wire::Frame;
use std::net::TcpStream;


/// One probe against one open listing (23.8, 26).
pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let kernel = &door.kernel;

    let Some((pvoice, target, cites)) = dialogue::decode_probe(&frame.body) else { return };
    let reading = mycelium::synthesis::probe(kernel, pvoice, &target, &cites);
    // A probe that moved the listing, or found it gone, releases the lease.
    if reading.moved() || reading.closure == dialogue::ProbeClosure::Gone {
        if let Ok(mut k) = crate::held::held!(kernel) {
            k.release_t_lease(pvoice);
        }
    }
    if reading.closure != dialogue::ProbeClosure::Nothing {
        eprintln!(
            "  probe voice {:02x} on {:02x}{:02x}… settled {:?} closure {:?} remaining {:?}",
            pvoice[0], target[0], target[1], reading.settled, reading.closure, reading.remaining
        );
    }
    let reply = Frame { lane: frame.lane, body: dialogue::encode_probe_reply(&reading) };
    answer(stream, reply);
}
