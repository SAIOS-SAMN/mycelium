// 78.146 — THE MIND AT ITS SEAT: a percept's records held as series and read
// back whole; a quantity's change the slide and the scale of its own line,
// composing as the flows do; a periodic record's orbit read with its exact
// period; and, against a fake house on one court's relay, a mind that follows
// the counsel only until it has read what its calls do, then calls the
// function whose outcome moves the record the counsel's worth favours.

use crate::anastome::dialogue::domain::{DomainGenesis, Founded, Reservation};
use crate::anastome::dialogue::seat::*;
use crate::client::sdk::seat::{mind_mail, MindMail};
use crate::client::sdk::seat_mind::*;
use crate::server::cognition::Kernel;
use crate::transducer::cga::{point_at, translator_at};
use ed25519_dalek::SigningKey;
use num_bigint::BigInt;
use num_rational::BigRational as Q;

fn q(n: i64, d: i64) -> Q {
    Q::new(BigInt::from(n), BigInt::from(d))
}
fn key(b: u8) -> SigningKey {
    SigningKey::from_bytes(&[b; 32])
}
fn vk(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}
fn domain() -> SigningKey {
    key(4)
}
fn house() -> SigningKey {
    key(21)
}
fn tier() -> SigningKey {
    key(22)
}
fn mind() -> SigningKey {
    key(30)
}

fn percept(seq: u64, local_time: Arg, records: Vec<Record>) -> Percept {
    Percept { domain: vk(&domain()), seat: 7, mind: vk(&mind()), local_time, seq, records }
}

/// A SLIDE AND A SCALE ARE MOTIONS OF THE QUANTITY'S LINE: each carries the
/// one value's point onto the other's, and two of each compose as their
/// flow does — slides by sum, scales by product.
#[test]
fn a_change_is_a_slide_and_a_scale_of_the_line() {
    let c = Change::between(&q(3, 1), &q(7, 1)).expect("reads");
    assert_eq!(c.delta(), Some(q(4, 1)), "the slide stands at the difference");
    assert!(c.slide.versor().sandwich(&point_at(&[q(3, 1)])).projective_eq(&point_at(&[q(7, 1)])), "the slide carries 3 to 7");
    let s = c.scale.clone().expect("both above zero");
    assert_eq!(s.parameter(), Some(q(2, 5)), "the scale by 7/3 stands at (ρ−1)/(ρ+1)");
    assert!(s.versor().sandwich(&point_at(&[q(3, 1)])).projective_eq(&point_at(&[q(7, 1)])), "the scale carries 3 to 7");
    let a = Change::between(&q(1, 1), &q(3, 1)).expect("reads");
    let whole = Change::between(&q(1, 1), &q(7, 1)).expect("reads");
    assert_eq!(a.slide.then(&c.slide), Some(whole.slide.clone()), "slides compose by sum");
    assert_eq!(a.scale.expect("scale").then(&s), whole.scale, "scales compose by product");
    let across = Change::between(&q(-1, 2), &q(5, 1)).expect("reads");
    assert_eq!(across.delta(), Some(q(11, 2)));
    assert_eq!(across.scale, None, "no dilation carries a value across zero");
    let still = Change::between(&q(5, 1), &q(5, 1)).expect("reads");
    assert!(still.slide.returned() && still.scale.is_some_and(|s| s.returned()), "no change is the identity on both flows");
}

