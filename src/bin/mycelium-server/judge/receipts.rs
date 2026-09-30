// 47.2 step 3, 45.8 — ACT_RECEIPTS: the place-range read a gap's back-fill
// and an era's replay fetch by.
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
    // The frame budget is wider than one header, so the body's share is
    // exact; a budget smaller than a header serves nothing.
    let Some(budget) = mycelium::network::wire::frame_budget().checked_sub(64) else {
        return;
    };
    let at_most = budget / mycelium::chronometry::KStepReceipt::MIN_TAPE;
    let receipts = match door.kernel.lock() {
        Ok(k) => k.receipts_page(&req.signer, req.after, req.upto, at_most, budget),
        Err(_) => return,
    };
    let body = dialogue::encode_receipts_reply(&receipts, budget);
    answer(stream, Frame { lane: frame.lane, body });
}
