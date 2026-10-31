use super::super::*;
use super::common::*;

/// Three keepers exchange readings, and a point is final only once two keepers hold it.
#[test]
fn final_point_needs_two_of_three_keepers() {
    let mut m = Mesh::new();
    assert!(m.read(0, tick(10, [4, 0, 0, 0], 1)));
    // a's reading alone: one keeper holds it.
    assert_eq!(m.hear(1, 0), Ok(Admitted::Anchored));
    assert_eq!(m.courts[0].final_point(), None, "a's own view: nobody cites a yet");
    assert_eq!(m.courts[1].final_point(), None, "b verified a's reading, but b has not cited it");
    // b reads: its reading cites a's at h 10. b holds a's reading and its
    // own citing it — two keepers.
    assert!(m.read(1, tick(12, [0, 5, 0, 0], 3)));
    let a10 = TPoint { origin: ORIGIN, h: 10, lanes: [4, 0, 0, 0], k: 1 };
    assert_eq!(m.courts[1].final_of(&verifying(&m.keys[0])), Some(a10));
    assert_eq!(m.courts[1].final_point(), Some(a10));
    // c hears both: a's reading is final in c's view on b's citation, and
    // b's point is not (nobody cites b).
    assert_eq!(m.hear(2, 0), Ok(Admitted::Anchored));
    assert_eq!(m.hear(2, 1), Ok(Admitted::Anchored));
    assert_eq!(m.courts[2].final_of(&verifying(&m.keys[1])), None);
    assert_eq!(m.courts[2].final_point(), Some(a10));
    // c reads, citing a and b: b's point at 12 is final in c's view.
    assert!(m.read(2, tick(14, [0, 0, 6, 0], 2)));
    let b12 = TPoint { origin: ORIGIN, h: 12, lanes: [0, 5, 0, 0], k: 3 };
    assert_eq!(m.courts[2].final_point(), Some(b12), "the greatest final point");
    // a hears c: c cites a's 10, so a's point is final in a's own view too.
    assert_eq!(m.hear(0, 2), Ok(Admitted::Anchored));
    assert_eq!(m.courts[0].final_of(&verifying(&m.keys[0])), Some(a10));
}

/// A forged signature, an altered lane, a broken parent and another universe's reading are refused, and the honest line still extends.
#[test]
fn admit_refuses_forged_signature_and_broken_parent() {
    let mut m = Mesh::new();
    m.read(0, tick(10, [1, 0, 0, 0], 1));
    m.read(0, tick(11, [2, 0, 0, 0], 1));
    assert_eq!(m.hear(1, 0), Ok(Admitted::Anchored));
    // A reading signed by c's key presented as a's.
    let forged = TickReading::sign(&m.keys[2], &ORIGIN, tick(20, [9, 0, 0, 0], 1), [0; 32], Vec::new()).expect("writes");
    let as_a = TickReading { keeper: verifying(&m.keys[0]), ..forged };
    assert_eq!(m.courts[1].admit(verifying(&m.keys[0]), vec![as_a.clone()]), Err(KeeperRefusal::Signature));
    // A lane altered after signing.
    let mut altered = m.courts[0].own_ring().into_iter().next_back().expect("a reading");
    altered.tick.lanes[0] = 3;
    assert_eq!(m.courts[1].admit(verifying(&m.keys[0]), vec![altered]), Err(KeeperRefusal::Signature));
    // Signed by a, naming a parent that is not its last reading.
    let broken = TickReading::sign(&m.keys[0], &ORIGIN, tick(12, [3, 0, 0, 0], 1), [0xee; 32], Vec::new()).expect("writes");
    assert_eq!(m.courts[1].admit(verifying(&m.keys[0]), vec![broken]), Err(KeeperRefusal::BrokenParent));
    // Another universe's reading does not verify here.
    let elsewhere = Origin { genesis: [1; 32], epoch: 146 };
    let r = TickReading::sign(&m.keys[0], &elsewhere, tick(13, [3, 0, 0, 0], 1), [0; 32], Vec::new()).expect("writes");
    assert_eq!(m.courts[1].admit(verifying(&m.keys[0]), vec![r]), Err(KeeperRefusal::Signature));
    // And the honest line still extends.
    m.read(0, tick(14, [4, 0, 0, 0], 2));
    assert_eq!(m.hear(1, 0), Ok(Admitted::Extended(1)));
}

