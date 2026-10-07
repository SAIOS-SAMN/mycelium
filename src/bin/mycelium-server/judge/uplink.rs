// 78.119 — ACT_UPLINK: a kernel's settlement tier sends up its fold, and this
// court — the verifier outside the tier's closure — admits it onto its line
// (`Kernel::admit_uplink`), answered by the receipt's stamp and chain hash, or
// the refusal by name.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue::uplink::{decode_uplink, encode_uplink_reply};
use mycelium::network::wire::Frame;
use std::net::TcpStream;

pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(fold) = decode_uplink(&frame.body) else { return };
    let verdict = match door.kernel.lock() {
        Ok(mut k) => k.admit_uplink(&fold),
        Err(_) => return,
    };
    answer(stream, Frame { lane: frame.lane, body: encode_uplink_reply(&verdict) });
}
