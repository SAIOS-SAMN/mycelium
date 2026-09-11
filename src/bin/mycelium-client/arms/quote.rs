// 24.3 arm 2 — QUOTE: ask the market what is open, and probe against it.
//
//! An occluded wire skips the ask entirely — the impedance is already
//! perceived, and the answer to it is patience rather than repetition. The
//! ask is paced by the dwell, which is a receipt count and not a timer.

use crate::Stroke;
use mycelium::physics::StrokeOutcome;
use mycelium::transducer::element::measure_lanes;

pub(crate) fn quote_arm(stroke: &mut Stroke<'_>, occluded: bool) {
let Stroke { entity, physics, c, outcome, m, tag } = stroke;
let lang = entity.atlas_standing_counts();
// The T lane is excluded from the dwell entirely (C4), so it neither
// holds a voice back nor releases it: a lane that never falls would
// make `dwell_elapsed` true forever and the quote unpaced.
let dwell_elapsed = [0usize, 1, 3]
    .iter()
    .any(|&l| m.last_pressure[l] > 0 && m.dwell_remaining[l] == 0);
let ask_quote = !occluded
    && (lang != m.last_lang || m.last_height > m.last_quoted_height || dwell_elapsed);
if ask_quote {
let lang_state = if lang != m.last_lang { "changed" } else { "same" };
eprintln!("[{tag}] quote asked: language {lang_state} book {}", m.last_height);
match entity.quote() {
    Ok(q) => {
        // 45.4: `q.k` is the BOOK's k-step, a coordinate on the hypha's
        // receipt chain. It paces the stroke line and it is not this
        // mind's clock — the mind's K counts the mind's own admitted acts
        // and is never stamped from a reply. An empty WouldBlock is
        // impedance, so a zero reply does not overwrite the last read.
        // A deferred quote carries the door's deficit as its surface and no
        // K (S13.4): the surface is dwelt on below and the book's K is
        // taken only from a quote that named one.
        if q.k != 0 {
            m.k_step = q.k;
        }
        // 24.3, V1: the language moved, so every listing may open an axis
        // again; a listing refused in this language is not retried on
        // every quote.
        if lang != m.last_lang {
            entity.reopen_axes();
        }
        m.last_lang = lang;
        // 23.8 (ruled 2026-09-09; S13.5): a height read off a reply only
        // advances. A deferral carries no height, and taking its zero as
        // the height last quoted made the next stroke ask again at once —
        // a poll on every deferral — and taking it as the book's height
        // regressed the pacing to `book 0` (voices 2 and 5, 2026-09-09).
        if q.height > m.last_quoted_height {
            m.last_quoted_height = q.height;
        }
        if q.height > m.last_height {
            m.last_height = q.height;
        }
        // V2: the height read is the mind's TUniversal, and it only
        // advances (23.8).
        entity.advance_height(q.height);
        m.last_pressure = q.surface;
        // C4, again at the quote: T is a cost, never a pace.
        m.dwell_remaining = q.surface;
        m.dwell_remaining[2] = 0;
        let (mut n_closed, mut n_descent, mut n_nothing, mut n_settled, mut n_silent, mut n_err) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let (mut n_closing, mut n_ruled_hit, mut n_stood, mut n_held) = (0u32, 0u32, 0u32, 0u32);
        // ANSWERING A DEMAND SPEAKS A FORM, and speaking one perceives a
        // whole grid. Trying every held carrier of every demand of every
        // listing served is a product of three carriers with `apply_form`
        // at the bottom of it, and nothing in the shape of a quote bounds
        // any of the three — the page is as wide as the market, a sentence
        // has as many demands as it has, and a chart holds what it holds.
        // That is rule 10's loop with no boundary, and it is paid for
        // here: one spend per carrier the voice actually speaks. A page
        // beyond one stroke is answered across quotes, never in one.
        let mut spend = physics.stroke();
        for cluster in &q.clusters {
            // THE LISTING ARRIVES WHOLE (L8q). It was a chain of cells, so
            // the only move here was `conclude_listing` — subtract the
            // cells held words already cover and probe with what deflated.
            // That is deflation, and it never produced a held generator on
            // any fleet reading: a listing closed by subtraction leaves no
            // word behind it.
            let mut pos = 0usize;
            let Some(e) = mycelium::transducer::element::Element::from_bytes(&cluster.element, &mut pos)
            else {
                continue;
            };
            let held = mycelium::transducer::element::Held::of(e);
            // A SENTENCE IS WHAT CAN BE ANSWERED. A listing that is not one
            // carries no demand vertex and asks this voice nothing; it is
            // silent here rather than deflated (rule 11).
            let Some(form) = held.form() else {
                n_silent += 1;
                continue;
            };
            let demands: Vec<_> = form
                .demand_vertices()
                .map(|(_, kind, chart, cells)| (kind.clone(), chart.clone(), cells.to_vec()))
                .collect();
            if demands.is_empty() {
                n_silent += 1;
                continue;
            }
            eprintln!(
                "[{tag}] listing {:02x}{:02x}… carries {} demands",
                cluster.address[0],
                cluster.address[1],
                demands.len()
            );
            for (kind, chart, cells) in &demands {
                // 50.5, L8r: the addresses standing on this demand's own
                // chart — THE STRATUM AS COORDINATES, not as forms. The
                // chart routes; what decides is whether the form
                // reproduces the demand's cells, which is exactly how
                // extraction step 3 decides a glue, and the form for that
                // is fetched by key at the one member about to speak.
                let stratum = entity.epigenome.chart_stratum(chart);
                let mut answered = false;
                for addr in &stratum {
                    // ONE SPEND PER FORM THIS VOICE SPEAKS. Below this is
                    // `apply_form`, which perceives a whole grid, and
                    // nothing in the shape of a quote bounds how many
                    // times it would be reached: the page is as wide as
                    // the market, a sentence has as many demands as it
                    // has, and a chart holds what it holds. A stratum
                    // beyond one stroke is answered on the next quote.
                    if !spend.spend() {
                        break;
                    }
                    let Some(word) = entity.epigenome.working_word(addr) else {
                        continue;
                    };
                    let Some(st) = entity.epigenome.standing_of(addr) else {
                        continue;
                    };
                    let Some(f) = word.form().cloned() else {
                        continue;
                    };
                    if !f.is_closed() {
                        continue;
                    }
                    if !reproduces(&entity.epigenome, &f, cells) {
                        continue;
                    }
                    let cited = word.address();
                    match mycelium::sdk::Standing::from_u8(st.as_u8()) {
                        Some(mycelium::sdk::Standing::Stood) => n_stood += 1,
                        Some(mycelium::sdk::Standing::Held) => n_held += 1,
                        Some(mycelium::sdk::Standing::Ruled) => n_ruled_hit += 1,
                        None => {}
                    }
                    n_closing += 1;
                    // THE TARGET NAMES THE DEMAND, not just the sentence:
                    // the listing's address and the demand's own key, so
                    // the court resolves which vertex is being answered
                    // (50.5 files one vertex per kind and chart).
                    let target = mycelium::transducer::sentence::demand_at(
                        cluster.address.as_bytes(),
                        kind,
                        chart,
                    );
                    // The cited form speaks in its OWN frame, so the
                    // placement is the identity (L14) and the transport
                    // carrying it there is the identity versor.
                    let cite = mycelium::anastome::dialogue::CiteWire {
                        addr: cited.clone(),
                        standing: st.as_u8(),
                        transport: mycelium::transducer::element::identity_versor(),
                        gauge: 0,
                        body: cells.clone(),
                    };
                    let mag = measure_lanes(cells);
                    StrokeOutcome::add(&mut outcome.uttered, &mag);
                    match entity.probe(&target, std::slice::from_ref(&cite)) {
                        Ok((code, _paid, _remaining)) => match code {
                            mycelium::anastome::dialogue::PROBE_CLOSED => {
                                c.derived += 1;
                                n_closed += 1;
                                physics.bump(&mag);
                                StrokeOutcome::add(&mut outcome.closed, &mag);
                                eprintln!(
                                    "[{tag}] demand {} on {:02x}{:02x}… closed by {:02x}{:02x}…",
                                    kind.label(),
                                    cluster.address[0],
                                    cluster.address[1],
                                    cited[0],
                                    cited[1]
                                );
                            }
                            mycelium::anastome::dialogue::PROBE_DESCENT => {
                                c.witnessed += 1;
                                n_descent += 1;
                                physics.bump(&mag);
                            }
                            mycelium::anastome::dialogue::PROBE_SETTLED => n_settled += 1,
                            _ => n_nothing += 1,
                        },
                        Err(_) => {
                            c.errors += 1;
                            n_err += 1;
                        }
                    }
                    answered = true;
                    break;
                }
                if !answered {
                    n_silent += 1;
                }
            }
        }
        let (atlas_ruled, atlas_stood, atlas_held) = entity.atlas_standing_counts();
        eprintln!(
            "[{tag}] [{}] quote closing {n_closing} by deflation (ruled {n_ruled_hit} stood {n_stood} held {n_held})",
            mycelium::chronometry::Height::stamp_of(entity.now_tu()),
        );
        eprintln!(
            "[{tag}] [{}] atlas words ruled {atlas_ruled} stood {atlas_stood} held {atlas_held}",
            mycelium::chronometry::Height::stamp_of(entity.now_tu()),
        );
        eprintln!(
            "[{tag}] [{}] quote {} clusters: closed {n_closed} descent {n_descent} nothing {n_nothing} settled {n_settled} silent {n_silent} err {n_err}",
            mycelium::chronometry::Height::stamp_of(entity.now_tu()),
            q.clusters.len(),
        );
        // MARKET CLEAR: a dormant outcome spends the tick at the
        // end of the stroke, not here.
    }
    Err(e) => {
        c.errors += 1;
        eprintln!("[{tag}] [{}] quote err: {e}", mycelium::chronometry::Height::stamp_of(entity.now_tu()));
    }
}
}
// Syllabus after quote so a voice can close dialect/idea charts
// before its first SYLLABUS line. Under the Budget, not a timer.
if !occluded
    && mycelium::economy::lanes_short(&physics.reserve_shortfall([true; 4])).is_empty()
    && (entity.epigenome.genome_len() == 0 || entity.epigenome.atlas_is_empty())
{
    if let Ok(n) = entity.pull_syllabus() {
        for a in entity.take_promoted() {
            eprintln!("[{tag}] promoted {:02x}{:02x}…", a[0], a[1]);
        }
        if n > 0 {
            c.learned += n as u64;
            eprintln!(
                "[{tag}] SYLLABUS +{n} ratified words (genome {})",
                entity.epigenome.genome_len(),
            );
        }
    }
}
}

