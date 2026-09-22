// 54.7 — the transport a chord carries: the dispiration between two
// sections, and the Burgers vector between their footprints on the sheet.

/// THE BURGERS VECTOR BETWEEN TWO FOOTPRINTS, as a geodesic on the
/// sheet (54.7, ruled 2026-09-21).
///
/// This read `d.iter().min()` for a footprint's basepoint and defended
/// it as "content order, never a centroid, which would be an average
/// standing in for a place". That defence was backwards. A lexicographic
/// minimum over `(i64, i64, i64)` is EXTRINSIC: it is decided by which
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
    da: &[(i64, i64, i64)],
    db: &[(i64, i64, i64)],
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
    da: &[(i64, i64, i64)],
    db: &[(i64, i64, i64)],
) -> Option<crate::transducer::sentence::Transport> {
    // THE INTRINSIC CENTRE of a support domain: its centre of mass,
    // exact over ℚ and invariant under the order the cells arrive in.
    let centre = |d: &[(i64, i64, i64)]| -> Option<[num_rational::BigRational; 3]> {
        if d.is_empty() {
            return None;
        }
        let n = num_bigint::BigInt::from(u64::try_from(d.len()).ok()?);
        let mut sum = [
            num_bigint::BigInt::from(0),
            num_bigint::BigInt::from(0),
            num_bigint::BigInt::from(0),
        ];
        for (r, c, k) in d {
            sum[0] += num_bigint::BigInt::from(*r);
            sum[1] += num_bigint::BigInt::from(*c);
            sum[2] += num_bigint::BigInt::from(*k);
        }
        Some(sum.map(|s| num_rational::BigRational::new(s, n.clone())))
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
