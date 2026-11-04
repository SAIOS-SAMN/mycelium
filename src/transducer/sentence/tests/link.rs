// MU-SENSING §2, §3.2; MU-RECURSION 71.2 (2) — the link at rank `n`, and
// connection by the grade of the shared cell (ROADMAP track 26 S4b).

use crate::transducer::alphabet::AlphabetRank;
use crate::transducer::base::Site;
use crate::transducer::family::transduce::{Enclosed, Transduction};
use crate::transducer::sample::{Place, Sample};
use crate::transducer::sentence::{by_grade, faces_of, incidence_of, incidence_of_sites, surrounding, Link, SectionRefusal};

fn sample(extents: &[usize], width: usize, symbols: Vec<u8>) -> Sample {
    Sample::new(extents.to_vec(), width, symbols, Place::new(vec![0])).expect("the fixture stands")
}

/// A LINK CARRIES A SITE OF RANK `n` AND ITS SYMBOLS AT THE SOURCE'S WIDTH:
/// the change a rank-3 pair records at width 2 reads back as links of three
/// coordinates and two-byte symbols, exactly the sites and symbols that
/// differ. The alphabet's section admits a rank-2 link at width 1 and
/// refuses the rank-3 one by name, never truncating it.
#[test]
fn link_carries_rank_n_coordinates() {
    // Two slices of a 2×2 sample at width 2; one site repainted.
    let x = sample(&[2, 2, 2], 2, vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]);
    let changed = sample(&[2, 2, 2], 2, {
        let mut s: Vec<u8> = (0..8).flat_map(|_| [0u8, 1]).collect();
        s[14] = 7;
        s[15] = 9;
        s
    });
    let need = crate::transducer::family::transduce::need_of(&x, &changed).expect("one rank");
    let t = Transduction::of(&x, &changed, &mut crate::physics::Physics::default().stroke_n(need)).expect("read");
    let links: Vec<Link> = t.projected().expect("one enclosure").iter().map(Link::of_change).collect();
    assert_eq!(links, vec![Link::new(Site::of([1, 1, 1]), vec![0, 1], vec![7, 9])], "the one change, at rank 3, width 2");
    let [link] = links.as_slice() else { panic!("one link") };
    assert_eq!(link.rank(), 3);
    assert_eq!(link.site.axes(), &[1, 1, 1], "every coordinate kept, none privileged");
    assert_eq!(link.section(), Err(SectionRefusal::Rank(AlphabetRank { rank: 3 })), "refused by name, never truncated");
    let planar = Link::new(Site::of([4, 5]), vec![2], vec![3]);
    assert_eq!(planar.section(), Ok((4, 5, 2, 3)));
    assert_eq!(Link::of_section(&(4, 5, 2, 3)), planar, "the section's link round-trips");
    let wide = Link::new(Site::of([4, 5]), vec![2, 0], vec![3, 0]);
    assert_eq!(wide.section(), Err(SectionRefusal::Width { width: 2 }));
    let mut a = Vec::new();
    let mut b = Vec::new();
    planar.put(&mut a);
    Link::new(Site::of([4, 5, 0]), vec![2], vec![3]).put(&mut b);
    assert_ne!(a, b, "the rank is written: two ranks never share a writing");
}

/// BODIES CONNECT THROUGH A SHARED CELL OF GRADE `n − 1`: at rank 3, two
/// sites of one symbol whose cubes share a face are one body, and two that
/// share only an edge (grade `n − 2`) or a corner are two. The same reading
/// holds at ranks 1, 2 and 4; no fixed 4- or 6-neighbourhood is read.
#[test]
fn bodies_connect_by_shared_cell_of_grade_n_minus_one() {
    // Rank 3, extents 2×2×2: symbol 1 at (0,0,0) and (0,0,1), a face.
    let face = sample(&[2, 2, 2], 1, vec![1, 1, 0, 0, 0, 0, 0, 0]);
    let e = Enclosed::of(&face);
    let ones: Vec<usize> = (0..e.bodies).filter(|&b| e.body_symbol(b).map(|s| s.bytes()) == Some(&[1][..])).collect();
    assert_eq!(ones.len(), 1, "a shared face joins them");
    // (0,0,0) and (0,1,1): an edge, grade 1 = n − 2.
    let edge = sample(&[2, 2, 2], 1, vec![1, 0, 0, 1, 0, 0, 0, 0]);
    let e = Enclosed::of(&edge);
    let ones = (0..e.bodies).filter(|&b| e.body_symbol(b).map(|s| s.bytes()) == Some(&[1][..])).count();
    assert_eq!(ones, 2, "an edge is contact, not a body");
    // (0,0,0) and (1,1,1): a corner, grade 0.
    let corner = sample(&[2, 2, 2], 1, vec![1, 0, 0, 0, 0, 0, 0, 1]);
    let e = Enclosed::of(&corner);
    let ones = (0..e.bodies).filter(|&b| e.body_symbol(b).map(|s| s.bytes()) == Some(&[1][..])).count();
    assert_eq!(ones, 2, "a corner is contact, not a body");
    // The incidence of a support at every rank is its faces alone.
    for n in 1..=4usize {
        let mut sites: Vec<Site> = Vec::new();
        let count = 3usize.pow(u32::try_from(n).expect("small"));
        for code in 0..count {
            let mut rest = code;
            let axes: Vec<i64> = (0..n)
                .map(|_| {
                    let d = (rest % 3) as i64;
                    rest /= 3;
                    d
                })
                .collect();
            sites.push(Site::new(axes).expect("a site"));
        }
        sites.sort_unstable();
        for (i, j) in incidence_of_sites(&sites) {
            let differ = sites[i].axes().iter().zip(sites[j].axes()).filter(|(a, b)| a != b).count();
            assert_eq!(differ, 1, "a face at n = {n}: one coordinate differs");
        }
        assert_eq!(faces_of(n).len(), 2 * n, "a cube has 2n faces at n = {n}");
        assert_eq!(surrounding(n).len() + 1, count, "3ⁿ − 1 neighbours at n = {n}");
        assert_eq!(by_grade(n).len(), surrounding(n).len(), "one set, two orders");
    }
    // At rank 2 the planar reading is the face incidence: an L of three.
    assert_eq!(incidence_of(&[(0, 0), (0, 1), (1, 0)]), vec![(0, 1), (0, 2)]);
    assert_eq!(incidence_of(&[(0, 0), (1, 1)]), Vec::<(usize, usize)>::new(), "a corner joins nothing");
}

/// The generated orders reproduce the offsets the rule language read at
/// rank 2: the faces in [`faces_of`]'s order, every neighbour in
/// [`surrounding`]'s lexicographic order, the rays in [`by_grade`]'s.
#[test]
fn neighbourhood_orders_agree_with_rank_two_offsets() {
    let pairs = |v: Vec<Vec<i64>>| crate::transducer::sentence::on_section(v);
    assert_eq!(pairs(faces_of(2)), vec![(-1, 0), (1, 0), (0, -1), (0, 1)]);
    assert_eq!(pairs(surrounding(2)), vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]);
    assert_eq!(pairs(by_grade(2)), vec![(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)]);
}
