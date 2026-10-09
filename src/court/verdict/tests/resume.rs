// 78.153 — REPLAYS METERED AND RESUMABLE: a stroke that runs out is
// unread, never a refusal; a replay resumed across strokes rules exactly as
// one that never ran out.

use crate::transducer::element::Element;
use crate::transducer::family::{extract_rule, extract_rule_with, mint_sentence, Grid, NoCrystal};
use crate::transducer::sentence::{as_element_form, Vertex};
use crate::verdict::*;

fn digits(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

fn stroke(n: usize) -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(n)
}

/// A bar recoloured and a compartment created beside it (the receiver's
/// fixture): one word vertex, one `Creation` demand — a GOOD word.
fn good() -> (Vec<(Grid, Grid)>, Element) {
    let family = vec![(digits(&["110", "000"]), digits(&["220", "003"]))];
    let rule = extract_rule(&family, &mut stroke(1 << 20)).expect("the bar's recolour extracts");
    let e = mint_sentence(&rule, &family, &NoCrystal, 0).expect("a sentence with a demand is spoken whole");
    (family, e)
}

/// The same sentence with its demand struck out: it leaves a compartment it
/// does not declare, `REFUSE_NOT_CLOSED`.
fn undeclared() -> (Vec<(Grid, Grid)>, Element) {
    let (family, e) = good();
    let mut f = crate::transducer::sentence::from_element_form(&e).expect("the tape reads back");
    let demands: Vec<_> = f.demand_vertices().map(|(at, ..)| at).collect();
    for at in &demands {
        f.vertices.drop_at(at);
    }
    f.reindex();
    (family, as_element_form(&f, e.magnitude()).expect("a tape"))
}

/// One attempt on a stroke of `n` reads from `cursor`.
fn attempt(e: &Element, family: &[(Grid, Grid)], n: usize, cursor: &mut ReplayCursor) -> Replayed {
    replay_resume(e, &|_| None, Some(family), &NoCrystal, &mut stroke(n), cursor, true)
}

/// The verdict on a stroke that never runs out, and the reads it took.
fn whole(e: &Element, family: &[(Grid, Grid)]) -> (Replayed, usize) {
    let mut b = stroke(1 << 20);
    let out = replay_resume(e, &|_| None, Some(family), &NoCrystal, &mut b, &mut ReplayCursor::default(), true);
    (out, (1 << 20) - b.remaining())
}

/// Strokes of `n` reads, each resuming the last, until the replay rules:
/// the verdict and the strokes it took.
fn resumed(e: &Element, family: &[(Grid, Grid)], n: usize) -> (Replayed, usize) {
    let mut cursor = ReplayCursor::default();
    let mut strokes = 0usize;
    loop {
        strokes += 1;
        assert!(strokes <= 4096, "a stroke of {n} reads moves the replay on");
        match attempt(e, family, n, &mut cursor) {
            Replayed::Unread => continue,
            out => return (out, strokes),
        }
    }
}

/// THE INVARIANT (78.153): a replay that needs more reads than one stroke
/// is `Unread` — not ruled, not refused — and resumed on the next strokes
/// it rules EXACTLY as a full-budget replay does: the same verdict, the
/// same refusal at the same coordinate, on every stroke size that moves it.
#[test]
fn a_replay_longer_than_a_stroke_is_unread_then_resumed_to_the_same_ruling() {
    for (family, e) in [good(), undeclared()] {
        let (full, need) = whole(&e, &family);
        assert!(need > 2, "the fixture needs more reads than a stroke of two: {need}");
        // One stroke too short: unread, and the cursor has moved.
        let mut cursor = ReplayCursor::default();
        assert_eq!(attempt(&e, &family, 2, &mut cursor), Replayed::Unread);
        assert!(cursor.progress() != ReplayCursor::default().progress(), "the stroke moved the cursor");
        // Resumed, on every stroke size from two reads to the whole need.
        for n in 2..=need {
            let (out, strokes) = resumed(&e, &family, n);
            assert_eq!(out, full, "a replay resumed on strokes of {n} rules as the whole one");
            assert!(n == need || strokes > 1, "a stroke of {n} < {need} reads is not the whole replay");
        }
    }
    assert_eq!(whole(&good().1, &good().0).0, Replayed::Clean, "the good word replays clean");
    assert!(
        matches!(whole(&undeclared().1, &undeclared().0).0, Replayed::Refused(ref r) if r.reason == REFUSE_NOT_CLOSED),
        "the undeclared word is refused, not closed"
    );
}

/// DEFECT 1 (lock audit): A SPENT STROKE NEVER REFUSES A GOOD WORD. On every
/// stroke short of the whole need the good word's replay is `Unread` — never
/// `Refused` — and on the whole need it is `Clean`.
#[test]
fn a_spent_stroke_never_refuses_a_good_word() {
    let (family, e) = good();
    let (full, need) = whole(&e, &family);
    assert_eq!(full, Replayed::Clean);
    for n in 0..need {
        assert_eq!(
            attempt(&e, &family, n, &mut ReplayCursor::default()),
            Replayed::Unread,
            "a stroke of {n} of {need} reads rules nothing"
        );
    }
    assert_eq!(attempt(&e, &family, need, &mut ReplayCursor::default()), Replayed::Clean);
}

