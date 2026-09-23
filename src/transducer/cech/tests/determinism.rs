// 12.2, 54.7 — THE GAUGE REDUCTION IS A FUNCTOR OF THE COMPLEX, not of the
// order its edges arrived in.
//
//! `solve_gauge` builds its spanning forest by union-find in the order the
//! edges are handed to it, and a different forest gives CONJUGATE chord
//! holonomies rather than equal ones — `Ω' = h Ω h⁻¹`. The conjugacy class
//! survives, so `class_key` would agree; the ELEMENT does not, and level 1
//! (`note_synonym`'s `canonical()` bytes, `form_operators`) keys by the
//! element. Three hyphae must agree bit-exactly across a recut, so this is
//! a property to hold, not to assume.
//!
//! What guarantees it is that every input walks in the key's own byte
//! order, audited at three places (54.7, 2026-09-22):
//!
//!   1. NO RANDOMISED CONTAINER EXISTS. `HashMap`, `HashSet`, `BTreeMap`
//!      and `BTreeSet` appear nowhere in `src/` and are banned by path in
//!      `clippy.toml` under rule 9. Rust's `RandomState` seeds per PROCESS,
//!      so a hashed traversal would diverge between two hyphae while
//!      staying identical inside either one — which is the one failure this
//!      test could never see, since its six orders share one seed.
//!   2. THE FOREST'S TIE-BREAK IS CANONICAL. `fill` emits edges as
//!      `for i in 0..n { for j in i+1..n }` over `sorted_verts`, so
//!      union-find meets them in key order with `i < j`.
//!   3. THE COBOUNDARY'S FACES ARE CANONICAL. `coboundary_coeff` walks
//!      `for i in 0..n`, drops vertex `i`, re-sorts the face and alternates
//!      by `i % 2` — `Σ(−1)ⁱ ∂ᵢ` over sorted vertices — and `link` walks
//!      `star.link(v)`, a trie, in key order.
//!
//! The clippy gate covers (1); these cover the regression that would
//! reintroduce drift under it — someone handing `solve_gauge` a `Vec` in
//! production order. Neither alone is enough.
//!
//! WHY IT IS LOAD-BEARING AND NOT HOUSEKEEPING. A tree does not merely
//! conjugate the holonomies; it SELECTS A FREE BASIS `{γ_e}_{e ∉ T}` for
//! `π₁(X¹, x₀)`. Under another tree an edge that was a chord becomes a
//! branch and its displacement is absorbed into other chords' solders, so
//! the GENERATING SET differs. `form_operators` counts distinct based
//! operators over that basis, so `|im(ρ_F)| = 1` on every orbited form and
//! the `13 = 14 − 1` operator count are BASIS-DEPENDENT readings. Axiom A
//! fixes the basepoint and the trie fixes the basis; they are two
//! different jobs and both are required for those numbers to mean
//! anything across hyphae.

use super::common::{quarter, sheet, v, vol};
use crate::transducer::alphabet::Rotor;
use crate::transducer::cech::{CoboundaryOutcome, Complex, Fill, Section};
use crate::transducer::element::identity_versor as ident;

/// Build the same triangle, inserting its three edges in a given order.
fn triangle_in(order: &[usize]) -> Complex {
    let (a, b, c) = (v(1), v(2), v(3));
    let mut cx = Complex::new();
    for x in [&a, &b, &c] {
        cx.set_section(x, Section::at(x.clone(), Rotor::identity()), vol());
    }
    // Three legs that do NOT agree round the triangle, so a chord stands
    // and there is a holonomy to be order-dependent about. A flat triangle
    // would pass this test while proving nothing.
    let edges: [(&crate::transducer::address::Address, &crate::transducer::address::Address, Rotor); 3] =
        [(&a, &b, quarter()), (&b, &c, sheet()), (&a, &c, Rotor::identity())];
    for &i in order {
        let (x, y, g) = &edges[i];
        cx.set_edge(*x, *y, g.clone(), ident(), vol());
    }
    cx
}

fn reading(cx: &mut Complex) -> (String, Vec<String>) {
    let a = v(1);
    cx.coboundary_on_link(&a, |_| vol());
    let tri = cx.link(&a, 2);
    let [key, ..] = tri.as_slice() else {
        return ("no triangle".to_string(), Vec::new());
    };
    let filled = cx.fill(key);
    // The ELEMENT, written out — not the class. A conjugate holonomy has
    // the same class and different bytes, and it is the bytes level 1 keys
    // by, so the bytes are what this test compares.
    let show = |f: &Fill| -> String {
        match f {
            Fill::Open => "open".to_string(),
            Fill::Filled(g, c) => format!("filled {g:?} {c:?}"),
            Fill::Class(_, r) => format!("class {r:?}"),
            Fill::Congruent(_, g, ell) => format!("congruent {g:?} {ell}"),
            Fill::Unspellable(r) => format!("unspellable {r:?}"),
        }
    };
    let outcome = match &filled {
        CoboundaryOutcome::Attached(f) => show(f),
        CoboundaryOutcome::AlreadyFilled => "already".to_string(),
        CoboundaryOutcome::Absent => "absent".to_string(),
    };
    (outcome, cx.orbits_of(key).iter().map(show).collect())
}

/// EVERY INSERTION ORDER REACHES THE SAME GAUGE — the same fill, and the
/// same chord holonomies BYTE FOR BYTE, not merely the same conjugacy
/// class.
#[test]
fn the_gauge_does_not_depend_on_the_order_the_edges_arrived_in() {
    let orders: [[usize; 3]; 6] =
        [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
    let mut first: Option<(String, Vec<String>)> = None;
    for order in orders {
        let mut cx = triangle_in(&order);
        let got = reading(&mut cx);
        match &first {
            None => {
                assert!(
                    got.0 != "no triangle" && got.0 != "absent",
                    "the fixture must actually build a triangle to be a test of anything, \
                     got {got:?}"
                );
                first = Some(got);
            }
            Some(want) => assert_eq!(
                &got, want,
                "insertion order {order:?} reached a different gauge. A different \
                 spanning forest gives CONJUGATE holonomies, which share a class and \
                 differ in bytes — and level 1 keys by the bytes, so two hyphae would \
                 disagree across the wire"
            ),
        }
    }
}

/// AND THE CHORD ITSELF IS NOT FLAT, so the assertion above is comparing a
/// real holonomy rather than six identities.
#[test]
fn the_fixture_carries_a_chord_that_stands() {
    let mut cx = triangle_in(&[0, 1, 2]);
    let (_, orbits) = reading(&mut cx);
    assert!(
        !orbits.is_empty(),
        "a triangle whose legs disagree files an orbit; with none, the order test \
         compares nothing"
    );
}