/// DOES THIS HELD FORM REPRODUCE THE DEMAND'S CELLS (50.5, 21 step 3)?
///
/// The demand carries the bodies it stands for as `(row, col, before,
/// after)` (50.5), which is a scene and its image in one carrier. The form
/// is spoken on the BEFORE side and what it says must lie in the AFTER
/// side — `cells_subset_of_grid`, exactly the test extraction's step 3
/// uses to decide a glue, so a voice answering a demand and a mind gluing
/// a form apply one reading.
///
/// BOTH GRIDS ARE FRAMED FROM ONE MIN-CORNER by `scene_of_chain`, which is
/// 50.6 seam 1's own reader and already says why: "both at the chain's own
/// min-corner, so the motion between them is the chain's and never an
/// artefact of two origins". Framing each side separately would let a
/// before-side with no cell in row 0 slide against its after-side.
///
/// The placement is the IDENTITY: a cited form speaks in its own frame
/// (L14), which is what `apply_form` places by.
fn reproduces(
    epi: &mycelium::sdk::Epigenome,
    f: &mycelium::transducer::sentence::Form,
    cells: &[(i64, i64, u8, u8)],
) -> bool {
    let Some((before, after)) = mycelium::transducer::family::scene_of_chain(cells) else {
        return false;
    };
    let placement = mycelium::transducer::alphabet::Word::identity();
    mycelium::transducer::family::Crystal::speak_form(epi, f, &before, &placement)
        .is_some_and(|sp| {
            mycelium::transducer::family::cells_subset_of_grid(&sp.cells, &after)
        })
}
