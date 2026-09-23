// 78.2, EN1 — the enclosure is the join of the centres; containment is the
// sign of the inversive product through GR1's pair reader; nesting is the
// parse.

use super::common::q;
use crate::transducer::cga::*;

fn z(n: i64) -> num_rational::BigRational {
    q(n, 1)
}

/// A nucleus of `r² = 1/4` about each centre.
fn bodies(centres: &[Vec<i64>]) -> Vec<Mv> {
    centres.iter().map(|c| sphere_at(&c.iter().map(|&x| z(x)).collect::<Vec<_>>(), &q(1, 4))).collect()
}

/// A centre padded with zeros to `n` axes.
fn at(c: &[num_rational::BigRational], n: usize) -> Vec<num_rational::BigRational> {
    (0..n)
        .map(|i| match c.get(i) {
            Some(x) => x.clone(),
            None => z(0),
        })
        .collect()
}

/// THE ENCLOSURE IS THE CARRIER OF THE ROUND THE CENTRES JOIN: three bodies
/// at `(0,0,0), (2,0,0), (0,2,0)` join a circle whose carrier is the sphere
/// about `(1,1,0)` through them, `ρ² = 2`, at `n = 3` and at `n = 4`; four
/// at the corners of the unit tetrahedron's frame join the sphere about
/// `(1,1,1)`, `ρ² = 3`. The bodies' radii do not enter: the join is of their
/// centres.
#[test]
fn the_enclosure_is_the_carrier_of_the_round_the_centres_join() {
    for n in [3usize, 4] {
        let pad = |c: &[i64]| c.iter().copied().chain(std::iter::repeat(0)).take(n).collect::<Vec<i64>>();
        let three = bodies(&[pad(&[0, 0, 0]), pad(&[2, 0, 0]), pad(&[0, 2, 0])]);
        let e = enclosure(&three).expect("three centres off one line span a round");
        assert_eq!(e, sphere_at(&at(&[z(1), z(1)], n), &z(2)), "n = {n}: the circle's carrier");
    }
    let four = bodies(&[vec![0, 0, 0], vec![2, 0, 0], vec![0, 2, 0], vec![0, 0, 2]]);
    assert_eq!(enclosure(&four), Some(sphere_at(&[z(1), z(1), z(1)], &z(3))), "four centres join their sphere");
    let collinear = bodies(&[vec![0, 0, 0], vec![1, 0, 0], vec![2, 0, 0]]);
    assert_eq!(enclosure(&collinear), None, "centres on one line span no round: the enclosure is flat");
}

/// A BODY INSIDE A THREE-BODY ENCLOSURE READS INSIDE BY SIGN, at `n = 3`
/// and `n = 4`: the enclosure about `(1,1)`, `ρ² = 2`. A small body near its
/// centre is inside; one far off is apart; a body of the cell stands on its
/// own enclosure; a body about the same centre with `r² = 9` wraps it; one
/// whose sphere crosses it stands on it; a volumeless body at its centre is
/// inside.
#[test]
fn a_body_inside_a_three_body_enclosure_reads_inside_by_sign() {
    for n in [3usize, 4] {
        let pad = |c: &[i64]| c.iter().copied().chain(std::iter::repeat(0)).take(n).collect::<Vec<i64>>();
        let cell = bodies(&[pad(&[0, 0, 0]), pad(&[2, 0, 0]), pad(&[0, 2, 0])]);
        let e = enclosure(&cell).expect("the round");
        let body = |c: &[num_rational::BigRational], r2| sphere_at(&at(c, n), &r2);
        assert_eq!(containment(&body(&[z(1), q(1, 2)], q(1, 100)), &e), Containment::Inside, "n = {n}: inside");
        assert_eq!(containment(&body(&[z(10)], q(1, 4)), &e), Containment::Apart, "n = {n}: apart");
        assert_eq!(containment(&cell[0], &e), Containment::On, "n = {n}: a body of the cell is on its enclosure");
        assert_eq!(containment(&body(&[z(1), z(1)], z(9)), &e), Containment::Wraps, "n = {n}: wraps it");
        assert_eq!(containment(&body(&[z(2), z(1)], q(1, 4)), &e), Containment::On, "n = {n}: crosses it");
        assert_eq!(containment(&body(&[z(1), z(1)], z(0)), &e), Containment::Inside, "n = {n}: a point inside");
    }
}

/// A NESTED PAIR OF ENCLOSURES READS ITS NESTING, AND NESTING IS THE PARSE:
/// a small enclosure about `(½,½)` (`ρ² = ½`) inside a middle one about the
/// same centre (`ρ² = 9/2`) inside a large one about the origin (`ρ² = 18`),
/// and one apart. Each is parsed under the wrapper closest to it, the one
/// every other wrapper also wraps.
#[test]
fn a_nested_pair_of_enclosures_reads_its_nesting() {
    let small = enclosure(&bodies(&[vec![0, 0, 0], vec![1, 0, 0], vec![0, 1, 0]])).expect("small");
    let large = enclosure(&bodies(&[vec![-3, -3, 0], vec![3, -3, 0], vec![-3, 3, 0]])).expect("large");
    let apart = enclosure(&bodies(&[vec![20, 0, 0], vec![21, 0, 0], vec![20, 1, 0]])).expect("apart");
    let middle = enclosure(&bodies(&[vec![-1, -1, 0], vec![2, -1, 0], vec![-1, 2, 0]])).expect("middle");
    assert_eq!(containment(&small, &large), Containment::Inside);
    assert_eq!(containment(&large, &small), Containment::Wraps);
    assert_eq!(containment(&small, &middle), Containment::Inside, "concentric, nested by the radical plane's side");
    assert_eq!(containment(&apart, &large), Containment::Apart);
    assert_eq!(
        nesting(&[small, large, apart, middle]),
        vec![Some(3), None, None, Some(1)],
        "small under middle under large; apart at a root"
    );
}
