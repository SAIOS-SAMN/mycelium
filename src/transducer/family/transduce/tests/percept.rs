// MU-SENSING §3.6, §4 (ROADMAP track 26 S4b) — no axis is privileged in
// perception: relabelling a family's axes permutes its percept exactly.

use super::{at, extents, sample_by, sample_n};
use crate::transducer::base::Site;
use crate::transducer::family::transduce::*;
use crate::transducer::sample::Sample;

/// The sample with its axes relabelled: output axis `j` reads axis
/// `order[j]`.
fn relabelled(s: &Sample, order: &[usize]) -> Sample {
    let e = extents(s);
    let out: Vec<usize> = order.iter().map(|&k| e[k]).collect();
    sample_by(&out, |t| {
        let mut source = vec![0i64; e.len()];
        for (j, &k) in order.iter().enumerate() {
            source[k] = t[j];
        }
        at(s, &source)
    })
}

/// A site relabelled the same way.
fn relabel(site: &Site, order: &[usize]) -> Site {
    Site::new(order.iter().map(|&k| site.axes()[k]).collect()).expect("a site")
}

/// Every body of a lifted frame as its symbol and its sites, sorted: read
/// off the bands, the test's own oracle.
fn bodies(e: &Enclosed, order: Option<&[usize]>) -> Vec<(Vec<u8>, Vec<Site>)> {
    let mut out = Vec::new();
    for body in 0..e.bodies {
        let mut sites = Vec::new();
        for (_, band) in e.body_bands(body) {
            let mut boxes: Vec<Vec<i64>> = vec![Vec::new()];
            for &(lo, hi) in &band.spans {
                boxes = boxes.into_iter().flat_map(|p| (lo..hi).map(move |c| [p.clone(), vec![c as i64]].concat())).collect();
            }
            for axes in boxes {
                let s = Site::new(axes).expect("a site");
                sites.push(match order {
                    Some(o) => relabel(&s, o),
                    None => s,
                });
            }
        }
        sites.sort_unstable();
        let symbol = e.body_symbol(body).expect("every body has a symbol").bytes().to_vec();
        out.push((symbol, sites));
    }
    out.sort_unstable();
    out
}

/// One residue link as its site and two symbols.
type Change = (Site, Vec<u8>, Vec<u8>);

/// Each held factorisation's residue as sorted sites and symbols.
fn residues(f: &Factored, order: Option<&[usize]>) -> Vec<Vec<Change>> {
    let mut out: Vec<Vec<Change>> = f
        .held()
        .iter()
        .map(|h| {
            let mut r: Vec<_> = h
                .residue
                .iter()
                .map(|l| {
                    let site = match order {
                        Some(o) => relabel(&l.site, o),
                        None => l.site.clone(),
                    };
                    (site, l.before.clone(), l.after.clone())
                })
                .collect();
            r.sort_unstable();
            r
        })
        .collect();
    out.sort_unstable();
    out
}

/// NO AXIS IS PRIVILEGED IN PERCEPTION: a rank-3 family of unequal extents,
/// perceived, and the same family with its axes relabelled, perceived:
/// the rank, the roles and the ground are one; every sample's bodies are
/// the first percept's bodies with their sites relabelled; and every pair's
/// vortex holds as many factorisations, whose residues are the first's
/// relabelled. The canonical axis order writes addresses and is no role.
#[test]
fn no_axis_is_privileged_in_perception() {
    let train = vec![
        (sample_n(&[2, 3, 4], "110000000000000000000002"), sample_n(&[2, 3, 4], "000000000000000000000211")),
        (sample_n(&[2, 3, 4], "100000000000000000000000"), sample_n(&[2, 3, 4], "000000000000000000000001")),
    ];
    let tests = vec![sample_n(&[2, 3, 4], "011000000000000000000000")];
    let order = [2usize, 0, 1];
    let moved_train: Vec<(Sample, Sample)> = train.iter().map(|(x, y)| (relabelled(x, &order), relabelled(y, &order))).collect();
    let moved_tests: Vec<Sample> = tests.iter().map(|x| relabelled(x, &order)).collect();
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 12);
    let p = TaskPercept::of(&train, &tests, &mut budget).expect("perceived");
    let q = TaskPercept::of(&moved_train, &moved_tests, &mut budget).expect("perceived");
    assert_eq!(p.rank(), 3);
    assert_eq!(q.rank(), p.rank());
    assert_eq!(q.perspective.relabelled(&order), Some(p.perspective.clone()), "every axis space, under any labelling");
    assert_eq!(q.ground, p.ground, "the ground reads no axis");
    for (a, b) in p.train.iter().zip(&q.train) {
        assert_eq!(bodies(&b.input, None), bodies(&a.input, Some(&order)), "the input's bodies, relabelled");
        assert_eq!(bodies(&b.output, None), bodies(&a.output, Some(&order)), "the output's bodies, relabelled");
        assert_eq!(b.factored.held().len(), a.factored.held().len(), "as many factorisations held");
        assert_eq!(residues(&b.factored, None), residues(&a.factored, Some(&order)), "the residues, relabelled");
    }
    for (a, b) in p.tests.iter().zip(&q.tests) {
        assert_eq!(bodies(b, None), bodies(a, Some(&order)));
    }
}

/// A family of two ranks is refused by name, never padded or cut.
#[test]
fn task_percept_refuses_two_ranks() {
    let train = vec![(sample_n(&[3], "010"), sample_n(&[1, 3], "010"))];
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 8);
    assert_eq!(TaskPercept::of(&train, &[], &mut budget), Err(PerceptRefusal::Rank { family: 1, sample: 2 }));
    assert_eq!(TaskPercept::of(&[], &[], &mut budget), Err(PerceptRefusal::NoSample));
}
