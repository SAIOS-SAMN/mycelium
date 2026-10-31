// 78.149 — μ IS DISCHARGE (the escrow rulings, 2026-10-08): the
// span a word stood is not μ, a claim is paid the closer's measured
// discharge and never more, and a word that names no body escrows and is
// paid nothing.

use super::common::*;
use crate::cognition::*;
use crate::transducer::address::Address;
use crate::transducer::family::{cipher_form, extract_rule, form_deflation, mint_sentence, sentence_measure, NoCrystal, TrainPair};
use crate::transducer::sentence::{as_element_form, from_element_form};

/// A recolour of two bodies, each pair closed by the one rule.
fn recoloured() -> Vec<TrainPair> {
    vec![
        (digits(&["1100", "0000"]), digits(&["0022", "0000"])),
        (digits(&["0000", "1100"]), digits(&["0000", "0022"])),
    ]
}

/// Two separated bodies, both carried two columns along (70.4), and the
/// voice's rule over it, measured and spoken (as `tests::solve` builds it).
fn carried() -> Vec<TrainPair> {
    vec![(digits(&["1100200"]), digits(&["0011002"]))]
}

fn rule_over(family: &[TrainPair]) -> crate::transducer::element::Element {
    let [(x, y)] = family else { panic!("one pair") };
    let f = cipher_form(x, y);
    let mag = sentence_measure(&f, family, &NoCrystal, 0).expect("every body is spoken");
    as_element_form(&f, mag).expect("a tape")
}

/// 78.149 (a) — A WORD THAT STOOD A SPAN MINTS NO SPAN AND ESCROWS ITS
/// DISCHARGE. The same rule minted at once and after standing 37 is one
/// word with one magnitude, 𝒯 zero; that magnitude is exactly the deflation
/// the held form measures over its family, `d`; and the court that hears it
/// on that family escrows `price(d)` and nothing for the span.
#[test]
fn mint_sentence_escrows_discharge_not_span() {
    let family = recoloured();
    let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("extract");
    let at_once = mint_sentence(&rule, &family, &NoCrystal, 0).expect("it mints");
    let stood = mint_sentence(&rule, &family, &NoCrystal, 37).expect("it mints");
    // 𝒯 is the ground the rule's motions covered (its null flux), never the
    // span it stood: standing 37 adds nothing to it.
    assert_eq!(stood.magnitude()[2], at_once.magnitude()[2], "the span stood is not written over 𝒯");
    assert_eq!(stood.magnitude(), at_once.magnitude(), "standing changes nothing the word measures");
    assert_eq!(stood.address(), at_once.address(), "and nothing it is");

    // The discharge, measured independently on the form the court holds.
    let held = from_element_form(&stood).expect("a form");
    let (d, rems) = form_deflation(&held, &family, &NoCrystal);
    assert!(rems.iter().all(|r| r.is_empty()), "the rule closes every pair: {rems:?}");
    assert!(d.iter().any(|&x| x > 0), "and discharges something: {d:?}");
    assert_eq!(stood.magnitude(), d, "a closed sentence's magnitude is its discharge, 𝒯 included");

    let mut k = Court::new();
    let task = crate::registry::task_id(&crate::arc::Task { train: family.clone(), test: Vec::new() }).expect("a task");
    k.bind_task_family(&task, family);
    assert_eq!(
        k.perceive_speech([1; 32], task.as_bytes(), std::slice::from_ref(&stood)),
        vec![crate::anastome::dialogue::SPEECH_STOOD]
    );
    let escrow = k.treasury().escrow_of(stood.address().as_bytes()).map(|e| e.mu).expect("escrowed");
    assert_eq!(escrow, crate::economy::price(&d, &k.physics().pressure), "escrowed at price(d), exactly");
    assert_eq!(k.discharge_of(stood.address().as_bytes(), &stood), d, "the one reader reads the same discharge");
}

