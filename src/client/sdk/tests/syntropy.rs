// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 3.8, 3.9 — Tests for the Bundle Synthesis Engine (Reanchor, Condense, Peel).

use crate::client::sdk::*;
use crate::transducer::alphabet::{LetterId, PlaneForm, Word};
use crate::transducer::element::{canonical_chain, from_canonical, identity_versor};
use crate::transducer::element::residual::from_connection;
use crate::transducer::rewrite::{self, LANE_GAUGE, LANE_SPATIAL};

#[test]
fn solve_gauge_between_finds_connecting_sheet() {
    let q = LetterId::quarter((0, 1)).expect("sensor quarter turn");
    let w1 = Word::from_letters(&[(q, false)]);
    let sheet = PlaneForm::sheet_word();
    let w2 = w1.rebase(&sheet);

    assert!(w1.same_class(&w2), "w1 and w2 share conjugacy class");
    assert_eq!(w1.class_key(), w2.class_key(), "w1 and w2 have identical class key");

    let found = crate::client::sdk::entity::syntropy::solve_gauge_between(&w1, &w2);
    assert!(found.is_some(), "solve_gauge_between found connecting rotor");
    let gamma = found.expect("connecting gauge rotor");
    assert!(
        w1.rebase(&gamma).rotor().projective_eq(&w2.rotor()),
        "w1 rebased by gamma equals w2"
    );
}

#[test]
fn solve_gauge_between_finds_generator_rotor() {
    let q = LetterId::quarter((0, 1)).expect("sensor quarter turn");
    let w1 = Word::from_letters(&[(q, false)]);
    let q_rot = LetterId::quarter((1, 2)).expect("sheet quarter turn");
    let g_test = Word::from_letters(&[(q_rot, false)]);
    let w2 = w1.rebase(&g_test);

    assert!(w1.same_class(&w2), "w1 and w2 share conjugacy class");
    assert_eq!(w1.class_key(), w2.class_key(), "w1 and w2 have identical class key");

    let found = crate::client::sdk::entity::syntropy::solve_gauge_between(&w1, &w2);
    assert!(found.is_some(), "solve_gauge_between found connecting rotor");
    // THE GAUGE IS A FUNCTION OF THE VERTICES, NOT OF THEIR TAPES. `γ = 1
    // + B₂·B₁⁻¹` is not projective in `B₂`, so it is built from the
    // canonical rays; handing in a tape that composes to a multiple of the
    // same rotor — which two spellings of one vertex routinely do (1.5) —
    // must reach the same gauge.
    let scaled = Word::from_letters(&[(q_rot, false), (q, false), (q_rot, true)]);
    assert!(scaled.rotor().projective_eq(&w2.rotor()), "one vertex, another tape");
    assert!(
        crate::client::sdk::entity::syntropy::solve_gauge_between(&w1, &scaled).is_some(),
        "and the gauge is found from either spelling"
    );
    let gamma = found.expect("connecting gauge rotor");
    assert!(
        w1.rebase(&gamma).rotor().projective_eq(&w2.rotor()),
        "w1 rebased by gamma equals w2"
    );
}

// L35 IS CLOSED BY DELETION, to its own DoD ("the claim runs on a fixture
// it builds, under a stated stroke, or is deleted").
//
// `probe_conjugate_reanchor_on_live_genome` read `var/1/genome` off the
// RUNNING FLEET. Three things were wrong with it and only the third made
// it fail: it passed vacuously whenever the file was absent, so a green
// suite said nothing about the claim; it read a different genome on every
// run, so a failure was not reproducible and a pass was not a proof; and
// its whole body sat inside `if let Some(...)`, so even with a genome
// present it asserted nothing unless the probe happened to fire.
//
// What ended it was 54.5 Road (d): the GEN3 record now carries the
// receipts envelope, `Element::from_bytes` refuses a record written
// before it, and the genome on disk is from the previous cut. §10 —
// A LAYOUT CHANGE IS A RECUT — and state written before it does not
// restore. A test whose subject is a file the recut wipes cannot be the
// claim's home.
//
// NOTHING IS LOST. `probe_conjugate_reanchor_on_word_pair`, directly
// below, builds its own conjugate pair, learns both, runs the probe under
// a STATED stroke (`stroke_n(1 << 8)`, not the machine's memory-calibrated
// one), and asserts every reading this made — the rewrite, the single
// retirement, the court's blind replay — with a STRICTER magnitude claim
// (`== 1` where this read `>= 1`).

