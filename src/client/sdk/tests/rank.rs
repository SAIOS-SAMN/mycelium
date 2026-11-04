// MU-SENSING §0, §5; ROADMAP track 26 S4b — A TASK OF ANY RANK IS
// PERCEIVED, NOT ONLY CARRIED: the mind takes a task's samples at their own
// rank, holds the percept under the task's citation (its bodies, its ground,
// each pair's vortex of factorisations), and only then offers the samples to
// the alphabet's section, which spells one rank.

use crate::anastome::dialogue::TaskEnvelope;
use crate::client::sdk::mind::perceive::{SceneKind, Stop, STOPS};
use crate::client::sdk::*;
use crate::transducer::family::transduce::Factored;
use crate::transducer::sample::{Place, Sample};

fn sample(extents: &[usize], digits: &str) -> Sample {
    Sample::new(extents.to_vec(), 1, digits.bytes().map(|b| b - b'0').collect(), Place::new(vec![0])).expect("the fixture stands")
}

/// A mind that has perceived `env`, and the section it offered the word
/// path.
fn perceived(env: &TaskEnvelope) -> (Option<crate::anastome::dialogue::TaskRasters>, Mind) {
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("perceiving at rank n"));
    let section = mind.perceive_envelope(env, SceneKind::Task);
    (section, mind)
}

/// A RANK-3 TASK IS PERCEIVED: two pairs of 2×2×3 samples, a body painted
/// at the far corner of each, are held under the task's citation at rank
/// 3, every axis read as space, each pair factored with every factorisation
/// recomposing its output, and the ground deduced over the family. The
/// alphabet does not spell rank 3, so the task stops perceived and is not
/// spelled, by name; nothing is truncated to a plane.
#[test]
fn rank_three_task_is_perceived() {
    let id = [3u8; 32];
    let train = vec![
        (sample(&[2, 2, 3], "100000000000"), sample(&[2, 2, 3], "100000000002")),
        (sample(&[2, 2, 3], "010000000000"), sample(&[2, 2, 3], "010000000002")),
    ];
    let env = TaskEnvelope { id, train: train.clone(), tests: vec![sample(&[2, 2, 3], "001000000000")] };
    let (section, mind) = perceived(&env);
    assert!(section.is_none(), "rank 3 is off the alphabet's section");
    let percept = mind.board.percepts.get(&id).expect("the percept is held under the citation");
    assert_eq!(percept.rank(), 3, "read at its own rank");
    assert_eq!(percept.perspective.space_axes(), vec![0, 1, 2], "every axis is space; none privileged");
    assert_eq!(percept.train.len(), 2);
    assert_eq!(percept.tests.len(), 1);
    for (pair, (x, y)) in percept.train.iter().zip(&train) {
        assert!(!pair.factored.held().is_empty(), "each pair factors");
        for f in pair.factored.held() {
            assert_eq!(f.recomposed(x).as_ref(), Ok(y), "motion then residue is the output");
        }
    }
    assert!(percept.ground.holds(&vec![0]), "the ground deduced at rank 3: {:?}", percept.ground);
    let stop = STOPS.iter().position(|s| *s == Stop::BeyondAlphabet.name()).expect("a named stop");
    assert_eq!(mind.board.stopped[0][stop], 1, "the task stops perceived beyond the alphabet, once");
}

/// A RANK-1 TASK IS PERCEIVED by the same path: a signal shifted by its
/// reflection is held at rank 1, its pair a vortex of the two signed
/// permutations of one phase; it stops beyond the alphabet by name.
#[test]
fn rank_one_task_is_perceived() {
    let id = [1u8; 32];
    let train = vec![(sample(&[5], "11000"), sample(&[5], "00011"))];
    let env = TaskEnvelope { id, train: train.clone(), tests: vec![sample(&[5], "10000")] };
    let (section, mind) = perceived(&env);
    assert!(section.is_none(), "rank 1 is off the alphabet's section");
    let percept = mind.board.percepts.get(&id).expect("held");
    assert_eq!(percept.rank(), 1);
    let [pair] = percept.train.as_slice() else { panic!("one pair") };
    let Factored::Vortex(held) = &pair.factored else { panic!("both motions of one phase land: {:?}", pair.factored) };
    assert_eq!(held.len(), 2);
    let reversal = held.iter().find(|f| f.residue.is_empty()).expect("the reversal carries the signal whole");
    assert_eq!(reversal.recomposed(&train[0].0).as_ref(), Ok(&train[0].1));
}

/// A RANK-2 TASK AT ONE BYTE A SYMBOL IS PERCEIVED AND SPELLED: the percept
/// is held, and the alphabet's section is handed to the word path as before.
#[test]
fn rank_two_task_is_perceived_and_offered_to_section() {
    let id = [2u8; 32];
    let env = TaskEnvelope { id, train: vec![(sample(&[2, 3], "100000"), sample(&[2, 3], "000001"))], tests: vec![sample(&[2, 3], "010000")] };
    let (section, mind) = perceived(&env);
    let section = section.expect("rank 2 at width 1 is on the section");
    assert_eq!(section.train, vec![(vec![vec![1, 0, 0], vec![0, 0, 0]], vec![vec![0, 0, 0], vec![0, 0, 1]])]);
    assert_eq!(mind.board.percepts.get(&id).map(|p| p.rank()), Some(2));
}
