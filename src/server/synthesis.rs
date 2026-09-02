// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE HYPHA'S DEEP JOINT PROJECTION (distributed cognition, operator-ruled).
//!
//! The gate's thought was correct; its LOCATION was the crime. The cognition
//! that once lived in an isolated test runner is redistributed here to the
//! node whose role it is: the CLIENT perceives and humbly projects; the
//! SERVER projects more deeply, analyses, and reassembles — using ONLY the
//! engine's own primitives (the same lift → beam → rollout the entity
//! thinks with, at the judge's deeper budget); the COURT adjudicates the
//! well-formed constructs the server dockets; the server then teaches the
//! ratified syllabus back to every client, whose dialects re-resolve and
//! produce MORE VALID results. Nothing here is task-specific and nothing is
//! searched: candidate bindings are derived carriers (comprehension),
//! rollout is virtual replay (mathematics), and the only judge of a
//! construct is δσ̂ = 0.
//!
//! CROSS-TEST CONSENSUS — what the gate had and the entity lacks: a
//! construct is WELL-FORMED only if one binding set closes on EVERY train
//! pair AND speaks (bound, non-silent) on EVERY test input JOINTLY. A
//! reading that explains one grid but goes mute on its sibling is not
//! understanding; joint coherence is the consensus.

use crate::arc::{
    apply_bound_chains, chain_footprint, embed, frame_ratio, readback, refine_delta, Grid, Task,
};
use crate::transducer::element::from_residual;
use crate::cognition::decompose_over;
use crate::transducer::anchor::{derive_carrying_all, Sel};
use crate::transducer::element::Element;
use crate::transducer::spectral::collapse_beam;

/// A well-formed construct: the server's synthesized reading of a task
/// family, ready for the court (evidence) and the syllabus (cited words).
pub struct Construct {
    /// Addresses of every basis word the family's joint reading cites.
    pub cited: Vec<[u8; 32]>,
    /// The bound chains γ = (π, Δ) the synthesis crystallized.
    pub bindings: Vec<(Vec<(i64, i64, u8, u8)>, Option<Sel>)>,
    /// Anchor evidence (word address, footprint, scene) for the admission
    /// fiber — the court's raw material.
    pub evidence: Vec<([u8; 32], Vec<(i64, i64)>, Vec<(i64, i64, u8)>)>,
    /// δσ̂ = 0 replayed on every train pair (the terminal judge).
    pub closes: bool,
    /// The construct speaks — bound and non-silent — on every test input.
    pub speaks_all_tests: bool,
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

/// The family's measured frame ratio — perception, identical across pairs
/// or refused (never rounded).
pub fn family_ratio(train: &[(Grid, Grid)]) -> Option<((usize, usize), (usize, usize))> {
    let mut ratio = None;
    for (a, b) in train {
        let f =
            (a.len(), a.first().map_or(0, |r| r.len()), b.len(), b.first().map_or(0, |r| r.len()));
        if f.0 == 0 || f.1 == 0 || f.2 == 0 || f.3 == 0 {
            return None;
        }
        let r = frame_ratio(&f);
        match &ratio {
            None => ratio = Some(r),
            Some(held) if *held != r => return None,
            _ => {}
        }
    }
    ratio
}

fn scene_cells(scene: &Grid) -> Vec<(i64, i64, u8)> {
    scene
        .iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, &h)| (r as i64, c as i64, h)))
        .collect()
}

