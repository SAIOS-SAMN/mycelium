// 65, 68 — the metabolic ledger per lane, and a draw as a vector.

/// D2 — THE METABOLIC LEDGER READS BOTH HALVES, AND NETS THEM PER
/// LANE (68).
///
/// A draw and its restore are conjugate, so a lane that cycles has
/// zero flux while a lane that spends does not — and the two must be
/// distinguishable, or "the engine is busy" and "the engine is
/// draining" read the same. Arrays throughout: netting the four into
/// one scalar trades a lane against another, which #34 refuses.
///
/// A REFUSED DRAW IS COUNTED AND MOVES NO LANE. `Reserve::draw` is
/// all-or-nothing, so a depletion must leave the reserve untouched
/// AND still register — otherwise a starved engine and an idle one
/// produce the same ledger.
#[test]
fn d2_the_ledger_separates_a_lane_that_cycles_from_one_that_spends() {
    use crate::physics::metabolism;
    let _ = metabolism::take_metabolism();
    let mut r = crate::physics::budget::Reserve::of([8, 8, 8, 8]);
    r.draw([3, 0, 0, 0]).expect("affordable");
    r.restore([3, 0, 0, 0]);
    r.draw([0, 5, 0, 0]).expect("affordable");
    r.draw([0, 99, 0, 0]).expect_err("gauge cannot cover it");
    let m = metabolism::take_metabolism();
    assert_eq!(m.drawn, [3, 5, 0, 0], "only afforded draws move a lane");
    assert_eq!(m.restored, [3, 0, 0, 0], "the conjugate half");
    assert_eq!(m.flux(), [0, 5, 0, 0], "lane 0 cycled; lane 1 spent");
    assert_eq!(m.depleted, [0, 1, 0, 0], "the refusal is a reading, named on the lane that refused it");
}

/// D2 — THE FLUX IS EXACT AT THE BOUND OF A LANE.
///
/// A lane that drew everything its counter holds and restored nothing, and
/// one that did the reverse, read their whole difference — never a value
/// clipped to a narrower integer (rule 11).
#[test]
fn the_flux_is_exact_at_the_bound_of_a_lane() {
    let m = crate::physics::metabolism::Metabolism {
        drawn: [u64::MAX, 0, u64::MAX, 7],
        restored: [0, u64::MAX, u64::MAX, 7],
        ..Default::default()
    };
    let whole = i128::from(u64::MAX);
    assert_eq!(
        m.flux(),
        [whole, -whole, 0, 0],
        "spent to the bound, returned from it, cycling at it, and cycling"
    );
}

/// D2 — ENTROPY IS PER DIMENSION, AND `Σℓ` IS THE ORBIT STRUCTURE.
///
/// The two cauterizations terminate different skeleta, so a ledger
/// that merges them cannot say which dimension the engine spends.
/// A rosette is kept apart from a period of zero because A19 spells
/// infinite order as `0` and the two readings are opposite — an orbit
/// that never closes against an orbit with no period at all; and an
/// unbound motion (PHY1) apart from both, by name.
#[test]
fn d2_entropy_and_orbit_are_counted_at_their_own_dimensions() {
    use crate::physics::metabolism;
    use crate::transducer::cga::Period;
    let _ = metabolism::take_metabolism();
    metabolism::note_cauterized();
    metabolism::note_released();
    metabolism::note_released();
    metabolism::note_orbit(Some(Period::Closes(4)));
    metabolism::note_orbit(Some(Period::Closes(2)));
    metabolism::note_orbit(Some(Period::Rosette));
    metabolism::note_orbit(Some(Period::Unbound));
    metabolism::note_orbit(None);
    let m = metabolism::take_metabolism();
    assert_eq!((m.cauterized, m.released), (1, 2), "two dimensions, two counts");
    assert_eq!(m.entropy(), 3, "and a total that does not lose them");
    assert_eq!((m.orbit_finite, m.orbit_sum), (2, 6), "Σℓ over closed orbits");
    assert_eq!(m.orbit_infinite, 1, "an orbit that never closes is not a period");
    assert_eq!((m.orbit_unbound, m.orbit_unread), (1, 1), "a motion that carries and never returns, and one unread, each named");
}

