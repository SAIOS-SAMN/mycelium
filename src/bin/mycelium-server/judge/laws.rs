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
        // 78.122 — the shared lexicon, at its version.
        LawsRequest::Lexicon => mycelium::physics::laws::codec::encode_lexicon_reply(),
        LawsRequest::Verify(t) => {
            // 78.116 — under the stroke the motion's modes need, never the
            // court's default: a verdict of the transition alone.
            let verdict = mycelium::physics::laws::verify_sized(&t);
            let mut out = Vec::new();
            put_verdict(&mut out, &verdict);
            out
        }
        // 78.118 — each transition of the batch, in the order sent.
        LawsRequest::Batch(ts) => {
            let verdicts: Vec<_> = ts.iter().map(mycelium::physics::laws::verify_sized).collect();
            mycelium::physics::laws::codec::encode_batch_reply(&verdicts)
        }
    };
    answer(stream, Frame { lane: frame.lane, body });
}
