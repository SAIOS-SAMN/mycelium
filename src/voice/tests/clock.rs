// 45.1, 45.4, 78.24 — the voice's clock is its own ledger: one node per
// admitted act, naming what it acts on by coordinates, and no reply moves
// it.

use crate::anastome::dialogue;
use crate::chronometry::{ActKind, Cite, Parent, ReceiptChain, Stamp};
use crate::voice::Voice;

/// 45.1, 45.4, OK1 — A VOICE'S CLOCK IS ITS OWN LEDGER. It seats with an
/// empty line and seals one node per admitted act — a form stood, an
/// utterance, a page learned — each under this voice's key and linked to
/// the one before; its K counts the orbits it closed, one per stroke's
/// close. Nothing sets it, so a reply carrying another kernel's k-step
/// cannot move it, and a request seals nothing.
#[test]
fn a_voice_seals_one_node_per_admitted_act_and_its_k_counts_its_orbits() {
    let mut v = Voice::named("worker-a");
    assert_eq!(v.identity(), Voice::named("worker-a").identity());
    assert_ne!(v.identity(), Voice::named("worker-b").identity());
    assert_eq!(v.k(), 0, "a voice that has spoken nothing accrues nothing");
    assert_eq!(v.chain().own().count(), 0, "and holds no node");

    let taught = Cite::Word {
        chart: crate::transducer::stalk::Invariant::counts(&[1]),
        class: None,
        at: Stamp { h: 7, vk: [9; 32], k: 3, i: 0 },
    };
    // Three acts of distinct measure, so the nodes can be told apart by
    // what they carried and not only by what kind they were.
    // Three acts in one stroke are three nodes of one orbit; its close is a
    // fourth node, carrying the orbit's frame, and moves K.
    v.seal(ActKind::Stood, [1, 0, 0, 0], Vec::new());
    v.seal(ActKind::Uttered, [0, 2, 0, 0], Vec::new());
    v.seal(ActKind::Learned, [0, 0, 0, 3], vec![taught.clone()]);
    assert_eq!((v.chain().own_len(), v.k()), (3, 0), "one admitted act is one node; K stands in the open orbit");
    assert!(matches!(v.orbit(Vec::new()), crate::chronometry::Close::Sealed(_)), "the stroke closes by name");
    assert_eq!(v.k(), 1, "K counts the closed orbit");
    assert_eq!(v.orbit(Vec::new()), crate::chronometry::Close::Nothing, "a close with no act seals nothing");

    let rs: Vec<_> = v.chain().own().cloned().collect();
    assert_eq!(rs.len(), 4);
    assert_eq!((rs[3].kind, rs[3].mu), (ActKind::Orbit, [1, 2, 0, 3]), "the close carries the orbit's frame");
    assert_eq!(rs[2].cites, vec![taught], "the node names what it learned by its coordinates");
    // 45.1: and what it measured, per lane, on the node itself.
    assert_eq!(rs[0].mu, [1, 0, 0, 0], "the act's own measure rides the node");
    assert_eq!(rs[1].mu, [0, 2, 0, 0]);
    assert_eq!(rs[2].mu, [0, 0, 0, 3]);
    assert_eq!(rs[0].stamp.vk, v.identity(), "under this voice's key");
    assert_eq!(rs[0].stamp.k, 0);
    assert_eq!(rs[1].prev, Some(Parent { at: rs[0].stamp, hash: rs[0].chain_hash }), "and links to its predecessor");
    assert_ne!(rs[0].operator_hash, rs[1].operator_hash, "the act is on the node");
    assert!(v.chain().verify_chain_integrity(), "the line re-derives end to end");

    let ask = v.quote().expect("the ask fits its lengths");
    assert_eq!(dialogue::act_tag(&ask), Some(dialogue::ACT_QUOTE));
    assert_eq!((v.chain().own_len(), v.k()), (4, 1), "a request is not an admitted act and seals nothing");
}

/// 78.24 (d1) — A VOICE'S LEDGER RESTORES UNDER ITS OWN KEY AND NO OTHER:
/// its K continues across a boot, and a ledger signed by another key is
/// refused whole.
#[test]
fn a_voices_ledger_restores_under_its_own_key_and_no_other() {
    let mut v = Voice::named("worker-a");
    v.seal(ActKind::Stood, [1, 0, 0, 0], Vec::new());
    v.orbit(Vec::new());
    v.seal(ActKind::Uttered, [0, 1, 0, 0], Vec::new());
    v.orbit(Vec::new());
    let mut tape = Vec::new();
    v.chain().put_tries(&mut tape).expect("the ledger writes");
    let chain = ReceiptChain::take_tries(&tape, &mut 0).expect("and reads");
    let mut booted = Voice::named("worker-a");
    assert!(booted.restore(chain.clone(), Vec::new()), "its own ledger restores");
    assert_eq!(booted.k(), 2, "and its K continues from where it stood");
    let mut other = Voice::named("worker-b");
    assert!(!other.restore(chain, Vec::new()), "another voice's ledger is refused");
    assert_eq!(other.k(), 0, "and nothing moved");
}

