use super::super::*;

/// STD1 (rule 11) — A LEDGER COUNTER PAST ITS CARRIER IS REFUSED AND
/// COUNTED, never pinned at a ceiling: the lane stands where it was and
/// the ledger's `overflowed` rises.
#[test]
fn bump_past_carrier_is_refused_and_counted() {
    let _ = metabolism::take_metabolism();
    let mut p = Physics::default();
    p.bump(&[u64::MAX, 0, 0, 0]);
    p.bump(&[1, 2, 0, 0]);
    assert_eq!(p.pressure, [u64::MAX, 2, 0, 0], "the full lane stood; the others stepped");
    assert_eq!(metabolism::take_metabolism().overflowed, 1, "one step refused, counted");
}

/// PHY1 — A LANE RELAXES BY WHAT IT SETTLED: the conjugate of `bump`,
/// per lane, never a literal off every lane alike. A lane that settled
/// nothing keeps its pressure; a second relax with nothing settled since
/// moves nothing.
#[test]
fn relax_lowers_each_lane_by_what_it_settled() {
    let mut p = Physics::default();
    p.bump(&[5, 5, 0, 0]);
    p.syntropy(&[3, 0, 0, 0]);
    p.relax();
    assert_eq!(p.pressure, [2, 5, 0, 0], "𝒮 settled three and relaxed three; 𝒢 settled nothing");
    p.relax();
    assert_eq!(p.pressure, [2, 5, 0, 0], "nothing settled since, nothing relaxed");
}

/// PHY1 — WHAT CONTACTS DID, BOOKED ONCE: condensed contacts build order
/// on the contacts' lanes, refused ones press on the refused contacts' own lanes, and a second booking of
/// the same totals moves nothing — the mark is what was already booked.
#[test]
fn book_contacts_books_condensed_and_refused_once() {
    let mut p = Physics::default();
    p.book_contacts(([0, 2, 0, 1], [3, 0, 0, 0]));
    assert_eq!((p.syntropic, p.pressure), ([0, 2, 0, 1], [3, 0, 0, 0]));
    p.book_contacts(([0, 2, 0, 1], [3, 0, 0, 0]));
    assert_eq!((p.syntropic, p.pressure), ([0, 2, 0, 1], [3, 0, 0, 0]), "booked once");
    p.book_contacts(([0, 3, 0, 1], [3, 1, 0, 0]));
    assert_eq!((p.syntropic, p.pressure), ([0, 3, 0, 1], [3, 1, 0, 0]), "only what moved since");
}

/// Pressure is held per lane, never summed, and is relieved per lane.
#[test]
fn relieve_lowers_pressure_per_lane() {
    let mut p = Physics::default();
    p.bump(&[1, 0, 0, 0]); // a spatial demand
    p.bump(&[0, 1, 0, 0]); // a gauge demand
    assert_eq!(p.metabolism_rate(), [1, 1, 0, 0], "per-lane, not summed");
    p.relieve(&[1, 0, 0, 0]);
    assert_eq!(p.metabolism_rate(), [0, 1, 0, 0]);
}

/// A germinated pattern charges the gauge lane — read off the germ's exact
/// per-lane support (`Element::lanes()`), the coordinate-exact carrier. (Post-
/// excision every perceived change is a pattern until the mesh consolidates
/// richer structure; the gauge/spatial lanes fill as those axes germinate.)
#[test]
fn lanes_of_germinated_recolour_is_gauge() {
    use crate::transducer::element::from_canonical;
    use crate::transducer::germinate::germinate;
    // A two-site recolour: hue transitions on a standing support are
    // GAUGE content, carried alongside the residual defect — the lane
    // projection is measured from the chain, never funneled to one axis.
    let g = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
    assert_eq!(g.lanes(), [false, true, false, false], "a recolour is gauge content; temporal is empty until the listing stands");
}

/// A pivot is a torsion charge; an unattributable pivot charges every lane.
#[test]
fn pivot_unattributed_charges_torsion_on_every_lane() {
    let mut p = Physics::default();
    p.pivot([false, false, false, false]); // unattributable
    assert_eq!(p.torsion, [1, 1, 1, 1], "an unresolved residual charges torsion");
}

/// THE TWO-STROKE RESERVE: entropic expansion SPENDS per lane; depletion is
/// per-lane (polytonic); the syntropic stroke REGENERATES — and one stroke
/// recovers LESS than one spend (REGEN < SPEND), so sustained expansion is
/// paced by the recovery the optimization earns.
#[test]
fn spend_depletes_and_regenerate_restores_per_lane() {
    let t = [false, false, true, false];
    let mut p = Physics::default();
    assert_eq!(p.reserve_shortfall([true; 4]), [0; 4], "full at rest: no lane is short");
    for _ in 0..8 {
        p.spend(t); // 32 − 8·4 = 0 on the residual lane
    }
    let short = p.reserve_shortfall([true; 4]);
    assert!(short[2] > 0, "the spent lane is short by what one expansion needs: {short:?}");
    assert_eq!((short[0], short[1], short[3]), (0, 0, 0), "other lanes unaffected — polytonic");
    assert_eq!(p.reserve_shortfall([true, false, false, false]), [0; 4], "a lane not wanted reads zero");
    p.regenerate();
    assert!(p.reserve_shortfall(t)[2] > 0, "one stroke is not recovery");
    p.regenerate();
    assert_eq!(p.reserve_shortfall(t), [0; 4], "two strokes re-earn expansion");
}