/// RECORDS → SERIES, READ BACK WHOLE: every non-key argument of every record
/// is one series, located by the record's kind, keys, occurrence and the
/// argument's position; its values read back as they were perceived, a
/// multivector coefficient for coefficient.
#[test]
fn records_are_held_as_series_and_read_back() {
    let mut m = SeatMind::new(vk(&domain()), 7);
    let unit = [9u8; 32];
    let motion = translator_at(&[q(1, 2), q(0, 1), q(2, 1)]);
    let beats: Vec<Vec<Record>> = (0..4i64)
        .map(|i| {
            vec![
                Record::Plain { kind: 5, args: vec![Arg::Key(unit), Arg::Q { num: i * 3, den: 2 }, Arg::Count(10 - i as u64)] },
                Record::Plain { kind: 5, args: vec![Arg::Key(unit), Arg::Flag(i % 2 == 0)] },
                Record::Plain { kind: 6, args: vec![Arg::Mv(motion.scale(&q(i + 1, 1)))] },
            ]
        })
        .collect();
    for (i, records) in beats.iter().enumerate() {
        let r = m.perceive(&percept(i as u64 + 1, Arg::Q { num: i as i64, den: 1 }, records.clone()));
        assert_eq!((r.beat, r.records, r.series), (i as u64 + 1, 3, 4));
    }
    let at = |kind, keys: Vec<[u8; 32]>, occurrence, position| Locator { kind, keys, occurrence, position };
    let values = |l: &Locator| -> Vec<Arg> { m.series(l).expect("held").seen().map(|s| s.value.clone()).collect() };
    for (l, i) in [(at(5, vec![unit], 0, 1), 1usize), (at(5, vec![unit], 0, 2), 2)] {
        let want: Vec<Arg> = beats.iter().map(|b| match &b[0] {
            Record::Plain { args, .. } => args[i].clone(),
            Record::Counsel { .. } => panic!("no counsel here"),
        }).collect();
        assert_eq!(values(&l), want, "{l} reads back as perceived");
        assert_eq!(Locator::of(&l.bytes().expect("writes")), Some(l.clone()), "a locator reads back from its key");
    }
    assert_eq!(values(&at(5, vec![unit], 1, 1)), (0..4).map(|i| Arg::Flag(i % 2 == 0)).collect::<Vec<_>>(), "the second record of one kind and key is its own occurrence");
    let held = values(&at(6, vec![], 0, 0));
    assert_eq!(held.len(), 4);
    for (i, a) in held.iter().enumerate() {
        let Arg::Mv(v) = a else { panic!("a multivector is held as one") };
        assert!(v.projective_eq(&motion), "one ray");
        assert_eq!(*a, Arg::Mv(motion.scale(&q(i as i64 + 1, 1))), "and its scale, coefficient for coefficient");
        if i > 0 {
            assert_ne!(*a, held[0], "a multivector is never compared only projectively");
        }
    }
    // The quantity's changes: each a slide by 3/2, composing to the whole run.
    let s = m.series(&at(5, vec![unit], 0, 1)).expect("held");
    let changes: Vec<&Change> = s.seen().filter_map(|x| x.change.as_ref()).collect();
    assert_eq!(changes.len(), 3);
    assert!(changes.iter().all(|c| c.delta() == Some(q(3, 2))));
    let composed = changes.iter().skip(1).try_fold(changes[0].slide.clone(), |a, c| a.then(&c.slide)).expect("one flow");
    assert_eq!(composed.parameter(), Some(q(9, 2)), "the run's slides compose to its whole difference");
}

/// AN ORBIT, WITH ITS EXACT PERIOD: a quantity cycling 0, 1, 3 returns every
/// three beats, a flag alternating every two, a constant stands still; the
/// record returns every lcm = six, and the period's span in the house's local
/// time is read off the beats (local time ½ a beat: three beats are 3/2).
#[test]
fn a_periodic_record_is_an_orbit_with_its_exact_period() {
    let mut m = SeatMind::new(vk(&domain()), 7);
    let cycle = [0i64, 1, 3];
    let mut last = Perceived::default();
    for beat in 1..=12u64 {
        let i = usize::try_from(beat).expect("small") % 3;
        let records = vec![Record::Plain {
            kind: 3,
            args: vec![Arg::Q { num: cycle[i], den: 1 }, Arg::Flag(beat % 2 == 0), Arg::Count(7)],
        }];
        last = m.perceive(&percept(beat, Arg::Q { num: beat as i64, den: 2 }, records));
        if beat < 4 {
            assert_eq!(m.series(&Locator { kind: 3, keys: vec![], occurrence: 0, position: 0 }).expect("held").orbit(), None, "no return is seen before it repeats");
        }
    }
    let at = |position| Locator { kind: 3, keys: vec![], occurrence: 0, position };
    assert_eq!(m.series(&at(0)).expect("held").orbit(), Some(Return::Orbit { period: 3, span: Some(q(3, 2)) }));
    assert_eq!(m.series(&at(1)).expect("held").orbit(), Some(Return::Orbit { period: 2, span: Some(q(1, 1)) }));
    assert_eq!(m.series(&at(2)).expect("held").orbit(), Some(Return::Fixed));
    assert_eq!(m.record_period(3, &[], 0), Some(6), "the record returns at the lcm");
    assert_eq!((last.series, last.fixed, last.orbits, last.periods.clone()), (3, 1, 2, vec![2, 3]));
    // A break in the cycle: the quantity no longer returns.
    let broken = vec![Record::Plain { kind: 3, args: vec![Arg::Q { num: 5, den: 1 }, Arg::Flag(true), Arg::Count(7)] }];
    m.perceive(&percept(13, Arg::Q { num: 13, den: 2 }, broken));
    assert_eq!(m.series(&at(0)).expect("held").orbit(), None, "a value off the orbit breaks it");
    assert_eq!(m.record_period(3, &[], 0), None);
}

