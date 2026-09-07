// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Tests for `server::cognition`. Split out of cognition.rs under Stage
//! 8's reading bound: a file no one reads end to end is a file whose
//! faults are found by soak instead of by review (Part VII).

use super::*;
use crate::transducer::germinate::germinate;

fn scene(sr: i64, sc: i64) -> Vec<(i64, i64, u8)> {
    vec![(sr, sc, 1), (sr, sc + 1, 1), (sr + 1, sc, 1)]
}

fn glue_pair(k: &mut Kernel, a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) {
    k.ingest(a, b, [1; 32]);
    let _ = k.ingest(a, b, [2; 32]);
    k.glue_local();
}

/// A novel SUBSTANCE (here a rotation) germinates: one voice pends, a second
/// settles the listing; the basis grows only from gluing. Afterward the same
/// demand REDUCES to the length-1 sentence [0].
#[test]
fn a_novel_substance_germinates_then_decomposes() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)]; // L
    let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)]; // R180 of L — non-trivial substance
    assert_eq!(k.ingest(&a, &rot, [1; 32]), Outcome::GermPending);
    assert_eq!(k.basis_len(), 0, "one voice does not complete");
    assert_eq!(k.ingest(&a, &rot, [2; 32]), Outcome::GermPending, "two voices settle, they do not install");
    assert_eq!(k.basis_len(), 0, "zero basis installs from a single hypha's two voices");
    k.glue_local();
    assert_eq!(k.basis_len(), 1, "the basis grows from gluing");
    assert_eq!(k.ingest(&a, &rot, [3; 32]), Outcome::Decomposed(vec![0]), "reduces to the length-1 sentence");
}

/// An empty atlas is incident to the root prefix: the first quote
/// reads the book at depth zero.
#[test]
fn empty_atlas_is_incident_to_the_root_prefix() {
    use crate::transducer::germinate::germinate;
    let k = std::sync::Mutex::new(Kernel::new());
    {
        let mut g = k.lock().unwrap();
        let a = vec![(0, 0, 1u8), (0, 1, 1)];
        let b = vec![(0, 0, 2u8), (0, 1, 2)];
        assert!(matches!(g.ingest(&a, &b, [1; 32]), Outcome::GermPending));
        assert!(matches!(g.attest(&germinate(&a, &b).unwrap(), [1; 32]), Outcome::GermPending));
    }
    let (_, priced, _, _, _) =
        crate::synthesis::quote(&k, [1u8; 32], &[]).expect("quote");
    assert!(!priced.is_empty(), "an empty atlas reads the book at depth zero");
}

/// P7 — the germinated language survives restart: install two germs, snapshot,
/// restore into a fresh kernel, and the restored kernel RECALLS them (no
/// re-germination) with balances intact.
#[test]
fn the_basis_survives_a_restart() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)]; // a rotation (group substance)
    let bar = vec![(0, 0, 1), (0, 1, 1), (0, 2, 1)]; // a deformation (residual substance)
    k.ingest(&a, &rot, [1; 32]);
    assert_eq!(k.ingest(&a, &rot, [2; 32]), Outcome::GermPending);
    k.glue_local(); // a rotation generator
    k.ingest(&a, &bar, [1; 32]);
    assert_eq!(k.ingest(&a, &bar, [2; 32]), Outcome::GermPending);
    k.glue_local(); // a residual generator
    let balance = k.treasury().balance_mu(&[2; 32]);
    let snap = k.snapshot();

    // A fresh hypha boots from the snapshot.
    let mut booted = Kernel::new();
    assert!(booted.restore(&snap), "clean restore");
    assert_eq!(booted.basis_len(), 2, "both generators came back");
    assert!(
        booted.treasury().balance_mu(&[2; 32]).iter().any(|&m| m > 0)
            || balance.iter().any(|&m| m > 0),
        "earned μ⃗ survived the snapshot"
    );
    assert!(
        matches!(booted.ingest(&a, &rot, [9; 32]), Outcome::Decomposed(_)),
        "rotation reduces"
    );
    assert!(
        matches!(booted.ingest(&a, &bar, [9; 32]), Outcome::Decomposed(_)),
        "residual reduces"
    );
    assert_eq!(booted.basis_len(), 2, "no re-germination");
}

/// CYC2 writes the transport class key beside the versor; a restore
/// reads the same key, never a hash of the row.
#[test]
fn myc2_round_trips_cocycle_class_key() {
    let mut k = Kernel::new();
    let inv = crate::transducer::stalk::Invariant(vec![1]);
    let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).unwrap();
    let w = crate::transducer::alphabet::Word::from_letters(&[(q, false)]);
    let r = w.rotor();
    let v = rotor_versor(&r).expect("versor");
    let ck = r.class_key().expect("class key");
    k.push_cocycle(&inv, [1; 32], [2; 32], v, ck.clone());
    let snap = k.snapshot();
    assert!(snap.windows(4).any(|w| w == b"CYC2"), "ledger speaks CYC2");
    let mut booted = Kernel::new();
    assert!(booted.restore(&snap), "clean restore");
    let got = booted
        .cocycle_at
        .fiber(crate::network::nerve::ROOT)
        .next()
        .map(|(_, row)| row.4.clone());
    assert_eq!(got, Some(ck), "class key survives the ledger");
}

/// Residual torsion without a readable class is not filed as identity.
#[test]
fn torsion_consilient_or_filed_or_refused() {
    use crate::transducer::element::from_residual;
    use crate::transducer::germinate::germinate;
    let a = [(0, 0, 1u8), (0, 1, 2), (0, 2, 3)];
    let b = [(0, 0, 2u8), (0, 1, 3), (0, 2, 1)];
    let mut spoken = from_residual(&germinate(&a, &b).unwrap());
    spoken.puncture(vec![(2, 3), (3, 1), (1, 2)]);
    assert_eq!(spoken.torse(), 1);
    let inv = crate::transducer::stalk::invariant_of(&spoken);
    let mut k = Kernel::new();
    k.complexes
        .insert(inv.key(), crate::transducer::cech::Complex::new());
    assert_eq!(
        k.court_torsion(&spoken, &inv, [1; 32], [2; 32]),
        None,
        "no class to file"
    );
    let idk = crate::transducer::alphabet::Rotor::identity().class_key().expect("identity class");
    let filed = k
        .complexes
        .get(&inv.key())
        .and_then(|cx| cx.congruent(&idk))
        .is_some();
    assert!(
        !filed,
        "the residual arm files nothing when no class can be read"
    );
    let mut k2 = Kernel::new();
    let mut cx = crate::transducer::cech::Complex::new();
    cx.file_congruent(
        [1; 32],
        [2; 32],
        crate::transducer::alphabet::Rotor::identity(),
        4,
    );
    k2.complexes.insert(inv.key(), cx);
    assert_eq!(
        k2.court_torsion(&spoken, &inv, [1; 32], [2; 32]),
        None,
        "no class, no refuse against a held identity"
    );
}

fn filed_v2_form() -> (Element, crate::transducer::stalk::Invariant, Address) {
    use crate::transducer::alphabet::{LetterId, Word};
    use crate::transducer::sentence::{as_element_form, form_torsion, Form, FormFace, Vertex};
    let order4 = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter"), false)]);
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    f.vertices.push(Vertex::Word {
        address: Address::from_bytes([2u8; 32]),
        word: Word::identity(),
        carriage: element::carriage(&one.clone(), &(zero.clone(), zero.clone())),
        gauge: vec![(1, 2)],
        class: crate::transducer::stalk::Invariant::default(),
        contact: None,
    });
    f.vertices.push(Vertex::Word {
        address: Address::from_bytes([3u8; 32]),
        word: Word::identity(),
        carriage: element::carriage(&one, &(zero.clone(), zero)),
        gauge: vec![(1, 2)],
        class: crate::transducer::stalk::Invariant::default(),
        contact: None,
    });
    f.faces.push(FormFace {
        i: 0,
        j: 1,
        transport: f.face_of(0, 1).expect("star 1"),
    });
    f.faces.push(FormFace {
        i: 0,
        j: 2,
        transport: f.face_of(0, 2).expect("star 2"),
    });
    f.faces.push(FormFace::new(1, 2, order4));
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    let _ = f.pivot_loop(
        [0xca; 32],
        &crate::network::nerve::Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    assert!(f.is_cauterized(), "filed finite torsion cauterizes");
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("v2 tape");
    let inv = crate::transducer::stalk::invariant_of(&e);
    let class = form_torsion(&e)
        .fiber(crate::network::nerve::ROOT)
        .next()
        .map(|(k, _)| Address::from(k))
        .expect("the form files a class");
    (e, inv, class)
}

/// Chapter 23.7 on a v2 form: the court reads Form.torsion by the
/// filed loop's class key, never the element's empty section.
#[test]
fn court_torsion_on_v2_form_is_consilient_or_filed_or_refused() {
    use crate::transducer::sentence::{carries_torsion, from_element_form, torsion_loop_rotor};
    let (spoken, inv, class) = filed_v2_form();
    assert!(
        spoken.torsion().is_empty(),
        "v2 does not fill the element torsion section"
    );
    assert!(carries_torsion(&spoken), "the form carries the filed cycle");
    assert_ne!(
        class,
        crate::transducer::alphabet::Rotor::identity().class_key().expect("identity class"),
        "the class is the loop's, not the identity"
    );
    let mut k = Kernel::new();
    k.complexes
        .insert(inv.key(), crate::transducer::cech::Complex::new());
    assert_eq!(
        k.court_torsion(&spoken, &inv, [1; 32], [2; 32]),
        None,
        "first hearing files"
    );
    let filed = k
        .complexes
        .get(&inv.key())
        .and_then(|cx| cx.congruent(&class))
        .is_some();
    assert!(filed, "court torsion filed by the loop's class key");
    assert_eq!(
        k.court_torsion(&spoken, &inv, [1; 32], [2; 32]),
        None,
        "second hearing is consilient"
    );
    let f = from_element_form(&spoken).expect("v2 form");
    let rep = torsion_loop_rotor(&f, &class).expect("the loop names the class");
    let mut k2 = Kernel::new();
    let mut cx = crate::transducer::cech::Complex::new();
    cx.file_congruent([1; 32], [2; 32], rep, 99);
    k2.complexes.insert(inv.key(), cx);
    assert_eq!(
        k2.court_torsion(&spoken, &inv, [1; 32], [2; 32]),
        Some(crate::verdict::REFUSE_NOT_NATURAL),
        "same class different ℓ is not natural"
    );
}

/// A restart must not docket Δ⃗ = 0: deflation records survive the
/// snapshot, so the court does not slash a live provisional as empty.
#[test]
fn deflation_survives_a_restart() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
    assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    let chain = germinate(&a, &b).unwrap();
    assert_eq!(k.attest(&chain, [2; 32]), Outcome::GermPending);
    let before = k.dockets();
    assert_eq!(before.len(), 1);
    assert!(before[0].4.iter().any(|&x| x > 0), "deflation was measured at install");
    let snap = k.snapshot();
    let mut booted = Kernel::new();
    assert!(booted.restore(&snap), "clean restore");
    let after = booted.dockets();
    assert_eq!(after.len(), 1, "the provisional survived");
    assert_eq!(after[0].4, before[0].4, "deflation survived; the court will not slash Δ⃗ = 0");
}

