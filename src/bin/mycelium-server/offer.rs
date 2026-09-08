// 47.3, 13.9 — THE OFFER ROUND: what this court offers at first vantage,
// to which hyphae, and what it does with the answer.

use crate::exchange::Mesh;
use crate::transport::tell_peer;
use mycelium::anastome::dialogue;
use mycelium::cognition::Kernel;
use std::sync::Mutex;

/// One cycle's offers. The receiving side of the same act is
/// `judge::offer`.
pub(crate) fn offer_round(kernel: &Mutex<Kernel>, mesh: &Mesh) {
    // OFFER what this court holds at first vantage. Without
    // this the exchange has nothing to move: a ruled root only
    // moves once a peer has glued a provisional, and a peer only
    // sees a provisional when it is offered. `moved false` on
    // every cycle was that deadlock.
    let offers = {
        let mut b = mycelium::physics::Physics::default().stroke();
        match kernel.lock() {
            Ok(k) => k.offers_pending(&mut b),
            Err(_) => Vec::new(),
        }
    };
    let mut offered_out = 0usize;
    // AN OFFER GOES TO THE HYPHAE COVERING ITS STRATUM, ONE PER
    // SPEND. It went to every destination in `dests`, with a
    // blocking `tell_peer` inside and neither a budget nor a
    // resume cursor: rule 10's refused shape written out, "for peer
    // in all_peers { exchange(peer) } on one stroke", and its
    // "write instead" column says "one peer per spend with a resume
    // cursor". Cost per cycle was offers × peers synchronous TCP
    // round trips on one thread; at a thousand peers with ten
    // offers that is ten thousand blocking connections before the
    // cycle can do anything else, and at ten thousand it never
    // completes -- the deadlock this loop was written to break,
    // reintroduced by the loop itself.
    //
    // The destination set is not "everyone" either. 42.6 bounds the
    // mesh cover by "the number of covering hyphae", and 47.3 says
    // the witness "holds ruled sections over the chart, since
    // replay needs them, so it is at nerve distance one from the
    // home". That set is `cover.fiber(lane, &inv)` on the lanes the
    // form occupies. A form whose chart no peer covers has no
    // eligible witness and waits, which is the chapter's answer and
    // not a failure to reach anyone.
    let mut offer_budget = mycelium::physics::Physics::default().stroke();
    for (oaddr, elem) in offers {
        let mut pos = 0usize;
        let Some(word) = mycelium::transducer::element::Element::from_bytes(&elem, &mut pos)
        else {
            continue;
        };
        let inv = mycelium::transducer::stalk::invariant_of(&word);
        let witnesses = match mesh.cover.lock() {
            Ok(c) => c.witnesses(&inv, &word.magnitude(), &mesh.vk),
            Err(_) => mycelium::network::nerve::Keyed::new(),
        };
        // 47.1: THE HOME SIGNS ITS OWN CAUSAL POSITION, with the
        // offered ADDRESS as the payload. The three fields this
        // replaces went out as `k: 0` and two all-zero signatures
        // at this one call site, so nothing on the mesh ever signed
        // an offer and the receiver had nothing to verify. A home
        // that has adopted no origin, or holds no receipt yet, has
        // no position to sign from and offers nothing.
        let Some(section) = crate::admission::sign_position(kernel, mesh, oaddr.as_bytes())
        else {
            continue;
        };
        for (vk, _) in witnesses.fiber(mycelium::network::nerve::ROOT) {
            if !offer_budget.spend() {
                break;
            }
            let Some(d) = mesh.addrs.lock().ok().and_then(|a| a.get(&vk).cloned()) else {
                continue;
            };
            let d = &d;
            let body = dialogue::encode_offer(&dialogue::Offer {
                addr: oaddr.clone(),
                home: mesh.vk,
                elem: elem.clone(),
                voices: vec![mesh.vk],
                deflation: word.magnitude(),
                section: section.clone(),
            });
            let Some((reply, _)) = tell_peer(d, body) else {
                continue;
            };
            offered_out += 1;
            // 47.3's SECOND VANTAGE. The peer replayed the offer
            // on its own tapes and sent back the form it reached;
            // that is a re-derivation, and the home converges on
            // it by `same_vertex` (M1), never by comparing the
            // bytes it happens to be spelled with. A reply with no
            // form carries no vantage: the witness is UNMEASURED
            // and the object keeps waiting, which is not the same
            // as a witness that disagreed.
            //
            // THE VANTAGE IS THE KEY THE REPLY NAMES. This read the
            // 32 bytes of the destination's BIND STRING as a
            // verifying key: on any deployment whose addresses are
            // not exactly 32 characters the `try_from` failed and
            // every second vantage was dropped on the floor, and
            // where it did not fail the witness was recorded under
            // an ASCII address that no bond and no room answers to.
            // Either way `converge_witness` never found an
            // independent second organ and every germ stayed
            // pending on one voice. An address is where a hypha
            // answers; a vantage is a key.
            let Some(w) = dialogue::decode_offer_reply(&reply) else {
                continue;
            };
            // THREE STATES, READ OFF THE CONTENT. A refusal names
            // the 34.1 constant the witness slashed with; a form is
            // the second vantage and goes to `converge_witness`,
            // which compares the two re-derivations by `same_vertex`
            // (M1); neither is a witness that reached nothing, held
            // unmeasured. A voice never witnesses itself (47.3).
            if !w.refused.is_empty() {
                eprintln!(
                    "  court offer refused {:02x}{:02x}… by {:02x}{:02x}…: {}",
                    oaddr[0],
                    oaddr[1],
                    w.vk[0],
                    w.vk[1],
                    String::from_utf8_lossy(&w.refused)
                );
                continue;
            }
            if w.vk == mesh.vk {
                continue;
            }
            let mut p = 0usize;
            let Some(theirs) =
                mycelium::transducer::element::Element::from_bytes(&w.reached, &mut p)
            else {
                continue;
            };
            if let Ok(mut k) = kernel.lock() {
                k.witness_rederived(&oaddr, w.vk, theirs);
            }
        }
    }
    if offered_out > 0 {
        eprintln!("  court offered {offered_out} germs to the overlap");
    }
}
