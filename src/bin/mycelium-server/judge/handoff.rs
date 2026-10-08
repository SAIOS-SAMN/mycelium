// 78.126 — ACT_HANDOFF: matter or a word crossing domains on this court — an
// offer, its acceptance or refusal, its expiry (`Kernel::handoff`), answered
// by the receipt or the refusal by name; or, sealing nothing, a handoff's
// standing read.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue::handoff::{decode_handoff_request, encode_handoff_reply, encode_read_reply, HandoffRequest};
use mycelium::network::wire::Frame;
use std::net::TcpStream;

pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(request) = decode_handoff_request(&frame.body) else { return };
    let body = match crate::held::held!(door.kernel) {
        Ok(mut k) => match request {
            HandoffRequest::Signed(h) => encode_handoff_reply(&k.handoff(&h)),
            HandoffRequest::Read(id) => encode_read_reply(k.handoff_read(&id)),
            HandoffRequest::Loops => mycelium::anastome::dialogue::handoff::encode_loops_reply(&k.mouth_loops()),
            HandoffRequest::TubeRead(id) => mycelium::anastome::dialogue::handoff::encode_tube_reading(k.tube_of(&id).as_ref()),
        },
        Err(_) => return,
    };
    // 78.137 — acknowledged only once durable.
    if !crate::persist::durable(&door.kernel, &door.state_path) {
        return;
    }
    answer(stream, Frame { lane: frame.lane, body });
}