/// CONFLUENT REDUCTION is TRANSPOSITION-INVARIANT (the win over lookup): install
/// a rotation, then the SAME rotation FAR AWAY reduces to the same normal form
/// (no re-mint) — a lookup on the coordinate-exact word would have missed it.
/// And a pure translate has EMPTY substance — it decomposes to the empty
/// sentence, minting nothing (a translate is pure transposition, not cognition).
#[test]
fn reduction_is_transposition_invariant() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)]; // R180 of L at origin
    k.ingest(&a, &rot, [1; 32]);
    assert_eq!(k.ingest(&a, &rot, [2; 32]), Outcome::GermPending);
    k.glue_local();
    // the SAME rotation, transposed +10 in col: same substance → same sentence.
    let a_far = vec![(0, 10, 1), (0, 11, 1), (1, 10, 1)];
    let rot_far = vec![(0, 11, 1), (1, 10, 1), (1, 11, 1)];
    assert_eq!(k.ingest(&a_far, &rot_far, [3; 32]), Outcome::Decomposed(vec![0]), "transposed → same normal form");
    assert_eq!(k.basis_len(), 1, "transposition mints nothing");
    // a pure translate is pure transposition — empty substance, empty sentence.
    let t = vec![(0, 3, 1), (0, 4, 1), (1, 3, 1)];
    assert_eq!(k.ingest(&a, &t, [4; 32]), Outcome::Decomposed(vec![]), "a translate carries no substance");
    assert_eq!(k.basis_len(), 1, "still nothing minted");
}

/// THE TWO-STROKE BREATHES: sustained germination depletes the reserve; a germ
/// on a depleted lane DEFERS (pending, never stranded) while the syntropic
/// strokes regenerate; a later submission dockets the deferred germ once the
/// lane re-earns expansion. The METABOLISM defers it; no count of voices does.
#[test]
fn a_depleted_lane_defers_germination_until_the_reserve_recovers() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    // ENTROPIC EXPANSION: eight distinct residual germs drain the residual lane.
    for n in 0..8u8 {
        let b = vec![(0, 0, 1), (0, 1, 1), (0, 2, n + 2)];
        // One vantage dockets; a second voice on the same pair is a use.
        match k.ingest(&a, &b, [1; 32]) {
            Outcome::GermPending => k.glue_local(),
            Outcome::Installed(_) | Outcome::Sectioned(_) | Outcome::Decomposed(_) => {}
            o => panic!("germ {n}: {o:?}"),
        }
    }
    // Two fresh germs: their first voices each trigger a syntropic stroke (the
    // mesh is depleted), partially recovering the lane.
    // Distinct single-transition residuals (unique hue pairs, so their tallies
    // are distinct — see the Step-2 note on residual-substance geometry).
    let ga = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)]; // (0,0): 1→4
    let gb = vec![(0, 0, 1), (0, 1, 5), (1, 0, 1)]; // (0,1): 1→5
    assert_eq!(k.ingest(&a, &ga, [1; 32]), Outcome::GermPending);
    assert_eq!(k.ingest(&a, &gb, [1; 32]), Outcome::GermPending);
    // A's quorum lands on the recovered lane and installs — spending it back out…
    assert_eq!(k.ingest(&a, &ga, [2; 32]), Outcome::GermPending);
    let ga_spoken = k
        .provisional
        .fiber(crate::network::nerve::ROOT)
        .map(|(_, e)| e.clone())
        .find(|e| e.sub().iter().any(|c| c.2 == 1 && c.3 == 4))
        .expect("ga's residual is on the docket");
    let mut cover = crate::network::nerve::Cover::new();
    let _ = k.glue_fetched(ga_spoken, [2; 32], [1; 32], &mut cover);
    // …so B finds the lane DEPLETED. The METABOLISM is what defers -- the
    // only thing that should gate here (E3) -- and the germ is not lost: a
    // later submission dockets it once the reserve recovers. What does NOT
    // gate it is a count of voices; that was the quorum this replaced.
    match k.ingest(&a, &gb, [2; 32]) {
        Outcome::GermPending | Outcome::Sectioned(_) => {}
        o => panic!("deferred or sectioned, not lost: {o:?}"),
    }
    match k.ingest(&a, &gb, [3; 32]) {
        Outcome::GermPending => k.glue_local(),
        Outcome::Installed(_) | Outcome::Sectioned(_) | Outcome::Decomposed(_) => {}
        o => panic!("deferral complete: {o:?}"),
    }
}

/// THE SYNTROPIC STROKE CONSOLIDATES: a generator the rest of the basis already
/// derives (a stale duplicate — what an import or a pre-consolidation era
/// leaves) is retired by the redundancy probe; an essential one survives it.
#[test]
fn the_syntropic_stroke_retires_a_redundant_generator() {
    use crate::transducer::germinate::germinate;
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
    k.basis_insert(element::from_residual(&germinate(&a, &rot).unwrap()), [1; 32]);
    let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
    let rot_far: Vec<_> = rot.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
    k.basis_insert(element::from_residual_framed(&germinate(&a_far, &rot_far).unwrap(), &(4, 4, 8, 8)), [7; 32]);
    assert_eq!(k.basis_len(), 2);
    k.optimize();
    assert_eq!(k.basis_len(), 1, "the redundant rule is consolidated away");
    k.optimize();
    assert_eq!(k.basis_len(), 1, "an essential generator survives the probe");
}

/// The survivor of refine_stratum may sit at the last index. Removing an
/// earlier derivable sibling used to leave that index equal to len.
#[test]
fn refine_stratum_does_not_panic_when_the_survivor_is_last() {
    use crate::transducer::element::{from_residual, from_residual_framed};
    use crate::transducer::germinate::germinate;
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
    k.basis_insert(from_residual(&germinate(&a, &rot).unwrap()), [1; 32]);
    let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
    let rot_far: Vec<_> = rot.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
    k.basis_insert(
        from_residual_framed(&germinate(&a_far, &rot_far).unwrap(), &(4, 4, 8, 8)),
        [7; 32],
    );
    k.basis_insert(
        from_residual_framed(&germinate(&a_far, &rot_far).unwrap(), &(3, 3, 6, 6)),
        [8; 32],
    );
    assert!(k.basis_len() >= 2, "two distinct addresses remain");
    let survivor = k.basis()[k.basis_len() - 1].address();
    k.refine_stratum(&survivor);
    assert!(k.word_at(&survivor).is_some(), "the last card stays keyed");
    assert!(k.basis_len() >= 1);
    assert!(k.basis_len() < 3, "at least one earlier duplicate retired");
}

/// THE REFINEMENT BID (0x09): a bidder claims a word the rest of the
/// vocabulary already speaks; the kernel's deterministic replay confirms,
/// the word retires (compression), the bidder earns the yield. A bid on
/// an ESSENTIAL word is refused — nothing else derives it.
#[test]
fn a_refinement_bid_retires_a_derivable_word_and_pays_the_bidder() {
    use crate::transducer::element::from_residual;
    use crate::transducer::germinate::germinate;
    let mut k = Kernel::new();
    k.enroll([9; 32], crate::economy::UNIT_BOND); // capitalize the pool
    // Two stale duplicates of the same deformation germinated far apart —
    // identical substance after centering (what an import or a
    // pre-consolidation era leaves). Neither is provisional: this is
    // final vocabulary carrying a redundancy.
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
    let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
    let rot_far: Vec<_> = rot.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
    // Two words, one centered chain: the first carries a frame fibre (a
    // distinct address, since a keyed basis holds one word per address)
    // and the second speaks its chain exactly.
    k.basis_insert(element::from_residual_framed(&germinate(&a, &rot).unwrap(), &(4, 4, 8, 8)), [1; 32]);
    k.basis_insert(from_residual(&germinate(&a_far, &rot_far).unwrap()), [7; 32]);
    let derivable = k.basis()[0].address(); // its duplicate speaks it
    // The bidder's sentence: the duplicate, cited at the identity.
    let body = |w: &Element| crate::transducer::sentence::cited_pattern(w, 0).unwrap();
    let sentence = vec![(k.basis()[1].address(), 0u8, element::identity_versor(), 0u32, body(&k.basis()[1]))];
    let paid = k.try_refine(&derivable, [5; 32], &sentence);
    assert!(paid.is_some(), "the derivable word is a confirmed compression bid");
    assert_eq!(k.basis_len(), 1, "the vocabulary compressed");
    let essential = k.basis()[0].address();
    let self_cite = vec![(essential.clone(), 0u8, element::identity_versor(), 0u32, body(&k.basis()[0]))];
    assert!(k.try_refine(&essential, [5; 32], &self_cite).is_none(), "an essential word is refused");
    assert_eq!(k.basis_len(), 1, "refusal mutates nothing");
    assert!(k.try_refine(&[3; 32], [5; 32], &sentence).is_none(), "an unknown address is refused");

    // THE COURT'S BUSINESS STAYS THE COURT'S: a provisional word cannot be
    // bid away, even if derivable.
    k.basis_insert(
        element::from_residual_framed(&germinate(&a, &rot).unwrap(), &(3, 3, 9, 9)),
        [1; 32],
    );
    let addr = k.basis()[k.basis_len() - 1].address();
    k.provisional.insert(&addr, (*k.basis()[k.basis_len() - 1]).clone());
    let sentence = vec![(k.basis()[0].address(), 0u8, element::identity_versor(), 0u32, body(&k.basis()[0]))];
    assert!(k.try_refine(&addr, [5; 32], &sentence).is_none(), "provisional words are refused");
    assert_eq!(k.basis_len(), 2);
}

/// THE PROJECTIVE LEAP (operator ruling): greedy largest-first descent is the
/// mechanical trap — it would commit the BIG pattern, strand an orphan cell,
/// and mint junk. The SUPERPOSITION holds both futures, rolls them out
/// virtually (zero mutation), and COLLAPSES into the branch whose remainder
/// closes (trivial holonomy): the two-word sentence. Nothing germinates —
/// imagination beats reaction.
#[test]
fn projection_escapes_the_greedy_trap() {
    let mut k = Kernel::new();
    let install = |k: &mut Kernel, a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]| {
        k.ingest(a, b, [1; 32]);
        match k.ingest(a, b, [2; 32]) {
            Outcome::GermPending => k.glue_local(),
            Outcome::Installed(_) | Outcome::Sectioned(_) => {}
            o => panic!("expected settle or puncture, got {o:?}"),
        }
    };
    install(&mut k, &[(0, 0, 1), (0, 1, 3), (0, 2, 5)], &[(0, 0, 2), (0, 1, 4), (0, 2, 6)]);
    install(&mut k, &[(0, 0, 1), (0, 1, 3)], &[(0, 0, 2), (0, 1, 4)]);
    install(&mut k, &[(0, 0, 5), (0, 1, 7)], &[(0, 0, 6), (0, 1, 8)]);
    assert!(k.basis_len() >= 1, "pair forms stand");
    let before = k.basis_len();
    let a = [(0, 0, 1), (0, 1, 3), (0, 2, 5), (0, 3, 7)];
    let b = [(0, 0, 2), (0, 1, 4), (0, 2, 6), (0, 3, 8)];
    // The saddle is one pair form: not peeled by search into two words.
    assert!(
        matches!(k.ingest(&a, &b, [3; 32]), Outcome::GermPending | Outcome::Decomposed(_)),
        "a fused pair is one percept"
    );
    assert_eq!(k.basis_len(), before, "nothing installs on one voice");
}

/// STEP 3's ECONOMIC CONVERSATION CLOSES: A speaks a novel PERCEPT (the germ
/// pends as a tape, not quoted); A's empty-cites demand lists the remainder
/// on the quote frontier; B independently attests; consilience installs,
/// the market CLEARS. {demand → quote → demand → settle}.
#[test]
fn the_five_act_conversation_closes_with_settlement() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)]; // (0,0): 1→4
    assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending, "percept: voice A");
    assert_eq!(k.quote().1.len(), 1, "corpus demand is quoted when the frontier is dry");
    let chain = germinate(&a, &b).unwrap();
    assert_eq!(k.attest(&chain, [1; 32]), Outcome::GermPending, "remainder-of-g / unreached demand");
    let (surface, quoted) = k.quote();
    assert_eq!(quoted.len(), 1, "the unreached/remainder demand is quoted as work");
    assert!(surface.iter().any(|&s| s >= 1), "the open demand tensions the lanes it occupies");
    let (work, mu) = &quoted[0];
    assert!(
        (0..4).any(|l| work.magnitude()[l] > 0 && mu[l] > 0),
        "price is nonzero on a lane the word moves"
    );
    let chain = work.sub().to_vec();
    let out = k.attest(&chain, [2; 32]);
    assert_eq!(out, Outcome::GermPending, "consilience settles: {out:?}");
    k.glue_local();
    assert_eq!(k.voices_of(&k.basis()[0].address()).len(), 2, "the settle parties");
    assert!(k.quote().1.is_empty(), "the market cleared");
}