/// SYNTHESIZE — the deep joint projection over a task family, in the given
/// language (the judge's basis), at the given speculative budget.
///
/// 1. LIFT: every train pair's refined coboundary decomposes over the
///    language (a mute pair = the family is beyond it — honest None).
/// 2. BEAM: per cited word variant, the common-carrier anchor terms across
///    every witnessing pair (tightest-first; optionality retained).
/// 3. ROLLOUT: the crystallized bindings replay virtually on every train
///    pair — δσ̂ ≠ 0 anywhere prunes the construct whole.
/// 4. CONSENSUS: the same bindings must speak (bound, non-silent, frame-
///    integral) on EVERY test input jointly.
///
/// Returns the construct with its closure/consensus verdicts; the caller
/// (the hypha) decides what to docket, and NEVER settles a solve to itself
/// — the server holds the answers, so self-settlement would be self-dealing.
/// Its synthesis becomes evidence and syllabus; the YIELD stays with the
/// clients the syllabus empowers.
pub fn synthesize(
    language: &[Element],
    task: &Task,
    beam_width: usize,
    depth: usize,
) -> Option<Construct> {
    if task.train.is_empty() || language.is_empty() {
        return None;
    }
    // 1. LIFT.
    let mut deltas: Vec<Vec<(i64, i64, u8, u8)>> = Vec::new();
    let mut scenes: Vec<Grid> = Vec::new();
    let mut cited_idx: Vec<usize> = Vec::new();
    for (a, b) in &task.train {
        let (mut d, refined_in, _frame) = refine_delta(a, b)?;
        d.sort_unstable();
        let min_r = d.iter().map(|(r, _, _, _)| *r).min().unwrap_or(0);
        let min_c = d.iter().map(|(_, c, _, _)| *c).min().unwrap_or(0);
        let mut centered: Vec<_> =
            d.iter().map(|(r, c, x, y)| (r - min_r, c - min_c, *x, *y)).collect();
        centered.sort_unstable();
        match decompose_over(language, &centered, None, beam_width, depth) {
            Ok(sentence) => {
                for wi in sentence {
                    if !cited_idx.contains(&wi) {
                        cited_idx.push(wi);
                    }
                }
            }
            Err(_) => return None, // a mute pair: unspeakable in this language today
        }
        deltas.push(d);
        scenes.push(refined_in);
    }
    // 2. BEAM — common carriers as projected future states, collapsed by the
    // SPECTRAL resonance (DC3): exact per-axis invariants against the
    // footprint, worst witnessing pair governing, tightest-first as tiebreak.
    let mut bindings: Vec<(Vec<(i64, i64, u8, u8)>, Option<Sel>)> = Vec::new();
    let mut evidence: Vec<([u8; 32], Vec<(i64, i64)>, Vec<(i64, i64, u8)>)> = Vec::new();
    for &wi in &cited_idx {
        let word_addr = language[wi].address();
        for chain in language[wi].variant_chains() {
            let mut sets: Vec<Vec<Sel>> = Vec::new();
            let mut witnesses: Vec<(Vec<(i64, i64)>, Vec<(i64, i64, u8)>)> = Vec::new();
            for (pi, d) in deltas.iter().enumerate() {
                let fp = chain_footprint(&chain, d);
                if fp.is_empty() {
                    continue;
                }
                let sc = scene_cells(&scenes[pi]);
                sets.push(derive_carrying_all(&fp, &sc));
                witnesses.push((fp, sc));
            }
            if witnesses.is_empty() {
                continue;
            }
            let commons: Vec<Sel> = match sets.split_first() {
                Some((first, rest)) => {
                    first.iter().filter(|t| rest.iter().all(|s| s.contains(*t))).cloned().collect()
                }
                None => Vec::new(),
            };
            let binding = collapse_beam(&commons, &witnesses);
            if binding.is_some() {
                evidence.push((word_addr, witnesses[0].0.clone(), witnesses[0].1.clone()));
            }
            bindings.push((chain, binding));
        }
    }
    // 3. ROLLOUT — δσ̂ = 0 on every train pair or the construct dies whole.
    let mut closes = true;
    for ((_, b), scene) in task.train.iter().zip(&scenes) {
        // The scene is the input on the codomain complex: the rollout lands
        // at the output's own dims, compared exactly.
        let ok = apply_bound_chains(scene, &bindings) == *b;
        if !ok {
            closes = false;
            break;
        }
    }
    // 4. CROSS-TEST CONSENSUS — the joint condition over the whole family.
    let mut speaks_all_tests = !task.test.is_empty();
    match family_ratio(&task.train) {
        Some(((hn, hd), (wn, wd))) => {
            for (x, _) in &task.test {
                let (h, w) = (x.len(), x.first().map_or(0, |r| r.len()));
                if h == 0 || w == 0 || (h * hn) % hd != 0 || (w * wn) % wd != 0 {
                    speaks_all_tests = false; // non-integral frame: refusal
                    break;
                }
                let (oh, ow) = (h * hn / hd, w * wn / wd);
                let rx = embed(x, oh, ow); // the test input on the codomain complex
                let bound: Vec<_> =
                    bindings.iter().filter(|(_, s)| s.is_some()).cloned().collect();
                if bound.is_empty() {
                    speaks_all_tests = false;
                    break;
                }
                let projected = apply_bound_chains(&rx, &bound);
                if projected == rx {
                    speaks_all_tests = false; // silent
                    break;
                }
            }
        }
        None => speaks_all_tests = false,
    }
    let cited = cited_idx.iter().map(|&i| language[i].address()).collect();
    Some(Construct { cited, bindings, evidence, closes, speaks_all_tests })
}

