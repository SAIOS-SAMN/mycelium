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
    let gamma = found.expect("connecting gauge rotor");
    assert!(
        w1.rebase(&gamma).rotor().projective_eq(&w2.rotor()),
        "w1 rebased by gamma equals w2"
    );
}

#[test]
fn probe_conjugate_reanchor_on_live_genome() {
    let path = "var/1/genome";
    let Ok(bytes) = std::fs::read(path) else {
        return; // skip if var/1/genome not present
    };
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("test_reanchor"));
    assert!(entity.epigenome.uncrystallize(&bytes), "genome restored");

    if let Some((rw, retires, after)) = entity.probe_conjugate_reanchor() {
        assert!(rw.is_rewrite(), "element is a rewrite");
        assert_eq!(retires.len(), 1, "reanchor retires one input");
        assert!(after.is_some(), "after element generated");

        // Blind replay verification: court accepts with zero refusal
        let replay_err = rewrite::replay(&rw);
        assert_eq!(replay_err, None, "court blind replay succeeds on reanchor rewrite");

        // Verify metabolic yield is credited to LANE_SPATIAL
        let mag = rw.magnitude();
        assert!(mag[LANE_SPATIAL] >= 1, "spatial lane yield is non-zero: {:?}", mag);
    }
}

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

    let result = entity.probe_conjugate_reanchor();
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
    let chain_word = |cells: &[(i64, i64, u8, u8)]| {
        from_canonical(&canonical_chain(cells).expect("valid canonical chain"))
    };

    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("test_condense"));
    let e1 = chain_word(&[(0, 0, 1u8, 2u8), (0, 1, 1, 2)]);
    let e2 = chain_word(&[(0, 0, 3u8, 4u8), (1, 0, 3, 4)]);

    entity.epigenome.learn(&e1.to_bytes());
    entity.epigenome.learn(&e2.to_bytes());
    assert_eq!(entity.epigenome.genome_len(), 2, "two sibling words learned");

    let result = entity.probe_bundle_condensation();
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

    let (rw, retires, after) = entity
        .probe_conjugate_reanchor()
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
