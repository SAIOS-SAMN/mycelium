// MU-SENSING §0, §5; ROADMAP track 26 S4b — A TASK OF ANY RANK IS
// PERCEIVED, NOT ONLY CARRIED: the mind takes a task's samples at the
// samples' own rank, holds the percept under the task's citation (its bodies, its ground,
// each pair's vortex of factorisations), and only then offers the samples to
// the alphabet's section, which spells one rank.

use crate::anastome::dialogue::TaskFamily;
use crate::client::sdk::mind::perceive::{SceneKind, Stop, STOPS};
use crate::client::sdk::*;
use crate::transducer::family::transduce::Factored;
use crate::transducer::sample::{Place, Sample};

fn sample(extents: &[usize], digits: &str) -> Sample {
    Sample::new(extents.to_vec(), 1, digits.bytes().map(|b| b - b'0').collect(), Place::new(vec![0])).expect("the fixture stands")
}

/// A mind that has perceived `env`, and the section it offered the word
/// path.
fn perceived(env: &TaskFamily) -> (Option<crate::anastome::dialogue::TaskSection>, Mind) {
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("perceiving at rank n"));
    let section = mind.perceive_task(env, SceneKind::Task);
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
    let env = TaskFamily { id, train: train.clone(), tests: vec![sample(&[2, 2, 3], "001000000000")] };
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
    let env = TaskFamily { id, train: train.clone(), tests: vec![sample(&[5], "10000")] };
    let (section, mind) = perceived(&env);
    assert!(section.is_none(), "rank 1 is off the alphabet's section");
    let percept = mind.board.percepts.get(&id).expect("held");
    assert_eq!(percept.rank(), 1);
    let [pair] = percept.train.as_slice() else { panic!("one pair") };
    let Factored::Vortex(held) = &pair.factored else { panic!("both motions of one phase land: {:?}", pair.factored) };
    assert_eq!(held.len(), 2);
    let reversal = held.iter().find(|f| f.residue.is_empty()).expect("the reversal carries the signal whole");
    assert_eq!(reversal.recomposed(&train[0].0).as_ref(), Ok(&train[0].1));
    let transitions = pair.carrier_transitions(&train[0].0, &train[0].1, reversal).expect("transitions lift onto carrier");
    assert!(!transitions.is_empty(), "carrier transitions between charts are produced");
}

/// A claim the in-process court heard, and the verdict it answered.
type Heard = (crate::anastome::dialogue::FactorisationAct, crate::anastome::dialogue::FactorVerdict);

/// A HYPHA IN PROCESS (track 26 S4c): a book holding the posted rank-3
/// task, heard on a socket; each claim of factorisation that arrives is
/// judged as the hypha's control surface judges it
/// (`server::factorisation::judge_factorisation`), recorded where
/// admitted, answered with its verdict, and handed to the test with it.
fn factoring_court(book: std::sync::Arc<std::sync::Mutex<crate::server::registry::TaskRegistry>>) -> (String, std::sync::mpsc::Receiver<Heard>) {
    use crate::anastome::dialogue;
    use std::io::Write;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("the court listens");
    let addr = listener.local_addr().expect("an address").to_string();
    let (heard, claims) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for conn in listener.incoming() {
            let Ok(mut s) = conn else { continue };
            while let Some(body) = super::fixtures::read_act(&mut s) {
                let reply = match dialogue::decode_factorisation(&body) {
                    Some(act) => {
                        let task = book.lock().expect("the book").task(&act.task).cloned();
                        let verdict = crate::server::factorisation::judge_factorisation(task.as_ref(), &act);
                        if verdict == dialogue::FactorVerdict::Admitted {
                            book.lock().expect("the book").record_factorisation(&act);
                        }
                        let reply = dialogue::encode_factor_verdict(&verdict);
                        let _ = heard.send((act, verdict));
                        reply
                    }
                    None => Vec::new(),
                };
                let _ = s.write_all(&super::fixtures::reply_bytes(&reply));
            }
        }
    });
    (addr, claims)
}

