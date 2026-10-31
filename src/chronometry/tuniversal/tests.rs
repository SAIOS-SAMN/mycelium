// 78.152 — the TUniversal law, proved by exact values: three keepers in one
// process exchange readings, a point is final only on two verified keepers,
// a forgery and a broken parent are refused, a disagreement is a residual and
// never an average, silence holds the clock, the point's writing reads back.

use super::*;
use ed25519_dalek::SigningKey;

fn key(n: u8) -> SigningKey {
    SigningKey::from_bytes(&[n; 32])
}

fn vk(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

const ORIGIN: Origin = Origin { genesis: [0x6a; 32], epoch: 146 };

fn tick(h: u64, lanes: [u64; 4], k: u64) -> Tick {
    Tick { h, lanes, k }
}

/// Three keepers, one court each, all in this process.
struct Mesh {
    keys: [SigningKey; 3],
    courts: [Keepers; 3],
}

impl Mesh {
    fn new() -> Mesh {
        let keys = [key(1), key(2), key(3)];
        let courts = [Keepers::new(ORIGIN, vk(&keys[0])), Keepers::new(ORIGIN, vk(&keys[1])), Keepers::new(ORIGIN, vk(&keys[2]))];
        Mesh { keys, courts }
    }

    fn read(&mut self, i: usize, t: Tick) -> bool {
        self.courts[i].read_own(&self.keys[i], t).expect("a reading writes")
    }

    /// Court `to` admits `from`'s ring, as a digest carries it.
    fn hear(&mut self, to: usize, from: usize) -> Result<Admitted, Refused> {
        let ring = self.courts[from].own_ring();
        let keeper = vk(&self.keys[from]);
        self.courts[to].admit(keeper, ring)
    }
}

/// A point prints in one lowercase writing and parses back exactly, and its wire form takes back whole.
#[test]
fn tpoint_print_parses_back_exactly() {
    let p = TPoint { origin: ORIGIN, h: 40_961, lanes: [3, u64::MAX, 0, 17], k: 812 };
    let text = p.print();
    assert_eq!(
        text,
        format!("{}.00000092/000000000000a001:0000000000000003:ffffffffffffffff:0000000000000000:0000000000000011:000000000000032c", "6a".repeat(32))
    );
    assert_eq!(TPoint::parse(&text), Some(p));
    assert_eq!(TPoint::parse(&text.to_uppercase()), None, "one writing: lowercase");
    assert_eq!(TPoint::parse(&format!("{text}:0000000000000000")), None, "a seventh field is no point");
    assert_eq!(TPoint::parse(&text.replacen("/000000000000a001", "/a001", 1)), None, "a short field is no point");
    let mut wire = Vec::new();
    p.put(&mut wire);
    let mut at = 0;
    assert_eq!((TPoint::take(&wire, &mut at), at), (Some(p), TPoint::LEN));
}

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
    assert_eq!(m.courts[1].final_of(&vk(&m.keys[0])), Some(a10));
    assert_eq!(m.courts[1].final_point(), Some(a10));
    // c hears both: a's reading is final in c's view on b's citation, and
    // b's point is not (nobody cites b).
    assert_eq!(m.hear(2, 0), Ok(Admitted::Anchored));
    assert_eq!(m.hear(2, 1), Ok(Admitted::Anchored));
    assert_eq!(m.courts[2].final_of(&vk(&m.keys[1])), None);
    assert_eq!(m.courts[2].final_point(), Some(a10));
    // c reads, citing a and b: b's point at 12 is final in c's view.
    assert!(m.read(2, tick(14, [0, 0, 6, 0], 2)));
    let b12 = TPoint { origin: ORIGIN, h: 12, lanes: [0, 5, 0, 0], k: 3 };
    assert_eq!(m.courts[2].final_point(), Some(b12), "the greatest final point");
    // a hears c: c cites a's 10, so a's point is final in a's own view too.
    assert_eq!(m.hear(0, 2), Ok(Admitted::Anchored));
    assert_eq!(m.courts[0].final_of(&vk(&m.keys[0])), Some(a10));
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
    let as_a = TickReading { keeper: vk(&m.keys[0]), ..forged };
    assert_eq!(m.courts[1].admit(vk(&m.keys[0]), vec![as_a.clone()]), Err(Refused::Signature));
    // A lane altered after signing.
    let mut altered = m.courts[0].own_ring().into_iter().next_back().expect("a reading");
    altered.tick.lanes[0] = 3;
    assert_eq!(m.courts[1].admit(vk(&m.keys[0]), vec![altered]), Err(Refused::Signature));
    // Signed by a, naming a parent that is not its last reading.
    let broken = TickReading::sign(&m.keys[0], &ORIGIN, tick(12, [3, 0, 0, 0], 1), [0xee; 32], Vec::new()).expect("writes");
    assert_eq!(m.courts[1].admit(vk(&m.keys[0]), vec![broken]), Err(Refused::BrokenParent));
    // Another universe's reading does not verify here.
    let elsewhere = Origin { genesis: [1; 32], epoch: 146 };
    let r = TickReading::sign(&m.keys[0], &elsewhere, tick(13, [3, 0, 0, 0], 1), [0; 32], Vec::new()).expect("writes");
    assert_eq!(m.courts[1].admit(vk(&m.keys[0]), vec![r]), Err(Refused::Signature));
    // And the honest line still extends.
    m.read(0, tick(14, [4, 0, 0, 0], 2));
    assert_eq!(m.hear(1, 0), Ok(Admitted::Extended(1)));
}