/// A disagreement between final points is held as an exact residual per lane and never averaged.
#[test]
fn residual_keeps_disagreement_never_averaged() {
    let mut m = Mesh::new();
    // a and b read at one tick, differing on S and G.
    m.read(0, tick(20, [10, 4, 0, 0], 5));
    m.read(1, tick(20, [7, 9, 0, 0], 5));
    for (to, from) in [(2, 0), (2, 1)] {
        m.hear(to, from).expect("admits");
    }
    // c cites both: both are final at h 20, and they differ.
    m.read(2, tick(21, [0, 0, 0, 1], 1));
    let (a, b) = (verifying(&m.keys[0]), verifying(&m.keys[1]));
    let pa = TPoint { origin: ORIGIN, h: 20, lanes: [10, 4, 0, 0], k: 5 };
    let pb = TPoint { origin: ORIGIN, h: 20, lanes: [7, 9, 0, 0], k: 5 };
    assert_eq!((m.courts[2].final_of(&a), m.courts[2].final_of(&b)), (Some(pa), Some(pb)));
    assert_eq!(m.courts[2].final_point(), None, "tick 20 is held to the residual; no average, no pick");
    let r = m.courts[2].residual();
    let edge = r.edges.iter().find(|e| e.h == 20 && [e.from, e.to] == [a, b] || [e.from, e.to] == [b, a]).expect("the a–b edge at 20");
    let sign = if edge.from == a { 1 } else { -1 };
    assert_eq!(edge.delta, [-3 * sign, 5 * sign, 0, 0], "b − a per lane, exactly");
    // A later point final alone moves the clock past the disagreement.
    m.read(0, tick(22, [11, 4, 0, 0], 6));
    m.hear(2, 0).expect("admits");
    m.read(2, tick(23, [0, 0, 0, 2], 2));
    m.hear(1, 2).expect("admits");
    m.hear(1, 0).expect("admits");
    m.read(1, tick(24, [7, 10, 0, 0], 6));
    assert_eq!(m.courts[1].final_point(), Some(TPoint { origin: ORIGIN, h: 23, lanes: [0, 0, 0, 2], k: 2 }));
}

/// With two keepers silent, the final point does not advance.
#[test]
fn final_point_survives_two_silent_keepers() {
    let mut m = Mesh::new();
    m.read(0, tick(10, [1, 0, 0, 0], 1));
    m.hear(1, 0).expect("admits");
    m.read(1, tick(11, [0, 1, 0, 0], 1));
    m.hear(0, 1).expect("admits");
    let before = m.courts[0].final_point();
    assert_eq!(before, Some(TPoint { origin: ORIGIN, h: 10, lanes: [1, 0, 0, 0], k: 1 }));
    // b and c fall silent: a keeps reading its own court. Its first reading
    // cites b's last, which makes b's 11 final (b signed it, a cites it) —
    // and past that, a alone moves nothing.
    let b11 = Some(TPoint { origin: ORIGIN, h: 11, lanes: [0, 1, 0, 0], k: 1 });
    for h in 12..40 {
        m.read(0, tick(h, [h, 0, 0, 0], h));
        assert_eq!(m.courts[0].final_point(), b11, "a alone moves no final point past what b last said");
    }
    // A reading whose measure did not move is no new reading.
    assert!(!m.read(0, tick(41, [39, 0, 0, 0], 39)));
}

