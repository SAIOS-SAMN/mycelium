// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! EXPANSION 1 — the presentation as a rewriting system: rules that are
//! theorems, a reduction that terminates, critical pairs that are found
//! rather than assumed, and a completion whose confluence is checked.

use crate::transducer::alphabet::rewrite::{
    complete, critical_pairs, is_confluent, reduce, rule_plane, rules_from_relators, shortlex_lt,
    Rule,
};
use crate::transducer::alphabet::{LetterId, Word};

fn budget(n: usize) -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(n)
}

/// EVERY RULE IS A THEOREM ABOUT THE GROUP, re-derived in the exact algebra
/// and not trusted from the relator it came from. A rule that failed this
/// would rewrite one operator into another — not a simplification but a
/// corruption — and the whole system rests on it.
#[test]
fn every_rule_from_the_relators_is_a_theorem() {
    let rules = rules_from_relators();
    assert!(!rules.is_empty(), "the stated relators orient into rules");
    for r in &rules {
        assert!(
            r.holds(),
            "a rule must name one element on both sides: {:?} → {:?}",
            r.lhs.letters(),
            r.rhs.letters()
        );
        assert!(
            shortlex_lt(&r.rhs, &r.lhs),
            "and it must be oriented downward, or reduction could not terminate"
        );
    }
}

/// THE PRIME LETTERS CONTRIBUTE NO RULE. `PrimeInfinite` records that a
/// prime has infinite order — there is no relation to orient, and including
/// one would assert a relation the alphabet denies.
#[test]
fn an_infinite_order_letter_states_no_relation() {
    let rules = rules_from_relators();
    for r in &rules {
        for (id, _) in r.lhs.letters().iter().chain(r.rhs.letters()) {
            assert!(
                *id != LetterId::PRIME_5,
                "a prime of infinite order is in no finite relation"
            );
        }
    }
}

/// REDUCTION TERMINATES AND PRESERVES THE OPERATOR. Every rewrite replaces a
/// factor by a shortlex-smaller one, so the word strictly descends in a
/// well-founded order; and the element it names is unchanged, which is
/// re-derived from the rotor rather than assumed from the rules.
#[test]
fn reduction_descends_and_holds_the_operator() {
    let rules = rules_from_relators();
    let q = LetterId::quarter((0, 1)).expect("quarter");
    // Five quarters is one quarter: q⁴ = 1 is a stated relator.
    let w = Word::from_letters(&[(q, false); 5]);
    let mut b = budget(4096);
    let out = reduce(&w, &rules, &mut b).expect("reduction finishes inside the budget");
    assert!(
        w.operator_eq(&out),
        "the element is unchanged: {:?} vs {:?}",
        w.rotor(),
        out.rotor()
    );
    assert!(
        !shortlex_lt(&w, &out),
        "and the tape did not grow: {} → {}",
        w.len(),
        out.len()
    );
    assert!(
        out.operator_eq(&Word::from_letters(&[(q, false)])),
        "q⁵ is q"
    );
}

/// A CRITICAL PAIR IS AN OVERLAP, and both kinds are read off the tapes: one
/// left side inside another, and the tail of one as the head of the next.
/// Every pair a rule makes with ITSELF by suffix-prefix overlap is the
/// classic source, and it must be found or confluence cannot be checked.
#[test]
fn critical_pairs_are_found_between_rules_that_overlap() {
    let rules = rules_from_relators();
    let overlapping = rules
        .iter()
        .flat_map(|a| rules.iter().map(move |b| (a, b)))
        .filter(|(a, b)| !critical_pairs(a, b).is_empty())
        .count();
    assert!(
        overlapping > 0,
        "the stated relators overlap — q⁴ alone overlaps itself — or there is nothing to complete"
    );
    // And every pair found names one element on both sides, which is what
    // makes it a pair to RESOLVE rather than a contradiction.
    let mut b = budget(65536);
    for a in &rules {
        for c in &rules {
            for (x, y) in critical_pairs(a, c) {
                assert!(
                    x.operator_eq(&y),
                    "a critical pair is two spellings of one element: {:?} vs {:?}",
                    x.letters(),
                    y.letters()
                );
                let _ = (&mut b,);
            }
        }
    }
}