/// PHY1 — A MOTION'S PERIOD IS ITS MODES', NOT ITS ROTATION'S. A turn of
/// order four reads closed at four; the same turn carried by a translation
/// reads unbound, which the rotation's order alone counted closed.
#[test]
fn a_motions_period_is_read_off_its_modes() {
    use crate::transducer::cga::{motion_modes, period_of, Period};
    use num_rational::BigRational;
    let quarter = crate::transducer::alphabet::LetterId::turn_90((0, 1)).expect("a quarter turn");
    let word = crate::transducer::alphabet::Word::from_letters(&[(quarter, false)]);
    let turn = crate::transducer::element::versor_of_word(&word).expect("a word is a versor");
    let mut b = crate::physics::Physics::default().stroke_n(1 << 16);
    let modes = motion_modes(&turn, &mut b).expect("read").expect("decomposes");
    assert_eq!(period_of(&modes), Some(Period::Closes(4)), "a quarter turn returns at four");
    let one = BigRational::from_integer(1.into());
    let zero = BigRational::from_integer(0.into());
    // A shift in the turn's own plane is a turn about another centre: it
    // still returns at four.
    let in_plane = crate::transducer::cga::translator_at(&[one.clone(), zero.clone(), zero.clone()]);
    let modes = motion_modes(&in_plane.geo(&turn), &mut b).expect("read").expect("decomposes");
    assert_eq!(period_of(&modes), Some(Period::Closes(4)), "a turn about another centre returns at four");
    // A shift along the turn's axis is a screw: it carries and never returns.
    let along = crate::transducer::cga::translator_at(&[zero.clone(), zero, one]);
    let modes = motion_modes(&along.geo(&turn), &mut b).expect("read").expect("decomposes");
    assert_eq!(period_of(&modes), Some(Period::Unbound), "a screw never returns");
}

/// ATTENTION IS ANISOTROPIC: A DRAW IS A VECTOR AND REFUSES PER LANE
/// (65).
///
/// A scalar stroke counts `spend()` calls — implementation events, not
/// physical work — which is why a constant calibrated against it decays
/// on any refactor. And it collapses the four lanes law 5 reads
/// separately: a mind bankrupt on SCALE but rich on GAUGE could not
/// express that at all, and an expensive dilation attempt would preempt
/// it out of turning.
///
/// `Reserve` draws `[u64; 4]` all-or-nothing and refuses naming WHICH
/// lane is gone, so an action with a zero projection on a depleted lane
/// is still affordable. That is the whole content of the anisotropy.
#[test]
fn a_depleted_lane_does_not_preempt_the_others() {
    use crate::physics::Reserve;

    // A mind spent on scale and rich on gauge and translation — the
    // exact state `Bubble.escrow`'s own doc describes.
    let mut r = Reserve::of([0, 10, 10, 5]);
    assert!(!r.is_spent(), "a mind with any lane left has not yielded");

    // IT CANNOT DILATE, and the refusal says which lane and by how
    // much rather than answering a bare false (rule 11).
    let d = r.draw([1, 0, 0, 0]).expect_err("scale is gone");
    assert_eq!((d.lane, d.asked, d.held), (0, 1, 0), "the refusal names the lane");
    assert_eq!(r.lanes(), [0, 10, 10, 5], "and an refused draw costs nothing");

    // BUT IT CAN STILL TURN AND SLIDE. Under a scalar stroke the
    // refused dilation would have burned the counter and preempted
    // these.
    r.draw([0, 3, 2, 0]).expect("gauge and translation are affordable");
    assert_eq!(r.lanes(), [0, 7, 8, 5], "the whole vector lands, or none of it");

    // ALL OR NOTHING ACROSS LANES: a draw affordable on three lanes and
    // not the fourth lands nowhere.
    let d = r.draw([0, 1, 1, 99]).expect_err("hue cannot cover it");
    assert_eq!(d.lane, 3, "and the refusal names the lane that could not");
    assert_eq!(r.lanes(), [0, 7, 8, 5], "with nothing deducted");

    // THE CONJUGATE. An escrow that can only be spent is the ratchet
    // one level down (62), so a draw returns.
    r.restore([0, 3, 2, 0]);
    assert_eq!(r.lanes(), [0, 10, 10, 5], "a returned draw is exactly the draw");

    // SPENT IS EVERY LANE, not any lane.
    let mut empty = Reserve::of([0, 0, 0, 0]);
    assert!(empty.is_spent());
    assert!(empty.draw([0, 0, 0, 0]).is_ok(), "a zero draw is always affordable");
}
