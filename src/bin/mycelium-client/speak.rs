// 21, 26 — WHAT A VOICE SPEAKS: the form it extracted, and the family it
// could not close.
//
//! A family the form engine does not close is a WITHHELD form, stood in the
//! dialect rather than dropped: the claim is kept and the court is told what
//! is missing.

use mycelium::sdk::{Entity};

/// The magnitude of a family, per lane: the sum over its pairs of the
/// coboundary's lane magnitudes — what thinking it through is priced on.
/// SPEECH IS A CLOSED CONNECTION (the overlay is apply g). A thought that
/// still has a remainder is work, not a message: the family's inputs and
/// the tests are replayed through the transport, and any cell the
/// connection cannot speak keeps the whole thought here, as this mind's
/// open remainder, until it closes. Only when every replay is closed is one
/// connection per rule minted — the versor's affine block and the gauge as
/// the group component, the volume spoken across the family on the lanes,
/// and an EMPTY chain — held in the epigenome and spoken up as the word.
/// Nothing residual reaches the wire. Returns (spoken, held as thought,
/// errors).
/// Speak a closed multi-germ sentence as one Element; a withheld sentence
/// is thought. Returns true when the sentence was uttered, so per-rule
/// minting is skipped.
/// Speak a rule as its crystallized form (chapter 30 step 4): the v2 tape
/// carrying the descent the form measures over the bundle through the
/// voice's working atlas. A form with a held loop, a remainder, or no
/// measured descent is withheld and held as thought; the wire never sees
/// a nerve.
pub(crate) fn speak_form(
    entity: &mut Entity,
    tag: &str,
    rule: &mycelium::transducer::sentence::SentenceRule,
    train: &mycelium::transducer::family::Family,
    family: &[u8],
) -> bool {
    use mycelium::transducer::alphabet::Rotor;
    use mycelium::transducer::sentence::{as_element_form, Vertex};
    let fam = format!("{:02x}{:02x}…", family[0], family[1]);
    let form = &rule.form;
    // Why the form is withheld, in the order the court would refuse it.
    let why_open = || {
        let held = form
            .delta1()
            .into_iter()
            .find(|(_, w)| !w.rotor().projective_eq(&Rotor::identity()));
        if let Some((i, w)) = held {
            format!("loop {i} holonomy {:02x}{:02x}…", w.key()[0], w.key()[1])
        } else if !form.remainder.is_empty() {
            format!("remainder {} cells", form.remainder.len())
        } else {
            "no measured descent".to_string()
        }
    };
    let (mag, remainders) = mycelium::transducer::family::form_descent(form, train, &entity.epigenome);
    let n_rem: usize = remainders.iter().map(|r| r.len()).sum();
    if remainders.iter().any(|r| !r.is_empty()) {
        let n_pairs = remainders.iter().filter(|r| !r.is_empty()).count();
        entity.withhold_form(form, train, &remainders, mag);
        eprintln!(
            "[{tag}] form withheld: family {fam} remainder {n_rem} cells over {n_pairs} pairs"
        );
        return false;
    }
    let Some(word) = as_element_form(form, mag) else {
        entity.hold_form(form, train, &remainders);
        eprintln!("[{tag}] form withheld: family {fam} {}", why_open());
        return false;
    };
    if entity.already_taught(&word) {
        entity.remember_spoken(&word);
        return true;
    }
    match entity.speak(std::slice::from_ref(&word)) {
        Ok(codes) => match codes.first().copied() {
            Some(mycelium::anastome::dialogue::SPEECH_REFUSED) => {
                eprintln!("[{tag}] form withheld: family {fam} refused on the wire");
                false
            }
            _ => {
                let glued = form.vertices.iter().filter(|v| matches!(v, Vertex::Glued { .. })).count();
                eprintln!(
                    "[{tag}] form spoken: {:02x}{:02x}… depth {} vertices {} glued {} faces {} carried {:?}",
                    word.address()[0],
                    word.address()[1],
                    form.depth,
                    form.vertices.len(),
                    glued,
                    form.faces.len(),
                    word.magnitude()
                );
                true
            }
        }
        Err(e) => {
            eprintln!("[{tag}] form withheld: family {fam} {e}");
            false
        }
    }
}

/// A family the form engine does not extract is a withheld form. Each
/// pair is its own form; U1 stands it. W13 does not utter beyond that.
pub(crate) fn withhold_unextracted(
    entity: &mut Entity,
    tag: &str,
    train: &mycelium::transducer::family::Family,
    family: &[u8],
) {
    let fam = format!("{:02x}{:02x}…", family[0], family[1]);
    let scale = mycelium::transducer::family::family_ratio(train);
    for (a, b) in train {
        let f = mycelium::transducer::family::pair_form(a, b, scale);
        let pair = vec![(a.clone(), b.clone())];
        let (mag, rem) = mycelium::transducer::family::form_descent(&f, &pair, &entity.epigenome);
        entity.withhold_form(&f, &pair, &rem, mag);
    }
    eprintln!("[{tag}] form withheld: family {fam} extract refused");
}