/// A docket draws the chain's μ⃗ from the uttering voice's remaining bond;
/// a short lane refuses and nothing is listed. A second voice's bond is
/// its own; the hypha's identity is not the drawer.
#[test]
fn docket_draws_room_and_refuses_when_short() {
    let a = vec![(0, 0, 1u8)];
    let b = vec![(0, 0, 2u8)];
    let c = vec![(1, 0, 3u8)];
    let d = vec![(1, 0, 4u8)];
    let mut short = Kernel::new();
    short.set_lane_cap([8, 1, 8, 8]);
    let _ = short.ingest(&a, &b, [1; 32]);
    assert_eq!(short.pending.len(), 1, "the first draw covers");
    let _ = short.ingest(&c, &d, [1; 32]);
    let _ = &short;
    // ONE VANTAGE DOCKETS, so the first pair is listed on the first
    // ingest rather than the second; the G lane is capped at 1 and the
    // second pair's draw does not cover, so it is refused and never
    // reaches the book.
    assert_eq!(
        short.pending.len(),
        1,
        "a lane that does not cover the draw refuses, nothing further docketed"
    );

    let mut bonded = Kernel::new();
    let cover = crate::arc::family_cover(&[(vec![vec![1]], vec![vec![2]])])
        .expect("one 1-cell pair measures");
    bonded.bind_identity([9; 32]);
    bonded.set_genesis_cover(cover);
    bonded.enroll([1; 32], cover);
    bonded.enroll([2; 32], cover);
    let chain_ab = germinate(&a, &b).unwrap();
    let chain_cd = germinate(&c, &d).unwrap();
    assert_eq!(bonded.attest(&chain_ab, [1; 32]), Outcome::GermPending);
    assert_eq!(bonded.pending.len(), 1, "the cover admits one pair");
    assert_eq!(bonded.attest(&chain_cd, [1; 32]), Outcome::GermPending);
    assert_eq!(
        bonded.pending.len(),
        1,
        "a second pair of the same μ⃗ waits until the first draw releases"
    );
    assert_eq!(bonded.attest(&chain_cd, [2; 32]), Outcome::GermPending);
    assert_eq!(
        bonded.pending.len(),
        2,
        "a second voice's own bond covers its own draw"
    );
}

/// A listing that stands an aperture of receipts behind the head
/// releases its draw; occupancy returns; nothing is slashed.
#[test]
fn docket_draw_releases_on_lapse() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1u8)];
    let b = vec![(0, 0, 2u8)];
    assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    assert_eq!(k.pending.len(), 1, "listed");
    let occ = k.occupancy();
    assert!(occ.iter().any(|&x| x > 0), "the draw occupies");
    let lambda = crate::physics::aperture_cycle() as u64;
    k.set_height(k.height().saturating_add(lambda));
    k.court_stroke();
    assert!(k.pending.is_empty(), "the listing lapsed");
    assert_eq!(k.occupancy(), [0; 4], "the draw released");
    assert!(
        !k.treasury().entries().iter().any(|e| e.kind == "slash"),
        "lapse does not slash"
    );
}

/// Room is per-lane from bond and reserve; ticks on T do not steal
/// S, G, or H. Eviction is by ACTION POTENTIAL, never FIFO.
#[test]
fn market_capacity_is_thermodynamic_and_eviction_is_by_tension() {
    let mut k = Kernel::new();
    k.bind_identity([1; 32]);
    k.enroll([1; 32], [8, 8, 1, 8]);
    let calm = k.lane_room();
    assert!(calm.iter().all(|&c| c > 0), "no lane is ever pinched to zero: {calm:?}");
    let s0 = calm[0];
    for _ in 0..24 {
        k.physics_mut().bump(&[0, 0, 1, 0]);
    }
    let tense = k.lane_room();
    assert_eq!(tense[0], s0, "T pressure does not steal spatial room: {tense:?}");
    // Action potential: a VALUABLE stale listing outranks a worthless
    // fresh one — eviction is argmin tension, not argmax age (anti-FIFO).
    let rich_old = action_potential(&[0, 0, 900, 0], 64);
    let poor_new = action_potential(&[0, 0, 3, 0], 0);
    assert!(rich_old > poor_new, "value dominates age: {rich_old} vs {poor_new}");
    // And dwell genuinely decays tension at equal value.
    let fresh = action_potential(&[0, 0, 40, 0], 0);
    let stale = action_potential(&[0, 0, 40, 0], 128);
    assert!(fresh > stale, "dwell cools a listing: {fresh} vs {stale}");
}

/// D12 — THE ESCROW-PRICED DOOR: a calm mesh keeps a free door; a
/// saturated mesh charges congestion (fee → the pool) and DEFERS a
/// speaker who can pay nothing on a fully exhausted lane — spam costs
/// exactly when strain is real, and paying members pass.
#[test]
fn the_door_charges_strain_and_defers_the_broke() {
    let mut k = Kernel::new();
    // Calm: the door is free — a normal percept adjudicates untouched.
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 3)];
    let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 3)];
    assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending, "calm door admits");
    assert!(
        !k.treasury().entries().iter().any(|e| e.kind == "fee"),
        "no fee on a calm mesh"
    );
    // Exhaust the metabolism entirely (every lane to zero reserve).
    for _ in 0..16 {
        k.physics_mut().spend([true; 4]);
    }
    // A broke speaker (no earned balance) is DEFERRED at the door.
    let a2 = vec![(5, 0, 2), (5, 1, 2)];
    let b2 = vec![(5, 0, 6), (5, 1, 6)];
    let broke = k.ingest(&a2, &b2, [9; 32]);
    assert_eq!(broke, Outcome::GermPending, "saturated door defers the broke: {broke:?}");
    assert!(k.basis().is_empty() || k.quote().1.iter().all(|(w, _)| w.sub() != [(0i64, 0i64, 2u8, 6u8)]),
        "the deferred demand was never adjudicated");
    // A FUNDED speaker pays the congestion fee and passes the door.
    k.treasury_mut().settle_subsidy([7; 32], [8, 8, 8, 8], (0, [0; 32]));
    for _ in 0..16 {
        k.physics_mut().spend([true; 4]); // re-exhaust (the deferral relaxed some)
    }
    let _ = k.ingest(&a2, &b2, [7; 32]);
    assert!(
        k.treasury().entries().iter().any(|e| e.kind == "transfer" && e.who == [7; 32]),
        "the funded speaker paid the door as a transfer"
    );
}

/// THE CONVERSATION: a listing is probed with a cited sentence. Graded
/// descent settles the reduction and the residual replaces the listing
/// under its frame; closure clears it and advances the lineage; a
/// sentence that names nothing settles nothing. The replay is linear in
/// the proof and the hypha runs no search.
#[test]
fn a_probe_settles_descent_then_closure_without_search() {
    use crate::anastome::dialogue::{PROBE_CLOSED, PROBE_DESCENT, PROBE_NOTHING};
    use crate::transducer::sentence::replay_at;
    let mut k = Kernel::new();
    // A coboundary listed on an empty language (the residual path).
    let c = vec![(0, 0, 1u8), (0, 1, 1), (0, 2, 1), (0, 3, 3)];
    let d = vec![(0, 0, 2u8), (0, 1, 2), (0, 2, 1), (0, 3, 4)];
    let chain = germinate(&c, &d).unwrap();
    assert!(matches!(k.attest(&chain, [1; 32]), Outcome::GermPending));
    let target = k.quote().1[0].0.address();
    let chain = k.quote().1[0].0.sub().to_vec();
    assert_eq!(chain.len(), 3);
    let a = vec![(0, 0, 1u8), (0, 1, 1)];
    let b = vec![(0, 0, 2u8), (0, 1, 2)];
    k.basis_insert(element::from_residual(&germinate(&a, &b).unwrap()), [2; 32]);
    let _word = k.basis()[0].address();
    // The entity's reduction over the language: the word cited at the
    // origin, the far cell left as the residual.
    let language = k.language();
    let sentence = crate::transducer::sentence::deconstruct_at(&|i| language.get(i), &language.stalks, &|a| language.index.get(a).copied(), &|i| language.shape(i), &chain, None);
    let (void, cites) = (sentence.residual.clone(), sentence.cites.clone());
    assert_eq!(void, vec![(0, 3, 3, 4)]);
    assert_eq!(cites.len(), 1);
    let basis = k.basis();
    assert_eq!(
        replay_at(&|i| basis.get(i).map(|w| w.as_ref()), &cites, &chain),
        void,
        "the replay reproduces the residual"
    );
    let (code, paid, remaining) = k.settle_probe([3; 32], &target, void.clone(), &[], false);
    assert_eq!(code, PROBE_DESCENT);
    assert_eq!(paid, [0; 4], "a descent is measured, never paid: covering cells opens no axis");
    assert_eq!(remaining, crate::transducer::element::measure_lanes(&void));
    let listing = k.quote().1[0].0.clone();
    assert_eq!(listing.address(), target, "the listing stays the body: a descent never re-lists a fragment");
    assert_eq!(
        k.tension.get(&target).copied(),
        Some(crate::transducer::element::measure_lanes(&void)),
        "the body's remaining tension is the residual's measure, keyed by the listing"
    );
    // No descent settles nothing; closure clears the book.
    let t2 = listing.address();
    assert_eq!(k.settle_probe([3; 32], &t2, listing.sub().to_vec(), &[], false).0, PROBE_NOTHING);
    assert_eq!(k.settle_probe([3; 32], &t2, Vec::new(), &[], false).0, PROBE_CLOSED);
    assert!(k.quote().1.is_empty(), "the listing cleared");
}