/// A disagreement between final points is held as an exact residual per lane and never averaged.
#[test]
fn residual_holds_disagreement_exactly_never_averaged() {
    let mut m = Mesh::new();
    // a and b read at one tick, differing on S and G.
    m.read(0, tick(20, [10, 4, 0, 0], 5));
    m.read(1, tick(20, [7, 9, 0, 0], 5));
    for (to, from) in [(2, 0), (2, 1)] {
        m.hear(to, from).expect("admits");
    }
    // c cites both: both are final at h 20, and they differ.
    m.read(2, tick(21, [0, 0, 0, 1], 1));
    let (a, b) = (vk(&m.keys[0]), vk(&m.keys[1]));
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
fn final_point_holds_while_two_keepers_silent() {
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

/// The shelf advances without the kernel and wakes a parked reader.
#[test]
fn tshelf_raise_final_wakes_parked_reader() {
    let shelf = std::sync::Arc::new(TShelf::new());
    let p = TPoint { origin: ORIGIN, h: 10, lanes: [1, 0, 0, 0], k: 1 };
    assert!(shelf.raise_final(p));
    assert!(!shelf.raise_final(p), "the same point is no advance");
    assert!(!shelf.raise_final(TPoint { h: 9, ..p }), "a lower one neither");
    assert_eq!((shelf.final_point(), shelf.advances()), (Some(p), 1));
    // A speaker's point is heard, never final.
    shelf.heard_at_door(Some(TPoint { h: 99, ..p }));
    assert_eq!((shelf.final_point(), shelf.heard().map(|q| q.h)), (Some(p), Some(99)));
    // A reader parked on what it saw wakes on the next advance.
    let seen = shelf.seen();
    let s2 = std::sync::Arc::clone(&shelf);
    let raiser = std::thread::spawn(move || s2.raise_final(TPoint { h: 11, ..p }));
    let woke = loop {
        let now = shelf.park(seen, crate::physics::stroke_floor());
        if now.advances > seen.advances {
            break now;
        }
    };
    assert!(raiser.join().expect("joins"));
    assert_eq!(woke.advances, 2);
}

/// Reset, branch and superpose cite the point they act on.
#[test]
fn ops_reset_branch_superpose_cite_point() {
    let p = TPoint { origin: ORIGIN, h: 30, lanes: [5, 1, 0, 2], k: 9 };
    let r = ops::Reset::at(p).expect("an epoch past");
    assert_eq!((r.epoch, r.cites), (147, ops::citation(&p)));
    let (x, y) = (ops::Branch::of(p, [1; 32]), ops::Branch::of(p, [2; 32]));
    assert_ne!(x.origin, y.origin, "two branches of one point are two universes");
    let a = TPoint { origin: x.origin, h: 4, lanes: [1, 0, 0, 0], k: 1 };
    let b = TPoint { origin: y.origin, h: 6, lanes: [0, 3, 0, 0], k: 2 };
    let s = ops::Superposed::of(a, b);
    assert_eq!(s.residual, [-1, 3, 0, 0, 1]);
    let by = TPoint { h: 7, ..p };
    assert_eq!(s.merged_by(by, [ops::citation(&a), ops::citation(&b)]), Some(by));
    assert_eq!(s.merged_by(by, [ops::citation(&a), ops::citation(&a)]), None, "a merge cites both");
    assert_eq!(s.merged_by(TPoint { h: 5, ..p }, [ops::citation(&a), ops::citation(&b)]), None);
    assert_eq!(p.unpack().hh, 2);
}

/// 78.152b — A RING PUSHED AS `ACT_TICK` IS THE RING A DIGEST CARRIES: one
/// codec, read back whole, and admitted with the same outcome and the same
/// final point as the digest's; a byte past the ring is no tick.
#[test]
fn decode_tick_admits_ring_as_digest_does() {
    use crate::anastome::dialogue as d;
    let mut m = Mesh::new();
    m.read(0, tick(10, [4, 0, 0, 0], 1));
    m.read(0, tick(11, [5, 0, 0, 0], 2));
    assert_eq!(m.hear(1, 0), Ok(Admitted::Anchored));
    m.read(1, tick(12, [0, 5, 0, 0], 3));
    let (a, b) = (vk(&m.keys[0]), vk(&m.keys[1]));
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
    let mut by_digest = Keepers::new(ORIGIN, vk(&m.keys[2]));
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

/// 78.152 (PERSISTED) — A KEEPER RESTARTED FROM ITS RECORD never signs a
/// height at or below its last, and its next reading extends its line at a
/// peer (`Extended`, never `Anchored`). The record reads back by value; a
/// record altered, cut short, or of another origin is no floor.
#[test]
fn resume_never_signs_at_or_below_last_height() {
    let mut m = Mesh::new();
    let a = vk(&m.keys[0]);
    let mut kept = None;
    assert_eq!(m.courts[0].read_own_kept(&m.keys[0], tick(10, [4, 0, 0, 0], 1), |r| {
        kept = Some(r.clone());
        true
    }), Some(true));
    assert_eq!(m.courts[0].read_own_kept(&m.keys[0], tick(11, [5, 0, 0, 0], 2), |r| {
        kept = Some(r.clone());
        true
    }), Some(true));
    assert_eq!(m.hear(1, 0), Ok(Admitted::Anchored));
    let rec = kept.expect("the last reading was kept");
    assert_eq!((rec.origin, rec.reading.tick.h, rec.reading.keeper), (ORIGIN, 11, a));
    // A reading the record did not keep is neither held nor published.
    assert_eq!(m.courts[0].read_own_kept(&m.keys[0], tick(12, [6, 0, 0, 0], 3), |_| false), Some(false));
    assert_eq!(m.courts[0].newest(&a).map(|r| r.tick.h), Some(11));

    // The record's bytes read back to the same values.
    let bytes = rec.encode().expect("a record writes");
    let back = LastSigned::decode(&bytes).expect("reads back");
    assert_eq!(back, rec);
    assert_eq!(back.hash, rec.reading.hash(&ORIGIN).expect("hashes"));
    let mut cut = bytes.clone();
    cut.pop();
    assert_eq!(LastSigned::decode(&cut), None, "a record cut short is none");
    let mut long = bytes.clone();
    long.push(0);
    assert_eq!(LastSigned::decode(&long), None, "a byte past the record is none");
    let mut altered = bytes.clone();
    if let Some(b) = altered.get_mut(106) {
        *b ^= 1;
    }
    assert_eq!(LastSigned::decode(&altered), None, "an altered tick does not verify");
    let mut version = bytes.clone();
    if let Some(b) = version.get_mut(4) {
        *b = 2;
    }
    assert_eq!(LastSigned::decode(&version), None, "another version is none");
    // On disk, as the home keeps it: written durably, read back by value.
    let dir = std::env::temp_dir().join(format!("mycelium-keeper-record-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a scratch home");
    let path = dir.join("keeper");
    assert_eq!(record::read(&path), Recorded::None, "a keeper that never signed here");
    assert!(record::write(&path, &rec));
    assert_eq!(record::read(&path), Recorded::Last(Box::new(rec.clone())));
    std::fs::write(&path, &cut).expect("writes");
    assert!(matches!(record::read(&path), Recorded::Unreadable(_)), "a record that does not read is refused, never none");
    let _ = std::fs::remove_dir_all(&dir);

    // THE RESTART: a fresh line, resumed from the record.
    let mut restarted = Keepers::new(ORIGIN, a);
    let elsewhere = Keepers::new(Origin { genesis: [1; 32], epoch: 146 }, a);
    assert!(!elsewhere.clone().resume(&back), "a record of another origin is no floor there");
    assert!(!Keepers::new(ORIGIN, vk(&m.keys[1])).resume(&back), "nor another keeper's");
    assert!(restarted.resume(&back));
    // At or below the last signed height, nothing is signed — with the
    // measure moved or not.
    for h in [9, 10, 11] {
        assert_eq!(restarted.read_own(&m.keys[0], tick(h, [99, 0, 0, 0], 99)), Some(false), "h {h} is at or below 11");
    }
    // Above it, the reading names the record's reading as its parent.
    assert_eq!(restarted.read_own(&m.keys[0], tick(13, [7, 0, 0, 0], 4)), Some(true));
    let ring = restarted.own_ring();
    let newest = ring.iter().next_back().expect("a reading");
    assert_eq!((newest.tick.h, newest.parent), (13, back.hash));
    // A peer holding the line extends it; it does not re-anchor.
    assert_eq!(m.courts[1].admit(a, ring.clone()), Ok(Admitted::Extended(1)));
    assert_eq!(m.courts[1].newest(&a).map(|r| r.tick.h), Some(13));
    // A peer that missed the last reading before the restart takes it from
    // the ring and extends too: the ring carries it.
    let mut late = Keepers::new(ORIGIN, vk(&m.keys[2]));
    let before: Vec<TickReading> = m.courts[0].own_ring().into_iter().take(1).collect();
    assert_eq!(late.admit(a, before), Ok(Admitted::Anchored));
    assert_eq!(late.admit(a, ring), Ok(Admitted::Extended(2)));
}

/// A STRICT MAJORITY OF THE KEEPER SET (ruled 2026-10-09): with
/// four keepers a point needs three — its own keeper and two others citing
/// it. One citer, enough at three keepers, finalises nothing at four.
#[test]
fn final_of_needs_three_of_four_keepers() {
    let keys = [key(1), key(2), key(3), key(4)];
    let mut ks: Vec<Keepers> = keys.iter().map(|k| Keepers::new(ORIGIN, vk(k))).collect();
    let hear = |ks: &mut Vec<Keepers>, to: usize, from: usize| {
        let ring = ks[from].own_ring();
        ks[to].admit(vk(&keys[from]), ring)
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
    assert_eq!(ks[0].final_of(&vk(&keys[0])), None, "a and b are two of four: no majority");
    // c hears a and cites it: a, b, c are three of four.
    assert_eq!(hear(&mut ks, 2, 0), Ok(Admitted::Anchored));
    assert!(ks[2].read_own(&keys[2], tick(14, [1, 0, 6, 0], 2)).expect("writes"));
    // a already holds c's line: the new reading extends it.
    assert!(matches!(hear(&mut ks, 0, 2), Ok(Admitted::Extended(_))), "c's line grows by its new reading");
    let a10 = TPoint { origin: ORIGIN, h: 10, lanes: [4, 0, 0, 0], k: 1 };
    assert_eq!(ks[0].final_of(&vk(&keys[0])), Some(a10), "three of four is a majority");
}