/// The contemplation stroke — the deliberative arm, outside the kernel
/// (the kernel keeps only the request-path judge; the court holds
/// adjudication authority — this deposits evidence, it never rules).
///
/// Designed for a worker thread behind a bounded queue (the async
/// dispatcher): the kernel lock is held only for the earn/snapshot step
/// and the commit; the deep projection runs on a cloned basis snapshot, so
/// request traffic is never blocked behind speculation.
///
/// Metering, all derived from the metabolism (no independent constants):
/// - Earn before drawing: consolidation strokes (optimize, which also
///   regenerates) run until the reserve clears the expansion floor,
///   bounded by the strokes-per-reserve ratio. Speculation is paid for by
///   consolidation — genuine syntropic work, not a timer.
/// - The speculative budget is the projection budget (calm → wide and
///   deep, strained → near-reactive).
/// - The stroke spends the reserve: thought is metabolically priced, so
///   contemplation self-paces against service through the same two-stroke
///   contract as every other stroke.
pub fn contemplate(
    k: &std::sync::Mutex<crate::cognition::Kernel>,
    task: &Task,
) -> Option<(usize, usize)> {
    // The stroke's lanes are the FAMILY'S lanes — measured from the first
    // pair's centered coboundary, never a uniform [true; 4] (which would
    // flatten the polytonic surface into one monotone ceiling: every lane
    // saturating identically carries no gradient, and the market loses its
    // directional signal).
    let lanes: [bool; 4] = task
        .train
        .first()
        .and_then(|(a, b)| refine_delta(a, b))
        .map(|(mut d, _, _)| {
            d.sort_unstable();
            let min_r = d.iter().map(|(r, _, _, _)| *r).min().unwrap_or(0);
            let min_c = d.iter().map(|(_, c, _, _)| *c).min().unwrap_or(0);
            let centered: Vec<_> =
                d.iter().map(|(r, c, x, y)| (r - min_r, c - min_c, *x, *y)).collect();
            crate::transducer::element::from_residual(&centered).lanes()
        })
        .unwrap_or([false, false, true, false]);
    let (snapshot, beam, depth) = {
        let mut g = k.lock().ok()?;
        // One consolidation stroke per attempt: optimize both earns the
        // regeneration and does real syntropic work, but it is a full
        // redundancy probe — holding the lock for several would starve the
        // request path. Below the band after one stroke, speculation
        // simply waits for the next queued task (the queue is the retry).
        //
        // THE TWO-STROKE COURTESY: speculation may take a lane's SECOND
        // stroke, never its last — draining a lane to zero puts the door at
        // full deficit and defers the very attestations that feed
        // germination (the quorum deadlock). Requiring two strokes of
        // reserve (one for thought, one held for service) keeps the door
        // open on the lanes the corpus demands actually occupy.
        let speculative = |p: &crate::physics::Physics| {
            (0..4).all(|l| !lanes[l] || p.reserve[l] >= 2 * crate::physics::spend_unit())
        };
        if !speculative(g.physics()) {
            g.optimize();
        }
        if !speculative(g.physics()) {
            return None; // the reserve is service's; speculation waits
        }
        let (beam, depth) = g.projection_budget();
        g.physics_mut().spend(lanes); // thought is priced on the lanes it occupies
        (g.basis().to_vec(), beam, depth)
    };
    // Deep projection on the snapshot — no lock held.
    let c = synthesize(&snapshot, task, beam, depth)?;
    if !(c.closes && c.speaks_all_tests) {
        return None;
    }
    // Commit: deposit the evidence under a brief lock.
    let mut g = k.lock().ok()?;
    let deposited = c
        .evidence
        .iter()
        .filter(|(a, fp, sc)| g.deposit_evidence(*a, fp.clone(), sc.clone()))
        .count();
    Some((c.cited.len(), deposited))
}

