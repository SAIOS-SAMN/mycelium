// 47.3, 13.9 — THE OFFER: what a transition that moved a provisional word
// sends to the hyphae covering its chart, and what the home does with the
// answer.

use crate::exchange::Mesh;
use crate::transport::tell_peer;
use mycelium::anastome::dialogue;
use mycelium::cognition::Kernel;
use mycelium::transducer::address::Address;
use std::sync::Mutex;

/// THE OFFERS ONE TRANSITION EMITTED, sent when it returns (47.3). A word
/// is offered by the act that admitted it to the docket, or by a transition
/// that carried it — a sentence waiting on it, a peer arriving on its chart
/// — and by nothing else: no round scans the docket for words to offer. The
/// receiving side of the same act is `judge::offer`.
///
/// The budget is this court's own stroke (rule 5), spent one witness per
/// spend. An offer goes to the hyphae covering the word's chart on the lanes
/// it occupies (42.6, 47.3's nerve distance one); a chart no peer covers has
/// no eligible witness, and the word waits for a transition that concerns it
/// — its extent, if nothing does, ends its orbit at its lapse.
pub(crate) fn offer(kernel: &Mutex<Kernel>, mesh: &Mesh, offers: Vec<(Address, Vec<u8>)>) {
    if offers.is_empty() {
        return;
    }
    let Some(mut offer_budget) = kernel.lock().ok().map(|mut k| k.stroke()) else {
        return;
    };
    let mut offered_out = 0usize;
    for (oaddr, elem) in offers {
        let mut pos = 0usize;
        let Some(word) = mycelium::transducer::element::Element::from_bytes(&elem, &mut pos) else {
            continue;
        };
        let inv = mycelium::transducer::stalk::invariant_of(&word);
        let witnesses = match mesh.cover.lock() {
            Ok(c) => c.witnesses(&inv, &word.magnitude(), &mesh.vk),
            Err(_) => mycelium::network::nerve::Keyed::new(),
        };
        // 47.1: THE HOME SIGNS ITS OWN CAUSAL POSITION, with the offered
        // ADDRESS as the payload; a home with no origin or no receipt has no
        // position to sign from and offers nothing. The prefix's chart is
        // the offered word's (47.1), never the cover.
        let Some(section) = crate::admission::sign_position(kernel, mesh, &inv, oaddr.as_bytes()) else {
            continue;
        };
        for (vk, _) in witnesses.scoped() {
            if !offer_budget.spend() {
                break;
            }
            let Some(d) = mesh.addrs.lock().ok().and_then(|a| a.get(&vk).cloned()) else {
                continue;
            };
            let Ok(witness) = <[u8; 32]>::try_from(vk.as_slice()) else {
                continue;
            };
            // A witness that answered this word has given its vantage, and
            // is not asked again (S12.4d).
            let (answered, k_now) = match kernel.lock() {
                Ok(k) => (
                    matches!(
                        k.offer_standing(oaddr.as_bytes(), &witness),
                        Some(mycelium::cognition::OfferStanding::Answered)
                    ),
                    k.k_step(),
                ),
                Err(_) => continue,
            };
            if answered {
                continue;
            }
            // An offer whose section cannot be framed is not spoken.
            let Some(body) = dialogue::encode_offer(&dialogue::Offer {
                addr: oaddr.clone(),
                home: mesh.vk,
                elem: elem.clone(),
                voices: vec![mesh.vk],
                deflation: word.magnitude(),
                section: section.clone(),
            }) else {
                continue;
            };
            if let Ok(mut k) = kernel.lock() {
                k.note_offered(oaddr.as_bytes(), witness, k_now);
            }
            let Some((reply, dt)) = tell_peer(&d, body) else {
                crate::transport::note_rtt_timeout(mesh, d.as_bytes());
                continue;
            };
            crate::transport::note_rtt_ok(mesh, d.as_bytes(), dt);
            offered_out += 1;
            if let Ok(mut k) = kernel.lock() {
                k.note_provisional("offers sent to witnesses");
            }
            // 23.8, I36 — A DEFERRAL IS IMPEDANCE, NEVER A VANTAGE: the
            // witness replayed nothing, and the word is carried again only
            // by a transition that concerns it. Nothing is printed per
            // deferral (C3).
            if crate::transport::is_deferred(&reply) {
                continue;
            }
            // 47.3's SECOND VANTAGE, the key the reply names, and never this
            // court's own (a voice never witnesses itself).
            let Some(w) = dialogue::decode_offer_reply(&reply) else {
                continue;
            };
            if w.vk() == mesh.vk {
                continue;
            }
            match w {
                dialogue::Witnessed::Refused { vk, why } => {
                    eprintln!(
                        "  court offer refused {:02x}{:02x}… by {:02x}{:02x}…: {}",
                        oaddr[0],
                        oaddr[1],
                        vk[0],
                        vk[1],
                        String::from_utf8_lossy(&why)
                    );
                    if let Ok(mut k) = kernel.lock() {
                        k.note_witness_answered(oaddr.as_bytes(), vk);
                        k.note_provisional("witness refused");
                    }
                }
                dialogue::Witnessed::Unmeasured { vk } => {
                    eprintln!(
                        "  court offer unmeasured {:02x}{:02x}… at {:02x}{:02x}…",
                        oaddr[0], oaddr[1], vk[0], vk[1]
                    );
                    if let Ok(mut k) = kernel.lock() {
                        k.note_provisional("witness unmeasured");
                    }
                }
                // The witness's re-derivation, read by the word's key: it
                // converges the word, and the ruling closes its orbit.
                dialogue::Witnessed::Reached { vk, form } => {
                    let mut p = 0usize;
                    let Some(theirs) = mycelium::transducer::element::Element::from_bytes(&form, &mut p) else {
                        continue;
                    };
                    if let Ok(mut k) = kernel.lock() {
                        k.witness_rederived(&oaddr, vk, theirs);
                    }
                }
            }
        }
    }
    if offered_out > 0 {
        eprintln!("  court offered {offered_out} germs to the overlap");
    }
}
