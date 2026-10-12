// 78.68 — THE WINDING CARRIER: a closed ring traversed again writes nothing
// until it returns, and each age is one `Return` act.

use crate::chronometry::ActKind;
use crate::cognition::court::winding::{Draw, Winding};
use crate::cognition::*;
use crate::transducer::address::Address;
use crate::transducer::stalk::Invariant;

fn returns(k: &Kernel) -> usize {
    k.chain().nodes().filter(|r| r.kind == ActKind::Return).count()
}

/// A RING OF PERIOD THREE: its first close enters the record; the next two
/// closes are carried — the winding moves and nothing is sealed — and the
/// third completes the age with one `Return`. Three ages, three returns,
/// never one act per pass. A ring closed to the identity (period one)
/// returns at every traversal after its first close, and a ring of infinite
/// order (period zero) never carries.
#[test]
fn a_traversed_ring_writes_one_return_per_age() {
    let mut k = Kernel::new();
    k.bind_identity([0x5c; 32]);
    let chart = Invariant::counts(&[1, 2]);
    let ring = b"ring";
    k.close_ring(&chart, ring, 3, &[], None);
    assert_eq!(k.winding_tally().first, 1, "the first close enters the record");
    for traversal in 1..=8usize {
        let before = k.chain().len();
        k.close_ring(&chart, ring, 3, &[], None);
        // The first close was the ring's first pass; a return falls where
        // the passes total a multiple of ℓ.
        let total = traversal + 1;
        assert_eq!(returns(&k), total / 3, "one Return per completed age");
        if total % 3 != 0 {
            assert_eq!(k.chain().len(), before, "a carried traversal seals nothing");
        }
    }
    let t = k.winding_tally();
    assert_eq!((t.returns, t.carried), (3, 5), "three ages; the other five passes carried");

    let identity = b"identity";
    k.close_ring(&chart, identity, 1, &[], None);
    let r0 = returns(&k);
    k.close_ring(&chart, identity, 1, &[], None);
    k.close_ring(&chart, identity, 1, &[], None);
    assert_eq!(returns(&k), r0 + 2, "period one: every traversal is an age");

    let open = b"rosette";
    let before = k.winding_tally().unbound;
    k.close_ring(&chart, open, 0, &[], None);
    k.close_ring(&chart, open, 0, &[], None);
    assert_eq!(k.winding_tally().unbound, before + 2, "infinite order: settled where it closes, never carried");
}

fn q(n: i64, d: i64) -> num_rational::BigRational {
    num_rational::BigRational::new(n.into(), d.into())
}

/// 78.75 — FACE CONTENT REACHES COGNITION. What a chart's complex hands back
/// through its pivots — here a fibre's charge (a boost) and a white-hole
/// sliver (a null flow at a finite point) — enters the court's field through
/// its one door, and its generators close into the chart's holonomy
/// algebra: the charge is a new dimension (a `RankStep` act); the same charge
/// again is held, no act; the sliver, whose generator does not commute with
/// the charge's, steps the rank again, and the closure holds their bracket —
/// more dimensions than the two generators. The boost stands on an 𝒮 orbit
/// and the null flow in a tube. Every fleet reading had read zero rank steps:
/// the faces' content never reached the algebra.
/// 78.101 — A NILPOTENT FACE IS A PROOF. A face of the court's nerve whose
/// holonomy is a slide never returns, but condenses — `Ωᵐ ∝ 1 + m·N`, a
/// relation of the grammar — so the court reads it as a proof and checks its
/// theorem, as it does a returning one.
#[test]
fn a_nerve_face_that_condenses_is_a_proof_checked() {
    let mut k = Kernel::new();
    k.bind_identity([0x7b; 32]);
    let chart = Invariant::counts(&[2, 4]);
    let mut cx = crate::transducer::cech::Complex::new();
    cx.arm_outflow();
    cx.flow_out(crate::transducer::cech::Flow {
        motion: crate::transducer::cga::translator_at(&[q(2, 1), q(0, 1), q(0, 1)]),
        line: crate::transducer::cech::Thread::Free,
    });
    k.complexes.insert(chart.key(), cx);
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 16);
    let before = k.winding_tally();
    k.book_outflow(&chart, b"face", &mut budget);
    let after = k.winding_tally();
    assert_eq!(after.nerve_closed, before.nerve_closed + 1, "the slide's face is a relation of the grammar: {after:?}");
    assert_eq!(after.theorems_checked, before.theorems_checked + 1, "and its theorem is checked");
}

