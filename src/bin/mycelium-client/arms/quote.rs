// 24.3 arm 2 — QUOTE: ask the market what is open, and attest against it.
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
        let heard = q.k != 0 || q.surface.iter().any(|&x| x != 0) || !q.clusters.is_empty();
        if heard {
            m.k_step = q.k;
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
        m.last_pressure = q.surface;
        // C4, again at the quote: T is a cost, never a pace.
        m.dwell_remaining = q.surface;
        m.dwell_remaining[2] = 0;
        let (mut n_closed, mut n_descent, mut n_nothing, mut n_settled, mut n_silent, mut n_attested, mut n_err) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let (mut n_closing, mut n_ruled_hit, mut n_stood, mut n_held) = (0u32, 0u32, 0u32, 0u32);
        for cluster in &q.clusters {
            if cluster.chain.is_empty() {
                continue;
            }
            // THINK, then CONVERSE. Incidence is deconstruction:
            // every served listing is concluded. Chart keys route;
            // they never decide comprehension.
            let mycelium::sdk::Concluded { cites, residual, familiar } =
                entity.conclude_listing(&cluster.chain);
            eprintln!(
                "[{tag}] [{}] cluster {} cells: concluded -> {} cites, {} residual{}",
                mycelium::chronometry::Height::stamp_of(entity.now_tu()),
                cluster.chain.len(),
                cites.len(),
                residual.len(),
                if familiar { " (familiar)" } else { "" },
            );
            if residual == cluster.chain {
                n_silent += 1;
                eprintln!(
                    "[{tag}] listing {:02x}{:02x}… held: no held word deflates it",
                    cluster.address[0], cluster.address[1]
                );
                continue;
            }
            let expressed = mycelium::transducer::element::from_residual(&residual);
            eprintln!(
                "[{tag}] listing {:02x}{:02x}… reduced by {} cites: residual {} of {}, expressed {:02x}{:02x}…",
                cluster.address[0],
                cluster.address[1],
                cites.len(),
                residual.len(),
                cluster.chain.len(),
                expressed.address()[0],
                expressed.address()[1]
            );
            n_closing += cites.len() as u32;
            for mycelium::anastome::dialogue::CiteWire { standing: st, .. } in &cites {
                match mycelium::sdk::Standing::from_u8(*st) {
                    Some(mycelium::sdk::Standing::Stood) => n_stood += 1,
                    Some(mycelium::sdk::Standing::Held) => n_held += 1,
                    Some(mycelium::sdk::Standing::Ruled) => n_ruled_hit += 1,
                    None => {}
                }
            }
            // MULTIPLICITY: several held words deflated one listing.
            // The glue between them is a new coordinate, never a
            // failure axis.
            let held: Vec<_> = cites
                .iter()
                .filter(|c| c.standing == mycelium::sdk::Standing::Held.as_u8())
                .filter_map(|c| entity.cited_word(&c.addr))
                .collect();
            // "MULTIPLICITY OPENS AN AXIS. `open_axis` fires when several
            // held words deflate one listing at once, and the glue between
            // them is the new coordinate" (24.3). Several is a SHAPE -- two
            // or more, so there is a glue to be the coordinate -- and it is
            // read off the slice. `held.len() >= 2` is rule 2's "a count is
            // never a branch"; the shape says which case it is without
            // measuring anything.
            if let [_, _, ..] = held.as_slice() {
                if let Some(a) = entity.open_axis(&cluster.address, &cluster.chain, &held) {
                    eprintln!(
                        "[{tag}] axis opened on {:02x}{:02x}…: {} held words deflated, glue spoken as {:02x}{:02x}…",
                        cluster.address[0],
                        cluster.address[1],
                        held.len(),
                        a[0],
                        a[1]
                    );
                }
            }
            // Two utterances at most: the sentence c.settles the
            // listing; a residual not yet claimed is attested once
            // as this voice's irreducible claim. Puncture cites
            // (u32::MAX sections) are speech, not silence.
            if let (0, []) = (cites.len(), residual.as_slice()) {
                n_silent += 1;
                continue;
            }
            let mag = measure_lanes(&cluster.chain);
            StrokeOutcome::add(&mut outcome.uttered, &mag);
            if !cites.is_empty() {
                match entity.probe(&cluster.address, &cites) {
                    Ok((code, _paid, _remaining)) => match code {
                        mycelium::anastome::dialogue::PROBE_CLOSED => {
                            c.derived += 1;
                            n_closed += 1;
                            physics.bump(&mag);
                            StrokeOutcome::add(&mut outcome.closed, &mag);
                            let _ = entity.puncture_verified(&cites);
                        }
                        mycelium::anastome::dialogue::PROBE_DESCENT => {
                            c.witnessed += 1;
                            n_descent += 1;
                            physics.bump(&mag);
                            let _ = entity.puncture_verified(&cites);
                        }
                        mycelium::anastome::dialogue::PROBE_SETTLED => n_settled += 1,
                        _ => n_nothing += 1,
                    },
                    Err(_) => {
                        c.errors += 1;
                        n_err += 1;
                    }
                }
            }
            // The residual is attested whenever it stands: consensus is
            // the hypha's receipts across voices, never this voice's
            // memory of having said it. A familiar residual is one this
            // genome already expressed — said again for the quorum.
            let _ = familiar;
            if let [_, ..] = residual.as_slice() {
                match entity.attest(&residual, &cites) {
                    Ok(reply) => {
                        n_attested += 1;
                        let _ = entity.puncture_verified(&cites);
                        if reply.settle.is_some() {
                            c.settles += 1;
                        }
                        if let t @ [_, ..] = reply.verdict.taught() {
                            let _ = entity.absorb_taught(t);
                        }
                        m.last_pressure = reply.pressure;
                        // The attest reply's height only advances too (S13.5).
                        if reply.height > m.last_height {
                            m.last_height = reply.height;
                        }
                        m.dwell_remaining = reply.pressure;
                    }
                    Err(_) => {
                        c.errors += 1;
                        n_err += 1;
                    }
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
            "[{tag}] [{}] quote {} clusters: closed {n_closed} descent {n_descent} nothing {n_nothing} settled {n_settled} silent {n_silent} attested {n_attested} err {n_err}",
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
    && !physics.depleted()
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
