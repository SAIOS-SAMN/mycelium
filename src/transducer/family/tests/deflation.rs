// 12.6, 26 — DEFLATION over a family: what each pair measures, and the
// pressure a bundle is under.

use super::common::*;
use crate::transducer::family::*;

/// Deflation is `deflation_of(lane_moves(after, before))` per pair, taken
/// where the form closes the pair. A raise or a miss on one pair does not
/// zero the others; what a form leaves unspoken on a pair it declares as
/// the demand it is (50.5), and it is spoken with that demand.
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
    &crate::transducer::family::refine::refine_delta(&x, &y).expect("delta"),
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
// 50.5 — THE UNSPOKEN REMAINDER IS THE DEMAND. Pair two's remainder is a
// site the form spoke nothing at, so the mint declares it as a `Source`
// demand on that site's chart and the sentence is spoken with it; the
// court's own reader finds nothing undeclared. A mint that spoke a form
// only when every remainder was empty would withhold it for saying less
// than the whole.
let spoken = mint_sentence(&rule, &family, &NoCrystal, 0).expect("spoken with its demand");
let f = crate::transducer::sentence::from_element_form(&spoken).expect("a sentence tape");
let demanded: Vec<(u16, crate::transducer::sentence::Demand, Vec<crate::transducer::sentence::Link>)> = f
    .demand_vertices()
    .map(|(i, k, _, links)| (i, k.clone(), links.to_vec()))
    .collect();
let [(_, kind, links)] = demanded.as_slice() else {
    panic!("one demand declares the one unspoken remainder, got {}", demanded.len())
};
assert!(matches!(kind, crate::transducer::sentence::Demand::Source { .. }), "unspoken links are a Source demand");
let unspoken: Vec<(i64, i64)> = rems2[1].iter().filter(|c| c.2 == 0).map(|c| (c.0, c.1)).collect();
let declared: Vec<(i64, i64)> = links.iter().map(|c| (c.0, c.1)).collect();
assert_eq!(declared, unspoken, "the demand names exactly the sites the form spoke nothing at");
assert!(
    crate::court::verdict::body::undeclared_remainder(&rems2, &f).is_empty(),
    "and the court's reader finds nothing undeclared"
);
assert_ne!(spoken.magnitude(), [0; 4], "the sentence measures what it closed and what it asks");
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
    mint_sentence(&rule, &learned, &NoCrystal, 0).is_some(),
    "on the family it was learned from it mints"
);

// The same rule against a family whose images it does not produce.
let other = vec![(grid(&["11"]), grid(&["33"]))];
assert!(
    crate::court::verdict::body::vertices_carry_frames(&rule.form, &other).is_some(),
    "the court would refuse this rule on this family"
);
assert!(
    mint_sentence(&rule, &other, &NoCrystal, 0).is_none(),
    "so the voice withholds it instead of spending a refusal on it"
);
}

/// LANE 𝒯 CARRIES THE SPAN THE BUNDLE STOOD OVER.
///
/// `mint_sentence`'s own line has always read "Lane 𝒯 is the scene-state
/// span the bundle stood over", and `sentence_measure` computed μ⃗ through
/// `measure_lanes` alone — which returns a LITERAL ZERO on lane 2 under a
/// comment deferring the measurement to the mint. The mint did not take it
/// either. Measured at 820678a: **197 spoken magnitudes, 0 with a 𝒯
/// component**; every word this engine has ever uttered was `[S, G, 0, H]`.
///
/// The consequence is the whole lane vector, not one slot.
/// `word.lanes()[2]` false → the court's `spend(word.lanes())` never spends
/// 𝒯 → `reserve[2]` never depletes → `reserve_deficit[2]` zero →
/// `yield_surface[2]` zero → the dwell's 𝒯 lane read 0 in **1247 of 1247**
/// readings, while S, G and H — all reduced to the same reserve deficit —
/// were equal in 92.5% of them and took two distinct values between them.
/// Law 5 gives four lanes so they can carry independent content.
///
/// A span of zero is a reading: a form minted in the stroke it was extracted
/// in stood over no receipts. What it is not is the only value available.
#[test]
fn a_minted_sentence_carries_the_span_on_the_temporal_lane() {
    let family = vec![(grid(&["11"]), grid(&["22"]))];
    let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("a recolour extracts");

    let at_once = mint_sentence(&rule, &family, &NoCrystal, 0).expect("it mints");
    assert_eq!(
        at_once.magnitude()[2],
        0,
        "a bundle that stood over no receipts measures zero on 𝒯"
    );

    let stood = mint_sentence(&rule, &family, &NoCrystal, 37).expect("it mints");
    assert_eq!(stood.magnitude()[2], 37, "and one that stood over 37 carries 37");

    // The other three lanes are the deflation's own and do not move with it.
    let (a, b) = (at_once.magnitude(), stood.magnitude());
    assert_eq!([a[0], a[1], a[3]], [b[0], b[1], b[3]], "𝒯 is its own lane");
    assert!(
        stood.lanes()[2],
        "and the word now OCCUPIES 𝒯, which is what the court spends against"
    );
    assert!(!at_once.lanes()[2], "while a zero span occupies nothing");
}

/// A form that declares no demand and leaves a remainder measures nothing
/// (50.5): the pair's own residual is the remainder, and the measure's
/// refusal names its links.
#[test]
fn a_remainder_no_demand_declares_is_refused_at_the_measure() {
    let (x, y) = (grid(&["1"]), grid(&["2"]));
    let rems = vec![residual_of(Some(&x), &y)];
    assert!(!rems[0].is_empty(), "the pair leaves a remainder");
    assert!(
        sentence_measure_from(&Form::default(), &[(x, y)], 0, [0; 4], &rems).is_none(),
        "no demand declares it, so the form measures nothing"
    );
}

/// A form whose demand does not cover the remainder measures nothing, by
/// the court's own reader (50.5, 23.1 step 3): a demand declares links,
/// not an exemption.
#[test]
fn a_remainder_the_demands_do_not_cover_is_refused_at_the_measure() {
    let (x, y) = (grid(&["12"]), grid(&["34"]));
    let rems = vec![residual_of(Some(&x), &y)];
    let elsewhere: Vec<crate::transducer::sentence::Link> = vec![(5, 5, 0, 9)];
    let chart = crate::transducer::stalk::invariant(&elsewhere);
    let mut f = Form::default();
    f.vertices.push(demand_vertex(crate::transducer::sentence::Demand::Creation, &chart, elsewhere));
    f.reindex();
    assert!(f.demand_vertices().next().is_some(), "the form declares a demand");
    assert!(
        sentence_measure_from(&f, &[(x, y)], 0, [0; 4], &rems).is_none(),
        "but not the links the pair leaves, so it measures nothing"
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
let g = spoken.as_ref().and_then(|s| s.grid());
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
