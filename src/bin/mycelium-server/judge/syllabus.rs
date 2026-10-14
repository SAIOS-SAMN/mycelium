// A19, 47.1 — ACT_DIGESTS and ACT_FETCH: the sheaf sync, section digests
// over the atlas and the stratum fetch that follows a discordance.
//
//! Both are reads over the language snapshot with no lock held. The digests
//! reply carries this court's signed cover digest behind it, so one round
//! trip tells a peer what sections differ and where this court stands.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::network::wire::Frame;
use std::net::TcpStream;
use crate::admission::sign_cover_digest;


/// The section digests over this court's atlas, with the signed cover
/// digest behind them (A19, 47.1).
pub(crate) fn digests(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let kernel = &door.kernel;
    let mesh = &door.mesh;

    let Some(dvoice) = dialogue::decode_digests_request(&frame.body) else { return };
    // 78.153b — the voice's own ruled digests off the published reading, the
    // language's atlas off its snapshot taken by pointer: neither read under
    // the kernel, which `ruled_digests_for` walked whole per ask.
    let read = door.published.take();
    let digests = match read.ruled_digests_for(&dvoice) {
        Some(d) => d.to_vec(),
        None => {
            let stalks = match crate::held::held!(kernel) {
                Ok(mut h) => h.run(|k, _| std::sync::Arc::clone(&k.language().stalks)),
                Err(_) => return,
            };
            mycelium::transducer::sheaf::digests(&stalks)
        }
    };
    drop(read);
    let Some(mut body) = dialogue::encode_digests_reply(&digests) else {
        eprintln!("  digests reply refused: a chart or a root passes the field that carries it");
        return;
    };
    body.extend(sign_cover_digest(kernel, &door.published, mesh));
    let reply = Frame { lane: frame.lane, body };
    answer(stream, reply);
}

/// The stratum fetch: the ruled words of the charts a peer named, up to
/// the frame budget, with the class records behind them (A19).
pub(crate) fn fetch(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    // 78.160 P1 — a mind's residue asked of the presentation rides the
    // fetch's act under its own marker.
    if let Some((pvoice, motion)) = dialogue::decode_presentation_request(&frame.body) {
        present(stream, frame, door, pvoice, &motion);
        return;
    }
    let kernel = &door.kernel;
    let Some((fvoice, charts)) = dialogue::decode_fetch(&frame.body) else { return };
    // The body's share of the frame, exact.
    let Some(mut budget) = mycelium::network::wire::frame_budget().checked_sub(64) else {
        return;
    };
    let mut words: Vec<dialogue::Taught> = Vec::new();
    let mut classes: Vec<dialogue::ClassPage> = Vec::new();
    let mut overran = false;
    // 78.100 — WHO ASKS: a key this court's mesh holds an address for is a
    // peer court (its exchange); any other is a voice, a learner who pays.
    let asker = match door.mesh.addrs.lock().is_ok_and(|a| a.get(&fvoice).is_some()) {
        true => mycelium::cognition::Asker::Court(fvoice),
        false => mycelium::cognition::Asker::Voice(fvoice),
    };
    // 78.153b — the seed of an empty chart off the published reading.
    let read = door.published.take();
    if let Ok(mut h) = crate::held::held!(kernel) {
        h.run(|k, reads| {
        // A VOICE'S HAVE-LIST IS NEVER A VANTAGE (47.3, 1.7). `have` says
        // which words the voice already holds so the page carries only what
        // it lacks; it stood here ALSO as evidence for
        // `confirm_peer_holds_addrs(&have_all, fvoice)`, which recorded the
        // voice as an organ that re-derived every provisional it named and
        // installed on it. A voice holds the sentences it spoke, so the
        // speaker witnessed its own speech — "free identities witnessing
        // themselves" — and 8 of 8 installs on a5da8d5 came through this
        // door. A second vantage is a hypha's replay (`judge::offer`) or a
        // hypha's served page (`exchange`); a mouth is not one.
        // Every page and every class reading spends the hold's one stroke:
        // a stroke spent serves a shorter page and fewer class records, the
        // next fetch resuming the class reading where this one stopped.
        for dialogue::ChartOf { inv, have } in &charts {
            // The words the reply still carries: it counts them in one byte
            // across every stratum's page (`serve_ruled_on`).
            let Some(room) = usize::from(u8::MAX).checked_sub(words.len()).filter(|&r| r > 0) else { break };
            let page = k.serve_ruled_on(inv, have, (budget, room), asker, read.syllabus_seed(), reads);
            // `serve_ruled` fills to the budget and no further, so what is
            // left is exact. A page WIDER than the budget is a broken page,
            // and the frame it would go in cannot carry it: the fetch is
            // answered with nothing rather than a reply the wire truncates.
            let width = |t: &dialogue::Taught| mycelium::chronometry::Stamp::LEN + 4 + t.word.len();
            let Some(left) = budget.checked_sub(page.iter().map(width).sum::<usize>())
            else {
                overran = true;
                break;
            };
            budget = left;
            words.extend(page);
            classes.extend(k.class_records_on(inv, &fvoice, reads));
            if budget == 0 {
                break;
            }
        }
        });
    }
    drop(read);
    if overran {
        eprintln!("  fetch refused: a served page ran past the frame budget");
        words.clear();
        classes.clear();
    }
    if !words.is_empty() {
        eprintln!(
            "  fetch voice {:02x} ← {} words across {} charts",
            fvoice[0],
            words.len(),
            charts.len()
        );
    }
    let Some(mut body) = dialogue::encode_syllabus_reply(&words) else {
        eprintln!("  fetch refused: the page does not fit its count");
        return;
    };
    if dialogue::append_class_records(&mut body, &classes).is_none() {
        eprintln!("  fetch refused: the class records cannot be framed");
        return;
    }
    let reply = Frame { lane: frame.lane, body };
    answer(stream, reply);
    // The words a peer court's fetch carried away, under the court that asked.
    if matches!(asker, mycelium::cognition::Asker::Court(_)) {
        mycelium::observe::note_replication(Some(fvoice), mycelium::observe::rep::FETCH_SERVED, words.len() as u64);
    }
}

/// 78.160 P1 — THE PRESENTATION ASKED BY A RESIDUE: the motion deconstructed
/// over this court's presentation on the hold's stroke, and the words that
/// deconstruct it the asker lacks, within the frame's body budget.
fn present(stream: &mut TcpStream, frame: &Frame, door: &Door, voice: [u8; 32], motion: &mycelium::transducer::cga::Mv) {
    let Some(budget) = mycelium::network::wire::frame_budget().checked_sub(64) else { return };
    let Ok(mut h) = crate::held::held!(&door.kernel) else { return };
    let presented = h.run(|k, reads| k.present(voice, motion, budget, reads));
    drop(h);
    let Some(body) = dialogue::encode_presentation_reply(&presented.words, &presented.answer) else {
        eprintln!("  presentation refused: a count or an address passes the field that carries it");
        return;
    };
    answer(stream, Frame { lane: frame.lane, body });
}
