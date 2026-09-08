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
    let Some(dialogue::Verdict { voice: cvoice, addr, ruling, receipt, pi, attach, prefix, sig }) =
        dialogue::decode_verdict(&frame.body)
    else {
        return;
    };
    // 47.2, BEFORE ANY CARRIER IS TOUCHED. A verdict slashes words, rebinds
    // selections and moves this court's head, so the frame is admitted
    // first or it is nothing: the origin is ours, the key is one this mesh
    // admitted, and the signature verifies over 47.1's prefix. A refused
    // frame sets no court head and applies no ruling.
    let origin = match door.mesh.origin.lock() {
        Ok(o) => *o,
        Err(_) => None,
    };
    let Some(origin) = origin else {
        eprintln!("  court verdict refused: this node has adopted no origin");
        answer(s, Frame { lane: frame.lane, body: vec![0] });
        return;
    };
    let knows =
        |vk: &[u8; 32]| door.mesh.addrs.lock().map(|a| a.contains(vk)).unwrap_or(false) || *vk == door.mesh.vk;
    if let Err(why) = mycelium::court::admission::admit_verdict(
        origin,
        &prefix,
        &cvoice,
        &sig,
        addr.as_bytes(),
        receipt,
        knows,
    ) {
        eprintln!(
            "  court verdict refused {:02x}{:02x}… from {:02x}{:02x}…: {why}",
            addr.as_bytes()[0], addr.as_bytes()[1], cvoice[0], cvoice[1]
        );
        answer(s, Frame { lane: frame.lane, body: vec![0] });
        return;
    }
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