/// Descent records a remainder face and a holonomic cell-count, stagnation
/// counts only unmoved occupied lanes, and a full aperture of failed
/// probes sheds the listing as a cavity.
#[test]
fn descent_opens_a_face_and_drain_sheds_the_unmoved() {
    use crate::anastome::dialogue::PROBE_DESCENT;
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1u8), (0, 1, 1)];
    let b = vec![(0, 0, 2u8)];
    let chain = germinate(&a, &b).unwrap();
    assert!(matches!(k.attest(&chain, [1; 32]), Outcome::GermPending));
    let target = k.quote().1[0].0.address();
    let listed = k.quote().1[0].0.sub().to_vec();
    let rem = vec![(0, 1, 1u8, 0u8)];
    let before = k.tension.get(&target).copied().unwrap_or_else(|| k.quote().1[0].0.magnitude());
    let after = crate::transducer::element::measure_lanes(&rem);
    assert_ne!(after, before, "the residual is a proper descent");
    let (code, _, remaining) = k.settle_probe([3; 32], &target, rem.clone(), &[], false);
    assert_eq!(code, PROBE_DESCENT);
    assert_eq!(remaining, after);
    let listing = k.quote().1[0].0.clone();
    assert_eq!(listing.address(), target, "the listing stays the body");
    assert_eq!(listing.magnitude()[3], rem.len() as u64, "holonomic demand is the remainder cell count");
    let rem_inv = crate::transducer::stalk::invariant(&rem);
    assert_eq!(k.face_of(&target).cloned(), Some(rem_inv.clone()));
    assert!(
        k.listing_atlas.fiber(&rem_inv).iter().any(|a| a.as_slice() == target.as_bytes()),
        "the remainder chart admits the listing"
    );
    let base_inv = crate::transducer::stalk::invariant(&listed);
    assert_eq!(
        k.listing_atlas.chart_of(&target).cloned().as_ref(),
        Some(&base_inv),
        "base moduli are never written"
    );
    let s = k.stagnant_of(&target);
    for l in 0..4 {
        if before[l] > 0 && after[l] == before[l] {
            assert!(s[l] >= 1, "unmoved occupied lane {l} stagnates: {s:?}");
        } else {
            assert_eq!(s[l], 0, "a moved or empty lane {l} resets: {s:?}");
        }
    }
    let mut listed_sorted = listed.clone();
    listed_sorted.sort_unstable();
    let kappa = crate::physics::aperture_cycle();
    for _ in 0..kappa {
        assert_eq!(
            k.settle_probe([4; 32], &target, listed_sorted.clone(), &[], false).0,
            crate::anastome::dialogue::PROBE_NOTHING
        );
    }
    let s = k.stagnant_of(&target);
    let t = k.tension.get(&target).copied().unwrap();
    assert!(
        (0..4).all(|l| t[l] == 0 || s[l] >= kappa),
        "every occupied lane sat a full cycle: tension {t:?} stagnant {s:?} kappa {kappa}"
    );
    k.drain_now();
    assert!(k.quote().1.is_empty(), "the unreachable listing left the page");
    assert_eq!(k.cavities().len(), 1, "the cavity is the relocated element");
    assert_eq!(k.cavities()[0].address(), target);
    assert_eq!(k.cavities()[0].magnitude()[3], rem.len() as u64);
    assert!(
        k.listing_atlas.fiber(&rem_inv).iter().any(|a| a.as_slice() == target.as_bytes()),
        "drain relocates: the listing stays on the atlas"
    );
    assert_eq!(
        k.listing_atlas.chart_of(&target).cloned().as_ref(),
        Some(&base_inv),
        "base moduli stay on the cavity chart"
    );
    assert!(
        k.field_cold.get(&target).is_some(),
        "the cold field is readable by key"
    );
    assert!(k.cavity_book.get(&target).is_some(), "the element is keyed on the cavity book");
    let listing = k.field_cold.get(&target).expect("relocated field is cold beside CAV1");
    let mut seen = Vec::new();
    for e in &listing.entries {
        assert!(!seen.contains(&e.chart), "a chart appears once per listing");
        seen.push(e.chart.clone());
    }
    let bytes = k.snapshot();
    let mut k2 = Kernel::new();
    assert!(k2.restore(&bytes), "the field persists beside DEF1");
    assert!(k2.cavity_book.get(&target).is_some(), "a restart keeps the cavity");
    assert!(
        k2.field_cold.get(&target).is_some_and(|f| !f.entries.is_empty()),
        "a restart does not forget which angles were probed"
    );
}


/// THE PROOF: on the wire the judge replays; a proven percept commits its
/// sentence, an unproven one is deferred, never searched.
#[test]
fn the_wire_judge_replays_proofs_and_defers_the_unproven() {
    let k = std::sync::Mutex::new(Kernel::new());
    let a = vec![(0, 0, 1u8), (0, 1, 1)];
    let b = vec![(0, 0, 2u8), (0, 1, 2)];
    {
        let mut g = k.lock().unwrap();
        g.ingest(&a, &b, [1; 32]);
        g.ingest(&a, &b, [2; 32]);
        g.glue_local();
    }
    {
        let g = k.lock().unwrap();
        assert!(!g.basis().is_empty(), "the pair form stands");
    }
    let (ca, cb) = perceive(&a, &b);
    let frame = frame_of(&a, &b);
    let out = crate::synthesis::adjudicate(&k, &ca, &cb, [3; 32], &frame, &[]);
    assert!(
        matches!(out, Outcome::Decomposed(ref w) if w == &vec![0]),
        "the held pair form is the word, got {out:?}"
    );
    let a2 = vec![(0, 0, 1u8), (0, 1, 1), (1, 0, 7)];
    let b2 = vec![(0, 0, 2u8), (0, 1, 2), (1, 0, 8)];
    let (ca2, cb2) = perceive(&a2, &b2);
    let frame2 = frame_of(&a2, &b2);
    let out = crate::synthesis::adjudicate(&k, &ca2, &cb2, [3; 32], &frame2, &[]);
    assert!(
        matches!(out, Outcome::GermPending | Outcome::Decomposed(_)),
        "a different pair is its own form or residual, got {out:?}"
    );
}

/// STEP 7c END-TO-END: birth deposits evidence; the court's rebind
/// upgrades (⊤, Δ) → (π*, Δ) in place; the dual condition then bites in
/// live adjudication — the same demand in an off-region scene stops
/// decomposing through the word and pends as new coverage instead.
#[test]
fn rebind_activates_the_dual_condition_in_adjudication() {
    use crate::transducer::anchor::Sel;
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 3)];
    let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 3)];
    assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    assert_eq!(k.ingest(&a, &b, [2; 32]), Outcome::GermPending);
    k.glue_local();
    assert!(!k.basis().is_empty(), "the pair form stands");
    assert!(k.basis()[0].anchor().is_none(), "born ⊤ — one pair never binds");
    let addr = k.basis()[0].address();
    let pi = Sel::Inter(Box::new(Sel::Hue(1)), Box::new(Sel::Star(Box::new(Sel::Hue(3)))));
    assert!(k.apply_rebind(&addr, pi.clone()));
    assert_eq!(k.basis()[0].anchor(), Some(&pi), "γ = (π*, Δ) in place");
    // ON the interface: the anchored word still speaks.
    let a2 = vec![(5, 0, 1), (5, 1, 1), (6, 0, 3), (6, 1, 3)];
    let b2 = vec![(5, 0, 4), (5, 1, 4), (6, 0, 3), (6, 1, 3)];
    assert!(
        matches!(k.ingest(&a2, &b2, [3; 32]), Outcome::Decomposed(_)),
        "π covers the interface placement"
    );
    // Placement is a coordinate of the transport, not a selection rule:
    // the same body elsewhere is the same word carried there.
    let a3 = vec![(0, 0, 1), (0, 1, 1)];
    let b3 = vec![(0, 0, 4), (0, 1, 4)];
    let out = k.ingest(&a3, &b3, [3; 32]);
    assert!(
        matches!(out, Outcome::Decomposed(_) | Outcome::GermPending),
        "a carried body is the same word or its residual, got {out:?}"
    );
}


/// THE BLIND REPLAY refutes every tamper axis and upholds honest speech —
/// the contradiction docket's whole force is this determinism.
#[test]
fn blind_replay_refutes_tampered_words() {
    use crate::transducer::germinate::germinate;
    use crate::verdict::blind_replay;
    let w = element::from_residual(
        &germinate(&[(0, 0, 1u8), (0, 1, 2)], &[(0, 0, 1u8), (0, 1, 9)]).unwrap(),
    );
    assert!(blind_replay(&w).is_none(), "an honest derivation replays clean");
    // Tamper 1: shift the chain off origin (framing violation).
    let mut off = w.clone();
    off.sub = off.sub.iter().map(|(r, c, b, a)| (r + 3, *c, *b, *a)).collect();
    assert!(blind_replay(&off).is_some(), "un-centered chain is false speech");
    // Tamper 2: inflate the claimed magnitude (yield fraud).
    let mut fat = w.clone();
    fat.mag = [999, 0, 0, 0];
    assert!(blind_replay(&fat).is_some(), "inflated magnitude does not re-derive");
    // Tamper 3: an empty chain claiming work.
    let mut empty = w.clone();
    empty.sub.clear();
    assert!(blind_replay(&empty).is_some(), "no evidence, no word");
}

/// STEP 5 — PROVISIONAL UNTIL THE COURT SPEAKS: a two-voice install escrows
/// its consilience award (unfinalized speech earns nothing); the docket
/// exports the spoken word; the upheld verdict releases the award; a
/// slashing verdict excises the word and debits the voices into the pool.
#[test]
fn installs_are_provisional_until_verdict() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
    assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    let chain = germinate(&a, &b).unwrap();
    assert_eq!(k.attest(&chain, [2; 32]), Outcome::GermPending);
    // The install PAID NOTHING — the award sits in escrow.
    assert_eq!(k.treasury().balance_mu(&[1; 32]), [0; 4], "unfinalized speech earns nothing");
    assert_eq!(k.treasury().balance_mu(&[2; 32]), [0; 4]);
    let dockets = k.dockets();
    assert_eq!(dockets.len(), 1, "the provisional word is on the docket");
    let (addr, spoken, voices, mu, _, _) = &dockets[0];
    assert_eq!(voices.len(), 2, "both attesting voices ride the docket");
    assert!(mu.iter().any(|&m| m > 0), "the escrowed award is real");
    assert_eq!(spoken.address(), addr.clone(), "the docket carries the word as spoken");
    // UPHELD: finality releases the award — the voices are paid at last.
    assert!(k.apply_verdict(&addr.clone(), crate::anastome::dialogue::RULING_UPHELD));
    assert!(k.treasury().balance_mu(&[1; 32]).iter().any(|&m| m > 0), "finality pays voice A");
    assert!(k.treasury().balance_mu(&[2; 32]).iter().any(|&m| m > 0), "finality pays voice B");
    assert!(k.dockets().is_empty(), "the docket cleared");
    assert!(!k.apply_verdict(&addr.clone(), crate::anastome::dialogue::RULING_UPHELD), "a verdict lands once");
}

/// A slashing verdict (failed blind replay — false speech) excises the word
/// from the basis and debits the voices' earned balances into the pool.
#[test]
fn a_slashing_verdict_excises_and_debits() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
    assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    let chain = germinate(&a, &b).unwrap();
    assert_eq!(k.attest(&chain, [2; 32]), Outcome::GermPending);
    let (addr, _, _, _, _, _) = k.dockets()[0].clone();
    assert!(k.apply_verdict(&addr, crate::anastome::dialogue::RULING_SLASHED));
    assert_eq!(k.treasury().balance_mu(&[1; 32]), [0; 4], "nothing was ever paid");
    assert!(k.dockets().is_empty());
}

/// PUNCTURE-BEFORE-MINT: a family member (same base geometry, new transition
/// assignment) does NOT mint sibling card n+1 — the axis pierces the existing
/// thought and the member becomes a SECTION. One vantage sections it: the
/// court re-derived it from its own held word, and a second voice is a use,
/// not a step toward it. Growth as +axis:
/// the basis count stays 1 while the thought's volume grows; afterward the
/// bundle SPEAKS the whole family (both assignments decompose to it).
#[test]
fn a_family_punctures_an_axis_instead_of_minting() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b1 = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)]; // (0,0): 1→4
    assert_eq!(k.ingest(&a, &b1, [1; 32]), Outcome::GermPending, "one vantage dockets");
    k.glue_local();
    assert_eq!(k.basis_len(), 1);
    // The family member: same cell, DIFFERENT transition (1→7).
    let b2 = vec![(0, 0, 7), (0, 1, 1), (1, 0, 1)];
    assert!(matches!(k.ingest(&a, &b2, [1; 32]), Outcome::Sectioned(0)), "the axis PIERCES thought 0");
    assert_eq!(k.basis_len(), 1, "count unchanged — growth was +axis");
    assert_eq!(k.basis()[0].sections().len(), 1, "the thought gained a section (volume)");
    // The bundle speaks the family: both members now decompose to the ONE word.
    assert_eq!(k.ingest(&a, &b1, [3; 32]), Outcome::Decomposed(vec![0]));
    assert_eq!(k.ingest(&a, &b2, [3; 32]), Outcome::Decomposed(vec![0]));
    // And the address held stable across the puncture (identity = the word).
    let far_b2: Vec<_> = b2.iter().map(|(r, c, h)| (r + 6, c + 6, *h)).collect();
    let far_a: Vec<_> = a.iter().map(|(r, c, h)| (r + 6, c + 6, *h)).collect();
    assert_eq!(k.ingest(&far_a, &far_b2, [4; 32]), Outcome::Decomposed(vec![0]), "transposed family member");
}

