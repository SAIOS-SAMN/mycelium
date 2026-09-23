// §8.1 — the console renders each reading whole: every declared plane, the
// flattened census named, every non-zero loss by its axis.

use mycelium::observe::{Loss, Planes, Reading, Source};
use mycelium::transducer::cga;

fn rendered(readings: Vec<Reading>) -> String {
    let src = Source::Voice([7; 32]);
    let held: Vec<(Vec<u8>, Reading)> = readings.into_iter().map(|r| (src.key(r.tag()), r)).collect();
    crate::console::render("test", held.len() as u32, &held).join("\n")
}

/// A census whose every turn stands on the grid plane is named FLATTENED,
/// and every plane its algebra declares is printed with its count.
#[test]
fn a_census_turning_only_on_the_grid_plane_is_named_flattened() {
    let grid = (cga::E1 | cga::E2) as u64;
    let out = rendered(vec![Reading::Planes(Planes { n: 3, read: 9, coupled: 0, counts: vec![(grid, 9)] })]);
    assert!(out.contains("FLATTENED"), "{out}");
    for p in cga::bivector_planes(3) {
        assert!(out.contains(&cga::plane_name(p)), "every declared plane is shown: {}", cga::plane_name(p));
    }
    let two = rendered(vec![Reading::Planes(Planes {
        n: 3,
        read: 9,
        coupled: 0,
        counts: vec![(grid, 5), ((cga::E1 | cga::E3) as u64, 4)],
    })]);
    assert!(!two.contains("FLATTENED") && two.contains("turns on 2 of 6"), "{two}");
}

/// Every non-zero axis of loss is named with its meaning; the zero ones are
/// named too, on one line, so their absence is read.
#[test]
fn every_axis_of_loss_is_read_by_name() {
    let mut l = Loss::default();
    l.axes[6] = 3;
    let out = rendered(vec![Reading::Loss(l)]);
    assert!(out.contains("tau_refused 3"), "{out}");
    assert!(out.contains("zero:") && out.contains("unheard"), "{out}");
}
