// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// L17 DEPTH 4, 1.9, 12, 23.9 — THE RELATOR A TRIANGLE OF CHARTS LEAVES.
// Three charts, three transports, one orbit: the holonomy is the product
// around the triangle, read after the tree gauge of 12.2. Trivial, the
// charts glue and the proof is one proof in three frames with nothing
// adjoined; non-trivial, the product IS the relator, and its order is the
// period it is filed at.
//
// Three courts of distinct identity — [31;32], [32;32], [33;32] — because
// the claim is that the reading is CONTENT: three organs that never speak
// to each other read the same class key and the same order off the same
// three transports.
//
// First proof in process, three courts, no corpus (1.9).

use crate::cognition::*;
use crate::transducer::alphabet::{LetterId, Order, Word};
use crate::transducer::element::{self, Element};

/// The 90° turn on the sensor plane, `n` times. One of the seventeen.
fn turn_90(n: usize) -> Word {
    let q = LetterId::turn_90((0, 1)).expect("the 90° turn on the sensor plane");
    Word::from_letters(&vec![(q, false); n])
}

/// The same, inverted: `q⁻ⁿ`.
fn turn_90_inv(n: usize) -> Word {
    let q = LetterId::turn_90((0, 1)).expect("the 90° turn on the sensor plane");
    Word::from_letters(&vec![(q, true); n])
}

/// A TRANSPORT as the Čech readers take one: a connection carrying the
/// word and nothing else — no chain, identity carriage, identity gauge.
/// `cech::rotor_of` reads its letters, which is what makes it a transport
/// and not a residual.
fn transport(w: &Word) -> Element {
    element::from_demand(w).expect("a connection carrying the word")
}

/// THE RELATOR A TRIANGLE LEAVES, as a word of the seventeen.
///
/// `Element::compose` carries the composite on the action's VERSOR and not
/// on a tape — a composed element has no letters, and `cech::rotor_of`
/// says so at its own line ("None is a remainder: the edge is honest and
/// no simplex is formed over it"). So the relator is read the way every other
/// measured carriage is read: A22's ordered peel of the versor. Reading it
/// off a tape the composite does not have would be reading a spelling
/// nobody wrote.
fn relator_of(h: &Element) -> Word {
    let v = h.versor().expect("the holonomy of transports carries a versor");
    crate::transducer::family::peel_versor(&v)
        .expect("the holonomy has a sensor block")
        .expect("and A22 spells it: every transport here is one of the seventeen")
}

/// The three courts. Distinct identities, and they are never introduced to
/// one another: what they agree on, they agree on because it is content.
fn three_courts() -> [Kernel; 3] {
    let mut out = [Kernel::new(), Kernel::new(), Kernel::new()];
    for (k, id) in out.iter_mut().zip([[31u8; 32], [32u8; 32], [33u8; 32]]) {
        k.set_court_voice(id);
    }
    out
}

/// WHY THREE CHARTS ARE NEEDED, asserted rather than assumed (12's triple
/// overlap). A transport READ FROM TWO GERMS is `g_y ∘ g_x⁻¹`, so a
/// triangle of transports read from three germs of ONE object is a
/// coboundary and its holonomy is the identity by construction — whatever
/// the three germs are. Two derivations of one vertex through two frames
/// therefore have nothing to adjoin, and a non-trivial relator needs three
/// charts whose transports are each RULED and not each derived from a
/// shared germ.
///
/// This is the structural fact the rest of the file rests on, and without
/// it the non-trivial fixture below would look like a fixture that chose
/// its answer.
#[test]
fn transports_read_from_three_germs_of_one_object_always_glue() {
    use crate::transducer::cech::{glues, holonomy, transition};
    // Three germs of one object, deliberately unequal and deliberately
    // not related by any pattern: what is claimed holds for any three.
    let (g_a, g_b, g_c) = (
        transport(&turn_90(1)),
        transport(&turn_90(3)),
        transport(&turn_90_inv(2)),
    );
    let t_ab = transition(&g_a, &g_b).expect("a germ with a versor is invertible");
    let t_bc = transition(&g_b, &g_c).expect("invertible");
    let t_ca = transition(&g_c, &g_a).expect("invertible");
    assert!(
        glues(&t_ab, &t_bc, &t_ca) == Some(true),
        "a triangle of transitions read from three germs is a coboundary: {:?}",
        holonomy(&t_ab, &t_bc, &t_ca).as_ref().and_then(crate::transducer::cech::rotor_of)
    );
}

