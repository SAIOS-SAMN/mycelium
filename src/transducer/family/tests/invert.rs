// 78.76 — THE REMAINDER INVERTED (`family/invert.rs`): the remainder read as a
// scene of its own — its unsaid sites, or the whole observed bodies they
// stand in, erasures counted apart — and a form that speaks its family whole
// has nothing to invert.

use super::common::grid;
use crate::transducer::family::invert::{invert_remainder, remainder_scene, Painted};

/// THE REMAINDER AS A SCENE. A pair whose observed output holds a two-cell
/// body, one cell of it spoken and one not: painted as SITES, the scene holds
/// the unsaid cell alone — the fragment; painted as BODIES, the whole observed
/// body. A cell the form painted where the scene holds nothing is an erasure,
/// counted and never painted. Every pair is kept, the frames the family's own.
#[test]
fn remainder_scene_paints_sites_or_bodies() {
    let x = grid(&["000", "000"]);
    let y = grid(&["220", "000"]);
    // The form spoke (0, 0) right, missed (0, 1), and painted (1, 2) wrongly.
    let rem = vec![(0, 1, 0, 2), (1, 2, 5, 0)];
    let family = vec![(x.clone(), y.clone()), (x.clone(), y.clone())];
    let rems = vec![rem, Vec::new()];
    let (sites, erasures) = remainder_scene(&family, &rems, Painted::Sites);
    assert_eq!(sites[0].1, grid(&["020", "000"]), "the unsaid site alone: a fragment of the body");
    assert_eq!(erasures, 1, "the wrong paint is an erasure, counted");
    assert_eq!(sites[1].1, grid(&["000", "000"]), "a pair with nothing unsaid asks for nothing, and stays a pair");
    let (bodies, _) = remainder_scene(&family, &rems, Painted::Bodies);
    assert_eq!(bodies[0].1, grid(&["220", "000"]), "the whole observed body the unsaid site stands in");
    assert_eq!(sites.len(), family.len(), "every pair kept: the frames are the family's own");
}

/// A FORM THAT SPEAKS ITS FAMILY WHOLE HAS NOTHING TO INVERT: no remainder,
/// no inversion.
#[test]
fn invert_remainder_inverts_nothing_for_whole_speech() {
    let family = vec![
        (grid(&["1100000", "0000000", "0002000", "0002200"]), grid(&["1100000", "0000000", "0000020", "0000022"])),
        (grid(&["0000110", "0000000", "0200000", "0220000"]), grid(&["0000110", "0000000", "0002000", "0002200"])),
    ];
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let rule = crate::transducer::family::extract_rule(&family, &mut budget).expect("a rule");
    let empty = crate::client::sdk::Epigenome::new();
    let (_, rem) = crate::transducer::family::form_deflation(&rule.form, &family, &empty);
    let left: usize = rem.iter().map(Vec::len).sum();
    match invert_remainder(&rule.form, &family, &empty, &mut budget) {
        None => assert_eq!(left, 0, "nothing inverted only where nothing was left"),
        Some((_, read)) => {
            assert_eq!(read.before, left as u64, "the inversion reads the speech's own remainder");
            assert!(!read.accepted || read.after < read.before, "accepted only where the speech left less");
        }
    }
}