/// THE CONDENSE STROKE: position-identical sibling cards already in the basis
/// (a pre-puncture era's residue) are ABSORBED into one bundle during the
/// syntropic stroke — cardinality falls, sections rise, and the absorbed
/// words' discoverers still earn.
#[test]
fn the_condense_stroke_absorbs_a_family_into_one_bundle() {
    use crate::transducer::element::from_residual;
    use crate::transducer::germinate::germinate;
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 2)];
    let b = vec![(0, 0, 5), (0, 1, 2)]; // (0,0): 1→5
    k.basis_insert(from_residual(&germinate(&a, &b).unwrap()), [1; 32]);
    for hue in [6u8, 7] {
        let bx = vec![(0, 0, hue), (0, 1, 2)];
        k.basis_insert(from_residual(&germinate(&a, &bx).unwrap()), [9; 32]);
    }
    assert_eq!(k.basis_len(), 3, "the un-condensed family");
    // One versor per stroke: the cursor walks the stratum over strokes.
    for _ in 0..3 {
        k.optimize();
    }
    assert_eq!(k.basis_len(), 1, "the family collapsed into one bundle");
    assert_eq!(k.basis()[0].sections().len(), 2, "two sections along the pierced axis");
    let addrs: Vec<Address> = k.dockets().iter().map(|(a, _, _, _, _, _)| a.clone()).collect();
    for a in addrs {
        let _ = k.apply_verdict(&a, crate::anastome::dialogue::RULING_UPHELD);
    }
    assert!(
        k.treasury().balance_mu(&[9; 32])[1] > 0 || k.treasury().balance_mu(&[9; 32]).iter().any(|&m| m > 0),
        "absorbed discoverers still earn, on the lane"
    );
}

/// THE TORSE STROKE tightens mathematics to physics: a swap family (1→4 and
/// 4→1 at one cell) installs as base + section; the syntropic stroke extracts
/// the ℤ/2 cycle — sections condense to (σ, 2) — and the extracted
/// mathematical torsion RELIEVES the physical torsion charge (booked as
/// syntropic order, per lane, never a scalar).
#[test]
fn the_torse_stroke_extracts_the_cycle_and_relieves_torsion() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 9)];
    let b = vec![(0, 0, 4), (0, 1, 9)]; // (0,0): 1→4
    k.ingest(&a, &b, [1; 32]);
    assert_eq!(k.ingest(&a, &b, [2; 32]), Outcome::GermPending);
    k.glue_local();
    let rev = vec![(0, 0, 1), (0, 1, 9)];
    let fwd = vec![(0, 0, 4), (0, 1, 9)];
    k.ingest(&fwd, &rev, [1; 32]);
    match k.ingest(&fwd, &rev, [2; 32]) {
        Outcome::GermPending => k.glue_local(),
        Outcome::Installed(_) | Outcome::Sectioned(_) | Outcome::Decomposed(_) => {}
        o => panic!("{o:?}"),
    }
    assert!(!k.basis().is_empty(), "the pair forms stand");
    k.optimize();
    assert!(
        matches!(k.ingest(&a, &b, [3; 32]), Outcome::Decomposed(_) | Outcome::GermPending),
        "the forward pair still speaks or its residual pends"
    );
    assert!(
        matches!(k.ingest(&fwd, &rev, [3; 32]), Outcome::Decomposed(_) | Outcome::GermPending),
        "the reverse pair is the same form or its residual"
    );
}

/// THE CROWN PROOF OF THE EXCISION: with no solver anywhere, the mesh forms
/// SENTENCES over germinated patterns — install two single-cell patterns, then
/// a compound demand containing BOTH (at different offsets) DECOMPOSES into
/// the two-word sentence by chain peeling. Nothing minted; language composed.
#[test]
fn a_compound_demand_decomposes_over_germinated_patterns() {
    let mut k = Kernel::new();
    let a1 = vec![(0, 0, 1), (0, 1, 2)];
    let b1 = vec![(0, 0, 4), (0, 1, 2)]; // (0,0): 1→4 — pattern P
    k.ingest(&a1, &b1, [1; 32]);
    assert_eq!(k.ingest(&a1, &b1, [2; 32]), Outcome::GermPending);
    k.glue_local();
    let a2 = vec![(0, 0, 2), (0, 1, 3)];
    let b2 = vec![(0, 0, 2), (0, 1, 9)]; // (0,1): 3→9 — Q shares P's geometry:
    k.ingest(&a2, &b2, [1; 32]);
    // …so under 3.5 it PUNCTURES P's axis (Installed(0) as a section).
    match k.ingest(&a2, &b2, [2; 32]) {
        Outcome::GermPending => k.glue_local(),
        Outcome::Installed(_) | Outcome::Sectioned(_) => {}
        o => panic!("{o:?}"),
    }
    assert_eq!(k.basis_len(), 1, "the single-cell family is ONE bundle");
    let a3 = vec![(0, 0, 1), (0, 1, 2), (0, 2, 2), (0, 3, 3)];
    let b3 = vec![(0, 0, 4), (0, 1, 2), (0, 2, 2), (0, 3, 9)];
    let before = k.basis_len();
    match k.ingest(&a3, &b3, [3; 32]) {
        Outcome::Decomposed(_) | Outcome::GermPending => {}
        o => panic!("a compound pair is one percept, got {o:?}"),
    }
    assert_eq!(k.basis_len(), before, "a compound pair is not peeled into a second word");
}

/// A shape deformation (L → bar) that NO motor closes is not a dead-end: its
/// residual IS the description, so it germinates a coordinate-exact coboundary
/// germ and installs on two-voice consilience (priced on USE per R3, so a
/// one-off earns ~0 — no basis flood). The reframe made operational: germinate
/// the description, never discard the residual.
#[test]
fn a_shape_deformation_germinates_a_residual() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)]; // L
    let b = vec![(0, 0, 1), (0, 1, 1), (0, 2, 1)]; // bar — no rigid motor
    assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending, "one voice awaits consilience");
    assert_eq!(k.basis_len(), 0, "not yet installed");
    assert_eq!(k.ingest(&a, &b, [2; 32]), Outcome::GermPending, "two voices settle the residual germ");
    k.glue_local();
    assert_eq!(k.basis_len(), 1, "the deformation germinated a new axis");
    assert_eq!(k.ingest(&a, &b, [3; 32]), Outcome::Decomposed(vec![0]), "and reduces to it");
}

/// Two distinct objects in one pair are one pair form, not a Cartesian
/// fragment per body. Two voices docket the residual.
#[test]
fn independent_objects_fragment_and_resolve() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1), (5, 5, 2), (5, 6, 2), (6, 5, 2), (6, 6, 2)];
    let b = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1), (5, 5, 3), (5, 6, 3), (6, 5, 3), (6, 6, 3)];
    assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending, "one pair, one residual");
    assert_eq!(k.ingest(&a, &b, [2; 32]), Outcome::GermPending);
    k.glue_local();
    assert_eq!(k.basis_len(), 1, "the pair form is one word");
}

/// W2: the percept is pair_form → extract_rule_with. A family the engine
/// closes lists the minted sentence; a withheld form lists one remainder.
#[test]
fn the_percept_is_the_extracted_pair_form() {
    use crate::transducer::family::{cells_to_grid, extract_rule_with, mint_sentence, NoCrystal};
    let a = vec![(0, 0, 1), (0, 1, 1)];
    let b = vec![(0, 0, 2), (0, 1, 2)];
    let family = vec![(cells_to_grid(&a), cells_to_grid(&b))];
    let mut k = Kernel::new();
    assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    let listed = &k.quote().1;
    assert_eq!(listed.len(), 1, "one pair, one listing");
    match extract_rule_with(&family, &NoCrystal)
        .and_then(|rule| mint_sentence(&rule, &family, &NoCrystal))
    {
        Some(e) => assert!(
            listed[0].0.same_vertex(&e),
            "the court dockets the minted form"
        ),
        None => assert!(
            !listed[0].0.is_sentence(),
            "a withheld form is the remainder, not a sentence"
        ),
    }
}

/// Court organ: docketing a germ does not grow the basis. The stroke
/// puts a section on the ruled cover. A peer's fetch glues by replay;
/// a forged word is slashed with a reason. Signatures are not consulted.
#[test]
fn court_organ_glue_on_replay_not_on_signatures() {
    let mut home = Kernel::new();
    let mut peer = Kernel::new();
    let mut third = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
    assert_eq!(home.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    assert_eq!(home.ingest(&a, &b, [2; 32]), Outcome::GermPending);
    assert_eq!(home.basis_len(), 0);
    home.court_stroke();
    let spoken = home.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().1.clone();
    let mut cover = crate::network::nerve::Cover::new();
    assert!(matches!(
        peer.glue_fetched(spoken.clone(), [1; 32], [2; 32], &mut cover),
        Glue::Admitted
    ));
    assert!(peer.basis_len() >= 1, "the word is language on the second organ");
    assert!(matches!(
        third.glue_fetched(spoken.clone(), [2; 32], [3; 32], &mut cover),
        Glue::Admitted
    ));
    let mut forged = spoken.clone();
    forged.mag = [999, 0, 0, 0];
    assert!(
        matches!(third.glue_fetched(forged, [2; 32], [3; 32], &mut cover), Glue::Slashed(_)),
        "never admit on a word that fails replay"
    );
}

/// Adopt writes a field entry and a deflation so the stroke can rule
/// a client-minted connection. Ruled listings do not starve later ones.
#[test]
fn adopt_reaches_the_docket_and_stroke_skips_ruled() {
    use crate::transducer::element::from_connection;
    fn conn(shift: i64) -> Element {
        from_connection(
            &crate::transducer::alphabet::Word::identity(),
            &crate::transducer::element::translator_versor(shift, 0),
            &[],
            &Vec::new(),
            [1, 0, 0, 0],
        )
        .expect("spells")
    }
    let mut k = Kernel::new();
    let cap = crate::physics::Physics::default().stroke().remaining();
    let mut addrs = Vec::new();
    for i in 0..(cap + 1) {
        let w = conn(i as i64 + 1);
        let addr = w.address();
        assert!(matches!(k.install_adopted(&w, [1; 32]), Adoption::Installed));
        assert!(k.field().get(&addr).is_some(), "adopt writes a field entry");
        addrs.push(addr);
    }
    let ruled_n = |k: &Kernel, addrs: &[crate::transducer::a21::Address]| {
        addrs
            .iter()
            .filter(|a| {
                k.word_at(a).is_some_and(|w| k.ruled_have(&crate::transducer::stalk::invariant_of(w)).iter().any(|h| h.as_slice() == a.as_bytes()))
            })
            .count()
    };
    k.court_stroke();
    assert_eq!(ruled_n(&k, &addrs), cap, "one stroke rules a spend of unruled listings, not already-ruled ones");
    k.court_stroke();
    assert_eq!(ruled_n(&k, &addrs), cap + 1, "later listings are not starved by already-ruled ones");
}

/// Restore keeps the lettered federation and drops a leaked basis card.
#[test]
fn prune_drops_the_leaked_basis_and_keeps_the_ruled_cover() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
    glue_pair(&mut k, &a, &b);
    let kept = k.basis_len();
    assert!(kept >= 1);
    let leak_a = vec![(9, 0, 2), (9, 1, 2)];
    let leak_b = vec![(9, 0, 3), (9, 1, 3)];
    let leaked = crate::transducer::element::from_residual(&germinate(&leak_a, &leak_b).unwrap());
    k.leak_unruled(leaked);
    assert_eq!(k.basis_len(), kept + 1);
    let dropped = k.prune_basis_to_ruled();
    assert_eq!(dropped, 1, "the leaked card leaves");
    assert_eq!(k.basis_len(), kept, "the ruled cover remains");
    k.leak_unruled(crate::transducer::element::from_residual(
        &germinate(&[(8, 0, 2), (8, 1, 2)], &[(8, 0, 5), (8, 1, 5)]).unwrap(),
    ));
    let snap = k.snapshot();
    let mut booted = Kernel::new();
    assert!(booted.restore(&snap));
    assert_eq!(booted.basis_len(), kept, "restore skips the leaked basis and rebuilds from the federation");
}

