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
        match dialogue::encode_offer_reply(&dialogue::Witnessed::Refused { vk: me, why: why.as_bytes().to_vec() }) {
            Some(body) => answer(s, Frame { lane: frame.lane, body }),
            None => eprintln!("  court offer answer refused: the reason passes its u16"),
        }
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
    let signer = section.receipt.stamp.vk;
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
    if section.prefix.payload != addr.as_bytes() || section.receipt.stamp.vk != home {
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
    // The offer is glued AS THE HOME, into this mesh's own cover. Glued as
    // this court itself, with an empty cover, the simplex would have one
    // vertex where 13.9 needs two: the section written at this court's key
    // would be the home's own word, read straight back out and compared
    // with itself — identity every time, an agreement manufactured by
    // writing the answer into the place the question is read from — and no
    // cocycle can be pushed across a simplex with one vertex; the empty
    // cover is the same fault from the other side, a simplex with no other
    // hypha in it because the cover the mesh holds was left behind.
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
                        // 78.24 (2b): an offered word is located by the
                        // home receipt's stamp the offer's section carries;
                        // an offer this court refuses leaves nothing filed.
                        k.file_admitted(&addr, section.receipt.stamp);
                        let w = witness_vantage(&mut k, &mut cover, word, home, me);
                        if matches!(w, dialogue::Witnessed::Refused { .. }) {
                            k.drop_admitted(&addr);
                        }
                        w
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
    let full = dialogue::encode_offer_reply(&answered).and_then(|body| Frame { lane: frame.lane, body }.encode());
    match full {
        Some(bytes) => {
            let _ = s.write_all(&bytes);
        }
        None => match dialogue::encode_offer_reply(&dialogue::Witnessed::Unmeasured { vk: me }) {
            Some(body) => answer(s, Frame { lane: frame.lane, body }),
            None => eprintln!("  court offer answer refused: the unmeasured reply does not frame"),
        },
    }
}

/// 47.3 — THE WITNESS'S VANTAGE: the offered form REPLAYED on this court's
/// own tapes, and nothing else decides the answer.
///
/// THE REPLAY DECIDES, NOT THE GLUE. `glue_fetched` replays the form
/// (`court_replay`) and then files it on this court's cover — a section at
/// the home's vantage, edges to the words this court holds, simplices filled —
/// and its `Glue` says how the form RELATES TO THIS COURT'S OWN SECTION on
/// the chart: `Dialect` is "a different section of the same chart", which
/// is true of almost every offered form and says nothing against the
/// replay that just stood. Read as "unmeasured", it made 285 of 285 offer
/// replies in the first minute of 2cf578a unmeasured, for 8 objects, one
/// of them 192 times: the home re-sent every unmeasured offer after Λ and
/// the witness answered `Dialect` again, because
///
/// A HEARD ADDRESS IS NOT REPLAYED AGAIN by `glue_fetched` — it returns
/// `Dialect` before the replay for any address this court marked heard,
/// slashed or not. So the second offer of a form this court had SLASHED
/// would have read as placed. Heard, the form is replayed here by name
/// (`court_replay`, the same reader the glue runs) and the answer is that
/// replay's; unheard, the glue runs once and its slash is the replay's
/// refusal. Either way the vantage is: this court re-derived the form, or
/// it refused it by name. Unmeasured is left for a court that could not
/// be read at all.
pub(crate) fn witness_vantage(
    k: &mut mycelium::cognition::Kernel,
    cover: &mut mycelium::network::nerve::Cover,
    word: mycelium::transducer::element::Element,
    home: [u8; 32],
    me: [u8; 32],
) -> dialogue::Witnessed {
    use mycelium::cognition::Glue;
    let refused = |why: &str| dialogue::Witnessed::Refused { vk: me, why: why.as_bytes().to_vec() };
    if k.heard_contains(word.address().as_bytes()) {
        return match k.court_replay(&word) {
            Some(why) => refused(why),
            None => dialogue::Witnessed::Reached { vk: me, form: word.to_bytes() },
        };
    }
    match k.glue_fetched(word.clone(), home, me, cover) {
        Glue::Slashed(why) => refused(why),
        Glue::Admitted | Glue::Attached | Glue::Dialect => {
            dialogue::Witnessed::Reached { vk: me, form: word.to_bytes() }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word() -> mycelium::transducer::element::Element {
        mycelium::transducer::element::from_residual(&vec![(0i64, 0i64, 1u8, 2u8)])
            .expect("one link is A5's chain")
    }

    /// A word whose magnitude does not re-derive from its chain: the replay
    /// refuses it by name, every time it is asked.
    fn false_word() -> mycelium::transducer::element::Element {
        let mut w = word();
        assert!(w.set_magnitude([9, 9, 9, 9]), "a residual holds no section, so any lane fits");
        w
    }

    /// The vantage is the form the witness replayed, reaching the offered
    /// address — asked once and asked again.
    #[test]
    fn a_replayed_offer_is_answered_with_the_form_reached_each_time_it_is_asked() {
        let mut k = mycelium::cognition::Kernel::new();
        let mut cover = mycelium::network::nerve::Cover::new();
        let (home, me) = ([2u8; 32], [3u8; 32]);
        let w = word();
        for _ in 0..2 {
            let dialogue::Witnessed::Reached { vk, form } =
                witness_vantage(&mut k, &mut cover, w.clone(), home, me)
            else {
                panic!("a replay that stood is a vantage");
            };
            assert_eq!(vk, me);
            let mut p = 0usize;
            let back = mycelium::transducer::element::Element::from_bytes(&form, &mut p)
                .expect("the reply carries the form");
            assert!(back.same_vertex(&w), "the witness reached the offered vertex");
            assert_eq!(back.address(), w.address());
        }
    }

    /// A form the replay refuses is refused by name — and refused AGAIN when
    /// it is offered again, though this court has marked it heard: the
    /// heard mark is not a verdict, the replay is.
    #[test]
    fn a_false_offer_is_refused_by_name_each_time_it_is_asked() {
        let mut k = mycelium::cognition::Kernel::new();
        let mut cover = mycelium::network::nerve::Cover::new();
        let (home, me) = ([2u8; 32], [3u8; 32]);
        let w = false_word();
        let first = witness_vantage(&mut k, &mut cover, w.clone(), home, me);
        let dialogue::Witnessed::Refused { vk, why } = first else {
            panic!("a replay that refused is a refusal");
        };
        assert_eq!(vk, me);
        assert!(!why.is_empty(), "the refusal is named");
        assert!(k.heard_contains(w.address().as_bytes()), "and the address is marked heard");
        let again = witness_vantage(&mut k, &mut cover, w, home, me);
        assert!(
            matches!(again, dialogue::Witnessed::Refused { .. }),
            "heard is not a verdict: the second offer is replayed and refused again"
        );
    }
}
