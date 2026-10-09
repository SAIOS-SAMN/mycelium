// 78.146 — THE MIND AT ITS SEAT: a percept's records held as series and read
// back whole; a quantity's change the slide and the scale of its own line,
// composing as the flows do; a periodic record's orbit read with its exact
// period; and, against a fake house on one court's relay, a mind that follows
// the counsel only until it has read what its calls do, then calls the
// function whose outcome moves the record the counsel's worth favours.
// 78.148: with no counsel, its own arguments explored and the dominant call
// chosen under its own valuation; the listing perceived; the seat taken
// again when lost; a versor series carried by its exact carrier.

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
    Percept { domain: vk(&domain()), sub_origin: None, seat: 7, mind: vk(&mind()), local_time, seq, records, point: None }
}

/// A SEAT KEEPS ITS BOUND (a sink keeps its bound; the 2026-10-09 growth of
/// the xylarium seat minds to 400 MB): whatever a house sends, the values a
/// seat holds across all its series write within one frame, each series'
/// oldest dropped and counted, and a series still reads its orbit on what it
/// holds.
#[test]
fn a_seat_holds_one_frame_of_values_across_its_series() {
    let mut m = SeatMind::new(vk(&domain()), 7);
    // Many series of wide quantities, each value some hundreds of bytes as it
    // writes: the seat's frame, shared among them, holds a few beats of each.
    let wide = BigInt::from(3u8).pow(2_000);
    // A hundred series share the frame: about nineteen values each, enough
    // to read a period of three (seven beats) and fewer than the forty sent.
    let kinds = 100u16;
    let beats = 40u64;
    for beat in 1..=beats {
        let records = (0..kinds)
            .map(|k| Record::Plain {
                kind: k,
                args: vec![Arg::BigQ { num: &wide + BigInt::from(beat % 3 + u64::from(k)), den: num_bigint::BigUint::from(1u8) }],
            })
            .collect();
        m.perceive(&percept(beat, Arg::Q { num: beat as i64, den: 1 }, records));
    }
    let mut held = 0usize;
    for (_, s) in m.every_series() {
        assert!(s.dropped > 0, "past the seat's share the oldest are dropped, and counted");
        for seen in s.seen() {
            let mut out = Vec::new();
            seen.value.put(&mut out).expect("written");
            held += out.len();
        }
        assert_eq!(s.orbit().map(|r| r.period()), Some(3), "the orbit is read on what is held");
    }
    assert!(held <= crate::network::wire::frame_budget(), "one frame across the seat: {held}");
}

