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
    let Some(dialogue::Verdict { voice: cvoice, addr, ruling, receipt, pi, attach, section }) =
        dialogue::decode_verdict(&frame.body)
    else {
        return;
    };
    let (a0, a1) = (addr.as_bytes()[0], addr.as_bytes()[1]);
    let refuse = |s: &mut TcpStream, why: &str| {
        eprintln!(
            "  court verdict refused {a0:02x}{a1:02x}… from {:02x}{:02x}…: {why}",
            cvoice[0], cvoice[1]
        );
        answer(s, Frame { lane: frame.lane, body: vec![0] });
    };
    // 47.2, IN FOUR STEPS, BEFORE ANY CARRIER IS TOUCHED. A verdict slashes
    // words, rebinds selections and moves this court's head, so the frame is
    // admitted first or it is nothing. A refused frame sets no court head
    // and applies no ruling.
    let origin = match door.mesh.origin.lock() {
        Ok(o) => *o,
        Err(_) => None,
    };
    let Some(origin) = origin else {
        refuse(s, "this node has adopted no origin");
        return;
    };
    // Steps 1 and 2: the origin is ours and the signature verifies over
    // 47.1's prefix under the key the frame claims.
    if let Err(why) =
        mycelium::court::admission::admit_signature(origin, &section.prefix, &cvoice, &section.sig)
    {
        refuse(s, why);
        return;
    }
    // The two bindings 47.1 fixes in the prefix itself: its payload is the
    // ruled ADDRESS and its prev hash is the receipt the ruling binds to.
    if let Err(why) =
        mycelium::court::admission::admit_ruled(&section.prefix, addr.as_bytes(), receipt)
    {
        refuse(s, why);
        return;
    }
    // Steps 3 and 4: K against this node's knowledge of the signer's head,
    // and a height that re-derives from the parents the receipt names. This
    // is the half that was missing — the frame carried no receipt, so a
    // ruling was admitted at any K and any height, and one replayed from a
    // spent position moved a court head as readily as a fresh one. A GAP is
    // not a refusal: the back-fill fetches what makes the chain close, and
    // nothing is applied until it does (47.2 step 3). An unknown signer is
    // that same gap from its genesis, which is the chapter's answer to a key
    // this mesh has not seen — never a refusal for being unknown, since
    // identities are free and room is what admits (47.3).
    let admitted = match door.kernel.lock() {
        Ok(mut k) => k.admit_receipt(origin, &section.prefix, &section.receipt),
        Err(_) => {
            refuse(s, "the court is not readable");
            return;
        }
    };
    match &admitted {
        mycelium::court::admission::Admitted::Parent(_) => {}
        mycelium::court::admission::Admitted::Gap { vk, from, to } => {
            eprintln!("  court receipt gap {:02x}{:02x}… {from}→{to} backfill", vk[0], vk[1]);
            answer(s, Frame { lane: frame.lane, body: vec![0] });
            return;
        }
        other => {
            refuse(s, other.reason().unwrap_or(mycelium::verdict::REFUSE_RECEIPT));
            return;
        }
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
