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
    let Some(dialogue::Offer { addr, home, elem: elem_bytes, section, .. }) =
        dialogue::decode_offer(&frame.body)
    else {
        return;
    };
    let me = door.mesh.vk;
    // 47.2, BEFORE ANY CARRIER IS TOUCHED. `glue_fetched` below writes a
    // Čech section into THIS MESH'S OWN COVER at the vantage the frame
    // names, pushes cocycles, marks classes, admits into `ruled`, releases
    // escrow and mints. Not one of 47.2's steps ran on this path: no
    // origin, no signature, no K, no height. Any host that could open a
    // socket could post an offer claiming to be any hypha and have this
    // court file a section under that stranger's key -- chapter 48's "free
    // identities witnessing themselves", from outside the mesh entirely.
    // I9b made it worse by making `home` load-bearing: before it, the
    // forged key was ignored; after it, the forged key is where the
    // section goes.
    let refuse_offer = |s: &mut TcpStream, why: &str| {
        eprintln!(
            "  court offer refused {:02x}{:02x}… from {:02x}{:02x}…: {why}",
            addr.as_bytes()[0],
            addr.as_bytes()[1],
            home[0],
            home[1]
        );
        answer(
            s,
            Frame {
                lane: frame.lane,
                body: dialogue::encode_offer_reply(&dialogue::Witnessed::Refused {
                    vk: me,
                    why: why.as_bytes().to_vec(),
                }),
            },
        );
    };
    let origin = match door.mesh.origin.lock() {
        Ok(o) => *o,
        Err(_) => None,
    };
    let Some(origin) = origin else {
        refuse_offer(s, "this node has adopted no origin");
        return;
    };
    // The signer is the key the receipt names, and the act's `home` is that
    // same key or the frame is two claims (A2's rule, one signer).
    let signer = section.receipt.vk;
    if let Err(why) =
        mycelium::court::admission::admit_signature(origin, &section.prefix, &signer, &section.sig)
    {
        refuse_offer(s, why);
        return;
    }
    // 47.1's binding: the prefix's payload is the OFFERED ADDRESS, and the
    // receipt names the hypha the act claims as its home. The tape is bound
    // through the address, which the witness re-derives from it below: two
    // spellings of one word have one address (law 2), so signing the
    // address signs the form and never the spelling.
    if section.prefix.payload != addr.as_bytes() || section.receipt.vk != home {
        refuse_offer(s, mycelium::verdict::REFUSE_RECEIPT);
        return;
    }
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
    let answered =
        match mycelium::transducer::element::Element::from_bytes(&elem_bytes, &mut pos) {
            Some(word) if word.address() != addr => {
                // The tape does not re-derive to the address the home
                // signed: identity is content (law 2), so this is not the
                // form that was offered whatever the frame says.
                dialogue::Witnessed::Refused {
                    vk: me,
                    why: mycelium::verdict::REFUSE_VERTEX_TAPE.as_bytes().to_vec(),
                }
            }
            Some(word) => {
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
                        // THE VANTAGE IS THE FORM THIS COURT REACHED BY
                        // REPLAYING THE OFFER on its own cover — the tape
                        // re-derived vertex by vertex, face by face, its
                        // citations resolved from THIS court's ruled words
                        // (`court_replay` inside `glue_fetched`) — and not
                        // the word this court happened to hold on the chart.
                        // `own_section(&inv)` stood here and answered with
                        // this court's OWN word on the chart, an older and
                        // different form, so the home compared the offered
                        // object against an unrelated one: `Inexact` on 57
                        // of 57 replayed offers over 200 minutes of a5da8d5,
                        // `co-signed (replayed)` 0, and once one such record
                        // stood under an object every later convergence read
                        // of it was `Inexact` too — 19,724 `Mayer–Vietoris
                        // exactness fails` on three courts, and no adjoined
                        // letter ever ruled. The replayed form is a
                        // re-derivation (47.3), not the bytes read back:
                        // this court never held them, it replayed them, and
                        // an address reached by replay is content (law 2).
                        // A glue this court could not place — a citation it
                        // does not hold, a word it heard before — has
                        // re-derived nothing and is unmeasured.
                        witness_answer(k.glue_fetched(word.clone(), home, me, &mut cover), &word, me)
                    }
                    // A court that cannot be read has judged nothing and
                    // says so: unmeasured is not a refusal of the offer.
                    _ => dialogue::Witnessed::Unmeasured { vk: me },
                }
            }
            None => dialogue::Witnessed::Refused {
                vk: me,
                why: mycelium::verdict::REFUSE_FORM_TAPE.as_bytes().to_vec(),
            },
        };
    // The witness's form may not fit the wire budget. A reply that cannot
    // carry it still ANSWERS -- as UNMEASURED, since the form did not
    // travel and the home must not read a frame it could not send as a
    // vantage. Refuse the PAYLOAD, never the reply, and never invent one:
    // the first cut of this ended `.unwrap_or_else(|| vec![lane, 0,0,0,0])`,
    // a hand-built frame for the case where a five-byte frame fails to
    // encode, which is the shape rule 11 refuses.
    let full = Frame { lane: frame.lane, body: dialogue::encode_offer_reply(&answered) };
    match full.encode() {
        Some(bytes) => {
            let _ = s.write_all(&bytes);
        }
        None => answer(
            s,
            Frame {
                lane: frame.lane,
                body: dialogue::encode_offer_reply(&dialogue::Witnessed::Unmeasured { vk: me }),
            },
        ),
    }
}