/// 78.149 (b) — A CLOSURE PAYS EACH CLAIM THE CLOSER'S MEASURED DISCHARGE,
/// LANE-WISE CAPPED BY ITS ESCROW, AND NOTHING ABOVE IT IS MINTED. Two
/// listings are bound to the task a solve closes: one escrowed above the
/// solve's discharge on every lane, one below it. The first voice is paid
/// exactly the discharge, the second exactly its escrow; the ledger mints
/// those two vectors and nothing else.
#[test]
fn pay_solve_pays_discharge_capped_by_escrow() {
    let mut k = Court::new();
    let family = carried();
    let rule = rule_over(&family);
    let pay = match k.judge_solve(&rule, &family) {
        SolveVerdict::Solved { pay, .. } => pay,
        other => panic!("the replay closes every pair: {other:?}"),
    };
    assert!(pay.iter().any(|&m| m > 0), "the solve discharges something: {pay:?}");

    let task = [7u8; 32];
    let (above, below) = (Address::from_bytes([41u8; 32]), Address::from_bytes([42u8; 32]));
    let (rich, poor) = ([3u8; 32], [4u8; 32]);
    let held_above: [u64; 4] = std::array::from_fn(|l| pay[l] + 3);
    let held_below: [u64; 4] = std::array::from_fn(|l| pay[l] / 2);
    let anchor = k.anchor();
    k.treasury.escrow_award(&above, vec![rich], held_above, anchor);
    k.treasury.escrow_award(&below, vec![poor], held_below, anchor);
    k.bind_listing_task(&above, task);
    k.bind_listing_task(&below, task);

    // Not the task's first closure: the solver is not paid, so every row
    // minted is a released claim.
    let _ = k.pay_solve(&task, [9; 32], &rule, pay, false);
    assert_eq!(k.treasury().balance_mu(&rich), pay, "an escrow above the discharge pays the discharge");
    assert_eq!(k.treasury().balance_mu(&poor), held_below, "an escrow below it caps it");
    let minted = k.treasury().minted().expect("minted adds up");
    assert_eq!(
        minted,
        std::array::from_fn::<u64, 4, _>(|l| pay[l] + held_below[l]),
        "nothing above the discharge was minted"
    );
    assert!(k.treasury().escrow_of(above.as_bytes()).is_none() && k.treasury().escrow_of(below.as_bytes()).is_none(), "both claims released");
}

/// 78.149 (c) — A LETTER ESCROWS AND IS PAID NOTHING. A rotor the
/// seventeen cannot spell is docketed as a letter carrying a demand's
/// bodies: its magnitude measures them (its existence as a word), its escrow
/// is the zero vector, and its ruling mints nothing.
#[test]
fn open_unspellable_letter_escrows_and_pays_nothing() {
    use crate::transducer::alphabet::{LetterId, Word};
    let mut k = Court::new();
    let quarter = LetterId::turn_90((0, 1)).expect("the 90° turn");
    let rotor = Word::from_letters(&[(quarter, false)]).rotor();
    let chart = crate::transducer::stalk::Invariant::counts(&[3]);
    let bodies: Vec<crate::transducer::sentence::Link> = vec![(0, 0, 0, 1), (0, 1, 0, 1)];
    let letter = crate::transducer::sentence::generator_form(&rotor, &chart, &bodies).expect("a letter");
    assert_ne!(letter.magnitude(), [0; 4], "the letter measures the bodies it was opened for");
    let addr = letter.address();

    let voice = [1u8; 32];
    k.open_unspellable(&rotor, &chart, &bodies, &voice);
    assert!(k.provisional.contains(addr.as_bytes()), "the letter is docketed");
    let escrow = k.treasury().escrow_of(addr.as_bytes()).map(|e| e.mu).expect("escrowed to its voice");
    assert_eq!(escrow, [0; 4], "a word that names no body escrows nothing");

    assert!(k.apply_verdict(addr.as_bytes(), crate::anastome::dialogue::RULING_UPHELD), "upheld");
    assert_eq!(k.treasury().balance_mu(&voice), [0; 4], "and is paid nothing");
    assert_eq!(k.treasury().minted().expect("minted adds up"), [0; 4], "nothing is minted");
}

/// 78.149 (c) — A CALLUS ESCROWS AND IS PAID NOTHING, even bound to a
/// family it could be measured on: it stands at no frame, so its discharge
/// there is the zero vector; its holonomic unit is its existence as a word.
#[test]
fn list_sentence_callus_escrows_and_pays_nothing() {
    use crate::transducer::alphabet::{Derivation, LetterId, Word};
    let mut k = Court::new();
    let quarter = LetterId::turn_90((0, 1)).expect("the 90° turn");
    let chart = crate::transducer::stalk::Invariant::counts(&[3]);
    let callus = crate::transducer::sentence::callus_form(
        Derivation::spelled(Word::from_letters(&[(quarter, false)])),
        std::slice::from_ref(&chart),
    )
    .expect("a callus");
    assert_eq!(callus.magnitude(), [0, 0, 0, 1], "its magnitude is the one unit of a word that names no body");
    let addr = callus.address();

    let family = recoloured();
    let task = [8u8; 32];
    k.bind_task_family(task, family);
    k.bind_listing_task(&addr, task);
    let discharge = k.discharge_of(addr.as_bytes(), &callus);
    assert_eq!(discharge, [0; 4], "it discharges nothing on the family it is bound to");

    let voice = [2u8; 32];
    let _ = k.list_sentence(&callus, voice, discharge);
    let escrow = k.treasury().escrow_of(addr.as_bytes()).map(|e| e.mu).expect("escrowed to its voice");
    assert_eq!(escrow, [0; 4], "and escrows nothing");

    assert!(k.apply_verdict(addr.as_bytes(), crate::anastome::dialogue::RULING_UPHELD), "upheld");
    assert_eq!(k.treasury().balance_mu(&voice), [0; 4], "and is paid nothing");
    assert_eq!(k.treasury().minted().expect("minted adds up"), [0; 4], "nothing is minted");
}
