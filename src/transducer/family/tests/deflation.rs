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
// H1, 70.4 — `"23"` is ONE body of two hues once hue moves to the fiber,
// so the second pair's intent (an output the rule cannot explain) is
// written with a gap rather than a colour change.
let x2 = grid(&["100"]);
let y2 = grid(&["203"]);
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
let demanded: Vec<(crate::network::nerve::coord::VertexAt, crate::transducer::sentence::Demand, Vec<crate::transducer::sentence::Link>)> = f
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

/// 34.2, 54.3 — THE MINT AND THE COURT READ ONE FORM. The mint declares what
/// its form left unspoken and measures the form the court will hold — read
/// from the address and receipts it mints, by the court's one decoder —
/// never the form in its hand. So on the family it minted over, the form the
/// court decodes leaves nothing undeclared, and the court's own replay
/// admits what the mint admitted.
#[test]
fn mint_sentence_admits_what_court_replays() {
    let first = vec![(grid(&["1"]), grid(&["2"]))];
    let rule = extract_rule(&first, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("recolour");
    let family = vec![(grid(&["1"]), grid(&["2"])), (grid(&["100"]), grid(&["203"]))];
    let spoken = mint_sentence(&rule, &family, &NoCrystal, 0).expect("spoken with its demand");
    let held = crate::transducer::sentence::from_element_form(&spoken).expect("the form the court holds");
    let (_, rems) = form_deflation(&held, &family, &NoCrystal);
    assert!(
        crate::court::verdict::body::undeclared_remainder(&rems, &held).is_empty(),
        "on the form the court holds, the mint left nothing undeclared: {rems:?}"
    );
    assert_eq!(
        crate::court::verdict::blind_replay_at(&spoken, &|_| None, Some(&family), &NoCrystal, &mut super::common::budget()),
        None,
        "the court admits on the same family what the mint admitted"
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
/// The fixture extracts a rule from one pair and mints it against a
/// DIFFERENT family — the same shape carried somewhere the rule does not
/// send it — so the form acts, its remainders are whatever they are, and the
/// vertex demonstrably does not carry its body. The court's own reader is
/// the one consulted, so the two cannot disagree.
#[test]
fn mint_sentence_withholds_rule_not_carrying_family() {
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

/// LANE 𝒯 CARRIES NO SPAN (78.149 (a), the ruling of
/// 2026-10-08, reversing R2).
///
/// R2 wrote the span the bundle stood over into lane 2 of every minted
/// magnitude, and this test asserted that a sentence standing 37 receipts
/// carried 37 there. By ruling the span is not μ: waiting is not work, and a
/// word escrowed and paid at its magnitude was paid for having waited. The
/// word's magnitude is its discharge on 𝒯 as on every lane — what its chains
/// measure there — and the span it stood is written nowhere.
///
/// What R2 measured downstream (𝒯 never spent on the reserve, the dwell's 𝒯
/// lane reading zero) is metabolism, not price: the book's pressure on 𝒯 is
/// the standing extent of its listings (`remeasure_pressure`), read there.
#[test]
fn mint_sentence_carries_no_span_on_temporal_lane() {
    let family = vec![(grid(&["11"]), grid(&["22"]))];
    let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("a recolour extracts");

    let at_once = mint_sentence(&rule, &family, &NoCrystal, 0).expect("it mints");
    let stood = mint_sentence(&rule, &family, &NoCrystal, 37).expect("it mints");
    assert_eq!(stood.magnitude()[2], 0, "a sentence that stood over 37 receipts carries none of them");
    assert_eq!(stood.magnitude(), at_once.magnitude(), "the span moves no lane");
    assert!(!stood.lanes()[2], "and the word does not occupy 𝒯 for having waited");
    let held = crate::transducer::sentence::from_element_form(&stood).expect("the form the court holds");
    let (pay, _) = form_deflation(&held, &family, &NoCrystal);
    assert_eq!(stood.magnitude(), pay, "its magnitude is the deflation it closed, and only that");
}

/// A form that declares no demand and leaves a remainder measures nothing
/// (50.5): the pair's own residual is the remainder, and the measure's
/// refusal names its links.
#[test]
fn sentence_measure_from_refuses_undeclared_remainder() {
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
fn sentence_measure_from_refuses_uncovered_remainder() {
    let (x, y) = (grid(&["12"]), grid(&["34"]));
    let rems = vec![residual_of(Some(&x), &y)];
    let elsewhere: Vec<crate::transducer::sentence::Link> = vec![(5, 5, 0, 9)];
    let chart = crate::transducer::stalk::invariant(&elsewhere);
    let mut f = Form::default();
    f.put_vertex(demand_vertex(crate::transducer::sentence::Demand::Creation, &chart, elsewhere))
        .expect("a demand vertex has a coordinate");
    f.reindex();
    assert!(f.demand_vertices().next().is_some(), "the form declares a demand");
    assert!(
        sentence_measure_from(&f, &[(x, y)], 0, [0; 4], &rems).is_none(),
        "but not the links the pair leaves, so it measures nothing"
    );
}

/// Bundle pressure is additive and refuses a raise on one pair.
#[test]
fn bundle_pressure_adds_and_names_raised_pair() {
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

/// A.17 — THE MINT'S REFUSAL IS NAMED, never a silent `None`: a form nothing
/// in which acts is refused as that, with no question standing (78.9, N).
#[test]
fn mint_sentence_why_refuses_form_where_nothing_acts() {
    use crate::transducer::sentence::{ObstructionKind, Refused};
    let rule = crate::transducer::sentence::rule_of_form(crate::transducer::sentence::Form::default());
    let refused = mint_sentence_why(&rule, &[], &NoCrystal, 0).err();
    assert_eq!(refused, Some(Refused::MintNothingActs));
    assert_eq!(refused.map(Refused::kind), Some(ObstructionKind::N));
}

/// 78.158a U1 — THE MINT'S REFUSAL NAMED WHOLE, on the solve paths as on
/// speech: its arm, its proof, its kind and the kinds of the demands the
/// refused form carries. The fixture is the deployment's 456: a form whose every
/// vertex is a demand, refused as nothing acting — and each arm's line read
/// on the same form, exactly.
#[test]
fn mint_refusal_line_names_arm_proof_kind_demands() {
    use crate::transducer::sentence::{Demand, ObstructionKind, Refused};
    let elsewhere: Vec<crate::transducer::sentence::Link> = vec![(5, 5, 0, 9)];
    let chart = crate::transducer::stalk::invariant(&elsewhere);
    let mut f = Form::default();
    f.put_vertex(demand_vertex(Demand::Creation, &chart, elsewhere)).expect("a demand vertex has a coordinate");
    f.reindex();
    let why = mint_sentence_why(&crate::transducer::sentence::rule_of_form(f.clone()), &[], &NoCrystal, 0)
        .expect_err("a form of demands alone is not minted");
    assert_eq!(why, Refused::MintNothingActs);
    assert_eq!(mint_refusal_line(why, &f), "mint refused (integrity): nothing in the form acts [N] demands [creation]");
    for (r, line) in [
        (
            Refused::MintVertexNotCarried(ObstructionKind::C, "a lamina lands on the wrong hue"),
            "mint refused (carry): a lamina lands on the wrong hue [C] demands [creation]",
        ),
        (Refused::MintLinksWithoutDemand, "mint refused (demands): links remain and the form declares no demand [I] demands [creation]"),
        (Refused::MintLinksUndeclared, "mint refused (demands): links undeclared by the form's demands [I] demands [creation]"),
        (Refused::CitedNotHeld, "mint refused (citation): the cited form is not held here [I] demands [creation]"),
        (Refused::CitedSealed, "mint refused (citation): the cited address is sealed, owed to no court [I] demands [creation]"),
        (Refused::OwedToEveryCourt, "mint refused (citation): owed to every court and none rules it [I] demands [creation]"),
        (Refused::MintNoTape, "mint refused (integrity): the form has no tape at its measure [I] demands [creation]"),
        (
            Refused::MintAddressUndecoded,
            "mint refused (integrity): the address does not decode, so the court holds no form [I] demands [creation]",
        ),
        (Refused::TrialLaneRose, "mint refused (other): a lane rose [C] demands [creation]"),
    ] {
        assert_eq!(mint_refusal_line(r, &f), line);
    }
    // A form carrying no demand names none.
    assert_eq!(
        mint_refusal_line(Refused::MintNothingActs, &Form::default()),
        "mint refused (integrity): nothing in the form acts [N] demands []"
    );
}
