// 54.7 — the transport a chord carries: the dispiration between two
// sections, and the Burgers vector between their footprints on the sheet.

use crate::transducer::base::Site;

/// AXIOM B's QUANTUM `M(X)` — THE FORM'S OWN LATTICE SCALE, and one
/// chart for every chord of this percept. Moved here from `family_percept`
/// beside its one reader, [`dispiration`] (BQ1, the file bound).
///
/// `M` must be a property of the form, not of a chord: two edges of one
/// cycle computing their own `M` would be transitions between DIFFERENT
/// stereographic charts, and composing them around the loop would be
/// composing maps with no common domain — the holonomy would read as
/// curvature that is only a change of ruler.
///
/// The scene's extent bounds every centre of mass it can hold: a cell
/// stands at `0 ≤ r < rows`, `0 ≤ c < cols`, `0 ≤ k < hues`, so any
/// centroid has `|p|² < rows² + cols² + hues²`. Taking
/// `M = 1 + rows + cols + hues` gives `M² > rows² + cols² + hues² > |p|²`
/// strictly, so `|u| = |p|/M < 1` and every footprint of this form lands
/// inside the stereographic image of the sheet. Derived from the scene
/// rather than chosen, so a bigger scene gets a bigger ruler and the
/// hyperbolic distances stay comparable within the form.
/// AXIOM B's CHART FOR THIS FORM — its own origin `P(X)` and quantum
/// `M(X)`, one stereographic chart for every chord it carries.
///
/// Two earlier cuts were both flat. The first took
/// `M = 1 + rows + cols + 256`; the second replaced the 256 with the scene's hue extent. Both
/// are the AXIS-ALIGNED BOUNDING BOX of the corpus — `max` along each
/// axis in turn — carried past the sensor boundary into the connection,
/// and both SUM INCOMMENSURABLE AXES, adding a palette index to two
/// lattice positions as though a hue were a length.
///
/// Worse, the lamina was fed ABSOLUTE `(r, c, k)`. The same form drawn
/// at the origin and at the far corner of a grid then lands on different
/// sheet points and reads as different geometry — position standing in
/// for coordinate (rule 15), inside the connection itself.
///
/// Both dissolve into one construction. The form's centre and its total
/// second moment:
///
/// ```text
///     P = (1/N) Σ pᵢ            translation-equivariant: P(g·X) = g·P(X)
///     S = Σ |pᵢ − P|²           one scalar, rotation-invariant, no max
///     M = 1 + S
/// ```
///
/// `S` is a MOMENT, not an EXTENT: every cell enters symmetrically and
/// no axis is privileged, so there is no box. The chart bound comes
/// free — a sub-domain's centre lies in the convex hull of the whole, so
/// `|c − P|² ≤ maxᵢ|pᵢ − P|² ≤ S`, and `M ≥ 1` gives `M² ≥ M > S`
/// strictly. Every footprint of this form therefore lands inside the
/// stereographic image of the sheet, and feeding `c(D) − P` rather than
/// `c(D)` makes the reading invariant under where the scene was drawn.
///
/// OPEN, and flagged rather than silently decided: the third axis is
/// HUE, and a hue is not a length. The `(row, col, hue)` lamina is the
/// sensor's existing convention (`blade::shells`), so the moment mixes a
/// palette index into a spatial scale. This predates the chart and is
/// not repaired here; it wants a ruling on whether hue is a coordinate
/// of the base or of the fibre.
/// A.11's SINGLE PRODUCER (56.3, ruled 2026-09-22). This summed the
/// cells a second time — a SHADOW PRODUCER of moments `blade::lift`
/// already computes exactly, and a lossy one: it took each cell as a
/// POINT MASS at integer coordinates, where `lift` integrates the
/// lamina's own unit square and carries the parallel-axis term
/// `N/12` at every order. Two producers of one quantity, and the
/// second disagreed with the first (law 12).
///
/// The form's chart is one body — the union of every cell its chords
/// stand on — so `centroid()` is `P` and the trace of `inertia` is the
/// scale. The trace is used HERE and only here, as a radius: it is an
/// `SO(2)` invariant and carries no orientation (A.11), which is what
/// the frame rotor is for and not this.
pub(super) fn form_chart(
    meet_chords: &[super::covered::MeetChord],
) -> ([num_rational::BigRational; 3], num_rational::BigRational) {
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let mut cells: Vec<Site> = Vec::new();
    for ((_, _, _, da), (_, _, _, db), _) in meet_chords.iter() {
        cells.extend(da.iter().cloned());
        cells.extend(db.iter().cloned());
    }
    cells.sort_unstable();
    cells.dedup();
    match crate::transducer::blade::shape_of(&cells).blade {
        // A form whose chords stand on nothing has no chart to speak
        // of; the pole and unit scale are the honest empty reading.
        b if b.support.is_empty() => ([zero.clone(), zero.clone(), zero], one),
        b => {
            let (c1, c2, c3) = match b.centroid() {
                Some(p) => p,
                // A body whose centroid does not read back is refused
                // rather than defaulted (rule 11).
                None => (zero.clone(), zero.clone(), zero.clone()),
            };
            ([c1, c2, c3], one + (&b.inertia[0] + &b.inertia[1]))
        }
    }
}