/// 78.152b — A RING PUSHED AS `ACT_TICK` IS THE RING A DIGEST CARRIES: one
/// codec, read back whole, and admitted with the same outcome and the same
/// final point as the digest's; a byte past the ring is no tick.
#[test]
fn decode_tick_agrees_with_digest_on_ring() {
    use crate::anastome::dialogue as d;
    let mut m = Mesh::new();
    m.read(0, tick(10, [4, 0, 0, 0], 1));
    m.read(0, tick(11, [5, 0, 0, 0], 2));
    assert_eq!(m.hear(1, 0), Ok(Admitted::Anchored));
    m.read(1, tick(12, [0, 5, 0, 0], 3));
    let (a, b) = (verifying(&m.keys[0]), verifying(&m.keys[1]));
    let ring = m.courts[1].own_ring();
    // On a digest, as `sign_cover_digest` writes it.
    let digest = d::encode_cover_digest(&[0; 4], &[[0; 32]; 4], &[0; 32], &b, "127.0.0.1:1", &ring, &[0; 64]).expect("a digest writes");
    let from_digest = d::decode_cover_digest(&digest).expect("reads whole").ring;
    // On its own act.
    let frame = d::encode_tick(&b, &ring).expect("a tick writes");
    assert_eq!(d::act_tag(&frame), Some(d::ACT_TICK));
    let (keeper, pushed) = d::decode_tick(&frame).expect("reads whole");
    assert_eq!((keeper, &pushed), (b, &ring), "the keeper and its ring, exactly");
    assert_eq!(pushed, from_digest, "one codec");
    // Two courts that knew a, one hearing b's digest, one b's tick.
    let mut by_digest = Keepers::new(ORIGIN, verifying(&m.keys[2]));
    let mut by_tick = by_digest.clone();
    for k in [&mut by_digest, &mut by_tick] {
        assert_eq!(k.admit(a, m.courts[0].own_ring()), Ok(Admitted::Anchored));
    }
    assert_eq!(by_digest.admit(b, from_digest), by_tick.admit(keeper, pushed), "one outcome");
    assert_eq!(by_digest.final_point(), by_tick.final_point());
    assert_eq!(by_tick.final_point(), Some(TPoint { origin: ORIGIN, h: 11, lanes: [5, 0, 0, 0], k: 2 }), "b cites a's 11");
    // A byte past the ring, or a frame cut short, is no tick.
    let mut long = frame.clone();
    long.push(0);
    assert_eq!(d::decode_tick(&long), None);
    assert_eq!(d::decode_tick(&frame[..frame.len() - 1]), None);
}

/// A STRICT MAJORITY OF THE KEEPER SET (ruled 2026-10-09): with
/// four keepers a point needs three — its own keeper and two others citing
/// it. One citer, enough at three keepers, finalises nothing at four.
#[test]
fn final_of_needs_three_of_four_keepers() {
    let keys = [key(1), key(2), key(3), key(4)];
    let mut ks: Vec<Keepers> = keys.iter().map(|k| Keepers::new(ORIGIN, verifying(k))).collect();
    let hear = |ks: &mut Vec<Keepers>, to: usize, from: usize| {
        let ring = ks[from].own_ring();
        ks[to].admit(verifying(&keys[from]), ring)
    };
    // c and d read before hearing a: the readings of c and d cite nothing of a.
    assert!(ks[2].read_own(&keys[2], tick(5, [1, 0, 0, 0], 1)).expect("writes"));
    assert!(ks[3].read_own(&keys[3], tick(6, [0, 1, 0, 0], 1)).expect("writes"));
    assert!(ks[0].read_own(&keys[0], tick(10, [4, 0, 0, 0], 1)).expect("writes"));
    assert_eq!(hear(&mut ks, 0, 2), Ok(Admitted::Anchored));
    assert_eq!(hear(&mut ks, 0, 3), Ok(Admitted::Anchored));
    // b hears a and cites it.
    assert_eq!(hear(&mut ks, 1, 0), Ok(Admitted::Anchored));
    assert!(ks[1].read_own(&keys[1], tick(12, [0, 5, 0, 0], 3)).expect("writes"));
    assert_eq!(hear(&mut ks, 0, 1), Ok(Admitted::Anchored));
    assert_eq!(ks[0].final_of(&verifying(&keys[0])), None, "a and b are two of four: no majority");
    // c hears a and cites it: a, b, c are three of four.
    assert_eq!(hear(&mut ks, 2, 0), Ok(Admitted::Anchored));
    assert!(ks[2].read_own(&keys[2], tick(14, [1, 0, 6, 0], 2)).expect("writes"));
    // a already holds c's line: the new reading extends it.
    assert!(matches!(hear(&mut ks, 0, 2), Ok(Admitted::Extended(_))), "c's line grows by its new reading");
    let a10 = TPoint { origin: ORIGIN, h: 10, lanes: [4, 0, 0, 0], k: 1 };
    assert_eq!(ks[0].final_of(&verifying(&keys[0])), Some(a10), "three of four is a majority");
}
