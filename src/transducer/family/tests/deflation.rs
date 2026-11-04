// 12.6, 26 — DEFLATION over a family: what each pair measures, and the
// pressure a bundle is under.

use super::fixtures::*;
use crate::transducer::family::*;

/// Deflation is `deflation_of(lane_moves(after, before))` per pair, taken
/// where the form closes the pair. A raise or a miss on one pair does not
/// zero the others; what a form leaves unspoken on a pair it declares as
/// the demand it is (50.5), and it is spoken with that demand.
#[test]
fn form_deflation_measures_deflation_per_pair() {
crate::transducer::raster::stand_on(0);
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
let before = element::measure_lanes(&crate::transducer::family::speak::residue_of(Some(&x), &y).expect("one enclosure needs no ground"));
assert_eq!(pay, before, "deflation of an empty remainder is the closed delta");
// H1, 70.4 — `"23"` is ONE body of two hues once hue moves to the fibre,
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
let demanded: Vec<(crate::network::nerve::coord::VertexAt, crate::transducer::sentence::Demand, Vec<crate::transducer::sentence::SectionLink>)> = f
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
fn mint_sentence_admits_what_court_reconstructs() {
crate::transducer::raster::stand_on(0);
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
        crate::court::verdict::reconstruct_blind_at(&spoken, &|_| None, Some(&family), &NoCrystal, &mut super::fixtures::budget()),
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
crate::transducer::raster::stand_on(0);
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
crate::transducer::raster::stand_on(0);
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
/// (50.5): the pair's own residue is the remainder, and the measure's
/// refusal names its links.
#[test]
fn sentence_measure_from_refuses_undeclared_remainder() {
    let (x, y) = (grid(&["1"]), grid(&["2"]));
    let rems = vec![residue_of(Some(&x), &y).expect("one enclosure needs no ground")];
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
    let rems = vec![residue_of(Some(&x), &y).expect("one enclosure needs no ground")];
    let elsewhere: Vec<crate::transducer::sentence::SectionLink> = vec![(5, 5, 0, 9)];
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
crate::transducer::raster::stand_on(0);
let x = grid(&["1"]);
let y = grid(&["2"]);
let f = Form::default();
let bundle = [(x.clone(), y.clone()), (x.clone(), y.clone())];
let (pairs, sum) = bundle_pressure(&f, &bundle, &NoCrystal, 0).expect("two pairs");
assert_eq!(pairs.len(), 2);
assert_eq!(
    sum[0],
    pairs[0][0] + pairs[1][0],
    "S is the sum of the pair residues, got {sum:?} from {pairs:?}"
);
let spoken = apply_form(&f, &x, &crate::transducer::alphabet::Word::identity(), &NoCrystal);
let g = spoken.as_ref().and_then(|s| s.grid());
let r1 = crate::transducer::raster::on_ground(crate::transducer::family::transduce::Ground::One(0), || residue_of(g.as_ref(), &y))
    .expect("on a ground of 0 the unsaid sites are written");
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
    use crate::transducer::sentence::{ObstructionKind, ObstructionRefusal};
    let rule = crate::transducer::sentence::rule_of_form(crate::transducer::sentence::Form::default());
    let refused = mint_sentence_why(&rule, &[], &NoCrystal, 0).err();
    assert_eq!(refused, Some(ObstructionRefusal::MintNothingActs));
    assert_eq!(refused.map(ObstructionRefusal::kind), Some(ObstructionKind::NoQuestion));
}

/// 78.158a U1 — THE MINT'S REFUSAL NAMED WHOLE, on the solve paths as on
/// speech: its arm, its proof, its kind and the kinds of the demands the
/// refused form carries. The fixture is the deployment's 456: a form whose every
/// vertex is a demand, refused as nothing acting — and each arm's line read
/// on the same form, exactly.
#[test]
fn mint_refusal_line_names_arm_proof_kind_demands() {
    use crate::transducer::sentence::{Demand, ObstructionKind, ObstructionRefusal};
    let elsewhere: Vec<crate::transducer::sentence::SectionLink> = vec![(5, 5, 0, 9)];
    let chart = crate::transducer::stalk::invariant(&elsewhere);
    let mut f = Form::default();
    f.put_vertex(demand_vertex(Demand::Creation, &chart, elsewhere)).expect("a demand vertex has a coordinate");
    f.reindex();
    let why = mint_sentence_why(&crate::transducer::sentence::rule_of_form(f.clone()), &[], &NoCrystal, 0)
        .expect_err("a form of demands alone is not minted");
    assert_eq!(why, ObstructionRefusal::MintNothingActs);
    assert_eq!(mint_refusal_line(why, &f), "mint refused (integrity): nothing in the form acts [N] demands [creation]");
    for (r, line) in [
        (
            ObstructionRefusal::MintVertexNotCarried(ObstructionKind::Cover, "a carried site lands on the wrong hue"),
            "mint refused (carry): a carried site lands on the wrong hue [C] demands [creation]",
        ),
        (ObstructionRefusal::MintLinksWithoutDemand, "mint refused (demands): links remain and the form declares no demand [I] demands [creation]"),
        (ObstructionRefusal::MintLinksUndeclared, "mint refused (demands): links undeclared by the form's demands [I] demands [creation]"),
        (ObstructionRefusal::CitedNotHeld, "mint refused (citation): the cited form is not held here [I] demands [creation]"),
        (ObstructionRefusal::CitedSealed, "mint refused (citation): the cited address is sealed, owed to no court [I] demands [creation]"),
        (ObstructionRefusal::OwedToEveryCourt, "mint refused (citation): owed to every court and none rules it [I] demands [creation]"),
        (ObstructionRefusal::MintNoTape, "mint refused (integrity): the form has no tape at its measure [I] demands [creation]"),
        (
            ObstructionRefusal::MintAddressUndecoded,
            "mint refused (integrity): the address does not decode, so the court holds no form [I] demands [creation]",
        ),
        (ObstructionRefusal::TrialLaneRose, "mint refused (other): a lane rose [C] demands [creation]"),
    ] {
        assert_eq!(mint_refusal_line(r, &f), line);
    }
    // A form carrying no demand names none.
    assert_eq!(
        mint_refusal_line(ObstructionRefusal::MintNothingActs, &Form::default()),
        "mint refused (integrity): nothing in the form acts [N] demands []"
    );
}

/// MU-DERIVATIONS D3.2 — WHAT A PAIR DEMANDED IS READ ON THE OUTPUT'S SITES,
/// the unit its speech's residue is read in. Across `29 × 29 → 30 × 30`
/// the input is one band whose turns are seams, so the identity's speech
/// lands, and one changed output site is one link — where `K_lcm` wrote its
/// `29 × 29` block of 841. A `2 × 2` doubled to `4 × 4` with one input body
/// recoloured demands the body's four output sites. A shrink whose input
/// boundary falls inside an output compartment lays down no grid, and the
/// pair demands its whole output against nothing, never a corner reading.
#[test]
fn unspoken_residue_is_read_on_output_sites() {
    let residue = |x: &crate::transducer::family::SectionSample, y: &crate::transducer::family::SectionSample| {
        let need = crate::transducer::family::transduce::need_of(x.sample(), y.sample()).expect("both frames stand");
        crate::transducer::raster::on_ground(crate::transducer::family::transduce::Ground::One(0), || {
            crate::transducer::family::deflation::unspoken_residue(x, y, &mut crate::physics::Physics::default().stroke_n(need))
        })
        .expect("on a ground of 0 the residue is written")
    };
    let x = crate::transducer::family::SectionSample::from(vec![vec![1; 29]; 29]);
    let mut y = crate::transducer::family::SectionSample::from(vec![vec![1; 30]; 30]);
    y.paint(crate::transducer::spelled::SectionSite::on_plane([1, 2]), 5).expect("a site of the sample");
    assert_eq!(residue(&x, &y), vec![(1, 2, 1, 5)]);
    assert_eq!(
        residue(&grid(&["10", "00"]), &grid(&["2200", "2200", "0000", "0000"])),
        vec![(0, 0, 1, 2), (0, 1, 1, 2), (1, 0, 1, 2), (1, 1, 1, 2)]
    );
    let shrink = grid(&["12"]);
    let out = grid(&["3"]);
    assert_eq!(
        residue(&grid(&["1", "2"]), &grid(&["1", "1", "3"])),
        crate::transducer::raster::on_ground(crate::transducer::family::transduce::Ground::One(0), || {
            crate::transducer::family::speak::residue_of(None, &grid(&["1", "1", "3"]))
        })
        .expect("on a ground of 0 the residue is written")
    );
    assert_eq!(residue(&shrink, &out), vec![(0, 0, 0, 3)]);
}

/// MU-DERIVATIONS D3.2 — A CLOSED PAIR PAYS WHAT IT DEMANDED, ON ONE RECORD:
/// on a family of unequal enclosures (`2 × 2 → 4 × 4`), where the form
/// leaves every remainder empty the pay is the measure of each pair's
/// unspoken residue on the output's sites — before and after read in one
/// unit.
#[test]
fn deflation_closure_pays_before_on_one_record() {
    crate::transducer::raster::stand_on(0);
    let family = vec![
        (grid(&["10", "00"]), grid(&["2200", "2200", "0000", "0000"])),
        (grid(&["00", "01"]), grid(&["0000", "0000", "0022", "0022"])),
    ];
    let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("a scaled recolour extracts");
    let (pay, rems) = form_deflation(&rule.form, &family, &NoCrystal);
    assert!(rems.iter().all(Vec::is_empty), "the form closes both pairs: {rems:?}");
    let mut before = [0u64; 4];
    for (x, y) in &family {
        let need = crate::transducer::family::transduce::need_of(x.sample(), y.sample()).expect("both frames stand");
        let r =crate::transducer::raster::on_ground_of(&family, || {
            crate::transducer::family::deflation::unspoken_residue(x, y, &mut crate::physics::Physics::default().stroke_n(need))
        })
        .expect("on the family's ground the residue is written");
        for (b, m) in before.iter_mut().zip(element::measure_lanes(&r)) {
            *b += m;
        }
    }
    assert_eq!(pay, crate::economy::deflation_of(&crate::economy::lane_moves(&[0; 4], &before)));
}

/// O1 (ruled 2026-11-01) — μ IS A SHARE OF EACH PAIR'S OWN FRAME. A slide of
/// two columns closes a `1 × 4` pair and a `1 × 6` pair; each moves four
/// sites between ground and painted, so S is `4/4 + 4/6 = 5/3`, never the
/// count 8; each carried a mass of 4, so 𝒯 is the flux over the carried mass
/// 8 taken as the frames' share of it, `flux · (4/4 + 4/6) / 8 = flux ·
/// 5/24`; and H, the shell count, stands as the count does.
#[test]
fn deflation_reads_mu_as_share_of_each_frame() {
    crate::transducer::raster::stand_on(0);
    let family = vec![(grid(&["1100"]), grid(&["0011"])), (grid(&["110000"]), grid(&["001100"]))];
    let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the slide extracts");
    let read = form_deflation_read(&rule.form, &family, &NoCrystal, &crate::transducer::alphabet::Word::identity());
    assert!(read.remainders.iter().all(Vec::is_empty), "the slide closes both pairs: {:?}", read.remainders);
    let q = |n: u64, d: u64| crate::core::Q::new(n.into(), d.into());
    assert_eq!(read.pay[0], 8, "the measure counts eight sites on S");
    assert_eq!(read.mu.lane(0), &q(5, 3), "μ on S: 4 of 4 and 4 of 6 sites");
    let t = crate::transducer::sentence::presentation::LANE_T;
    assert!(read.pay[t] > 0, "the slide carries a flux");
    assert_eq!(read.mu.lane(t), &(q(read.pay[t], 1) * q(5, 24)), "𝒯: the flux over the carried mass, as the frames' share of it");
    assert_eq!(read.mu.lane(3), &q(read.pay[3], 1), "H stands as its count");
}
