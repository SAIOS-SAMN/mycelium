// 12.6, 26 — DEFLATION over a family: what each pair measures, and the
// pressure a bundle is under.

use super::common::*;
use crate::transducer::family::*;

/// Deflation is `deflation_of(lane_moves(after, before))` per pair, taken
/// where the form closes the pair. A raise or a miss on one pair does not
/// zero the others; a form is spoken only when every remainder is empty.
#[test]
fn form_deflation_measures_deflation_per_pair() {
let x = grid(&["1"]);
let y = grid(&["2"]);
let family_exact = vec![(x.clone(), y.clone())];
let rule = extract_rule(&family_exact, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("recolour");
let (pay, rems) = form_deflation(&rule.form, &family_exact, &NoCrystal);
assert!(
    rems.iter().all(|r| r.is_empty()),
    "an exact speak leaves empty remainders, got {rems:?}"
);
assert_ne!(pay, [0; 4], "exact speak measures the closed delta");
let before = element::measure_lanes(
    &crate::arc::refine_delta(&x, &y).expect("delta").delta,
);
assert_eq!(pay, before, "deflation of an empty remainder is the closed delta");
let x2 = grid(&["10"]);
let y2 = grid(&["23"]);
let family = vec![(x, y), (x2, y2)];
let (pay2, rems2) = form_deflation(&rule.form, &family, &NoCrystal);
assert!(rems2[0].is_empty(), "pair one closes");
assert!(
    !rems2[1].is_empty(),
    "pair two leaves a remainder, got {:?}",
    rems2[1]
);
assert_ne!(
    pay2, [0; 4],
    "a miss on pair two does not zero pair one's deflation"
);
// S9.5 — THE PAY GATE IS CLOSURE. Pair two leaves a remainder, so it pays
// NOTHING, however much of its delta the form happened to speak: a partial
// speaking is not a closed pair. Pair one closes and pays its whole delta,
// so the family's pay is exactly pair one's.
assert_eq!(
    pay2, pay,
    "an unclosed pair contributes nothing, so the pay is pair one's alone"
);
assert!(
    mint_sentence(&rule, &family, &NoCrystal).is_none(),
    "a form is spoken only when every remainder is empty"
);
}

/// A RULE THAT FAILS ON ITS OWN FAMILY IS NOT UTTERED. `mint_sentence`
/// already holds the speaker's side of `REFUSE_NOT_A_SENTENCE` — a form
/// nothing in which acts is withheld rather than spoken and refused — and
/// this is the speaker's side of `REFUSE_VERTEX_BODY` by the same argument:
/// the court replays against the family the voice already holds, so a rule
/// that fails there fails here, and uttering it spends a round trip to have
/// the refusal SEAL the address (50.9) against a rule the voice could have
/// withheld.
///
/// Measured on the fleet at e98e8a2: 954 sentences spoken, 611 refused, and
/// 488 of those `vertex does not carry its body onto its image`.
///
/// The fixture extracts a rule from one pair and mints it against a
/// DIFFERENT family — the same shape carried somewhere the rule does not
/// send it — so the form acts, its remainders are whatever they are, and the
/// vertex demonstrably does not carry its body. The court's own reader is
/// the one consulted, so the two cannot disagree.
#[test]
fn a_rule_that_does_not_carry_its_own_family_is_withheld() {
let learned = vec![(grid(&["11"]), grid(&["22"]))];
let rule = extract_rule(&learned, &mut crate::physics::Physics::default().stroke_n(1 << 20))
    .expect("a recolour extracts");
assert!(rule.form.acts(), "the rule acts, so the first clause does not decide");
assert!(
    mint_sentence(&rule, &learned, &NoCrystal).is_some(),
    "on the family it was learned from it mints"
);

// The same rule against a family whose images it does not produce.
let other = vec![(grid(&["11"]), grid(&["33"]))];
assert!(
    crate::court::verdict::body::vertices_carry_family(&rule.form, &other).is_some(),
    "the court would refuse this rule on this family"
);
assert!(
    mint_sentence(&rule, &other, &NoCrystal).is_none(),
    "so the voice withholds it instead of spending a refusal on it"
);
}

#[test]
fn bundle_pressure_is_additive_and_refuses_a_raise_on_one_pair() {
let x = grid(&["1"]);
let y = grid(&["2"]);
let f = Form::default();
let bundle = [(x.clone(), y.clone()), (x.clone(), y.clone())];
let (pairs, sum) = bundle_pressure(&f, &bundle, &NoCrystal, 0).expect("two pairs");
assert_eq!(pairs.len(), 2);
assert_eq!(
    sum[0],
    pairs[0][0] + pairs[1][0],
    "S is the sum of the pair residuals, got {sum:?} from {pairs:?}"
);
let spoken = apply_form(&f, &x, &crate::transducer::alphabet::Word::identity(), &NoCrystal);
let g = spoken.as_ref().map(|s| s.grid());
let r1 = residual_of(g.as_ref(), &y);
let mut cat = r1.clone();
cat.extend(r1);
let merged = element::measure_lanes(&cat);
assert_ne!(
    sum[3], merged[3],
    "H of a concatenated chain at shared coordinates is a merge, sum {sum:?} merged {merged:?}"
);
let pair_one_down = [1u64, 0, 0, 2];
let pair_two_raise = [0u64, 0, 0, 3];
let before_pairs = [[3u64, 0, 0, 5], [0, 0, 0, 1]];
let after_pairs = [pair_one_down, pair_two_raise];
let before_sum = [3u64, 0, 0, 6];
let after_sum = [1u64, 0, 0, 5];
let summed = crate::economy::lane_moves(&after_sum, &before_sum);
assert!(
    crate::economy::lanes_risen(&summed).is_empty()
        && !crate::economy::lanes_fallen(&summed).is_empty(),
    "the summed μ⃗ falls on some lane and rises on none: {summed:?}"
);
assert_eq!(
    pairs_raised(&before_pairs, &after_pairs),
    Some(vec![1]),
    "the reading names PAIR TWO, and the sum descending does not hide it"
);
}
