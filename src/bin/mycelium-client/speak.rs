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
/// the tests are replayed through the transport, and any painted site the
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
/// carrying the trajectory the form measures over the bundle through the
/// voice's working atlas. A form with a held orbit, a remainder, or no
/// measured trajectory is withheld and held as thought; the wire never sees
/// a nerve.
/// THE FAMILY ID AS THE JOURNAL PRINTS IT, and it is FOUR bytes, not two.
///
/// It was two, and the first of those is `TAG_TASK` — a constant — so the
/// line carried eight bits of identity over a thousand-task corpus. 53.2's
/// census maps a journal id back to the task file through `task_id`, and
/// `1019…` resolved to EIGHT tasks: the stall could be seen and not named.
///
/// 78.113 — AND FOUR LEADING BYTES WERE STILL NOT IDENTITY: the task tag, a
/// length and a listing key's own tag — `10830113…` named 17 tasks and
/// `104c134a…` 22 (read 2026-10-06, finding the stall). The label is the
/// first six bytes of the WHOLE id's digest (`payload_hash`, 48 bits): a
/// label to read the journal by, never a key; a census maps it back by the
/// same digest of `task_id`.
pub(crate) fn fam_id(family: &[u8]) -> String {
    mycelium::chronometry::payload_hash(family).iter().take(6).map(|b| format!("{b:02x}")).collect::<String>() + "…"
}

