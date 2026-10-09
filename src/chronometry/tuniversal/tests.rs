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

#[test]
fn a_point_prints_and_parses_back_exactly() {
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

#[test]
fn three_keepers_exchange_and_a_point_is_final_only_on_two() {
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

#[test]
fn a_forged_signature_and_a_broken_parent_are_refused() {
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

#[test]
fn a_disagreement_is_an_exact_residual_and_never_averaged() {
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

#[test]
fn with_two_keepers_silent_the_final_point_does_not_advance() {
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

#[test]
fn the_shelf_advances_without_the_kernel_and_wakes_a_parked_reader() {
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

#[test]
fn reset_branch_and_superpose_cite_the_point() {
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