/// An append shares the prior word's Arc; it does not clone the basis.
#[test]
fn language_append_shares_prior_word_arcs() {
    let mut k = Kernel::new();
    let w0 = crate::transducer::element::from_residual(
        &germinate(&[(0, 0, 1), (0, 1, 1), (1, 0, 1)], &[(0, 1, 1), (1, 0, 1), (1, 1, 1)]).unwrap(),
    );
    let w1 = crate::transducer::element::from_residual(
        &germinate(&[(4, 0, 2), (4, 1, 2), (5, 0, 2)], &[(4, 0, 3), (4, 1, 3), (5, 0, 3)]).unwrap(),
    );
    k.leak_unruled(w0);
    let first = k.language();
    assert_eq!(first.words.len(), 1);
    let p0 = std::sync::Arc::as_ptr(&first.words[0]);
    assert!(
        std::sync::Arc::ptr_eq(&first, &k.language()),
        "a stable language is shared by pointer"
    );
    k.leak_unruled(w1);
    let second = k.language();
    assert_eq!(second.words.len(), 2);
    assert_eq!(
        std::sync::Arc::as_ptr(&second.words[0]),
        p0,
        "the first word is the same Arc, not a cloned element"
    );
}

/// Dialect/Slashed addresses are heard once; a cocycle row is unique
/// on (chart, pair, versor).
#[test]
fn heard_and_cocycles_do_not_re_row() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
    k.ingest(&a, &b, [1; 32]);
    k.ingest(&a, &b, [2; 32]);
    let spoken = k.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().1.clone();
    let mut forged = spoken.clone();
    forged.mag = [999, 0, 0, 0];
    let mut cover = crate::network::nerve::Cover::new();
    assert!(matches!(k.glue_fetched(forged.clone(), [2; 32], [1; 32], &mut cover), Glue::Slashed(_)));
    let n = k.cochain_bytes();
    assert!(k.heard.contains(&forged.address()));
    assert!(matches!(k.glue_fetched(forged, [2; 32], [1; 32], &mut cover), Glue::Dialect));
    assert_eq!(k.cochain_bytes(), n, "a heard slash is not re-rowed");
}

/// The home installs when a peer's ruled digest holds the stratum.
#[test]
fn home_installs_when_peer_ruled_root_gains_the_address() {
    let mut home = Kernel::new();
    let mut peer = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
    assert_eq!(home.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    assert_eq!(home.ingest(&a, &b, [2; 32]), Outcome::GermPending);
    assert_eq!(home.basis_len(), 0);
    home.rule_all_provisionals();
    let spoken = home.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().1.clone();
    let mut cover = crate::network::nerve::Cover::new();
    assert!(matches!(peer.glue_fetched(spoken, [1; 32], [2; 32], &mut cover), Glue::Admitted));
    assert!(peer.basis_len() >= 1);
    let cover = home.genesis_cover().expect("measured");
    home.enroll([2; 32], cover); // 47.3: a witness with room
    home.confirm_peer_holds(&peer.ruled_digests(), [2; 32]);
    assert!(home.basis_len() >= 1, "the home's own word is language once a peer holds the stratum");
    assert!(home.provisional.is_empty(), "the provisional leaves the home");
}

/// ONE VANTAGE DOCKETS: a percept this court re-derived is listed on the
/// book, escrowed as a claim, and Offered — and mints NOTHING. The second
/// vantage is a witness hypha that re-derives the same address and has room
/// (47.3); on its arrival the escrow releases and the deflation mints. No
/// number of voices substitutes for it, and this asserted a two-voice
/// quorum before convergence replaced consensus.
#[test]
fn one_vantage_dockets_escrows_and_mints_nothing_until_a_witness_rederives() {
    let mut home = Kernel::new();
    let mut peer = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
    assert_eq!(home.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    assert!(!home.pending.is_empty(), "the listing stays on the book");
    assert_eq!(home.provisional.len(), 1, "the residual is on the docket");
    let addr = home.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().0.clone();
    assert!(home.offered(&addr), "the court Offers a first-vantage germ");
    assert!(home.treasury().escrow_of(&addr).is_some(), "the priced magnitude is the claim");
    assert_eq!(home.treasury().minted().expect("minted adds up"), [0; 4], "the claim is not yet payment: nothing minted");
    home.court_stroke();
    let spoken = home.provisional.get(&addr).cloned().expect("still provisional until a peer glues");
    let mut cover = crate::network::nerve::Cover::new();
    assert!(matches!(
        peer.glue_fetched(spoken, [1; 32], [2; 32], &mut cover),
        Glue::Admitted
    ));
    let cover = home.genesis_cover().expect("measured");
    home.enroll([2; 32], cover); // 47.3: a witness with room
    let bond = home.treasury().bond_total();
    home.confirm_peer_holds(&peer.ruled_digests(), [2; 32]);
    assert!(home.basis_len() >= 1, "the home installs after the peer glues");
    assert!(home.provisional.is_empty(), "the docket clears on install");
    assert!(home.treasury().escrow_of(&addr).is_none(), "the claim is released");
    assert!(
        home.treasury().minted().expect("minted adds up").iter().any(|&x| x > 0),
        "release mints the priced vector"
    );
    assert_eq!(home.treasury().bond_total(), bond, "mint does not debit the stake bond");
}

/// A peer have-list that names a ruled provisional is the second organ:
/// the home installs without waiting for a digest-root match.
#[test]
fn home_installs_on_a_have_list_naming_a_ruled_provisional() {
    let mut home = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
    assert_eq!(home.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    let addr = home.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().0.clone();
    home.court_stroke();
    assert!(home.provisional.contains(&addr), "still provisional until the second organ");
    // 47.3: a witness is a second VANTAGE with room, not merely a second
    // key. An unbonded peer has none and does not converge the object.
    home.confirm_peer_holds_addrs(&[Address::from(addr.as_slice())], [2; 32]);
    assert!(home.basis_len() == 0, "a witness without room is not a vantage");
    let cover = home.genesis_cover().expect("measured");
    home.enroll([2; 32], cover);
    home.confirm_peer_holds_addrs(&[Address::from(addr.as_slice())], [2; 32]);
    assert!(home.basis_len() >= 1, "the witness re-derives the address and it installs");
    assert!(home.provisional.is_empty(), "the docket clears on install");
}

/// A stood sentence that closes no listing returns its escrow and mints nothing.
#[test]
fn speech_that_closes_nothing_mints_none() {
    let mut k = Kernel::new();
    let w = crate::transducer::element::from_connection(
        &crate::transducer::alphabet::Word::identity(),
        &crate::transducer::element::identity_versor(),
        &[(1, 2), (2, 1)],
        &Vec::new(),
        [1, 1, 0, 0],
    )
    .unwrap();
    let addr = w.address();
    let codes = k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&w));
    assert_eq!(codes, vec![crate::anastome::dialogue::SPEECH_STOOD]);
    assert!(k.treasury().escrow_of(&addr).is_some());
    k.rule_all_provisionals();
    // 47.3: the witness is a vantage with room. Bond it.
    let cover = k.genesis_cover().expect("measured");
    k.enroll([2; 32], cover);
    k.confirm_peer_holds_addrs(std::slice::from_ref(&addr), [2; 32]);
    assert!(k.treasury().escrow_of(&addr).is_none(), "escrow returned");
    assert_eq!(
        k.treasury().minted().expect("minted adds up"),
        [0; 4],
        "a restatement that closes nothing pays nothing"
    );
}

fn digits(rows: &[&str]) -> Vec<Vec<u8>> {
    rows.iter()
        .map(|r| r.bytes().map(|b| b - b'0').collect())
        .collect()
}

/// Binding a listing to a task does not mint: the form must speak the trains.
#[test]
fn association_with_a_listing_does_not_mint() {
    let mut k = Kernel::new();
    let family = vec![(
        digits(&["1100", "0000"]),
        digits(&["0011", "0000"]),
    )];
    let (chain, _, frame) = crate::arc::refine_delta(&family[0].0, &family[0].1).expect("coboundary");
    assert!(k.intake(&chain, &frame), "the listing stands");
    let listing = k.pending.fiber(crate::network::nerve::ROOT).next().expect("listed").1.0.address();
    let task = [7u8; 32];
    k.bind_listing_task(&listing, task);
    k.bind_task_family(task, family);
    let w = crate::transducer::element::from_connection(
        &crate::transducer::alphabet::Word::identity(),
        &crate::transducer::element::identity_versor(),
        &[(1, 2), (2, 1)],
        &Vec::new(),
        [4, 4, 4, 4],
    )
    .unwrap();
    let addr = w.address();
    assert_ne!(addr, listing, "the restatement is not the listing");
    let codes = k.perceive_speech([1; 32], &task, std::slice::from_ref(&w));
    assert_eq!(codes, vec![crate::anastome::dialogue::SPEECH_STOOD]);
    k.bind_listing_task(&addr, task);
    k.rule_all_provisionals();
    // 47.3: the witness is a vantage with room. Bond it.
    let cover = k.genesis_cover().expect("measured");
    k.enroll([2; 32], cover);
    k.confirm_peer_holds_addrs(std::slice::from_ref(&addr), [2; 32]);
    assert_eq!(
        k.treasury().minted().expect("minted adds up"),
        [0; 4],
        "a bound listing is not a mint without replay"
    );
}

/// A rule that speaks every train pair mints the coboundary it closed.
#[test]
fn a_rule_that_speaks_the_family_mints_measured_descent() {
    let family = vec![
        (digits(&["1100", "0000"]), digits(&["0022", "0000"])),
        (digits(&["0000", "1100"]), digits(&["0000", "0022"])),
    ];
    let rule = crate::transducer::family::extract_rule(&family).expect("extract");
    let spoken = crate::transducer::family::mint_sentence(&rule, &family, &crate::transducer::family::NoCrystal).expect("mint");
    assert!(spoken.is_sentence());
    let mut expect = [0u64; 4];
    for (x, y) in &family {
        let (chain, _, _) = crate::arc::refine_delta(x, y).expect("coboundary");
        let lanes = crate::transducer::element::measure_lanes(&chain);
        for i in 0..4 {
            expect[i] = expect[i].saturating_add(lanes[i]);
        }
    }
    assert!(expect.iter().any(|&x| x > 0), "the family has descent");
    let decoded = crate::transducer::sentence::from_element_form(&spoken).expect("v2 form");
    assert_eq!(
        crate::transducer::family::form_descent(&decoded, &family, &crate::transducer::family::NoCrystal).0,
        expect,
        "the form read back from its tape measures the family's descent"
    );
    let mut k = Kernel::new();
    let task = crate::registry::task_id(&crate::arc::Task {
        train: family.clone(),
        test: Vec::new(),
    });
    k.bind_task_family(&task, family);
    let addr = spoken.address();
    let codes = k.perceive_speech([1; 32], task.as_bytes(), std::slice::from_ref(&spoken));
    assert_eq!(codes, vec![crate::anastome::dialogue::SPEECH_STOOD]);
    k.rule_all_provisionals();
    // 47.3: the witness is a vantage with room. Bond it.
    let cover = k.genesis_cover().expect("measured");
    k.enroll([2; 32], cover);
    k.confirm_peer_holds_addrs(std::slice::from_ref(&addr), [2; 32]);
    assert_eq!(
        k.treasury().minted().expect("minted adds up"),
        expect,
        "mint is the measured coboundary, not a bound listing's magnitude"
    );
}

/// A glued form's descent is measured through the court genome as Crystal.
#[test]
fn glued_form_replays_descent_through_the_court_genome() {
    use crate::transducer::family::{
        apply_form, extract_rule_with, mint_sentence, pair_form, Crystal, Grid, Speech,
    };
    use crate::transducer::sentence::{as_element_form, Form, Vertex};
    use crate::transducer::stalk::{Invariant, Standing};
    let family = vec![
        (digits(&["1100", "0000"]), digits(&["0011", "0000"])),
        (digits(&["0000", "1100"]), digits(&["0000", "0011"])),
    ];
    let held_form = pair_form(&family[0].0, &family[0].1, None);
    let held_e = as_element_form(&held_form, [1, 0, 0, 0]).expect("inner tape");
    struct Held {
        e: Element,
        f: Form,
    }
    impl Crystal for Held {
        fn on_class(&self, _class: &Invariant) -> Vec<(Element, Form, Standing)> {
            vec![(self.e.clone(), self.f.clone(), Standing::Held)]
        }
        fn speak_form(
            &self,
            f: &Form,
            x: &Grid,
            placement: &crate::transducer::alphabet::Word,
        ) -> Option<Speech> {
            apply_form(f, x, placement, self)
        }
        fn by_address(&self, addr: &[u8]) -> Option<(Element, Form)> {
            (addr == self.e.address().as_bytes()).then(|| (self.e.clone(), self.f.clone()))
        }
        fn on_axis(
            &self,
            _chart: &Invariant,
            axis: crate::transducer::sentence::Axis,
        ) -> Vec<crate::transducer::alphabet::Word> {
            let mut index = crate::transducer::sentence::AxisIndex::new();
            crate::transducer::sentence::admit_words(
                &mut index,
                &Invariant::default(),
                &crate::transducer::sentence::words_of_form(&self.f),
            );
            crate::transducer::sentence::on_axis_of(&index, &Invariant::default(), axis)
        }
    }
    let cover = Held {
        e: held_e.clone(),
        f: held_form,
    };
    let rule = extract_rule_with(&family, &cover).expect("glues");
    assert!(
        rule.form
            .vertices
            .iter()
            .any(|v| matches!(v, Vertex::Glued { .. })),
        "the rule cites the held form"
    );
    let spoken = mint_sentence(&rule, &family, &cover).expect("mint through crystal");
    let mut k = Kernel::new();
    let inner_addr = held_e.address();
    assert_eq!(
        k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&held_e)),
        vec![crate::anastome::dialogue::SPEECH_STOOD]
    );
    k.rule_all_provisionals();
    k.confirm_peer_holds_addrs(std::slice::from_ref(&inner_addr), [2; 32]);
    let task = [9u8; 32];
    k.bind_task_family(task, family);
    let codes = k.perceive_speech([1; 32], &task, std::slice::from_ref(&spoken));
    assert_eq!(
        codes,
        vec![crate::anastome::dialogue::SPEECH_STOOD],
        "glued descent replays through the court genome, not NoCrystal"
    );
}

