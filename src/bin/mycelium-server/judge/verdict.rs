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
    let Some(verdict) = dialogue::decode_verdict(&frame.body) else {
        return;
    };
    let dialogue::Verdict { voice: cvoice, addr, ruling, receipt, section, .. } = &verdict;
    let (cvoice, ruling, receipt) = (*cvoice, *ruling, *receipt);
    let (a0, a1) = (addr.as_bytes()[0], addr.as_bytes()[1]);
    let say = |s: &mut TcpStream, r: dialogue::Ruled| {
        if let dialogue::Ruled::Refused(why) = &r {
            eprintln!(
                "  court verdict refused {a0:02x}{a1:02x}… from {:02x}{:02x}…: {}",
                cvoice[0],
                cvoice[1],
                String::from_utf8_lossy(why)
            );
        }
        match dialogue::encode_ruled(&r) {
            Some(body) => answer(s, Frame { lane: frame.lane, body }),
            None => eprintln!("  court verdict answer refused: the reason passes its u16"),
        }
    };
    let refuse = |s: &mut TcpStream, why: &str| {
        say(s, dialogue::Ruled::Refused(why.as_bytes().to_vec()))
    };
    // 47.2, IN FOUR STEPS, BEFORE ANY CARRIER IS TOUCHED. A verdict slashes
    // words, rebinds selections and moves this court's head, so the frame is
    // admitted first or it is nothing. A refused frame sets no court head
    // and applies no ruling.
    let origin = match door.mesh.origin.lock() {
        Ok(o) => *o,
        Err(_) => None,
    };
    // A node that has adopted no origin has none equal to the frame's, which
    // is 47.2 step 1 exactly: REFUSE_ORIGIN, not a sentence of my own. §2.4
    // takes its refusals from 34.1 and this one is already there.
    let Some(origin) = origin else {
        refuse(s, mycelium::verdict::REFUSE_ORIGIN);
        return;
    };
    // Steps 1 and 2: the origin is ours and the signature verifies over
    // 47.1's prefix under THE KEY THE RECEIPT NAMES. It verified under
    // `cvoice`, a separate field of the act body, while step 3 below reads
    // the head at `section.receipt.vk` -- two different keys for a chapter
    // that checks one signer, so a frame could pass step 2 on its own key
    // and write step 3's state under someone else's.
    let signer = section.receipt.vk;
    let verified = match mycelium::court::admission::admit_signature(
        origin,
        &section.prefix,
        &signer,
        &section.sig,
    ) {
        Ok(v) => v,
        Err(why) => {
            refuse(s, why);
            return;
        }
    };
    // 47.1's bindings: the prefix's payload is the ruled ADDRESS, its prev
    // hash is the head the ruling binds to, and the receipt names the key
    // this act claims as its court.
    if let Err(why) = mycelium::court::admission::admit_ruled(
        &section.prefix,
        &section.receipt,
        &cvoice,
        addr.as_bytes(),
        receipt,
    ) {
        refuse(s, why);
        return;
    }
    // THE ACT'S FIELDS AGREE WITH ITS RULING, or it is malformed (34.1
    // `REFUSE_MALFORMED`). Read once, before any carrier is touched, so the
    // match below is total over what the act carries and no arm answers
    // for a payload that is not there.
    let payload = match mycelium::court::admission::admit_payload(&verdict) {
        Ok(p) => p,
        Err(why) => {
            refuse(s, why);
            return;
        }
    };
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
    // A court that cannot be read has judged nothing. That is not a refusal
    // of the frame and there is no 34.1 constant for it: nothing is written,
    // and the caller reads silence as it already reads an unreachable peer.
    let Ok(mut k) = door.kernel.lock() else {
        eprintln!("  court verdict unread {a0:02x}{a1:02x}…: the court is not readable");
        return;
    };
    let admitted = k.admit_receipt(origin, verified, &section.receipt);
    use mycelium::court::admission::Admitted;
    match &admitted {
        Admitted::Parent(_) => {}
        // A GAP IS NOT A REFUSAL. The back-fill fetches what makes the chain
        // close, and nothing is applied until it does (47.2 step 3). The
        // reply names the span so the sender can send it.
        //
        // WHAT IS STILL OWED: step 3 also says "the frame is docketed as a
        // pending citation is, the missing receipts are fetched from the
        // signer or any peer holding them". This receiver dockets nothing
        // and fetches nothing, so the span it asks for has no local entry
        // waiting to close against it. Recorded, not papered over.
        // A CITATION THIS NODE DOES NOT HOLD is the same shape of answer:
        // the frame is well formed and its author is not at fault, and
        // what is missing is a receipt of the signer the citation names.
        // Asked for as the span it sits at, which is what the back-fill
        // chases across signers (47.2 step 3).
        Admitted::MissingParent(p) => {
            eprintln!(
                "  court receipt cites {:02x}{:02x}… K {} unheld, backfill",
                p.vk[0], p.vk[1], p.k
            );
            drop(k);
            say(s, dialogue::Ruled::Gap { from: p.k, to: p.k + 1 });
            return;
        }
        Admitted::Gap { vk, from, to } => {
            eprintln!("  court receipt gap {:02x}{:02x}… {from}→{to} backfill", vk[0], vk[1]);
            drop(k);
            say(s, dialogue::Ruled::Gap { from: *from, to: *to });
            return;
        }
        // Every remaining arm named, so no catch-all reaches for a reason
        // that is not there. `reason()` is None on Parent and Gap alone,
        // and both are above; `unwrap_or(REFUSE_RECEIPT)` invented a cause
        // for an arm it could never see (rule 11, checklist (i)).
        Admitted::Stale => {
            drop(k);
            refuse(s, mycelium::verdict::REFUSE_RECEIPT_STALE);
            return;
        }
        Admitted::Equivocation { .. } => {
            drop(k);
            refuse(s, mycelium::verdict::REFUSE_EQUIVOCATION);
            return;
        }
        Admitted::Refused(why) => {
            let why = *why;
            drop(k);
            refuse(s, why);
            return;
        }
    }
    k.set_court_head(receipt);
    k.set_court_voice(cvoice);
    // ONE ARM, ONE READING. This folded three unrelated readings into one
    // bool — `apply_rebind`'s, `apply_attach(..) > 0` (a count deciding a
    // branch, checklist (b)), and `apply_verdict`'s — and answered `false`
    // alike for "the act named RULING_REBIND and carried no selection" and
    // "the rebind did not take". The malformed act was refused above by its
    // own name; what is left is total over the payload the act carries, and
    // a ruling that found nothing to move is inert.
    use mycelium::court::admission::Payload;
    let ruled = match payload {
        Payload::Rebind(term) => match k.apply_rebind(addr, term) {
            true => dialogue::Ruled::Applied,
            false => dialogue::Ruled::Inert,
        },
        Payload::Attach(base, g) => match k.apply_attach(addr, &base, &g) {
            0 => dialogue::Ruled::Inert,
            _ => dialogue::Ruled::Applied,
        },
        Payload::Plain => match k.apply_verdict(addr, ruling) {
            true => dialogue::Ruled::Applied,
            false => dialogue::Ruled::Inert,
        },
    };
    let held = k.word_at(addr).is_some();
    drop(k);
    if held
        && matches!(
            ruling,
            dialogue::RULING_UPHELD | dialogue::RULING_REBIND | dialogue::RULING_ATTACHED
        )
    {
        if let Ok(mut sy) = door.syllabus.lock() {
            syllabus_admit(&mut sy, [addr.clone()]);
        }
    }
    say(s, ruled);
}
