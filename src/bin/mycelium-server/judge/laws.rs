// 78.62 steps 2 and 3 — ACT_LAWS: the universal law register served to any
// kernel that asks, and a transition a kernel composed verified against it.
//
//! The court does not simulate: it replays the transition against L0
//! (`physics::laws::verify`) and answers every breach with its law. Nothing is
//! admitted, no receipt is written and no book moves — the verdict is a
//! reading the kernel acts on. The verifier runs under a stroke of its own,
//! as the observer's census does: the court's metabolism is the acts it
//! admits, and this admits none.

use crate::door::Door;
use crate::transport::answer;
use mycelium::network::wire::Frame;
use mycelium::physics::laws::codec::{decode_laws_request, encode_register_reply, put_verdict, LawsRequest};
use std::net::TcpStream;

pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, _door: &Door) {
    let Some(request) = decode_laws_request(&frame.body) else { return };
    let body = match request {
        LawsRequest::Register => encode_register_reply(),
        LawsRequest::Verify(t) => {
            let mut budget = mycelium::physics::Physics::default().stroke();
            let verdict = mycelium::physics::laws::verify(&t, &mut budget);
            let mut out = Vec::new();
            put_verdict(&mut out, &verdict);
            out
        }
    };
    answer(stream, Frame { lane: frame.lane, body });
}
