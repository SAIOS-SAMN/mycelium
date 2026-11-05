// MU-SENSING §3.9 (2), §6.2; ROADMAP track 26 S4c — A CLAIM OF FACTORISATION
// ADJUDICATED ON THE HYPHA: the act heard off the wire, the posted rank-3
// pair it names found on the book, the stated motion reconstructed and the
// residue derived by the court, admitted on a match and recorded; a residue
// that differs refused with the residue of the residues; the stated ground
// read against the task, never deduced.

use crate::anastome::dialogue::{decode_factorisation, encode_factorisation, FactorVerdict, FactorisationAct, PostedTask};
use crate::court::verdict::{REFUSE_FACTOR_MOTION, REFUSE_FACTOR_NO_PAIR, REFUSE_FACTOR_UNSIGNED, REFUSE_GROUND_NOT_ONE_SYMBOL, REFUSE_GROUND_NO_CANDIDATE};
use crate::server::factorisation::judge_factorisation;
use crate::server::registry::TaskRegistry;
use crate::server::tests::fixtures::rank_three_task;
use crate::transducer::base::Site;
use crate::transducer::family::transduce::{factorisations, factorisations_need, Enclosure, Factorisation, Motion};
use crate::transducer::sentence::Link;

fn key() -> ed25519_dalek::SigningKey {
    ed25519_dalek::SigningKey::from_bytes(&[7u8; 32])
}

/// Every factorisation the mind's own reading holds for train pair `pair`.
fn held(task: &PostedTask, pair: usize) -> Vec<Factorisation> {
    let (x, y) = &task.train[pair];
    let mut budget = crate::physics::Physics::default().stroke_n(factorisations_need(x.rank(), 1).expect("a need"));
    factorisations(x, y, &mut budget).expect("the pair factors").held().to_vec()
}

/// A claim as it arrives off the wire: written, signed and read back.
fn heard(cite: &[u8; 32], pair: usize, motion: &Motion, residue: &[Link], ground: Option<&[u8]>) -> FactorisationAct {
    decode_factorisation(&encode_factorisation(&key(), cite, pair, motion, residue, ground).expect("the claim writes")).expect("the claim reads")
}

fn refused(reason: &str) -> FactorVerdict {
    FactorVerdict::Refused { reason: reason.as_bytes().to_vec() }
}

/// THE COURT ADMITS A MATCHING CLAIM: on the posted rank-3 task, every
/// factorisation the mind's reading holds for each train pair, stated as a
/// signed claim, is admitted by the court's own reconstruction and recorded
/// once under the task's citation; the same claim heard again records
/// nothing new. The court searched for nothing: each verdict reads the
/// stated motion alone.
#[test]
fn court_admits_a_matching_factorisation_claim() {
    let task = rank_three_task();
    let mut book = TaskRegistry::new();
    let cite = book.post(task.clone());
    let mut claims = 0usize;
    for pair in 0..task.train.len() {
        let held = held(&task, pair);
        assert!(!held.is_empty(), "pair {pair} factors");
        for f in held {
            let act = heard(&cite, pair, &f.motion, &f.residue, None);
            assert_eq!(judge_factorisation(book.task(&cite), &act), FactorVerdict::Admitted, "pair {pair}: {:?}", f.motion);
            book.record_factorisation(&act);
            claims += 1;
        }
    }
    assert_eq!(book.factorisations_of(&cite), claims, "each admitted claim recorded once");
    let f = held(&task, 0).remove(0);
    book.record_factorisation(&heard(&cite, 0, &f.motion, &f.residue, None));
    assert_eq!(book.factorisations_of(&cite), claims, "a claim admitted again records nothing new");
}

/// THE COURT REFUSES A CLAIM WHOSE RESIDUE DIFFERS, with the residue of the
/// residues: the link the claim left out is `unclaimed`, the link it
/// invented `uncarried`, exactly, never a flag. A claim that is unsigned,
/// names no train pair (the withheld pairs are the court's) or states a
/// motion onto another enclosure is refused by name before any residue is
/// derived.
#[test]
fn court_refuses_a_factorisation_whose_residue_differs() {
    let task = rank_three_task();
    let mut book = TaskRegistry::new();
    let cite = book.post(task.clone());
    let f = held(&task, 0).into_iter().find(|f| !f.residue.is_empty()).expect("a factorisation that leaves a residue");
    let left_out = f.residue[0].clone();
    let invented = Link::new(Site::of([0, 0, 0]), vec![1], vec![5]);
    let mut stated: Vec<Link> = f.residue[1..].to_vec();
    stated.push(invented.clone());
    let verdict = judge_factorisation(book.task(&cite), &heard(&cite, 0, &f.motion, &stated, None));
    assert_eq!(verdict, FactorVerdict::Residue { unclaimed: vec![left_out], uncarried: vec![invented] });
    assert_eq!(book.factorisations_of(&cite), 0, "nothing refused is recorded");

    let mut unsigned = heard(&cite, 0, &f.motion, &f.residue, None);
    unsigned.signed = false;
    assert_eq!(judge_factorisation(book.task(&cite), &unsigned), refused(REFUSE_FACTOR_UNSIGNED));
    assert_eq!(judge_factorisation(book.task(&cite), &heard(&cite, 2, &f.motion, &f.residue, None)), refused(REFUSE_FACTOR_NO_PAIR));
    assert_eq!(judge_factorisation(None, &heard(&[9; 32], 0, &f.motion, &f.residue, None)), refused("no such task at this court"));
    let elsewhere = Motion { permutation: f.motion.permutation.clone(), onto: Enclosure::new(vec![2, 2, 4]).expect("an enclosure") };
    let FactorVerdict::Refused { reason } = judge_factorisation(book.task(&cite), &heard(&cite, 0, &elsewhere, &f.residue, None)) else {
        panic!("a motion onto another enclosure is no motion of the pair")
    };
    assert!(reason.starts_with(REFUSE_FACTOR_MOTION.as_bytes()), "{}", String::from_utf8_lossy(&reason));
}

/// A CLAIM OF FACTORISATION STATES ITS GROUND (MU-SENSING §6.2): the stated
/// ground rides the claim and reads back; the court reads it against the
/// task's train pairs as a candidate at the samples' width and never
/// deduces one. A candidate stated is admitted; a symbol that is no
/// candidate, or a stating that is not one symbol at the width, is refused
/// by name; none stated is reconstructed under none.
#[test]
fn factorisation_claim_states_its_ground() {
    let task = rank_three_task();
    let mut book = TaskRegistry::new();
    let cite = book.post(task.clone());
    let f = held(&task, 1).remove(0);
    let on = |ground: Option<&[u8]>| heard(&cite, 1, &f.motion, &f.residue, ground);
    assert_eq!(on(Some(&[0])).ground, Some(vec![0]), "the stated ground rides the claim");
    assert_eq!(judge_factorisation(book.task(&cite), &on(Some(&[0]))), FactorVerdict::Admitted, "a candidate of the task's ground");
    assert_eq!(judge_factorisation(book.task(&cite), &on(None)), FactorVerdict::Admitted, "none stated, none read");
    assert_eq!(judge_factorisation(book.task(&cite), &on(Some(&[2]))), refused(REFUSE_GROUND_NO_CANDIDATE), "absent from every input");
    assert_eq!(judge_factorisation(book.task(&cite), &on(Some(&[0, 0]))), refused(REFUSE_GROUND_NOT_ONE_SYMBOL), "two bytes at width 1");
}