#[test]
fn a_charts_outflow_enters_the_field_and_steps_its_holonomy_algebra() {
    use crate::transducer::cga::{self, Mv, E1, E3};
    let charge = {
        let (pi, pj) = (cga::point_at(&[q(0, 1), q(0, 1), q(0, 1)]), cga::point_at(&[q(3, 1), q(4, 1), q(0, 1)]));
        crate::physics::capacitor::Fibre::new(pi, pj, q(2, 1), q(1, 1)).expect("a fibre").charge(&q(3, 2))
    };
    let sliver = {
        let (c, p) = ([q(1, 1), q(1, 1), q(0, 1)], [q(4, 1), q(5, 1), q(0, 1)]);
        let (cc, pp) = (cga::point_at(&c), cga::point_at(&p));
        let sigma = cc.sub(&cga::e_inf().scale(&-cc.geo(&pp).scalar_part()));
        let t = cga::translator_at(&c);
        let k = t.geo(&Mv::blade(E1, q(1, 1)).geo(&Mv::blade(E3, q(1, 1)))).geo(&t.reverse()).grade(2);
        crate::physics::source::Source::of(&sigma, &pp, &k).expect("a source").sliver(&q(1, 2))
    };
    let mut k = Kernel::new();
    k.bind_identity([0x7a; 32]);
    let chart = Invariant::counts(&[2, 3]);
    let mut cx = crate::transducer::cech::Complex::new();
    cx.arm_outflow();
    k.complexes.insert(chart.key(), cx);
    let rank_steps = |k: &Kernel| k.chain().nodes().filter(|r| r.kind == ActKind::RankStep).count();
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 16);
    let hand_back = |k: &mut Kernel, m: &Mv| {
        if let Some(cx) = k.complexes.get_mut(&chart.key()) {
            cx.flow_out(crate::transducer::cech::Flow { motion: m.clone(), line: crate::transducer::cech::Thread::Free });
        }
    };
    hand_back(&mut k, &charge);
    k.book_outflow(&chart, b"face", &mut budget);
    assert_eq!(rank_steps(&k), 1, "the charge: a new dimension");
    assert_eq!(k.winding_tally().outflow_rank_steps, 1);
    hand_back(&mut k, &charge);
    k.book_outflow(&chart, b"face", &mut budget);
    assert_eq!(rank_steps(&k), 1, "the same charge again: held, no act");
    hand_back(&mut k, &sliver);
    k.book_outflow(&chart, b"face", &mut budget);
    assert_eq!(rank_steps(&k), 2, "the sliver steps it again");
    let basis = k.lie_basis.get(&chart.key()).expect("the chart's algebra").clone();
    assert!(basis.len() > 2, "their bracket joined: more than the two generators ({})", basis.len());
    assert_eq!(k.winding_tally().outflow, 3, "three motions read");
    let (tubes, ..) = k.tube_reading();
    assert!(tubes >= 1, "the null flow stands in a tube");
    assert!(k.flux_field.orbits.totals().1[crate::transducer::rewrite::LANE_SPATIAL] > 0, "the boost moved on 𝒮");
}

