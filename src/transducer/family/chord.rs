// 54.7 — the transport a chord carries: the dispiration between two
// sections, and the Burgers vector between the sections' footprints on the sheet.

use crate::transducer::base::Site;

/// AXIOM B's QUANTUM `M(X)` — THE FORM'S OWN LATTICE SCALE, and one
/// chart for every chord of this percept. Moved here from `family_percept`
/// beside its one reader, [`dispiration`] (BQ1, the file bound).
///
/// `M` must be a property of the form, not of a chord: two edges of one
/// cycle each computing its own `M` would be transitions between DIFFERENT
/// stereographic charts, and composing them around the orbit would be
/// composing maps with no common domain — the holonomy would read as
/// curvature that is only a change of ruler.
///
/// The scene's extent bounds every centre of mass it can hold: a compartment
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
/// Worse, the compartment was fed ABSOLUTE `(r, c, k)`. The same form drawn
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
/// `S` is a MOMENT, not an EXTENT: every compartment enters symmetrically and
/// no axis is privileged, so there is no box. The chart bound comes
/// free — a sub-domain's centre lies in the convex hull of the whole, so
/// `|c − P|² ≤ maxᵢ|pᵢ − P|² ≤ S`, and `M ≥ 1` gives `M² ≥ M > S`
/// strictly. Every footprint of this form therefore lands inside the
/// stereographic image of the sheet, and feeding `c(D) − P` rather than
/// `c(D)` makes the reading invariant under where the scene was drawn.
///
/// OPEN, and flagged rather than silently decided: the third axis is
/// HUE, and a hue is not a length. The `(row, col, hue)` compartment is the
/// sensor's existing convention (`blade::shells`), so the moment mixes a
/// palette index into a spatial scale. This predates the chart and is
/// not repaired here; it wants a ruling on whether hue is a coordinate
/// of the base or of the fibre.
/// A.11's SINGLE PRODUCER (56.3, ruled 2026-09-22). This summed the
/// compartments a second time — a SHADOW PRODUCER of moments `blade::lift`
/// already computes exactly, and a lossy one: it took each compartment as a
/// POINT MASS at integer coordinates, where `lift` reads each compartment as
/// its round and carries the rounds' extent `N/12` on each grid axis. Two producers of one quantity, and the
/// second disagreed with the first (law 12).
///
/// The form's chart is one body — the union of every compartment its chords
/// stand on — so `centroid()` is `P` and the trace of `inertia` on every
/// axis is the scale. The trace is used HERE and only here, as a radius:
/// it is an `SO(n)` invariant and carries no orientation (A.11), which is
/// what the frame rotor is for and not this.
pub(super) fn form_chart(
    meet_chords: &[super::covered::MeetChord],
) -> (Vec<num_rational::BigRational>, num_rational::BigRational) {
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let mut cells: Vec<Site> = Vec::new();
    for ((_, _, _, da), (_, _, _, db), _, _) in meet_chords.iter() {
        cells.extend(da.iter().cloned());
        cells.extend(db.iter().cloned());
    }
    cells.sort_unstable();
    cells.dedup();
    match crate::transducer::blade::shape_of(&cells).blade {
        // A form whose chords stand on nothing has no chart to speak
        // of; the pole and unit scale are the honest empty reading.
        // The pole is the origin of every base, the empty coordinate list
        // read by inclusion.
        b if b.support.is_empty() => (Vec::new(), one),
        b => {
            #[expect(
                clippy::manual_unwrap_or_default,
                reason = "rule 11 — the pole is this reader's recorded reading of an unread centroid, written as a match so it stays named; its refusal is FR2's"
            )]
            let origin = match b.centroid() {
                Some(p) => p,
                // A body whose centroid does not read back stands at the
                // pole, as before (the reading this reader has always
                // given; rule 11's refusal is FR2's to carry).
                None => Vec::new(),
            };
            // THE TRACE ON EVERY AXIS THE BODY HAS (audit N-4). The bound
            // above is `|c − P|² ≤ S` for a centre read on every axis, and
            // `S` must be the second moment on every axis for it to hold: it
            // read `inertia[0] + inertia[1]`, the grid plane by layout
            // index, so two footprints apart along the frame could leave the
            // sheet and be counted `tau_refused` for the quantum's defect.
            let trace = (0..b.n())
                .filter_map(|i| b.moment2(i, i))
                .fold(num_rational::BigRational::from_integer(num_bigint::BigInt::from(0)), |s, m| s + m);
            (origin, one + trace)
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
/// dispiration, no matter what the scene does. Both compartments are now
/// carried onto the upper sheet by the form's quantum `M(X)` and the
/// transport is Axiom B's own transition:
///
/// ```text
///     v_a = compartment(p(D_a), M),  v_b = compartment(p(D_b), M),  v² = −1
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
    origin: &[num_rational::BigRational],
    quantum: &num_rational::BigRational,
    measured: Option<&crate::transducer::sentence::Transport>,
    da: &[Site],
    db: &[Site],
) -> Option<crate::transducer::sentence::Transport> {
    let slip = basepoint_transvection(origin, quantum, da, db);
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
    origin: &[num_rational::BigRational],
    quantum: &num_rational::BigRational,
    da: &[Site],
    db: &[Site],
) -> Option<crate::transducer::sentence::Transport> {
    // THE INTRINSIC CENTRE of a support domain: its centre of mass, exact
    // over ℚ and invariant under the order the compartments arrive in, on every
    // axis the sites have (the sheet is `ℍⁿ`, BQ4). A footprint mixing two
    // bases has no centre and is counted as unfooted.
    let centre = |d: &[Site]| -> Option<Vec<num_rational::BigRational>> {
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
        Some(sum.into_iter().map(|s| num_rational::BigRational::new(s, n.clone())).collect())
    };
    let (a, b) = (centre(da)?, centre(db)?);
    if a == b {
        // THE TWO ENDS MET. Not "collinear" — they do not lie on a
        // line — and not a coincidence, which would make a structural
        // fact sound contingent. Two footprints sharing a centre stand
        // over ONE base point, so they are two sections of one fibre
        // and the footprints' difference is VERTICAL. There is no horizontal
        // displacement for a 1-cell to carry, so none is invented
        // (rule 11); what differs between them is higher-moment shape,
        // which is a 2-cell's business.
        return None;
    }
    // Read from the form's own origin, so the transport does not depend
    // on where in the grid the scene happened to be drawn (rule 15).
    // The origin is read by inclusion: an axis it does not name is zero.
    let compartment = |p: &[num_rational::BigRational]| {
        let offset: Vec<num_rational::BigRational> = p
            .iter()
            .enumerate()
            .map(|(i, x)| match origin.get(i) {
                Some(o) => x - o,
                None => x.clone(),
            })
            .collect();
        crate::transducer::cga::hyperbolic_compartment(&offset, quantum)
    };
    let carriage = match (compartment(&a), compartment(&b)) {
        (Some(va), Some(vb)) => crate::transducer::cga::geodesic_transvection(&va, &vb),
        // Off the stereographic chart: the quantum did not cover this
        // form's extent, which is a defect in `M(X)` and not a reading.
        _ => None,
    };
    let carriage = carriage?;
    Some(crate::transducer::sentence::Transport {
        word: crate::transducer::alphabet::Word::identity(),
        carriage,
        gauge: Vec::new(),
    })
}
