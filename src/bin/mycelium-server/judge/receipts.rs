// 47.2 step 3, 45.8 — ACT_RECEIPTS: the place-range read a gap's back-fill
// and an era's replay fetch by.
//
//! A page of one signer's receipts, from the hot chain, `ARC1` (an era
//! replay only, 78.147), or what this court admitted of the signer, as many
//! as fit the frame budget and never past a hole — or, for a catch-up below
//! the signer's fold root, the root. A read over the kernel with no carrier
//! touched.

use crate::door::Door;
use crate::transport::answer_written;
use mycelium::anastome::dialogue;
use mycelium::network::wire::Frame;
use std::net::TcpStream;

pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(req) = dialogue::decode_receipts_request(&frame.body) else { return };
    // A budget smaller than a header serves nothing.
    let Some((at_most, budget)) = mycelium::cognition::Kernel::receipts_frame() else {
        return;
    };
    // 78.147: a catch-up below the signer's fold root is answered with the
    // root and its two signatures; only an era replay pages the archive.
    let answer = match door.kernel.lock() {
        Ok(k) => k.serve_receipts(&req.signer, req.after, req.upto, req.replay, at_most, budget),
        Err(_) => return,
    };
    let body = match answer {
        dialogue::ReceiptsAnswer::Page(receipts) => dialogue::encode_receipts_reply(&receipts, budget),
        dialogue::ReceiptsAnswer::Root(proof) => match dialogue::encode_fold_root_reply(&proof) {
            Some(body) => body,
            None => {
                eprintln!("  receipts root refused: its proof cannot be written");
                return;
            }
        },
    };
    // A page the asker no longer waits for is counted, never dropped silent.
    if !answer_written(stream, Frame { lane: frame.lane, body }) {
        mycelium::observe::note_serve(mycelium::observe::ServeAxis::Unwritten, 1);
    }
}