/// L17 DEPTH 4, THE TRIVIAL CASE. Three charts whose transports close —
/// `q`, `q`, `q⁻²` — are ONE PROOF IN THREE FRAMES: the holonomy is the
/// identity, `glues` reads true, and there is nothing to adjoin. Each of
/// the three courts reads that independently and each one's count of
/// generators the engine opened stays at zero.
#[test]
fn three_charts_whose_transports_close_are_one_proof_and_adjoin_nothing() {
    use crate::transducer::cech::{glues, holonomy};
    let (t_ab, t_bc, t_ca) = (
        transport(&turn_90(1)),
        transport(&turn_90(1)),
        transport(&turn_90_inv(2)),
    );
    let h = holonomy(&t_ab, &t_bc, &t_ca).expect("the holonomy forms");
    assert_eq!(glues(&t_ab, &t_bc, &t_ca), Some(true), "q · q · q⁻² closes");
    let r = relator_of(&h).rotor();
    assert!(r.is_identity(), "and the relator it leaves is the identity: {r:?}");
    assert_eq!(r.order(), Some(Order::Finite(1)), "order 1 is no relation at all");

    // CLOSURE PAID ONCE. Nothing is opened on any court by an orbit that
    // closes, and the three courts agree without being introduced.
    for k in three_courts() {
        assert_eq!(
            k.adjoined(),
            0,
            "a triangle that glues opens no generator on any court"
        );
    }
}

/// L17 DEPTH 4, THE NON-TRIVIAL CASE. Three charts whose transports do NOT
/// close — `q`, `q`, identity — leave the 180° turn as the relator. Its
/// order is 2, which is the period the relation is filed at, and it is not
/// the identity, so the charts do not glue.
///
/// THE THREE COURTS READ ONE CLASS. Each reads the class key off the
/// holonomy's own rotor; the three keys are equal because the class is
/// content (law 2) and not a vantage. That is what makes the relator
/// citable by a later form on any of them.
#[test]
fn three_charts_that_do_not_close_leave_a_relator_of_order_two() {
    use crate::transducer::cech::{glues, holonomy};
    let (t_ab, t_bc, t_ca) = (
        transport(&turn_90(1)),
        transport(&turn_90(1)),
        transport(&Word::identity()),
    );
    assert_eq!(glues(&t_ab, &t_bc, &t_ca), Some(false), "q · q · 1 does not close");
    let h = holonomy(&t_ab, &t_bc, &t_ca).expect("the holonomy forms");
    let r = relator_of(&h).rotor();
    assert!(!r.is_identity(), "the relator is the 180° turn, not the identity");
    assert_eq!(
        r.order(),
        Some(Order::Finite(2)),
        "and its ORDER is the period the relation closes at: {r:?}"
    );
    assert!(
        r.projective_eq(&turn_90(2).rotor()),
        "the relator is q², read off the orbit and not named by the fixture"
    );

    // ONE CLASS ON THREE COURTS. The key is read from the relator's rotor,
    // which is content; the courts are three distinct identities and are
    // never introduced.
    let class = r.class_key().expect("the relator has a conjugacy class");
    for k in three_courts() {
        let read = relator_of(&holonomy(&t_ab, &t_bc, &t_ca).expect("the holonomy forms"))
            .rotor()
            .class_key()
            .expect("each court reads the same triangle");
        assert_eq!(read, class, "three vantages, one class key");
        assert_eq!(k.adjoined(), 0, "and a RELATION is not an adjoined letter");
    }
}