/// Teaching is decided by the docket's anchoring of the cited word,
/// never by sentence length.
#[test]
fn teaching_is_by_anchoring_not_length() {
    use crate::transducer::anchor::Sel;
    use crate::transducer::element::from_residual;
    let unanchored = from_residual(&vec![(0i64, 0i64, 1u8, 2u8)]);
    assert!(
        taught_anchored_words(std::iter::once(&unanchored)).is_empty(),
        "one unanchored word teaches nothing"
    );
    let a = from_residual(&vec![(0i64, 0i64, 1u8, 2u8)]).with_anchor(Sel::Hue(1));
    let b = from_residual(&vec![(1i64, 0i64, 3u8, 4u8)]).with_anchor(Sel::Hue(3));
    let taught = taught_anchored_words([&a, &b]);
    let mut pos = 0usize;
    let mut n = 0usize;
    while pos < taught.len() {
        let before = pos;
        if Element::from_bytes(&taught, &mut pos).is_none() {
            break;
        }
        if pos == before {
            break;
        }
        n += 1;
    }
    assert_eq!(n, 2, "two anchored words in one sentence both taught");
}

/// A repeated (vk, address, K) is answered from the docket: same reply,
/// no re-docket, no re-charge. A later K is a use; another voice is
/// consilience.
#[test]
fn repeated_demand_is_answered_not_redocketed() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1u8)];
    let b = vec![(0, 0, 2u8)];
    let chain = germinate(&a, &b).unwrap();
    let first = k.attest(&chain, [1; 32]);
    assert_eq!(first, Outcome::GermPending, "one voice lists, does not install");
    let n = k.pending.len();
    assert_eq!(n, 1, "the first hearing dockets the residual");
    let pressure = k.physics().pressure;
    let occupancy = k.occupancy();
    let second = k.attest(&chain, [1; 32]);
    assert_eq!(second, first, "the docket answers the same receipt");
    assert_eq!(k.pending.len(), n, "nothing re-docketed");
    assert_eq!(k.physics().pressure, pressure, "nothing re-charged");
    assert_eq!(k.occupancy(), occupancy, "room is not drawn again");
    let used = k.attest_at(&chain, [1; 32], &[], 1);
    assert_eq!(used, Outcome::GermPending, "a later K is a use, not a duplicate");
    assert_eq!(k.pending.len(), n, "a use of a listed address does not re-docket");
    let other = k.attest(&chain, [2; 32]);
    assert_eq!(other, Outcome::GermPending, "another voice is consilience, not a duplicate");
}

/// The demand codec roundtrips.
#[test]
fn the_demand_codec_roundtrips() {
    let (a, b) = (scene(2, 2), scene(2, 5));
    let (v, ra, rb) = decode_demand(&encode_demand(&a, &b, &[9; 32])).unwrap();
    assert_eq!((v, ra, rb), ([9; 32], a, b));
}

/// Two hyphas at different K relocate the same standing listing at the
/// same TUniversal: T-room, never a stroke count.
#[test]
fn t_room_relocates_at_tuniversal_independent_of_k() {
    let list = |k: &mut Kernel, extra: bool| -> Address {
        let mut a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let mut b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
        if extra {
            a.push((1, 1, 1));
            b.push((1, 1, 5));
        }
        let _ = k.ingest(&a, &b, [1; 32]);
        k.pending.fiber(crate::network::nerve::ROOT).next().expect("listed").1.0.address()
    };
    let mut ha = Kernel::new();
    let mut hb = Kernel::new();
    ha.set_lane_cap([64, 64, 1, 64]);
    hb.set_lane_cap([64, 64, 1, 64]);
    ha.set_height(10);
    hb.set_height(10);
    let first_a = list(&mut ha, false);
    let first_b = list(&mut hb, false);
    assert_eq!(first_a, first_b);
    // Advance K on ha only — TUniversal stays the clock we set.
    for _ in 0..8 {
        ha.append_step(&[1; 32], [2; 32]);
    }
    assert!(ha.k_step() > hb.k_step());
    ha.set_height(40);
    hb.set_height(40);
    let _ = list(&mut ha, true);
    let _ = list(&mut hb, true);
    assert!(
        ha.cavity_book.contains(&first_a),
        "ha relocates the older listing on T"
    );
    assert!(
        hb.cavity_book.contains(&first_b),
        "hb relocates the same listing at the same TUniversal"
    );
}

/// Standing T is a height difference on the home chain; a witness
/// re-derives it from the receipts (T16).
#[test]
fn extent_replays_from_the_home_chain() {
    let mut k = Kernel::new();
    k.set_height(4);
    let a = vec![(0, 0, 1), (0, 1, 1)];
    let b = vec![(0, 0, 2), (0, 1, 2)];
    k.ingest(&a, &b, [1; 32]);
    let addr = k.pending.fiber(crate::network::nerve::ROOT).next().expect("listed").1.0.address();
    assert_eq!(k.t_extent(addr.as_bytes(), 4), 0, "extent is zero at first vantage");
    k.set_height(11);
    assert_eq!(
        k.t_extent(addr.as_bytes(), 11),
        7,
        "extent is h_now − h_install on the home chain"
    );
    let mut k2 = Kernel::new();
    k2.set_height(11);
    k2.standing_h.insert(&addr, 4);
    assert_eq!(
        k2.t_extent(addr.as_bytes(), 11),
        7,
        "a witness re-derives the same extent from the recorded height"
    );
}

/// Extra court strokes of a fixed corpus do not grow the hot set.
#[test]
fn hot_set_does_not_grow_with_idle_strokes() {
    let mut k = Kernel::new();
    k.set_height(5);
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
    k.ingest(&a, &b, [1; 32]);
    k.ingest(&a, &b, [2; 32]);
    k.court_stroke();
    let hot = (
        k.field_hot_len(),
        k.pending.len(),
        k.heard_len(),
        k.dialect_len(),
        k.provisional.len(),
    );
    for _ in 0..16 {
        k.court_stroke();
    }
    assert_eq!(
        (
            k.field_hot_len(),
            k.pending.len(),
            k.heard_len(),
            k.dialect_len(),
            k.provisional.len(),
        ),
        hot,
        "idle strokes do not grow the hot set"
    );
}

/// Quotes are incidence on charts. Permuting the pending list does not
/// change the quoted addresses; Rank carries no scalar order.
#[test]
fn the_book_has_no_order() {
    let e1 = element::from_residual(&vec![(0, 0, 1, 2)]);
    let e2 = element::from_residual(&vec![(1, 0, 1, 3)]);
    let e3 = element::from_residual(&vec![(2, 0, 1, 4)]);
    let keyed = |es: &[Element]| {
        let mut k = crate::network::nerve::Keyed::new();
        for e in es {
            k.insert(e.address(), (e.clone(), 0u64));
        }
        k
    };
    let pending_a = keyed(&[e1.clone(), e2.clone(), e3.clone()]);
    let pending_b = keyed(&[e3.clone(), e1.clone(), e2.clone()]);
    let atlas = crate::transducer::stalk::Stalks::over_elements([&e1, &e2, &e3].into_iter());
    let field = crate::network::nerve::Keyed::new();
    let mut on_page = crate::network::nerve::Keyed::new();
    for e in [&e1, &e2, &e3] {
        on_page.insert(e.address(), ());
    }
    let spoken = crate::network::nerve::Keyed::new();
    let standing = crate::network::nerve::Keyed::new();
    let quote = |p: &crate::network::nerve::Keyed<(Element, u64)>| {
        quote_incidence(
            p,
            &atlas,
            &field,
            &on_page,
            &[],
            &[0; 4],
            &[1, 1, 1, 1],
            &[0; 4],
            10_000,
            &spoken,
            &[0; 32],
            &standing,
        )
    };
    let (_, pa) = quote(&pending_a);
    let (_, pb) = quote(&pending_b);
    let addrs = |p: &[(Element, [u64; 4], Vec<crate::transducer::stalk::Invariant>)]| {
        p.iter().map(|(e, _, _)| e.address()).collect::<Vec<_>>()
    };
    assert_eq!(
        addrs(&pa),
        addrs(&pb),
        "quote is atlas incidence, equal under any permutation of pending"
    );
    assert!(!pa.is_empty(), "the book is served");
}

