//! 78.189 — THE ATLAS ON Sⁿ: charts meet by their rounds' inversive
//! product, every conformal map of Sⁿ keeps the meetings, and a triangle
//! of charts glues where its circuit of transitions composes to a scalar,
//! its holonomy kept where it does not.
use crate::transducer::blade::body::nucleus_form;
use crate::transducer::cga::{dilator, translator_at, Containment, Mv};
use crate::transducer::stalk::sphere::{gluing, meeting, overlaps, Gluing, Meeting};
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

/// A chart's round: centre `c` on the base, second moment `r²`.
fn round(c: &[(i64, i64)], r2: (i64, i64)) -> Mv {
    let c: Vec<BigRational> = c.iter().map(|&(n, d)| q(n, d)).collect();
    nucleus_form(&c, &q(r2.0, r2.1))
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// Charts meet by the sign of their rounds' inversive product: cut, nested
/// either way, or apart.
#[test]
fn charts_meet_by_inversive_product() {
    let a = round(&[(0, 1), (0, 1), (0, 1)], (4, 1));
    let cut = round(&[(3, 1), (0, 1), (0, 1)], (4, 1));
    let inside = round(&[(1, 2), (0, 1), (0, 1)], (1, 4));
    let apart = round(&[(9, 1), (0, 1), (0, 1)], (1, 1));
    assert_eq!(meeting(&a, &cut), Meeting::Overlap(Containment::On), "radii 2 and 2 at distance 3 cut");
    assert_eq!(meeting(&inside, &a), Meeting::Overlap(Containment::Inside), "a small cap inside a large one");
    assert_eq!(meeting(&a, &inside), Meeting::Overlap(Containment::Wraps), "and the large one wraps it");
    assert_eq!(meeting(&a, &apart), Meeting::Apart, "radii 2 and 1 at distance 9 are apart");
}

/// A CONFORMAL MAP OF Sⁿ KEEPS EVERY MEETING: the charts carried by one
/// dilation and one translation meet exactly as before. No grid, size or
/// place enters the atlas.
#[test]
fn every_meeting_survives_a_conformal_map() {
    let rounds = [
        round(&[(0, 1), (0, 1), (0, 1)], (4, 1)),
        round(&[(3, 1), (0, 1), (0, 1)], (4, 1)),
        round(&[(1, 2), (0, 1), (0, 1)], (1, 4)),
        round(&[(9, 1), (0, 1), (0, 1)], (1, 1)),
        round(&[(0, 1), (5, 2), (1, 3)], (9, 4)),
    ];
    let map = translator_at(&[q(-7, 3), q(11, 5), q(2, 1)]).geo(&dilator(&q(7, 3)));
    let moved: Vec<Mv> = rounds.iter().map(|r| map.sandwich(r)).collect();
    for i in 0..rounds.len() {
        for j in 0..rounds.len() {
            if i != j {
                assert_eq!(meeting(&rounds[i], &rounds[j]), meeting(&moved[i], &moved[j]), "charts {i} and {j}");
            }
        }
    }
}

/// THE ATLAS'S EDGES ARE ITS OVERLAPS: a pair apart carries no transition,
/// and the motion is asked only where two charts overlap.
#[test]
fn overlaps_hold_an_edge_per_meeting_pair() {
    let rounds = [
        round(&[(0, 1), (0, 1), (0, 1)], (4, 1)),
        round(&[(3, 1), (0, 1), (0, 1)], (4, 1)),
        round(&[(9, 1), (0, 1), (0, 1)], (1, 1)),
    ];
    let mut asked = Vec::new();
    let atlas = overlaps(
        &rounds,
        |a, b| {
            asked.push((a, b));
            None
        },
        &mut stroke(),
    )
    .expect("the stroke reads three pairs");
    assert_eq!(atlas.edges.len(), 1, "only charts 0 and 1 overlap");
    assert_eq!(asked, vec![(0, 1)], "the motion is asked on the overlap alone");
    assert!(overlaps(&rounds, |_, _| None, &mut crate::physics::Physics::default().stroke_n(2)).is_none(), "a stroke of two reads does not cover three pairs");
}

/// A TRIANGLE GLUES WHERE ITS CIRCUIT IS A SCALAR, and keeps its holonomy
/// where it is not: translations by `d₁`, `d₂` and `d₁ + d₂` glue; stating
/// `d₁ + d₂ + e` on the third edge leaves the translation by `−e` as the
/// triangle's holonomy.
#[test]
fn a_triangle_glues_or_keeps_its_holonomy() {
    let rounds = [
        round(&[(0, 1), (0, 1), (0, 1)], (4, 1)),
        round(&[(1, 1), (0, 1), (0, 1)], (4, 1)),
        round(&[(1, 1), (1, 1), (0, 1)], (4, 1)),
    ];
    let d1 = [q(1, 1), q(0, 1), q(0, 1)];
    let d2 = [q(0, 1), q(1, 1), q(0, 1)];
    let sum = [q(1, 1), q(1, 1), q(0, 1)];
    let off = [q(1, 1), q(1, 1), q(1, 3)];
    let motion = |third: [BigRational; 3]| {
        let (d1, d2) = (d1.clone(), d2.clone());
        move |a: usize, b: usize| match (a, b) {
            (0, 1) => Some(translator_at(&d1)),
            (1, 2) => Some(translator_at(&d2)),
            (0, 2) => Some(translator_at(&third)),
            _ => None,
        }
    };
    let glued = overlaps(&rounds, motion(sum.clone()), &mut stroke()).expect("three pairs read");
    let triangles = gluing(&glued, &mut stroke()).expect("one triangle read");
    assert_eq!(triangles.len(), 1, "three charts that pairwise overlap make one triangle");
    assert_eq!(triangles[0].gluing, Gluing::Glued);
    let twisted = overlaps(&rounds, motion(off), &mut stroke()).expect("three pairs read");
    let triangles = gluing(&twisted, &mut stroke()).expect("one triangle read");
    match &triangles[0].gluing {
        Gluing::Holonomy(h) => assert!(h.projective_eq(&translator_at(&[q(0, 1), q(0, 1), q(-1, 3)])), "the holonomy is the translation by −e"),
        g => panic!("a circuit off by e is held as its holonomy, got {g:?}"),
    }
    let unread = overlaps(&rounds, |a, b| (a, b).ne(&(1, 2)).then(|| translator_at(&d1)), &mut stroke()).expect("three pairs read");
    assert_eq!(gluing(&unread, &mut stroke()).expect("one triangle")[0].gluing, Gluing::Unread, "an edge with no stated motion leaves the triangle unread");
}