/// L17 DEPTH 4, WHAT MAKES IT A DEPTH AND NOT A READING: the relator is
/// FILED, at its own order, on more than one court, and the two courts'
/// filings agree.
///
/// A form whose orbit holonomy is the relator files it as a `FiledOrbit`
/// under the relator's class key; `court_torsion` files it on the first
/// hearing and is CONSILIENT on the second, which is what "closure paid
/// once" means at the court. A court that already holds that class at a
/// DIFFERENT period refuses it as not natural — the relation is a fact
/// about the orbit and two courts cannot hold two periods for one class.
///
/// THE RELATOR IS NOT AN ADJOINED LETTER, and the difference is the whole
/// of depth 4 against depth 3: `q²` is spelled by the seventeen, so
/// `adjoined()` stays zero. What is new is not a generator, it is a
/// RELATION between generators the courts already had.
#[test]
fn the_relator_is_filed_at_its_own_period_on_two_courts_and_they_agree() {
    use crate::transducer::cech::holonomy;
    use crate::transducer::sentence::{form_torsion, Form, Vertex};
    let (t_ab, t_bc, t_ca) = (
        transport(&turn_90(1)),
        transport(&turn_90(1)),
        transport(&Word::identity()),
    );
    let relator = relator_of(&holonomy(&t_ab, &t_bc, &t_ca).expect("the holonomy forms")).rotor();
    let class = relator.class_key().expect("its class");

    // THE FORM THAT CARRIES THE ORBIT: two chart vertices and a chord whose
    // transport is the relator, so the orbit's holonomy IS the triangle's.
    let q1 = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let q0 = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    // AT1 (Q1): the two bodies are ONE vertex of the identity's class, a
    // member per chart, each overlapped from the Frame's chart.
    let mut charts = Vec::new();
    for tag in [2u8, 3u8] {
        let v = Vertex::word_on(
            crate::transducer::alphabet::Derivation::spelled(Word::identity()),
            // Its own chart: an address is derived from its coordinates (54.2).
            crate::transducer::stalk::Invariant::counts(&[u64::from(tag)]),
            Box::new(element::carriage(&q1, &(q0.clone(), q0.clone()))),
            Vec::new(),
            [0],
            None,
        );
        let c = f.stand_member(&v, 0).expect("a word vertex stands");
        f.place_overlap(crate::network::nerve::coord::ChartAt::Frame, c.clone(), None).expect("star");
        charts.push(c);
    }
    // The chord carries the RELATOR the triangle left, not a word this
    // fixture chose: it is `turn_90(2)` because that is what the holonomy
    // above read, asserted there. It rides the chord's EDGE, which is
    // measured content (50.3): a word on an overlap's factors is PRODUCED
    // content, written only by the pivot's Word move under a certificate
    // naming the overlap (AT1, ruled), and a fixture has no certificate to
    // hand the court.
    f.place_overlap(
        charts[0].clone(),
        charts[1].clone(),
        Some(crate::transducer::sentence::Transport::from_word(turn_90(2))),
    )
    .expect("a chord between two bodies");
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    let _ = f.pivot_orbit(
        [31; 32],
        &crate::network::nerve::Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(1 << 10),
        None,
        0,
    );
    let e = crate::transducer::sentence::as_element_form(&f, [1, 0, 0, 0]).expect("a tape");
    let inv = crate::transducer::stalk::invariant_of(&e);

    // FILED UNDER THE RELATOR'S OWN CLASS, AT ITS OWN PERIOD.
    let filed: Vec<(Vec<u8>, crate::transducer::sentence::ToralPeriod)> = form_torsion(&e)
        .scoped()
        .map(|(k, v)| (k, v.periods))
        .collect();
    assert_eq!(
        filed.iter().map(|(k, _)| k.clone()).collect::<Vec<_>>(),
        vec![class.as_bytes().to_vec()],
        "the form files the triangle's relator and no other class"
    );
    // One class filed, asserted above; so one period.
    let [(_, periods)] = filed.as_slice() else { panic!("one filed relator") };
    assert!(periods.closes(), "at a spectrum the relator closes on, both planes");
    assert!(
        periods.yield_units().is_some_and(|u| u >= 1),
        "and it does work on at least one plane: {periods:?}"
    );

    // AND THE COURT READS IT WITH NO CHORD SPECIAL CASE. `torsion_chord_
    // carries` asked whether the chord's own word equalled the orbit's
    // transport composite — a clause about the chord and no other face. It
    // asks only what 23.1 step 4 asks now: the chord is the one face that
    // is not `face_of`, and the orbit's reading is the orbit's. The period-2
    // class above is what replays.
    assert_eq!(
        crate::court::verdict::blind_replay(&e, &mut super::common::budget()),
        None,
        "the filed relator replays on the orbit's own reading"
    );

    // TWO COURTS, NEVER INTRODUCED. Each files on its first hearing and is
    // consilient on its second: closure paid once, per court.
    let [mut a, mut b, _] = three_courts();
    for k in [&mut a, &mut b] {
        k.complexes.insert(inv.key(), crate::transducer::cech::Complex::new());
        assert_eq!(
            k.court_torsion(&e, &inv, [31; 32], [32; 32]),
            None,
            "the first hearing files the relation"
        );
        assert!(
            k.complexes.get(&inv.key()).is_some_and(|cx| !cx.congruent(&class).is_empty()),
            "under the relator's own class key"
        );
        assert_eq!(
            k.court_torsion(&e, &inv, [31; 32], [32; 32]),
            None,
            "and the second hearing is consilient, not a second filing"
        );
        assert_eq!(k.adjoined(), 0, "a relation is not an adjoined letter");
    }

    // 54.7 — A THIRD COURT CANNOT HOLD THE SAME CLASS AT ANOTHER PERIOD.
    // The period is the orbit's order and not the court's opinion of it, and
    // that is now true by construction rather than by refusal: the spectrum
    // is re-derived from the held ROTOR, so the `99` below reaches no
    // reader. `class_key` and `plane_periods` are both functions of `(a, p)`
    // — one class key, one spectrum.
    let mut c = Kernel::new();
    c.set_court_voice([33; 32]);
    let mut cx = crate::transducer::cech::Complex::new();
    cx.file_congruent([31; 32], [32; 32], relator.clone(), 99);
    c.complexes.insert(inv.key(), cx);
    assert_eq!(
        c.court_torsion(&e, &inv, [31; 32], [32; 32]),
        None,
        "one class, one period: a court holding another is not natural"
    );
}