/// KNUTH–BENDIX COMPLETES THIS PRESENTATION and the result is CONFLUENT —
/// checked, not assumed. Confluence is what makes the normal form canonical:
/// every word then reduces to one tape whatever order the rules are tried
/// in, so a geodesic is a property of the element and not of the path taken
/// to it.
///
/// The completion is budgeted and reports whether it finished. A run that
/// spends its budget returns the rules found so far and `false`, and those
/// rules are still sound — every one is a theorem — but not known confluent.
#[test]
fn completion_finishes_and_the_system_is_confluent() {
    let seed = rules_from_relators();
    let mut b = budget(1 << 20);
    let (rules, finished) = complete(&seed, &mut b);
    assert!(finished, "completion of the finite part terminates");
    assert!(
        rules.len() >= seed.len(),
        "completion adds rules or leaves the seed, never loses one"
    );
    for r in &rules {
        assert!(r.holds(), "every completed rule is still a theorem");
    }
    let mut b = budget(1 << 20);
    assert_eq!(
        is_confluent(&rules, &mut b),
        Some(true),
        "every critical pair of the completed system resolves"
    );
}

/// THE GEODESIC IS THE ELEMENT'S, NOT THE PATH'S. Two different spellings of
/// one operator reduce to the same tape under a confluent system — which is
/// the whole reason to complete it, and the property `Word::geodesic`'s
/// plane-local reduction does not have.
#[test]
fn two_spellings_of_one_element_reduce_to_one_tape() {
    let seed = rules_from_relators();
    let mut b = budget(1 << 20);
    let (rules, finished) = complete(&seed, &mut b);
    assert!(finished);

    let q = LetterId::quarter((0, 1)).expect("quarter");
    // q³ and q⁻¹ are one element, spelled two ways and at two lengths.
    let long = Word::from_letters(&[(q, false), (q, false), (q, false)]);
    let short = Word::from_letters(&[(q, true)]);
    assert!(long.operator_eq(&short), "q³ = q⁻¹ in a group where q⁴ = 1");

    let mut b = budget(4096);
    let a = long.geodesic_under(&rules, &mut b).expect("reduces");
    let mut b = budget(4096);
    let c = short.geodesic_under(&rules, &mut b).expect("reduces");
    assert_eq!(
        crate::transducer::alphabet::rewrite::tape_key(&a),
        crate::transducer::alphabet::rewrite::tape_key(&c),
        "one element, one geodesic: {:?} vs {:?}",
        a.letters(),
        c.letters()
    );
}

/// THE WORD PROBLEM IS DECIDED BY THE ALGEBRA, not by the rewriting. The
/// representation is faithful over ℚ, so `operator_eq` answers in one
/// product and one projective comparison — no search, no completion. This is
/// what makes the rules above a question of SPELLING rather than of
/// identity.
#[test]
fn the_operator_is_decided_without_rewriting() {
    let q = LetterId::quarter((0, 1)).expect("quarter");
    let a = Word::from_letters(&[(q, false), (q, false), (q, false), (q, false)]);
    assert!(
        a.operator_eq(&Word::identity()),
        "q⁴ is the identity, decided by the rotor and nothing else"
    );
    let b = Word::from_letters(&[(q, false)]);
    assert!(!a.operator_eq(&b), "and q is not");
    // Projective, never coefficient-wise: Φ and −Φ are one vertex (1.5).
    let five = Word::from_letters(&[(q, false); 5]);
    assert!(five.operator_eq(&b), "q⁵ and q are one operator");
}

/// A rule whose letters share one plane names it; a commutator spans two and
/// names none. A reading for a caller that wants one plane's rules.
#[test]
fn a_rule_names_its_plane_only_when_it_has_one() {
    let rules = rules_from_relators();
    let spanning = rules.iter().filter(|r| rule_plane(r).is_none()).count();
    let single = rules.iter().filter(|r| rule_plane(r).is_some()).count();
    assert!(single > 0, "the quarter-turn rules live on one plane each");
    assert!(
        spanning > 0,
        "and the complementary-plane commutators span two, so they name none"
    );
}

/// An orientation of two identical tapes is no rule: there is nothing to
/// rewrite, and a rule from it would rewrite a word to itself forever.
#[test]
fn a_relation_between_one_tape_and_itself_is_not_a_rule() {
    let q = LetterId::quarter((0, 1)).expect("quarter");
    let w = Word::from_letters(&[(q, false)]);
    assert!(Rule::orient(&w, &w).is_none());
}
