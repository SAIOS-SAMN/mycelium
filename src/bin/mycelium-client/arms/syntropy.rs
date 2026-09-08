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
entity.discover();
// Syllabus and sheaf sync live on the depletion dwell so the
// door serves quotes and percepts on the hot stroke.
{
    // THE PIVOT: expand dirty ideas once this phase, then pivot
    // new words on their charts only.
    let pivoted_mu = entity.expand_ideas();
    StrokeOutcome::add(&mut outcome.pivoted, &pivoted_mu);
    let pivoted = entity.pivot_thoughts();
    if pivoted > 0 {
        eprintln!("[{tag}] pivot: {pivoted} held thoughts spoken as demands on charts the language now holds");
    }
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
// THE SHEAF SYNC (Q3): converge the genome on the hypha's
// language by discordant stratum — the entity thinks in the
// whole language, not the syllabus drip.
if let Ok(n) = entity.sync_genome() {
    if n > 0 {
        c.learned += n as u64;
        eprintln!(
            "[{tag}] SYNC +{n} words by stratum (genome {})",
            entity.epigenome.genome_len(),
        );
    }
}
// REFINEMENT PROBE (0x09): one genome word per syntropic stroke —
// if the rest of the vocabulary already speaks it, bid the
// compression. The hypha's replay rules; on acceptance the word
// retires fleet-wide and the entity earns the compression yield.
if let Some((word, cites)) = entity.seek_refinement() {
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
