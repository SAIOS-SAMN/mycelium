// §6 across a fold, 47.3 — ACT_ENTRIES: the ledger entries a witness
// re-derives a fold over, a page from a cursor within the frame budget.

use crate::context::Context;
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::network::wire::Delimited;
use mycelium::network::channel::Channel;

pub(crate) fn judge(stream: &mut Channel, frame: &Delimited, context: &Context) {
    let Some(req) = dialogue::decode_entries_request(&frame.body) else { return };
    // The frame's whole body: `encode_entries_reply` counts its own header
    // against it, and the entries the page can hold are the body less that
    // header over an entry's fewest bytes (D4.6); the reply keeps those that
    // fit at each entry's own length, and says how many it carries.
    let budget = mycelium::network::wire::frame_budget();
    let Some(entries_room) = budget.checked_sub(dialogue::ENTRIES_REPLY_HEADER) else {
        return;
    };
    let at_most = entries_room / mycelium::economy::Entry::MIN_TAPE;
    // 78.153b — answered off the published reading, never under the court:
    // a witness's fetch waited on the court behind a busy door, was
    // deferred, and the fold went unmeasured (217–284 an hour per court,
    // epoch 147). The reading carries the journal at the head the receipts
    // were published at, so a witness's two fetches read one standing.
    let (total, entries) = context.published.take().rows_range(&req.signer, req.from, req.upto, req.cursor, at_most);
    let Some(body) = dialogue::encode_entries_reply(total, &entries, budget) else {
        eprintln!("  entries reply refused: the page's count passes its u32");
        return;
    };
    answer(stream, Delimited { lane: frame.lane, body });
}