pub(crate) fn speak_form(
    entity: &mut Entity,
    tag: &str,
    rule: &mycelium::transducer::sentence::SentenceRule,
    train: &mycelium::transducer::family::Family,
    frames: &mycelium::transducer::family::Frames,
    family: &[u8],
    budget: &mut mycelium::physics::Budget,
) -> Option<[u64; 4]> {
    use mycelium::transducer::alphabet::Rotor;
    use mycelium::transducer::family::trace;
    use mycelium::transducer::sentence::as_element_form;
    let fam = fam_id(family);
    let (mag, remainders) =
        mycelium::transducer::family::form_deflation(&rule.form, train, &entity.epigenome);
    // 78.81 — THE REMAINDER INTERPRETED IN THE ACT THAT SPOKE IT: inverted,
    // and what is still unsaid read as light, confluence and constructors.
    // An inversion the speech accepted IS the form spoken from here on, its
    // remainders the ones it leaves; the readings go to the stroke line.
    let (joined, reading) =
        mycelium::transducer::family::interpret::interpret(&rule.form, train, &remainders, &entity.epigenome, budget);
    let (rule, mut mag, remainders) = match joined {
        Some((form, after)) => {
            let (m, _) = mycelium::transducer::family::form_deflation(&form, train, &entity.epigenome);
            if let Some(inv) = reading.inversion {
                eprintln!(
                    "[{tag}] remainder inverted: family {fam} links {} -> {} joined {} closed {}",
                    inv.before, inv.after, inv.joined, inv.closed
                );
            }
            (std::borrow::Cow::Owned(mycelium::transducer::sentence::SentenceRule { form, ratio: rule.ratio }), m, after)
        }
        None => (std::borrow::Cow::Borrowed(rule), mag, remainders),
    };
    let rule: &mycelium::transducer::sentence::SentenceRule = &rule;
    let form = &rule.form;
    // Why the form is withheld, in the order the court would refuse it.
    let why_open = || {
        let held = form
            .delta1()
            .into_iter()
            .find(|(_, w)| !w.rotor().projective_eq(&Rotor::identity()));
        if let Some((chord, w)) = held {
            // The orbit by its chord's two vertices' coordinates (AT1).
            let at = mycelium::transducer::sentence::At::face(chord.from_vertex(), chord.to_vertex()).label();
            format!("orbit at {at} holonomy {:02x}{:02x}…", w.key()[0], w.key()[1])
        } else if !form.remainder.is_empty() {
            format!("remainder {} links", form.remainder.len())
        } else {
            "no measured trajectory".to_string()
        }
    };
    // LANE 𝒯 IS THE BUNDLE'S SPAN ON THIS ARM TOO. `form_deflation` sums
    // `measure_lanes` over the pairs, and a chain has no clock in it, so this
    // magnitude arrives as `[S, G, 0, H]`. The mint takes the span (R2) and
    // this arm does not go through the mint — it lays the form down with
    // `as_element_form` directly — so R2 reached one half of the fork and
    // left the other. Measured at 5eb95b4: 23 spoken magnitudes on this path,
    // 0 with a 𝒯 component.
    //
    // An unmeasurable span withholds rather than minting against an invented
    // extent, exactly as the demands arm does.
    let Some(deliberation) = entity.span_of(form) else {
        trace::trace("speak/span", trace::Wrote::Nothing, None);
        eprintln!("[{tag}] form withheld: family {fam} the span is unmeasurable");
        return None;
    };
    mag[2] = deliberation.span();
    // 50.5 (F step 5): a sentence with demand vertices is uttered WHOLE —
    // its closed sub-complex and its demands together — never withheld
    // and never expressed as a residual.
    if form.demand_vertices().next().is_some() {
        // A form carrying demands leaves as the demands it carries, which
        // under rule 17 is a complete answer and not a failure to write.
        trace::trace("speak/form", trace::Wrote::Demand, None);
        // The remainders are measured above and ride with it: that arm holds
        // what it does not speak, and it holds it against these.
        return speak_with_demands(entity, tag, rule, (train, frames), family, &remainders, budget);
    }
    let n_rem: usize = remainders.iter().map(|r| r.len()).sum();
    if remainders.iter().any(|r| !r.is_empty()) {
        let n_pairs = remainders.iter().filter(|r| !r.is_empty()).count();
        trace::trace("speak/remainder", trace::Wrote::Nothing, None);
        entity.withhold_perceived(form.clone(), frames, train, &remainders, mag, family, budget);
        eprintln!(
            "[{tag}] form withheld: family {fam} remainder {n_rem} links over {n_pairs} pairs"
        );
        return None;
    }
    let Some(word) = as_element_form(form, mag) else {
        trace::trace("speak/mint", trace::Wrote::Nothing, None);
        entity.hold_perceived(form.clone(), frames, train, &remainders, family, budget);
        eprintln!("[{tag}] form withheld: family {fam} {}", why_open());
        return None;
    };
    if entity.already_taught(&word) {
        trace::trace("speak/taught", trace::Wrote::Key(word.address().as_bytes()), None);
        entity.remember_spoken(&word);
        return Some(word.magnitude());
    }
    match entity.speak_with(std::slice::from_ref(&word), train, family, budget) {
        // One sentence was sent, so the reply is one code.
        Ok(codes) => match codes.as_slice() {
            [mycelium::anastome::dialogue::SPEECH_REFUSED] => {
                // THE `1019…` STALL'S OWN LINE. The court refused what the
                // voice put on the wire, and until now the only record was
                // this `eprintln!` — no coordinate, no cost, nothing a
                // timeline could line up against the extraction that made
                // it. The bytes are the tape the wire carried and threw away.
                trace::trace_bytes(
                    "speak/refused",
                    trace::Wrote::Key(word.address().as_bytes()),
                    None,
                    word.description().len() as i128,
                );
                eprintln!("[{tag}] form withheld: family {fam} refused on the wire");
                None
            }
            _ => {
                trace::trace_bytes(
                    "speak/form",
                    trace::Wrote::Key(word.address().as_bytes()),
                    None,
                    word.description().len() as i128,
                );
                let glued = form.glued_vertices().count();
                eprintln!(
                    "[{tag}] form spoken: {:02x}{:02x}… depth {} vertices {} glued {} faces {} \
                     carried {:?} deliberation {} span {}",
                    word.address()[0],
                    word.address()[1],
                    form.depth,
                    form.vertices.len(),
                    glued,
                    form.faces.len(),
                    word.magnitude(),
                    deliberation.label(),
                    deliberation.span()
                );
                Some(word.magnitude())
            }
        }
        Err(e) => {
            trace::trace("speak/wire", trace::Wrote::Nothing, None);
            eprintln!("[{tag}] form withheld: family {fam} {e}");
            None
        }
    }
}

/// THE MINT'S REFUSAL BY ARM (78.67): a vertex that does not carry its
/// family is `carry`; links the form's demands do not account for are
/// `demands`; a citation this mind does not hold is `citation`; a form
/// with nothing acting, no tape or an address that does not decode is
/// `integrity`. Exhaustive over the refusals the mint returns; any other is
/// named `other` rather than folded into one of these.
fn mint_arm(why: mycelium::transducer::sentence::Refused) -> &'static str {
    use mycelium::transducer::sentence::Refused;
    match why {
        Refused::MintVertexNotCarried(_) => "carry",
        Refused::MintLinksWithoutDemand | Refused::MintLinksUndeclared => "demands",
        Refused::CitedNotHeld | Refused::CitedSealed | Refused::OwedToEveryCourt => "citation",
        Refused::MintNothingActs | Refused::MintNoTape | Refused::MintAddressUndecoded => "integrity",
        _ => "other",
    }
}

