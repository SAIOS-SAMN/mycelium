// §6 across a fold, 47.3 — ACT_ROWS: the ledger rows a witness re-derives a
// fold over, a page from a cursor within the frame budget.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::network::wire::Frame;
use std::net::TcpStream;

pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(req) = dialogue::decode_rows_request(&frame.body) else { return };
    // As in `receipts`: the body's share of the frame, exact.
    let Some(budget) = mycelium::network::wire::frame_budget().checked_sub(64) else {
        return;
    };
    let at_most = budget / mycelium::economy::Entry::TAPE;
    // 78.153b — answered off the published reading, never under the kernel:
    // a witness's fetch waited on the kernel behind a busy door, was
    // deferred, and the fold went unmeasured (217–284 an hour per court,
    // epoch 147). The reading carries the journal at the head the receipts
    // were published at, so a witness's two fetches read one standing.
    let (total, rows) = door.published.take().rows_range(&req.signer, req.from, req.upto, req.cursor, at_most);
    let Some(body) = dialogue::encode_rows_reply(total, &rows, budget) else {
        eprintln!("  rows reply refused: the page's count passes its u32");
        return;
    };
    answer(stream, Frame { lane: frame.lane, body });
}