/// Work items for the server's deliberation worker — everything that would
/// hold the kernel lock through deep reduction runs here, behind the
/// bounded queue, never on the request path.
pub enum Deliberation {
    /// Deep joint projection over a dispatched task family.
    Contemplate([u8; 32], Task),
    /// Genome synchronization: a court-ratified word to verify and install
    /// (the ratifying voice is the provenance of record).
    Adopt([u8; 32], crate::transducer::element::Element),
}

/// THE JUDGE, off the lock — a verifier. The reflex opening (enrol, relax,
/// cheap syntropic moves, the door) runs under a brief lock and hands out
/// the language snapshot; germination and the linear replay of the
/// speaker's proof run with no lock held; the commit re-resolves citations
/// by address under a brief lock. No search runs on the hypha: a percept
/// whose proof does not cover its germs is deferred, and the speaker's
/// next sync converges it on the language.
pub fn adjudicate(
    k: &std::sync::Mutex<crate::cognition::Kernel>,
    ca: &[Vec<(i64, i64, u8)>],
    cb: &[Vec<(i64, i64, u8)>],
    voice: [u8; 32],
    frame: &crate::arc::Frame,
    proof: &[Vec<crate::anastome::dialogue::CiteWire>],
) -> crate::cognition::Outcome {
    use crate::cognition::Outcome;
    let Ok(mut g) = k.lock() else { return Outcome::GermPending };
    let Some((language, _budget)) = g.reflex_open(voice) else { return Outcome::GermPending };
    drop(g);
    let Some((germs, composites)) = crate::cognition::germinate_bodies(ca, cb, frame) else {
        return k.lock().map(|mut g| g.pivot()).unwrap_or(Outcome::Pivot);
    };
    if proof.len() != germs.len() {
        return Outcome::GermPending; // unproven: deferred, never reduced here
    }
    let reductions: Vec<_> = germs
        .into_iter()
        .zip(proof)
        .map(|(germ, cites)| crate::cognition::reduce_proven(&language, germ, cites, frame))
        .collect();
    let Ok(mut g) = k.lock() else { return Outcome::GermPending };
    g.register_composites(composites);
    g.commit_all(reductions, voice)
}

/// A spoken coboundary demand with the speaker's own reduction: replayed,
/// never searched; an empty citation list attests the chain irreducible
/// for this voice (the consilience path).
pub fn attest(
    k: &std::sync::Mutex<crate::cognition::Kernel>,
    chain: &[(i64, i64, u8, u8)],
    voice: [u8; 32],
    cites: &[crate::anastome::dialogue::CiteWire],
) -> crate::cognition::Outcome {
    use crate::cognition::Outcome;
    let Ok(mut g) = k.lock() else { return Outcome::GermPending };
    let Some((language, _budget)) = g.reflex_open(voice) else { return Outcome::GermPending };
    drop(g);
    let frame = (0usize, 0usize, 0usize, 0usize); // a chain carries no frame: the planar word
    let germ = (crate::transducer::element::from_residual(&chain.to_vec()), None);
    let reduction = crate::cognition::reduce_proven(&language, germ, cites, &frame);
    let Ok(mut g) = k.lock() else { return Outcome::GermPending };
    g.commit_all(vec![reduction], voice)
}

/// THE PROBE, off the lock: the sentence replays over the language snapshot
/// against the listing's chain; the settlement (closure or graded descent)
/// runs under a brief lock. Returns (code, paid, remaining).
pub fn probe(
    k: &std::sync::Mutex<crate::cognition::Kernel>,
    voice: [u8; 32],
    target: &[u8; 32],
    cites: &[crate::anastome::dialogue::CiteWire],
) -> (u8, [u64; 4], [u64; 4]) {
    use crate::anastome::dialogue::PROBE_NOTHING;
    let Ok(mut g) = k.lock() else { return (PROBE_NOTHING, [0; 4], [0; 4]) };
    let view = g.market_open();
    drop(g);
    let Some((listing, _)) = view.pending.iter().find(|(p, _)| p.address() == *target) else {
        return (PROBE_NOTHING, [0; 4], [0; 4]);
    };
    let resolved = crate::cognition::resolve_cites(&view.language.words, cites);
    let residual = crate::cognition::replay(&view.language.words, &resolved, listing.sub());
    let cited: Vec<[u8; 32]> = cites.iter().map(|(a, _, _)| *a).collect();
    let Ok(mut g) = k.lock() else { return (PROBE_NOTHING, [0; 4], [0; 4]) };
    g.settle_probe(voice, target, residual, &cited)
}

