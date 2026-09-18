// 47.3, 45.8 — ACT_FOLD: the second organ of a fold fetches the home's
// prefix, re-derives it, co-signs the root, and holds the era.
//
//! 47.2 steps 1 and 2 on the proposal's section before anything is
//! fetched; then the receipts by `ACT_RECEIPTS` and the rows by `ACT_ROWS`
//! from the home, paged by the frame budget; `Kernel::verify_fold` on both;
//! the witness's own signed position over the root hash in reply, and the
//! era filed under the home's key.

use crate::door::Door;
use crate::transport::{answer, tell_peer_within};
use mycelium::anastome::dialogue;
use mycelium::network::wire::Frame;
use std::net::TcpStream;

pub(crate) fn judge(s: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(p) = dialogue::decode_fold(&frame.body) else { return };
    let me = door.mesh.vk;
    let home = p.section.receipt.vk;
    let say = |s: &mut TcpStream, w: dialogue::FoldWitness| {
        if let Some(body) = dialogue::encode_fold_reply(&w) {
            answer(s, Frame { lane: frame.lane, body });
        }
    };
    let refuse = |s: &mut TcpStream, why: &str| {
        eprintln!("  court fold refused {:02x}{:02x}…: {why}", home[0], home[1]);
        say(s, dialogue::FoldWitness::Refused { vk: me, why: why.as_bytes().to_vec() });
    };
    let origin = door.mesh.origin.lock().ok().and_then(|o| *o);
    // R5 (34.2): a witness with no origin says so by name, and an origin
    // refusal names the two origins it compared, so a fold refused after
    // a co-sign can be read for which side moved.
    let Some(origin) = origin else {
        refuse(s, "this node has adopted no origin");
        return;
    };
    if let Err(why) =
        mycelium::court::admission::admit_signature(origin, &p.section.prefix, &home, &p.section.sig)
    {
        if why == mycelium::verdict::REFUSE_ORIGIN {
            eprintln!(
                "  court fold refused {:02x}{:02x}…: {why} [origin {:02x}{:02x}… e{} carried {:02x}{:02x}… e{}]",
                home[0],
                home[1],
                origin.0[0],
                origin.0[1],
                origin.1,
                p.section.prefix.genesis[0],
                p.section.prefix.genesis[1],
                p.section.prefix.epoch
            );
            say(s, dialogue::FoldWitness::Refused { vk: me, why: why.as_bytes().to_vec() });
            return;
        }
        refuse(s, why);
        return;
    }
    if p.section.prefix.payload != p.root.hash {
        refuse(s, mycelium::verdict::REFUSE_RECEIPT);
        return;
    }
    // The prefix, fetched from the home by K range and by rows cursor;
    // a home that cannot be reached leaves the fold unmeasured.
    let Some(dest) = door.mesh.addrs.lock().ok().and_then(|a| a.get(&home).cloned()) else {
        eprintln!("  court fold unmeasured {:02x}{:02x}…: home address unknown", home[0], home[1]);
        say(s, dialogue::FoldWitness::Unmeasured { vk: me });
        return;
    };
    let Some(receipts) = fetch_receipts(&dest, &me, &home, p.k0, p.root.k) else {
        eprintln!("  court fold unmeasured {:02x}{:02x}…: fetch receipts failed", home[0], home[1]);
        say(s, dialogue::FoldWitness::Unmeasured { vk: me });
        return;
    };
    let Some(rows) = fetch_rows(&dest, &me, &home, p.k0, p.root.k) else {
        eprintln!("  court fold unmeasured {:02x}{:02x}…: fetch rows failed", home[0], home[1]);
        say(s, dialogue::FoldWitness::Unmeasured { vk: me });
        return;
    };
    let proposal = mycelium::cognition::Proposal { k0: p.k0, root: p.root, epochs: p.epochs.clone() };
    let verdict = match door.kernel.lock() {
        Ok(k) => k.verify_fold(home, &proposal, &receipts, &rows),
        Err(_) => {
            say(s, dialogue::FoldWitness::Unmeasured { vk: me });
            return;
        }
    };
    if let Err(why) = verdict {
        refuse(s, why);
        return;
    }
    // The witness's own signed position over the root hash (47.1).
    let Some(section) = crate::admission::sign_position(&door.kernel, &door.mesh, &mycelium::transducer::stalk::Invariant::default(), &p.root.hash) else {
        say(s, dialogue::FoldWitness::Unmeasured { vk: me });
        return;
    };
    if let Ok(mut k) = door.kernel.lock() {
        k.hold_era(mycelium::chronometry::Era {
            root: p.root,
            signer: home,
            witness: me,
            sig_home: p.section.sig,
            sig_witness: section.sig,
            identities: p.epochs.len() as u32,
            // The K range the witness fetched, named on the descriptor so
            // it stands without the payload it is about to shed (45.8).
            first: p.k0,
            count: receipts.len() as u32,
            at: None,
            receipts,
        });
    }
    eprintln!(
        "  court fold witnessed {:02x}{:02x}… co-signed {:02x}{:02x}…",
        home[0], home[1], p.root.hash[0], p.root.hash[1]
    );
    say(s, dialogue::FoldWitness::CoSigned { section: Box::new(section) });
}

/// The home's receipts `from..=to`, page by page; `None` when a page
/// comes back empty or malformed before the span is whole.
fn fetch_receipts(dest: &str, me: &[u8; 32], home: &[u8; 32], from: u64, to: u64) -> Option<Vec<mycelium::chronometry::KStepReceipt>> {
    let mut out = Vec::new();
    let mut next = from;
    while next <= to {
        let (reply, _) = tell_peer_within(dest, dialogue::encode_receipts_request(me, home, next, to), 16)?;
        if crate::transport::is_deferred(&reply) {
            return None;
        }
        let page = dialogue::decode_receipts_reply(&reply)?;
        // The page arrives in K order, so its tail names the K the next
        // page opens after — the chain's order, not the order of arrival.
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — FIFO drain head: a receipts page is served in K order, so its tail is the K already served"
        )]
        let last = page.last()?.step_index;
        out.extend(page);
        next = last.checked_add(1)?;
    }
    Some(out)
}

/// The home's rows anchored in `from..=to`, page by page from the cursor.
fn fetch_rows(dest: &str, me: &[u8; 32], home: &[u8; 32], from: u64, to: u64) -> Option<Vec<mycelium::economy::Entry>> {
    let mut out = Vec::new();
    let mut cursor = 0u32;
    loop {
        let (reply, _) = tell_peer_within(dest, dialogue::encode_rows_request(me, home, from, to, cursor), 16)?;
        if crate::transport::is_deferred(&reply) {
            return None;
        }
        let (total, page) = dialogue::decode_rows_reply(&reply)?;
        if page.is_empty() {
            return (cursor >= total).then_some(out);
        }
        cursor = cursor.checked_add(page.len() as u32)?;
        out.extend(page);
        if cursor >= total {
            return Some(out);
        }
    }
}