/// THE MIND SPEAKS ITS FACTORISATIONS BEYOND THE ALPHABET, end to end in
/// process: a rank-3 task posted on a book, served as its envelope,
/// perceived by a mind at its own rank (the alphabet does not spell it, so
/// it stops `BeyondAlphabet`), and every factorisation the mind holds of
/// each train pair spoken as one signed claim, which the court reconstructs
/// and admits, and records once. A vortex speaks per factorisation; the
/// task perceived again speaks nothing new.
#[test]
fn mind_speaks_factorisations_beyond_the_alphabet() {
    use crate::anastome::dialogue::{decode_task, encode_task, FactorVerdict};
    let book = std::sync::Arc::new(std::sync::Mutex::new(crate::server::registry::TaskRegistry::new()));
    let posted = crate::server::tests::fixtures::rank_three_task();
    let cite = book.lock().expect("the book").post(posted);
    let served = {
        let mut b = book.lock().expect("the book");
        let (id, task) = b.dispatch().expect("the rank-3 task is dispatched");
        let e = task.family(id);
        encode_task(&id, &e.train, &e.tests).expect("served at rank 3")
    };
    let env = decode_task(&served).expect("the envelope reads");
    assert_eq!(env.id, cite);
    let (court, claims) = factoring_court(book.clone());
    let name = "factoring beyond the alphabet";
    let mut mind = Mind::open(&court, Voice::named(name));
    assert!(mind.perceive_task(&env, SceneKind::Task).is_none(), "rank 3 is off the alphabet's section");
    let percept = mind.board.percepts.get(&cite).cloned().expect("perceived at its own rank");
    let ground = match &percept.ground {
        crate::transducer::family::transduce::Ground::One(g) => Some(g.clone()),
        _ => None,
    };
    let held: usize = percept.train.iter().map(|p| p.factored.held().len()).sum();
    assert!(held > percept.train.len(), "a pair's vortex holds several factorisations: {held}");
    mind.settle();
    let heard: Vec<_> = claims.try_iter().collect();
    assert_eq!(heard.len(), held, "one claim per held factorisation");
    for (pair, p) in percept.train.iter().enumerate() {
        let spoken: Vec<_> = heard.iter().filter(|(act, _)| act.pair == pair).collect();
        assert_eq!(spoken.len(), p.factored.held().len(), "pair {pair}: the vortex speaks per factorisation");
        for (f, (act, verdict)) in p.factored.held().iter().zip(spoken) {
            assert_eq!((act.voice, act.task, act.signed), (Voice::named(name).id(), cite, true), "signed by the mind, naming the task");
            assert_eq!((&act.motion, &act.residue), (&f.motion, &f.residue), "the motion and residue the mind read");
            assert_eq!(act.ground, ground, "stating the ground the mind read where it read one symbol");
            assert_eq!(*verdict, FactorVerdict::Admitted, "the court reconstructs it and admits");
        }
    }
    assert_eq!(book.lock().expect("the book").factorisations_of(&cite), held, "each admitted claim recorded once");
    assert!(mind.perceive_task(&env, SceneKind::Task).is_none());
    assert_eq!(mind.hypha.in_flight(), 0, "the task perceived again speaks nothing new");
}

