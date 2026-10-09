// 78.92 (2) — ACT_WHY: why one act on this court's ledger stands, answered
// from the kernel's own ledger and the words it holds (`Kernel::why`).
//
//! A reading: nothing is admitted, no receipt is written and no book moves.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue::why::{decode_why, encode_why_reply};
use mycelium::network::wire::Frame;
use std::net::TcpStream;

pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(ask) = decode_why(&frame.body) else { return };
    let why = match crate::held::held!(door.kernel) {
        // Stays on the kernel (78.153b): stamp→word and its Reasons are not
        // on the shelf.
        Ok(mut h) => h.run(|k, _| k.why(ask)),
        Err(_) => return,
    };
    answer(stream, Frame { lane: frame.lane, body: encode_why_reply(why.as_ref()) });
}
