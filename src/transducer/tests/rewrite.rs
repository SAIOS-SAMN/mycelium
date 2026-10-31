use crate::transducer::rewrite::*;

/// 47.1 AT THE CONSTRUCTOR: an address too long to cite is refused
/// where the element is built. The writer used to drop the slot and
/// leave a tape that did not decode, and the key of the element
/// carrying it fell through to the form tag alone — every such element
/// at one address, which is law 2 broken at its root.
#[test]
fn mint_refuses_address_too_long_to_cite() {
    let long = crate::transducer::address::Address::new(vec![0u8; u16::MAX as usize + 1]);
    let short = crate::transducer::address::Address::from_bytes([7u8]);
    assert!(long.cited().is_none(), "it does not fit the prefix");
    assert!(short.cited().is_some(), "and an address that fits does");
    let payload = || RewritePayload::Retire { cites: vec![short.clone()], freed: [1, 0, 0, 0] };
    assert!(
        mint(
            RewriteKind::Retire,
            std::slice::from_ref(&long),
            short.clone(),
            payload(),
            [1, 0, 0, 0],
        )
        .is_none(),
        "a rewrite that cannot name what it rewrote is not minted"
    );
    assert!(
        mint(RewriteKind::Retire, std::slice::from_ref(&short), long, payload(), [1, 0, 0, 0])
            .is_none(),
        "nor one that cannot name what it reached"
    );
}

/// PD1's return — A TORSE CLOSES AT ITS PERIOD, its generator's order
/// exactly: a Torse at twice the order would be paid twice over, and is
/// refused on replay by name; at the order, for either kind, it mints and
/// replays green (the court's Congruent fill mints the holonomy kind).
#[test]
fn replay_refuses_torse_at_twice_generator_order() {
    use crate::transducer::address::Address;
    use crate::transducer::element::Generator;
    let q = crate::transducer::alphabet::LetterId::turn_90((0, 1)).expect("the sensor plane");
    let turn_90 = crate::transducer::alphabet::Word::from_letters(&[(q, false)]).rotor();
    let swap = Generator::Hue(vec![(1, 2), (2, 1)]);
    let turn = Generator::Holonomy(Box::new(turn_90));
    for (generator, order) in [(swap, 2u64), (turn, 4)] {
        let torse = |period: u64| {
            let payload = RewritePayload::Torse { generator: generator.clone(), period };
            let mu = [0, 0, 0, period - 1];
            mint(RewriteKind::Torse, &[Address::from_bytes([1u8])], Address::from_bytes([2u8]), payload, mu)
                .expect("a Torse that yields is minted")
        };
        assert!(replay(&torse(order)).is_none(), "{generator:?} at its order replays green");
        assert_eq!(
            replay(&torse(2 * order)),
            Some("torse period is not its generator's order"),
            "{generator:?} at twice its order is refused by name"
        );
    }
}

/// A condense rewrite round-trips through its tape and replays both ways.
#[test]
fn mint_condense_round_trips_and_replays() {
    let after = crate::transducer::address::Address::from_bytes([7u8]);
    let payload = RewritePayload::Condense {
        sections: vec![vec![(1, 4)]],
        gauge: crate::transducer::element::identity_versor(),
    };
    let e = mint(
        RewriteKind::Condense,
        &[
            crate::transducer::address::Address::from_bytes([1u8]),
            crate::transducer::address::Address::from_bytes([2u8]),
        ],
        after,
        payload,
        [0, 1, 0, 0],
    )
    .expect("work");
    assert!(e.is_rewrite());
    assert!(replay(&e).is_none(), "both ways re-derive");
    let back = Rewrite::from_element(&e).unwrap();
    assert_eq!(back.claimed[LANE_GAUGE], 1);
    assert_eq!(back.before.len(), 2);
}

/// G6. Retirement by citation is the compression axis: the structure
/// moves toward the geodesic and the result does not move at all (3.8,
/// T10). The yield is the description the language stopped carrying, on
/// the lanes it occupied — never a constant, which is what the `Refine`
/// arm this replaces paid (`yield_from_payload` hardcoded 1 on the shell
/// lane and `yield_vector` carried `.max(1)` so the two would agree).
#[test]
fn mint_retire_pays_freed_lanes() {
    use crate::transducer::address::Address;
    let word = Address::from_bytes([1u8]);
    let speaker = Address::from_bytes([2u8]);
    let freed = [3u64, 0, 0, 5];
    let e = mint(
        RewriteKind::Retire,
        std::slice::from_ref(&word),
        word.clone(),
        RewritePayload::Retire { cites: vec![speaker.clone()], freed },
        freed,
    )
    .expect("a retirement that frees something is work");
    assert!(replay(&e).is_none(), "it re-derives from its own tape");
    let back = Rewrite::from_element(&e).expect("round trip");
    assert_eq!(back.claimed, freed, "the yield is the measure, not a constant");
    assert_eq!(back.after.address(), &word, "the vertex it names is the one it retires");
    match back.payload {
        RewritePayload::Retire { ref cites, freed: f } => {
            assert_eq!(cites, &vec![speaker], "the cites survive the tape");
            assert_eq!(f, freed);
        }
        _ => panic!("the tape decodes to the move it encoded"),
    }
}

/// The three things a retirement cannot be, checkable from the tape
/// alone. That the cites genuinely reproduce the retired substance is a
/// re-derivation over the language and is the kernel's, made before the
/// mint; the court checks here what the tape can prove.
#[test]
fn replay_refuses_retirement_proving_nothing() {
    use crate::transducer::address::Address;
    let word = Address::from_bytes([1u8]);
    let other = Address::from_bytes([2u8]);
    let freed = [3u64, 0, 0, 0];
    let mk = |cites: Vec<Address>, after: Address| {
        mint(
            RewriteKind::Retire,
            std::slice::from_ref(&word),
            after,
            RewritePayload::Retire { cites, freed },
            freed,
        )
        .map(|e| replay(&e))
    };
    assert_eq!(
        mk(Vec::new(), word.clone()),
        Some(Some("retirement cites nothing that speaks it")),
        "a deletion is not a retirement"
    );
    assert_eq!(
        mk(vec![word.clone()], word.clone()),
        Some(Some("retirement cites the word it retires")),
        "a word is not its own proof"
    );
    assert_eq!(
        mk(vec![other.clone()], other),
        Some(Some("retirement changes the vertex it names")),
        "the meaning does not move"
    );
    assert!(
        mint(
            RewriteKind::Retire,
            std::slice::from_ref(&word),
            word.clone(),
            RewritePayload::Retire { cites: vec![Address::from_bytes([2u8])], freed: [0; 4] },
            [0; 4],
        )
        .is_none(),
        "a retirement that frees nothing is not work"
    );
}

/// A rewrite whose yield is zero on every lane is not work and is not minted.
#[test]
fn mint_refuses_zero_yield() {
    assert!(mint(
        RewriteKind::Condense,
        &[crate::transducer::address::Address::from_bytes([1u8])],
        crate::transducer::address::Address::from_bytes([2u8]),
        RewritePayload::Condense { sections: vec![], gauge: crate::transducer::element::identity_versor() },
        [0; 4],
    )
    .is_none());
}