/// THE BURGERS VECTOR BETWEEN TWO FOOTPRINTS, as a geodesic on the
/// sheet (54.7, ruled 2026-09-21).
///
/// This read `d.iter().min()` for a footprint's basepoint and defended
/// it as "content order, never a centroid, which would be an average
/// standing in for a place". That defence was backwards. A lexicographic
/// minimum over a site's coordinates is EXTRINSIC: it is decided by which
/// axis the tuple happens to name first, so renaming the axes moves the
/// basepoint while the scene stands still. The centre of mass is
/// equivariant — `p(g·D) = g·p(D)` for any affine `g` — so it is the
/// intrinsic representative and `min` was the rule 15 violation.
///
/// The displacement was then `τ = b − a`, an affine difference in `ℝ³`
/// injected as `B = Σ τᵢ (eᵢ ∧ e₋) = u ∧ e₋`. A single wedge of two
/// 1-vectors is a simple 2-blade by construction:
///
/// ```text
///     B ∧ B = (u ∧ e₋) ∧ (u ∧ e₋) = 0  ⟹  ⟨B²⟩₄ = 0  ⟹  I₂ ≡ 0
/// ```
///
/// so a flat translation injected as an `e₋` boost CANNOT generate a
/// dispiration, no matter what the scene does. Both cells are now
/// carried onto the upper sheet by the form's quantum `M(X)` and the
/// transport is Axiom B's own transition:
///
/// ```text
///     v_a = lamina(p(D_a), M),  v_b = lamina(p(D_b), M),  v² = −1
///     R   = 1 − v_b v_a,        ⟨R⟩₂ = v_a ∧ v_b
/// ```
///
/// `None` where the two footprints share a centre, since a zero
/// displacement is not a dislocation and none is invented (rule 11).
/// The 4 non-simple readings are therefore NOT from `τ` — they are the
/// composition `Ω_rot · Ω_trans` in `dispiration` below, where a face
/// word's rotation axis has a component along the boost direction, the
/// two bivectors fail to commute, and `B_rot ∧ B_trans ≠ 0`.
/// THE DEFECT A CHORD CARRIES — a DISPIRATION, not one of its halves.
///
/// A first cut wrote `measured.or_else(transvection)`: the rotational
/// reading OR the translational one, whichever was present. That is the
/// projection into uncoupled sub-actions — a chord whose two sections
/// differ in BOTH carriage and footprint has a rotation about an axis
/// and a slip along it at once, and choosing one discards the coupling
/// that makes it helicoidal.
///
/// ```text
///     Ω_ab = (φ_b φ_a⁻¹) · T(τ),   τ = p(D_b) − p(D_a)
/// ```
///
/// The composite's generator is non-simple where both parts are
/// present, so `I₂ = ⟨B∧B⟩₄ ≠ 0` and the screw survives. Where the
/// carriages agree it degenerates to a pure Volterra dislocation —
/// uniform Burgers vector, `I₂ = 0` honestly — and where the footprints
/// agree it is the measured rotation alone. Each is a reading of what
/// the two sections differ in, never a shape imposed on all of them.
pub(super) fn dispiration(
    origin: &[num_rational::BigRational; 3],
    quantum: &num_rational::BigRational,
    measured: Option<&crate::transducer::sentence::Transport>,
    da: &[Site],
    db: &[Site],
) -> Option<crate::transducer::sentence::Transport> {
    let slip = basepoint_transvection(origin, quantum, da, db);
    crate::transducer::family::cover::tally(|t| {
        t.chords_offered += 1;
        if slip.is_some() {
            t.tau_active += 1;
        }
    });
    match (measured, slip) {
        (None, s) => s,
        (Some(m), None) => Some(m.clone()),
        // BOTH: compose, and the product is the helicoidal transport.
        (Some(m), Some(s)) => Some(crate::transducer::sentence::Transport {
            word: m.word.clone(),
            carriage: m.carriage.geo(&s.carriage),
            gauge: m.gauge.clone(),
        }),
    }
}