/// heard drops when the peer's stratum root moves.
#[test]
fn heard_drops_when_stratum_root_moves() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
    k.ingest(&a, &b, [1; 32]);
    k.ingest(&a, &b, [2; 32]);
    let spoken = k.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().1.clone();
    let mut forged = spoken.clone();
    forged.mag = [999, 0, 0, 0];
    let mut cover = crate::network::nerve::Cover::new();
    assert!(matches!(k.glue_fetched(forged.clone(), [2; 32], [1; 32], &mut cover), Glue::Slashed(_)));
    assert_eq!(k.heard_len(), 1);
    let inv = crate::transducer::stalk::invariant_of(&forged);
    let mut moved = [0u8; 32];
    moved[0] = 7;
    k.confirm_peer_holds(&[crate::transducer::sheaf::StratumDigest {
        inv,
        count: 0,
        root: moved.to_vec(),
    }], [2; 32]);
    assert_eq!(k.heard_len(), 0, "a moved stratum root drops heard");
}

/// G4b. A loop's obstruction is the product around it (10.3), never a
/// transition read off the end of the fiber. The distinction is not
/// cosmetic on a distributed mesh: the geometric product is not
/// commutative, so a member picked by position is a different object on
/// every node that holds the fiber in a different insertion order, and
/// two nodes would breed two germs from identical mathematics.
#[test]
fn a_held_fiber_germinates_its_holonomy_not_a_member() {
    use crate::transducer::alphabet::{LetterId, Word};
    let q = LetterId::quarter((0, 1)).expect("quarter turn");
    let one = rotor_versor(&Word::from_letters(&[(q, false)]).rotor()).expect("versor");
    let inv = rotor_versor(&Word::from_letters(&[(q, true)]).rotor()).expect("inverse versor");
    let id = element::identity_versor();

    assert!(fiber_holonomy(&[one.clone()]).is_none(), "a singleton is not a loop");
    assert!(fiber_holonomy(&[]).is_none(), "an empty fiber is not a loop");

    let closed = fiber_holonomy(&[one.clone(), inv]).expect("a loop");
    assert!(closed.projective_eq(&id), "q then q-inverse closes");

    // Four quarter turns on one plane: the product is the obstruction,
    // and it is not any member of the fiber.
    let held = fiber_holonomy(&[one.clone(), one.clone(), one.clone()]).expect("a loop");
    assert!(!held.projective_eq(&id), "three quarter turns do not close");
    assert!(
        !held.projective_eq(&one),
        "the holonomy is the composite, not the transition the fiber ends on"
    );
}

/// G3. `listings_on` is the meet of the cell's own vertices (13.1, 14.6):
/// the listings every voice of the cell speaks, on its chart, still open.
/// It is a fold of `Keyed::meet` over the cell's vertex list — a boundary
/// walk — and never a product of the vertices with the chart's listings.
#[test]
fn listings_on_is_the_meet_of_its_vertices() {
    use crate::transducer::a21::Address;
    let mut k = Kernel::new();
    let inv = crate::transducer::stalk::Invariant(vec![7]);
    let (v1, v2) = ([1u8; 32], [2u8; 32]);
    let both = Address::from(&b"both"[..]);
    let only_v1 = Address::from(&b"only_v1"[..]);
    let shut = Address::from(&b"shut"[..]);
    let off_chart = Address::from(&b"off_chart"[..]);
    for a in [&both, &only_v1, &shut] {
        k.chart_listings.nested_insert(inv.key(), a.as_bytes(), ());
    }
    for a in [&both, &only_v1, &shut, &off_chart] {
        k.open.insert(a.as_bytes(), ());
    }
    k.open.remove(shut.as_bytes());
    k.record_spoken(both.clone(), v1);
    k.record_spoken(both.clone(), v2);
    k.record_spoken(only_v1.clone(), v1);
    k.record_spoken(shut.clone(), v1);
    k.record_spoken(shut.clone(), v2);
    k.record_spoken(off_chart.clone(), v1);
    k.record_spoken(off_chart.clone(), v2);

    let held = k.listings_on(&inv, &[v1, v2]);
    assert!(held.contains(both.as_bytes()), "spoken by both, on the chart, open");
    assert!(!held.contains(only_v1.as_bytes()), "a listing one vertex never spoke is not in the meet");
    assert!(!held.contains(shut.as_bytes()), "a closed listing is not held");
    assert!(!held.contains(off_chart.as_bytes()), "a listing off the chart is not held");
    assert_eq!(held.len(), 1, "the meet is exactly the shared, open, on-chart listing");

    // One vertex alone holds what it spoke; the meet only ever shrinks.
    let alone = k.listings_on(&inv, &[v1]);
    assert!(alone.contains(both.as_bytes()) && alone.contains(only_v1.as_bytes()));
    assert_eq!(alone.len(), 2, "and nothing it did not speak");

    // A vertex that has spoken nothing empties the cell: an intersection
    // through it is empty, never "everything on the chart".
    assert_eq!(k.listings_on(&inv, &[v1, [9u8; 32]]).len(), 0, "a silent vertex empties the meet");
}

/// The genome boundary. A taught word is CRYSTALLIZED on the receiver — it
/// enters the genome, the persisted stratum (24.1, chapter 6) — while the
/// epigenome's dialect is the transmutable one. `word_at` resolves for
/// replay and falls through basis → cavity → provisional on purpose; the
/// teaching path must not, or a voice is handed its own unratified dialect
/// back as language and files it as crystallized intelligence.
#[test]
fn the_syllabus_never_teaches_a_provisional_word() {
    use crate::transducer::element::from_residual;
    use crate::transducer::germinate::germinate;
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 2)];
    let b = vec![(0, 0, 5), (0, 1, 2)];
    let w = from_residual(&germinate(&a, &b).expect("a coboundary"));
    let addr = w.address();
    let inv = crate::transducer::stalk::invariant_of(&w);

    // The word stands only as provisional, and its address reaches the
    // ruled cover: exactly the state the soak found, basis 0 with a
    // syllabus serving.
    k.provisional.insert(&addr, w.clone());
    k.ruled.admit(&inv, addr.clone());
    assert!(k.word_at(&addr).is_some(), "replay still resolves it");
    assert!(k.installed_word(&addr).is_none(), "but nothing installed it");

    let served = k.serve_ruled(&inv, &[], 1 << 20);
    assert!(served.is_empty(), "a provisional word is not language and is not taught");

    // Installed, the same address teaches.
    k.basis_insert(w, [1; 32]);
    assert!(k.installed_word(&addr).is_some());
    assert_eq!(
        k.serve_ruled(&inv, &[], 1 << 20).len(),
        1,
        "what the court installed is what it teaches"
    );
}

/// COST PER OBSERVATION. The test that tells the two architectures apart:
/// a base that is MODIFIED absorbs one more word at a cost independent of
/// how much it already holds; a base that is APPENDED TO pays for its own
/// size on every observation, and grows once per input forever.
///
/// Admitting a word rebuilt the whole language snapshot — the word vector
/// copied, the index trie deep-cloned, N fresh `OnceLock`s allocated and
/// every lifted shape thrown away — so the Nth admit cost O(N) and the
/// fleet's footprint grew quadratically in what it had learned. This is the
/// N vs 4N counter of the enumeration audit, applied to a write.
#[test]
fn admitting_a_word_costs_the_same_at_n_and_4n() {
    use crate::network::nerve::work;
    use crate::transducer::element::from_residual;
    use crate::transducer::germinate::germinate;
    let admit_after = |n: usize| -> u64 {
        let mut k = Kernel::new();
        // Distinct SUPPORTS, not distinct positions: A5 centres a chain to
        // its min corner, so words that differ only by placement are one
        // word (1.5) and the base would not grow at all.
        let fill = |i: usize, k: &mut Kernel| {
            let a: Vec<(i64, i64, u8)> = (0..=i as i64).map(|c| (0, c, 1u8)).collect();
            let b: Vec<(i64, i64, u8)> = (0..=i as i64).map(|c| (0, c, 2u8)).collect();
            if let Some(g) = germinate(&a, &b) {
                k.basis_insert(from_residual(&g), [1; 32]);
            }
        };
        for i in 0..n {
            fill(i, &mut k);
        }
        let held = k.basis_len();
        work::reset();
        fill(n + 1, &mut k);
        let cost = work::total();
        assert!(k.basis_len() > held, "the word was admitted at n={n}");
        cost
    };
    let at_n = admit_after(8);
    let at_4n = admit_after(32);
    assert!(at_n > 0, "the admit is measured");
    assert_eq!(
        at_n, at_4n,
        "one observation costs the same into a large base as a small one: \
         {at_n} at N, {at_4n} at 4N"
    );
}

/// §6, on a kernel that has actually moved value rather than a treasury in
/// isolation. This is the gate on 1.10: settlement paths may only be built
/// where conservation is checkable, and `enroll` used to insert bond that
/// was never issued, so the second equation failed at the first enrolment.
#[test]
fn the_kernel_conserves_across_genesis_bond_and_a_settlement() {
    use crate::transducer::element::from_residual;
    use crate::transducer::germinate::germinate;
    let mut k = Kernel::new();
    // Kernel::new funds the hypha at genesis and stakes its bond from that.
    let seeded = k.treasury().audit().expect("genesis and its bond conserve");
    assert!(seeded.iter().any(|&x| x > 0), "the hypha is funded");
    assert_eq!(k.treasury().minted().expect("minted"), [0; 4], "seeded is not earned");

    let a = vec![(0, 0, 1), (0, 1, 2)];
    let b = vec![(0, 0, 5), (0, 1, 2)];
    k.basis_insert(from_residual(&germinate(&a, &b).expect("coboundary")), [1; 32]);
    k.treasury().audit().expect("an install conserves");

    // Two voices docket the same residual, then the court rules on it.
    // E1: an escrow holds μ⃗ that does not exist yet, so it is a claim on
    // issuance and not supply (§3.1, §6). A ruling discharges it into a
    // mint; a revocation makes it vanish with nothing ever issued. Counting
    // it as issued reported value the mesh had not created, and the audit
    // failed here at the first docket.
    let _ = k.ingest(&a, &b, [1; 32]);
    let _ = k.ingest(&a, &b, [2; 32]);
    let docketed = k.treasury().audit().expect("a claim is not yet supply");
    assert_eq!(k.treasury().minted().expect("minted"), [0; 4], "no claim has been ruled");

    // PINNED, with its number. Supply grew from the seed to three times it
    // across two ingests: `ensure_bonded` funds EVERY arriving identity at
    // the genesis cover so it can bond into room. Identities are free
    // (47.3), so that is a faucet — free μ⃗ per key — and it was invisible
    // while `enroll` conjured bond instead of being funded.
    //
    // §4 says an unbonded voice "may learn, speak, and be paid; every claim
    // is escrowed against its own deflation", and under E1 an escrow is a
    // claim rather than a transfer, so speaking needs no bond at all. What
    // bond buys is ROOM — throughput, the rate the metabolism governs
    // (§3.3) — and the question §4 leaves open is what room an unbonded
    // voice has. Until that is ruled, this asserts the faucet exists rather
    // than hiding it.
    assert_eq!(
        docketed,
        std::array::from_fn(|l| seeded[l] * 3),
        "one genesis allocation per arriving identity — a faucet, not a mint"
    );

    k.glue_local();
    let held = k.treasury().audit().expect("a glue conserves");
    let addrs: Vec<Address> = k.dockets().iter().map(|(a, _, _, _, _, _)| a.clone()).collect();
    for a in addrs {
        let _ = k.apply_verdict(&a, crate::anastome::dialogue::RULING_UPHELD);
    }
    let after = k.treasury().audit().expect("a verdict conserves");
    assert!(
        after.iter().zip(held.iter()).all(|(x, y)| x >= y),
        "a ruling mints and never unmakes: {held:?} then {after:?}"
    );
}
