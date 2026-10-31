// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 42.6 AS AMENDED, L12 — THE FOUR COVERS AND THE MEET. A reduction is
// bounded by a presentation only when it runs against a basis every lane
// the target occupies holds. These are claims the four covers could not
// make while `adjoined_basis` asked lane `3` by literal and nothing ever
// read lanes 0, 1 or 2: four `GeneratorIndex`es were filled and three of
// them had no consumer.

use crate::transducer::address::{Address, TAG_FORM};
use crate::transducer::alphabet::{Derivation, LetterId, Word};
use crate::transducer::element;
use crate::transducer::family::{factor_versor, occupied_lanes, Crystal};
use crate::transducer::sentence::{Form, Vertex};
use crate::transducer::stalk::Invariant;

fn turn_90(n: usize) -> Word {
    let q = LetterId::turn_90((0, 1)).expect("the 90° turn on the sensor plane");
    Word::from_letters(&vec![(q, false); n])
}

/// A SWAP OF TWO HUES: a real 2-cycle, so `CycleType::of` is not
/// `Identity` and the `𝒢` axis is occupied. A hue map of fixed points
/// recolours nothing and occupies nothing, which is why the pairs here are
/// crossed.
fn swap() -> Vec<(u8, u8)> {
    vec![(1, 2), (2, 1)]
}

/// A one-word-vertex form carrying `w` as its action and `gauge` as its
/// hue map: a generator as the language holds one (L5).
fn generator_form(w: Word, gauge: Vec<(u8, u8)>, tag: u8) -> (Address, Form) {
    let mut f = Form::default();
    f.put_vertex(Vertex::word_on(
        Derivation::spelled(w),
        // `tag` named the vertex and now charts it (54.2).
        Invariant::counts(&[u64::from(tag)]),
        Box::new(element::identity_versor()),
        gauge,
        [0u64],
        None,
    ))
    .expect("a generator's word vertex has a class");
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();
    (Address::from_bytes([TAG_FORM, tag]), f)
}

/// A mind holding its generators in the FOUR COVERS, filled by
/// `admit_generators_by_lane` and drained by `generators_under_lane` —
/// the same pair the epigenome uses, so what this fixture proves about the
/// read is what the live mind does (§3.4). `generators_at_meet` is the
/// trait's own, unoverridden.
struct Lanes {
    index: crate::transducer::sentence::LaneIndex,
    form: Form,
}

impl Lanes {
    fn holding(w: Word, gauge: Vec<(u8, u8)>, tag: u8) -> Lanes {
        let (address, form) = generator_form(w, gauge, tag);
        let mut index = crate::transducer::sentence::LaneIndex::default();
        crate::transducer::sentence::admit_generators_by_lane(&mut index, &address, &form);
        Lanes { index, form }
    }
}

impl Crystal for Lanes {
    fn on_class(&self, _c: &Invariant) -> Vec<(&element::Element, &Form, crate::transducer::stalk::Standing)> {
        Vec::new()
    }
    fn on_body(&self, _c: &Invariant) -> Vec<(&element::Element, &Form, crate::transducer::stalk::Standing)> {
        Vec::new()
    }
    fn speak_form(
        &self,
        f: &Form,
        x: &crate::transducer::family::Grid,
        placement: &Word,
    ) -> Option<crate::transducer::family::Speech> {
        crate::transducer::family::apply_form(f, x, placement, self)
    }
    fn by_address(&self, _a: &[u8]) -> Option<(&element::Element, std::borrow::Cow<'_, Form>)> {
        None
    }
    fn on_axis(&self, _c: &Invariant, _axis: crate::transducer::sentence::Axis) -> Vec<Word> {
        Vec::new()
    }
    /// The whole-language read, kept answering the `𝒽` cover so a test can
    /// show the difference between it and the meet.
    fn generators_under(
        &self,
        chart: &Invariant,
        budget: &mut crate::physics::Budget,
    ) -> Vec<(Address, Address, Word)> {
        self.generators_on_lane(3, chart, budget)
    }
    fn generators_on_lane(
        &self,
        lane: usize,
        chart: &Invariant,
        budget: &mut crate::physics::Budget,
    ) -> Vec<(Address, Address, Word)> {
        crate::transducer::sentence::generators_under_lane(&self.index, lane, chart, budget)
            .into_iter()
            .filter_map(|(form, vertex)| {
                let w = crate::transducer::sentence::word_at_vertex(&self.form, vertex.as_bytes())?;
                Some((form, vertex, w))
            })
            .collect()
    }
}

fn budget() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 20)
}

fn lanes_of(v: &[(usize, Invariant)]) -> Vec<usize> {
    v.iter().map(|(l, _)| *l).collect()
}

/// THE LANES A TARGET OCCUPIES ARE READ, NOT ASSUMED. A turn alone is
/// content on `𝒽`; the same turn carried with a hue swap is content on
/// `𝒢` AND `𝒽`, two independent covers and never one.
#[test]
fn occupied_lanes_adds_gauge_lane_for_recolouring_turn() {
    use crate::transducer::sentence::CycleType;
    let turn = turn_90(2).rotor();
    let alone = occupied_lanes(&turn, &CycleType::Identity);
    assert_eq!(lanes_of(&alone), vec![3], "a turn is holonomic and nothing else");
    let with_hue = occupied_lanes(&turn, &CycleType::of(&swap()));
    assert_eq!(
        lanes_of(&with_hue),
        vec![1, 3],
        "the same turn that recolours carries content on the gauge cover too"
    );
}