fn basepoint_transvection(
    origin: &[num_rational::BigRational; 3],
    quantum: &num_rational::BigRational,
    da: &[Site],
    db: &[Site],
) -> Option<crate::transducer::sentence::Transport> {
    // THE INTRINSIC CENTRE of a support domain: its centre of mass, exact
    // over ℚ and invariant under the order the cells arrive in, on every
    // axis the sites have. The sheet's lamina reads three of them (BQ4 lifts
    // it to `n`), so a centre of another `n` is not carried and is counted
    // as unfooted rather than cut to three.
    let centre = |d: &[Site]| -> Option<[num_rational::BigRational; 3]> {
        let [first, ..] = d else { return None };
        let n = num_bigint::BigInt::from(u64::try_from(d.len()).ok()?);
        let mut sum: Vec<num_bigint::BigInt> = vec![num_bigint::BigInt::from(0); first.n()];
        for s in d {
            if s.n() != sum.len() {
                return None;
            }
            for (acc, a) in sum.iter_mut().zip(s.axes()) {
                *acc += num_bigint::BigInt::from(*a);
            }
        }
        let centre: Vec<num_rational::BigRational> =
            sum.into_iter().map(|s| num_rational::BigRational::new(s, n.clone())).collect();
        <[num_rational::BigRational; 3]>::try_from(centre).ok()
    };
    let (a, b) = match (centre(da), centre(db)) {
        (Some(a), Some(b)) => (a, b),
        _ => {
            crate::transducer::family::cover::tally(|t| t.tau_unfooted += 1);
            return None;
        }
    };
    if a == b {
        // THE TWO ENDS MET. Not "collinear" — they do not lie on a
        // line — and not a coincidence, which would make a structural
        // fact sound contingent. Two footprints sharing a centre stand
        // over ONE base point, so they are two sections of one fibre
        // and their difference is VERTICAL. There is no horizontal
        // displacement for a 1-cell to carry, so none is invented
        // (rule 11); what differs between them is higher-moment shape,
        // which is a 2-cell's business.
        crate::transducer::family::cover::tally(|t| t.tau_met += 1);
        return None;
    }
    // Read from the form's own origin, so the transport does not depend
    // on where in the grid the scene happened to be drawn (rule 15).
    let lamina = |p: &[num_rational::BigRational; 3]| {
        crate::transducer::cga::hyperbolic_lamina(
            &(&p[0] - &origin[0]),
            &(&p[1] - &origin[1]),
            &(&p[2] - &origin[2]),
            quantum,
        )
    };
    let carriage = match (lamina(&a), lamina(&b)) {
        (Some(va), Some(vb)) => crate::transducer::cga::geodesic_transvection(&va, &vb),
        // Off the stereographic chart: the quantum did not cover this
        // form's extent, which is a defect in `M(X)` and not a reading.
        _ => None,
    };
    let carriage = match carriage {
        Some(c) => c,
        None => {
            crate::transducer::family::cover::tally(|t| t.tau_refused += 1);
            return None;
        }
    };
    Some(crate::transducer::sentence::Transport {
        word: crate::transducer::alphabet::Word::identity(),
        carriage,
        gauge: Vec::new(),
    })
}
