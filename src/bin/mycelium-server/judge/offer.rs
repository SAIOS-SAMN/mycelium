// 47.3, 13.9 — ACT_OFFER: the second organ replays a home's first-vantage
// germ on its own tapes and answers with what it reached.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::network::wire::Frame;
use std::io::Write;
use std::net::TcpStream;

/// THE SECOND ORGAN ARRIVES HERE. A home Offers a first-vantage germ; the
/// peer replays it and glues it into its own cover, and its ruled root
/// moves, which is the evidence the home installs on (MU-ENGINEERING §3).
/// ACT_OFFER had an encoder and a decoder and no caller at either end: the
/// home marked germs `offered` in a carrier nothing read, so no peer ever
/// saw a provisional, no root ever moved, and every percept sat at `germ
/// pending (one voice)` forever with basis 0. The gate was never the germ
/// quorum; it was that the offer never left the process.
pub(crate) fn judge(s: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(dialogue::Offer { elem: elem_bytes, .. }) = dialogue::decode_offer(&frame.body) else {
        return;
    };
    let mut pos = 0usize;
    // THE WITNESS RE-DERIVES. This hypha replays the offered form on its
    // OWN tapes and reaches a form of its own; that form is what 47.3's
    // second vantage is, and until now it was reduced to one bit on the
    // way back. The reply carries it, so the home compares two
    // re-derivations by `same_vertex` (M1) rather than assuming that a
    // peer which did not slash agreed.
    let (glued, reached) =
        match mycelium::transducer::element::Element::from_bytes(&elem_bytes, &mut pos) {
            Some(word) => {
                let mut cover = mycelium::network::nerve::Cover::new();
                match door.kernel.lock() {
                    Ok(mut k) => {
                        let vk = door.mesh.vk;
                        let g = k.glue_fetched(word.clone(), vk, vk, &mut cover);
                        let ok = !matches!(g, mycelium::cognition::Glue::Slashed(_));
                        // What this court holds at that address after
                        // replay, which is its own reading of the form.
                        let mine = k.word_at(&word.address()).cloned().unwrap_or(word);
                        (ok, ok.then_some(mine))
                    }
                    Err(_) => (false, None),
                }
            }
            None => (false, None),
        };
    let mut rb = vec![glued as u8];
    if let Some(w) = reached.as_ref() {
        rb.extend_from_slice(&w.to_bytes());
    }
    // The witness's form may not fit the wire budget. A reply that cannot
    // carry it still ANSWERS, with the verdict bit alone, which the home
    // reads as "no form travelled" and holds as unmeasured. Refuse the
    // PAYLOAD, never the reply -- and never invent one: the first cut of
    // this ended `.unwrap_or_else(|| vec![lane, 0,0,0,0])`, a hand-built
    // frame for the case where a five-byte frame fails to encode, which is
    // the shape rule 11 refuses.
    let full = Frame { lane: frame.lane, body: rb };
    match full.encode() {
        Some(bytes) => {
            let _ = s.write_all(&bytes);
        }
        None => answer(s, Frame { lane: frame.lane, body: vec![glued as u8] }),
    }
}
