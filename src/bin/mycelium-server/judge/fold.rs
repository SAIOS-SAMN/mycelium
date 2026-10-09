// 47.3, 45.8 — ACT_FOLD: the second organ of a fold fetches the home's
// prefix, re-derives it, co-signs the root, and holds the era.
//
//! 47.2 steps 1 and 2 on the proposal's section before anything is
//! fetched; then the receipts by `ACT_RECEIPTS` and the rows by `ACT_ROWS`
//! from the home, paged by the frame budget; `Kernel::verify_fold` on both;
//! the witness's own signed position over the root hash in reply, and the
//! era filed under the home's key.

use crate::door::Door;
use crate::transport::{answer, tell_peer_measured};
use mycelium::anastome::dialogue;
use mycelium::chronometry::Place;
use mycelium::network::wire::Frame;
use std::net::TcpStream;

pub(crate) fn judge(s: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(p) = dialogue::decode_fold(&frame.body) else { return };
    let me = door.mesh.vk;
    let home = p.section.receipt.stamp.vk;
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
    // The prefix, fetched from the home by place range and by rows cursor;
    // a home that cannot be reached leaves the fold unmeasured.
    let Some(dest) = door.mesh.addrs.lock().ok().and_then(|a| a.get(&home).cloned()) else {
        eprintln!("  court fold unmeasured {:02x}{:02x}…: home address unknown", home[0], home[1]);
        say(s, dialogue::FoldWitness::Unmeasured { vk: me });
        return;
    };
    let Some(receipts) = fetch_receipts(&door.mesh, &dest, &me, &home, p.after, p.root.at) else {
        eprintln!("  court fold unmeasured {:02x}{:02x}…: fetch receipts failed", home[0], home[1]);
        say(s, dialogue::FoldWitness::Unmeasured { vk: me });
        return;
    };
    let Some(rows) = fetch_rows(&door.mesh, &dest, &me, &home, p.after, p.root.at) else {
        eprintln!("  court fold unmeasured {:02x}{:02x}…: fetch rows failed", home[0], home[1]);
        say(s, dialogue::FoldWitness::Unmeasured { vk: me });
        return;
    };
    let proposal = mycelium::cognition::Proposal { after: p.after, root: p.root, epochs: p.epochs.clone() };
    // 78.153b — re-derived off the published reading: the witness's bond is
    // the shelf's, the rest is what the home sent; the hash of every fetched
    // receipt and the nets of every row no longer run under the kernel (the
    // lock audit, row 11).
    let verdict = door.published.take().verify_fold(home, &proposal, &receipts, &rows);
    if let Err(why) = verdict {
        // PHASE 1 DIAGNOSTIC — the witness's side of a fold that does not
        // conserve: what it fetched against what the home proposed.
        if why == mycelium::verdict::REFUSE_FOLD_INEXACT {
            static SEEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
            let n = SEEN.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
            if n.is_power_of_two() {
                let ours = mycelium::economy::nets_of(&rows).map(|n| n.rows);
                let differ = match &ours {
                    Ok(o) => o
                        .iter()
                        .zip(p.epochs.iter())
                        .find(|(a, b)| a != b)
                        .map(|(a, b)| format!("{:02x}{:02x}… {:?} vs proposed {:02x}{:02x}… {:?}", a.0[0], a.0[1], a.1, b.0[0], b.0[1], b.1)),
                    Err(e) => Some(format!("nets refused: {e}")),
                };
                eprintln!(
                    "  fold inexact ({n}) of {:02x}{:02x}…: rows fetched {} after {:?} root {:?}; parties ours {:?} proposed {}; first difference {differ:?}",
                    home[0],
                    home[1],
                    rows.len(),
                    p.after,
                    p.root.at,
                    ours.as_ref().map(Vec::len),
                    p.epochs.len()
                );
            }
        }
        refuse(s, why);
        return;
    }
    // The witness's own signed position over the root hash (47.1).
    let Some(section) = crate::admission::sign_position(&door.kernel, &door.mesh, &mycelium::transducer::stalk::Invariant::default(), &p.root.hash) else {
        say(s, dialogue::FoldWitness::Unmeasured { vk: me });
        return;
    };
    // The era's first place is the first receipt the witness fetched and
    // verified: `verify_fold` refused an empty prefix.
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 15 — FIFO drain head: the prefix is fetched in line order, so its head is the era's first place"
    )]
    let Some(first) = receipts.first().map(|r| r.stamp.place()) else {
        say(s, dialogue::FoldWitness::Unmeasured { vk: me });
        return;
    };
    // 78.147: the era is held with its root's proof — the home's signed
    // position verified above and this witness's own — so this court, the
    // root's second holder, answers a catch-up below it with it.
    // OWED (78.153b): the era frame and its disk append stay under the kernel.
    if let Ok(mut h) = crate::held::held!(door.kernel) {
        h.run(|k, _| k.hold_era_proved(mycelium::chronometry::Era {
            root: p.root,
            signer: home,
            witness: me,
            sig_home: p.section.sig,
            sig_witness: section.sig,
            identities: p.epochs.len() as u64,
            // The place range the witness fetched, named on the descriptor
            // so it stands without the payload it is about to shed (45.8).
            first,
            count: receipts.len() as u64,
            at: None,
            receipts,
        }, &p.section, &section));
    }
    eprintln!(
        "  court fold witnessed {:02x}{:02x}… co-signed {:02x}{:02x}…",
        home[0], home[1], p.root.hash[0], p.root.hash[1]
    );
    say(s, dialogue::FoldWitness::CoSigned { section: Box::new(section) });
}

/// The home's receipts strictly after `after` up to `upto`, page by page;
/// `None` when a page comes back empty or malformed before the span is
/// whole.
fn fetch_receipts(
    mesh: &crate::exchange::Mesh,
    dest: &str,
    me: &[u8; 32],
    home: &[u8; 32],
    after: Option<Place>,
    upto: Place,
) -> Option<Vec<mycelium::chronometry::KStepReceipt>> {
    let mut out = Vec::new();
    let mut next = after;
    while next.is_none_or(|n| n < upto) {
        let reply = tell_peer_measured(mesh, dest, dialogue::encode_receipts_request(me, home, next, upto)).map(|(r, _)| r);
        let page = crate::backfill::page_of(reply, mycelium::observe::CatchupPath::FoldWitness)?;
        // The page arrives in line order, so its tail names the place the
        // next page opens after — the chain's order, not the order of
        // arrival.
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — FIFO drain head: a receipts page is served in line order, so its tail is the place already served"
        )]
        let last = page.last()?.stamp.place();
        out.extend(page);
        next = Some(last);
    }
    Some(out)
}

/// The home's rows standing at `from` or later and written before the act
/// at `upto`, page by page from the cursor.
fn fetch_rows(
    mesh: &crate::exchange::Mesh,
    dest: &str,
    me: &[u8; 32],
    home: &[u8; 32],
    from: Option<Place>,
    upto: Place,
) -> Option<Vec<mycelium::economy::Entry>> {
    let mut out = Vec::new();
    let mut cursor = 0u64;
    loop {
        let (reply, _) = tell_peer_measured(mesh, dest, dialogue::encode_rows_request(me, home, from, upto, cursor))?;
        if crate::transport::is_deferred(&reply) {
            return None;
        }
        let (total, page) = dialogue::decode_rows_reply(&reply)?;
        if page.is_empty() {
            return (cursor >= total).then_some(out);
        }
        cursor = cursor.checked_add(page.len() as u64)?;
        out.extend(page);
        if cursor >= total {
            return Some(out);
        }
    }
}