/// A domain founded with two tiers, `house` its sitting reserve.
fn founded() -> Kernel {
    let mut k = Kernel::new();
    let mut tiers = vec![vk(&house()), vk(&tier())];
    tiers.sort_unstable();
    let g = DomainGenesis { domain: vk(&domain()), tiers };
    assert!(matches!(k.admit_domain(&g.sign(&domain())), Founded::Sealed { .. }));
    let r = Reservation { domain: vk(&domain()), tier: vk(&house()), nonce: [1; 32] }.sign(&domain());
    assert!(matches!(k.designate_reserve(&r), Founded::Reserved { now: true, .. }));
    k
}

/// THE FAKE HOUSE: seat 7, function 1 raises record 10's quantity by one and
/// function 2 lowers it by one; each beat it perceives the quantity and
/// counsels both, in the order and with the worths given. The mind reads its
/// mail off the relay, perceives, decides and calls; the house applies each
/// call and answers it. Returns the mind's decisions and the quantity at the
/// end.
fn play(counsel: [(u16, i64); 2], beats: u64) -> (Vec<Decision>, i64) {
    let mut k = founded();
    let d = vk(&domain());
    let me = vk(&mind());
    let fns = vec![SeatFn { id: 1, tags: vec![] }, SeatFn { id: 2, tags: vec![] }];
    let seats = Seats { domain: d, epoch: 1, seats: vec![Seat { seat: 7, role: Role::Steward, on: [6; 32], beat: Arg::Q { num: 1, den: 1 }, fns }] };
    k.seat_post(&Post::Seats(seats).sign(&house()).expect("writes")).expect("listed");
    k.seat_post(&Post::Take(Take { domain: d, seat: 7, mind: me, nonce: [2; 32] }).sign(&mind()).expect("writes")).expect("relayed");
    k.seat_post(&Post::Taken(Taken { domain: d, seat: 7, mind: me, ok: true, reason: 0 }).sign(&house()).expect("writes")).expect("relayed");
    let mut m = SeatMind::new(d, 7);
    let (mut after, mut house_after, mut x, mut nonce) = (0u64, 0u64, 0i64, 0u8);
    let mut decisions = Vec::new();
    for beat in 1..=beats {
        let mut records = vec![Record::Plain { kind: 10, args: vec![Arg::Q { num: x, den: 1 }] }];
        for (fn_id, worth) in counsel {
            records.push(Record::Counsel { fn_id, args: vec![], worth: Arg::Q { num: worth, den: 2 }, cost: Arg::Q { num: 1, den: 3 } });
        }
        k.seat_post(&Post::Percept(percept(beat, Arg::Q { num: beat as i64, den: 1 }, records)).sign(&house()).expect("writes")).expect("relayed");
        // The mind's stroke: its mail, typed by the SDK.
        let raw = k.seat_mail(&ReadMail::sign(&mind(), after, [0; 32])).expect("signed");
        for piece in mind_mail(me, raw).expect("typed") {
            match piece {
                MindMail::Taken(seq, t) => {
                    after = seq;
                    assert!(t.ok);
                }
                MindMail::Outcome(seq, o) => {
                    after = seq;
                    assert!(m.outcome(&o).is_some(), "the outcome names the call the mind awaits");
                }
                MindMail::Percept(seq, p) => {
                    after = seq;
                    m.perceive(&p);
                    let decision = m.decide();
                    if let Decision::Call(c) = &decision {
                        nonce += 1;
                        let call = Post::Call(Call { domain: d, seat: 7, mind: me, nonce: [nonce; 32], fn_id: c.fn_id, args: c.args.clone() }).sign(&mind()).expect("writes");
                        let reference = call.reference().expect("writes");
                        k.seat_post(&call).expect("relayed");
                        m.called(c.fn_id, c.args.clone(), reference);
                    }
                    decisions.push(decision);
                }
            }
        }
        // The house's stroke: each call applied and answered by its reference.
        for piece in k.seat_mail(&ReadMail::sign(&house(), house_after, [0; 32])).expect("signed") {
            house_after = piece.seq;
            let s = piece.post().expect("reads");
            let Post::Call(c) = &s.post else { continue };
            x += match c.fn_id {
                1 => 1,
                _ => -1,
            };
            let outcome = Outcome { domain: d, seat: 7, mind: me, call: s.reference().expect("writes"), applied: true, reason: 0, deed: [8; 32] };
            k.seat_post(&Post::Outcome(outcome).sign(&house()).expect("writes")).expect("relayed");
        }
    }
    (decisions, x)
}