/// 78.69, 78.75 — THE COURT'S RANK IS THE MASTER EQUATION'S OWN TERM. A ruled
/// sentence's rings are re-derived from its address — the mint keeps them:
/// the slide family's two rings stand in the minted form as in the percept —
/// composed as sentences, and their carried words `W̄ᵢ` closed into the
/// chart's holonomy algebra: a rank step, the first time; the same sentence
/// again is held. A sentence of one pair is a tree, and is read ringless by
/// why — acyclic — never as nothing.
#[test]
fn the_courts_rank_is_read_off_the_re_derived_sentences() {
    let g = |rows: &[&str]| super::common::digits(rows);
    let slides = vec![
        (g(&["1100000", "0000000", "0002000", "0002200"]), g(&["1100000", "0000000", "0000020", "0000022"])),
        (g(&["0000110", "0000000", "0200000", "0220000"]), g(&["0000110", "0000000", "0002000", "0002200"])),
    ];
    let rule = crate::transducer::family::extract_rule(&slides, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("a rule");
    let word = crate::transducer::family::mint_sentence(&rule, &slides, &crate::transducer::family::NoCrystal, 0).expect("a sentence");
    let rings = |f: &crate::transducer::sentence::Form| crate::transducer::sentence::Form::cycles_of(&f.faces, &f.vertices).kept.len();
    let minted = crate::transducer::sentence::from_element_form(&word).expect("its form");
    assert_eq!(rings(&minted), rings(&rule.form), "the mint keeps the percept's rings");
    assert!(rings(&minted) > 0, "the slide's form holds rings");
    let addr = word.address();
    let mut k = Kernel::new();
    k.bind_identity([0x3c; 32]);
    k.provisional.insert(&addr, word);
    let b = &mut crate::physics::Physics::default().stroke_n(1 << 16);
    k.step_rank(addr.as_bytes(), b);
    let t = k.winding_tally();
    assert_eq!(t.sentences, rings(&minted) as u64, "every ring re-derived and composed as a sentence: {t:?}");
    assert_eq!(t.rank_steps, 1, "its carried words open the chart's algebra: one rank step: {t:?}");
    assert_eq!(k.chain().nodes().filter(|r| r.kind == ActKind::RankStep).count(), 1, "one RankStep act");
    k.step_rank(addr.as_bytes(), b);
    assert_eq!(k.winding_tally().rank_held, 1, "the same sentence again adds no dimension: held");

    let one_pair = super::common::sentence_of(&[(0, 0, 1), (0, 1, 1), (2, 3, 2)], &[(1, 0, 1), (1, 1, 1), (2, 4, 2)]).expect("a sentence");
    let tree = one_pair.address();
    k.provisional.insert(&tree, one_pair);
    let before = k.winding_tally();
    k.step_rank(tree.as_bytes(), b);
    let t = k.winding_tally();
    assert_eq!((t.rank_ringless - before.rank_ringless, t.rank_acyclic - before.rank_acyclic), (1, 1), "a tree is ringless, acyclic by name: {t:?}");
}

/// THE CARRIER READ BACK BY ITS OWN COORDINATES: period, winding, rewrite,
/// accrued draws and torsion — the accrued pay is a claim, kept whole.
#[test]
fn a_winding_carrier_reads_back_whole() {
    let w = Winding {
        ell: 4,
        m: 3,
        rewrite: Some(Address::from(vec![7, 8, 9])),
        draws: vec![Draw { listing: Address::from(vec![1, 2]), parties: vec![[3; 32], [4; 32]], share: [5, 0, 6, 1] }],
        torse: vec![([9; 32], [1, 2, 3, 4])],
    };
    let mut v = Vec::new();
    w.put(&mut v);
    let mut p = 0usize;
    assert_eq!(Winding::take(&v, &mut p), Some(w), "the carrier reads back");
    assert_eq!(p, v.len(), "whole");
}

/// 78.69 — THE PRESSURE A WITNESS RAISED IS RELIEVED EXACTLY WHEN THE WORD
/// LEAVES THE BOOK: `steps` times its magnitude raised, held against its
/// address, and released whole by `pending_take` — never raised `steps`
/// times and relieved once, the ratchet that climbed 𝒯 on the courts.
#[test]
fn a_witnessed_charge_is_relieved_whole_when_the_word_leaves() {
    let mut k = Kernel::new();
    let word = super::common::sentence_of(&[(0, 0, 1), (0, 1, 1)], &[(0, 0, 2), (0, 1, 2)]).expect("a sentence");
    let addr = word.address();
    k.pending.insert(&addr, word);
    let before = k.physics.pressure;
    k.charge_witnessed(&addr, &[1, 0, 3, 0], 4);
    assert_eq!(k.physics.pressure, [before[0] + 4, before[1], before[2] + 12, before[3]], "four steps raised");
    let _ = k.pending_take(addr.as_bytes()).expect("the word was pending");
    assert!(k.witnessed.get(addr.as_bytes()).is_none(), "the charge left with the word");
    assert_eq!(k.physics.pressure[2], before[2], "𝒯 back where it stood: the raised twelve relieved whole");
}