/// 78.156 — THE SUB-INSTANCE LOCATED AT (TPoint, u), ITS DILATION EXACT:
/// between two located percepts the world's axis advance against the final
/// point's tick advance, `du/dh` as an exact rational; a final point that
/// stood leaves the ratio unread (the world moved, TUniversal did not); a
/// percept with no point locates nothing and the reading stands; two origins
/// share no clock.
#[test]
fn a_seats_experienced_dilation_is_read_exactly() {
    use crate::chronometry::tuniversal::{Origin, TPoint};
    let origin = Origin { genesis: [6; 32], epoch: 156 };
    let at = |h: u64| TPoint { origin, h, lanes: [1, 0, 0, 0], k: 1 };
    let located = |seq: u64, u: Arg, h: u64| Percept { point: Some(at(h)), ..percept(seq, u, vec![]) };
    let mut m = SeatMind::scoped(vk(&domain()), Some([3; 32]), 7);
    assert_eq!(m.sub_origin, Some([3; 32]));
    let r = m.perceive(&located(1, Arg::Q { num: 1, den: 1 }, 100));
    assert_eq!((r.dilation, m.dilation()), (None, None), "one location: no advance yet");
    assert_eq!(m.located(), Some(&Located { point: at(100), u: q(1, 1) }));
    // u from 1 to 3/2 while h rose 100 → 115: du/dh = (1/2)/15 = 1/30.
    let r = m.perceive(&located(2, Arg::Q { num: 3, den: 2 }, 115));
    let want = Dilation { du: q(1, 2), dh: 15, ratio: Some(q(1, 30)) };
    assert_eq!(r.dilation.as_ref(), Some(&want));
    assert_eq!(m.dilation(), Some(&want));
    assert!(r.to_string().contains("dilation du 1/2 dh 15 du/dh 1/30"), "the perceived line carries it: {r}");
    // The final point stood: the world moved and the ratio is unread.
    let r = m.perceive(&located(3, Arg::Count(2), 115));
    assert_eq!(r.dilation, Some(Dilation { du: q(1, 2), dh: 0, ratio: None }));
    // No point: nothing located, the reading stands.
    let r = m.perceive(&percept(4, Arg::Count(9), vec![]));
    assert_eq!((r.dilation, m.located().map(|l| l.u.clone())), (None, Some(q(2, 1))));
    assert_eq!(m.dilation(), Some(&Dilation { du: q(1, 2), dh: 0, ratio: None }));
    // Another origin: no common clock.
    let elsewhere = Percept { point: Some(TPoint { origin: Origin { genesis: [7; 32], epoch: 1 }, ..at(200) }), ..percept(5, Arg::Count(3), vec![]) };
    assert_eq!(m.perceive(&elsewhere).dilation, None);
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
    let changes: Vec<Change> = s.changes();
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
    let seats = Seats { domain: d, sub_origin: None, epoch: 1, seats: vec![Seat { seat: 7, role: Role::Steward, on: [6; 32], beat: Arg::Q { num: 1, den: 1 }, fns }] };
    k.seat_post(&Post::Seats(seats).sign(&house()).expect("writes")).expect("listed");
    k.seat_post(&Post::Take(Take { domain: d, sub_origin: None, seat: 7, mind: me, nonce: [2; 32] }).sign(&mind()).expect("writes")).expect("relayed");
    k.seat_post(&Post::Taken(Taken { domain: d, sub_origin: None, seat: 7, mind: me, ok: true, reason: 0 }).sign(&house()).expect("writes")).expect("relayed");
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
        let raw = k.seat_mail(&ReadMail::sign(&mind(), after, [0; 32])).expect("signed").mail;
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
                        let call = Post::Call(Call { domain: d, sub_origin: None, seat: 7, mind: me, nonce: [nonce; 32], fn_id: c.fn_id, args: c.args.clone() }).sign(&mind()).expect("writes");
                        let reference = call.reference().expect("writes");
                        k.seat_post(&call).expect("relayed");
                        m.called(c.fn_id, c.args.clone(), reference);
                    }
                    decisions.push(decision);
                }
            }
        }
        // The house's stroke: each call applied and answered by its reference.
        for piece in k.seat_mail(&ReadMail::sign(&house(), house_after, [0; 32])).expect("signed").mail {
            house_after = piece.seq;
            let s = piece.post().expect("reads");
            let Post::Call(c) = &s.post else { continue };
            x += match c.fn_id {
                1 => 1,
                _ => -1,
            };
            let outcome = Outcome { domain: d, sub_origin: None, seat: 7, mind: me, call: s.reference().expect("writes"), applied: true, reason: 0, deed: [8; 32] };
            k.seat_post(&Post::Outcome(outcome).sign(&house()).expect("writes")).expect("relayed");
        }
    }
    (decisions, x)
}

