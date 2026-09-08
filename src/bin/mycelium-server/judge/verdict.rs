// 42, 47.2 — ACT_VERDICT: a court's ruling on one address arrives here and
// moves this node's court head, its selections and its syllabus.

use crate::door::Door;
use crate::syllabus::syllabus_admit;
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::network::wire::Frame;
use std::net::TcpStream;

/// One ruling from a court on one address (42).
pub(crate) fn judge(s: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(dialogue::Verdict { voice: cvoice, addr, ruling, receipt, pi, attach }) =
        dialogue::decode_verdict(&frame.body)
    else {
        return;
    };
    let applied = {
        let mut k = door.kernel.lock().unwrap();
        k.set_court_head(receipt);
        k.set_court_voice(cvoice);
        match ruling {
            dialogue::RULING_REBIND => match pi {
                Some(term) => k.apply_rebind(&addr, term),
                None => false,
            },
            dialogue::RULING_ATTACHED => match attach {
                Some((base, g)) => k.apply_attach(&addr, &base, &g) > 0,
                None => false,
            },
            _ => k.apply_verdict(&addr, ruling),
        }
    };
    let held = door.kernel.lock().map(|k| k.word_at(&addr).is_some()).unwrap_or(false);
    if held
        && matches!(
            ruling,
            dialogue::RULING_UPHELD | dialogue::RULING_REBIND | dialogue::RULING_ATTACHED
        )
    {
        if let Ok(mut sy) = door.syllabus.lock() {
            syllabus_admit(&mut sy, [addr]);
        }
    }
    let reply = Frame { lane: frame.lane, body: vec![applied as u8] };
    answer(s, reply);
}
