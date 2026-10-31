// 26, the actions' physics — A VOICE SENDS ONLY A SOLVE IT CAN STAND BEHIND:
// the rule replayed on its own train exactly as the court will replay it on
// the withheld pairs; no rule, a rule its train refutes, or a rule the court
// already refused is kept home, each counted by name.

use crate::client::sdk::*;

fn grid(rows: &[&str]) -> Vec<Vec<u8>> {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// The domino painted 2 and carried two columns right, on two rows.
fn family() -> Vec<(crate::arc::Grid, crate::arc::Grid)> {
    vec![
        (grid(&["1100", "0000"]), grid(&["0022", "0000"])),
        (grid(&["0000", "1100"]), grid(&["0000", "0022"])),
    ]
}

/// A voice keeps home a solve its own train refutes: the rule replayed on a
/// train it does not speak is refused at home, as the court would refuse it.
#[test]
fn judge_on_keeps_home_solve_its_train_refutes() {
    let train = family();
    let rule = crate::transducer::family::extract_rule(&train, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("the family extracts");
    let spoken = crate::transducer::family::mint_sentence(&rule, &train, &crate::transducer::family::NoCrystal, 0)
        .expect("and mints");
    use crate::client::sdk::entity::perceive::{judge_on, Judge, Reading, Scene, Verdict};
    let entity = Mind::open("127.0.0.1:1", Voice::named("solver"));
    let outs = vec![grid(&["0000", "0000"])];
    // NO RULE: nothing is read, nothing sent — the home check reads a minted
    // rule or nothing (`Judge` carries the rule; a raster is no proof of a
    // motion). Wire v1.2: a task is named by its 32-byte citation.
    let task = [7u8; 32];
    // A TRAIN THE RULE DOES NOT SPEAK: the pairs left unchanged. The voice's
    // own replay refuses it, as the court's would; nothing is sent.
    let other = vec![
        (grid(&["1100", "0000"]), grid(&["1100", "0000"])),
        (grid(&["0000", "1100"]), grid(&["0000", "1100"])),
    ];
    let judge = Judge { scene: Scene { id: task.to_vec(), train: other, tests: Vec::new(), reading: Reading::Task }, outputs: outs, rule: spoken };
    assert!(matches!(judge_on(&entity.epigenome, &judge), Verdict::Kept(crate::client::sdk::entity::perceive::Stop::Uncrossed)), "refuted at home: it cannot be re-minted without its own pairs");
}

/// 78.158a U4 — THE SEAT'S SOLVE IS THE MIND'S OWN CALL. Told it called fn
/// 1 with the reference the relay answered, the mind reads the house's
/// outcome naming that reference as fn 1's — never `fn None` — and the
/// next percept reads it learned: (1, applied).
#[test]
fn outcome_reads_seat_solve_learned_against_fn_one() {
    use crate::anastome::dialogue::seat::{Arg, Outcome, Percept};
    use crate::client::sdk::seat_mind::task::FN_SOLVE;
    let (domain, mind, reference) = ([4u8; 32], [30u8; 32], [9u8; 32]);
    let mut m = crate::client::sdk::seat_mind::SeatMind::scoped(domain, None, 7);
    m.called(FN_SOLVE, vec![Arg::Count(1)], reference);
    assert_eq!(m.awaiting(), Some(FN_SOLVE), "the mind awaits its solve");
    let o = Outcome { domain, sub_origin: None, seat: 7, mind, call: reference, applied: true, reason: 0, deed: [0; 32] };
    assert_eq!(m.outcome(&o), Some(FN_SOLVE), "the outcome answers fn 1");
    let p = Percept { domain, sub_origin: None, seat: 7, mind, local_time: Arg::Count(1), seq: 1, records: vec![], point: None };
    assert_eq!(m.perceive(&p).learned, Some((FN_SOLVE, true)), "learned (1, applied)");
    // An outcome naming another reference answers nothing of this mind's.
    m.called(FN_SOLVE, vec![Arg::Count(2)], reference);
    let other = Outcome { call: [8; 32], ..o };
    assert_eq!(m.outcome(&other), None);
}

/// THE DECLINATION (ruled 2026-10-23): a seat whose reading
/// stops short of a solve calls fn 2 with the task's citation and its stop's
/// code, and the call travels whole through the codec the house reads — a
/// function apart from the solve's.
#[test]
fn decline_args_name_task_and_stop() {
    use crate::anastome::dialogue::seat::{put_args, take_args, Arg};
    use crate::client::sdk::seat_mind::task::{decline_args, FN_DECLINE, FN_SOLVE};
    assert_ne!(FN_DECLINE, FN_SOLVE, "a declination is no solve");
    let cite = [0x4b; 32];
    let args = decline_args(&cite, 1);
    assert_eq!(args, vec![Arg::Key(cite), Arg::Count(1)], "the citation, then the stop's code");
    let mut b = Vec::new();
    put_args(&mut b, &args).expect("the args write");
    let mut p = 0;
    assert_eq!(take_args(&b, &mut p), Some(args), "read back whole");
    assert_eq!(p, b.len());
}

/// 78.158a SIT-ALL — A LISTING OF N SEATS SITS N: `listed_seats` reads every
/// seat a SEATS listing offers with its epoch, and `reseat` reads a listing
/// against the seats sat — a first listing takes all N; a re-listing takes
/// the seats new to it and lets go the seats gone from it; an explicit list
/// the listing agrees with changes nothing.
#[test]
fn reseat_takes_new_seats_and_drops_gone() {
    use crate::anastome::dialogue::seat::{Arg, Post, Role, Seat, SeatFn, Seats};
    use crate::client::sdk::seat::{listed_seats, reseat};
    let house = ed25519_dalek::SigningKey::from_bytes(&[21; 32]);
    let seat = |id: u32| Seat { seat: id, role: Role::Steward, on: [6; 32], beat: Arg::Q { num: 1, den: 1 }, fns: vec![SeatFn { id: 1, tags: vec![] }] };
    let listing = |epoch: u64, ids: &[u32]| {
        Post::Seats(Seats { domain: [4; 32], sub_origin: None, epoch, seats: ids.iter().map(|&i| seat(i)).collect() }).sign(&house).expect("writes")
    };
    let (epoch, first) = listed_seats(&listing(1, &[3, 5, 8, 13])).expect("a SEATS listing");
    assert_eq!((epoch, first.clone()), (1, vec![3, 5, 8, 13]));
    let (take, let_go) = reseat(&[], &first);
    assert_eq!((take.len(), let_go), (4, vec![]), "a listing of 4 seats sits 4");
    let (epoch, second) = listed_seats(&listing(2, &[5, 13, 21])).expect("a SEATS listing");
    assert_eq!(epoch, 2);
    assert_eq!(reseat(&first, &second), (vec![21], vec![3, 8]), "a new world is taken, the gone ones let go");
    assert_eq!(reseat(&second, &second), (vec![], vec![]), "the same listing changes nothing");
    let not_seats = Post::Take(crate::anastome::dialogue::seat::Take { domain: [4; 32], sub_origin: None, seat: 1, mind: [30; 32], nonce: [1; 32] })
        .sign(&house)
        .expect("writes");
    assert_eq!(listed_seats(&not_seats), None, "a post that is no SEATS lists nothing");
}