#[test]
fn probe_conjugate_reanchor_on_word_pair() {
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("test_word_reanchor"));

    let q = LetterId::quarter((0, 1)).expect("sensor quarter turn");
    let w1 = Word::from_letters(&[(q, false)]);
    let sheet = PlaneForm::sheet_word();
    let w2 = w1.rebase(&sheet);

    let id_v = identity_versor();
    let e1 = from_connection(&w1, &id_v, &[], &Vec::new(), [1, 0, 0, 0]).expect("e1 connection");
    let e2 = from_connection(&w2, &id_v, &[], &Vec::new(), [1, 0, 0, 0]).expect("e2 connection");

    entity.epigenome.learn(&e1.to_bytes());
    entity.epigenome.learn(&e2.to_bytes());
    assert_eq!(entity.epigenome.genome_len(), 2, "two conjugate words in genome");

    // Under a STATED stroke: the machine's own stroke is calibrated from
    // the memory it has free when the test runs (51.11), which is no part
    // of this claim.
    let budget = entity.physics.stroke_n(1 << 8);
    let result = entity.probe_conjugate_reanchor_under(budget);
    assert!(result.is_some(), "probe found conjugate pair");
    let (rw, retires, _) = result.expect("reanchor rewrite");

    assert!(rw.is_rewrite(), "element is rewrite");
    assert_eq!(retires.len(), 1, "retires one conjugate duplicate");

    let replay_err = rewrite::replay(&rw);
    assert_eq!(replay_err, None, "court blind replay succeeds");

    let mag = rw.magnitude();
    assert_eq!(mag[LANE_SPATIAL], 1, "spatial yield is exactly 1");
}

#[test]
fn probe_bundle_condensation_folds_sibling_sections() {
    let chain_word = |links: &[(i64, i64, u8, u8)]| {
        from_canonical(&canonical_chain(links).expect("valid canonical chain"))
    };

    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("test_condense"));
    let e1 = chain_word(&[(0, 0, 1u8, 2u8), (0, 1, 1, 2)]);
    let e2 = chain_word(&[(0, 0, 3u8, 4u8), (1, 0, 3, 4)]);

    entity.epigenome.learn(&e1.to_bytes());
    entity.epigenome.learn(&e2.to_bytes());
    assert_eq!(entity.epigenome.genome_len(), 2, "two sibling words learned");

    let budget = entity.physics.stroke_n(1 << 8);
    let result = entity.probe_bundle_condensation_under(budget);
    assert!(result.is_some(), "bundle condensation found sibling pair");
    let (rw, retires, base) = result.expect("condensation candidate");
    assert!(rw.is_rewrite(), "element is rewrite");
    assert_eq!(retires.len(), 1, "retires absorbed sibling");
    assert!(base.is_some(), "base bundle produced");

    let replay_err = rewrite::replay(&rw);
    assert_eq!(replay_err, None, "court blind replay succeeds on condense rewrite");

    let mag = rw.magnitude();
    assert!(mag[LANE_GAUGE] >= 1, "gauge lane yield is non-zero: {:?}", mag);
}

#[test]
fn bid_bundle_rewrite_retires_inputs_and_credits_syntropy() {
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("test_bid_rewrite"));

    let q = LetterId::quarter((0, 1)).expect("sensor quarter turn");
    let w1 = Word::from_letters(&[(q, false)]);
    let sheet = PlaneForm::sheet_word();
    let w2 = w1.rebase(&sheet);

    let id_v = identity_versor();
    let e1 = from_connection(&w1, &id_v, &[], &Vec::new(), [1, 0, 0, 0]).expect("e1 connection");
    let e2 = from_connection(&w2, &id_v, &[], &Vec::new(), [1, 0, 0, 0]).expect("e2 connection");

    entity.epigenome.learn(&e1.to_bytes());
    entity.epigenome.learn(&e2.to_bytes());
    assert_eq!(entity.epigenome.genome_len(), 2, "two words learned");

    let budget = entity.physics.stroke_n(1 << 8);
    let (rw, retires, after) = entity
        .probe_conjugate_reanchor_under(budget)
        .expect("conjugate reanchor candidate");

    // Standing the rewrite in dialect and verifying physics yield accumulation
    let before_spatial = entity.physics.syntropic[LANE_SPATIAL];
    let mu = rw.magnitude();
    entity.physics.syntropy(&mu);
    assert_eq!(
        entity.physics.syntropic[LANE_SPATIAL],
        before_spatial + mu[LANE_SPATIAL],
        "syntropy increases on spatial lane"
    );

    // Retiring the reanchored duplicate removes it from genome
    for r in &retires {
        assert!(entity.epigenome.retire(r.as_bytes()), "input retired");
    }
    assert_eq!(entity.epigenome.genome_len(), 1, "only base word remains in genome");
    assert!(after.is_none(), "word reanchor has no new element");
}