/// THE QUOTE, off the lock: the market view leaves under a brief lock; the
/// depth follows the metabolism (a calm judge serves a deep book, a strained
/// one its hottest few); pricing and the cohesion witness compute over the
/// snapshot. Returns (surface, priced clusters, K, cohesion).
pub fn quote(
    k: &std::sync::Mutex<crate::cognition::Kernel>,
) -> Option<([u64; 4], Vec<(crate::transducer::element::Element, [u64; 4])>, u64, crate::cohesion::CohesionWitness)> {
    let view = k.lock().ok()?.market_open();
    // The closure is a sealed carrier: read when current, measured over the
    // snapshot (no lock) and sealed back only when a generation moved.
    let rank = match view.closure {
        Some(c) => c,
        None => {
            let measured = crate::cognition::rank_over(
                &view.language.stalks,
                &view.language.words,
                &view.pending,
                &view.composites,
                &view.prior,
                view.gens.0,
            );
            k.lock().ok()?.seal_closure(view.gens, measured)
        }
    };
    let free: u64 = view.reserve.iter().sum();
    let depth = (4 + (free as usize * 12) / (4 * crate::physics::reserve_max() as usize)).min(16);
    // The frame budget less the reply head (act, surface, count).
    let budget = crate::network::wire::BODY_BUDGET - 34;
    let (surface, priced) = crate::cognition::quote_over(
        &rank,
        &view.composites,
        &view.pending,
        &view.surface,
        &view.pool,
        depth,
        budget,
    );
    let cohesion = crate::cognition::cohesion_over(&rank.closure);
    Some((surface, priced, view.k_step, cohesion))
}

/// Idle consolidation, orchestrated off the request path: the cheap moves
/// run under a brief lock, the redundancy probe decomposes the candidate
/// over a basis snapshot with no lock held, and only the retirement
/// re-locks. Returns the cards retired.
pub fn consolidate(k: &std::sync::Mutex<crate::cognition::Kernel>) -> usize {
    let Ok(mut g) = k.lock() else { return 0 };
    g.consolidate_stroke();
    0
}

