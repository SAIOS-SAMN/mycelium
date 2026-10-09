// 21, L8w — THE BUDGET AN EXTRACTION IS GIVEN and what it does when it
// runs out: what was measured is kept, what was not reached is said.
//
//! Radical conservation. An extraction cut off used to return `None`,
//! discarding every pair form it had assembled along with the whole
//! boundary of the family, and cached nothing — so the next stroke
//! re-pulled the same family and discarded it identically. On the fleet
//! that read 469 of 474 families extracted to nothing and 3763 idle
//! strokes in a row. These tests run at the PRODUCTION budget; the file
//! beside this one runs at `1 << 20`, which is why none of them saw it.

use super::common::*;
use crate::transducer::family::*;

/// L8w, RADICAL CONSERVATION: AN EXTRACTION THE BUDGET CUTS OFF KEEPS
/// WHAT IT MEASURED AND SAYS WHAT IT DID NOT.
///
/// This test asserted the opposite, and asserting it is what let the
/// defect stand as a specification: "a bundle the budget cannot reach the
/// end of is unmeasured, never a rule over the pairs it saw", against
/// `extract_rule_with(..., stroke_n(1)).is_none()`. The reasoning — that
/// a rule read off two pairs of four is not what the family states — is
/// right about the RULE and wrong about the DISCARD. What the unreached
/// pairs state is not asserted either way: they leave as demand vertices,
/// which is a claim of ignorance and not a claim about the family, and
/// the form is not closed while it carries one.
///
/// MEASURED ON THE FLEET, 2026-09-11: 469 of 474 families extracted to
/// `None` under the production budget, every pair form assembled along
/// the way discarded with them, and the same families re-pulled and
/// re-discarded on the next stroke — 3763 idle strokes in a row, zero
/// generators held.
#[test]
fn an_extraction_cut_off_by_its_budget_keeps_what_it_reached() {
    let family = vec![
        (grid(&["1100", "0000"]), grid(&["0011", "0000"])),
        (grid(&["0000", "1100"]), grid(&["0000", "0011"])),
    ];
    let mut ph = crate::physics::Physics::default();
    // One spend reaches the first pair form and no further.
    let mut one = ph.stroke_n(1);
    let partial = extract_rule_with(&family, &NoCrystal, &mut one)
        .expect("what the budget reached is kept, never discarded with the bundle");
    assert!(
        partial.form.demand_vertices().next().is_some(),
        "and the pair it did not reach leaves as a demand, not as silence"
    );
    assert!(
        !partial.form.is_closed(),
        "a form that still carries a demand is not closed, so nothing mints it as whole"
    );
    // The same family with a budget that reaches the end is the rule of
    // the WHOLE bundle, and it asks for nothing.
    let mut whole = ph.stroke_n(1 << 20);
    let full = extract_rule_with(&family, &NoCrystal, &mut whole)
        .expect("the bundle read end to end is a rule");
    assert_eq!(
        full.form.ratio,
        family_ratio(&family).expect("scale"),
        "and it carries the family's own frame ratio"
    );
    assert_eq!(
        full.form.demand_vertices().count(),
        0,
        "read end to end, it asks for nothing"
    );
}

/// EXTRACTION IS SIZED BY THE FAMILY, NOT BY THE MACHINE (L8w). The
/// production budget was `spend_unit()` — `reserve_max()/8`, read off the
/// box's free memory at boot, three on an eight-core host — so whether a
/// family could be spelled at all depended on what else the machine was
/// doing. Every extraction test passed `stroke_n(1 << 20)` instead, a
/// budget 350,000× the fleet's, which is why the suite was green while
/// the fleet extracted nothing.
#[test]
fn the_production_budget_reads_a_family_end_to_end() {
    let family = vec![
        (grid(&["1100", "0000"]), grid(&["0011", "0000"])),
        (grid(&["0000", "1100"]), grid(&["0000", "0011"])),
    ];
    let mut ph = crate::physics::Physics::default();
    let frames = perceive_family(&family);
    let mut sized = ph.stroke_n(crate::transducer::family::extraction_spend(&family, &frames));
    let rule = extract_rule_on(&family, &frames, &NoCrystal, &mut sized)
        .expect("a family the mesh hands over is a family that can be measured");
    assert_eq!(
        rule.form.demand_vertices().count(),
        0,
        "the sized budget reaches the end of the bundle: nothing is left asking"
    );
    assert!(
        crate::transducer::family::extraction_spend(&family, &frames)
            > crate::physics::Physics::default().stroke().remaining(),
        "and it is larger than the fixed spend unit that could not reach one pair"
    );
}

