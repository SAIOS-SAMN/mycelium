// 78.146, WIRE v1.2 — A TASK IS NAMED BY ITS CITATION: 32 bytes, the citation
// of its registry content address, on ACT_TASK (by id and in the envelope)
// and ACT_SOLVE. A request by citation reads back as that citation and never
// as a plain request, and a plain request never as one by citation; the
// registry resolves the citation to the task and answers its envelope — train
// and test inputs, never the held answers — or nothing; a task whose content
// address passes the u16 wall is still named and answered.
// Conformance by decoded values, never bytes (Concordance 1.4).

use crate::anastome::dialogue::*;
use crate::arc::Task;
use crate::registry::{citation, task_address, TaskRegistry};

fn toy() -> Task {
    Task {
        train: vec![(vec![vec![1, 1], vec![0, 0]], vec![vec![2, 2], vec![0, 0]])],
        test: vec![(vec![vec![0, 0], vec![1, 1]], vec![vec![0, 0], vec![2, 2]])],
    }
}

/// A task whose content address passes 65 535 bytes: many test inputs, each
/// adding its own address.
fn large() -> Task {
    let test = (0..4000u32)
        .map(|i| {
            let row: Vec<u8> = (0..30u32).map(|c| u8::try_from((i * 7 + c) % 9 + 1).expect("a hue")).collect();
            (vec![row.clone(), row], Vec::new())
        })
        .collect();
    Task { train: toy().train, test }
}

/// A task is asked and answered by its citation, and the envelope carries
/// test inputs only, never the held answers.
#[test]
fn task_by_citation_round_trips_with_envelope() {
    let voice = [4u8; 32];
    let mut reg = TaskRegistry::new();
    let cite = reg.register(toy()).expect("a grid pair's chain is A5's");
    assert_eq!(cite, citation(&task_address(&toy()).expect("addressed")), "the book's id is the citation");
    assert!(reg.task(&cite).is_some_and(|t| t.train == toy().train && t.test == toy().test), "the citation resolves to the task");
    assert!(reg.task(&[9u8; 32]).is_none(), "an unknown citation names nothing");
    let ask = encode_task_by_citation(&voice, &cite);
    assert_eq!(decode_task_by_citation(&ask), Some((voice, cite)), "the citation reads back whole");
    assert_eq!(decode_task_request(&ask), None, "a request by citation is no plain request");
    let plain = encode_task_request(&voice);
    assert_eq!(decode_task_request(&plain), Some(voice));
    assert_eq!(decode_task_by_citation(&plain), None, "a plain request names no task");
    let mut longer = ask.clone();
    longer.push(0);
    assert_eq!(decode_task_by_citation(&longer), None, "a byte left over is no request");
    // The door's answer: the task the citation names, its test inputs only.
    let task = reg.task(&cite).expect("held");
    let tests: Vec<_> = task.test.iter().map(|(x, _)| x.clone()).collect();
    let env = decode_task_envelope(&encode_task_envelope(&cite, &task.train, &tests).expect("writes")).expect("reads");
    assert_eq!(env.id, cite, "the envelope names the task by its citation");
    assert_eq!(env.train, toy().train);
    assert_eq!(env.tests, vec![toy().test[0].0.clone()], "the held answers never ride");
}

/// A task whose content address passes the u16 wall is still named by its
/// 32-byte citation.
#[test]
fn citation_names_task_past_u16_wall() {
    let mut reg = TaskRegistry::new();
    let id = task_address(&large()).expect("addressed");
    assert!(id.as_bytes().len() > usize::from(u16::MAX), "the content address passes the wall: {}", id.as_bytes().len());
    let cite = reg.register(large()).expect("registers");
    assert_eq!(cite, citation(&id), "named by 32 bytes");
    assert!(reg.task(&cite).is_some(), "resolved court-side");
    let key = ed25519_dalek::SigningKey::from_bytes(&[5u8; 32]);
    let solve = decode_solve(&encode_solve(&key, &cite, &[vec![vec![1]]]).expect("writes")).expect("reads");
    assert_eq!((solve.task, solve.signed), (cite, true), "a solve names it by the citation, signed");
}