/// 78.56 IV — A COURT'S RESERVE LIVES WITHIN ITS CAPACITY: Σ bonds plus
/// the work verified this window. Emptied, it refills toward that ceiling
/// at the regeneration law's rate and never past it; verified work raises
/// the ceiling; held above a lowered ceiling — an idle window after a
/// productive one — it contracts to it.
#[test]
fn regenerate_within_bounds_reserve_by_bonds_and_work() {
    let mut p = Physics { reserve: Reserve::of([0; 4]), ..Default::default() };
    // A bond with nothing on 𝒯, as the deployment's courts hold: every lane
    // still stands within the whole bond (Σ = 9), never 𝒯's own 0.
    let bonds = [3, 3, 0, 3];
    for _ in 0..16 {
        p.regenerate_within(bonds);
    }
    assert_eq!(p.reserve.lanes(), [9; 4], "refilled to Σ bonds on every lane, 𝒯 included, and no further");
    p.syntropy(&[5, 0, 0, 0]);
    p.regenerate_within(bonds);
    assert!(p.reserve.lanes()[0] > 9, "verified work raises the ceiling on its lane");
    p.regenerate_within(bonds);
    assert_eq!(p.reserve.lanes()[0], 9, "an idle window contracts it to Σ bonds");
}

/// PHY1 — THE STROKE IS A DRAW. A stroke that moved no lane draws
/// nothing; one that moved lanes draws one expansion on each, as one
/// vector. A lane that cannot cover it refuses by name, counted on that
/// lane, and the work already done is charged what the lanes hold —
/// never past empty.
#[test]
fn spend_draws_moved_lanes_and_names_refusing_lane() {
    let _ = metabolism::take_metabolism();
    let mut idle = Physics::default();
    assert_eq!(idle.spend([false; 4]), None, "a stroke that moved nothing draws nothing");
    assert_eq!(idle.reserve.lanes(), [reserve_max(); 4], "and costs nothing");
    let mut p = Physics::default();
    assert_eq!(p.spend([true, false, true, false]), None, "two lanes moved, two expansions drawn");
    assert_eq!(
        p.reserve.lanes(),
        [reserve_max() - spend_unit(), reserve_max(), reserve_max() - spend_unit(), reserve_max()]
    );
    // Spent down on 𝒮 alone, the next draw that needs 𝒮 refuses on 𝒮.
    let mut short = Physics::default();
    for _ in 0..STROKES_PER_RESERVE {
        short.spend([true, false, false, false]);
    }
    let _ = metabolism::take_metabolism();
    let refused = short.spend([true, true, false, false]).expect("𝒮 cannot cover an expansion");
    assert_eq!(refused.lane, 0, "the refusal names 𝒮");
    assert_eq!(metabolism::take_metabolism().depleted, [1, 0, 0, 0], "counted on the lane that refused");
    assert_eq!(short.reserve.lanes()[1], reserve_max() - spend_unit(), "𝒢 is still charged what it did");
    assert_eq!(short.reserve.lanes()[0], 0, "and 𝒮 never past empty");
}

/// The metabolism is per-lane syntropic minus entropic — the order flux.
#[test]
fn order_flux_is_syntropic_minus_entropic_per_lane() {
    let mut p = Physics::default();
    p.syntropy(&[1, 0, 0, 0]); // built order on 𝒮
    p.syntropy(&[1, 0, 0, 0]);
    p.pivot([false, true, false, false]); // disorder on 𝒢
    assert_eq!(p.order_flux(), [2, -1, 0, 0], "syntropic − entropic, never summed");
}

/// Pressure on G dwells G and keeps expanding on S, T, H.
#[test]
fn dwell_holds_only_wanted_pressured_lane() {
    let mut p = Physics::default();
    let g = [false, true, false, false];
    while crate::economy::lanes_short(&p.reserve_shortfall(g)).is_empty() {
        p.spend(g);
    }
    assert!(p.reserve_shortfall(g)[1] > 0, "G is spent");
    assert_eq!(
        p.reserve_shortfall([true, false, true, true]),
        [0; 4],
        "S, T and H still expand"
    );
    let pressure = [0u64, 7, 0, 0];
    let sth = [true, false, true, true];
    assert_eq!(
        p.dwell(&pressure, sth),
        [0, 0, 0, 0],
        "expanding S, T, H does not dwell for G"
    );
    assert_eq!(
        p.dwell(&pressure, g),
        [0, 7, 0, 0],
        "a stroke that needs G dwells G's pressure only"
    );
    let outcome = StrokeOutcome { pivoted: [0, 3, 0, 0], ..Default::default() };
    assert_eq!(outcome.moved(), g, "the stroke moved G and no other lane");
    assert_eq!(StrokeOutcome::default().moved(), [false; 4], "a stroke that moved nothing");
}