/// THE PARTIAL IS CACHED (L8w). It was not: an extraction cut off by its
/// budget cached nothing, so the next stroke re-pulled the same family,
/// was handed the same purse, exhausted it at the same point and
/// discarded the same work — for as long as the node ran. Caching the
/// partial is not caching "this family has no rule"; it is holding the
/// form the mind HAS, whose demands go to the market. The entry is keyed
/// by the atlas root, so the extraction runs again the moment this mind's
/// language moves, which is the only thing that could spell more of it.
#[test]
fn a_partial_extraction_is_cached_and_advances_when_the_language_moves() {
    let family = vec![
        (grid(&["1100", "0000"]), grid(&["0011", "0000"])),
        (grid(&["0000", "1100"]), grid(&["0000", "0011"])),
    ];
    let mut ph = crate::physics::Physics::default();
    let mut cache = crate::transducer::family::ExtractMemo::new();
    let root = crate::transducer::address::Address::new(Vec::new());
    let mut small = ph.stroke_n(2);
    let partial =
        extract_rule_cached(&mut cache, b"fam", &family, root.clone(), &mut small, &NoCrystal, None)
            .expect("the small budget says what it measured");
    assert!(
        partial.form.demand_vertices().next().is_some(),
        "carrying the pair it could not reach as a demand"
    );
    // THE SECOND STROKE READS THE CACHE, not the family: the language has
    // not moved, so there is nothing more to spell, and no work is redone.
    let mut again = ph.stroke_n(1 << 20);
    let held = extract_rule_cached(&mut cache, b"fam", &family, root, &mut again, &NoCrystal, None)
        .expect("the cached partial stands");
    assert_eq!(
        again.remaining(),
        1 << 20,
        "a cache hit spends nothing: the groundhog re-extraction is gone"
    );
    assert_eq!(held.form.vertices, partial.form.vertices, "and it is the same form");
    // THE LANGUAGE MOVES, so the extraction runs again and reaches further.
    let moved = crate::transducer::address::Address::new(vec![9u8]);
    let mut large = ph.stroke_n(1 << 20);
    let whole = extract_rule_cached(&mut cache, b"fam", &family, moved, &mut large, &NoCrystal, None)
        .expect("a moved atlas re-reads the family");
    assert_eq!(
        whole.form.demand_vertices().count(),
        0,
        "and with a budget that reaches the end, the demand is discharged"
    );
}

/// 78.158a U3 — THE MEMO HOLDS NO `None`. An extraction that answers
/// `None` was held as the family's answer until the atlas root moved, so the
/// next asking at that root answered `None` again without reading. Now a
/// `None` replaces nothing: the memo does not answer for the family, so it
/// is read again on the next asking, and the rule held for the last family
/// stays.
///
/// The fixture's `None` is the family's own (two pairs in two frame
/// ratios, `family_ratio`). A stroke spent inside the extraction no longer
/// reaches `None` at all — an extraction cut off keeps what it measured
/// (the L8w test above) — so the spent-then-full shape 78.158 names has no
/// fixture; the invariant it rests on is this one.
#[test]
fn an_extraction_that_answers_none_is_never_held_for_its_family() {
    let root = crate::transducer::address::Address::new(vec![3u8]);
    let ph = &mut crate::physics::Physics::default();
    let one = vec![(grid(&["1100", "0000"]), grid(&["2200", "0000"]))];
    let ratioless = vec![(grid(&["1"]), grid(&["2"])), (grid(&["1"]), grid(&["22", "22"]))];
    let mut cache = ExtractMemo::new();
    assert!(extract_rule_cached(&mut cache, b"one", &one, root.clone(), &mut ph.stroke_n(1 << 20), &NoCrystal, None).is_some());
    assert!(cache.holds(b"one", &root), "the rule is held");
    let mut first = ph.stroke_n(1 << 20);
    assert!(extract_rule_cached(&mut cache, b"two", &ratioless, root.clone(), &mut first, &NoCrystal, None).is_none());
    assert!(!cache.holds(b"two", &root), "its None is not held for it");
    assert!(cache.holds(b"one", &root), "and the last family's rule stands");
    let mut again = ph.stroke_n(1 << 20);
    assert!(extract_rule_cached(&mut cache, b"two", &ratioless, root.clone(), &mut again, &NoCrystal, None).is_none());
    assert!(again.remaining() < 1 << 20, "asked again, the family is read again, never answered by the memo");
}