/// A GENERATOR IS FILED ON EVERY LANE IT OCCUPIES. This is the admission
/// half, and it must agree with the read half exactly: a generator filed
/// under one class vector and looked up under another is held and
/// unreachable, which is indistinguishable from not held.
#[test]
fn admit_generators_by_lane_files_recolouring_on_gauge() {
    use crate::transducer::sentence::CycleType;
    let turn = turn_90(1).rotor();
    let holonomic = Lanes::holding(turn_90(1), Vec::new(), 1);
    let recolouring = Lanes::holding(turn_90(1), swap(), 2);
    let h_chart = crate::transducer::sentence::versor_chart_of(&turn).expect("the 𝒽 chart");
    let g_chart = {
        let cv = occupied_lanes(&turn, &CycleType::of(&swap()));
        cv.iter().find(|(l, _)| *l == 1).map(|(_, c)| c.clone()).expect("the 𝒢 chart")
    };
    assert_eq!(
        holonomic.generators_on_lane(3, &h_chart, &mut budget()).len(),
        1,
        "the turning generator is on 𝒽"
    );
    assert_eq!(
        holonomic.generators_on_lane(1, &g_chart, &mut budget()).len(),
        0,
        "and it is NOT on 𝒢, because it recolours nothing"
    );
    assert_eq!(
        recolouring.generators_on_lane(3, &h_chart, &mut budget()).len(),
        1,
        "the recolouring generator is on 𝒽 too — it still turns"
    );
    assert_eq!(
        recolouring.generators_on_lane(1, &g_chart, &mut budget()).len(),
        1,
        "and on 𝒢 as well: one generator, two covers, not a double count"
    );
}

/// THE MEET REFUSES WHAT ONE LANE WOULD HAVE OFFERED — the whole claim of
/// L12. The mind holds the turn and nothing else; a target that turns AND
/// recolours asks for a basis at the meet of `𝒢` and `𝒽`, and the meet is
/// empty, because dividing out the turn would leave the recolouring
/// underived and assert a bound the presentation does not give.
#[test]
fn generators_at_meet_refuses_generator_one_lane_holds() {
    use crate::transducer::sentence::CycleType;
    let held = Lanes::holding(turn_90(1), Vec::new(), 1);
    let turn = turn_90(2).rotor();
    let holonomic_only = occupied_lanes(&turn, &CycleType::Identity);
    let with_hue = occupied_lanes(&turn, &CycleType::of(&swap()));
    assert_eq!(
        held.generators_at_meet(&holonomic_only, &mut budget()).len(),
        1,
        "a target that only turns reaches it"
    );
    assert_eq!(
        held.generators_at_meet(&with_hue, &mut budget()).len(),
        0,
        "the same target that recolours does not: 𝒢 does not hold it"
    );
}

/// AND THE MEET ADMITS WHAT EVERY OCCUPIED LANE HOLDS. The same target,
/// against a mind whose generator turns and recolours, reaches it — one
/// member of the meet, reached through two covers and counted once,
/// because the generator is addressed by `(form, vertex)` and not by which
/// cover answered.
#[test]
fn generators_at_meet_admits_generator_every_lane_holds() {
    use crate::transducer::sentence::CycleType;
    let held = Lanes::holding(turn_90(1), swap(), 2);
    let with_hue = occupied_lanes(&turn_90(2).rotor(), &CycleType::of(&swap()));
    assert_eq!(lanes_of(&with_hue), vec![1, 3], "the target occupies two lanes");
    let at_meet = held.generators_at_meet(&with_hue, &mut budget());
    let [(form, _, _)] = at_meet.as_slice() else {
        panic!("one generator, reached through both covers, counted once")
    };
    assert_eq!(
        *form,
        Address::from_bytes([TAG_FORM, 2]),
        "and it is the one the mind was taught"
    );
}

/// A TARGET OCCUPYING NOTHING GETS NO BASIS. The meet over an empty family
/// is formally everything; answering everything here would be the union
/// L12 exists to refuse, and a target on no lane is the identity and has
/// nothing to reduce.
#[test]
fn generators_at_meet_gives_no_basis_on_no_lane() {
    let held = Lanes::holding(turn_90(1), swap(), 2);
    assert_eq!(
        held.generators_at_meet(&[], &mut budget()).len(),
        0,
        "no lane occupied, no basis — not the whole language"
    );
}

/// A PLACEMENT IS NOT A LANE OF A DERIVATION. A derivation claims a ROTOR:
/// `factor_versor` replays its citations against the versor's sensor rotor
/// and against nothing else, and the placement rides on the vertex beside
/// the derivation (18.2 quotients the body's absolute position). So a
/// target that translates while it turns still derives through a purely
/// rotational generator.
///
/// Measured, and the reason this test exists: reading the target's own
/// carriage into the lane vector put `𝒮` in the meet, no generator is ever
/// held on `𝒮`, and the citation in `generator_pay`'s 180° turn was lost —
/// a refusal by bookkeeping of a claim the derivation never made.
#[test]
fn factor_versor_derives_translating_target_through_turn() {
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let held = Lanes::holding(turn_90(1), Vec::new(), 1);
    // V = D(1)·T(3,0)·R(180° turn): a turn carried to another place.
    let turn = element::versor_of_word(&turn_90(2)).expect("a plane versor");
    let placed = element::carriage(&one, &(q(3), q(0))).geo(&turn);
    let d = factor_versor(&held, &mut budget(), &placed, &[])
        .expect("a linear block")
        .expect("the peel closes");
    assert!(!d.is_spelled(), "the mind factored through what it holds");
    let [(form, _, _)] = d.generators.as_slice() else { panic!("one generator") };
    assert_eq!(
        *form,
        Address::from_bytes([TAG_FORM, 1]),
        "cited by the address of the form the mind was taught, placement and all"
    );
}