/// Rule 1, law 3, A21 §2 — THE SIBLINGS OF A COORDINATE ARE READ AT IT, and
/// the read must answer everything the whole read it replaced would have found.
///
/// Both probes computed a chart and then used it to FILTER a read of the
/// whole genome — `for (j, h) in g.iter().enumerate()` inside
/// `for offset in 0..g.len()`, the whole carrier per candidate, unbounded by
/// any Budget. The chart is the coordinate and the atlas is keyed by it.
///
/// A narrower read passes the probes' own fixtures silently, which is why
/// this asserts the read against the WHOLE READ rather than against a probe
/// succeeding. `Stratum::fiber` reaches the node at an exact path and
/// answers the members standing THERE; `fiber_under` collects the subtree.
/// Two words of one conjugacy class at different hues sit at different
/// depths under the same versor chart — `connection_chart` is
/// `[0, class_axes…, cycle lengths…]` and `versor_chart` is its prefix — so
/// the exact read answers the empty fibre for both, and measured, it did:
/// the conjugate-pair fixture found no pair at all.
#[test]
fn the_conjugacy_fibre_answers_every_sibling_the_whole_genome_read_would_have_found() {
    use crate::transducer::stalk::invariant_of;
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("test_fibre_covers_whole_read"));

    // Two words of one conjugacy class, and a third of another, so a read
    // that answered everything would pass this as readily as one that
    // answered nothing.
    let q = LetterId::quarter((0, 1)).expect("sensor quarter turn");
    let w1 = Word::from_letters(&[(q, false)]);
    let w2 = w1.rebase(&PlaneForm::sheet_word());
    let other = PlaneForm::sheet_word();
    let id_v = identity_versor();
    for w in [&w1, &w2, &other] {
        let e = from_connection(w, &id_v, &[], &Vec::new(), [1, 0, 0, 0])
            .expect("connection");
        entity.epigenome.learn(&e.to_bytes());
    }

    let ck1 = w1.class_key().expect("class");
    assert_ne!(ck1, other.class_key().expect("class"), "the third is another class");

    // THE WHOLE READ: every genome word whose class key matches, which is
    // what the inner loop did.
    let by_whole_read: Vec<Vec<u8>> = entity
        .epigenome
        .genome_words()
        .filter(|h| {
            h.letters()
                .and_then(|l| l.spelling.word.class_key())
                .is_some_and(|ck| ck == ck1)
        })
        .map(|h| h.address().into_vec())
        .collect();
    assert!(by_whole_read.len() >= 2, "the whole read finds both conjugates: {}", by_whole_read.len());

    // THE COORDINATE: the subtree under the versor chart, confirmed by the
    // exact class key exactly as the probe confirms it.
    let vc = crate::transducer::sentence::versor_chart(&w1).expect("versor chart");
    let by_coordinate: Vec<Vec<u8>> = entity
        .epigenome
        .atlas()
        .fiber_under(&vc)
        .into_iter()
        .filter(|a| {
            entity
                .epigenome
                .word_at(a)
                .and_then(|h| h.letters())
                .and_then(|l| l.spelling.word.class_key())
                .is_some_and(|ck| ck == ck1)
        })
        .collect();

    for a in &by_whole_read {
        assert!(
            by_coordinate.contains(a),
            "the fibre answers {:02x}{:02x}… which the whole read found",
            a[0],
            a[1]
        );
    }
    assert_eq!(
        by_coordinate.len(),
        by_whole_read.len(),
        "and answers nothing the whole read did not"
    );

    // AND THE EXACT READ DOES NOT, which is the defect this replaced: the
    // third word sits at its own chart, so the versor chart's own node holds
    // only what happens to stand at that depth.
    let exact = entity.epigenome.atlas().fiber(&vc);
    assert!(
        exact.len() < by_coordinate.len(),
        "`fiber` reads one node and `fiber_under` the subtree: {} vs {}",
        exact.len(),
        by_coordinate.len()
    );

    // The condensation probe's coordinate is the WHOLE chart, so its fibre is
    // the exact node and needs no subtree. Named by the word itself and not
    // by where it landed in the genome — the genome's order is arrival, and a
    // member taken by position is a choice (rule 15).
    let e1 = from_connection(&w1, &id_v, &[], &Vec::new(), [1, 0, 0, 0]).expect("connection");
    let inv = invariant_of(&e1);
    assert!(
        entity.epigenome.atlas().fiber(&inv).contains(&e1.address().into_vec()),
        "a word stands in the fibre of its own chart"
    );
}