/// 50.5 — THE SENTENCE WITH ITS DEMANDS, uttered whole: the closed
/// sub-complex measures what its words closed, the demand vertices what
/// they ask for (`sentence_measure`); one `sentence spoken` line, one
/// `demand` line per (kind, chart) and one `edge remainder` line per pair
/// of bodies no single versor carried (34.3).
/// A FORM THIS ARM DOES NOT SPEAK IS HELD, as `speak_form` holds the ones
/// it does not speak. Both of this arm's non-speaking branches returned
/// `false` and let the form go: the mint's refusal and the court's. So a
/// form carrying demand vertices — which is exactly a form with unfinished
/// work on it — was the one shape the engine threw away, while a form with
/// no demands became an idea the pivot orbit would see next stroke (30.2).
///
/// Measured on the fleet at fbfebd4 over twenty minutes: 2690 forms down
/// this arm's dropping branch against 67 down `speak_form`'s holding one —
/// **97.6% of every withheld form**. That is why the contemplation carries
/// tens of ideas where it carried scores, and why `pivot_orbit` fires on
/// nothing: the ideas that would carry a curved orbit are in the 97.6%.
///
/// The asymmetry predates D5 and D6 — they raised the traffic through it
/// from 43/min to 134.5/min, they did not create it. `remainders` rides in
/// from `speak_form`, which has already measured them, rather than being
/// re-derived here (law 12). The scene is the family as perceived: its pairs
/// and the frames read from them once.
fn speak_with_demands(
    entity: &mut Entity,
    tag: &str,
    rule: &mycelium::transducer::sentence::SentenceRule,
    (train, frames): (&mycelium::transducer::family::Family, &mycelium::transducer::family::Frames),
    family: &[u8],
    remainders: &[Vec<(i64, i64, u8, u8)>],
    budget: &mut mycelium::physics::Budget,
) -> Option<[u64; 4]> {
    use mycelium::transducer::sentence::Vertex;
    let fam = fam_id(family);
    let form = &rule.form;
    // Lane 𝒯 is the span in receipts this form's idea has stood over. A form
    // the contemplation does not hold was minted in the stroke it was
    // extracted in and stood over none.
    // A span this mind cannot measure is not a zero: the form is withheld
    // rather than minted against an invented extent.
    let Some(deliberation) = entity.span_of(form) else {
        eprintln!("[{tag}] form withheld: family {fam} the span is unmeasurable");
        return None;
    };
    let word = match mycelium::transducer::family::mint_sentence_why(rule, train, &entity.epigenome, deliberation.span()) {
        Ok(w) => w,
        Err(why) => {
            // The mint refused it, so it is not a word — it is not stood in
            // the dialect, and it is held as the open work it is. THE ARM IS
            // NAMED (78.67): this line read "trajectory, carry or demands"
            // for every arm, so 250 refusals a voice in two hours (epoch
            // 117) could not say which one held; the arm is the mint's own
            // typed refusal, its kind by 78.9.
            entity.hold_perceived(form.clone(), frames, train, remainders, family, budget);
            eprintln!(
                "[{tag}] form withheld: family {fam} mint refused ({}): {} [{:?}]",
                mint_arm(why),
                why.proof(),
                why.kind()
            );
            return None;
        }
    };
    let addr = word.address();
    // An address shorter than two bytes shows what it has; the journal
    // line names a prefix, not a value.
    let hx = |b: &[u8]| match b {
        [x, y, ..] => format!("{x:02x}{y:02x}…"),
        [x] => format!("{x:02x}…"),
        [] => "…".to_string(),
    };
    if entity.already_taught(&word) {
        entity.remember_spoken(&word);
        return Some(word.magnitude());
    }
    match entity.speak_with(std::slice::from_ref(&word), train, family, budget) {
        // One sentence was sent, so the reply is one code.
        Ok(codes) => match codes.as_slice() {
            [mycelium::anastome::dialogue::SPEECH_REFUSED] => {
                // The court refused it. The work it names is still open, so
                // it is held: the refusal names a coordinate (50.9) and the
                // pivot orbit is what acts on it.
                entity.hold_perceived(form.clone(), frames, train, remainders, family, budget);
                eprintln!("[{tag}] sentence withheld: family {fam} refused on the wire");
                None
            }
            _ => {
                let demands = form.demand_vertices().count();
                // AN OVERLAP BETWEEN TWO WORD VERTICES CARRYING NO EDGE — the
                // Frame's star excluded, read by the overlap's own ends (AT1).
                let between_words = |o: &mycelium::network::nerve::coord::Overlap| {
                    matches!(form.vertices.at(&o.from_vertex()), Some(Vertex::Word { .. }))
                        && matches!(form.vertices.at(&o.to_vertex()), Some(Vertex::Word { .. }))
                };
                let remainder_edges =
                    form.faces.entries().filter(|(o, f)| !f.has_edge() && between_words(o)).count();
                eprintln!(
                    "[{tag}] sentence spoken: {} vertices {} demands {demands} orbits {} \
                     remainder edges {remainder_edges} deliberation {} span {}",
                    hx(addr.as_bytes()),
                    form.vertices.len(),
                    form.orbits.len(),
                    deliberation.label(),
                    deliberation.span()
                );
                for (_, kind, chart, _) in form.demand_vertices() {
                    eprintln!("[{tag}] demand {} chart {} on {}", kind.label(), hx(chart.key().as_bytes()), hx(addr.as_bytes()));
                }
                for (o, f) in form.faces.entries() {
                    if !f.has_edge() && between_words(&o) {
                        let at = mycelium::transducer::sentence::At::face(o.from_vertex(), o.to_vertex()).label();
                        eprintln!("[{tag}] edge remainder {at} on {}", hx(addr.as_bytes()));
                    }
                }
                // 78.69 fix 3: the sentence closed on its family and the court
                // took it — the relation this voice composed, booked as the
                // order it built by its own magnitude, read off the word. No
                // sentence record is kept for it (78.69: sentences are the
                // client's, re-derived; the court keeps the generators).
                let mag = word.magnitude();
                entity.physics_mut().syntropy(&mag);
                Some(mag)
            }
        },
        Err(e) => {
            // The wire failed, which says nothing about the form: it is held
            // exactly as it would be had it never been offered.
            entity.hold_perceived(form.clone(), frames, train, remainders, family, budget);
            eprintln!("[{tag}] sentence withheld: family {fam} {e}");
            None
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

/// 21 STEP 3: WHAT THIS MIND OWES A COURT. For every form a court answered
/// `Awaiting` on, the cited form and the citer in one frame, cited first,
/// so the court rules the generator before it hears the sentence that
/// derives through it.
///
/// THE QUEUE WAS FILLED AND NEVER DRAINED. `utter.rs` pushes an `Owed` on
/// every `Awaiting` adjudication, and `Entity::speak_owed` — its only
/// drain — had no caller anywhere in either binary. Each entry carries the
/// form, the citer, the TASK and the whole `train`, which is the family's
/// grids, so the queue grew by a family every time a court said it was
/// waiting on a citation this voice held. Measured 2026-09-12: a voice at
/// 8.8 GiB was killed by the kernel OOM at 05:10, the fleet's voices
/// cycling between 600 MiB and 3.9 GiB while the three courts sat at
/// 165–209 MiB over five hours.
///
/// So it cost memory AND it cost the thing it exists for: no awaited
/// citation was ever taught, which is the step that lets a court rule a
/// generator it has not seen. It is drained beside `speak_contracted`, the
/// twin it was written next to.
pub(crate) fn speak_owed(entity: &mut Entity, tag: &str) {
    let spoken = entity.speak_owed();
    if let [(addr, code), ..] = spoken.as_slice() {
        eprintln!(
            "[{tag}] owed citers spoken again: {} (first {:02x}{:02x}… code {code}), {} still owed",
            spoken.len(),
            addr[0],
            addr[1],
            entity.owed_pending(),
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
    let fam = fam_id(family);
    for (a, b) in train {
        // One pair, sized by the pair (L8w): a fixed spend unit does not
        // reach one scene, and this arm would withhold as much as the
        // extraction it stands in for.
        let pair = [(a.clone(), b.clone())];
        let frames = mycelium::transducer::family::perceive_family(&pair);
        let mut budget = entity
            .stroke_n(mycelium::transducer::family::extraction_spend(&pair, &frames));
        // `None` is a vertex the stroke could not factor through this
        // mind's presentation: unmeasured, so this pair is left for the
        // next stroke rather than withheld with a reach never read.
        let Some(f) = mycelium::transducer::family::scene_form_on(
            &entity.epigenome,
            &mut budget,
            a,
            b,
            &frames,
        ) else {
            continue;
        };
        let pair = vec![(a.clone(), b.clone())];
        let (mag, rem) = mycelium::transducer::family::form_deflation(&f, &pair, &entity.epigenome);
        entity.withhold_perceived(f.clone(), &frames, &pair, &rem, mag, family, &mut budget);
    }
    eprintln!("[{tag}] form withheld: family {fam} extract refused");
}