/// TRACK 26 S4d — THE WORD MACHINERY READS A SAMPLE OF ANY RANK, END TO
/// END: its carrier is the posted sample itself, admitted onto the
/// alphabet's section (`spelled::SectionSample`), never a copy written out
/// as rows. At rank 2 the word path holds the very samples posted, each
/// with its place. At rank 3 the same carrier refuses by the alphabet's
/// rank, read off the sample; the family's symbol set is read over the
/// rank-3 samples posted beside it; and the mind holds the perception under
/// the task's citation and states it, one signed claim per factorisation it
/// holds, each the motion and residue it read.
#[test]
fn word_machinery_reads_a_rank_three_sample_end_to_end() {
    use crate::anastome::dialogue::decode_factorisation;
    use crate::transducer::alphabet::AlphabetRank;
    use crate::transducer::sentence::SectionRefusal;
    use crate::transducer::spelled::SectionSample;
    use crate::transducer::symbols::{on_test_inputs, SymbolSet};
    // RANK 2: the word path holds the samples posted, as posted.
    let flat = TaskFamily { id: [2u8; 32], train: vec![(sample(&[2, 3], "100000"), sample(&[2, 3], "000001"))], tests: vec![sample(&[2, 3], "010000")] };
    let (section, _) = perceived(&flat);
    let section = section.expect("rank 2 at one byte is on the section");
    assert_eq!(section.train[0].0.sample(), &flat.train[0].0, "the input sample itself, its place included");
    assert_eq!(section.tests[0].sample(), &flat.tests[0], "and the test input");
    // RANK 3: the carrier refuses by the rank it reads, never truncated.
    let id = [33u8; 32];
    let train = vec![
        (sample(&[2, 2, 3], "100000000000"), sample(&[2, 2, 3], "100000000002")),
        (sample(&[2, 2, 3], "010000000000"), sample(&[2, 2, 3], "010000000002")),
    ];
    let deep = TaskFamily { id, train: train.clone(), tests: vec![sample(&[2, 2, 3], "001000000003")] };
    let off = SectionRefusal::Rank(AlphabetRank { rank: 3 });
    assert_eq!(SectionSample::of(train[0].0.clone()), Err(off), "the word machinery's carrier reads the rank off the sample");
    assert!(matches!(deep.section(), Err(r) if r == off), "the envelope is refused by the alphabet's name");
    // The symbol set the word path reads is read over the rank-3 samples.
    let sigma = on_test_inputs(&deep.tests, || SymbolSet::of_family(&[])).expect("one width");
    assert_eq!((sigma.len(), sigma.index(&[3])), (3, Some(2)), "0, 1 and 3, read off the rank-3 test input");
    // THE MIND HOLDS THE PERCEPTION AND STATES IT.
    let (section, mind) = perceived(&deep);
    assert!(section.is_none(), "rank 3 stops beyond the alphabet");
    let percept = mind.board.percepts.get(&id).expect("held under the citation");
    assert_eq!(percept.rank(), 3, "an object of rank 3, every axis read");
    let held: Vec<_> = percept.train.iter().enumerate().flat_map(|(pair, p)| p.factored.held().iter().map(move |f| (pair, f))).collect();
    let claims = mind.factorisation_claims(&id);
    assert_eq!(claims.len(), held.len(), "one stated claim per held factorisation");
    for (bytes, (pair, f)) in claims.iter().zip(&held) {
        let act = decode_factorisation(bytes).expect("the claim reads");
        assert!(act.signed, "signed by the mind");
        assert_eq!((act.task, act.pair), (id, *pair), "naming the task and the pair");
        assert_eq!((&act.motion, &act.residue), (&f.motion, &f.residue), "the motion and the residue the mind read");
    }
}

/// A RANK-2 TASK AT ONE BYTE A SYMBOL IS PERCEIVED AND SPELLED: the percept
/// is held, and the alphabet's section is handed to the word path as before.
#[test]
fn rank_two_task_is_perceived_and_offered_to_section() {
    let id = [2u8; 32];
    let env = TaskFamily { id, train: vec![(sample(&[2, 3], "100000"), sample(&[2, 3], "000001"))], tests: vec![sample(&[2, 3], "010000")] };
    let (section, mind) = perceived(&env);
    let section = section.expect("rank 2 at width 1 is on the section");
    assert_eq!(section.train, crate::transducer::spelled::pairs_of(vec![(vec![vec![1, 0, 0], vec![0, 0, 0]], vec![vec![0, 0, 0], vec![0, 0, 1]])]));
    assert_eq!(mind.board.percepts.get(&id).map(|p| p.rank()), Some(2));
}
