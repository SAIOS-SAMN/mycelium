// 24.3 arm 4 — SYNTROPY: on depletion, crystallize, pivot, and regenerate.
//
//! The depleted stroke is where a voice does its inward work: it sheds, it
//! expands its dirty ideas once, and it pivots the new words on their own
//! charts.

use crate::Stroke;
use mycelium::physics::StrokeOutcome;

pub(crate) fn syntropy_arm(stroke: &mut Stroke<'_>, genome_path: &str) {
let Stroke { entity, physics, c, outcome, m, tag } = stroke;
let shed = entity.shed();
if shed > 0 {
    eprintln!("[{tag}] shed {shed} dialect");
}
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
    // the orbit count and period beside K, velocity in kernel coordinates.
    if let Some((n, period)) = ring {
        eprintln!("[{tag}] orbit ring {n} period {period}");
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
// 3. Axiomatic Floor / Adjunction (37.7): one genome word per syntropic
// stroke, read at its own conjugacy coordinate. Where the language measured
// a gauge transition it cannot spell, that rotor IS the generator the
// presentation lacks — it is adjoined to the dialect here rather than bid to
// the court, because a compression bid asks for a retirement and the court's
// replay expects a spatial reconstruction an algebraic witness has none of.
else if entity.adjoin_unspellable_gauge() {
    c.refined += 1;
    eprintln!(
        "[{tag}] ADJOINED a gauge the seventeen cannot spell (genome {})",
        entity.epigenome.genome_len(),
    );
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
    let _ = entity.save_genome(genome_path);
    m.last_genome = entity.epigenome.genome_len();
}
physics.regenerate();
}
