// 12.6, 26 — DESCENT over a family: the deflation each pair measures, and
// the pressure a bundle is under.

use super::common::*;
use crate::transducer::family::*;

/// Deflation is per-pair measure_lanes(delta(x,y)) − measure_lanes(delta(spoken,y)),
/// accepted only if that pair descends. A raise or a miss on one pair
/// does not zero the others; a form is spoken only when every remainder
/// is empty.
#[test]
fn form_descent_measures_deflation_per_pair() {
let x = grid(&["1"]);
let y = grid(&["2"]);
let family_exact = vec![(x.clone(), y.clone())];
let rule = extract_rule(&family_exact).expect("recolour");
let (pay, rems) = form_descent(&rule.form, &family_exact, &NoCrystal);
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
let (pay2, rems2) = form_descent(&rule.form, &family, &NoCrystal);
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
assert!(
    mint_sentence(&rule, &family, &NoCrystal).is_none(),
    "a form is spoken only when every remainder is empty"
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
assert!(
    crate::economy::descends(&after_sum, &before_sum),
    "the summed μ⃗ descends"
);
assert!(
    !pairs_descend(&before_pairs, &after_pairs),
    "a raise on pair two is refused even though the sum descends"
);
}