/// 47.3 — WHAT THE WITNESS ANSWERS, read off its own glue of the offered
/// form. Placed on this court's cover after replay (`Admitted`, `Attached`),
/// the form it reached IS the replayed form, and that is the second
/// vantage; slashed, the refusal travels by name; a glue this court could
/// not place — a citation it does not hold, a word it had heard — has
/// re-derived nothing, and unmeasured is not agreement and not refusal.
pub(crate) fn witness_answer(
    glue: mycelium::cognition::Glue,
    reached: &mycelium::transducer::element::Element,
    me: [u8; 32],
) -> dialogue::Witnessed {
    use mycelium::cognition::Glue;
    match glue {
        Glue::Slashed(r) => dialogue::Witnessed::Refused { vk: me, why: r.as_bytes().to_vec() },
        Glue::Admitted | Glue::Attached => {
            dialogue::Witnessed::Reached { vk: me, form: reached.to_bytes() }
        }
        Glue::Dialect => dialogue::Witnessed::Unmeasured { vk: me },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mycelium::cognition::Glue;

    fn word() -> mycelium::transducer::element::Element {
        mycelium::transducer::element::from_residual(&vec![(0i64, 0i64, 1u8, 2u8)])
            .expect("one cell is A5's chain")
    }

    /// The vantage is the form the witness replayed, reaching the offered
    /// address — never a word it held before the offer arrived.
    #[test]
    fn a_placed_offer_is_answered_with_the_replayed_form() {
        let w = word();
        let me = [3u8; 32];
        for glue in [Glue::Admitted, Glue::Attached] {
            let dialogue::Witnessed::Reached { vk, form } = witness_answer(glue, &w, me) else {
                panic!("a placed glue is a vantage");
            };
            assert_eq!(vk, me);
            let mut p = 0usize;
            let back = mycelium::transducer::element::Element::from_bytes(&form, &mut p)
                .expect("the reply carries the form");
            assert!(back.same_vertex(&w), "the witness reached the offered vertex");
            assert_eq!(back.address(), w.address());
        }
    }

    /// A slash travels by name; a glue this court could not place is
    /// unmeasured, not a vantage.
    #[test]
    fn a_slash_is_refused_by_name_and_an_unplaced_glue_is_unmeasured() {
        let w = word();
        let me = [3u8; 32];
        let dialogue::Witnessed::Refused { vk, why } = witness_answer(Glue::Slashed("why"), &w, me) else {
            panic!("a slash is a refusal");
        };
        assert_eq!((vk, why.as_slice()), (me, b"why".as_slice()));
        assert!(matches!(
            witness_answer(Glue::Dialect, &w, me),
            dialogue::Witnessed::Unmeasured { vk } if vk == me
        ));
    }
}