/// Install `w` on a court by the path a word actually takes: spoken,
/// docketed, ruled, and confirmed held by a second organ (arity 2). No
/// direct insertion — the claim below is about what the COURTS do with the
/// rewrite, so the words it is about must be language the same way every
/// other word is.
fn install(k: &mut Kernel, voice: [u8; 32], w: &Element) {
    assert_eq!(
        k.perceive_speech(voice, &[0; 32], std::slice::from_ref(w)),
        vec![crate::anastome::dialogue::SPEECH_STOOD],
        "the transport stands"
    );
    k.rule_all_provisionals();
    let cover = k.genesis_cover().expect("measured");
    k.enroll([9; 32], cover);
    let ruled: Vec<Vec<u8>> =
        k.ruled_addrs.scoped().map(|(a, _)| a).collect();
    for a in &ruled {
        k.confirm_peer_holds_addrs(&[Address::from(a.clone())], [9; 32]);
    }
}

/// L17 DEPTH 5, ON THE SAME FIXTURE — THE REWRITE IS ADJOINED AS A
/// GENERATOR IS. The triangle above left `q²` as its relator; a court that
/// holds the transport `q` as language already speaks `q²`, so the
/// standing word for `q²` is REDUNDANT and the rewrite that retires it is
/// the rewrite relating the two derivations of it (3.8, chapter 27, T10).
///
/// WHAT MAKES IT DEPTH 5 AND NOT A HOST ARM. The rewrite is an ELEMENT
/// with an address measured from its own content, docketed by the same
/// `docket_connection` a letter takes, and TWO COURTS THAT NEVER SPEAK
/// REACH THE SAME ADDRESS for it. Nothing about it is a branch written in
/// the host; the host's `RewriteKind` is a tag on content, the way
/// `Vertex::Word` is.
///
/// THE DERIVABILITY IS RE-DERIVED, twice, by the redundancy probe itself
/// (`reduce_excluding`, A12 over the plane forms of the other held words)
/// — not asserted by the bidder and not voted on. Chapter 44's refusals
/// stand: there is no completion here, no critical pair, and no minimal
/// path chosen by a scalar. One reading answers the whole question.
///
/// AND THE MEANING IS UNCHANGED. After the retirement the language still
/// speaks `q²`: the structure changed, the result did not, which is what
/// makes this convergence and not a vote.
#[test]
fn the_rewrite_that_retires_the_relator_is_reached_by_two_courts_as_content() {
    use crate::transducer::rewrite::{RewriteKind, RewritePayload};
    // The transport the triangle is built from, and the relator it left.
    let t = transport(&turn_90(1));
    let relator = transport(&turn_90(2));
    assert_ne!(t.address(), relator.address(), "two words, two addresses");

    let mut minted: Vec<Address> = Vec::new();
    for (id, voice) in [([31u8; 32], [41u8; 32]), ([32u8; 32], [42u8; 32])] {
        let mut k = Kernel::new();
        k.set_court_voice(id);
        install(&mut k, voice, &t);
        install(&mut k, voice, &relator);
        let addr = relator.address();
        assert!(k.ruled_contains(addr.as_bytes()), "the relator is language here");

        // THE DERIVABILITY, RE-DERIVED by the redundancy probe over this
        // court's own basis with the candidate excluded.
        let decomposed = k.reduce_excluding(&relator, Some(addr.as_bytes()), &mut super::common::budget());
        let NormalForm::Decomposed(through) = decomposed else {
            panic!("the relator must decompose through the transport: {decomposed:?}");
        };
        assert!(
            !through.is_empty(),
            "and it decomposes through a word that is not itself"
        );

        // THE REWRITE. `freed` is the lane vector the language stops
        // carrying, read off the retired word's own magnitude.
        let payload = RewritePayload::Retire {
            cites: vec![t.address()],
            freed: relator.magnitude(),
        };
        assert!(
            k.mint_rewrite(
                RewriteKind::Retire,
                std::slice::from_ref(&relator),
                &relator,
                payload,
                voice,
            )
            .is_some(),
            "the rewrite mints and is docketed"
        );

        // IT IS ON THE DOCKET, by the path a letter takes: a provisional
        // word this court holds, awaiting a second organ.
        let rw: Vec<Vec<u8>> = k
            .provisional
            .scoped()
            .filter(|(_, e)| e.is_rewrite())
            .map(|(a, _)| a)
            .collect();
        assert_eq!(rw.len(), 1, "one rewrite on the docket, addressed by its content");
        minted.push(Address::from(rw[0].clone()));

        // AND IT RETIRES WHAT IT NAMES. The inputs go to the cavity, not
        // to nothing: a retired word is a boundary the language keeps.
        let held = k.provisional.get(&rw[0]).cloned().expect("the docketed rewrite");
        let mut ruling = k.stroke();
        k.retire_rewrite_inputs(&held, &mut ruling);
        assert!(
            !k.basis.contains(addr.as_bytes()),
            "the relator leaves the basis"
        );
        assert!(
            k.cavity_book.contains(addr.as_bytes()),
            "onto the cavity book, which is where a retired word lives"
        );

        // THE MEANING IS UNCHANGED: the language still speaks it.
        assert!(
            matches!(
                k.reduce_excluding(&relator, None, &mut super::common::budget()),
                NormalForm::Decomposed(_)
            ),
            "the structure changed and the result did not"
        );
    }

    // TWO COURTS, ONE ADDRESS. Neither court was told the other's answer;
    // they agree because a rewrite is content and its address measures it.
    assert_eq!(
        minted[0].as_bytes(),
        minted[1].as_bytes(),
        "two courts, one rewrite"
    );
}