/// 78.37, TP1 — THE ORBIT'S DOCKET RIDES ITS CLOSE BY COORDINATES AND
/// SURVIVES A RESTART: a held class and a demand, one with its cover and one
/// whose enclosure is owed, cited on the close, written through the ledger's
/// tries and read back whole; neither locates by a stamp; and the restored
/// close re-seeds the next orbit's questions.
#[test]
fn an_orbits_docket_rides_its_close_and_survives_a_restart() {
    let p = {
        let id = crate::transducer::alphabet::LetterId::pythagorean_5((0, 1)).expect("the sensor plane");
        crate::transducer::alphabet::Word::from_letters(&[(id, false)]).rotor()
    };
    let kappa = crate::transducer::sentence::ClassVector::of_rotor(
        &p,
        crate::transducer::element::identity_versor(),
        crate::transducer::sentence::CycleType::Identity,
    )
    .expect("a class reads");
    let docket = vec![
        Cite::Class { class: Box::new(kappa.clone()), rotor: Box::new(p.clone()), cover: Some(vec![vec![1, 2], vec![3]]) },
        // 78.47 — the demand's residue rides by its canonical ray.
        Cite::Demand {
            class: Box::new(kappa),
            residue: crate::transducer::cga::rotator(
                &num_rational::BigRational::from_integer(4.into()),
                &num_rational::BigRational::from_integer(1.into()),
            )
            .canonical(),
            cover: None,
        },
    ];
    assert!(docket.iter().all(|c| c.at().is_none()), "a docket entry locates by its coordinates, never a stamp");
    let mut v = Voice::named("worker-a");
    v.seal(ActKind::Stood, [1, 0, 0, 0], Vec::new());
    assert!(matches!(v.orbit(docket.clone()), crate::chronometry::Close::Sealed(_)), "the close seals with its docket");
    let mut tape = Vec::new();
    v.chain().put_tries(&mut tape).expect("the ledger writes");
    let chain = ReceiptChain::take_tries(&tape, &mut 0).expect("and reads");
    let close = chain.own().filter(|r| r.kind == ActKind::Orbit).last().expect("the close");
    assert_eq!(close.cites, docket, "the docket read back whole, by coordinates");
    let mut c = crate::transducer::bubble::Contemplation::new();
    c.docket_reseed(&close.cites);
    assert_eq!(c.docket_reading().open, 2, "the restart keeps the plan");
    assert_eq!(c.docket_reading().owed, 1, "and the entry whose cover did not read says so");
}

/// 78.37, R1 — THE PLAN CARRIES FORWARD: an entry not answered within one
/// orbit stays open, is cited by the next close as well — the last Orbit act
/// is the whole plan — and re-seeds from either close.
#[test]
fn an_open_entry_is_cited_by_every_close_until_answered() {
    let p = {
        let id = crate::transducer::alphabet::LetterId::pythagorean_5((0, 1)).expect("the sensor plane");
        crate::transducer::alphabet::Word::from_letters(&[(id, false)]).rotor()
    };
    let kappa = crate::transducer::sentence::ClassVector::of_rotor(
        &p,
        crate::transducer::element::identity_versor(),
        crate::transducer::sentence::CycleType::Identity,
    )
    .expect("a class reads");
    let entry = Cite::Demand { class: Box::new(kappa), residue: None, cover: Some(vec![vec![7]]) };
    let mut c = crate::transducer::bubble::Contemplation::new();
    c.docket_reseed(std::slice::from_ref(&entry));
    let mut v = Voice::named("worker-a");
    let mut closes = Vec::new();
    for _ in 0..2 {
        v.seal(ActKind::Stood, [1, 0, 0, 0], Vec::new());
        assert!(matches!(v.orbit(c.docket_cites()), crate::chronometry::Close::Sealed(_)), "the close seals");
        c.docket_sealed();
        closes.push(v.chain().own().filter(|r| r.kind == ActKind::Orbit).last().expect("the close").cites.clone());
    }
    assert!(closes.iter().all(|cites| cites.contains(&entry)), "cited by both closes: {closes:?}");
    for cites in &closes {
        let mut again = crate::transducer::bubble::Contemplation::new();
        again.docket_reseed(cites);
        assert_eq!(again.docket_reading().open, 1, "and re-seeds from either");
    }
    assert_eq!(c.docket_reading().open, 1, "never left by replacement");
}