fn called(d: &Decision) -> (u16, bool) {
    match d {
        Decision::Call(c) => (c.fn_id, c.followed),
        other => panic!("the mind called nothing: {other}"),
    }
}

/// THE MIND LEARNS ITS CALL. The house counsels function 2 first, at a worth
/// below zero, and function 1 at a worth above: a counsel-follower lowers the
/// record forever. The mind follows the counsel while it has read nothing;
/// reads that function 2 lowered the record the counsel's worth says to
/// raise; tries function 1, untried; and from then on calls function 1, which
/// dominates — against the counsel's first, and says why.
#[test]
fn the_mind_learns_to_call_what_raises_the_favoured_record() {
    let (decisions, x) = play([(2, -1), (1, 1)], 8);
    assert_eq!(decisions.len(), 8);
    assert_eq!(called(&decisions[0]), (2, true), "no experience: the counsel, followed");
    assert!(matches!(&decisions[0], Decision::Call(c) if c.why == Why::NoExperience), "{}", decisions[0]);
    assert_eq!(called(&decisions[1]), (1, false), "function 2 moved the record against its worth: function 1 is tried");
    assert!(matches!(&decisions[1], Decision::Call(c) if c.why == Why::Untried { against: vec![2] }), "{}", decisions[1]);
    for d in &decisions[2..] {
        assert_eq!(called(d), (1, false), "{d}");
        let Decision::Call(c) = d else { panic!("{d}") };
        let Why::Dominates { over, along } = &c.why else { panic!("{d}") };
        assert_eq!(over, &vec![2]);
        assert_eq!(along, &vec![(Locator { kind: 10, keys: vec![], occurrence: 0, position: 0 }, Direction::Raise, q(1, 1))]);
    }
    assert_eq!(x, 6, "one lowered, then seven raised");
}

/// THE SAME HOUSE, THE WORTH TURNED: the counsel's worth now says the
/// record's falling is what matters, so the mind learns to call function 2 —
/// the counsel's second.
#[test]
fn the_mind_follows_the_worth_not_the_counsels_order() {
    let (decisions, x) = play([(1, -1), (2, 1)], 6);
    assert_eq!(called(&decisions[0]), (1, true), "no experience: the counsel, followed");
    assert_eq!(called(&decisions[1]), (2, false));
    assert!(decisions[2..].iter().all(|d| called(d) == (2, false)), "function 2 dominates once read");
    assert_eq!(x, 1 - 5);
}
