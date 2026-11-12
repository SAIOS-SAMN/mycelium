// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-SENSING §4, §6.2 (track 26 S6) — A SOLVE OF OUTPUTS AND A SPOKEN
// SENTENCE STATE THE PERSPECTIVE THEY WERE PERCEIVED OR SPOKEN UNDER, as a
// claim of factorisation has since S6 stage 2, and the court reconstructs
// under it: it reads its own copy of the task's pairs through the stated
// perspective and never chooses one.

use super::fixtures::*;
use crate::cognition::*;
use crate::transducer::family::transduce::{Ground, Sense, SignedPermutation};
use crate::transducer::family::{NoCrystal, TrainPair};
use crate::transducer::perspective::{Perspective, Role, SensorMotion};

/// A canvas of two halves, 0 and 5, along the first axis, so the ground is
/// a vortex of 0 and 5; a body of 1 standing on the 0 half moves one site
/// along the first axis, in two train pairs; the withheld pair is a third
/// such move. The solve's `halves` family with its axes exchanged.
fn moves() -> (Vec<TrainPair>, Vec<TrainPair>) {
    let train = vec![
        (
            digits(&["000", "010", "000", "000", "000", "555", "555", "555"]),
            digits(&["000", "000", "010", "000", "000", "555", "555", "555"]),
        ),
        (
            digits(&["000", "000", "001", "000", "000", "555", "555", "555"]),
            digits(&["000", "000", "000", "001", "000", "555", "555", "555"]),
        ),
    ];
    let withheld = vec![(
        digits(&["000", "000", "000", "010", "000", "555", "555", "555"]),
        digits(&["000", "000", "000", "000", "010", "555", "555", "555"]),
    )];
    (train, withheld)
}

/// The sensor turned: read axis 0 is the sample's axis 1 and read axis 1
/// its axis 0, so the move reads along the second axis, as the solve's
/// `halves` reads it.
fn turned() -> Perspective {
    let swap = SignedPermutation::new(vec![(1, Sense::Kept), (0, Sense::Kept)]).expect("a signed permutation");
    Perspective::of_sample(2).shifted(&SensorMotion::Pose(swap)).expect("the pose turns")
}

/// The rule perceived under `perspective`: extracted and minted on the
/// family as it reads it, under the ground 0.
fn rule_under(train: &[TrainPair], perspective: &Perspective) -> crate::transducer::element::Element {
    let read = stated_perspective(train, perspective).expect("the section is read under the perspective");
    crate::transducer::raster::on_ground(Ground::One(0), || {
        let rule = crate::transducer::family::extract_rule(&read, &mut budget()).expect("the move extracts");
        crate::transducer::family::mint_sentence(&rule, &read, &NoCrystal, 0).expect("and mints")
    })
}

/// THE SOLVE OF OUTPUTS AND THE SPEECH STATE THEIR PERSPECTIVE; THE COURT
/// RECONSTRUCTS UNDER IT. A rule read under the turned perspective is
/// solved where the court reads its withheld pair under that perspective,
/// and not on the pair as posted. The solve act carries the perspective
/// and reads it back. A sentence spoken under the turned perspective stands
/// where it states it, and is refused where it states none (the family
/// read as posted). A perspective of another rank, or one reading an axis
/// as other than space, is refused by name.
#[test]
fn solve_and_speech_state_their_perspective_and_court_reconstructs_under_it() {
    use crate::anastome::dialogue::{decode_solve, decode_speech, encode_solve_spoken, encode_speech_spoken, Adjudication, Spoken};
    let (train, withheld) = moves();
    let turned = turned();
    let rule = rule_under(&train, &turned);

    // The solve: the court reads the withheld pair through the stated
    // perspective and reconstructs there.
    let read = stated_perspective(&withheld, &turned).expect("the withheld pair is read under it");
    let under = crate::transducer::raster::on_ground(Ground::One(0), || Court::new().judge_solve(&rule, &read));
    assert!(matches!(under, SolveVerdict::Solved { .. }), "reconstructed under the stated perspective, it closes: {under:?}");
    let posted = crate::transducer::raster::on_ground(Ground::One(0), || Court::new().judge_solve(&rule, &withheld));
    assert!(!matches!(posted, SolveVerdict::Solved { .. }), "read as posted, the turned rule does not close: {posted:?}");

    // The solve act states the perspective, signed with it.
    let key = ed25519_dalek::SigningKey::from_bytes(&[3u8; 32]);
    let outputs = crate::anastome::dialogue::spoken_outputs(&[withheld[0].1.clone()]).expect("a sample");
    let act = decode_solve(&encode_solve_spoken(&key, &[4; 32], &outputs, Some(b"tape"), Some(&[0]), &turned).expect("fits")).expect("reads");
    assert_eq!((act.perspective, act.signed), (turned.clone(), true), "the stated perspective rides the solve");

    // The speech: each sentence states its perspective; the court hears it
    // on its copy of the family read through it.
    let task = crate::registry::citation(
        &crate::registry::task_address(&crate::transducer::family::Task { train: train.clone(), test: Vec::new() }).expect("a task identity"),
    );
    let hear = |perspective: Option<Perspective>| {
        let tape = rule.to_bytes().expect("the record writes");
        let said = Spoken { tape, standing: 0, contraction: None, agreement: None, ground: Some(vec![0]), perspective };
        let act = decode_speech(&encode_speech_spoken(&[1; 32], task, 0, &[said]).expect("the speech is delimited")).expect("the speech reads");
        let mut k = Court::new();
        k.bind_task_family(task, train.clone());
        // 78.153: a fixture asking what one act hears hands it a stroke
        // that holds it.
        (act.perspectives.clone(), k.hear_act_on(&act, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("heard"))
    };
    let (carried, heard) = hear(Some(turned.clone()));
    assert_eq!(carried, vec![Some(turned.clone())], "the stated perspective rides the speech");
    assert!(matches!(heard.as_slice(), [Adjudication::Stood { .. }]), "heard under the stated perspective, it stands: {heard:?}");
    let (carried, heard) = hear(None);
    assert_eq!(carried, vec![None], "none stated, none carried");
    assert!(matches!(heard.as_slice(), [Adjudication::Refused { .. }]), "heard on the family as posted, it does not close: {heard:?}");

    // A perspective the court does not read the section under.
    assert_eq!(stated_perspective(&train, &Perspective::of_sample(3)).err(), Some(StatedPerspectiveRefusal::Rank), "another rank");
    let sequenced = turned.shifted(&SensorMotion::Recast { axis: 0, role: Role::Sequence }).expect("a recast");
    assert_eq!(stated_perspective(&train, &sequenced).err(), Some(StatedPerspectiveRefusal::Role), "an axis read as a sequence");
    assert_eq!(StatedPerspectiveRefusal::Rank.reason(), crate::court::verdict::REFUSE_PERSPECTIVE_RANK);
    assert_eq!(StatedPerspectiveRefusal::Role.reason(), crate::court::verdict::REFUSE_PERSPECTIVE_ROLE);
    let (_, heard) = hear(Some(sequenced));
    assert!(matches!(heard.as_slice(), [Adjudication::Refused { .. }]), "a perspective the section is not read under is refused: {heard:?}");
}
