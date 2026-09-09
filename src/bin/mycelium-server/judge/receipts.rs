// 47.2 step 3, 45.8 — ACT_RECEIPTS: the K-range read a gap's back-fill and
// an era's replay fetch by.
//
//! A page of one signer's receipts, from the hot chain, `ARC1`, or what
//! this court admitted of the signer, as many as fit the frame budget and
//! never past a hole. A read over the kernel with no carrier touched.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::network::wire::Frame;
use std::net::TcpStream;

pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(req) = dialogue::decode_receipts_request(&frame.body) else { return };
    let budget = mycelium::network::wire::frame_budget().saturating_sub(64);
    let at_most = budget / mycelium::chronometry::KStepReceipt::MIN_TAPE;
    let receipts = match door.kernel.lock() {
        Ok(k) => k.receipts_range(&req.signer, req.from, req.to, at_most),
        Err(_) => return,
    };
    let body = dialogue::encode_receipts_reply(&receipts, budget);
    answer(stream, Frame { lane: frame.lane, body });
}