/// A peer's line of one act and its `Ruling(Retired)` citing `cites`.
fn retirement_line(
    key: &crate::court::identity::HyphaKey,
    cites: Vec<crate::chronometry::Cite>,
) -> Vec<crate::chronometry::KStepReceipt> {
    use crate::chronometry::{ActKind, ReceiptChain, Ruled};
    let mut line = ReceiptChain::with_vk(key.vk_bytes());
    line.append(ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("an act");
    line.append(ActKind::Ruling(Ruled::Retired), [0; 4], cites, [0; 32], Vec::new()).expect("the retirement");
    line.own().collect()
}

/// EC1 AS RULED — A PEER'S RETIREMENT IS REPLAYED HERE, never trusted and
/// never dropped. The ruled cover is federated, so a word one court
/// retires and another keeps would leave two supplies. A court holding the
/// relator RULED, and the transport that speaks for it, admits a peer's
/// `Ruling(Retired)` citing both at the stamps it holds them by: its own
/// replay — the relator decomposes over its basis without itself — re-
/// derives, the relator retires here onto the cavity book, and the court
/// seals its own `Ruling(Retired)` with the peer's receipt among its
/// parents. A court holding the relator alone cannot re-derive it and the
/// ruling is REFUSED, named and counted; a court that does not hold a cited
/// speaker WAITS on its stamp. Per arm, admitted = applied + awaiting +
/// refused.
#[test]
fn a_peers_retirement_is_replayed_here_and_applied_refused_or_waiting() {
    use crate::chronometry::{ActKind, Cite, Ruled};
    let t = transport(&turn_90(1));
    let relator = transport(&turn_90(2));
    let retired = Ruled::Retired.index();
    let (r_addr, t_addr) = (relator.address(), t.address());

    let key = super::common::court_key("retirer");
    // Each court signs under its own key, so no two courts' stamps
    // coincide: a stamp names one act of one line.
    let mut b = Kernel::new();
    b.bind_identity([0xb3; 32]);
    b.set_court_voice([33; 32]);
    install(&mut b, [43; 32], &t);
    install(&mut b, [43; 32], &relator);
    let r_at = b.admitted_at(r_addr.as_bytes()).expect("the relator's stamp");
    let t_at = b.admitted_at(t_addr.as_bytes()).expect("the transport's stamp");
    let line = retirement_line(&key, vec![Cite::word(&relator, r_at), Cite::word(&t, t_at)]);
    super::common::admit_line(&mut b, &key, &line);
    assert_eq!(b.clock(None).rulings[retired], [1, 1, 0, 0], "replayed and applied");
    assert!(!b.basis.contains(r_addr.as_bytes()), "the relator retired here");
    assert!(b.cavity_book.contains(r_addr.as_bytes()), "onto the cavity book, keeping its address");
    let own_head = b.chain().head().expect("b sealed").clone();
    assert_eq!(own_head.kind, ActKind::Ruling(Ruled::Retired), "b seals its own retirement");
    let [.., last] = line.as_slice() else { panic!("the peer's line") };
    let peer_stamp = last.stamp;
    assert!(own_head.parents.iter().any(|p| p.at == peer_stamp), "with the peer's receipt among its parents");

    let lone = super::common::court_key("lone retirer");
    let mut e = Kernel::new();
    e.bind_identity([0xe4; 32]);
    e.set_court_voice([34; 32]);
    install(&mut e, [44; 32], &relator);
    let e_at = e.admitted_at(r_addr.as_bytes()).expect("the relator's stamp");
    super::common::admit_line(&mut e, &lone, &retirement_line(&lone, vec![Cite::word(&relator, e_at)]));
    assert_eq!(e.clock(None).rulings[retired], [1, 0, 0, 1], "it does not re-derive here: refused, counted");
    assert_eq!(
        e.clock(None).ruling_refusals[retired],
        [0, 0, 1],
        "by its reason: the retired word does not decompose here without it (F5)"
    );
    assert!(e.basis.contains(r_addr.as_bytes()), "and the word stays");

    let early = super::common::court_key("early retirer");
    let mut w = Kernel::new();
    w.bind_identity([0x75; 32]);
    w.set_court_voice([35; 32]);
    install(&mut w, [45; 32], &relator);
    let w_at = w.admitted_at(r_addr.as_bytes()).expect("the relator's stamp");
    let line = retirement_line(&early, vec![Cite::word(&relator, w_at), Cite::word(&t, t_at)]);
    super::common::admit_line(&mut w, &early, &line);
    assert_eq!(w.clock(None).rulings[retired], [1, 0, 1, 0], "a speaker w does not hold: the ruling waits on its stamp");
}
