// 78.78 — CONFLUENCE, READ (`family/confluence.rs`): an unsaid output body is a
// creation, a single carriage or a confluence by the input bodies it stands
// on; a confluence's mass, centre, inverse generator and containment are read
// exactly.

use super::fixtures::digits;
use crate::transducer::family::confluence::{read_pair, Reproduces};
use crate::transducer::sentence::tests::fixtures::link;
use crate::transducer::spelled::SectionSite;

fn budget() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// The 441 sites of a 21×21 body carried `rows` down.
fn block(rows: i64) -> Vec<SectionSite> {
    (0..21).flat_map(|r| (0..21).map(move |c| SectionSite::on_plane([r + rows, c]))).collect()
}

/// The translation one row down.
fn down() -> crate::transducer::cga::Mv {
    use num_rational::BigRational as Q;
    crate::transducer::cga::translator_at(&[Q::from_integer(1.into()), Q::from_integer(0.into())])
}

/// D3.4 — PAST FOUR HUNDRED SITES THE CHECK IS READ: a body of 441 sites
/// carried by a translation onto its image reads `Yes` (the former cap wrote
/// `false` past 400); onto itself, `No`.
#[test]
fn confluence_reproduces_past_four_hundred_sites() {
    use crate::transducer::family::confluence::carried_onto;
    let (sites, shifted) = (block(0), block(1));
    assert_eq!(carried_onto(&down(), &sites, &shifted, &mut budget()), Reproduces::Yes);
    assert_eq!(carried_onto(&down(), &sites, &sites, &mut budget()), Reproduces::No, "the last row is carried off it");
}

/// D3.4 — A SPENT TOKEN READS UNREAD, NEVER NO: two reads a site, `2·m` in
/// all, and a token one read short of it leaves the check unread.
#[test]
fn confluence_reads_unread_on_spent_token() {
    use crate::transducer::family::confluence::carried_onto;
    let (sites, shifted) = (block(0), block(1));
    let stroke = |n| crate::physics::Physics::default().stroke_n(n);
    assert_eq!(carried_onto(&down(), &sites, &shifted, &mut stroke(2 * 441)), Reproduces::Yes, "exactly 2·m reads");
    assert_eq!(carried_onto(&down(), &sites, &shifted, &mut stroke(2 * 441 - 1)), Reproduces::Unread);
}

/// D3.4 — AN IMAGE OFF THE LATTICE READS NO: two cells three apart gathered
/// into two adjacent ones are concentric spheres, and their dilation sends a
/// corner cell to a point between sites, no compartment of any frame.
#[test]
fn confluence_off_lattice_image_reads_no() {
    use crate::transducer::family::confluence::{carried_onto, gathered};
    let on = SectionSite::on_plane;
    let (inputs, output) = (vec![on([0, 0]), on([0, 3])], vec![on([0, 1]), on([0, 2])]);
    let ((c_in, _), (c_out, _)) = (gathered(&inputs), gathered(&output));
    let v = c_out.geo(&c_in);
    assert!(crate::transducer::cga::is_versor(&v), "two concentric spheres compose a dilation");
    assert_eq!(carried_onto(&v, &inputs, &output, &mut budget()), Reproduces::No);
}

/// TWO INPUTS BRIDGED: two single cells, and an output bar joining them. The
/// bar stands on both: a confluence of two, its inputs contained, its mass
/// grown (2 → 5), its centre kept (the bar is centred between them) — the
/// spheres concentric, the generator a dilation with an exact ratio — and
/// the dilation does not carry two cells onto five.
#[test]
fn read_pair_reads_bridged_pair_as_confluence() {
    crate::transducer::raster::stand_on(0);
    let x = digits(&["10001"]);
    let y = digits(&["11111"]);
    let rem = vec![link(0, 1, 0, 1), link(0, 2, 0, 1), link(0, 3, 0, 1)];
    let p = read_pair(&x, &y, &rem, &mut budget());
    let [c] = p.confluences.as_slice() else { panic!("one confluence: {p:?}") };
    assert_eq!((c.inputs, c.mass_in, c.mass_out), (2, 2, 5));
    assert!(c.contains && c.centre_kept, "both inputs stand in the bar, which is centred between them");
    assert!(c.ratio.is_some(), "concentric: the dilation's ratio is read exactly");
    assert_eq!(c.reproduces, Reproduces::No, "a dilation carries two cells onto two, never onto five");
    assert_eq!(p.confluence_sites, 3, "the three unsaid sites are the confluence's");
}

/// CREATION AND SINGLE CARRIAGE: an output body on no input is a creation;
/// one on exactly one input is a single carriage.
#[test]
fn read_pair_tells_creation_from_single_carriage() {
    crate::transducer::raster::stand_on(0);
    let x = digits(&["100", "000"]);
    let y = digits(&["110", "002"]);
    let rem = vec![link(0, 1, 0, 1), link(1, 2, 0, 2)];
    let p = read_pair(&x, &y, &rem, &mut budget());
    assert_eq!((p.singles, p.single_sites), (1, 1), "the grown cell stands on its one input");
    assert_eq!((p.creations, p.creation_sites), (1, 1), "the new cell stands on none");
    assert!(p.confluences.is_empty());
}