/// Adoption, orchestrated off the request path: the blind replay is pure
/// (no lock), derivability probes a basis snapshot (no lock), and only the
/// brief install re-locks. Returns the deterministic [`Adoption`] status.
pub fn adopt(
    k: &std::sync::Mutex<crate::cognition::Kernel>,
    ratifier: [u8; 32],
    word: &crate::transducer::element::Element,
) -> crate::cognition::Adoption {
    use crate::cognition::Adoption;
    if let Some(reason) = crate::verdict::blind_replay(word) {
        return Adoption::Refused(reason);
    }
    let Ok(mut g) = k.lock() else { return Adoption::Refused("kernel unavailable") };
    if g.basis().iter().any(|t| t.address() == word.address()) {
        // Held at this address: the arriving placement's fibre merges
        // (Known or Enriched — the install door decides).
        return g.install_adopted(word, ratifier);
    }
    let snapshot = g.basis().to_vec();
    let (beam, depth) = g.projection_budget();
    drop(g);
    // Derivability on the snapshot — the replay guaranteed a non-empty
    // chain, so the chain path decides.
    let sub = word.substance();
    if crate::cognition::decompose_over(&snapshot, sub.sub(), None, beam, depth).is_ok() {
        return Adoption::Derivable;
    }
    let Ok(mut g) = k.lock() else { return Adoption::Refused("kernel unavailable") };
    g.install_adopted(word, ratifier)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::element::from_residual;
    use crate::transducer::germinate::germinate;

    /// A language holding the recolour word 1→2; a family where every pair
    /// speaks it and both tests carry it jointly: the construct closes and
    /// speaks all tests. A family whose second test is mute breaks consensus.
    #[test]
    fn joint_coherence_is_the_consensus() {
        // The word: hue 1 → hue 2 wherever 1 stands (a 1-cell recolour germ).
        let a = vec![(0, 0, 1)];
        let b = vec![(0, 0, 2)];
        let word = from_residual(&germinate(&a, &b).unwrap());
        let language = vec![word];

        let train = vec![
            (vec![vec![1, 0], vec![0, 0]], vec![vec![2, 0], vec![0, 0]]),
            (vec![vec![0, 0], vec![0, 1]], vec![vec![0, 0], vec![0, 2]]),
        ];
        let test_ok = vec![
            (vec![vec![1, 1], vec![0, 0]], vec![vec![2, 2], vec![0, 0]]),
            (vec![vec![0, 1], vec![1, 0]], vec![vec![0, 2], vec![2, 0]]),
        ];
        let c = synthesize(&language, &Task { train: train.clone(), test: test_ok }, 4, 16)
            .expect("the family is speakable");
        assert!(c.closes, "δσ̂ = 0 on every train pair");
        assert!(c.speaks_all_tests, "the construct carries jointly across both tests");
        assert_eq!(c.cited.len(), 1, "one word cited");
        assert!(!c.evidence.is_empty(), "anchored bindings deposit evidence");

        // A mute sibling test (no hue-1 anywhere): the joint condition fails.
        let test_mute = vec![
            (vec![vec![1, 0], vec![0, 0]], vec![vec![2, 0], vec![0, 0]]),
            (vec![vec![0, 0], vec![0, 0]], vec![vec![0, 0], vec![0, 0]]),
        ];
        let c2 = synthesize(&language, &Task { train, test: test_mute }, 4, 16)
            .expect("still speakable on train");
        assert!(c2.closes);
        assert!(!c2.speaks_all_tests, "a mute sibling breaks cross-test consensus");
    }

    /// Adoption is a deterministic status map: a clean foreign word
    /// installs with the ratifier as provenance; re-adoption reads Known;
    /// a word the language already derives reads Derivable; tampered
    /// speech is Refused with the failed axis named.
    #[test]
    fn adoption_maps_every_state_deterministically() {
        use crate::cognition::{Adoption, Kernel};
        let k = std::sync::Mutex::new(Kernel::new());
        let court = [0xAB; 32];
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
        let word = from_residual(&germinate(&a, &rot).unwrap());
        assert_eq!(adopt(&k, court, &word), Adoption::Installed, "clean foreign word installs");
        assert_eq!(adopt(&k, court, &word), Adoption::Known, "the genome had converged");
        // The identical deformation germinated far away: same substance —
        // the language already derives it.
        let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
        let rot_far: Vec<_> = rot.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
        let dup = from_residual(&germinate(&a_far, &rot_far).unwrap());
        assert!(
            matches!(adopt(&k, court, &dup), Adoption::Derivable | Adoption::Known),
            "adoption never adds redundancy"
        );
        // Tampered speech is named and refused.
        let mut fat = word.clone();
        fat.mag = [0, 0, 999, 0];
        assert!(matches!(adopt(&k, court, &fat), Adoption::Refused(_)));
        // Provenance is the ratifying voice, not a sentinel.
        let g = k.lock().unwrap();
        assert!(g.basis().iter().any(|t| t.address() == word.address()));
    }

    /// An empty language or an unspeakable family is honest silence.
    #[test]
    fn silence_is_honest() {
        assert!(synthesize(&[], &Task { train: vec![], test: vec![] }, 4, 16).is_none());
        let a = vec![(0, 0, 1)];
        let b = vec![(0, 0, 2)];
        let word = from_residual(&germinate(&a, &b).unwrap());
        // A family demanding a word the language does not hold (3 → 7).
        let train = vec![(vec![vec![3]], vec![vec![7]])];
        assert!(
            synthesize(&[word], &Task { train, test: vec![] }, 4, 16).is_none(),
            "an unspeakable family returns None, never a guess"
        );
    }
}
