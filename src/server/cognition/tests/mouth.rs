// 78.128 — A WORMHOLE BETWEEN DOMAINS: a mouth pair built by two domains'
// reserves; traversals through it alone, tolled to its keeper; closed by
// either, what is in flight returning; loops of mouths read as curvature.

use crate::anastome::dialogue::domain::{DomainGenesis, Founded, Reservation};
use crate::anastome::dialogue::handoff::{Handed, Handoff, HandoffRefusal, Level, Mouth, Offer, State, Tube};
use crate::anastome::dialogue::universe::Geometry;
use crate::server::cognition::Kernel;
use crate::transducer::cga::Mv;
use ed25519_dalek::SigningKey;
use num_rational::BigRational;

fn key(b: u8) -> SigningKey {
    SigningKey::from_bytes(&[b; 32])
}

fn vk(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

/// Domain `d`'s key and its reserve tier.
fn domain(d: u8) -> SigningKey {
    key(d)
}
fn reserve(d: u8) -> SigningKey {
    key(d + 20)
}

/// Domains 4, 5 and 6, each its own reserve, and a second tier of 4's (24 + 1).
fn founded() -> Kernel {
    let mut k = Kernel::new();
    for d in [4u8, 5, 6] {
        let mut tiers = vec![vk(&reserve(d))];
        if d == 4 {
            tiers.push(vk(&key(30)));
            tiers.sort_unstable();
        }
        let g = DomainGenesis { domain: vk(&domain(d)), tiers };
        assert!(matches!(k.admit_domain(&g.sign(&domain(d))), Founded::Sealed { .. }));
        let r = Reservation { domain: vk(&domain(d)), tier: vk(&reserve(d)), nonce: [0; 32] };
        assert!(matches!(k.designate_reserve(&r.sign(&domain(d))), Founded::Reserved { .. }));
    }
    k
}

fn one() -> Mv {
    crate::transducer::element::identity_versor()
}

/// A turn in the `e₁e₂` plane: `2 + e₁₂`, a versor.
fn turn() -> Mv {
    Mv::scalar(BigRational::from_integer(2.into())).add(&Mv::blade(0b0011, BigRational::from_integer(1.into())))
}

/// A throat of unit mass in two spaces of unit curvature radius.
fn tube() -> Tube {
    Tube { mass: (1, 1), from_space: Geometry::Curved((1, 1)), to_space: Geometry::Curved((1, 1)), supersedes: None, core: false }
}

fn mouth(a: u8, b: u8, from_frame: Mv, to_frame: Mv, toll: [u64; 4]) -> Mouth {
    Mouth {
        from_tier: vk(&reserve(a)),
        to_tier: vk(&reserve(b)),
        keeper: vk(&reserve(a)),
        from_frame,
        to_frame,
        toll,
        nonce: [a; 32],
        tariff: None,
        tube: Some(tube()),
    }
}

/// The pair offered by `a`'s reserve — funded with the throat's bank first,
/// so balances read as before — and accepted by `b`'s.
fn open(k: &mut Kernel, m: Mouth, a: u8, b: u8) -> [u8; 32] {
    let bank = m.tube.as_ref().map_or(0, |t| t.mass.0.div_ceil(t.mass.1));
    let anchor = k.anchor();
    k.treasury.genesis(vk(&reserve(a)), [bank, 0, 0, 0], anchor);
    let id = m.id();
    assert!(matches!(k.handoff(&Handoff::Mouth(Box::new(m)).sign(&reserve(a))), Handed::Sealed { .. }));
    assert!(matches!(k.handoff(&Handoff::Accept { id }.sign(&reserve(b))), Handed::Sealed { state: State::Accepted, .. }));
    id
}

/// One of matter from domain `a`'s reserve to `b`'s.
fn cargo(a: u8, b: u8, nonce: u8) -> Offer {
    Offer {
        from_tier: vk(&reserve(a)),
        to_tier: vk(&reserve(b)),
        out_nets: vec![([1; 32], [-1, 0, 0, 0])],
        in_nets: vec![([2; 32], [1, 0, 0, 0])],
        charter: [0; 32],
        nonce: [nonce; 32],
    }
}

/// THE BUILDERS ARE THE ANCIENTS, THE FRAMES REAL: a pair offered by a tier
/// that is not its domain's reserve, to a tier that is not the other's,
/// within one domain, kept by a stranger, or with a zero frame — refused.
#[test]
fn a_mouth_pair_is_built_by_two_reserves_on_real_frames() {
    let mut k = founded();
    let offer = |k: &mut Kernel, m: Mouth, by: &SigningKey| match k.handoff(&Handoff::Mouth(Box::new(m)).sign(by)) {
        Handed::Refused(r) => r,
        other => panic!("refused: {other:?}"),
    };
    // 78.136 — a line builds, but its own reserve keeps; the far end is the
    // other domain's reserve.
    let line_kept_by_other = Mouth { from_tier: vk(&key(30)), keeper: vk(&reserve(5)), ..mouth(4, 5, one(), one(), [0; 4]) };
    assert_eq!(offer(&mut k, line_kept_by_other, &key(30)), HandoffRefusal::BadKeeper);
    let to_a_line = Mouth { to_tier: vk(&key(30)), ..mouth(5, 4, one(), one(), [0; 4]) };
    assert_eq!(offer(&mut k, to_a_line, &reserve(5)), HandoffRefusal::NotReserve, "accepted by the far domain's reserve");
    let same_place = mouth(4, 4, turn(), turn(), [0; 4]);
    assert_eq!(offer(&mut k, same_place, &reserve(4)), HandoffRefusal::Unbound, "a road joins two distinct places");
    let stranger = Mouth { keeper: [9; 32], ..mouth(4, 5, one(), one(), [0; 4]) };
    assert_eq!(offer(&mut k, stranger, &reserve(4)), HandoffRefusal::BadKeeper);
    assert_eq!(offer(&mut k, mouth(4, 5, Mv::zero(), one(), [0; 4]), &reserve(4)), HandoffRefusal::Degenerate, "no horizon carried");
    assert_eq!(offer(&mut k, mouth(4, 5, one(), one(), [0; 4]), &reserve(5)), HandoffRefusal::NotTheTier);
}

/// THROUGH THE MOUTH ALONE, TOLLED TO ITS KEEPER: no mouth, no crossing; a
/// mouth joining other domains is not this crossing's; through the open
/// mouth either way, the toll escrowed from the sender and paid to the keeper
/// on acceptance.
#[test]
fn a_traversal_crosses_through_an_open_mouth_and_pays_the_keeper() {
    let mut k = founded();
    let anchor = k.anchor();
    k.treasury.genesis(vk(&reserve(5)), [10, 0, 0, 0], anchor);
    assert!(matches!(k.handoff(&Handoff::Offer(Box::new(cargo(4, 5, 1))).sign(&reserve(4))), Handed::Refused(HandoffRefusal::NoMouth)));
    let id = open(&mut k, mouth(4, 5, one(), turn(), [3, 0, 0, 0]), 4, 5);
    let other = open(&mut k, mouth(5, 6, one(), one(), [0; 4]), 5, 6);
    let wrong = Handoff::Traverse { offer: Box::new(cargo(4, 5, 1)), mouth: other };
    assert!(matches!(k.handoff(&wrong.sign(&reserve(4))), Handed::Refused(HandoffRefusal::NotTheMouth)));
    // Y to X, back through the mouth X built: the toll is X's to keep.
    let back = Handoff::Traverse { offer: Box::new(cargo(5, 4, 2)), mouth: id };
    let Handed::Sealed { id: crossing, .. } = k.handoff(&back.sign(&reserve(5))) else { panic!("crossing") };
    assert_eq!(k.treasury.balance_mu(&vk(&reserve(5)))[0], 7, "the toll escrowed from the sender");
    assert!(matches!(k.handoff(&Handoff::Accept { id: crossing }.sign(&reserve(4))), Handed::Sealed { state: State::Accepted, .. }));
    assert_eq!(k.treasury.balance_mu(&vk(&reserve(4)))[0], 3, "paid to the keeper");
}

/// CLOSED BY EITHER BUILDER, WHAT IS IN FLIGHT RETURNS: a crossing offered
/// and unanswered returns when the mouth closes, its toll refunded; a closed
/// mouth carries nothing.
#[test]
fn a_mouth_closed_returns_what_is_in_flight() {
    let mut k = founded();
    let anchor = k.anchor();
    k.treasury.genesis(vk(&reserve(4)), [5, 0, 0, 0], anchor);
    let id = open(&mut k, mouth(4, 5, one(), one(), [2, 0, 0, 0]), 4, 5);
    let go = Handoff::Traverse { offer: Box::new(cargo(4, 5, 1)), mouth: id };
    let Handed::Sealed { id: crossing, .. } = k.handoff(&go.sign(&reserve(4))) else { panic!() };
    assert_eq!(k.treasury.balance_mu(&vk(&reserve(4)))[0], 3);
    assert!(matches!(k.handoff(&Handoff::Close { id }.sign(&reserve(5))), Handed::Sealed { state: State::Returned, .. }), "either builder");
    assert_eq!(k.handoff_read(&crossing), Some((State::Returned, false, false)), "in flight, returned");
    assert_eq!(k.treasury.balance_mu(&vk(&reserve(4)))[0], 6, "its toll refunded, and the throat's bank back to its face");
    let again = Handoff::Traverse { offer: Box::new(cargo(4, 5, 2)), mouth: id };
    assert!(matches!(k.handoff(&again.sign(&reserve(4))), Handed::Refused(HandoffRefusal::NoMouth)));
}

/// LOOPS OF MOUTHS ARE CURVATURE, RECORDED: three domains joined in a cycle
/// whose chart transitions compose to the identity — nothing recorded; one
/// whose transitions turn — the loop recorded at the mouth that closed it,
/// with its exact holonomy, and traversal still open.
#[test]
fn a_loop_of_mouths_that_turns_is_recorded_as_curvature() {
    let mut k = founded();
    let r = turn();
    open(&mut k, mouth(4, 5, r.clone(), one(), [0; 4]), 4, 5);
    open(&mut k, mouth(5, 6, one(), one(), [0; 4]), 5, 6);
    let closing = open(&mut k, mouth(6, 4, one(), one(), [0; 4]), 6, 4);
    let loops = k.mouth_loops();
    let [l] = loops.as_slice() else { panic!("one loop: {loops:?}") };
    assert_eq!(l.mouth, closing, "recorded at the mouth that closed it");
    assert!(!l.holonomy.projective_eq(&one()), "it turns");
    assert!(matches!(
        k.handoff(&Handoff::Traverse { offer: Box::new(cargo(6, 4, 1)), mouth: closing }.sign(&reserve(6))),
        Handed::Sealed { .. }
    ), "curvature never refuses a crossing");
    assert!(l.tethered, "a cycle of tubes: an isometry of curved space");
    // Consistent: the third mouth carries the turn back.
    let mut k = founded();
    open(&mut k, mouth(4, 5, r.clone(), one(), [0; 4]), 4, 5);
    open(&mut k, mouth(5, 6, one(), one(), [0; 4]), 5, 6);
    open(&mut k, mouth(6, 4, one(), r, [0; 4]), 6, 4);
    assert!(k.mouth_loops().is_empty(), "the transitions compose to the identity");
}

/// An untethered mouth as one opened before tubes stands on the record:
/// two free frames, open, between `a` and `b`.
fn legacy(k: &mut Kernel, a: u8, b: u8, from_frame: Mv) -> [u8; 32] {
    let m = Mouth { tube: None, ..mouth(a, b, from_frame, one(), [0; 4]) };
    let id = m.id();
    let held = crate::server::cognition::mouth::MouthHeld { mouth: m, state: State::Accepted, opened: Some(k.chain.head_height()), at: Vec::new() };
    k.handoffs.mouths.insert(id, held);
    id
}

/// A MOUTH IS A TUBE ON CURVED SPACE: offered without its tube, with a frame
/// that is no motion of `ℍ³` (a parabolic slide fixing the point at
/// infinity, an odd reflection), or with no mass — refused. Its tube reads
/// each mouth's place on the sheet and each end's horizon level from the
/// throat's mass: equal where the curvatures are, apart where they are not.
#[test]
fn a_mouth_is_a_tube_between_places_on_curved_space() {
    use crate::transducer::cga::{translator_at, E1};
    use num_rational::BigRational as Q;
    let mut k = founded();
    let offer = |k: &mut Kernel, m: Mouth| match k.handoff(&Handoff::Mouth(Box::new(m)).sign(&reserve(4))) {
        Handed::Refused(r) => Some(r),
        _ => None,
    };
    assert_eq!(offer(&mut k, Mouth { tube: None, ..mouth(4, 5, one(), one(), [0; 4]) }), Some(HandoffRefusal::Untethered));
    let slide = translator_at(&[Q::from_integer(1.into()), Q::from_integer(0.into()), Q::from_integer(0.into())]);
    assert_eq!(offer(&mut k, mouth(4, 5, slide, one(), [0; 4])), Some(HandoffRefusal::NotIsometry), "a flat slide");
    let mirror = Mv::blade(E1, Q::from_integer(1.into()));
    assert_eq!(offer(&mut k, mouth(4, 5, one(), mirror, [0; 4])), Some(HandoffRefusal::NotIsometry), "an odd reflection");
    let massless = Mouth { tube: Some(Tube { mass: (0, 1), ..tube() }), ..mouth(4, 5, one(), one(), [0; 4]) };
    assert_eq!(offer(&mut k, massless), Some(HandoffRefusal::BadTube));
    let id = open(&mut k, mouth(4, 5, turn(), one(), [0; 4]), 4, 5);
    let t = k.tube_of(&id).expect("a tube");
    for place in [&t.from_place, &t.to_place] {
        assert_eq!(place.geo(place), Mv::scalar(Q::from_integer((-1).into())), "on the sheet, v² = −1");
    }
    assert_eq!(t.from_level, t.to_level, "one throat, one curvature: equal ends");
    let cosh2 = crate::core::certified::cosh(&crate::core::certified::Encl::exact(Q::from_integer(2.into())), 64);
    let Level::Curved(c) = &t.from_level else { panic!("a curved face") };
    assert!(cosh2.contains(c) || *c >= cosh2.hi, "c = cosh(2m/R), never undersized");
    let wide = Mouth { tube: Some(Tube { to_space: Geometry::Curved((2, 1)), ..tube() }), nonce: [9; 32], ..mouth(4, 6, one(), one(), [0; 4]) };
    let other = open(&mut k, wide, 4, 6);
    let t = k.tube_of(&other).expect("a tube");
    let (Level::Curved(from), Level::Curved(to)) = (&t.from_level, &t.to_level) else { panic!("curved faces") };
    assert!(to < from, "the flatter space's horizon sits at a lower level: the curvatures differ, not the builders");
}

/// THE PROGENITORS BUILD, THE ANCIENTS KEEP: a progenitor line offers a road
/// between two distinct places of its own domain, kept by its reserve and
/// accepted by it, banking the throat from its own μ; matter crosses within
/// the domain along it; and a line offers a tube to another domain, kept by
/// its own reserve. A plain offer within one domain is still no handoff.
#[test]
fn a_progenitor_line_builds_roads_its_ancients_keep() {
    let mut k = founded();
    let line = key(30);
    let anchor = k.anchor();
    k.treasury.genesis(vk(&line), [2, 0, 0, 0], anchor);
    // Two distinct places: the pole, and one step along a geodesic from it.
    let n = |v: i64| num_rational::BigRational::from_integer(v.into());
    let step = crate::transducer::cga::hyperbolic_compartment(&[n(1), n(0), n(0)], &n(2)).expect("on the sheet");
    let away = crate::transducer::cga::geodesic_transvection(&crate::transducer::cga::hyperboloid_pole(), &step).expect("a transvection");
    let road = Mouth { from_tier: vk(&line), to_tier: vk(&reserve(4)), keeper: vk(&reserve(4)), ..mouth(4, 4, one(), away, [0; 4]) };
    let id = road.id();
    assert!(matches!(k.handoff(&Handoff::Mouth(Box::new(road)).sign(&line)), Handed::Sealed { .. }), "a line offers a road");
    assert_eq!(k.treasury.balance_mu(&vk(&line))[0], 1, "the builder banks the throat");
    assert!(matches!(k.handoff(&Handoff::Accept { id }.sign(&reserve(4))), Handed::Sealed { state: State::Accepted, .. }), "its ancients accept");
    let within = Offer { from_tier: vk(&line), to_tier: vk(&reserve(4)), ..cargo(4, 4, 1) };
    assert!(matches!(k.handoff(&Handoff::Offer(Box::new(within.clone())).sign(&line)), Handed::Refused(HandoffRefusal::Unbound)), "no road, no handoff within");
    let along = Handoff::Traverse { offer: Box::new(within), mouth: id };
    assert!(matches!(k.handoff(&along.sign(&line)), Handed::Sealed { .. }), "along the road");
    let across = Mouth { from_tier: vk(&line), keeper: vk(&reserve(4)), nonce: [8; 32], ..mouth(4, 5, one(), one(), [0; 4]) };
    assert!(matches!(k.handoff(&Handoff::Mouth(Box::new(across)).sign(&line)), Handed::Sealed { .. }), "a line builds across, its reserve keeping");
}

/// A THROAT IS BANKED, FED, DRAWN AND RETURNED — NEVER RETIRED: the
/// offerer banks `⌈m⌉` μ-S (refunded where the offer is refused); only the
/// keeper feeds and draws, in turn; the throat widens as it is fed; a draw
/// returns to each face in proportion and never leaves it massless; closed,
/// every face gets back what it holds, and no μ is lost.
#[test]
fn a_throat_is_banked_fed_drawn_and_returned_never_retired() {
    use crate::server::cognition::mouth::throat_key;
    let mut k = founded();
    let (offerer, keeper) = (vk(&reserve(4)), vk(&reserve(5)));
    let refused = Mouth { tube: Some(Tube { mass: (5, 2), ..tube() }), nonce: [3; 32], ..mouth(4, 5, one(), one(), [0; 4]) };
    let anchor = k.anchor();
    k.treasury.genesis(offerer, [3, 0, 0, 0], anchor);
    let Handed::Sealed { id: no, .. } = k.handoff(&Handoff::Mouth(Box::new(refused)).sign(&reserve(4))) else { panic!() };
    assert_eq!(k.treasury.balance_mu(&offerer)[0], 0, "⌈5/2⌉ = 3 banked at the offer");
    assert!(matches!(k.handoff(&Handoff::Refuse { id: no }.sign(&reserve(5))), Handed::Sealed { .. }));
    assert_eq!(k.treasury.balance_mu(&offerer)[0], 3, "refused: the bank back to its face");
    let kept_by_far = Mouth { keeper, tube: Some(Tube { mass: (5, 2), ..tube() }), ..mouth(4, 5, one(), one(), [0; 4]) };
    let id = open(&mut k, kept_by_far, 4, 5);
    assert_eq!(k.treasury.held_of(&throat_key(&id, &offerer)).map(|h| h.mu[0]), Some(3));
    let before = k.tube_of(&id).expect("a tube").from_level;
    k.treasury.genesis(keeper, [4, 0, 0, 0], anchor);
    let act = |k: &mut Kernel, h: Handoff, by: &SigningKey| k.handoff(&h.sign(by));
    assert_eq!(act(&mut k, Handoff::Feed { mouth: id, mu_s: 4, seq: 0 }, &reserve(4)), Handed::Refused(HandoffRefusal::NotTheTier), "the keeper feeds");
    assert_eq!(act(&mut k, Handoff::Feed { mouth: id, mu_s: 4, seq: 1 }, &reserve(5)), Handed::Refused(HandoffRefusal::Stale));
    assert!(matches!(act(&mut k, Handoff::Feed { mouth: id, mu_s: 4, seq: 0 }, &reserve(5)), Handed::Sealed { .. }));
    assert_eq!(act(&mut k, Handoff::Feed { mouth: id, mu_s: 4, seq: 0 }, &reserve(5)), Handed::Refused(HandoffRefusal::Stale), "a resend is no second feed");
    let fed = k.tube_of(&id).expect("a tube");
    assert_eq!(fed.mass, (13, 2), "5/2 + 4");
    let (Level::Curved(after), Level::Curved(before)) = (&fed.from_level, &before) else { panic!("curved faces") };
    assert!(after > before, "fed, the throat widens");
    let mut booted = Kernel::new();
    assert!(booted.restore(&k.snapshot()), "restores");
    assert_eq!(booted.tube_of(&id), Some(fed), "the bank restored");
    assert_eq!(act(&mut k, Handoff::Draw { mouth: id, mu_s: 7, seq: 1 }, &reserve(5)), Handed::Refused(HandoffRefusal::Starved), "never massless");
    assert!(matches!(act(&mut k, Handoff::Draw { mouth: id, mu_s: 6, seq: 1 }, &reserve(5)), Handed::Sealed { .. }));
    // 6 of 7 in proportion to 3 : 4 — ⌊18/7⌋ = 2 and ⌊24/7⌋ = 3, the rest to
    // the keeper. The offerer holds 3 beside its 3 banked (`open` funds it).
    assert_eq!(k.treasury.balance_mu(&offerer)[0], 3 + 2);
    assert_eq!(k.treasury.balance_mu(&keeper)[0], 4);
    assert_eq!(k.tube_of(&id).map(|t| t.mass), Some((1, 2)), "13/2 − 6");
    assert!(matches!(act(&mut k, Handoff::Close { id }, &reserve(4)), Handed::Sealed { state: State::Returned, .. }));
    assert_eq!(k.treasury.balance_mu(&offerer)[0], 3 + 3, "closed: every face its hold");
    assert_eq!(k.treasury.balance_mu(&keeper)[0], 4, "no μ retired: 3 + 4 in, 3 + 4 out");
    assert_eq!(k.tube_of(&id).map(|t| t.mass), Some((5, 2)), "the record keeps its committed mass, its bank let go");
}

/// A TUBE SUPERSEDES AN UNTETHERED MOUTH: citing it, the tube opens and the
/// old mouth closes — what was in flight through it returns, its record
/// stays. A cycle through an untethered mouth reads as history, not tethered;
/// a tube cannot supersede another tube or a closed mouth.
#[test]
fn a_tube_supersedes_an_untethered_mouth_and_its_record_stays() {
    let mut k = founded();
    let old = legacy(&mut k, 4, 5, turn());
    open(&mut k, mouth(5, 6, one(), one(), [0; 4]), 5, 6);
    let closing = open(&mut k, mouth(6, 4, one(), one(), [0; 4]), 6, 4);
    let loops = k.mouth_loops();
    let [l] = loops.as_slice() else { panic!("one loop: {loops:?}") };
    assert_eq!((l.mouth, l.tethered), (closing, false), "through an untethered mouth: history");
    let go = Handoff::Traverse { offer: Box::new(cargo(4, 5, 1)), mouth: old };
    let Handed::Sealed { id: crossing, .. } = k.handoff(&go.sign(&reserve(4))) else { panic!("an untethered mouth still carries") };
    let new = Mouth { tube: Some(Tube { supersedes: Some(old), ..tube() }), nonce: [7; 32], ..mouth(4, 5, turn(), one(), [0; 4]) };
    let tube_id = open(&mut k, new.clone(), 4, 5);
    assert_eq!(k.handoffs.mouths.get(&old).map(|h| h.state), Some(State::Returned), "closed, its record kept");
    assert_eq!(k.handoff_read(&crossing).map(|r| r.0), Some(State::Returned), "in flight, returned");
    let again = Mouth { nonce: [8; 32], ..new };
    assert!(matches!(k.handoff(&Handoff::Mouth(Box::new(again)).sign(&reserve(4))), Handed::Refused(HandoffRefusal::NotSupersedable)));
    let not_legacy = Mouth { tube: Some(Tube { supersedes: Some(tube_id), ..tube() }), nonce: [6; 32], ..mouth(4, 5, one(), one(), [0; 4]) };
    assert!(matches!(k.handoff(&Handoff::Mouth(Box::new(not_legacy)).sign(&reserve(4))), Handed::Refused(HandoffRefusal::NotSupersedable)));
    let mut booted = Kernel::new();
    assert!(booted.restore(&k.snapshot()), "restores");
    assert_eq!(booted.tube_of(&tube_id), k.tube_of(&tube_id), "the tube restored");
    assert_eq!(booted.tube_of(&old), None, "the untethered record, untethered");
}
