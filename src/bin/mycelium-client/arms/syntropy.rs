// 24.3 arm 4 — SYNTROPY: on depletion, crystallize, pivot, and regenerate.
//
//! The depleted stroke is where a voice does its inward work: it sheds, it
//! expands its dirty ideas once, and it pivots the new words on their own
//! charts.

use crate::Stroke;
use mycelium::physics::StrokeOutcome;

pub(crate) fn syntropy_arm(stroke: &mut Stroke<'_>, genome_path: &str) {
let Stroke { entity, physics, c, outcome, m, tag } = stroke;
// C3 — the shed prints nothing per stroke (ruled 2026-09-26): what it took,
// by door, is the meet graph's shed reading on the console.
entity.shed();
// 24.3, I37 — THE SYNC IS PACED AS THE QUOTE IS. Discovery, the syllabus
// pull and the sheaf sync are wire acts, and they ran on every depleted
// stroke of every voice: on the fleet of 2026-09-08 two hyphae each
// served 131,072 digest requests in ten minutes and their reflex pools
// deferred the quotes and probes that were the work. They are asked when
// the working language changed since the last sync, when the book's height
// moved (the height I39 made real), when a lane's dwell elapsed, or while
// the mind holds no language at all; never because a stroke was depleted.
// The T lane is excluded from the dwell (C4), as at the quote.
let lang = entity.atlas_standing_counts();
let dwell_elapsed = [0usize, 1, 3]
    .iter()
    .any(|&l| m.last_pressure[l] > 0 && m.dwell_remaining[l] == 0);
let unlearned = entity.epigenome.atlas_is_empty();
let sync_due = lang != m.last_synced_lang
    || m.last_height > m.last_synced_height
    || dwell_elapsed
    || unlearned;
if sync_due {
    entity.discover();
}
// Syllabus and sheaf sync live on the depletion dwell so the
// door serves quotes and percepts on the hot stroke.
{
    // THE PIVOT: expand dirty ideas once this phase, then pivot
    // new words on their charts only.
    let (pivoted_mu, ring) = entity.expand_ideas();
    StrokeOutcome::add(&mut outcome.pivoted, &pivoted_mu);
    // 45.7, S13.3: the mind's stroke ring, once per completed circuit —
    // the circuit count and period beside K, velocity in kernel
    // coordinates (73.7: the circuit is the clock).
    if let Some((n, period)) = ring {
        eprintln!("[{tag}] circuit ring {n} period {period}");
    }
    let pivoted = entity.pivot_thoughts();
    if pivoted > 0 {
        eprintln!("[{tag}] pivot: {pivoted} held thoughts spoken as demands on charts the language now holds");
    }
    if sync_due {
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
// THE SHEAF SYNC (Q3): converge the genome on the hypha's
// language by discordant stratum — the entity thinks in the
// whole language, not the syllabus drip.
if sync_due {
    if let Ok(n) = entity.sync_genome() {
        if n > 0 {
            c.learned += n as u64;
            eprintln!(
                "[{tag}] SYNC +{n} words by stratum (genome {})",
                entity.epigenome.genome_len(),
            );
        }
    }
    m.last_synced_lang = lang;
    m.last_synced_height = m.last_height;
}
// BUNDLE SYNTHESIS ENGINE:
// 1. Change of Basepoint / Conjugate Overlay (Reanchor):
// Extract gauge frame transitions γ ∈ Spin(4,0) between conjugate sections,
// earning structured metabolic yield on LANE_SPATIAL.
if let Some((rw, retires, after)) = entity.probe_conjugate_reanchor() {
    if let Ok(earned) = entity.bid_bundle_rewrite(&rw, &retires, after.as_ref()) {
        c.refined += 1;
        c.reanchored += 1;
        eprintln!(
            "[{tag}] REANCHOR {:02x}{:02x}… (gauge transition, genome {}) μ⃗ {earned:?}",
            rw.address()[0],
            rw.address()[1],
            entity.epigenome.genome_len(),
        );
    }
}
// 2. Fiber Section Condensation (Condense):
// Fold sibling forms projecting to the same base cycle into sections of a single bundle,
// earning structured metabolic yield on LANE_GAUGE.
else if let Some((rw, retires, after)) = entity.probe_bundle_condensation() {
    if let Ok(earned) = entity.bid_bundle_rewrite(&rw, &retires, after.as_ref()) {
        c.refined += 1;
        c.condensed += 1;
        eprintln!(
            "[{tag}] CONDENSE {:02x}{:02x}… (bundle condensation, genome {}) μ⃗ {earned:?}",
            rw.address()[0],
            rw.address()[1],
            entity.epigenome.genome_len(),
        );
    }
}
// 3. The algebraic step (Expansions 1, 2 and 37.7): one genome word per
// syntropic stroke, read at its own conjugacy coordinate. A gauge the
// language measured and cannot spell IS the generator the presentation
// lacks and is adjoined to the dialect; a word that induces a held word's
// operator, or is a section of its bundle by a gauge the seventeen spell,
// is bid to the court with that witness, and the court rules it on its own
// algebra (`ACT_REFINE_ALGEBRAIC`).
else if let Some(step) = entity.algebraic_step() {
    match step {
        mycelium::sdk::AlgebraicStep::Adjoined => {
            c.refined += 1;
            eprintln!(
                "[{tag}] ADJOINED a gauge the seventeen cannot spell (genome {})",
                entity.epigenome.genome_len(),
            );
        }
        mycelium::sdk::AlgebraicStep::Bid(bid) => {
            let mycelium::sdk::AlgebraicBid { word, held, witness } = *bid;
            if let Ok((code, mu)) = entity.bid_algebraic_refinement(word.as_bytes(), held.as_bytes(), &witness) {
                if code == mycelium::anastome::dialogue::REFINE_ACCEPTED {
                    c.refined += 1;
                    eprintln!(
                        "[{tag}] REFINED {:02x}{:02x}… algebraically against {:02x}{:02x}… ({}, genome {}) μ⃗ {mu:?}",
                        word[0],
                        word[1],
                        held[0],
                        held[1],
                        match witness {
                            mycelium::anastome::dialogue::AlgebraicWitness::SameOperator => "same operator",
                            mycelium::anastome::dialogue::AlgebraicWitness::Section(_) => "section",
                        },
                        entity.epigenome.genome_len(),
                    );
                }
            }
        }
    }
}
// 4. Citation Refinement (Refine): if the rest of the vocabulary already
// speaks this word spatially, bid the compression.
else if let Some((word, cites)) = entity.seek_refinement() {
    if let Ok((code, mu)) = entity.bid_refinement(&word, &cites) {
        if code == mycelium::anastome::dialogue::REFINE_ACCEPTED {
            c.refined += 1;
            eprintln!(
                "[{tag}] REFINED {:02x}{:02x}… (vocabulary compressed, genome {}) μ⃗ {mu:?}",
                word[0],
                word[1],
                entity.epigenome.genome_len(),
            );
        } else if entity.epigenome.retire(&word) {
            // This genome derives the word by its other words and
            // the judge does not hold it: the copy is compression
            // this dialect owes itself. Retired locally; a word the
            // judge does hold returns through the sync.
            c.refined += 1;
        }
    }
}
if entity.epigenome.genome_len() != m.last_genome {
    // LD0: a genome that was not saved is said, never discarded.
    if let Err(why) = entity.save_genome(genome_path) {
        eprintln!("[{tag}] genome not saved: {why}");
    }
    m.last_genome = entity.epigenome.genome_len();
}
physics.regenerate();
}
