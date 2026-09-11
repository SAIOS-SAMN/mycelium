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
    // 50.5 (F step 5): a sentence with demand vertices is uttered WHOLE —
    // its closed sub-complex and its demands together — never withheld
    // and never expressed as a residual.
    if form.demand_vertices().next().is_some() {
        return speak_with_demands(entity, tag, rule, train, family);
    }
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
    match entity.speak_with(std::slice::from_ref(&word), train, family) {
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

/// 50.5 — THE SENTENCE WITH ITS DEMANDS, uttered whole: the closed
/// sub-complex measures what its words closed, the demand vertices what
/// they ask for (`sentence_measure`); one `sentence spoken` line, one
/// `demand` line per (kind, chart) and one `edge remainder` line per pair
/// of bodies no single versor carried (34.3).
fn speak_with_demands(
    entity: &mut Entity,
    tag: &str,
    rule: &mycelium::transducer::sentence::SentenceRule,
    train: &mycelium::transducer::family::Family,
    family: &[u8],
) -> bool {
    use mycelium::transducer::sentence::Vertex;
    let fam = format!("{:02x}{:02x}…", family[0], family[1]);
    let form = &rule.form;
    let Some(word) = mycelium::transducer::family::mint_sentence(rule, train, &entity.epigenome) else {
        eprintln!("[{tag}] form withheld: family {fam} no measured descent and no demand");
        return false;
    };
    let addr = word.address();
    let hx = |b: &[u8]| format!("{:02x}{:02x}…", b.first().copied().unwrap_or(0), b.get(1).copied().unwrap_or(0));
    if entity.already_taught(&word) {
        entity.remember_spoken(&word);
        return true;
    }
    match entity.speak_with(std::slice::from_ref(&word), train, family) {
        Ok(codes) => match codes.first().copied() {
            Some(mycelium::anastome::dialogue::SPEECH_REFUSED) => {
                eprintln!("[{tag}] sentence withheld: family {fam} refused on the wire");
                false
            }
            _ => {
                let demands = form.demand_vertices().count();
                let remainder_edges = form
                    .faces
                    .iter()
                    .filter(|f| f.i != 0 && f.j != 0 && f.edge.is_none())
                    .filter(|f| {
                        matches!(form.vertices.get(f.i as usize), Some(Vertex::Word { .. }))
                            && matches!(form.vertices.get(f.j as usize), Some(Vertex::Word { .. }))
                    })
                    .count();
                eprintln!(
                    "[{tag}] sentence spoken: {} vertices {} demands {demands} loops {} remainder edges {remainder_edges}",
                    hx(addr.as_bytes()),
                    form.vertices.len(),
                    form.loops.len()
                );
                for (_, kind, chart, _) in form.demand_vertices() {
                    eprintln!("[{tag}] demand {} chart {} on {}", kind.label(), hx(chart.key().as_bytes()), hx(addr.as_bytes()));
                }
                for f in &form.faces {
                    if f.i != 0 && f.j != 0 && f.edge.is_none() {
                        eprintln!("[{tag}] edge remainder {}-{} on {}", f.i, f.j, hx(addr.as_bytes()));
                    }
                }
                true
            }
        },
        Err(e) => {
            eprintln!("[{tag}] sentence withheld: family {fam} {e}");
            false
        }
    }
}

/// 50.9: the sentences contracted after the last refusals, uttered with
/// their certificates over the family and task each was minted for. The
/// court that refused verifies the contraction before it replays.
pub(crate) fn speak_contracted(entity: &mut Entity, tag: &str) {
    for (addr, code) in entity.speak_contracted() {
        eprintln!(
            "[{tag}] contracted sentence spoken: {:02x}{:02x}… code {code}",
            addr[0], addr[1]
        );
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
        // One pair, sized by the pair (L8w): `entity.stroke()` is the same
        // three-spend purse that could not reach a single `pair_form`, so
        // this fallback withheld as much as the extraction it stands in
        // for — 469 families a pass, both arms saying nothing.
        let pair = [(a.clone(), b.clone())];
        let mut budget = entity
            .stroke_n(mycelium::transducer::family::extraction_spend(&pair));
        // `None` is a vertex the stroke could not factor through this
        // mind's presentation: unmeasured, so this pair is left for the
        // next stroke rather than withheld with a reach never read.
        let Some(f) = mycelium::transducer::family::pair_form(
            &entity.epigenome,
            &mut budget,
            a,
            b,
            scale,
        ) else {
            continue;
        };
        let pair = vec![(a.clone(), b.clone())];
        let (mag, rem) = mycelium::transducer::family::form_descent(&f, &pair, &entity.epigenome);
        entity.withhold_form(&f, &pair, &rem, mag);
    }
    eprintln!("[{tag}] form withheld: family {fam} extract refused");
}
