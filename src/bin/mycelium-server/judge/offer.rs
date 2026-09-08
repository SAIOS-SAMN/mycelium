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
    let Some(dialogue::Offer { home, elem: elem_bytes, .. }) = dialogue::decode_offer(&frame.body)
    else {
        return;
    };
    let me = door.mesh.vk;
    let mut pos = 0usize;
    // THE WITNESS RE-DERIVES. This hypha replays the offered form on its
    // OWN tapes and reaches a form of its own; that form is what 47.3's
    // second vantage is, and until now it was reduced to one bit on the
    // way back. The reply carries it, so the home compares two
    // re-derivations by `same_vertex` (M1) rather than assuming that a
    // peer which did not slash agreed.
    //
    // The offer is glued AS THE HOME, into this mesh's own cover. It stood
    // as `glue_fetched(word, me, me, &mut Cover::new())`: one vertex where
    // there should be two, so the section written at this court's key was
    // the home's own word, read straight back out and compared with itself.
    // That is identity every time -- an agreement manufactured by writing
    // the answer into the place the question is read from -- and no cocycle
    // can be pushed across a cell with one vertex. The empty cover was the
    // same fault in the other direction: the cell had no other hypha in it
    // because the cover the mesh actually holds was left behind.
    let (refused, reached) =
        match mycelium::transducer::element::Element::from_bytes(&elem_bytes, &mut pos) {
            Some(word) => {
                let inv = mycelium::transducer::stalk::invariant_of(&word);
                match (door.kernel.lock(), door.mesh.cover.lock()) {
                    (Ok(mut k), Ok(mut cover)) => {
                        // One typed reading, every arm of `Glue` named. The
                        // REFUSAL travels, not a bit saying there was one:
                        // `Slashed(reason)` is a 34.1 constant the home logs
                        // and acts on.
                        //
                        // THE SECOND VANTAGE IS THIS COURT'S OWN SECTION.
                        // It was `word_at(&word.address())` — the offered
                        // address — and the `Admitted` arm of `glue_fetched`
                        // has just filed the HOME'S OWN TAPE there, so the
                        // witness handed the home back the bytes it sent and
                        // the home's `same_vertex` compared the form with
                        // itself. That is 1.7's byte exactness: "it can only
                        // confirm that a voice sent bytes it had seen
                        // before". `own_section` reads what this court wrote
                        // from its own perception, at its own vantage, and
                        // `None` — no section of its own on the class — is a
                        // witness that has re-derived nothing.
                        match k.glue_fetched(word, home, me, &mut cover) {
                            mycelium::cognition::Glue::Slashed(r) => {
                                (r.as_bytes().to_vec(), Vec::new())
                            }
                            mycelium::cognition::Glue::Admitted
                            | mycelium::cognition::Glue::Attached
                            | mycelium::cognition::Glue::Dialect => (
                                Vec::new(),
                                match k.own_section(&inv) {
                                    Some(w) => w.to_bytes(),
                                    None => Vec::new(),
                                },
                            ),
                        }
                    }
                    _ => (Vec::new(), Vec::new()),
                }
            }
            None => (mycelium::verdict::REFUSE_FORM_TAPE.as_bytes().to_vec(), Vec::new()),
        };
    let answered = dialogue::Witnessed { vk: me, reached, refused: refused.clone() };
    // The witness's form may not fit the wire budget. A reply that cannot
    // carry it still ANSWERS, with this court's key and its refusal, which
    // the home reads as "no form travelled" and holds as unmeasured. Refuse
    // the PAYLOAD, never the reply -- and never invent one: the first cut of
    // this ended `.unwrap_or_else(|| vec![lane, 0,0,0,0])`, a hand-built
    // frame for the case where a five-byte frame fails to encode, which is
    // the shape rule 11 refuses.
    let full = Frame { lane: frame.lane, body: dialogue::encode_offer_reply(&answered) };
    match full.encode() {
        Some(bytes) => {
            let _ = s.write_all(&bytes);
        }
        None => answer(
            s,
            Frame {
                lane: frame.lane,
                body: dialogue::encode_offer_reply(&dialogue::Witnessed {
                    vk: me,
                    reached: Vec::new(),
                    refused,
                }),
            },
        ),
    }
}
