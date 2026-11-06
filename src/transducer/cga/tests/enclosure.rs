// 78.2, EN1 — the enclosure is the join of the centres; containment is the
// sign of the inversive product through GR1's pair reader; nesting is the
// parse.

use super::fixtures::q;
use crate::transducer::cga::*;

fn z(n: i64) -> num_rational::BigRational {
    q(n, 1)
}

/// A nucleus at the gauge `ρ = 1/2` about each centre.
fn bodies(centres: &[Vec<i64>]) -> Vec<Mv> {
    centres.iter().map(|c| membrane_at(&c.iter().map(|&x| z(x)).collect::<Vec<_>>(), &q(1, 2)).expect("a positive gauge")).collect()
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
/// at `(0,0,0), (6,0,0), (0,8,0)` join a circle whose carrier is the
/// membrane about `(3,4,0)` through them at the gauge `ρ = 5`, at `n = 3`
/// and at `n = 4`; four at `(0,0,0), (2,0,0), (0,3,0), (0,0,6)` join the
/// membrane about `(1, 3/2, 3)` at `ρ = 7/2`. The bodies' gauges do not
/// enter: the join is of the bodies' centres.
#[test]
fn enclosure_carries_round_centres_join() {
    for n in [3usize, 4] {
        let pad = |c: &[i64]| c.iter().copied().chain(std::iter::repeat(0)).take(n).collect::<Vec<i64>>();
        let three = bodies(&[pad(&[0, 0, 0]), pad(&[6, 0, 0]), pad(&[0, 8, 0])]);
        let e = enclosure(&three).expect("three centres off one line span a round");
        assert_eq!(Some(e), membrane_at(&at(&[z(3), z(4)], n), &z(5)), "n = {n}: the circle's carrier");
    }
    let four = bodies(&[vec![0, 0, 0], vec![2, 0, 0], vec![0, 3, 0], vec![0, 0, 6]]);
    assert_eq!(enclosure(&four), membrane_at(&[z(1), q(3, 2), z(3)], &q(7, 2)), "four centres join one sphere");
    let collinear = bodies(&[vec![0, 0, 0], vec![1, 0, 0], vec![2, 0, 0]]);
    assert_eq!(enclosure(&collinear), None, "centres on one line span no round: the enclosure is flat");
}

/// A BODY INSIDE A THREE-BODY ENCLOSURE READS INSIDE BY SIGN, at `n = 3`
/// and `n = 4`: the enclosure about `(1,1)` through the cell's centres. A
/// small body near its centre is inside; one far off is apart; a body of
/// the cell stands on its own enclosure; a body about the same centre at the
/// gauge `ρ = 3` wraps it; one whose membrane crosses it stands on it; a
/// volumeless body at its centre, a point, is inside.
#[test]
fn containment_reads_body_against_enclosure_by_sign() {
    for n in [3usize, 4] {
        let pad = |c: &[i64]| c.iter().copied().chain(std::iter::repeat(0)).take(n).collect::<Vec<i64>>();
        let cell = bodies(&[pad(&[0, 0, 0]), pad(&[2, 0, 0]), pad(&[0, 2, 0])]);
        let e = enclosure(&cell).expect("the round");
        let body = |c: &[num_rational::BigRational], rho| membrane_at(&at(c, n), &rho).expect("a positive gauge");
        assert_eq!(containment(&body(&[z(1), q(1, 2)], q(1, 10)), &e), Containment::Inside, "n = {n}: inside");
        assert_eq!(containment(&body(&[z(10)], q(1, 2)), &e), Containment::Apart, "n = {n}: apart");
        assert_eq!(containment(&cell[0], &e), Containment::On, "n = {n}: a body of the cell is on its enclosure");
        assert_eq!(containment(&body(&[z(1), z(1)], z(3)), &e), Containment::Wraps, "n = {n}: wraps it");
        assert_eq!(containment(&body(&[z(2), z(1)], q(1, 2)), &e), Containment::On, "n = {n}: crosses it");
        assert_eq!(containment(&point_at(&at(&[z(1), z(1)], n)), &e), Containment::Inside, "n = {n}: a point inside");
    }
}

/// A NESTED PAIR OF ENCLOSURES READS ITS NESTING, AND NESTING IS THE PARSE:
/// a small enclosure about `(½,½)` (`ρ² = ½`) inside a middle one about the
/// same centre (`ρ² = 9/2`) inside a large one about the origin (`ρ² = 18`),
/// and one apart. Each is parsed under the wrapper closest to it, the one
/// every other wrapper also wraps.
#[test]
fn nesting_parses_each_under_closest_wrapper() {
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