/// DEFECT 1 AT ITS SITE: the edge re-measure's `versor_between` draws from
/// the stroke, and where it runs out (`NoPose::Unbounded`) the face is
/// SPENT — the next stroke resumes at it — never `REFUSE_EDGE`. A forged
/// frame-axis motor reaches the coset read (`motor_carries`); on every
/// stroke short of what the edges need it is spent, and on the whole need
/// it is refused, by application.
#[test]
fn an_edge_on_a_spent_stroke_is_spent_never_refused() {
    use crate::verdict::body::{edges_rederive_frames_from, perceive_family};
    let a_in = ["110000", "000000", "110000", "000000"];
    let a_out = ["220000", "000000", "220000", "000000"];
    let b_in = ["011000", "000000", "011000", "000000"];
    let b_out = ["022000", "000000", "022000", "000000"];
    let family = vec![(digits(&a_in), digits(&a_out)), (digits(&b_in), digits(&b_out))];
    let rule = extract_rule_with(&family, &NoCrystal, &mut stroke(1 << 20)).expect("extracts");
    let motor = rule
        .form
        .faces
        .entries()
        .find(|(_, f)| f.producer == crate::transducer::sentence::Producer::Motor)
        .map(|(o, _)| o)
        .expect("the two dominoes' chart steps between its frames: a motor overlap");
    let mut forged = rule.form.clone();
    if let Some(f) = forged.faces.at_mut(&motor) {
        let q = |n: i64| num_rational::BigRational::from_integer(n.into());
        f.place.carriage = crate::transducer::cga::at_three::translator(&q(1), &q(0), &q(0));
    }
    let perceived = perceive_family(&family);
    for (is_forged, form) in [(false, &rule.form), (true, &forged)] {
        let mut b = stroke(1 << 20);
        let full = edges_rederive_frames_from(form, &family, &perceived, &mut b, 0);
        let need = (1 << 20) - b.remaining();
        for n in 0..need {
            assert!(
                matches!(edges_rederive_frames_from(form, &family, &perceived, &mut stroke(n), 0), Err(Halt::Spent(_))),
                "a stroke of {n} of {need} reads is spent on the edges, never refused"
            );
        }
        match (is_forged, full) {
            (false, Ok(())) => {}
            (true, Err(Halt::Refused(r))) => assert_eq!(r.reason, REFUSE_EDGE, "the forged motor is refused"),
            (forged, other) => panic!("forged {forged}: {other:?}"),
        }
    }
    // The good rule's form carries the word vertex the replay needs.
    assert!(rule.form.vertices.entries().any(|(_, v)| matches!(v, Vertex::Word { .. })));
}

/// 78.158a U2 — THE CALLER'S STROKE IS HONOURED, NEVER DRAINED. A replay
/// whose one unit costs one read more than a default stroke grants is
/// UNREAD on a caller's stroke no wider than a default one — refused by
/// name, kind N, no reading of the word — and read whole on a caller's
/// stroke as wide as the unit, which is left as it stood.
#[test]
fn an_off_lock_reader_reads_each_round_on_the_callers_wider_stroke() {
    use crate::transducer::sentence::ObstructionKind;
    let (_, e) = good();
    let unit = usize::try_from(crate::physics::spend_unit()).expect("a stroke's reads fit a usize") + 1;
    let mut run = |b: &mut crate::physics::Budget, _: &mut ReplayCursor| {
        for _ in 0..unit {
            if !b.spend() {
                return Replayed::Unread;
            }
        }
        Replayed::Clean
    };
    let unread = crate::verdict::blind::to_completion(&e, &mut stroke(1), &mut run);
    assert_eq!(unread.as_ref().map(|r| r.reason), Some(REFUSE_UNREAD), "no default stroke reads the unit");
    assert_eq!(unread.map(|r| r.obstruction.kind()), Some(ObstructionKind::N), "and that is no reading of the word");
    let mut wide = stroke(unit);
    assert_eq!(crate::verdict::blind::to_completion(&e, &mut wide, &mut run), None, "the caller's stroke reads it");
    assert_eq!(wide.remaining(), unit, "and is left as it stood");
}

/// THE OFF-LOCK READERS keep their answer: run to the end over fresh
/// strokes, `blind_replay_at` on the caller's stroke of one read is the
/// whole replay's verdict.
#[test]
fn an_off_lock_reader_runs_the_replay_to_its_end() {
    for (family, e) in [good(), undeclared()] {
        let (full, _) = whole(&e, &family);
        let off = blind_replay_at(&e, &|_| None, Some(&family), &NoCrystal, &mut stroke(1));
        assert_eq!(off.as_ref(), full.refusal(), "the off-lock reader answers the whole verdict");
    }
}