/// 78.146 — A TASK BEFORE THE SEAT, ITS STEPS READ EXACTLY: a kind-4 record
/// `[Key(citation)]` with no counsel puts the task before the seat (with
/// counsel it does not); a rule the seam speaks as each D4 motion (and a hue
/// move) reads back as that kind's count in the house's order and the hue
/// count `0x100 + 16·from + to`; a hue map alone is its hue counts; the
/// outputs cross as the solve act writes grids.
#[test]
fn a_task_before_the_seat_is_read_as_its_steps() {
    use crate::client::sdk::rule::{sentence, Step};
    use crate::client::sdk::seat_mind::task::*;
    use crate::transducer::family::NoCrystal;
    let task = Record::Plain { kind: KIND_TASK, args: vec![Arg::Key([7; 32])] };
    assert_eq!(tasks_before(&percept(1, Arg::Count(1), vec![task.clone()])), vec![[7u8; 32]], "wire v1.2: the citation, a Key");
    let raw = Record::Plain { kind: KIND_TASK, args: vec![Arg::Bytes(vec![7, 7, 7])] };
    assert!(tasks_before(&percept(1, Arg::Count(1), vec![raw])).is_empty(), "a raw id is no citation");
    let counsel = Record::Counsel { fn_id: 2, args: vec![], worth: Arg::Q { num: 1, den: 1 }, cost: Arg::Q { num: 0, den: 1 } };
    assert!(tasks_before(&percept(2, Arg::Count(2), vec![task, counsel])).is_empty(), "a counselled percept is the house's own mind");
    use crate::client::sdk::rule::Symmetry;
    // The symmetry read off by index, as a grid is read (the fixture's
    // expectation, never the seam's construction), and a hue map.
    let apply = |kind: Symmetry, g: &crate::arc::Grid| -> crate::arc::Grid {
        let (h, w) = (g.len(), g[0].len());
        match kind {
            Symmetry::Rotate90 => (0..w).map(|c| (0..h).rev().map(|r| g[r][c]).collect()).collect(),
            Symmetry::Rotate180 => g.iter().rev().map(|row| row.iter().rev().copied().collect()).collect(),
            Symmetry::Rotate270 => (0..w).rev().map(|c| (0..h).map(|r| g[r][c]).collect()).collect(),
            Symmetry::FlipH => g.iter().map(|row| row.iter().rev().copied().collect()).collect(),
            Symmetry::FlipV => g.iter().rev().cloned().collect(),
            Symmetry::Transpose => (0..w).map(|c| (0..h).map(|r| g[r][c]).collect()).collect(),
        }
    };
    let recolour = |from: u8, to: u8, g: &crate::arc::Grid| -> crate::arc::Grid {
        g.iter().map(|row| row.iter().map(|&h| if h == from { to } else { h }).collect()).collect()
    };
    let x: crate::arc::Grid = vec![vec![1, 2, 0], vec![1, 0, 0], vec![0, 0, 0]];
    for (i, kind) in D4.iter().enumerate() {
        let Some(step) = Step::grid_motion(*kind, (3, 3), (3, 3)) else { panic!("{kind:?} fits a square") };
        let family = vec![(x.clone(), recolour(1, 3, &apply(*kind, &x)))];
        let rule = match sentence(&[step, Step::Hue(vec![(1, 3)])], &family, &NoCrystal) {
            Ok(r) => r,
            Err(e) => panic!("{kind:?} mints: {e:?}"),
        };
        assert_eq!(grid_steps(&rule, (3, 3), (3, 3)), Ok(vec![i as u64 + 1, hue_step(1, 3)]), "{kind:?}");
    }
    let hue_only = match sentence(&[Step::Hue(vec![(2, 5)])], &[(x.clone(), recolour(2, 5, &x))], &NoCrystal) {
        Ok(r) => r,
        Err(e) => panic!("the hue map mints: {e:?}"),
    };
    let shape = |g: &crate::arc::Grid| (g.len(), g[0].len());
    assert_eq!(grid_steps(&hue_only, shape(&x), shape(&x)), Ok(vec![0x100 + 16 * 2 + 5]));
    let outs = vec![x.clone(), vec![vec![4]]];
    let args = solve_args(&hue_only, &outs, &[9]).expect("writes");
    let [Arg::Bytes(tape), Arg::Bytes(grids), Arg::Count(9)] = args.as_slice() else { panic!("{args:?}") };
    assert_eq!(crate::anastome::dialogue::decode_grids(grids), Some(outs), "the outputs read back as the solve act writes them");
    let mut p = 0;
    assert_eq!(crate::transducer::element::Element::from_bytes(tape, &mut p).map(|e| e.address()), Some(hue_only.address()), "the tape is the rule");
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

/// The listing the kernel's relay serves, read as the mind reads it: its
/// seat's functions.
fn listed_fns(k: &Kernel, seat: u32) -> (u64, Vec<SeatFn>) {
    let body = k.seat_listing(&vk(&domain()), None).expect("a listing is held");
    let listing = decode_listing_reply(&body, None).expect("decodes").expect("one SEATS");
    crate::client::sdk::seat::seat_fns(&listing, seat).expect("the seat is listed")
}

fn list(k: &mut Kernel, epoch: u64, fns: Vec<SeatFn>) {
    let seats = Seats { domain: vk(&domain()), sub_origin: None, epoch, seats: vec![Seat { seat: 7, role: Role::Steward, on: [6; 32], beat: Arg::Q { num: 1, den: 1 }, fns }] };
    k.seat_post(&Post::Seats(seats).sign(&house()).expect("writes")).expect("listed");
}

/// 78.148 — THE FAKE HOUSE OF ONE COUNT FUNCTION, FOCUS (fn 2), never told
/// to the mind: value 0 raises record 10, value 1 raises record 11, value 2
/// raises records 10, 11 and 12; 3 and above are BAD ARGUMENTS. The house
/// counsels nothing. The mind reads the listing off the relay, then each
/// beat its mail; it perceives, decides and calls; the house applies or
/// refuses each call and answers it. Returns the decisions, the values the
/// mind called, the records, and the mind.
fn play_own(beats: u64) -> (Vec<Decision>, Vec<u64>, [i64; 3], SeatMind) {
    let mut k = founded();
    let d = vk(&domain());
    let me = vk(&mind());
    list(&mut k, 1, vec![SeatFn { id: 2, tags: vec![TAG_COUNT] }]);
    k.seat_post(&Post::Take(Take { domain: d, sub_origin: None, seat: 7, mind: me, nonce: [2; 32] }).sign(&mind()).expect("writes")).expect("relayed");
    k.seat_post(&Post::Taken(Taken { domain: d, sub_origin: None, seat: 7, mind: me, ok: true, reason: 0 }).sign(&house()).expect("writes")).expect("relayed");
    let mut m = SeatMind::new(d, 7);
    let (epoch, fns) = listed_fns(&k, 7);
    assert_eq!((epoch, fns.clone()), (1, vec![SeatFn { id: 2, tags: vec![TAG_COUNT] }]), "the listing reads back whole");
    m.perceive_listing(&fns);
    let (mut after, mut house_after, mut x, mut nonce) = (0u64, 0u64, [0i64; 3], 0u8);
    let (mut decisions, mut values) = (Vec::new(), Vec::new());
    for beat in 1..=beats {
        let records = (0..3u16).map(|i| Record::Plain { kind: 10 + i, args: vec![Arg::Q { num: x[usize::from(i)], den: 1 }] }).collect();
        k.seat_post(&Post::Percept(percept(beat, Arg::Q { num: beat as i64, den: 1 }, records)).sign(&house()).expect("writes")).expect("relayed");
        let raw = k.seat_mail(&ReadMail::sign(&mind(), after, [0; 32])).expect("signed").mail;
        for piece in mind_mail(me, raw).expect("typed") {
            match piece {
                MindMail::Taken(seq, _) => after = seq,
                MindMail::Outcome(seq, o) => {
                    after = seq;
                    assert!(m.outcome(&o).is_some(), "the outcome names the call the mind awaits");
                }
                MindMail::Percept(seq, p) => {
                    after = seq;
                    m.perceive(&p);
                    let decision = m.decide();
                    if let Decision::Call(c) = &decision {
                        let [Arg::Count(v)] = c.args.as_slice() else { panic!("one Count: {decision}") };
                        values.push(*v);
                        nonce += 1;
                        let call = Post::Call(Call { domain: d, sub_origin: None, seat: 7, mind: me, nonce: [nonce; 32], fn_id: c.fn_id, args: c.args.clone() }).sign(&mind()).expect("writes");
                        let reference = call.reference().expect("writes");
                        k.seat_post(&call).expect("relayed");
                        m.called(c.fn_id, c.args.clone(), reference);
                    }
                    decisions.push(decision);
                }
            }
        }
        for piece in k.seat_mail(&ReadMail::sign(&house(), house_after, [0; 32])).expect("signed").mail {
            house_after = piece.seq;
            let s = piece.post().expect("reads");
            let Post::Call(c) = &s.post else { continue };
            let raised: &[usize] = match (c.fn_id, c.args.as_slice()) {
                (2, [Arg::Count(0)]) => &[0],
                (2, [Arg::Count(1)]) => &[1],
                (2, [Arg::Count(2)]) => &[0, 1, 2],
                _ => &[],
            };
            raised.iter().for_each(|i| x[*i] += 1);
            let (applied, reason) = match raised.is_empty() {
                true => (false, REASON_BAD_ARGUMENTS),
                false => (true, 0),
            };
            let outcome = Outcome { domain: d, sub_origin: None, seat: 7, mind: me, call: s.reference().expect("writes"), applied, reason, deed: [8; 32] };
            k.seat_post(&Post::Outcome(outcome).sign(&house()).expect("writes")).expect("relayed");
        }
    }
    (decisions, values, x, m)
}

/// 78.148 — ITS OWN ARGUMENTS, NO SCRIPT. With no counsel the mind explores
/// FOCUS's one Count in increasing order — 0, 1, 2 — and 3, which the house
/// refuses BAD ARGUMENTS: the refusal bounds the domain below 3 and 3 is
/// never called again. Every value tried, it reads under its own valuation
/// (REACH: the records its calls moved, the way they moved them) that 2
/// moves records 10, 11 and 12 at least as far as 0 and 1 on every one,
/// further on one — and calls 2 from then on, saying so.
#[test]
fn the_mind_explores_its_own_arguments_and_calls_the_dominant() {
    let (decisions, values, x, m) = play_own(9);
    assert_eq!(values, vec![0, 1, 2, 3, 2, 2, 2, 2, 2], "explored upward, the refused value never again, then the dominant");
    let explored: Vec<Option<(usize, u64)>> = decisions
        .iter()
        .take(4)
        .map(|d| match d {
            Decision::Call(Choice { why: Why::Exploring { at, .. }, followed: false, .. }) => *at,
            other => panic!("exploring: {other}"),
        })
        .collect();
    assert_eq!(explored, vec![None, Some((0, 1)), Some((0, 2)), Some((0, 3))], "the least first, then the argument upward");
    assert_eq!(m.repertoire().iter().map(|o| (o.id, o.bounds())).collect::<Vec<_>>(), vec![(2, vec![Some(3)])], "BAD ARGUMENTS at 3 bounds the domain");
    let refused = m.call_experience(2, &[Arg::Count(3)]).expect("tried");
    assert_eq!((refused.applied, refused.refused.clone(), refused.read), (0, vec![(REASON_BAD_ARGUMENTS, 1)], 0));
    let at = |kind| Locator { kind, keys: vec![], occurrence: 0, position: 0 };
    for d in &decisions[4..] {
        let Decision::Call(c) = d else { panic!("{d}") };
        let Why::Reached { valuation, values, over, along } = &c.why else { panic!("{d}") };
        assert_eq!((*valuation, values.clone()), (Valuation::Reach, vec![2]), "{d}");
        assert_eq!(over, &vec![(2, vec![0]), (2, vec![1])], "{d}");
        assert_eq!(along, &vec![(at(10), Direction::Raise, q(1, 1)), (at(11), Direction::Raise, q(1, 1)), (at(12), Direction::Raise, q(1, 1))], "{d}");
        assert!(d.to_string().contains("valuation REACH"), "the beat line names its valuation: {d}");
    }
    assert_eq!(m.reach().len(), 3, "reach favours the three records the calls moved");
    assert_eq!(x, [1 + 1 + 5, 1 + 1 + 5, 1 + 5], "0 once, 1 once, 2 six times; 3 applied never");
}

/// 78.148 — THE LISTING IS THE MIND'S REPERTOIRE: a function whose arguments
/// are not all Counts or Flags is not the mind's to explore (fn 1, two
/// Bytes); a Count function is (fn 2) — and a Flag one is two values from
/// the start. A call refused NO SUCH FUNCTION takes the function out; the
/// listing read again restores it.
#[test]
fn the_listing_is_perceived_as_the_minds_repertoire() {
    let mut k = founded();
    list(&mut k, 3, vec![SeatFn { id: 1, tags: vec![6, 6] }, SeatFn { id: 2, tags: vec![TAG_COUNT] }, SeatFn { id: 5, tags: vec![TAG_FLAG] }]);
    let (epoch, fns) = listed_fns(&k, 7);
    assert_eq!(epoch, 3);
    let mut m = SeatMind::new(vk(&domain()), 7);
    assert_eq!(m.decide(), Decision::NothingToCall, "no counsel and no repertoire: nothing to call");
    m.perceive_listing(&fns);
    let held: Vec<(u16, bool, Vec<Option<u64>>)> = m.repertoire().iter().map(|o| (o.id, o.domains.is_some(), o.bounds())).collect();
    assert_eq!(held, vec![(1, false, vec![]), (2, true, vec![None]), (5, true, vec![Some(2)])]);
    m.perceive(&percept(1, Arg::Count(1), vec![]));
    let Decision::Call(c) = m.decide() else { panic!("the mind explores") };
    assert_eq!((c.fn_id, c.args.clone(), c.followed), (2, vec![Arg::Count(0)], false), "fn 1 skipped, fn 2 at its least");
    m.called(2, c.args, [1; 32]);
    let refusal = Outcome { domain: vk(&domain()), sub_origin: None, seat: 7, mind: vk(&mind()), call: [1; 32], applied: false, reason: REASON_NO_SUCH_FUNCTION, deed: [0; 32] };
    assert_eq!(m.outcome(&refusal), Some(2));
    assert_eq!(m.repertoire().iter().map(|o| o.id).collect::<Vec<_>>(), vec![1, 5], "no such function: gone until the listing is read again");
    m.perceive(&percept(2, Arg::Count(2), vec![]));
    let Decision::Call(c) = m.decide() else { panic!("the flag function is explored") };
    assert_eq!((c.fn_id, c.args), (5, vec![Arg::Flag(false)]));
    m.perceive_listing(&fns);
    assert_eq!(m.repertoire().iter().map(|o| o.id).collect::<Vec<_>>(), vec![1, 2, 5], "the listing read again restores it");
}

/// 78.148 — THE SEAT HELD AGAIN. A TAKEN refused lets the take go and the
/// next stroke takes again; held, the listing is to be read; a call refused
/// NOT THE HOLDER lets it go, NO SUCH FUNCTION asks the listing; a take no
/// TAKEN answers is posted again after a full aperture cycle of strokes —
/// and, on one court's relay, the house reads the second take.
#[test]
fn the_seat_is_taken_again_when_lost() {
    use crate::client::sdk::seat::SeatHold;
    let mut k = founded();
    let (d, me) = (vk(&domain()), vk(&mind()));
    list(&mut k, 1, vec![SeatFn { id: 2, tags: vec![TAG_COUNT] }]);
    let mut h = SeatHold::default();
    let mut after = 0u64;
    let mut nonce = 0u8;
    let mut take = |k: &mut Kernel, h: &mut SeatHold| {
        assert!(h.stroke(), "the stroke posts the take");
        nonce += 1;
        k.seat_post(&Post::Take(Take { domain: d, sub_origin: None, seat: 7, mind: me, nonce: [nonce; 32] }).sign(&mind()).expect("writes")).expect("relayed");
        h.take_posted();
    };
    take(&mut k, &mut h);
    k.seat_post(&Post::Taken(Taken { domain: d, sub_origin: None, seat: 7, mind: me, ok: false, reason: 1 }).sign(&house()).expect("writes")).expect("relayed");
    for piece in mind_mail(me, k.seat_mail(&ReadMail::sign(&mind(), after, [0; 32])).expect("signed").mail).expect("typed") {
        let MindMail::Taken(seq, t) = piece else { panic!("a TAKEN") };
        after = seq;
        assert!(!h.taken(t.ok), "refused: let go, no listing");
    }
    assert_eq!(h, SeatHold::default(), "the take let go");
    take(&mut k, &mut h);
    let takes = k.seat_mail(&ReadMail::sign(&house(), 0, [0; 32])).expect("signed").mail.iter().filter(|m| m.sub == SUB_TAKE).count();
    assert_eq!(takes, 2, "the house reads the second take");
    k.seat_post(&Post::Taken(Taken { domain: d, sub_origin: None, seat: 7, mind: me, ok: true, reason: 0 }).sign(&house()).expect("writes")).expect("relayed");
    for piece in mind_mail(me, k.seat_mail(&ReadMail::sign(&mind(), after, [0; 32])).expect("signed").mail).expect("typed") {
        let MindMail::Taken(_, t) = piece else { panic!("a TAKEN") };
        assert!(h.taken(t.ok), "held: the listing is to be read");
    }
    assert!(h.held && !h.stroke(), "held: the stroke reads its mail");
    assert_eq!(listed_fns(&k, 7).1, vec![SeatFn { id: 2, tags: vec![TAG_COUNT] }]);
    assert!(h.outcome(false, REASON_NO_SUCH_FUNCTION) && h.held, "no such function: the listing again, the seat still held");
    assert!(!h.outcome(false, REASON_BAD_ARGUMENTS) && h.held, "bad arguments: information for the mind, the seat still held");
    assert!(!h.outcome(false, REASON_NOT_THE_HOLDER) && !h.held, "not the holder: let go");
    assert!(h.stroke(), "and taken again");
    h.take_posted();
    for _ in 1..SeatHold::patience() {
        assert!(!h.stroke(), "an unanswered take waits within the aperture cycle");
    }
    assert!(h.stroke(), "unanswered a full aperture cycle: posted again");
    assert_eq!(SeatHold::patience(), u64::from(crate::physics::aperture_cycle()));
}

/// 78.148 — A VERSOR'S CHANGE IS THE VERSOR THAT CARRIES IT (the operator's
/// field law): reflections alternating — odd versors, a bell's pulse — are
/// carried by `e₂e₁` and `e₁e₂` in turn, an orbit of period 2; a steady
/// motion's powers are carried by the motion itself every beat, an orbit of
/// period 1 that is not standing still; a held versor is fixed; a null
/// point is no versor and its carrier is refused. Exact, never numbers.
#[test]
fn a_versor_series_changes_by_its_exact_carrier() {
    use crate::transducer::cga::{Mv, E1, E2};
    let e = |mask| Mv::blade(mask, q(1, 1)).widened(3);
    let (e1, e2) = (e(E1), e(E2));
    let c = carrier(&e1, &e2).expect("two reflections");
    assert!(c.geo(&e1).sub(&e2).is_zero(), "the carrier carries e1 onto e2 exactly");
    assert!(c.sub(&e2.geo(&e1)).is_zero(), "and it is e2 e1");
    assert_eq!(carrier(&point_at(&[q(1, 1), q(0, 1), q(0, 1)]), &e1).map(|_| ()), None, "a null point is no versor: refused");
    let r = translator_at(&[q(1, 2), q(0, 1), q(0, 1)]);
    let mut m = SeatMind::new(vk(&domain()), 7);
    let mut power = r.clone();
    for beat in 1..=6u64 {
        let bell = match beat % 2 {
            1 => e1.clone(),
            _ => e2.clone(),
        };
        let records = vec![
            Record::Plain { kind: 20, args: vec![Arg::Mv(bell)] },
            Record::Plain { kind: 21, args: vec![Arg::Mv(power.clone())] },
            Record::Plain { kind: 22, args: vec![Arg::Mv(e1.clone())] },
        ];
        m.perceive(&percept(beat, Arg::Q { num: beat as i64, den: 1 }, records));
        power = power.geo(&r);
    }
    let at = |kind| Locator { kind, keys: vec![], occurrence: 0, position: 0 };
    let bell = m.series(&at(20)).expect("held");
    assert_eq!(bell.orbit(), Some(Return::Orbit { period: 2, span: Some(q(2, 1)) }), "the pulse returns every two beats");
    let carried: Vec<Mv> = bell.carriers();
    assert_eq!(carried.len(), 5);
    assert!(carried.iter().all(|c| c.sub(&e2.geo(&e1)).is_zero() || c.sub(&e1.geo(&e2)).is_zero()));
    let steady = m.series(&at(21)).expect("held");
    let powers = steady.carriers();
    assert_eq!(powers.len(), steady.seen().count() - 1, "every power carried");
    assert!(powers.iter().all(|c| c.sub(&r).is_zero()), "each power carried by the motion itself");
    assert_eq!(steady.orbit(), Some(Return::Orbit { period: 1, span: Some(q(1, 1)) }), "a steady carrier is an orbit, not a standstill");
    assert_eq!(m.series(&at(22)).expect("held").orbit(), Some(Return::Fixed), "a held versor is carried by one: fixed");
}

/// A FUNCTION A HOUSE COUNSELLED IS NEVER PROBED BLIND (barren-realms,
/// 2026-10-09: `handle` closes every throat; `forge` spends real energy).
/// Counselled once, on a later beat with no counsel the mind explores only
/// its other functions.
#[test]
fn a_counselled_function_is_never_explored_blind() {
    let mut m = SeatMind::new(vk(&domain()), 7);
    m.perceive_listing(&[SeatFn { id: 13, tags: vec![TAG_COUNT] }, SeatFn { id: 2, tags: vec![TAG_COUNT] }]);
    let counsel = Record::Counsel { fn_id: 13, args: vec![Arg::Count(1)], worth: Arg::Count(1), cost: Arg::Count(0) };
    m.perceive(&percept(1, Arg::Count(1), vec![counsel]));
    m.perceive(&percept(2, Arg::Count(2), vec![]));
    match m.decide() {
        Decision::Call(c) => assert_eq!(c.fn_id, 2, "the uncounselled function is explored, never fn 13: {c:?}"),
        other => panic!("a call: {other}"),
    }
}
